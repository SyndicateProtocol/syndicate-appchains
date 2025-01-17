/// Arbitrum rollup block-builder implementation
///
/// This module provides functionality for encoding batches of transactions
/// that can be submitted by the batcher.
use base64::prelude::*;
use eyre::{eyre, Result};
use serde::ser::Error;
use serde::Serialize;
use serde::Serializer;

/// Each `L1IncomingMessage` is an L2 block. When the message is empty, the block is derived from the next delayed message instead.
#[derive(Debug, Serialize)]
pub struct Batch(pub Vec<Option<L1IncomingMessage>>);

/// See arbos/arbostypes/incomingmessage.go for the nitro version of this struct.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct L1IncomingMessage {
    pub header: L1IncomingMessageHeader,
    #[serde(serialize_with = "serialize_l2_msg")]
    pub l2_msg: Vec<Vec<u8>>,
}

fn l2_msg_to_bytes(msg: &Vec<Vec<u8>>) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    if msg.is_empty() {
        return Err(eyre!("cannot serialize empty l2 msg"));
    }
    if msg.len() > 1 {
        data.push(L2MessageKind::Batch as u8);
        for tx in msg {
            data.extend_from_slice(&tx.len().to_be_bytes());
            data.push(L2MessageKind::SignedTx as u8);
            data.extend(tx);
        }
    } else {
        data.push(L2MessageKind::SignedTx as u8);
        data.extend(&msg[0]);
    }
    Ok(data)
}

fn serialize_l2_msg<S>(msg: &Vec<Vec<u8>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&BASE64_STANDARD.encode(l2_msg_to_bytes(msg).map_err(S::Error::custom)?))
}

enum L2MessageKind {
    Batch = 3,
    SignedTx = 4,
}

/// See arbos/arbostypes/incomingmessage.go for the nitro version of this struct.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct L1IncomingMessageHeader {
    pub block_number: u64,
    pub timestamp: u64,
}

const BROTLI_MESSAGE_HEADER_BYTE: u8 = 0;

enum BatchSegmentKind {
    L2Message = 0,
    // L2MessageBrotli = 1,
    DelayedMessages = 2,
    AdvanceTimestamp = 3,
    AdvanceL1BlockNumber = 4,
}

impl Batch {
    /// Encode the `Batch` into RLP calldata
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut ts = 0;
        let mut block = 0;
        let mut input = vec![];
        for msg in &self.0 {
            let mut data = msg.as_ref().map_or_else(
                || {
                    Ok::<_, eyre::Error>(alloy::rlp::encode(
                        BatchSegmentKind::DelayedMessages as u8,
                    ))
                },
                |x| {
                    let mut data = vec![];
                    if ts != x.header.timestamp {
                        let mut buffer = vec![];
                        buffer.push(BatchSegmentKind::AdvanceTimestamp as u8);
                        buffer.append(&mut alloy::rlp::encode(x.header.timestamp.wrapping_sub(ts)));
                        data.append(&mut alloy::rlp::encode(buffer.as_slice()));
                        ts = x.header.timestamp;
                    }
                    if block != x.header.block_number {
                        let mut buffer = vec![];
                        buffer.push(BatchSegmentKind::AdvanceL1BlockNumber as u8);
                        buffer.append(&mut alloy::rlp::encode(
                            x.header.block_number.wrapping_sub(block),
                        ));
                        data.append(&mut alloy::rlp::encode(buffer.as_slice()));
                        block = x.header.block_number;
                    }
                    let mut buffer = vec![];
                    buffer.push(BatchSegmentKind::L2Message as u8);
                    buffer.append(&mut l2_msg_to_bytes(&x.l2_msg)?);
                    data.append(&mut alloy::rlp::encode(buffer.as_slice()));
                    Ok(data)
                },
            )?;
            input.append(&mut data);
        }
        let mut out: Vec<u8> = vec![];
        brotli::enc::BrotliCompress(
            &mut input.as_slice(),
            &mut out,
            &brotli::enc::BrotliEncoderInitParams(),
        )?;
        let mut result = vec![BROTLI_MESSAGE_HEADER_BYTE];
        result.append(&mut out);
        Ok(result)
    }

    /// For testing purposes only. Docker image is currently built for arm architecture.
    #[cfg(target_os = "macos")]
    #[cfg(test)]
    async fn geth_encode(&self) -> Result<Vec<u8>> {
        use eyre::OptionExt;
        use tokio::io::AsyncWriteExt;

        let json = serde_json::to_string(&self)?;
        let mut child = tokio::process::Command::new("docker")
            .arg("run")
            .arg("-i")
            .arg("--init")
            .arg("--rm")
            .arg("ghcr.io/syndicateprotocol/generate_batch")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .spawn()?;
        child
            .stdin
            .take()
            .ok_or_eyre("Failed to take child stdin")?
            .write_all(json.as_bytes())
            .await?;
        let output = child.wait_with_output().await?;
        Ok(hex::decode(output.stdout)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_encoding() -> Result<(), serde_json::Error> {
        let batch = Batch(vec![None]);
        assert_eq!(serde_json::to_string(&batch)?, r#"[null]"#);
        Ok(())
    }

    #[tokio::test]
    async fn test_delayed_msg_encode() -> Result<()> {
        let batch = Batch(vec![None]);
        assert_eq!(batch.encode()?, hex::decode("000b00800203")?);
        Ok(())
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_tx_encode() -> Result<()> {
        use alloy::{
            network::TransactionBuilder, primitives::Address, rlp::Encodable,
            rpc::types::TransactionRequest,
        };

        let signer = alloy::signers::local::PrivateKeySigner::random();
        let wallet = alloy::network::EthereumWallet::from(signer);
        let mut tx = vec![];
        TransactionRequest::default()
            .to(Address::default())
            .value(alloy::primitives::U256::from(1))
            .nonce(0)
            .gas_limit(0)
            .max_fee_per_gas(0)
            .max_priority_fee_per_gas(0)
            .build(&wallet)
            .await?
            .encode(&mut tx);
        let batch = Batch(vec![Some(L1IncomingMessage {
            header: L1IncomingMessageHeader {
                block_number: 1,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs(),
            },
            l2_msg: vec![tx],
        })]);
        let b1 = batch.encode()?;
        let b2 = batch.geth_encode().await?;
        assert_eq!(b1[0], b1[0]);
        let mut d1: Vec<u8> = vec![];
        let mut d2: Vec<u8> = vec![];
        brotli::BrotliDecompress(&mut &b1[1..], &mut d1)?;
        brotli::BrotliDecompress(&mut &b2[1..], &mut d2)?;
        assert_eq!(d1, d2);
        Ok(())
    }
}
