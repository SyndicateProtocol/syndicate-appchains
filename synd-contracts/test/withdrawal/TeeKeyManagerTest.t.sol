// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test, console} from "forge-std/Test.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {TeeKeyManager} from "../../src/withdrawal/TeeKeyManager.sol";
import {ITeeKeyManager} from "../../src/withdrawal/ITeeKeyManager.sol";
import {IAttestationDocVerifier} from "../../src/withdrawal/IAttestationDocVerifier.sol";

// Mock for IAttestationDocVerifier
contract MockAttestationDocVerifier is
    IAttestationDocVerifier // Implements interface
{
    address public mockPublicKeyToReturn;
    bool public shouldRevert;

    function verifyAttestationDocProof(bytes calldata, /*_publicValues*/ bytes calldata /*_proofBytes*/ )
        external
        view
        override
        returns (address)
    {
        if (shouldRevert) {
            revert("MockAttestationDocVerifier: Forced revert");
        }
        return mockPublicKeyToReturn;
    }

    function setPublicKeyToReturn(address _key) external {
        mockPublicKeyToReturn = _key;
    }

    function setShouldRevert(bool _revert) external {
        shouldRevert = _revert;
    }
}

contract TeeKeyManagerTest is Test {
    TeeKeyManager internal keyManager;
    MockAttestationDocVerifier internal mockVerifier;

    address internal owner = vm.addr(1);
    address internal user1 = vm.addr(2);
    address internal user2 = vm.addr(3);
    address internal teeKey1 = vm.addr(4);
    address internal teeKey2 = vm.addr(5);

    bytes internal dummyPublicValues = abi.encode("publicValues");
    bytes internal dummyProofBytes = abi.encode("proofBytes");

    event KeyAdded(address indexed key);
    event KeysRevoked();

    function setUp() public {
        vm.startPrank(owner);
        mockVerifier = new MockAttestationDocVerifier();
        keyManager = new TeeKeyManager(IAttestationDocVerifier(address(mockVerifier)));
        vm.stopPrank();
    }

    function test_InitialState() public view {
        assertEq(address(keyManager.attestationDocVerifier()), address(mockVerifier), "Initial verifier mismatch");
        assertEq(keyManager.owner(), owner, "Initial owner mismatch");
        assertFalse(keyManager.isKeyValid(teeKey1), "Key should not be valid initially");
    }

    // --- Test addKey ---
    function test_AddKey_Success() public {
        assertFalse(keyManager.isKeyValid(teeKey1), "Key should initially be invalid");
        mockVerifier.setPublicKeyToReturn(teeKey1);

        vm.expectEmit(true, true, true, true, address(keyManager));
        emit KeyAdded(teeKey1);
        keyManager.addKey(dummyPublicValues, dummyProofBytes);

        assertTrue(keyManager.isKeyValid(teeKey1), "teeKey1 should be valid");
    }

    function test_AddKey_FailsIfKeyAlreadyExists() public {
        mockVerifier.setPublicKeyToReturn(teeKey1);
        keyManager.addKey(dummyPublicValues, dummyProofBytes);

        vm.expectRevert("TeeKeyManager: Key already exists or failed to add");
        keyManager.addKey(dummyPublicValues, dummyProofBytes);
    }

    function test_AddKey_FailsIfVerifierReverts() public {
        mockVerifier.setShouldRevert(true);
        vm.expectRevert("MockAttestationDocVerifier: Forced revert");
        keyManager.addKey(dummyPublicValues, dummyProofBytes);
    }

    // --- Test revokeAllKeys ---
    function test_RevokeAllKeys_Success() public {
        // Add a couple of keys
        mockVerifier.setPublicKeyToReturn(teeKey1);
        keyManager.addKey(dummyPublicValues, dummyProofBytes);
        mockVerifier.setPublicKeyToReturn(teeKey2);
        keyManager.addKey(abi.encode("publicValues2"), abi.encode("proofBytes2"));

        assertTrue(keyManager.isKeyValid(teeKey1), "teeKey1 should be valid before revoke");
        assertTrue(keyManager.isKeyValid(teeKey2), "teeKey2 should be valid before revoke");

        vm.startPrank(owner);
        vm.expectEmit(true, true, true, true, address(keyManager));
        emit KeysRevoked();
        keyManager.revokeAllKeys();
        vm.stopPrank();

        assertFalse(keyManager.isKeyValid(teeKey1), "teeKey1 should be invalid after revoke");
        assertFalse(keyManager.isKeyValid(teeKey2), "teeKey2 should be invalid after revoke");
    }

    function test_RevokeAllKeys_FailsIfNotOwner() public {
        vm.startPrank(user1);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, user1));
        keyManager.revokeAllKeys();
        vm.stopPrank();
    }

    function test_RevokeAllKeys_WhenNoKeysExist() public {
        vm.startPrank(owner);
        vm.expectEmit(true, true, true, true, address(keyManager));
        emit KeysRevoked();
        keyManager.revokeAllKeys();
        vm.stopPrank();
        // No assertion needed other than it doesn't revert and emits event
    }

    // --- Test updateAttestationDocVerifier ---
    function test_UpdateAttestationDocVerifier_Success() public {
        // Add a key first
        mockVerifier.setPublicKeyToReturn(teeKey1);
        keyManager.addKey(dummyPublicValues, dummyProofBytes);
        assertTrue(keyManager.isKeyValid(teeKey1), "teeKey1 should be valid before update");

        MockAttestationDocVerifier newMockVerifier = new MockAttestationDocVerifier();

        vm.startPrank(owner);
        // Expect KeysRevoked because updateAttestationDocVerifier calls revokeAllKeys
        vm.expectEmit(true, true, true, true, address(keyManager));
        emit KeysRevoked();
        keyManager.updateAttestationDocVerifier(IAttestationDocVerifier(address(newMockVerifier)));
        vm.stopPrank();

        assertEq(address(keyManager.attestationDocVerifier()), address(newMockVerifier), "Verifier address not updated");
        assertFalse(keyManager.isKeyValid(teeKey1), "teeKey1 should be invalid after verifier update (keys revoked)");

        // Test adding a key with the new verifier
        address newTeeKey = vm.addr(6);
        newMockVerifier.setPublicKeyToReturn(newTeeKey);

        vm.expectEmit(true, true, true, true, address(keyManager));
        emit KeyAdded(newTeeKey);
        keyManager.addKey(abi.encode("publicValuesNew"), abi.encode("proofBytesNew"));
        assertTrue(keyManager.isKeyValid(newTeeKey), "New key should be valid with new verifier");
    }

    function test_UpdateAttestationDocVerifier_FailsIfNotOwner() public {
        MockAttestationDocVerifier newMockVerifier = new MockAttestationDocVerifier();
        vm.startPrank(user1);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, user1));
        keyManager.updateAttestationDocVerifier(IAttestationDocVerifier(address(newMockVerifier)));
        vm.stopPrank();
    }
}
