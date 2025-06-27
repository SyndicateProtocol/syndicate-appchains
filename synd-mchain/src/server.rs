//! The `synd-mchain` RPC server

// Use mock time for testing (see `methods::common::test_utils::SystemTime`)
#[cfg(test)]
use crate::methods::common::test_utils::SystemTime;
use crate::{
    db::{ArbitrumBatch, ArbitrumDB, DelayedMessage, MBlock},
    methods::{
        common::{appchain_config, Context},
        eth_methods::{
            eth_call, eth_chain_id, eth_get_block_by_hash, eth_get_block_by_number, eth_get_code,
            eth_get_logs, eth_subscribe,
        },
        mchain_methods::{add_batch, get_source_chains_processed_blocks, rollback_to_block},
    },
    metrics::MchainMetrics,
};
use alloy::{
    primitives::{Address, Bytes, U256},
    sol_types::SolValue as _,
};
use jsonrpsee::RpcModule;
#[cfg(not(test))]
use std::time::SystemTime;
use std::{collections::VecDeque, sync::Mutex, time::UNIX_EPOCH};
use tracing::error;

// 000b00800203 corresponds to a batch containing a single delayed message
const EMPTY_BATCH: Bytes = Bytes::from_static(&alloy::hex!("000b00800203"));

/// Starts the `synd-mchain` RPC server
#[allow(clippy::unwrap_used)]
pub fn start_mchain<T: ArbitrumDB + Send + Sync + 'static>(
    chain_id: u64,
    finality_delay: u64,
    db: T,
    metrics: MchainMetrics,
) -> RpcModule<(T, MchainMetrics, Mutex<Context>)> {
    db.check_version();
    let init_msg = DelayedMessage {
        kind: 11, // L1MessageType::Initialize
        sender: Address::ZERO,
        data: (
            U256::from(chain_id),
            [1u8],      // initMsgVersion
            U256::ZERO, // currentDataCost
            appchain_config(chain_id),
        )
            .abi_encode_packed()
            .into(),
        base_fee_l1: U256::ZERO,
    };
    let mut pending_ts: VecDeque<u64> = Default::default();
    let mut finalized_block = 1u64;
    if db.get_state().batch_count == 0 {
        let batch = ArbitrumBatch::new(EMPTY_BATCH, vec![init_msg]);
        db.add_batch(MBlock { payload: Some(batch), ..Default::default() }).unwrap();
    } else {
        let db_init = &db.get_block(1).unwrap().messages[0].0;
        if db_init != &init_msg {
            error!("init message does not match - are the cli arguments correct?");
            assert_eq!(db_init, &init_msg);
        }
        // search for the finalized head. store unfinalized timestamps in a queue.
        finalized_block = db.get_state().batch_count;
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        while finalized_block > 1 {
            let block_ts = db.get_block(finalized_block).unwrap().timestamp;
            if block_ts + finality_delay <= now {
                break;
            }
            finalized_block -= 1;
            pending_ts.push_front(block_ts);
        }
    }
    assert_eq!(finalized_block + pending_ts.len() as u64, db.get_state().batch_count);
    let mut module = RpcModule::new((
        db,
        metrics,
        Mutex::new(Context { finalized_block, pending_ts, subs: Default::default() }),
    ));

    // -------------------------------------------------
    // mchain methods
    // -------------------------------------------------
    module.register_method("mchain_addBatch", add_batch).unwrap();
    module.register_method("mchain_rollbackToBlock", rollback_to_block).unwrap();
    module
        .register_method(
            "mchain_getSourceChainsProcessedBlocks",
            get_source_chains_processed_blocks,
        )
        .unwrap();

    // -------------------------------------------------
    // eth methods
    // -------------------------------------------------
    module
        .register_subscription(
            "eth_subscribe",
            "eth_subscription",
            "eth_unsubscribe",
            move |p, pending, ctx, e| async move { eth_subscribe(p, pending, ctx, e).await },
        )
        .unwrap();

    module.register_method("eth_chainId", eth_chain_id).unwrap();
    module.register_method("eth_getCode", eth_get_code).unwrap();
    module.register_method("eth_getLogs", eth_get_logs).unwrap();
    module.register_method("eth_getBlockByHash", eth_get_block_by_hash).unwrap();
    module
        .register_method("eth_getBlockByNumber", move |p, c, e| {
            eth_get_block_by_number(p, c, e, finality_delay)
        })
        .unwrap();

    module.register_method("eth_call", eth_call).unwrap();

    module
}
