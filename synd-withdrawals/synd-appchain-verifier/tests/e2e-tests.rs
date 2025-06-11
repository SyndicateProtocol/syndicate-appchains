//! Test the verifier by running it with the input and config files
use std::process::Command;
use withdrawals_shared::types::BlockVerifierInput;
#[tokio::test]
async fn test_run_appchain_verifier() {
    let sequencing_chain_input = std::fs::read_to_string("./tests/seq_input.json").unwrap();
    let settlement_chain_input = std::fs::read_to_string("./tests/set_input.json").unwrap();

    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("synd-appchain-verifier")
        .arg("--")
        .arg("--config")
        .arg("{\"SequencingContractAddress\":\"0xb89d1d2e9bc9a14855e6c8509dd5435422ccdd8f\",\"SettlementDelay\":60}")
        .arg("--sequencing-chain-input")
        .arg(sequencing_chain_input)
        .arg("--settlement-chain-input")
        .arg(settlement_chain_input)
        .arg("--appchain-config-hash")
        .arg("0xee916e135c68392315f7b10e2b304e5e519950e64c111e816e0cdef42b8e8763")
        .output()
        .expect("Failed to run verifier");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json_start = stdout.find('[').expect("No JSON output found in stdout");
    let json = &stdout[json_start..];
    let parsed: Vec<BlockVerifierInput> =
        serde_json::from_str(json).expect("Failed to parse output JSON");
    assert!(parsed.len() == 1);
    assert_eq!(parsed[0].messages.len(), 0);
    assert_eq!(parsed[0].batch.to_string(), "0x008b3e8086038468484516840482041fb870000402f86b8307c8301901840bebc20182520894b8efb7d8a0f93d5a42ee8a5bd5e89cacc2160d878203e880c080a0048b49cd14cd3aeb8be676400ec0ac367f2ca8ff426986c1b4586c105c028718a066267feb18f0fb225b79eac7511f36187af83c7b907d3fc755e94a5bf55f1eae03");
}
