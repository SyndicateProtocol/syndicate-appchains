// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {AttestationDocVerifier} from "../../src/withdrawal/AttestationDocVerifier.sol";
import {SP1VerifierGateway} from "@sp1-contracts/SP1VerifierGateway.sol";

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

    address verifier;
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

    function setUp() public virtual {
        SP1ProofFixtureJson memory fixture = loadFixture(getFixturePath());
        verifier = address(new SP1VerifierGateway(address(1)));
        attestationDocVerifier = new AttestationDocVerifier(
            verifier, fixture.vkey, fixture.rootCertHash, fixture.pcr0, fixture.pcr1, fixture.pcr2, fixture.pcr8, 0
        );
    }

    function test_ValidAttestationDocVerifierProof() public {
        SP1ProofFixtureJson memory fixture = loadFixture(getFixturePath());
        vm.warp(1748509951); // timestamp within the validity window

        vm.mockCall(verifier, abi.encodeWithSelector(SP1VerifierGateway.verifyProof.selector), abi.encode(true));

        address publicKey = attestationDocVerifier.verifyAttestationDocProof(fixture.publicValues, fixture.proof);

        assert(publicKey == address(0x0BD6f0f44257D315C16E3d67835F8d41BD11377E));
    }

    function testRevert_InvalidAttestationDocVerifierProof() public {
        vm.expectRevert();

        SP1ProofFixtureJson memory fixture = loadFixture(getFixturePath());
        vm.warp(1748509951); // timestamp within the validity window

        // Create a fake proof.
        bytes memory fakeProof = new bytes(fixture.proof.length);

        attestationDocVerifier.verifyAttestationDocProof(fixture.publicValues, fakeProof);
    }
}

contract AttestationDocVerifierGroth16Test is BaseAttestationDocVerifierTest {
    function getFixturePath() public pure override returns (string memory) {
        return "/test/withdrawal/fixtures/groth16-fixture.json";
    }
}

contract AttestationDocVerifierPlonkTest is BaseAttestationDocVerifierTest {
    function getFixturePath() public pure override returns (string memory) {
        return "/test/withdrawal/fixtures/plonk-fixture.json";
    }
}
