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
}

abstract contract BaseAttestationDocVerifierTest is Test {
    using stdJson for string;

    address verifier;
    AttestationDocVerifier public attestationDocVerifier;

    function loadFixture(
        string memory fixturePath
    ) public view returns (SP1ProofFixtureJson memory) {
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, fixturePath);
        string memory json = vm.readFile(path);

        SP1ProofFixtureJson memory fixture;
        fixture.proof = json.readBytes(".proof");
        fixture.publicValues = json.readBytes(".publicValues");
        fixture.vkey = json.readBytes32(".vkey");

        return fixture;
    }

    function getFixturePath() public pure virtual returns (string memory);

    function setUp() public virtual {
        SP1ProofFixtureJson memory fixture = loadFixture(getFixturePath());
        verifier = address(new SP1VerifierGateway(address(1)));
        attestationDocVerifier = new AttestationDocVerifier(
            verifier,
            fixture.vkey
        );
    }

    function test_ValidAttestationDocVerifierProof() public {
        SP1ProofFixtureJson memory fixture = loadFixture(getFixturePath());
        vm.warp(1748509951); // timestamp within the validity window

        vm.mockCall(
            verifier,
            abi.encodeWithSelector(SP1VerifierGateway.verifyProof.selector),
            abi.encode(true)
        );

        address publicKey = attestationDocVerifier.verifyAttestationDocProof(
            fixture.publicValues,
            fixture.proof
        );

        assert(
            publicKey == address(0x00F652c7f894F01CEa2eeaDA38C2423C4B2c1Ad8)
        );
    }

    function testRevert_InvalidAttestationDocVerifierProof() public {
        vm.expectRevert();

        SP1ProofFixtureJson memory fixture = loadFixture(getFixturePath());
        vm.warp(1748509951); // timestamp within the validity window

        // Create a fake proof.
        bytes memory fakeProof = new bytes(fixture.proof.length);

        attestationDocVerifier.verifyAttestationDocProof(
            fixture.publicValues,
            fakeProof
        );
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
