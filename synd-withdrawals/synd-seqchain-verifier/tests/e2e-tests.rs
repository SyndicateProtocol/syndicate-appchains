//! Test the verifier by running it with the input and config files
use std::process::Command;
use synd_seqchain_verifier::types::BlockVerifierInput;

#[tokio::test]
async fn test_run_verifier() {
    let input = std::fs::read_to_string("./tests/exo_input.json").unwrap();

    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("synd-seqchain-verifier")
        .arg("--")
        .arg("--config")
        .arg("{\"ArbitrumBridgeAddress\":\"0x27b5ba9331f20afd816c247d53bdf1ec577b04cd\"}")
        .arg("--l1-chain-input")
        .arg(input)
        .arg("--seq-config-hash")
        .arg("0xbfcbca07feca76b64a0aadd658c71e44f9ac6e77c1379d87a7748af65b82d160")
        .output()
        .expect("Failed to run verifier");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json_start = stdout.find('[').expect("No JSON output found in stdout");
    let json = &stdout[json_start..];

    let parsed: Vec<BlockVerifierInput> =
        serde_json::from_str(json).expect("Failed to parse output JSON");
    assert!(parsed.len() == 1);
    assert_eq!(parsed[0].min_block_number, 3254384);
    assert_eq!(parsed[0].max_block_number, 4694384);
    assert_eq!(parsed[0].min_timestamp, 1663069704);
    assert_eq!(parsed[0].max_timestamp, 1750189704);
    assert_eq!(parsed[0].messages.len(), 0);
    assert_eq!(parsed[0].batch.len(), 1377);
}
