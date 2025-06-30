//! The `mchain` method implementations for the `synd-mchain` RPC server

use crate::{
    db::{to_err, ArbitrumDB, MBlock, Slot, State},
    methods::common::{create_header, err, Context},
    metrics::MchainMetrics,
};
use alloy::eips::BlockNumberOrTag;
use jsonrpsee::{
    server::SubscriptionMessage,
    types::{ErrorObjectOwned, Params},
    Extensions,
};
use std::sync::Mutex;
use tracing::error;

/// `mchain_addBatch`
#[allow(clippy::unwrap_used)]
pub fn add_batch<T: ArbitrumDB + Send + Sync + 'static>(
    p: Params<'_>,
    (db, metrics, mutex): &(T, MchainMetrics, Mutex<Context>),
    _: &Extensions,
) -> Result<Option<u64>, ErrorObjectOwned> {
    let (batch,): (MBlock,) = p.parse()?;
    let timestamp = batch.timestamp;
    let seq_block_number = batch.slot.seq_block_number;
    let block = db.add_batch(batch)?;

    metrics.record_sequencing_block(seq_block_number, timestamp);

    Ok(block.inspect(|&block| {
        metrics.record_last_block(block, timestamp);
        let mut data = mutex.lock().unwrap();
        data.pending_ts.push_back(timestamp);
        assert_eq!(data.finalized_block + data.pending_ts.len() as u64, block);
        data.subs.retain_mut(|sink| {
            !sink.is_closed() &&
                sink.try_send(SubscriptionMessage::from(
                    serde_json::value::to_raw_value(&create_header(
                        block,
                        seq_block_number,
                        timestamp,
                    ))
                    .unwrap(),
                ))
                .inspect_err(|err| error!("try_send failed: {err}"))
                .is_ok()
        });
        drop(data);
    }))
}

/// `mchain_rollbackToBlock`
#[allow(clippy::unwrap_used)]
pub fn rollback_to_block(
    params: Params<'_>,
    (db, metrics, mutex): &(impl ArbitrumDB + Send + Sync, MchainMetrics, Mutex<Context>),
    _: &Extensions,
) -> Result<(), ErrorObjectOwned> {
    let (block_number,): (u64,) = params.parse()?;
    let state = db.get_state();
    if block_number > state.batch_count {
        return Err(err("cannot set head past the last block"));
    }
    if block_number == 0 {
        return Err(err("cannot set head before the first block"));
    }

    // Get the block to roll back to
    let block = db.get_block(block_number).unwrap();
    let l1_block_number = block.slot.seq_block_number;
    let block_message_count = block.after_message_count();
    let timestamp = block.timestamp;

    if block_number == state.batch_count {
        // Reset the pending batch count.
        // Do not record or log the reorg since it has no user impact.
        db.put_state(&State { timestamp, slot: block.slot, ..state });
        return Ok(());
    }

    // Reset the state.
    // If the other deletions fail, incomplete data is ignored.
    db.put_state(&State {
        batch_count: block_number,
        batch_acc: block.after_batch_acc,
        message_count: block.after_message_count(),
        message_acc: block.after_message_acc(),
        timestamp,
        slot: block.slot,
    });

    metrics.record_reorg(block_number, state.batch_count, timestamp);

    // Delete the reorged blocks and messages
    for i in block_number..state.batch_count {
        db.delete_block(i + 1);
    }
    for i in block_message_count..state.message_count {
        db.delete_message_acc(i);
    }

    // Update stale finality data
    let mut data = mutex.lock().unwrap();
    if block_number < data.finalized_block {
        metrics.record_finalized_block(block_number, timestamp);
        data.finalized_block = block_number;
        data.pending_ts.clear();
    } else {
        let removed = (state.batch_count - block_number) as usize;
        let data_len = data.pending_ts.len();
        assert!(data_len >= removed);
        data.pending_ts.truncate(data_len - removed);
    }
    assert_eq!(data.finalized_block + data.pending_ts.len() as u64, block_number);

    data.subs.retain_mut(|sink| {
        !sink.is_closed() &&
            sink.try_send(SubscriptionMessage::from(
                serde_json::value::to_raw_value(&create_header(
                    block_number,
                    l1_block_number,
                    timestamp,
                ))
                .unwrap(),
            ))
            .inspect_err(|err| error!("try_send failed: {err}"))
            .is_ok()
    });
    drop(data);
    Ok(())
}

