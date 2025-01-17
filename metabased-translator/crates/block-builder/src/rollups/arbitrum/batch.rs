use alloy::primitives::{Address, Bytes};
use alloy::rpc::types::{TransactionInput, TransactionRequest};
use base64::prelude::*;
use serde::Serialize;
use serde::Serializer;

/// Define the `Batch` struct
/// Each `L1IncomingMessage` is an L2 block. When the message is empty, the block is derived from the next delayed message instead.
#[derive(Debug, Serialize)]
pub struct Batch(pub Vec<Option<L1IncomingMessage>>);

/// Define the `L1IncomingMessage` struct
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct L1IncomingMessage {
    /// header
    pub header: L1IncomingMessageHeader,
    /// transactions
    #[serde(serialize_with = "serialize_l2msg")]
    pub l2msg: Vec<Vec<u8>>,
}

fn l2msg_to_bytes(msg: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut data = Vec::new();
    assert!(!msg.is_empty(), "cannot serialize empty l2msg");
    if msg.len() > 1 {
        data.push(L2MessageKind::BATCH.0);
        for tx in msg {
            data.extend_from_slice(&tx.len().to_be_bytes());
            data.push(L2MessageKind::SIGNED_TX.0);
            data.extend(tx);
        }
    } else {
        data.push(L2MessageKind::SIGNED_TX.0);
        data.extend(&msg[0]);
    }
    data
}

fn serialize_l2msg<S>(msg: &Vec<Vec<u8>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&BASE64_STANDARD.encode(l2msg_to_bytes(msg)))
}

/// Define the `L2MessageKind` struct
#[derive(Debug)]
struct L2MessageKind(u8);

impl L2MessageKind {
    /// batch
    const BATCH: Self = Self(3);
    /// signed tx
    const SIGNED_TX: Self = Self(4);
}

/// Define the `L1IncomingMessageHeader` struct
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct L1IncomingMessageHeader {
    /// block number
    pub block_number: u64,
    /// timestamp
    pub timestamp: u64,
}

const BROTLI_MESSAGE_HEADER_BYTE: u8 = 0;
const BATCH_SEGMENT_KIND_L2_MESSAGE: u8 = 0;
// const BATCH_SEGMENT_KIND_L2_MESSAGE_BROTLI: u8 = 1;
const BATCH_SEGMENT_KIND_DELAYED_MESSAGES: u8 = 2;
const BATCH_SEGMENT_KIND_ADVANCE_TIMESTAMP: u8 = 3;
const BATCH_SEGMENT_KIND_ADVANCE_L1_BLOCK_NUMBER: u8 = 4;

// Implementation for the `Batch` struct
impl Batch {
    /// Encode the `Batch` into RLP
    pub fn encode(&self) -> std::io::Result<Vec<u8>> {
        let mut ts = 0;
        let mut block = 0;
        let mut input: Vec<u8> = vec![];
        for msg in &self.0 {
            let mut data = msg.as_ref().map_or_else(
                || alloy::rlp::encode(BATCH_SEGMENT_KIND_DELAYED_MESSAGES),
                |x| {
                    let mut data: Vec<u8> = vec![];
                    if ts != x.header.timestamp {
                        let mut buffer: Vec<u8> = vec![];
                        buffer.push(BATCH_SEGMENT_KIND_ADVANCE_TIMESTAMP);
                        buffer.append(&mut alloy::rlp::encode(x.header.timestamp.wrapping_sub(ts)));
                        data.append(&mut alloy::rlp::encode(buffer.as_slice()));
                        ts = x.header.timestamp;
                    }
                    if block != x.header.block_number {
                        let mut buffer: Vec<u8> = vec![];
                        buffer.push(BATCH_SEGMENT_KIND_ADVANCE_L1_BLOCK_NUMBER);
                        buffer.append(&mut alloy::rlp::encode(
                            x.header.block_number.wrapping_sub(block),
                        ));
                        data.append(&mut alloy::rlp::encode(buffer.as_slice()));
                        block = x.header.block_number;
                    }
                    let mut buffer: Vec<u8> = vec![];
                    buffer.push(BATCH_SEGMENT_KIND_L2_MESSAGE);
                    buffer.append(&mut l2msg_to_bytes(&x.l2msg));
                    data.append(&mut alloy::rlp::encode(buffer.as_slice()));
                    data
                },
            );
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
    #[allow(dead_code)]
    async fn geth_encode(&self) -> eyre::Result<Vec<u8>> {
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
        tokio::io::AsyncWriteExt::write_all(
            &mut eyre::OptionExt::ok_or_eyre(child.stdin.take(), "Failed to take child stdin")?,
            json.as_bytes(),
        )
        .await?;
        let output = child.wait_with_output().await?;
        Ok(hex::decode(output.stdout)?)
    }
}

/// Create a new batcher tx
pub fn new_batcher_tx(from: Address, to: Address, data: Bytes) -> TransactionRequest {
    let input = TransactionInput::new(data);
    TransactionRequest::default().from(from).to(to).input(input)
}

// Unit Tests
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
    async fn test_delayed_msg_encode() -> eyre::Result<()> {
        let batch = Batch(vec![None]);
        assert_eq!(batch.encode()?, hex::decode("000b00800203")?);
        Ok(())
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_tx_encode() -> eyre::Result<()> {
        use alloy::{network::TransactionBuilder, rlp::Encodable};

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
            l2msg: vec![tx],
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
