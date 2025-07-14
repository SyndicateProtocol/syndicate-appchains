// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {AttestationDocVerifier, PublicValuesStruct} from "../../src/withdrawal/AttestationDocVerifier.sol";
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

    function testRevert_ValidityWindowNotStarted() public virtual {
        SP1ProofFixtureJson memory fixture = loadFixture(getFixturePath());
        vm.warp(1); // timestamp before validity window starts

        vm.expectRevert("Validity window has not started");
        attestationDocVerifier.verifyAttestationDocProof(fixture.publicValues, fixture.proof);
    }

    function testRevert_ValidityWindowEnded() public virtual {
        SP1ProofFixtureJson memory fixture = loadFixture(getFixturePath());
        vm.warp(1748509951 + 86400 * 365); // timestamp after validity window ends (1 year later)

        vm.expectRevert("Validity window has ended");
        attestationDocVerifier.verifyAttestationDocProof(fixture.publicValues, fixture.proof);
    }

    function testRevert_TimestampManipulationEdgeCase() public virtual {
        SP1ProofFixtureJson memory fixture = loadFixture(getFixturePath());

        // Test edge case where timestamp is exactly at validity window end
        // This test demonstrates potential timestamp manipulation vulnerability

        // First decode the public values to get the actual validity window
        PublicValuesStruct memory publicValues = abi.decode(fixture.publicValues, (PublicValuesStruct));

        // Set timestamp to exactly validity window end + expiration tolerance
        vm.warp(publicValues.validity_window_end + attestationDocVerifier.expirationTolerance());

        // This should still work due to expiration tolerance
        address publicKey = attestationDocVerifier.verifyAttestationDocProof(fixture.publicValues, fixture.proof);
        assert(publicKey == address(0x0BD6f0f44257D315C16E3d67835F8d41BD11377E));

        // But one second later should fail
        vm.warp(publicValues.validity_window_end + attestationDocVerifier.expirationTolerance() + 1);
        vm.expectRevert("Validity window has ended");
        attestationDocVerifier.verifyAttestationDocProof(fixture.publicValues, fixture.proof);
    }

    function testRevert_MalformedPublicValues() public virtual {
        SP1ProofFixtureJson memory fixture = loadFixture(getFixturePath());
        vm.warp(1748509951); // timestamp within the validity window

        // Test with malformed public values that would cause ABI decoding to fail
        bytes memory malformedPublicValues = hex"1234"; // Too short to be valid

        vm.expectRevert();
        attestationDocVerifier.verifyAttestationDocProof(malformedPublicValues, fixture.proof);
    }

    function testRevert_WrongRootCertHash() public virtual {
        SP1ProofFixtureJson memory fixture = loadFixture(getFixturePath());
        vm.warp(1748509951); // timestamp within the validity window

        // Modify the public values to have wrong root cert hash
        PublicValuesStruct memory publicValues = abi.decode(fixture.publicValues, (PublicValuesStruct));
        publicValues.root_cert_hash = bytes32(uint256(0xdeadbeef)); // Wrong hash

        bytes memory modifiedPublicValues = abi.encode(publicValues);

        vm.expectRevert("Root cert hash mismatch");
        attestationDocVerifier.verifyAttestationDocProof(modifiedPublicValues, fixture.proof);
    }

    function testRevert_WrongPCRValues() public virtual {
        SP1ProofFixtureJson memory fixture = loadFixture(getFixturePath());
        vm.warp(1748509951); // timestamp within the validity window

        // Test each PCR mismatch
        PublicValuesStruct memory publicValues = abi.decode(fixture.publicValues, (PublicValuesStruct));

        // Test PCR0 mismatch
        publicValues.pcr_0 = bytes32(uint256(0xdeadbeef));
        bytes memory modifiedPublicValues = abi.encode(publicValues);
        vm.expectRevert("PCR0 mismatch");
        attestationDocVerifier.verifyAttestationDocProof(modifiedPublicValues, fixture.proof);

        // Reset and test PCR1 mismatch
        publicValues = abi.decode(fixture.publicValues, (PublicValuesStruct));
        publicValues.pcr_1 = bytes32(uint256(0xdeadbeef));
        modifiedPublicValues = abi.encode(publicValues);
        vm.expectRevert("PCR1 mismatch");
        attestationDocVerifier.verifyAttestationDocProof(modifiedPublicValues, fixture.proof);

        // Reset and test PCR2 mismatch
        publicValues = abi.decode(fixture.publicValues, (PublicValuesStruct));
        publicValues.pcr_2 = bytes32(uint256(0xdeadbeef));
        modifiedPublicValues = abi.encode(publicValues);
        vm.expectRevert("PCR2 mismatch");
        attestationDocVerifier.verifyAttestationDocProof(modifiedPublicValues, fixture.proof);

        // Reset and test PCR8 mismatch
        publicValues = abi.decode(fixture.publicValues, (PublicValuesStruct));
        publicValues.pcr_8 = bytes32(uint256(0xdeadbeef));
        modifiedPublicValues = abi.encode(publicValues);
        vm.expectRevert("PCR8 mismatch");
        attestationDocVerifier.verifyAttestationDocProof(modifiedPublicValues, fixture.proof);
    }

    function testConstructorWithZeroExpirationTolerance() public virtual {
        SP1ProofFixtureJson memory fixture = loadFixture(getFixturePath());

        // Deploy with zero expiration tolerance
        AttestationDocVerifier strictVerifier = new AttestationDocVerifier(
            address(gateway),
            fixture.vkey,
            fixture.rootCertHash,
            fixture.pcr0,
            fixture.pcr1,
            fixture.pcr2,
            fixture.pcr8,
            0 // zero expiration tolerance
        );

        // Verify it works within validity window
        vm.warp(1748509951);
        address publicKey = strictVerifier.verifyAttestationDocProof(fixture.publicValues, fixture.proof);
        assert(publicKey == address(0x0BD6f0f44257D315C16E3d67835F8d41BD11377E));

        // Verify it fails immediately after validity window
        PublicValuesStruct memory publicValues = abi.decode(fixture.publicValues, (PublicValuesStruct));
        vm.warp(publicValues.validity_window_end + 1);
        vm.expectRevert("Validity window has ended");
        strictVerifier.verifyAttestationDocProof(fixture.publicValues, fixture.proof);
    }

    function testConstructorWithLargeExpirationTolerance() public virtual {
        SP1ProofFixtureJson memory fixture = loadFixture(getFixturePath());

        // Deploy with very large expiration tolerance (1 year)
        uint64 largeExpiration = 365 * 24 * 60 * 60; // 1 year in seconds
        AttestationDocVerifier lenientVerifier = new AttestationDocVerifier(
            address(gateway),
            fixture.vkey,
            fixture.rootCertHash,
            fixture.pcr0,
            fixture.pcr1,
            fixture.pcr2,
            fixture.pcr8,
            largeExpiration
        );

        // Verify it works even long after validity window
        PublicValuesStruct memory publicValues = abi.decode(fixture.publicValues, (PublicValuesStruct));
        vm.warp(publicValues.validity_window_end + largeExpiration);

        address publicKey = lenientVerifier.verifyAttestationDocProof(fixture.publicValues, fixture.proof);
        assert(publicKey == address(0x0BD6f0f44257D315C16E3d67835F8d41BD11377E));

        // But should still fail beyond the tolerance
        vm.warp(publicValues.validity_window_end + largeExpiration + 1);
        vm.expectRevert("Validity window has ended");
        lenientVerifier.verifyAttestationDocProof(fixture.publicValues, fixture.proof);
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
