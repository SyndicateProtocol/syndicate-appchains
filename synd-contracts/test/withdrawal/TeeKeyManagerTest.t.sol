// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {Test} from "forge-std/Test.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {TeeKeyManager} from "../../src/withdrawal/TeeKeyManager.sol";
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

    function test_InitialState() public {
        assertEq(address(keyManager.attestationDocVerifier()), address(mockVerifier), "Initial verifier mismatch");
        assertEq(keyManager.owner(), owner, "Initial owner mismatch");
        vm.expectRevert(abi.encodeWithSelector(TeeKeyManager.InvalidPublicKey.selector, teeKey1));
        keyManager.isKeyValid(teeKey1);
    }

    // --- Test addKey ---
    function test_AddKey_Success() public {
        vm.expectRevert(abi.encodeWithSelector(TeeKeyManager.InvalidPublicKey.selector, teeKey1));
        keyManager.isKeyValid(teeKey1);

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

        vm.expectRevert(abi.encodeWithSelector(TeeKeyManager.InvalidPublicKey.selector, teeKey1));
        keyManager.isKeyValid(teeKey1);

        vm.expectRevert(abi.encodeWithSelector(TeeKeyManager.InvalidPublicKey.selector, teeKey2));
        keyManager.isKeyValid(teeKey2);
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
        vm.expectRevert(abi.encodeWithSelector(TeeKeyManager.InvalidPublicKey.selector, teeKey1));
        keyManager.isKeyValid(teeKey1);

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

    function test_UpdateAttestationDocVerifier_WithZeroAddress() public {
        // Add a key first
        mockVerifier.setPublicKeyToReturn(teeKey1);
        keyManager.addKey(dummyPublicValues, dummyProofBytes);
        assertTrue(keyManager.isKeyValid(teeKey1), "teeKey1 should be valid before update");

        vm.startPrank(owner);
        // Should be able to set zero address (though not recommended)
        keyManager.updateAttestationDocVerifier(IAttestationDocVerifier(address(0)));
        vm.stopPrank();

        assertEq(address(keyManager.attestationDocVerifier()), address(0), "Verifier should be zero address");
        assertFalse(keyManager.isKeyValid(teeKey1), "teeKey1 should be invalid after update");
    }

    function test_UpdateAttestationDocVerifier_StateCleanup() public {
        // Add multiple keys
        mockVerifier.setPublicKeyToReturn(teeKey1);
        keyManager.addKey(dummyPublicValues, dummyProofBytes);
        mockVerifier.setPublicKeyToReturn(teeKey2);
        keyManager.addKey(abi.encode("publicValues2"), abi.encode("proofBytes2"));

        assertTrue(keyManager.isKeyValid(teeKey1), "teeKey1 should be valid before update");
        assertTrue(keyManager.isKeyValid(teeKey2), "teeKey2 should be valid before update");

        // Update verifier should revoke all keys
        MockAttestationDocVerifier newVerifier = new MockAttestationDocVerifier();

        vm.startPrank(owner);
        keyManager.updateAttestationDocVerifier(IAttestationDocVerifier(address(newVerifier)));
        vm.stopPrank();

        // All keys should be revoked
        assertFalse(keyManager.isKeyValid(teeKey1), "teeKey1 should be invalid after update");
        assertFalse(keyManager.isKeyValid(teeKey2), "teeKey2 should be invalid after update");

        // Should be able to add new keys with new verifier
        address newTeeKey = vm.addr(6);
        newVerifier.setPublicKeyToReturn(newTeeKey);
        keyManager.addKey(abi.encode("newPublicValues"), abi.encode("newProofBytes"));
        assertTrue(keyManager.isKeyValid(newTeeKey), "New key should be valid with new verifier");
    }

    function test_AddKey_WithMaliciousVerifier() public {
        // Test what happens when verifier returns unexpected addresses
        MockAttestationDocVerifier maliciousVerifier = new MockAttestationDocVerifier();

        vm.startPrank(owner);
        keyManager.updateAttestationDocVerifier(IAttestationDocVerifier(address(maliciousVerifier)));
        vm.stopPrank();

        // Verifier returns address(0) - should be allowed but not recommended
        maliciousVerifier.setPublicKeyToReturn(address(0));
        keyManager.addKey(dummyPublicValues, dummyProofBytes);
        assertTrue(keyManager.isKeyValid(address(0)), "Zero address should be valid if verifier returns it");

        // Verifier returns contract address - should be allowed
        maliciousVerifier.setPublicKeyToReturn(address(this));
        keyManager.addKey(abi.encode("publicValues2"), abi.encode("proofBytes2"));
        assertTrue(keyManager.isKeyValid(address(this)), "Contract address should be valid if verifier returns it");
    }

    function test_AddKey_DuplicateHandling() public {
        mockVerifier.setPublicKeyToReturn(teeKey1);
        keyManager.addKey(dummyPublicValues, dummyProofBytes);

        // Try adding same key again - should fail
        vm.expectRevert("TeeKeyManager: Key already exists or failed to add");
        keyManager.addKey(dummyPublicValues, dummyProofBytes);

        // Verify key is still valid
        assertTrue(keyManager.isKeyValid(teeKey1), "teeKey1 should still be valid");
    }

    function test_AddKey_VerifierRevertHandling() public {
        mockVerifier.setShouldRevert(true);

        // Should propagate the revert from verifier
        vm.expectRevert("MockAttestationDocVerifier: Forced revert");
        keyManager.addKey(dummyPublicValues, dummyProofBytes);
    }

    function test_RevokeAllKeys_EmptyState() public {
        // Test revoking keys when none exist
        vm.startPrank(owner);

        // Should not revert even if no keys exist
        vm.expectEmit(true, true, true, true, address(keyManager));
        emit KeysRevoked();
        keyManager.revokeAllKeys();

        vm.stopPrank();
    }

    function test_RevokeAllKeys_LargeKeySet() public {
        // Test with many keys to check gas efficiency
        address[] memory keys = new address[](10);
        for (uint256 i = 0; i < keys.length; i++) {
            keys[i] = vm.addr(i + 10);
            mockVerifier.setPublicKeyToReturn(keys[i]);
            keyManager.addKey(abi.encode("publicValues", i), abi.encode("proofBytes", i));
            assertTrue(keyManager.isKeyValid(keys[i]), "Key should be valid after adding");
        }

        // Revoke all keys
        vm.startPrank(owner);
        keyManager.revokeAllKeys();
        vm.stopPrank();

        // Verify all keys are revoked
        for (uint256 i = 0; i < keys.length; i++) {
            assertFalse(keyManager.isKeyValid(keys[i]), "Key should be invalid after revoke");
        }
    }

    function test_Constructor_WithZeroAddress() public {
        // Test constructor with zero address verifier
        vm.startPrank(owner);
        TeeKeyManager keyManagerWithZero = new TeeKeyManager(IAttestationDocVerifier(address(0)));
        vm.stopPrank();

        assertEq(address(keyManagerWithZero.attestationDocVerifier()), address(0), "Verifier should be zero address");
        assertFalse(keyManagerWithZero.isKeyValid(teeKey1), "No keys should be valid initially");
    }

    function test_isKeyValid_NonExistentKey() public {
        // Test checking validity of keys that were never added
        assertFalse(keyManager.isKeyValid(teeKey1), "Non-existent key should not be valid");
        assertFalse(keyManager.isKeyValid(address(0)), "Zero address should not be valid");
        assertFalse(keyManager.isKeyValid(address(this)), "Contract address should not be valid");
    }

    function test_AddKey_EdgeCaseAddresses() public {
        // Test with edge case addresses
        address maxAddress = address(type(uint160).max);
        address minAddress = address(1);

        // Test max address
        mockVerifier.setPublicKeyToReturn(maxAddress);
        keyManager.addKey(dummyPublicValues, dummyProofBytes);
        assertTrue(keyManager.isKeyValid(maxAddress), "Max address should be valid");

        // Test min address
        mockVerifier.setPublicKeyToReturn(minAddress);
        keyManager.addKey(abi.encode("publicValues2"), abi.encode("proofBytes2"));
        assertTrue(keyManager.isKeyValid(minAddress), "Min address should be valid");
    }

    function test_Ownership_SecurityProperties() public {
        // Test that ownership functions work correctly
        assertEq(keyManager.owner(), owner, "Owner should be correct");

        // Test ownership transfer (Ownable uses immediate transfer)
        vm.startPrank(owner);
        keyManager.transferOwnership(user1);
        vm.stopPrank();

        assertEq(keyManager.owner(), user1, "Owner should be transferred immediately");

        // Original owner should no longer be able to revoke keys
        vm.startPrank(owner);
        vm.expectRevert(abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, owner));
        keyManager.revokeAllKeys();
        vm.stopPrank();

        // New owner should be able to revoke keys
        vm.startPrank(user1);
        keyManager.revokeAllKeys();
        vm.stopPrank();
    }
}
