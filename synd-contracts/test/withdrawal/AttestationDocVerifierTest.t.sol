// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {AttestationDocVerifier} from "../../src/withdrawal/AttestationDocVerifier.sol";
import {SP1VerifierGateway} from "@sp1-contracts/SP1VerifierGateway.sol";
import {SP1Verifier as SP1VerifierGroth16} from "@sp1-contracts/v5.0.0/SP1VerifierGroth16.sol";
import {SP1Verifier as SP1VerifierPlonk} from "@sp1-contracts/v5.0.0/SP1VerifierPlonk.sol";
import {Groth16Verifier} from "@sp1-contracts/v5.0.0/Groth16Verifier.sol";

struct SP1ProofFixtureJson {
    bytes proof;
    bytes publicValues;
    bytes32 vkey;
    bytes32 rootCertHash;
    bytes32 pcr0;
    bytes32 pcr1;
    bytes32 pcr2;
    bytes32 pcr8;
}

abstract contract BaseAttestationDocVerifierTest is Test {
    using stdJson for string;

    SP1VerifierGateway public gateway;
    AttestationDocVerifier public attestationDocVerifier;

    function loadFixture(string memory fixturePath) public view returns (SP1ProofFixtureJson memory) {
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, fixturePath);
        string memory json = vm.readFile(path);

        SP1ProofFixtureJson memory fixture;
        fixture.proof = json.readBytes(".proof");
        fixture.publicValues = json.readBytes(".publicValues");
        fixture.vkey = json.readBytes32(".vkey");
        fixture.rootCertHash = json.readBytes32(".rootCertHash");
        fixture.pcr0 = json.readBytes32(".pcr0");
        fixture.pcr1 = json.readBytes32(".pcr1");
        fixture.pcr2 = json.readBytes32(".pcr2");
        fixture.pcr8 = json.readBytes32(".pcr8");

        return fixture;
    }

    function getFixturePath() public pure virtual returns (string memory);


    // TODO this shouldn't be necessary
    function getProofType() internal virtual returns (bytes4);

    function deploySp1Verifier() internal virtual returns (address);

    function setUp() public virtual {
        SP1ProofFixtureJson memory fixture = loadFixture(getFixturePath());
        gateway = new SP1VerifierGateway(address(this));

        address sp1Verifier = deploySp1Verifier();
        gateway.addRoute(sp1Verifier);

        attestationDocVerifier = new AttestationDocVerifier(
            address(gateway),
            fixture.vkey,
            fixture.rootCertHash,
            fixture.pcr0,
            fixture.pcr1,
            fixture.pcr2,
            fixture.pcr8,
            0
        );
    }

    function test_ValidAttestationDocVerifierProof() public {
        SP1ProofFixtureJson memory fixture = loadFixture(getFixturePath());
        vm.warp(1748509951); // timestamp within the validity window

        bytes memory proof = fixture.proof;
        address publicKey = attestationDocVerifier.verifyAttestationDocProof(fixture.publicValues, proof);

        assert(publicKey == address(0x0BD6f0f44257D315C16E3d67835F8d41BD11377E));
    }

    function testRevert_InvalidAttestationDocVerifierProof() public virtual {
        SP1ProofFixtureJson memory fixture = loadFixture(getFixturePath());
        vm.warp(1748509951); // timestamp within the validity window

        // Create a fake proof and patch its selector to ensure it passes the gateway to the verifier.
        bytes memory fakeProof = new bytes(fixture.proof.length);
        bytes4 newSelector = getProofType();
        assembly {
            mstore(add(fakeProof, 0x20), newSelector)
        }

        vm.expectRevert(abi.encodeWithSelector(Groth16Verifier.ProofInvalid.selector));
        attestationDocVerifier.verifyAttestationDocProof(fixture.publicValues, fakeProof);
    }
}

contract AttestationDocVerifierGroth16Test is BaseAttestationDocVerifierTest {
    function getFixturePath() public pure override returns (string memory) {
        return "/test/withdrawal/fixtures/groth16-fixture.json";
    }

    function getProofType() internal override returns (bytes4) {
        return bytes4(new SP1VerifierGroth16().VERIFIER_HASH());
    }

    function deploySp1Verifier() internal override returns (address) {
        return address(new SP1VerifierGroth16());
    }
}

contract AttestationDocVerifierPlonkTest is BaseAttestationDocVerifierTest {
    function getFixturePath() public pure override returns (string memory) {
        return "/test/withdrawal/fixtures/plonk-fixture.json";
    }

    function getProofType() internal override returns (bytes4) {
        return bytes4(new SP1VerifierPlonk().VERIFIER_HASH());
    }

    function deploySp1Verifier() internal override returns (address) {
        return address(new SP1VerifierPlonk());
    }

    function testRevert_InvalidAttestationDocVerifierProof() public override {
        SP1ProofFixtureJson memory fixture = loadFixture(getFixturePath());
        vm.warp(1748509951); // timestamp within the validity window

        // Create a fake proof and patch its selector to ensure it passes the gateway to the verifier.
        bytes memory fakeProof = new bytes(fixture.proof.length);
        bytes4 newSelector = getProofType();
        assembly {
            mstore(add(fakeProof, 0x20), newSelector)
        }

        vm.expectRevert("error ec operation");
        attestationDocVerifier.verifyAttestationDocProof(fixture.publicValues, fakeProof);
    }
}