/// `mchain_getSourceChainsProcessedBlocks`
pub fn get_source_chains_processed_blocks(
    params: Params<'_>,
    (db, _, _): &(impl ArbitrumDB + Send + Sync, MchainMetrics, Mutex<Context>),
    _: &Extensions,
) -> Result<(Slot, u64), ErrorObjectOwned> {
    let (tag,): (BlockNumberOrTag,) = params.parse()?;
    match tag {
        BlockNumberOrTag::Pending => {
            let state = db.get_state();
            Ok((state.slot, state.batch_count + 1))
        }
        BlockNumberOrTag::Number(block_num) => {
            let block = db.get_block(block_num)?;
            Ok((block.slot, block_num))
        }
        _ => Err(to_err(format!("unexpected block tag: {tag}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::ArbitrumBatch;
    use alloy::primitives::Bytes;
    use shared::service_start_utils::MetricsState;
    use std::{
        collections::{HashMap, VecDeque},
        sync::{Mutex, RwLock},
    };

    #[derive(Debug)]
    struct TestDB(RwLock<HashMap<Bytes, Bytes>>);

    impl TestDB {
        fn new() -> Self {
            Self(RwLock::new(HashMap::new()))
        }
    }

    impl ArbitrumDB for TestDB {
        fn get<K: AsRef<[u8]>>(&self, key: K) -> Option<Bytes> {
            self.0.read().unwrap().get(key.as_ref()).cloned()
        }
        fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) {
            self.0
                .write()
                .unwrap()
                .insert(key.as_ref().to_owned().into(), value.as_ref().to_owned().into());
        }
        fn delete<K: AsRef<[u8]>>(&self, key: K) {
            self.0.write().unwrap().remove(key.as_ref());
        }
    }

    fn get_test_context() -> (TestDB, MchainMetrics, Mutex<Context>) {
        let mock_db = TestDB::new();
        let mut metrics_state = MetricsState::default();
        let metrics = MchainMetrics::new(&mut metrics_state.registry);
        let context =
            Mutex::new(Context { finalized_block: 0, pending_ts: VecDeque::new(), subs: vec![] });
        (mock_db, metrics, context)
    }

    fn get_test_mblock() -> MBlock {
        MBlock {
            payload: Some(ArbitrumBatch::default()),
            timestamp: 1000,
            slot: Slot { seq_block_number: 1, ..Default::default() },
        }
    }

    #[test]
    fn test_add_batch() {
        let batch = get_test_mblock();

        let json = serde_json::to_value([batch]).unwrap().to_string();
        let params = Params::new(Some(json.as_str()));
        let result = add_batch(params, &get_test_context(), &Extensions::default());

        assert!(result.is_ok());
        let block_num = result.unwrap();
        assert!(block_num.is_some());
        assert_eq!(block_num.unwrap(), 1);
    }

    #[test]
    fn test_invalid_rollback() {
        let json = serde_json::to_value([0u64]).unwrap().to_string();
        let params = Params::new(Some(json.as_str()));
        let result = rollback_to_block(params, &get_test_context(), &Extensions::default());

        assert!(result.is_err()); // Should fail because we can't rollback to block 0
    }

    #[test]
    fn test_rollback_to_block() {
        let (mock_db, metrics, context) = get_test_context();

        // First add a batch to have something to rollback from
        let batch = get_test_mblock();
        mock_db.add_batch(batch).unwrap();

        // Test valid rollback
        let json = serde_json::to_value([1u64]).unwrap().to_string();
        let params = Params::new(Some(json.as_str()));
        let result =
            rollback_to_block(params, &(mock_db, metrics, context), &Extensions::default());

        assert!(result.is_ok());
    }

    #[test]
    fn test_get_source_chains_processed_blocks_pending() {
        // Test pending state
        let json = serde_json::to_value([BlockNumberOrTag::Pending]).unwrap().to_string();
        let params = Params::new(Some(json.as_str()));
        let result =
            get_source_chains_processed_blocks(params, &get_test_context(), &Extensions::default());
        assert!(result.is_ok());
        let (_, block_num) = result.unwrap();
        assert_eq!(block_num, 1); // Initial state has block 1
    }

    #[test]
    fn test_get_source_chains_processed_blocks_number() {
        let (mock_db, metrics, context) = get_test_context();

        // Add batch
        let batch = get_test_mblock();
        mock_db.add_batch(batch).unwrap();

        // Test specific block number
        let json = serde_json::to_value([BlockNumberOrTag::Number(1)]).unwrap().to_string();
        let params = Params::new(Some(json.as_str()));
        let result = get_source_chains_processed_blocks(
            params,
            &(mock_db, metrics, context),
            &Extensions::default(),
        );

        assert!(result.is_ok());
        let (_, block_num) = result.unwrap();
        assert_eq!(block_num, 1);
    }
}
