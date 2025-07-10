///Module containing a contract's types and functions.
/**

```solidity
library Checkpoints {
    struct Checkpoint208 { uint48 _key; uint208 _value; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod Checkpoints {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Checkpoint208 { uint48 _key; uint208 _value; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Checkpoint208 {
        #[allow(missing_docs)]
        pub _key: alloy::sol_types::private::primitives::aliases::U48,
        #[allow(missing_docs)]
        pub _value: alloy::sol_types::private::primitives::aliases::U208,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<48>,
            alloy::sol_types::sol_data::Uint<208>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U48,
            alloy::sol_types::private::primitives::aliases::U208,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Checkpoint208> for UnderlyingRustTuple<'_> {
            fn from(value: Checkpoint208) -> Self {
                (value._key, value._value)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Checkpoint208 {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    _key: tuple.0,
                    _value: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Checkpoint208 {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Checkpoint208 {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::tokenize(&self._key),
                    <alloy::sol_types::sol_data::Uint<
                        208,
                    > as alloy_sol_types::SolType>::tokenize(&self._value),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Checkpoint208 {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Checkpoint208 {
            const NAME: &'static str = "Checkpoint208";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Checkpoint208(uint48 _key,uint208 _value)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self._key)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        208,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self._value)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Checkpoint208 {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust._key)
                    + <alloy::sol_types::sol_data::Uint<
                        208,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust._value,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    48,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust._key,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    208,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust._value,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`Checkpoints`](self) contract instance.

See the [wrapper's documentation](`CheckpointsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> CheckpointsInstance<P, N> {
        CheckpointsInstance::<P, N>::new(address, provider)
    }
    /**A [`Checkpoints`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Checkpoints`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct CheckpointsInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for CheckpointsInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("CheckpointsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > CheckpointsInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`Checkpoints`](self) contract instance.

See the [wrapper's documentation](`CheckpointsInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> CheckpointsInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> CheckpointsInstance<P, N> {
            CheckpointsInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > CheckpointsInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > CheckpointsInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library Checkpoints {
    struct Checkpoint208 {
        uint48 _key;
        uint208 _value;
    }
}

interface SyndicateToken {
    error AccessControlBadConfirmation();
    error AccessControlUnauthorizedAccount(address account, bytes32 neededRole);
    error BurnOnlyDuringLockPeriod();
    error CheckpointUnorderedInsertion();
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error ERC20ExceededSafeSupply(uint256 increasedSupply, uint256 cap);
    error ERC20InsufficientAllowance(address spender, uint256 allowance, uint256 needed);
    error ERC20InsufficientBalance(address sender, uint256 balance, uint256 needed);
    error ERC20InvalidApprover(address approver);
    error ERC20InvalidReceiver(address receiver);
    error ERC20InvalidSender(address sender);
    error ERC20InvalidSpender(address spender);
    error ERC2612ExpiredSignature(uint256 deadline);
    error ERC2612InvalidSigner(address signer, address owner);
    error ERC5805FutureLookup(uint256 timepoint, uint48 clock);
    error ERC6372InconsistentClock();
    error ExceedsTotalSupply();
    error InvalidAccountNonce(address account, uint256 currentNonce);
    error InvalidShortString();
    error SafeCastOverflowedUintDowncast(uint8 bits, uint256 value);
    error StringTooLong(string str);
    error TransfersLocked();
    error UnlockTimestampInPast();
    error UnlockTimestampTooLate();
    error VotesExpiredSignature(uint256 expiry);
    error ZeroAddress();
    error ZeroAmount();

    event Approval(address indexed owner, address indexed spender, uint256 value);
    event DelegateChanged(address indexed delegator, address indexed fromDelegate, address indexed toDelegate);
    event DelegateVotesChanged(address indexed delegate, uint256 previousVotes, uint256 newVotes);
    event EIP712DomainChanged();
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    event TokensBurnedByManager(address indexed from, uint256 amount, address indexed burner);
    event Transfer(address indexed from, address indexed to, uint256 value);
    event UnlockTimestampUpdated(uint256 oldTimestamp, uint256 newTimestamp, address indexed updatedBy);

    constructor(address defaultAdmin, address syndTreasuryAddress);

    function AIRDROP_MANAGER_ROLE() external view returns (bytes32);
    function CLOCK_MODE() external view returns (string memory);
    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
    function DOMAIN_SEPARATOR() external view returns (bytes32);
    function EMISSION_MINTER_ROLE() external view returns (bytes32);
    function INITIAL_MINT_SUPPLY() external view returns (uint256);
    function MAX_LOCK_DURATION() external view returns (uint256);
    function TOTAL_SUPPLY() external view returns (uint256);
    function allowance(address owner, address spender) external view returns (uint256);
    function approve(address spender, uint256 value) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
    function burn(uint256 amount) external;
    function burnFrom(address from, uint256 amount) external;
    function checkpoints(address account, uint32 pos) external view returns (Checkpoints.Checkpoint208 memory);
    function clock() external view returns (uint48);
    function decimals() external view returns (uint8);
    function delegate(address delegatee) external;
    function delegateBySig(address delegatee, uint256 nonce, uint256 expiry, uint8 v, bytes32 r, bytes32 s) external;
    function delegates(address account) external view returns (address);
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function getCurrentTotalSupply() external view returns (uint256);
    function getPastTotalSupply(uint256 timepoint) external view returns (uint256);
    function getPastVotes(address account, uint256 timepoint) external view returns (uint256);
    function getPastVotingPower(address account, uint256 blockNumber) external view returns (uint256);
    function getRemainingLockTime() external view returns (uint256);
    function getRoleAdmin(bytes32 role) external view returns (bytes32);
    function getVotes(address account) external view returns (uint256);
    function getVotingPower(address account) external view returns (uint256);
    function grantRole(bytes32 role, address account) external;
    function hasRole(bytes32 role, address account) external view returns (bool);
    function maxLockTimestamp() external view returns (uint256);
    function mint(address to, uint256 amount) external;
    function name() external view returns (string memory);
    function nonces(address owner) external view returns (uint256);
    function numCheckpoints(address account) external view returns (uint32);
    function permit(address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external;
    function renounceRole(bytes32 role, address callerConfirmation) external;
    function revokeRole(bytes32 role, address account) external;
    function setUnlockTimestamp(uint256 newUnlockTimestamp) external;
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function symbol() external view returns (string memory);
    function totalSupply() external view returns (uint256);
    function transfer(address to, uint256 value) external returns (bool);
    function transferFrom(address from, address to, uint256 value) external returns (bool);
    function transfersLocked() external view returns (bool);
    function unlockTimestamp() external view returns (uint256);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "defaultAdmin",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "syndTreasuryAddress",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "AIRDROP_MANAGER_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "CLOCK_MODE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "DEFAULT_ADMIN_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "DOMAIN_SEPARATOR",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "EMISSION_MINTER_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "INITIAL_MINT_SUPPLY",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "MAX_LOCK_DURATION",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "TOTAL_SUPPLY",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "allowance",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "approve",
    "inputs": [
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "balanceOf",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "burn",
    "inputs": [
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "burnFrom",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "checkpoints",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "pos",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct Checkpoints.Checkpoint208",
        "components": [
          {
            "name": "_key",
            "type": "uint48",
            "internalType": "uint48"
          },
          {
            "name": "_value",
            "type": "uint208",
            "internalType": "uint208"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "clock",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint48",
        "internalType": "uint48"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "decimals",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delegate",
    "inputs": [
      {
        "name": "delegatee",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegateBySig",
    "inputs": [
      {
        "name": "delegatee",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "nonce",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "expiry",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "v",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "r",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegates",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eip712Domain",
    "inputs": [],
    "outputs": [
      {
        "name": "fields",
        "type": "bytes1",
        "internalType": "bytes1"
      },
      {
        "name": "name",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "version",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "chainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "verifyingContract",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "extensions",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCurrentTotalSupply",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPastTotalSupply",
    "inputs": [
      {
        "name": "timepoint",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPastVotes",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "timepoint",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPastVotingPower",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "blockNumber",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRemainingLockTime",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRoleAdmin",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getVotes",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getVotingPower",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "grantRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "hasRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "maxLockTimestamp",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "mint",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "name",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "nonces",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "numCheckpoints",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "permit",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "v",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "r",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "callerConfirmation",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "revokeRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setUnlockTimestamp",
    "inputs": [
      {
        "name": "newUnlockTimestamp",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "supportsInterface",
    "inputs": [
      {
        "name": "interfaceId",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "symbol",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "totalSupply",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "transfer",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferFrom",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transfersLocked",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "unlockTimestamp",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "Approval",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "spender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DelegateChanged",
    "inputs": [
      {
        "name": "delegator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "fromDelegate",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "toDelegate",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DelegateVotesChanged",
    "inputs": [
      {
        "name": "delegate",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "previousVotes",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newVotes",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EIP712DomainChanged",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleAdminChanged",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "previousAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "newAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleGranted",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleRevoked",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TokensBurnedByManager",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "burner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Transfer",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UnlockTimestampUpdated",
    "inputs": [
      {
        "name": "oldTimestamp",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newTimestamp",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "updatedBy",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AccessControlBadConfirmation",
    "inputs": []
  },
  {
    "type": "error",
    "name": "AccessControlUnauthorizedAccount",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "neededRole",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "BurnOnlyDuringLockPeriod",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CheckpointUnorderedInsertion",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureLength",
    "inputs": [
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureS",
    "inputs": [
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20ExceededSafeSupply",
    "inputs": [
      {
        "name": "increasedSupply",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "cap",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20InsufficientAllowance",
    "inputs": [
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "allowance",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "needed",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20InsufficientBalance",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "balance",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "needed",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20InvalidApprover",
    "inputs": [
      {
        "name": "approver",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20InvalidReceiver",
    "inputs": [
      {
        "name": "receiver",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20InvalidSender",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC20InvalidSpender",
    "inputs": [
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC2612ExpiredSignature",
    "inputs": [
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC2612InvalidSigner",
    "inputs": [
      {
        "name": "signer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC5805FutureLookup",
    "inputs": [
      {
        "name": "timepoint",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "clock",
        "type": "uint48",
        "internalType": "uint48"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC6372InconsistentClock",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ExceedsTotalSupply",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidAccountNonce",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "currentNonce",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidShortString",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SafeCastOverflowedUintDowncast",
    "inputs": [
      {
        "name": "bits",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "StringTooLong",
    "inputs": [
      {
        "name": "str",
        "type": "string",
        "internalType": "string"
      }
    ]
  },
  {
    "type": "error",
    "name": "TransfersLocked",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UnlockTimestampInPast",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UnlockTimestampTooLate",
    "inputs": []
  },
  {
    "type": "error",
    "name": "VotesExpiredSignature",
    "inputs": [
      {
        "name": "expiry",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ZeroAddress",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroAmount",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod SyndicateToken {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610180604052346100845761001b610015610158565b906103bd565b610023610089565b614b42611c83823960805181612923015260a0518161295a015260c051816128ea015260e051816133300152610100518161335501526101205181612e9701526101405181612ed7015261016051818181610b2c0152611f700152614b4290f35b61008f565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b906100bb90610093565b810190811060018060401b038211176100d357604052565b61009d565b906100eb6100e4610089565b92836100b1565b565b5f80fd5b60018060a01b031690565b610105906100f1565b90565b610111816100fc565b0361011857565b5f80fd5b9050519061012982610108565b565b91906040838203126101535780610147610150925f860161011c565b9360200161011c565b90565b6100ed565b6101766167c58038038061016b816100d8565b92833981019061012b565b9091565b60018060401b03811161019657610192602091610093565b0190565b61009d565b906101ad6101a88361017a565b6100d8565b918252565b5f7f53796e6469636174650000000000000000000000000000000000000000000000910152565b6101e3600961019b565b906101f0602083016101b2565b565b6101fa6101d9565b90565b5f7f53594e4400000000000000000000000000000000000000000000000000000000910152565b61022e600461019b565b9061023b602083016101fd565b565b610245610224565b90565b90565b90565b61026261025d61026792610248565b61024b565b6100f1565b90565b6102739061024e565b90565b5f0190565b90565b90565b61029561029061029a9261027b565b61024b565b61027e565b90565b6102a9629e3400610281565b90565b634e487b7160e01b5f52601160045260245ffd5b6102cf6102d59193929361027e565b9261027e565b82018092116102e057565b6102ac565b6102f96102f46102fe92610248565b61024b565b61027e565b90565b5f1b90565b906103125f1991610301565b9181191691161790565b61033061032b6103359261027e565b61024b565b61027e565b90565b90565b9061035061034b6103579261031c565b610338565b8254610306565b9055565b90565b61037261036d61037792610248565b610301565b61035b565b90565b6103835f61035e565b90565b90565b61039d6103986103a292610386565b61024b565b61027e565b90565b6103ba6b02e87669c308736a04000000610389565b90565b906103df6103c96101f2565b6103d16101f2565b6103d961023d565b916104a5565b816103fa6103f46103ef5f61026a565b6100fc565b916100fc565b14610489578061041a61041461040f5f61026a565b6100fc565b916100fc565b1461046d5761045c61046b926104384261043261029d565b906102c0565b6101605261044f6104485f6102e5565b600c61033b565b61045761037a565b610941565b506104656103a5565b90610a0f565b565b5f63d92e233d60e01b81528061048560048201610276565b0390fd5b5f63d92e233d60e01b8152806104a160048201610276565b0390fd5b906104b092916104b2565b565b906104bd92916104bf565b565b906104ca92916104cc565b565b906104d792916104d9565b565b906104e49291610531565b565b5f7f3100000000000000000000000000000000000000000000000000000000000000910152565b610517600161019b565b90610524602083016104e6565b565b61052e61050d565b90565b90610545929161053f610526565b90610547565b565b90610553939291610599565b565b90565b90565b60200190565b5190565b61057961057461057e926100f1565b61024b565b6100f1565b90565b61058a90610565565b90565b61059690610581565b90565b6105aa6105fa946105df939461062e565b6105be816105b86006610555565b90610abc565b610120526105d6836105d06007610555565b90610abc565b61014052610558565b6105f16105eb82610561565b9161055b565b2060e052610558565b61060c61060682610561565b9161055b565b20610100524660a05261061d610bc1565b6080526106293061058d565b60c052565b906106389161063a565b565b9061064491610646565b565b9061065091610897565b565b634e487b7160e01b5f525f60045260245ffd5b5190565b634e487b7160e01b5f52602260045260245ffd5b906001600283049216801561069d575b602083101461069857565b610669565b91607f169161068d565b5f5260205f2090565b601f602091010490565b1b90565b919060086106d99102916106d35f19846106ba565b926106ba565b9181191691161790565b91906106f96106f46107019361031c565b610338565b9083546106be565b9055565b5f90565b61071b91610715610705565b916106e3565b565b5b818110610729575050565b806107365f600193610709565b0161071e565b9190601f811161074c575b505050565b61075861077d936106a7565b906020610764846106b0565b83019310610785575b610776906106b0565b019061071d565b5f8080610747565b91506107768192905061076d565b1c90565b906107a7905f1990600802610793565b191690565b816107b691610797565b906002021790565b906107c881610665565b9060018060401b038211610886576107ea826107e4855461067d565b8561073c565b602090601f831160011461081e5791809161080d935f92610812575b50506107ac565b90555b565b90915001515f80610806565b601f1983169161082d856106a7565b925f5b81811061086e57509160029391856001969410610854575b50505002019055610810565b610864910151601f841690610797565b90555f8080610848565b91936020600181928787015181550195019201610830565b61009d565b90610895916107be565b565b906108a66108ad92600361088b565b600461088b565b565b5f90565b151590565b6108c19061035b565b90565b906108ce906108b8565b5f5260205260405f2090565b6108e390610581565b90565b906108f0906108da565b5f5260205260405f2090565b9061090860ff91610301565b9181191691161790565b61091b906108b3565b90565b90565b9061093661093161093d92610912565b61091e565b82546108fc565b9055565b6109496108af565b5061095e610958828490610c5e565b156108b3565b5f146109e75761098660016109815f610979600586906108c4565b0185906108e6565b610921565b9061098f610c8c565b906109cc6109c66109c07f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d956108b8565b926108da565b926108da565b926109d5610089565b806109df81610276565b0390a4600190565b50505f90565b6109f6906100fc565b9052565b9190610a0d905f602085019401906109ed565b565b80610a2a610a24610a1f5f61026a565b6100fc565b916100fc565b14610a4657610a4491610a3c5f61026a565b919091610cbd565b565b610a69610a525f61026a565b5f91829163ec442f0560e01b8352600483016109fa565b0390fd5b5f90565b90565b610a88610a83610a8d92610a71565b61024b565b61027e565b90565b90565b610aa7610aa2610aac92610a90565b610301565b61035b565b90565b610ab960ff610a93565b90565b90610ac5610a6d565b50610ad7610ad283610558565b610561565b610aea610ae46020610a74565b9161027e565b105f14610afe5750610afb90610e57565b90565b5f610b0c610b129392610d67565b0161088b565b610b22610b1d610aaf565b6108b8565b90565b5f90565b7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f90565b610b57905161035b565b90565b610b639061035b565b9052565b610b709061027e565b9052565b90959492610bbf94610bae610bb892610ba4608096610b9a60a088019c5f890190610b5a565b6020870190610b5a565b6040850190610b5a565b6060830190610b67565b01906109ed565b565b610bc9610b25565b50610bd2610b29565b610c1c610bdf60e0610b4d565b91610c0d610bee610100610b4d565b46610bf83061058d565b91610c01610089565b96879560208701610b74565b602082018103825203826100b1565b610c2e610c2882610561565b9161055b565b2090565b5f1c90565b60ff1690565b610c49610c4e91610c32565b610c37565b90565b610c5b9054610c3d565b90565b610c85915f610c7a610c8093610c726108af565b5060056108c4565b016108e6565b610c51565b90565b5f90565b610c94610c88565b503390565b7f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a67490565b9182610cd9610cd3610cce5f61026a565b6100fc565b916100fc565b141580610d44575b610cf4575b610cf292919091610f7b565b565b610cfc610f05565b80610d23575b15610ce6575f6336e278fd60e21b815280610d1f60048201610276565b0390fd5b50610d3f610d39610d32610c99565b3390610c5e565b156108b3565b610d02565b5081610d60610d5a610d555f61026a565b6100fc565b916100fc565b1415610ce1565b90565b90565b610d81610d7c610d8692610d6a565b61024b565b61027e565b90565b60209181520190565b90825f9392825e0152565b610dbc610dc5602093610dca93610db381610665565b93848093610d89565b95869101610d92565b610093565b0190565b610de39160208201915f818403910152610d9d565b90565b610e00610dfb610df583610561565b9261055b565b610b4d565b9060208110610e0e575b5090565b610e20905f19906020036008026106ba565b165f610e0a565b610e33610e3891610c32565b61031c565b90565b610e4f610e4a610e549261027e565b610301565b61035b565b90565b610e5f610a6d565b50610e6981610558565b90610e7382610561565b610e86610e80601f610d6d565b9161027e565b11610ebb5750610eb381610ead610ea7610ea2610eb895610de6565b610e27565b91610561565b17610e3b565b6108b8565b90565b610edd90610ec7610089565b91829163305a27a960e01b835260048301610dce565b0390fd5b90565b610ef0610ef591610c32565b610ee1565b90565b610f029054610ee4565b90565b610f0d6108af565b50610f18600c610ef8565b610f2a610f245f6102e5565b9161027e565b141580610f35575b90565b5042610f52610f4c610f47600c610ef8565b61027e565b9161027e565b10610f32565b916020610f79929493610f7260408201965f830190610b67565b0190610b67565b565b9291610f8984838391611084565b83610fa4610f9e610f995f61026a565b6100fc565b916100fc565b14610fb9575b610fb79293919091611251565b565b610fc16111f3565b93610fca611230565b9480610fde610fd88861027e565b9161027e565b11610feb57509350610faa565b85906110075f928392630e58ae9360e11b845260048401610f58565b0390fd5b90611015906108da565b5f5260205260405f2090565b60409061104a611051949695939661104060608401985f8501906109ed565b6020830190610b67565b0190610b67565b565b9061105e910361027e565b90565b9061106c910161027e565b90565b9190611082905f60208501940190610b67565b565b919091806110a261109c6110975f61026a565b6100fc565b916100fc565b145f14611183576110c66110bf836110ba6002610ef8565b6102c0565b600261033b565b5b826110e26110dc6110d75f61026a565b6100fc565b916100fc565b145f14611157576111066110ff836110fa6002610ef8565b611053565b600261033b565b5b91909161115261114061113a7fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef936108da565b936108da565b93611149610089565b9182918261106f565b0390a3565b61117e826111786111695f879061100b565b9161117383610ef8565b611061565b9061033b565b611107565b6111966111915f839061100b565b610ef8565b806111a96111a38561027e565b9161027e565b106111d1576111bc6111cc918490611053565b6111c75f849061100b565b61033b565b6110c7565b906111ef9091925f93849363391434e360e21b855260048501611021565b0390fd5b6111fb610705565b506112066002610ef8565b90565b60018060d01b031690565b61122861122361122d92611209565b61024b565b61027e565b90565b611238610705565b5061124860018060d01b03611214565b90565b90565b90565b916112a96112a36112b0948061127761127161126c5f61026a565b6100fc565b916100fc565b146112e1575b8461129861129261128d5f61026a565b6100fc565b916100fc565b146112b2575b6114d9565b926114d9565b909161150e565b565b6112da600b60026112d46112ce6112c8896113c3565b9361124b565b9161124e565b90611416565b505061129e565b611309600b60016113036112fd6112f7896113c3565b9361124b565b9161124e565b90611416565b505061127d565b5f90565b61132061132691611209565b91611209565b019060018060d01b03821161133757565b6102ac565b9061134f91611349611310565b50611314565b90565b90565b60ff1690565b61136f61136a61137492611352565b61024b565b611355565b90565b6113809061135b565b9052565b9160206113a592949361139e60408201965f830190611377565b0190610b67565b565b6113bb6113b66113c09261027e565b61024b565b611209565b90565b6113cb611310565b50806113e56113df60018060d01b03611214565b9161027e565b116113f6576113f3906113a7565b90565b60d06114125f9283926306dfcc6560e41b845260048401611384565b0390fd5b9061144c6114529392611427611310565b50611430611310565b50809361144561143e6116c0565b949261176d565b9091611c53565b916117e2565b91909190565b61146461146a91611209565b91611209565b90039060018060d01b03821161147c57565b6102ac565b906114949161148e611310565b50611458565b90565b906114a1906108da565b5f5260205260405f2090565b60018060a01b031690565b6114c46114c991610c32565b6114ad565b90565b6114d690546114b8565b90565b6114f06114f5916114e8610c88565b506009611497565b6114cc565b90565b90611502906108da565b5f5260205260405f2090565b9190918061152461151e856100fc565b916100fc565b1415806116a2575b611536575b505050565b8061155161154b6115465f61026a565b6100fc565b916100fc565b03611612575b508161157361156d6115685f61026a565b6100fc565b916100fc565b0361157f575b80611531565b6115c66115b96115c092611595600a86906114f8565b906115b36115ad6115a76001936113c3565b9361124b565b9161124e565b90611416565b9290611214565b91611214565b9190916115f37fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926108da565b926116086115ff610089565b92839283610f58565b0390a25f80611579565b61165161165761164a611627600a85906114f8565b600261164461163e611638896113c3565b9361124b565b9161124e565b90611416565b9290611214565b91611214565b9190916116847fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926108da565b92611699611690610089565b92839283610f58565b0390a25f611557565b50816116b66116b05f6102e5565b9161027e565b1161152c565b5f90565b6116c86116bc565b506116d1611811565b90565b5490565b90565b6116ef6116ea6116f4926116d8565b61024b565b61027e565b90565b61170661170c9193929361027e565b9261027e565b820391821161171757565b6102ac565b90565b60301c90565b60018060d01b031690565b61173c6117419161171f565b611725565b90565b61174e9054611730565b90565b61176561176061176a92610248565b61024b565b611209565b90565b611775611310565b506117815f82016116d4565b8061179461178e5f6102e5565b9161027e565b145f146117aa5750506117a65f611751565b5b90565b6117d75f916117d26117cc846117dd9601926117c660016116db565b906116f7565b9161171c565b611826565b01611744565b6117a7565b916118065f61180b946117f3611310565b506117fc611310565b500192919261171c565b611a2b565b91909190565b6118196116bc565b5061182343611bec565b90565b5f5260205f200190565b5490565b61183e60406100d8565b90565b65ffffffffffff1690565b9061185690611841565b9052565b9061186490611209565b9052565b5f5260205f2090565b634e487b7160e01b5f52603260045260245ffd5b61188e81611830565b8210156118a8576118a0600191611868565b910201905f90565b611871565b6118b79051611841565b90565b906118cb65ffffffffffff91610301565b9181191691161790565b6118e96118e46118ee92611841565b61024b565b611841565b90565b90565b90611909611904611910926118d5565b6118f1565b82546118ba565b9055565b61191e9051611209565b90565b60301b90565b9061193965ffffffffffff1991611921565b9181191691161790565b61195761195261195c92611209565b61024b565b611209565b90565b90565b9061197761197261197e92611943565b61195f565b8254611927565b9055565b906119ac60205f6119b2946119a482820161199e8488016118ad565b906118f4565b019201611914565b90611962565b565b91906119c5576119c391611982565b565b610652565b90815491680100000000000000008310156119fa57826119f29160016119f895018155611885565b906119b4565b565b61009d565b65ffffffffffff1690565b611a16611a1b91610c32565b6119ff565b90565b611a289054611a0a565b90565b90929192611a37611310565b50611a40611310565b50611a4a82611830565b80611a5d611a575f6102e5565b9161027e565b115f14611b2d57611a8390611a7d8491611a7760016116db565b906116f7565b90611826565b90611a8f5f8301611a1e565b92611a9b5f8401611744565b9380611aaf611aa985611841565b91611841565b11611b1157611ac6611ac084611841565b91611841565b145f14611ae1575050611adc905f859101611962565b5b9190565b611b0c9250611b0786611afe611af5611834565b945f860161184c565b6020840161185a565b6119ca565b611add565b5f632520601d60e01b815280611b2960048201610276565b0390fd5b50611b5891611b5385611b4a611b41611834565b945f860161184c565b6020840161185a565b6119ca565b611b615f611751565b9190565b611b79611b74611b7e92611841565b61024b565b61027e565b90565b90565b611b98611b93611b9d92611b81565b61024b565b611355565b90565b611ba990611b84565b9052565b916020611bce929493611bc760408201965f830190611ba0565b0190610b67565b565b611be4611bdf611be99261027e565b61024b565b611841565b90565b611bf46116bc565b5080611c0e611c0865ffffffffffff611b65565b9161027e565b11611c1f57611c1c90611bd0565b90565b6030611c3b5f9283926306dfcc6560e41b845260048401611bad565b0390fd5b634e487b7160e01b5f52605160045260245ffd5b91909180600114611c7257600203611c3f57611c6e91611481565b905b565b50611c7c9161133c565b90611c7056fe60806040526004361015610013575b61148e565b61001d5f356102fc565b806301ffc9a7146102f757806306fdde03146102f2578063095ea7b3146102ed57806318160ddd146102e857806323b872dd146102e3578063248a9ca3146102de5780632f2ff15d146102d9578063313ce567146102d45780633644e515146102cf57806336568abe146102ca5780633a46b1a8146102c557806340c10f19146102c057806342966c68146102bb5780634bf5d7e9146102b65780634f1bfc9e146102b1578063587cde1e146102ac5780635c19a95c146102a75780636fcfff45146102a257806370a082311461029d57806379cc6790146102985780637a8cd156146102935780637ecebe001461028e57806383f1211b146102895780638426adf214610284578063844c90261461027f57806384b0196e1461027a5780638a542521146102755780638d3343d6146102705780638e539e8c1461026b578063902d55a51461026657806391d148541461026157806391ddadf41461025c57806395d89b41146102575780639ab24eb0146102525780639b7ef64b1461024d578063a217fddf14610248578063a9059cbb14610243578063aa082a9d1461023e578063b0ca253e14610239578063bb4d443614610234578063c02ae7541461022f578063c3cda5201461022a578063d505accf14610225578063d547741f14610220578063dd62ed3e1461021b5763f1127ed80361000e57611458565b611374565b611313565b6112d9565b61122f565b611173565b61113e565b611108565b6110d3565b611061565b61102c565b610fbc565b610f45565b610f10565b610edb565b610e78565b610e43565b610dcc565b610d97565b610d33565b610cc8565b610b83565b610b4e565b610af5565b610ac0565b610a8b565b610a57565b610a22565b6109ed565b61098f565b61095a565b6108e5565b610874565b610841565b6107ef565b6107b9565b610785565b610750565b61071b565b6106bf565b610658565b6105bc565b61054d565b6104f5565b610433565b610384565b60e01c90565b60405190565b5f80fd5b5f80fd5b63ffffffff60e01b1690565b61032581610310565b0361032c57565b5f80fd5b9050359061033d8261031c565b565b9060208282031261035857610355915f01610330565b90565b61030c565b151590565b61036b9061035d565b9052565b9190610382905f60208501940190610362565b565b346103b4576103b061039f61039a36600461033f565b61152b565b6103a7610302565b9182918261036f565b0390f35b610308565b5f9103126103c357565b61030c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b61040961041260209361041793610400816103c8565b938480936103cc565b958691016103d5565b6103e0565b0190565b6104309160208201915f8184039101526103ea565b90565b34610463576104433660046103b9565b61045f61044e6116c4565b610456610302565b9182918261041b565b0390f35b610308565b60018060a01b031690565b61047c90610468565b90565b61048881610473565b0361048f57565b5f80fd5b905035906104a08261047f565b565b90565b6104ae816104a2565b036104b557565b5f80fd5b905035906104c6826104a5565b565b91906040838203126104f057806104e46104ed925f8601610493565b936020016104b9565b90565b61030c565b346105265761052261051161050b3660046104c8565b906116da565b610519610302565b9182918261036f565b0390f35b610308565b610534906104a2565b9052565b919061054b905f6020850194019061052b565b565b3461057d5761055d3660046103b9565b610579610568611726565b610570610302565b91829182610538565b0390f35b610308565b90916060828403126105b7576105b461059d845f8501610493565b936105ab8160208601610493565b936040016104b9565b90565b61030c565b346105ed576105e96105d86105d2366004610582565b9161173c565b6105e0610302565b9182918261036f565b0390f35b610308565b90565b6105fe816105f2565b0361060557565b5f80fd5b90503590610616826105f5565b565b906020828203126106315761062e915f01610609565b90565b61030c565b61063f906105f2565b9052565b9190610656905f60208501940190610636565b565b346106885761068461067361066e366004610618565b6117b5565b61067b610302565b91829182610643565b0390f35b610308565b91906040838203126106b557806106a96106b2925f8601610609565b93602001610493565b90565b61030c565b5f0190565b346106ee576106d86106d236600461068d565b90611801565b6106e0610302565b806106ea816106ba565b0390f35b610308565b60ff1690565b610702906106f3565b9052565b9190610719905f602085019401906106f9565b565b3461074b5761072b3660046103b9565b610747610736611830565b61073e610302565b91829182610706565b0390f35b610308565b34610780576107603660046103b9565b61077c61076b611846565b610773610302565b91829182610643565b0390f35b610308565b346107b45761079e61079836600461068d565b9061185a565b6107a6610302565b806107b0816106ba565b0390f35b610308565b346107ea576107e66107d56107cf3660046104c8565b9061190b565b6107dd610302565b91829182610538565b0390f35b610308565b3461081e576108086108023660046104c8565b90611a92565b610810610302565b8061081a816106ba565b0390f35b610308565b9060208282031261083c57610839915f016104b9565b90565b61030c565b3461086f57610859610854366004610823565b611a9e565b610861610302565b8061086b816106ba565b0390f35b610308565b346108a4576108843660046103b9565b6108a061088f611b78565b610897610302565b9182918261041b565b0390f35b610308565b90565b90565b6108c36108be6108c8926108a9565b6108ac565b6104a2565b90565b6108d7629e34006108af565b90565b6108e26108cb565b90565b34610915576108f53660046103b9565b6109116109006108da565b610908610302565b91829182610538565b0390f35b610308565b9060208282031261093357610930915f01610493565b90565b61030c565b61094190610473565b9052565b9190610958905f60208501940190610938565b565b3461098a5761098661097561097036600461091a565b611c14565b61097d610302565b91829182610945565b0390f35b610308565b346109bd576109a76109a236600461091a565b611c33565b6109af610302565b806109b9816106ba565b0390f35b610308565b63ffffffff1690565b6109d4906109c2565b9052565b91906109eb905f602085019401906109cb565b565b34610a1d57610a19610a08610a0336600461091a565b611c4a565b610a10610302565b918291826109d8565b0390f35b610308565b34610a5257610a4e610a3d610a3836600461091a565b611c75565b610a45610302565b91829182610538565b0390f35b610308565b34610a8657610a70610a6a3660046104c8565b90611daa565b610a78610302565b80610a82816106ba565b0390f35b610308565b34610abb57610a9b3660046103b9565b610ab7610aa6611ddb565b610aae610302565b91829182610538565b0390f35b610308565b34610af057610aec610adb610ad636600461091a565b611e53565b610ae3610302565b91829182610538565b0390f35b610308565b34610b2557610b053660046103b9565b610b21610b10611e68565b610b18610302565b9182918261036f565b0390f35b610308565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610b7e57610b5e3660046103b9565b610b7a610b69610b2a565b610b71610302565b91829182610538565b0390f35b610308565b34610bb157610b9b610b96366004610823565b612033565b610ba3610302565b80610bad816106ba565b0390f35b610308565b60ff60f81b1690565b610bc890610bb6565b9052565b5190565b60209181520190565b60200190565b610be8906104a2565b9052565b90610bf981602093610bdf565b0190565b60200190565b90610c20610c1a610c1384610bcc565b8093610bd0565b92610bd9565b905f5b818110610c305750505090565b909192610c49610c436001928651610bec565b94610bfd565b9101919091610c23565b93959194610ca4610c99610cb895610c8b610cae95610cc59c9a610c7e60e08c01925f8d0190610bbf565b8a820360208c01526103ea565b9088820360408a01526103ea565b97606087019061052b565b6080850190610938565b60a0830190610636565b60c0818403910152610c03565b90565b34610cff57610cd83660046103b9565b610cfb610ce36120bb565b93610cf2979597939193610302565b97889788610c53565b0390f35b610308565b7f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a67490565b610d30610d04565b90565b34610d6357610d433660046103b9565b610d5f610d4e610d28565b610d56610302565b91829182610643565b0390f35b610308565b7f9b12e0c5707e494915e58b0564f18aaad9b74ac69bfc815a1edadc8e4bd032eb90565b610d94610d68565b90565b34610dc757610da73660046103b9565b610dc3610db2610d8c565b610dba610302565b91829182610643565b0390f35b610308565b34610dfc57610df8610de7610de2366004610823565b612145565b610def610302565b91829182610538565b0390f35b610308565b90565b610e18610e13610e1d92610e01565b6108ac565b6104a2565b90565b610e356b033b2e3c9fd0803ce8000000610e04565b90565b610e40610e20565b90565b34610e7357610e533660046103b9565b610e6f610e5e610e38565b610e66610302565b91829182610538565b0390f35b610308565b34610ea957610ea5610e94610e8e36600461068d565b906121b3565b610e9c610302565b9182918261036f565b0390f35b610308565b65ffffffffffff1690565b610ec290610eae565b9052565b9190610ed9905f60208501940190610eb9565b565b34610f0b57610eeb3660046103b9565b610f07610ef66121e1565b610efe610302565b91829182610ec6565b0390f35b610308565b34610f4057610f203660046103b9565b610f3c610f2b6121f5565b610f33610302565b9182918261041b565b0390f35b610308565b34610f7557610f71610f60610f5b36600461091a565b61220b565b610f68610302565b91829182610538565b0390f35b610308565b90565b610f91610f8c610f9692610f7a565b6108ac565b6104a2565b90565b610fae6b02e87669c308736a04000000610f7d565b90565b610fb9610f99565b90565b34610fec57610fcc3660046103b9565b610fe8610fd7610fb1565b610fdf610302565b91829182610538565b0390f35b610308565b90565b5f1b90565b61100d61100861101292610ff1565b610ff4565b6105f2565b90565b61101e5f610ff9565b90565b611029611015565b90565b3461105c5761103c3660046103b9565b611058611047611021565b61104f610302565b91829182610643565b0390f35b610308565b346110925761108e61107d6110773660046104c8565b9061223a565b611085610302565b9182918261036f565b0390f35b610308565b1c90565b90565b6110ae9060086110b39302611097565b61109b565b90565b906110c1915461109e565b90565b6110d0600c5f906110b6565b90565b34611103576110e33660046103b9565b6110ff6110ee6110c4565b6110f6610302565b91829182610538565b0390f35b610308565b346111395761113561112461111e3660046104c8565b9061225c565b61112c610302565b91829182610538565b0390f35b610308565b3461116e5761116a61115961115436600461091a565b612272565b611161610302565b91829182610538565b0390f35b610308565b346111a3576111833660046103b9565b61119f61118e612287565b611196610302565b91829182610538565b0390f35b610308565b6111b1816106f3565b036111b857565b5f80fd5b905035906111c9826111a8565b565b909160c08284031261122a576111e3835f8401610493565b926111f181602085016104b9565b926111ff82604083016104b9565b9261122761121084606085016111bc565b9361121e8160808601610609565b9360a001610609565b90565b61030c565b346112645761124e6112423660046111cb565b94939093929192612307565b611256610302565b80611260816106ba565b0390f35b610308565b60e0818303126112d45761127f825f8301610493565b9261128d8360208401610493565b9261129b81604085016104b9565b926112a982606083016104b9565b926112d16112ba84608085016111bc565b936112c88160a08601610609565b9360c001610609565b90565b61030c565b3461130e576112f86112ec366004611269565b9594909493919361245b565b611300610302565b8061130a816106ba565b0390f35b610308565b346113425761132c61132636600461068d565b90612579565b611334610302565b8061133e816106ba565b0390f35b610308565b919060408382031261136f578061136361136c925f8601610493565b93602001610493565b90565b61030c565b346113a5576113a161139061138a366004611347565b9061259b565b611398610302565b91829182610538565b0390f35b610308565b6113b3816109c2565b036113ba57565b5f80fd5b905035906113cb826113aa565b565b91906040838203126113f557806113e96113f2925f8601610493565b936020016113be565b90565b61030c565b61140390610eae565b9052565b60018060d01b031690565b61141b90611407565b9052565b90602080611441936114375f8201515f8601906113fa565b0151910190611412565b565b9190611456905f6040850194019061141f565b565b346114895761148561147461146e3660046113cd565b90612609565b61147c610302565b91829182611443565b0390f35b610308565b5f80fd5b5f90565b634e487b7160e01b5f52601160045260245ffd5b6114b66114bc91611407565b91611407565b019060018060d01b0382116114cd57565b611496565b906114e5916114df611492565b506114aa565b90565b6114f46114fa91611407565b91611407565b90039060018060d01b03821161150c57565b611496565b906115249161151e611492565b506114e8565b90565b5f90565b611533611527565b508061154e611548637965db0b60e01b610310565b91610310565b1490811561155b575b5090565b611565915061261f565b5f611557565b606090565b634e487b7160e01b5f52602260045260245ffd5b90600160028304921680156115a4575b602083101461159f57565b611570565b91607f1691611594565b60209181520190565b5f5260205f2090565b905f92918054906115da6115d383611584565b80946115ae565b916001811690815f1461163157506001146115f5575b505050565b61160291929394506115b7565b915f925b81841061161957505001905f80806115f0565b60018160209295939554848601520191019290611606565b92949550505060ff19168252151560200201905f80806115f0565b90611656916115c0565b90565b634e487b7160e01b5f52604160045260245ffd5b90611677906103e0565b810190811067ffffffffffffffff82111761169157604052565b611659565b906116b66116af926116a6610302565b9384809261164c565b038361166d565b565b6116c190611696565b90565b6116cc61156b565b506116d760036116b8565b90565b6116f7916116e6611527565b506116ef612645565b919091612652565b600190565b5f90565b5f1c90565b61171161171691611700565b61109b565b90565b6117239054611705565b90565b61172e6116fc565b506117396002611719565b90565b9161176692611749611527565b5061175e611755612645565b829084916126a2565b91909161272e565b600190565b5f90565b611778906105f2565b90565b906117859061176f565b5f5260205260405f2090565b90565b6117a06117a591611700565b611791565b90565b6117b29054611794565b90565b60016117ce6117d4926117c661176b565b50600561177b565b016117a8565b90565b906117f2916117ed6117e8826117b5565b6127cb565b6117f4565b565b906117fe91612824565b50565b9061180b916117d7565b565b5f90565b90565b61182861182361182d92611811565b6108ac565b6106f3565b90565b61183861180d565b506118436012611814565b90565b61184e61176b565b506118576128d0565b90565b908061187561186f61186a612645565b610473565b91610473565b03611886576118839161298a565b50565b5f63334bd91960e11b81528061189e600482016106ba565b0390fd5b6118b66118b16118bb92610468565b6108ac565b610468565b90565b6118c7906118a2565b90565b6118d3906118be565b90565b906118e0906118ca565b5f5260205260405f2090565b90565b6119036118fe61190892611407565b6108ac565b6104a2565b90565b6119429161193761193161192c61193d946119246116fc565b50600a6118d6565b6118ec565b91612a6b565b90612b80565b6118ef565b90565b9061195f9161195a611955610d68565b6127cb565b6119ca565b565b61197561197061197a92610ff1565b6108ac565b610468565b90565b61198690611961565b90565b61199d6119986119a292610ff1565b6108ac565b6104a2565b90565b6119b46119ba919392936104a2565b926104a2565b82018092116119c557565b611496565b90816119e66119e06119db5f61197d565b610473565b91610473565b14611a7657806119fe6119f85f611989565b916104a2565b14611a5a57611a15611a0e611726565b82906119a5565b611a2e611a28611a23610e20565b6104a2565b916104a2565b11611a3e57611a3c91612ca7565b565b5f63177e3fc360e01b815280611a56600482016106ba565b0390fd5b5f631f2a200560e01b815280611a72600482016106ba565b0390fd5b5f63d92e233d60e01b815280611a8e600482016106ba565b0390fd5b90611a9c91611945565b565b80611ab1611aab5f611989565b916104a2565b14611ac257611ac09033612d05565b565b5f631f2a200560e01b815280611ada600482016106ba565b0390fd5b90611af1611aea610302565b928361166d565b565b67ffffffffffffffff8111611b1157611b0d6020916103e0565b0190565b611659565b90611b28611b2383611af3565b611ade565b918252565b5f7f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c74000000910152565b611b5e601d611b16565b90611b6b60208301611b2d565b565b611b75611b54565b90565b611b8061156b565b50611b896121e1565b611ba2611b9c611b97612d64565b610eae565b91610eae565b03611bb257611baf611b6d565b90565b5f6301bfc1c560e61b815280611bca600482016106ba565b0390fd5b5f90565b90611bdc906118ca565b5f5260205260405f2090565b60018060a01b031690565b611bff611c0491611700565b611be8565b90565b611c119054611bf3565b90565b611c2b611c3091611c23611bce565b506009611bd2565b611c07565b90565b611c4490611c3f612645565b612db7565b565b5f90565b611c5c90611c56611c46565b50612e42565b90565b90611c69906118ca565b5f5260205260405f2090565b611c8b611c9091611c846116fc565b505f611c5f565b611719565b90565b90611cad91611ca8611ca3610d04565b6127cb565b611caf565b565b80611cca611cc4611cbf5f61197d565b610473565b91610473565b14611d8e5781611ce2611cdc5f611989565b916104a2565b14611d7257611cf8611cf2611e68565b1561035d565b611d5657611d07818390612d05565b3390611d51611d3f611d397fbef4f81c1814c641ede85ebaacf19d048b2c5b55980adfa6ef0f956c651335a2936118ca565b936118ca565b93611d48610302565b91829182610538565b0390a3565b5f63b8b5ca2d60e01b815280611d6e600482016106ba565b0390fd5b5f631f2a200560e01b815280611d8a600482016106ba565b0390fd5b5f63d92e233d60e01b815280611da6600482016106ba565b0390fd5b90611db491611c93565b565b611dc5611dcb919392936104a2565b926104a2565b8203918211611dd657565b611496565b611de36116fc565b50611dee600c611719565b611e00611dfa5f611989565b916104a2565b148015611e2f575b611e2357611e20611e19600c611719565b4290611db6565b90565b611e2c5f611989565b90565b5042611e4c611e46611e41600c611719565b6104a2565b916104a2565b1015611e08565b611e6590611e5f6116fc565b50612e71565b90565b611e70611527565b50611e7b600c611719565b611e8d611e875f611989565b916104a2565b141580611e98575b90565b5042611eb5611eaf611eaa600c611719565b6104a2565b916104a2565b10611e95565b611ed490611ecf611eca611015565b6127cb565b611f4e565b565b90611ee25f1991610ff4565b9181191691161790565b611f00611efb611f05926104a2565b6108ac565b6104a2565b90565b90565b90611f20611f1b611f2792611eec565b611f08565b8254611ed6565b9055565b916020611f4c929493611f4560408201965f83019061052b565b019061052b565b565b80611f61611f5b426104a2565b916104a2565b11156120175780611f9a611f947f00000000000000000000000000000000000000000000000000000000000000006104a2565b916104a2565b11611ffb57611fa9600c611719565b611fb482600c611f0b565b903390611fe17fdd6896dcf1d4b311cca87dd19bbba2ea9ce2f867c1568878a0438a66a1afeeec926118ca565b92611ff6611fed610302565b92839283611f2b565b0390a2565b5f63ef69af6560e01b815280612013600482016106ba565b0390fd5b5f63a565835360e01b81528061202f600482016106ba565b0390fd5b61203c90611ebb565b565b5f90565b606090565b612050906118be565b90565b67ffffffffffffffff811161206b5760208091020190565b611659565b9061208261207d83612053565b611ade565b918252565b369037565b906120b161209983612070565b926020806120a78693612053565b9201910390612087565b565b600f60f81b90565b6120c361203e565b506120cc61156b565b506120d561156b565b506120de6116fc565b506120e7611bce565b506120f061176b565b506120f9612042565b50612102612e89565b9061210b612ec9565b90469061211730612047565b906121215f610ff9565b9061213361212e5f611989565b61208c565b9061213c6120b3565b96959493929190565b61216e612173916121546116fc565b50612168612162600b6118ec565b91612a6b565b90612b80565b6118ef565b90565b90612180906118ca565b5f5260205260405f2090565b60ff1690565b61219e6121a391611700565b61218c565b90565b6121b09054612192565b90565b6121da915f6121cf6121d5936121c7611527565b50600561177b565b01612176565b6121a6565b90565b5f90565b6121e96121dd565b506121f2612d64565b90565b6121fd61156b565b5061220860046116b8565b90565b61223261222d612228612237936122206116fc565b50600a6118d6565b6118ec565b612f09565b6118ef565b90565b61225791612246611527565b5061224f612645565b91909161272e565b600190565b9061226f916122696116fc565b5061190b565b90565b6122849061227e6116fc565b5061220b565b90565b61228f6116fc565b50612298611726565b90565b7fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf90565b6122f46122fb946122ea6060949897956122e0608086019a5f870190610636565b6020850190610938565b604083019061052b565b019061052b565b565b60200190565b5190565b939594909291954261232161231b896104a2565b916104a2565b1161239a579161238c9161239393612383612398989961236b61234261229b565b61235c8b938b612350610302565b958694602086016122bf565b6020820181038252038261166d565b61237d61237782612303565b916122fd565b20612f7e565b92909192612f9b565b9182612fe5565b612db7565b565b6123b5875f918291632341d78760e11b835260048301610538565b0390fd5b7f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c990565b919461242561242f9298979561241b60a0966124116124369a61240760c08a019e5f8b0190610636565b6020890190610938565b6040870190610938565b606085019061052b565b608083019061052b565b019061052b565b565b91602061245992949361245260408201965f830190610938565b0190610938565b565b969591939294909442612476612470836104a2565b916104a2565b1161253057906124df6124e89493926124c76124906123b9565b6124b88c80948c916124a28d91613037565b91926124ac610302565b978896602088016123dd565b6020820181038252038261166d565b6124d96124d382612303565b916122fd565b20612f7e565b92909192612f9b565b806124fb6124f587610473565b91610473565b03612510575061250e9293919091612652565b565b849061252c5f9283926325c0072360e11b845260048401612438565b0390fd5b61254b905f91829163313c898160e11b835260048301610538565b0390fd5b9061256a91612565612560826117b5565b6127cb565b61256c565b565b906125769161298a565b50565b906125839161254f565b565b9061258f906118ca565b5f5260205260405f2090565b6125c0916125b66125bb926125ae6116fc565b506001612585565b611c5f565b611719565b90565b6125cd6040611ade565b90565b5f90565b5f90565b6125e06125c3565b90602080836125ed6125d0565b8152016125f86125d4565b81525050565b6126066125d8565b90565b9061261c916126166125fe565b5061306a565b90565b612627611527565b5061264161263b6301ffc9a760e01b610310565b91610310565b1490565b61264d611bce565b503390565b916126609291600192613092565b565b60409061268b612692949695939661268160608401985f850190610938565b602083019061052b565b019061052b565b565b9061269f91036104a2565b90565b9291926126b081839061259b565b90816126c56126bf5f196104a2565b916104a2565b106126d2575b5050509050565b816126e56126df876104a2565b916104a2565b1061270b5761270293946126fa919392612694565b905f92613092565b805f80806126cb565b5061272a849291925f938493637dc7a0d960e11b855260048501612662565b0390fd5b918261274a61274461273f5f61197d565b610473565b91610473565b146127a4578161276a61276461275f5f61197d565b610473565b91610473565b1461277d5761277b929190916131a1565b565b6127a06127895f61197d565b5f91829163ec442f0560e01b835260048301610945565b0390fd5b6127c76127b05f61197d565b5f918291634b637e8f60e11b835260048301610945565b0390fd5b6127dd906127d7612645565b9061326e565b565b906127eb60ff91610ff4565b9181191691161790565b6127fe9061035d565b90565b90565b90612819612814612820926127f5565b612801565b82546127df565b9055565b61282c611527565b5061284161283b8284906121b3565b1561035d565b5f146128ca5761286960016128645f61285c6005869061177b565b018590612176565b612804565b90612872612645565b906128af6128a96128a37f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d9561176f565b926118ca565b926118ca565b926128b8610302565b806128c2816106ba565b0390a4600190565b50505f90565b6128d861176b565b506128e230612047565b61291461290e7f0000000000000000000000000000000000000000000000000000000000000000610473565b91610473565b1480612950575b5f14612945577f000000000000000000000000000000000000000000000000000000000000000090565b61294d61331a565b90565b504661298461297e7f00000000000000000000000000000000000000000000000000000000000000006104a2565b916104a2565b1461291b565b612992611527565b5061299e8183906121b3565b5f14612a26576129c55f6129c05f6129b86005869061177b565b018590612176565b612804565b906129ce612645565b90612a0b612a056129ff7ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b9561176f565b926118ca565b926118ca565b92612a14610302565b80612a1e816106ba565b0390a4600190565b50505f90565b612a40612a3b612a4592610eae565b6108ac565b6104a2565b90565b916020612a69929493612a6260408201965f83019061052b565b0190610eb9565b565b612a736121dd565b50612a7c6121e1565b81612a8f612a8983612a2c565b916104a2565b1015612aa25750612a9f90613423565b90565b90612abd5f928392637669fc0f60e11b845260048401612a48565b0390fd5b5490565b90565b612adc612ad7612ae192612ac5565b6108ac565b6104a2565b90565b90565b65ffffffffffff1690565b612afe612b0391611700565b612ae7565b90565b612b109054612af2565b90565b90565b612b2a612b25612b2f92612b13565b6108ac565b6104a2565b90565b60301c90565b60018060d01b031690565b612b4f612b5491612b32565b612b38565b90565b612b619054612b43565b90565b612b78612b73612b7d92610ff1565b6108ac565b611407565b90565b90612bd490612b8d611492565b50612b995f8401612ac1565b612ba25f611989565b908080612bb8612bb26005612ac8565b916104a2565b11612c35575b5090612bcf5f860193919293612ae4565b613a76565b80612be7612be15f611989565b916104a2565b145f14612bfd575050612bf95f612b64565b5b90565b612c2a5f91612c25612c1f84612c30960192612c196001612b16565b90611db6565b91612ae4565b613a6c565b01612b57565b612bfa565b80612c43612c499291613701565b90611db6565b9083612c7b612c75612c705f612c6a818c01612c658991612ae4565b613a6c565b01612b06565b610eae565b91610eae565b105f14612c8c5750905b905f612bbe565b9150612ca290612c9c6001612b16565b906119a5565b612c85565b80612cc2612cbc612cb75f61197d565b610473565b91610473565b14612cde57612cdc91612cd45f61197d565b9190916131a1565b565b612d01612cea5f61197d565b5f91829163ec442f0560e01b835260048301610945565b0390fd5b9081612d21612d1b612d165f61197d565b610473565b91610473565b14612d3d57612d3b9190612d345f61197d565b90916131a1565b565b612d60612d495f61197d565b5f918291634b637e8f60e11b835260048301610945565b0390fd5b612d6c6121dd565b50612d7643613423565b90565b90612d8a60018060a01b0391610ff4565b9181191691161790565b90565b90612dac612da7612db3926118ca565b612d94565b8254612d79565b9055565b90612e4091612e3a612dc882611c14565b612ddd84612dd860098690611bd2565b612d97565b82818590612e1d612e17612e117f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f956118ca565b926118ca565b926118ca565b92612e26610302565b80612e30816106ba565b0390a49291613b05565b91613b1d565b565b612e69612e64612e5f612e6e93612e57611c46565b50600a6118d6565b6118ec565b613ccb565b613d4a565b90565b612e8390612e7d6116fc565b50613d9b565b90565b90565b612e9161156b565b50612ec67f0000000000000000000000000000000000000000000000000000000000000000612ec06006612e86565b90613eb6565b90565b612ed161156b565b50612f067f0000000000000000000000000000000000000000000000000000000000000000612f006007612e86565b90613eb6565b90565b612f11611492565b50612f1d5f8201612ac1565b80612f30612f2a5f611989565b916104a2565b145f14612f46575050612f425f612b64565b5b90565b612f735f91612f6e612f6884612f79960192612f626001612b16565b90611db6565b91612ae4565b613a6c565b01612b57565b612f43565b612f9890612f8a61176b565b50612f936128d0565b613f04565b90565b92612fb692612fbf94612fac611bce565b5092909192613fca565b909291926140f5565b90565b916020612fe3929493612fdc60408201965f830190610938565b019061052b565b565b612fee81613037565b91613001612ffb846104a2565b916104a2565b0361300a575050565b6130245f9283926301d4b62360e61b845260048401612fc2565b0390fd5b600161303491016104a2565b90565b61304b906130436116fc565b506008611c5f565b61306761305782611719565b9161306183613028565b90611f0b565b90565b9061308a61308561308f9361307d6125fe565b50600a6118d6565b6118ec565b61426b565b90565b9092816130af6130a96130a45f61197d565b610473565b91610473565b1461317a57836130cf6130c96130c45f61197d565b610473565b91610473565b14613153576130f3836130ee6130e760018690612585565b8790611c5f565b611f0b565b6130fd575b505050565b9190916131486131366131307f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925936118ca565b936118ca565b9361313f610302565b91829182610538565b0390a35f80806130f8565b61317661315f5f61197d565b5f918291634a1406b160e11b835260048301610945565b0390fd5b61319d6131865f61197d565b5f91829163e602df0560e01b835260048301610945565b0390fd5b91826131bd6131b76131b25f61197d565b610473565b91610473565b141580613228575b6131d8575b6131d69291909161428c565b565b6131e0611e68565b80613207575b156131ca575f6336e278fd60e21b815280613203600482016106ba565b0390fd5b5061322361321d613216610d04565b33906121b3565b1561035d565b6131e6565b508161324461323e6132395f61197d565b610473565b91610473565b14156131c5565b91602061326c92949361326560408201965f830190610938565b0190610636565b565b9061328361327d8383906121b3565b1561035d565b61328b575050565b6132a55f92839263e2517d3f60e01b84526004840161324b565b0390fd5b7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f90565b9095949261331894613307613311926132fd6080966132f360a088019c5f890190610636565b6020870190610636565b6040850190610636565b606083019061052b565b0190610938565b565b61332261176b565b5061332b6132a9565b6133a27f0000000000000000000000000000000000000000000000000000000000000000916133937f00000000000000000000000000000000000000000000000000000000000000004661337e30612047565b91613387610302565b968795602087016132cd565b6020820181038252038261166d565b6133b46133ae82612303565b916122fd565b2090565b90565b6133cf6133ca6133d4926133b8565b6108ac565b6106f3565b90565b6133e0906133bb565b9052565b9160206134059294936133fe60408201965f8301906133d7565b019061052b565b565b61341b613416613420926104a2565b6108ac565b610eae565b90565b61342b6121dd565b508061344561343f65ffffffffffff612a2c565b916104a2565b116134565761345390613407565b90565b60306134725f9283926306dfcc6560e41b8452600484016133e4565b0390fd5b90565b61348d61348861349292613476565b6108ac565b6104a2565b90565b90565b6134ac6134a76134b192613495565b6108ac565b6106f3565b90565b6134d3906134cd6134c76134d8946106f3565b916104a2565b90611097565b6104a2565b90565b90565b6134f26134ed6134f7926134db565b6108ac565b6106f3565b90565b1b90565b61351d90613517613511613522946106f3565b916104a2565b906134fa565b6104a2565b90565b90565b61353c61353761354192613525565b6108ac565b6104a2565b90565b90565b61355b61355661356092613544565b6108ac565b6106f3565b90565b90565b61357a61357561357f92613563565b6108ac565b6104a2565b90565b90565b61359961359461359e92613582565b6108ac565b6106f3565b90565b90565b6135b86135b36135bd926135a1565b6108ac565b6104a2565b90565b90565b6135d76135d26135dc926135c0565b6108ac565b6106f3565b90565b90565b6135f66135f16135fb926135df565b6108ac565b6104a2565b90565b90565b61361561361061361a926135fe565b6108ac565b6106f3565b90565b61363161362c61363692613582565b6108ac565b6104a2565b90565b90565b61365061364b61365592613639565b6108ac565b6106f3565b90565b61366c613667613671926135fe565b6108ac565b6104a2565b90565b61368861368361368d92612b13565b6108ac565b6106f3565b90565b90565b6136a76136a26136ac92613690565b6108ac565b6104a2565b90565b906136ba91026104a2565b90565b634e487b7160e01b5f52601260045260245ffd5b6136dd6136e3916104a2565b916104a2565b9081156136ee570490565b6136bd565b906136fe91016104a2565b90565b6137096116fc565b508061371e6137186001612b16565b916104a2565b1115613a6957806139336139106139006138f06138e06138d06138c06138b06138a06138906138808b61387a6138736139399f613853613843613863926137656001612b16565b908061377d613777600160801b613479565b916104a2565b1015613a3b575b806137a061379a68010000000000000000613528565b916104a2565b1015613a0d575b806137bf6137b9640100000000613566565b916104a2565b10156139df575b806137dc6137d6620100006135a4565b916104a2565b10156139b1575b806137f86137f26101006135e2565b916104a2565b1015613983575b8061381361380d601061361d565b916104a2565b1015613955575b61382d6138276004613658565b916104a2565b101561393c575b61383e6003613693565b6136af565b61384d6001613674565b906134b4565b61385d81866136d1565b906136f3565b61386d6001613674565b906134b4565b80926136d1565b906136f3565b61388a6001613674565b906134b4565b61389a818c6136d1565b906136f3565b6138aa6001613674565b906134b4565b6138ba818a6136d1565b906136f3565b6138ca6001613674565b906134b4565b6138da81886136d1565b906136f3565b6138ea6001613674565b906134b4565b6138fa81866136d1565b906136f3565b61390a6001613674565b906134b4565b9161392d6139276139228580946136d1565b6104a2565b916104a2565b1161431c565b90612694565b90565b6139509061394a6001613674565b906134fe565b613834565b61396c61397d916139666004613601565b906134b4565b91613977600261363c565b906134fe565b9061381a565b61399a6139ab9161399460086135c3565b906134b4565b916139a56004613601565b906134fe565b906137ff565b6139c86139d9916139c26010613585565b906134b4565b916139d360086135c3565b906134fe565b906137e3565b6139f6613a07916139f06020613547565b906134b4565b91613a016010613585565b906134fe565b906137c6565b613a24613a3591613a1e60406134de565b906134b4565b91613a2f6020613547565b906134fe565b906137a7565b613a52613a6391613a4c6080613498565b906134b4565b91613a5d60406134de565b906134fe565b90613784565b90565b5f5260205f200190565b93919092613a826116fc565b505b81613a97613a91836104a2565b916104a2565b1015613afd57613aa8828290614368565b90613abe5f613ab8888590613a6c565b01612b06565b613ad0613aca87610eae565b91610eae565b115f14613ae05750915b91613a84565b929150613af790613af16001612b16565b906119a5565b90613ada565b925050915090565b613b1790613b116116fc565b50611c75565b90565b90565b91909180613b33613b2d85610473565b91610473565b141580613cb1575b613b45575b505050565b80613b60613b5a613b555f61197d565b610473565b91610473565b03613c21575b5081613b82613b7c613b775f61197d565b610473565b91610473565b03613b8e575b80613b40565b613bd5613bc8613bcf92613ba4600a86906118d6565b90613bc2613bbc613bb6600193614401565b936118ec565b91613b1a565b90614454565b92906118ef565b916118ef565b919091613c027fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926118ca565b92613c17613c0e610302565b92839283611f2b565b0390a25f80613b88565b613c60613c66613c59613c36600a85906118d6565b6002613c53613c4d613c4789614401565b936118ec565b91613b1a565b90614454565b92906118ef565b916118ef565b919091613c937fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926118ca565b92613ca8613c9f610302565b92839283611f2b565b0390a25f613b66565b5081613cc5613cbf5f611989565b916104a2565b11613b3b565b5f613cdf91613cd86116fc565b5001612ac1565b90565b613cf6613cf1613cfb926109c2565b6108ac565b6104a2565b90565b613d0790613547565b9052565b916020613d2c929493613d2560408201965f830190613cfe565b019061052b565b565b613d42613d3d613d47926104a2565b6108ac565b6109c2565b90565b613d52611c46565b5080613d6a613d6463ffffffff613ce2565b916104a2565b11613d7b57613d7890613d2e565b90565b6020613d975f9283926306dfcc6560e41b845260048401613d0b565b0390fd5b613db2613db791613daa6116fc565b506008611c5f565b611719565b90565b90565b613dd1613dcc613dd692613dba565b610ff4565b6105f2565b90565b613de360ff613dbd565b90565b5f5260205f2090565b905f9291805490613e09613e0283611584565b80946115ae565b916001811690815f14613e605750600114613e24575b505050565b613e319192939450613de6565b915f925b818410613e4857505001905f8080613e1f565b60018160209295939554848601520191019290613e35565b92949550505060ff19168252151560200201905f8080613e1f565b90613e8591613def565b90565b90613ea8613ea192613e98610302565b93848092613e7b565b038361166d565b565b613eb390613e88565b90565b90613ebf61156b565b50613ec98261176f565b613ee2613edc613ed7613dd9565b6105f2565b916105f2565b14155f14613ef75750613ef4906144de565b90565b613f019150613eaa565b90565b604291613f0f61176b565b50604051917f19010000000000000000000000000000000000000000000000000000000000008352600283015260228201522090565b5f90565b613f55613f5a91611700565b611eec565b90565b90565b613f74613f6f613f7992613f5d565b6108ac565b6104a2565b90565b613fb1613fb894613fa7606094989795613f9d608086019a5f870190610636565b60208501906106f9565b6040830190610636565b0190610636565b565b613fc2610302565b3d5f823e3d90fd5b939293613fd5611bce565b50613fde613f45565b50613fe761176b565b50613ff185613f49565b61402361401d7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0613f60565b916104a2565b116140b05790614046602094955f9493929361403d610302565b94859485613f7c565b838052039060015afa156140ab5761405e5f51610ff4565b8061407961407361406e5f61197d565b610473565b91610473565b1461408f575f916140895f610ff9565b91929190565b506140995f61197d565b6001916140a55f610ff9565b91929190565b613fba565b5050506140bc5f61197d565b9060039291929190565b634e487b7160e01b5f52602160045260245ffd5b600411156140e457565b6140c6565b906140f3826140da565b565b806141086141025f6140e9565b916140e9565b145f14614113575050565b8061412761412160016140e9565b916140e9565b145f1461414a575f63f645eedf60e01b815280614146600482016106ba565b0390fd5b8061415e61415860026140e9565b916140e9565b145f1461418c5761418861417183613f49565b5f91829163fce698f760e01b835260048301610538565b0390fd5b61419f61419960036140e9565b916140e9565b146141a75750565b6141c2905f9182916335e2f38360e21b835260048301610643565b0390fd5b634e487b7160e01b5f52603260045260245ffd5b5f5260205f2090565b6141ec81612ac1565b821015614206576141fe6001916141da565b910201905f90565b6141c6565b9061421590610eae565b9052565b9061422390611407565b9052565b9061425d6142545f6142376125c3565b9461424e614246838301612b06565b83880161420b565b01612b57565b60208401614219565b565b61426890614227565b90565b614289915f6142839261427c6125fe565b50016141e3565b5061425f565b90565b929161429a8483839161450e565b836142b56142af6142aa5f61197d565b610473565b91610473565b146142ca575b6142c89293919091614698565b565b6142d2611726565b936142db61467d565b94806142ef6142e9886104a2565b916104a2565b116142fc575093506142bb565b85906143185f928392630e58ae9360e11b845260048401611f2b565b0390fd5b6143246116fc565b50151590565b61433e61433961434392613639565b6108ac565b6104a2565b90565b614352614358916104a2565b916104a2565b908115614363570490565b6136bd565b61438d614393926143776116fc565b508281169218614387600261432a565b90614346565b906119a5565b90565b90565b6143ad6143a86143b292614396565b6108ac565b6106f3565b90565b6143be90614399565b9052565b9160206143e39294936143dc60408201965f8301906143b5565b019061052b565b565b6143f96143f46143fe926104a2565b6108ac565b611407565b90565b614409611492565b508061442361441d60018060d01b036118ef565b916104a2565b1161443457614431906143e5565b90565b60d06144505f9283926306dfcc6560e41b8452600484016143c2565b0390fd5b9061448a6144909392614465611492565b5061446e611492565b50809361448361447c6121e1565b9492612f09565b9091614b13565b91614757565b91909190565b6144aa6144a56144af92613544565b6108ac565b6104a2565b90565b369037565b906144dc6144c483611b16565b926020806144d28693611af3565b92019103906144b2565b565b6144e661156b565b506144f0816147c1565b906145036144fe6020614496565b6144b7565b918252602082015290565b9190918061452c6145266145215f61197d565b610473565b91610473565b145f1461460d57614550614549836145446002611719565b6119a5565b6002611f0b565b5b8261456c6145666145615f61197d565b610473565b91610473565b145f146145e157614590614589836145846002611719565b612694565b6002611f0b565b5b9190916145dc6145ca6145c47fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef936118ca565b936118ca565b936145d3610302565b91829182610538565b0390a3565b614608826146026145f35f8790611c5f565b916145fd83611719565b6136f3565b90611f0b565b614591565b61462061461b5f8390611c5f565b611719565b8061463361462d856104a2565b916104a2565b1061465b57614646614656918490612694565b6146515f8490611c5f565b611f0b565b614551565b906146799091925f93849363391434e360e21b855260048501612662565b0390fd5b6146856116fc565b5061469560018060d01b036118ef565b90565b916146f06146ea6146f794806146be6146b86146b35f61197d565b610473565b91610473565b14614728575b846146df6146d96146d45f61197d565b610473565b91610473565b146146f9575b611c14565b92611c14565b9091613b1d565b565b614721600b600261471b61471561470f89614401565b936118ec565b91613b1a565b90614454565b50506146e5565b614750600b600161474a61474461473e89614401565b936118ec565b91613b1a565b90614454565b50506146c4565b9161477b5f61478094614768611492565b50614771611492565b5001929192612ae4565b6149c5565b91909190565b61479a61479561479f92613dba565b6108ac565b6104a2565b90565b90565b6147b96147b46147be926147a2565b6108ac565b6104a2565b90565b6147d66147db916147d06116fc565b5061176f565b613f49565b6147e560ff614786565b16806147fa6147f4601f6147a5565b916104a2565b116148025790565b5f632cd44ac360e21b81528061481a600482016106ba565b0390fd5b5490565b61482c6040611ade565b90565b5f5260205f2090565b6148418161481e565b82101561485b5761485360019161482f565b910201905f90565b6141c6565b634e487b7160e01b5f525f60045260245ffd5b61487d9051610eae565b90565b9061489165ffffffffffff91610ff4565b9181191691161790565b6148af6148aa6148b492610eae565b6108ac565b610eae565b90565b90565b906148cf6148ca6148d69261489b565b6148b7565b8254614880565b9055565b6148e49051611407565b90565b60301b90565b906148ff65ffffffffffff19916148e7565b9181191691161790565b61491d61491861492292611407565b6108ac565b611407565b90565b90565b9061493d61493861494492614909565b614925565b82546148ed565b9055565b9061497260205f6149789461496a828201614964848801614873565b906148ba565b0192016148da565b90614928565b565b919061498b5761498991614948565b565b614860565b90815491680100000000000000008310156149c057826149b89160016149be95018155614838565b9061497a565b565b611659565b909291926149d1611492565b506149da611492565b506149e48261481e565b806149f76149f15f611989565b916104a2565b115f14614ac757614a1d90614a178491614a116001612b16565b90611db6565b90613a6c565b90614a295f8301612b06565b92614a355f8401612b57565b9380614a49614a4385610eae565b91610eae565b11614aab57614a60614a5a84610eae565b91610eae565b145f14614a7b575050614a76905f859101614928565b5b9190565b614aa69250614aa186614a98614a8f614822565b945f860161420b565b60208401614219565b614990565b614a77565b5f632520601d60e01b815280614ac3600482016106ba565b0390fd5b50614af291614aed85614ae4614adb614822565b945f860161420b565b60208401614219565b614990565b614afb5f612b64565b9190565b634e487b7160e01b5f52605160045260245ffd5b91909180600114614b3257600203614aff57614b2e91611511565b905b565b50614b3c916114d2565b90614b3056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\x80`@R4a\0\x84Wa\0\x1Ba\0\x15a\x01XV[\x90a\x03\xBDV[a\0#a\0\x89V[aKBa\x1C\x83\x829`\x80Q\x81a)#\x01R`\xA0Q\x81a)Z\x01R`\xC0Q\x81a(\xEA\x01R`\xE0Q\x81a30\x01Ra\x01\0Q\x81a3U\x01Ra\x01 Q\x81a.\x97\x01Ra\x01@Q\x81a.\xD7\x01Ra\x01`Q\x81\x81\x81a\x0B,\x01Ra\x1Fp\x01RaKB\x90\xF3[a\0\x8FV[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0\xBB\x90a\0\x93V[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\xD3W`@RV[a\0\x9DV[\x90a\0\xEBa\0\xE4a\0\x89V[\x92\x83a\0\xB1V[V[_\x80\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x01\x05\x90a\0\xF1V[\x90V[a\x01\x11\x81a\0\xFCV[\x03a\x01\x18WV[_\x80\xFD[\x90PQ\x90a\x01)\x82a\x01\x08V[V[\x91\x90`@\x83\x82\x03\x12a\x01SW\x80a\x01Ga\x01P\x92_\x86\x01a\x01\x1CV[\x93` \x01a\x01\x1CV[\x90V[a\0\xEDV[a\x01vag\xC5\x808\x03\x80a\x01k\x81a\0\xD8V[\x92\x839\x81\x01\x90a\x01+V[\x90\x91V[`\x01\x80`@\x1B\x03\x81\x11a\x01\x96Wa\x01\x92` \x91a\0\x93V[\x01\x90V[a\0\x9DV[\x90a\x01\xADa\x01\xA8\x83a\x01zV[a\0\xD8V[\x91\x82RV[_\x7FSyndicate\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x01\xE3`\ta\x01\x9BV[\x90a\x01\xF0` \x83\x01a\x01\xB2V[V[a\x01\xFAa\x01\xD9V[\x90V[_\x7FSYND\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x02.`\x04a\x01\x9BV[\x90a\x02;` \x83\x01a\x01\xFDV[V[a\x02Ea\x02$V[\x90V[\x90V[\x90V[a\x02ba\x02]a\x02g\x92a\x02HV[a\x02KV[a\0\xF1V[\x90V[a\x02s\x90a\x02NV[\x90V[_\x01\x90V[\x90V[\x90V[a\x02\x95a\x02\x90a\x02\x9A\x92a\x02{V[a\x02KV[a\x02~V[\x90V[a\x02\xA9b\x9E4\0a\x02\x81V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x02\xCFa\x02\xD5\x91\x93\x92\x93a\x02~V[\x92a\x02~V[\x82\x01\x80\x92\x11a\x02\xE0WV[a\x02\xACV[a\x02\xF9a\x02\xF4a\x02\xFE\x92a\x02HV[a\x02KV[a\x02~V[\x90V[_\x1B\x90V[\x90a\x03\x12_\x19\x91a\x03\x01V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x030a\x03+a\x035\x92a\x02~V[a\x02KV[a\x02~V[\x90V[\x90V[\x90a\x03Pa\x03Ka\x03W\x92a\x03\x1CV[a\x038V[\x82Ta\x03\x06V[\x90UV[\x90V[a\x03ra\x03ma\x03w\x92a\x02HV[a\x03\x01V[a\x03[V[\x90V[a\x03\x83_a\x03^V[\x90V[\x90V[a\x03\x9Da\x03\x98a\x03\xA2\x92a\x03\x86V[a\x02KV[a\x02~V[\x90V[a\x03\xBAk\x02\xE8vi\xC3\x08sj\x04\0\0\0a\x03\x89V[\x90V[\x90a\x03\xDFa\x03\xC9a\x01\xF2V[a\x03\xD1a\x01\xF2V[a\x03\xD9a\x02=V[\x91a\x04\xA5V[\x81a\x03\xFAa\x03\xF4a\x03\xEF_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x04\x89W\x80a\x04\x1Aa\x04\x14a\x04\x0F_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x04mWa\x04\\a\x04k\x92a\x048Ba\x042a\x02\x9DV[\x90a\x02\xC0V[a\x01`Ra\x04Oa\x04H_a\x02\xE5V[`\x0Ca\x03;V[a\x04Wa\x03zV[a\tAV[Pa\x04ea\x03\xA5V[\x90a\n\x0FV[V[_c\xD9.#=`\xE0\x1B\x81R\x80a\x04\x85`\x04\x82\x01a\x02vV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x04\xA1`\x04\x82\x01a\x02vV[\x03\x90\xFD[\x90a\x04\xB0\x92\x91a\x04\xB2V[V[\x90a\x04\xBD\x92\x91a\x04\xBFV[V[\x90a\x04\xCA\x92\x91a\x04\xCCV[V[\x90a\x04\xD7\x92\x91a\x04\xD9V[V[\x90a\x04\xE4\x92\x91a\x051V[V[_\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x05\x17`\x01a\x01\x9BV[\x90a\x05$` \x83\x01a\x04\xE6V[V[a\x05.a\x05\rV[\x90V[\x90a\x05E\x92\x91a\x05?a\x05&V[\x90a\x05GV[V[\x90a\x05S\x93\x92\x91a\x05\x99V[V[\x90V[\x90V[` \x01\x90V[Q\x90V[a\x05ya\x05ta\x05~\x92a\0\xF1V[a\x02KV[a\0\xF1V[\x90V[a\x05\x8A\x90a\x05eV[\x90V[a\x05\x96\x90a\x05\x81V[\x90V[a\x05\xAAa\x05\xFA\x94a\x05\xDF\x93\x94a\x06.V[a\x05\xBE\x81a\x05\xB8`\x06a\x05UV[\x90a\n\xBCV[a\x01 Ra\x05\xD6\x83a\x05\xD0`\x07a\x05UV[\x90a\n\xBCV[a\x01@Ra\x05XV[a\x05\xF1a\x05\xEB\x82a\x05aV[\x91a\x05[V[ `\xE0Ra\x05XV[a\x06\x0Ca\x06\x06\x82a\x05aV[\x91a\x05[V[ a\x01\0RF`\xA0Ra\x06\x1Da\x0B\xC1V[`\x80Ra\x06)0a\x05\x8DV[`\xC0RV[\x90a\x068\x91a\x06:V[V[\x90a\x06D\x91a\x06FV[V[\x90a\x06P\x91a\x08\x97V[V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[Q\x90V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x01`\x02\x83\x04\x92\x16\x80\x15a\x06\x9DW[` \x83\x10\x14a\x06\x98WV[a\x06iV[\x91`\x7F\x16\x91a\x06\x8DV[_R` _ \x90V[`\x1F` \x91\x01\x04\x90V[\x1B\x90V[\x91\x90`\x08a\x06\xD9\x91\x02\x91a\x06\xD3_\x19\x84a\x06\xBAV[\x92a\x06\xBAV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x91\x90a\x06\xF9a\x06\xF4a\x07\x01\x93a\x03\x1CV[a\x038V[\x90\x83Ta\x06\xBEV[\x90UV[_\x90V[a\x07\x1B\x91a\x07\x15a\x07\x05V[\x91a\x06\xE3V[V[[\x81\x81\x10a\x07)WPPV[\x80a\x076_`\x01\x93a\x07\tV[\x01a\x07\x1EV[\x91\x90`\x1F\x81\x11a\x07LW[PPPV[a\x07Xa\x07}\x93a\x06\xA7V[\x90` a\x07d\x84a\x06\xB0V[\x83\x01\x93\x10a\x07\x85W[a\x07v\x90a\x06\xB0V[\x01\x90a\x07\x1DV[_\x80\x80a\x07GV[\x91Pa\x07v\x81\x92\x90Pa\x07mV[\x1C\x90V[\x90a\x07\xA7\x90_\x19\x90`\x08\x02a\x07\x93V[\x19\x16\x90V[\x81a\x07\xB6\x91a\x07\x97V[\x90`\x02\x02\x17\x90V[\x90a\x07\xC8\x81a\x06eV[\x90`\x01\x80`@\x1B\x03\x82\x11a\x08\x86Wa\x07\xEA\x82a\x07\xE4\x85Ta\x06}V[\x85a\x07<V[` \x90`\x1F\x83\x11`\x01\x14a\x08\x1EW\x91\x80\x91a\x08\r\x93_\x92a\x08\x12W[PPa\x07\xACV[\x90U[V[\x90\x91P\x01Q_\x80a\x08\x06V[`\x1F\x19\x83\x16\x91a\x08-\x85a\x06\xA7V[\x92_[\x81\x81\x10a\x08nWP\x91`\x02\x93\x91\x85`\x01\x96\x94\x10a\x08TW[PPP\x02\x01\x90Ua\x08\x10V[a\x08d\x91\x01Q`\x1F\x84\x16\x90a\x07\x97V[\x90U_\x80\x80a\x08HV[\x91\x93` `\x01\x81\x92\x87\x87\x01Q\x81U\x01\x95\x01\x92\x01a\x080V[a\0\x9DV[\x90a\x08\x95\x91a\x07\xBEV[V[\x90a\x08\xA6a\x08\xAD\x92`\x03a\x08\x8BV[`\x04a\x08\x8BV[V[_\x90V[\x15\x15\x90V[a\x08\xC1\x90a\x03[V[\x90V[\x90a\x08\xCE\x90a\x08\xB8V[_R` R`@_ \x90V[a\x08\xE3\x90a\x05\x81V[\x90V[\x90a\x08\xF0\x90a\x08\xDAV[_R` R`@_ \x90V[\x90a\t\x08`\xFF\x91a\x03\x01V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\t\x1B\x90a\x08\xB3V[\x90V[\x90V[\x90a\t6a\t1a\t=\x92a\t\x12V[a\t\x1EV[\x82Ta\x08\xFCV[\x90UV[a\tIa\x08\xAFV[Pa\t^a\tX\x82\x84\x90a\x0C^V[\x15a\x08\xB3V[_\x14a\t\xE7Wa\t\x86`\x01a\t\x81_a\ty`\x05\x86\x90a\x08\xC4V[\x01\x85\x90a\x08\xE6V[a\t!V[\x90a\t\x8Fa\x0C\x8CV[\x90a\t\xCCa\t\xC6a\t\xC0\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x95a\x08\xB8V[\x92a\x08\xDAV[\x92a\x08\xDAV[\x92a\t\xD5a\0\x89V[\x80a\t\xDF\x81a\x02vV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a\t\xF6\x90a\0\xFCV[\x90RV[\x91\x90a\n\r\x90_` \x85\x01\x94\x01\x90a\t\xEDV[V[\x80a\n*a\n$a\n\x1F_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\nFWa\nD\x91a\n<_a\x02jV[\x91\x90\x91a\x0C\xBDV[V[a\nia\nR_a\x02jV[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\t\xFAV[\x03\x90\xFD[_\x90V[\x90V[a\n\x88a\n\x83a\n\x8D\x92a\nqV[a\x02KV[a\x02~V[\x90V[\x90V[a\n\xA7a\n\xA2a\n\xAC\x92a\n\x90V[a\x03\x01V[a\x03[V[\x90V[a\n\xB9`\xFFa\n\x93V[\x90V[\x90a\n\xC5a\nmV[Pa\n\xD7a\n\xD2\x83a\x05XV[a\x05aV[a\n\xEAa\n\xE4` a\ntV[\x91a\x02~V[\x10_\x14a\n\xFEWPa\n\xFB\x90a\x0EWV[\x90V[_a\x0B\x0Ca\x0B\x12\x93\x92a\rgV[\x01a\x08\x8BV[a\x0B\"a\x0B\x1Da\n\xAFV[a\x08\xB8V[\x90V[_\x90V[\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x90V[a\x0BW\x90Qa\x03[V[\x90V[a\x0Bc\x90a\x03[V[\x90RV[a\x0Bp\x90a\x02~V[\x90RV[\x90\x95\x94\x92a\x0B\xBF\x94a\x0B\xAEa\x0B\xB8\x92a\x0B\xA4`\x80\x96a\x0B\x9A`\xA0\x88\x01\x9C_\x89\x01\x90a\x0BZV[` \x87\x01\x90a\x0BZV[`@\x85\x01\x90a\x0BZV[``\x83\x01\x90a\x0BgV[\x01\x90a\t\xEDV[V[a\x0B\xC9a\x0B%V[Pa\x0B\xD2a\x0B)V[a\x0C\x1Ca\x0B\xDF`\xE0a\x0BMV[\x91a\x0C\ra\x0B\xEEa\x01\0a\x0BMV[Fa\x0B\xF80a\x05\x8DV[\x91a\x0C\x01a\0\x89V[\x96\x87\x95` \x87\x01a\x0BtV[` \x82\x01\x81\x03\x82R\x03\x82a\0\xB1V[a\x0C.a\x0C(\x82a\x05aV[\x91a\x05[V[ \x90V[_\x1C\x90V[`\xFF\x16\x90V[a\x0CIa\x0CN\x91a\x0C2V[a\x0C7V[\x90V[a\x0C[\x90Ta\x0C=V[\x90V[a\x0C\x85\x91_a\x0Cza\x0C\x80\x93a\x0Cra\x08\xAFV[P`\x05a\x08\xC4V[\x01a\x08\xE6V[a\x0CQV[\x90V[_\x90V[a\x0C\x94a\x0C\x88V[P3\x90V[\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t\x90V[\x91\x82a\x0C\xD9a\x0C\xD3a\x0C\xCE_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14\x15\x80a\rDW[a\x0C\xF4W[a\x0C\xF2\x92\x91\x90\x91a\x0F{V[V[a\x0C\xFCa\x0F\x05V[\x80a\r#W[\x15a\x0C\xE6W_c6\xE2x\xFD`\xE2\x1B\x81R\x80a\r\x1F`\x04\x82\x01a\x02vV[\x03\x90\xFD[Pa\r?a\r9a\r2a\x0C\x99V[3\x90a\x0C^V[\x15a\x08\xB3V[a\r\x02V[P\x81a\r`a\rZa\rU_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14\x15a\x0C\xE1V[\x90V[\x90V[a\r\x81a\r|a\r\x86\x92a\rjV[a\x02KV[a\x02~V[\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[a\r\xBCa\r\xC5` \x93a\r\xCA\x93a\r\xB3\x81a\x06eV[\x93\x84\x80\x93a\r\x89V[\x95\x86\x91\x01a\r\x92V[a\0\x93V[\x01\x90V[a\r\xE3\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\r\x9DV[\x90V[a\x0E\0a\r\xFBa\r\xF5\x83a\x05aV[\x92a\x05[V[a\x0BMV[\x90` \x81\x10a\x0E\x0EW[P\x90V[a\x0E \x90_\x19\x90` \x03`\x08\x02a\x06\xBAV[\x16_a\x0E\nV[a\x0E3a\x0E8\x91a\x0C2V[a\x03\x1CV[\x90V[a\x0EOa\x0EJa\x0ET\x92a\x02~V[a\x03\x01V[a\x03[V[\x90V[a\x0E_a\nmV[Pa\x0Ei\x81a\x05XV[\x90a\x0Es\x82a\x05aV[a\x0E\x86a\x0E\x80`\x1Fa\rmV[\x91a\x02~V[\x11a\x0E\xBBWPa\x0E\xB3\x81a\x0E\xADa\x0E\xA7a\x0E\xA2a\x0E\xB8\x95a\r\xE6V[a\x0E'V[\x91a\x05aV[\x17a\x0E;V[a\x08\xB8V[\x90V[a\x0E\xDD\x90a\x0E\xC7a\0\x89V[\x91\x82\x91c0Z'\xA9`\xE0\x1B\x83R`\x04\x83\x01a\r\xCEV[\x03\x90\xFD[\x90V[a\x0E\xF0a\x0E\xF5\x91a\x0C2V[a\x0E\xE1V[\x90V[a\x0F\x02\x90Ta\x0E\xE4V[\x90V[a\x0F\ra\x08\xAFV[Pa\x0F\x18`\x0Ca\x0E\xF8V[a\x0F*a\x0F$_a\x02\xE5V[\x91a\x02~V[\x14\x15\x80a\x0F5W[\x90V[PBa\x0FRa\x0FLa\x0FG`\x0Ca\x0E\xF8V[a\x02~V[\x91a\x02~V[\x10a\x0F2V[\x91` a\x0Fy\x92\x94\x93a\x0Fr`@\x82\x01\x96_\x83\x01\x90a\x0BgV[\x01\x90a\x0BgV[V[\x92\x91a\x0F\x89\x84\x83\x83\x91a\x10\x84V[\x83a\x0F\xA4a\x0F\x9Ea\x0F\x99_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x0F\xB9W[a\x0F\xB7\x92\x93\x91\x90\x91a\x12QV[V[a\x0F\xC1a\x11\xF3V[\x93a\x0F\xCAa\x120V[\x94\x80a\x0F\xDEa\x0F\xD8\x88a\x02~V[\x91a\x02~V[\x11a\x0F\xEBWP\x93Pa\x0F\xAAV[\x85\x90a\x10\x07_\x92\x83\x92c\x0EX\xAE\x93`\xE1\x1B\x84R`\x04\x84\x01a\x0FXV[\x03\x90\xFD[\x90a\x10\x15\x90a\x08\xDAV[_R` R`@_ \x90V[`@\x90a\x10Ja\x10Q\x94\x96\x95\x93\x96a\x10@``\x84\x01\x98_\x85\x01\x90a\t\xEDV[` \x83\x01\x90a\x0BgV[\x01\x90a\x0BgV[V[\x90a\x10^\x91\x03a\x02~V[\x90V[\x90a\x10l\x91\x01a\x02~V[\x90V[\x91\x90a\x10\x82\x90_` \x85\x01\x94\x01\x90a\x0BgV[V[\x91\x90\x91\x80a\x10\xA2a\x10\x9Ca\x10\x97_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14_\x14a\x11\x83Wa\x10\xC6a\x10\xBF\x83a\x10\xBA`\x02a\x0E\xF8V[a\x02\xC0V[`\x02a\x03;V[[\x82a\x10\xE2a\x10\xDCa\x10\xD7_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14_\x14a\x11WWa\x11\x06a\x10\xFF\x83a\x10\xFA`\x02a\x0E\xF8V[a\x10SV[`\x02a\x03;V[[\x91\x90\x91a\x11Ra\x11@a\x11:\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93a\x08\xDAV[\x93a\x08\xDAV[\x93a\x11Ia\0\x89V[\x91\x82\x91\x82a\x10oV[\x03\x90\xA3V[a\x11~\x82a\x11xa\x11i_\x87\x90a\x10\x0BV[\x91a\x11s\x83a\x0E\xF8V[a\x10aV[\x90a\x03;V[a\x11\x07V[a\x11\x96a\x11\x91_\x83\x90a\x10\x0BV[a\x0E\xF8V[\x80a\x11\xA9a\x11\xA3\x85a\x02~V[\x91a\x02~V[\x10a\x11\xD1Wa\x11\xBCa\x11\xCC\x91\x84\x90a\x10SV[a\x11\xC7_\x84\x90a\x10\x0BV[a\x03;V[a\x10\xC7V[\x90a\x11\xEF\x90\x91\x92_\x93\x84\x93c9\x144\xE3`\xE2\x1B\x85R`\x04\x85\x01a\x10!V[\x03\x90\xFD[a\x11\xFBa\x07\x05V[Pa\x12\x06`\x02a\x0E\xF8V[\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x12(a\x12#a\x12-\x92a\x12\tV[a\x02KV[a\x02~V[\x90V[a\x128a\x07\x05V[Pa\x12H`\x01\x80`\xD0\x1B\x03a\x12\x14V[\x90V[\x90V[\x90V[\x91a\x12\xA9a\x12\xA3a\x12\xB0\x94\x80a\x12wa\x12qa\x12l_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x12\xE1W[\x84a\x12\x98a\x12\x92a\x12\x8D_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x12\xB2W[a\x14\xD9V[\x92a\x14\xD9V[\x90\x91a\x15\x0EV[V[a\x12\xDA`\x0B`\x02a\x12\xD4a\x12\xCEa\x12\xC8\x89a\x13\xC3V[\x93a\x12KV[\x91a\x12NV[\x90a\x14\x16V[PPa\x12\x9EV[a\x13\t`\x0B`\x01a\x13\x03a\x12\xFDa\x12\xF7\x89a\x13\xC3V[\x93a\x12KV[\x91a\x12NV[\x90a\x14\x16V[PPa\x12}V[_\x90V[a\x13 a\x13&\x91a\x12\tV[\x91a\x12\tV[\x01\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x137WV[a\x02\xACV[\x90a\x13O\x91a\x13Ia\x13\x10V[Pa\x13\x14V[\x90V[\x90V[`\xFF\x16\x90V[a\x13oa\x13ja\x13t\x92a\x13RV[a\x02KV[a\x13UV[\x90V[a\x13\x80\x90a\x13[V[\x90RV[\x91` a\x13\xA5\x92\x94\x93a\x13\x9E`@\x82\x01\x96_\x83\x01\x90a\x13wV[\x01\x90a\x0BgV[V[a\x13\xBBa\x13\xB6a\x13\xC0\x92a\x02~V[a\x02KV[a\x12\tV[\x90V[a\x13\xCBa\x13\x10V[P\x80a\x13\xE5a\x13\xDF`\x01\x80`\xD0\x1B\x03a\x12\x14V[\x91a\x02~V[\x11a\x13\xF6Wa\x13\xF3\x90a\x13\xA7V[\x90V[`\xD0a\x14\x12_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a\x13\x84V[\x03\x90\xFD[\x90a\x14La\x14R\x93\x92a\x14'a\x13\x10V[Pa\x140a\x13\x10V[P\x80\x93a\x14Ea\x14>a\x16\xC0V[\x94\x92a\x17mV[\x90\x91a\x1CSV[\x91a\x17\xE2V[\x91\x90\x91\x90V[a\x14da\x14j\x91a\x12\tV[\x91a\x12\tV[\x90\x03\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x14|WV[a\x02\xACV[\x90a\x14\x94\x91a\x14\x8Ea\x13\x10V[Pa\x14XV[\x90V[\x90a\x14\xA1\x90a\x08\xDAV[_R` R`@_ \x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x14\xC4a\x14\xC9\x91a\x0C2V[a\x14\xADV[\x90V[a\x14\xD6\x90Ta\x14\xB8V[\x90V[a\x14\xF0a\x14\xF5\x91a\x14\xE8a\x0C\x88V[P`\ta\x14\x97V[a\x14\xCCV[\x90V[\x90a\x15\x02\x90a\x08\xDAV[_R` R`@_ \x90V[\x91\x90\x91\x80a\x15$a\x15\x1E\x85a\0\xFCV[\x91a\0\xFCV[\x14\x15\x80a\x16\xA2W[a\x156W[PPPV[\x80a\x15Qa\x15Ka\x15F_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x03a\x16\x12W[P\x81a\x15sa\x15ma\x15h_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x03a\x15\x7FW[\x80a\x151V[a\x15\xC6a\x15\xB9a\x15\xC0\x92a\x15\x95`\n\x86\x90a\x14\xF8V[\x90a\x15\xB3a\x15\xADa\x15\xA7`\x01\x93a\x13\xC3V[\x93a\x12KV[\x91a\x12NV[\x90a\x14\x16V[\x92\x90a\x12\x14V[\x91a\x12\x14V[\x91\x90\x91a\x15\xF3\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x08\xDAV[\x92a\x16\x08a\x15\xFFa\0\x89V[\x92\x83\x92\x83a\x0FXV[\x03\x90\xA2_\x80a\x15yV[a\x16Qa\x16Wa\x16Ja\x16'`\n\x85\x90a\x14\xF8V[`\x02a\x16Da\x16>a\x168\x89a\x13\xC3V[\x93a\x12KV[\x91a\x12NV[\x90a\x14\x16V[\x92\x90a\x12\x14V[\x91a\x12\x14V[\x91\x90\x91a\x16\x84\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x08\xDAV[\x92a\x16\x99a\x16\x90a\0\x89V[\x92\x83\x92\x83a\x0FXV[\x03\x90\xA2_a\x15WV[P\x81a\x16\xB6a\x16\xB0_a\x02\xE5V[\x91a\x02~V[\x11a\x15,V[_\x90V[a\x16\xC8a\x16\xBCV[Pa\x16\xD1a\x18\x11V[\x90V[T\x90V[\x90V[a\x16\xEFa\x16\xEAa\x16\xF4\x92a\x16\xD8V[a\x02KV[a\x02~V[\x90V[a\x17\x06a\x17\x0C\x91\x93\x92\x93a\x02~V[\x92a\x02~V[\x82\x03\x91\x82\x11a\x17\x17WV[a\x02\xACV[\x90V[`0\x1C\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x17<a\x17A\x91a\x17\x1FV[a\x17%V[\x90V[a\x17N\x90Ta\x170V[\x90V[a\x17ea\x17`a\x17j\x92a\x02HV[a\x02KV[a\x12\tV[\x90V[a\x17ua\x13\x10V[Pa\x17\x81_\x82\x01a\x16\xD4V[\x80a\x17\x94a\x17\x8E_a\x02\xE5V[\x91a\x02~V[\x14_\x14a\x17\xAAWPPa\x17\xA6_a\x17QV[[\x90V[a\x17\xD7_\x91a\x17\xD2a\x17\xCC\x84a\x17\xDD\x96\x01\x92a\x17\xC6`\x01a\x16\xDBV[\x90a\x16\xF7V[\x91a\x17\x1CV[a\x18&V[\x01a\x17DV[a\x17\xA7V[\x91a\x18\x06_a\x18\x0B\x94a\x17\xF3a\x13\x10V[Pa\x17\xFCa\x13\x10V[P\x01\x92\x91\x92a\x17\x1CV[a\x1A+V[\x91\x90\x91\x90V[a\x18\x19a\x16\xBCV[Pa\x18#Ca\x1B\xECV[\x90V[_R` _ \x01\x90V[T\x90V[a\x18>`@a\0\xD8V[\x90V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90a\x18V\x90a\x18AV[\x90RV[\x90a\x18d\x90a\x12\tV[\x90RV[_R` _ \x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[a\x18\x8E\x81a\x180V[\x82\x10\x15a\x18\xA8Wa\x18\xA0`\x01\x91a\x18hV[\x91\x02\x01\x90_\x90V[a\x18qV[a\x18\xB7\x90Qa\x18AV[\x90V[\x90a\x18\xCBe\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x03\x01V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x18\xE9a\x18\xE4a\x18\xEE\x92a\x18AV[a\x02KV[a\x18AV[\x90V[\x90V[\x90a\x19\ta\x19\x04a\x19\x10\x92a\x18\xD5V[a\x18\xF1V[\x82Ta\x18\xBAV[\x90UV[a\x19\x1E\x90Qa\x12\tV[\x90V[`0\x1B\x90V[\x90a\x199e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91a\x19!V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x19Wa\x19Ra\x19\\\x92a\x12\tV[a\x02KV[a\x12\tV[\x90V[\x90V[\x90a\x19wa\x19ra\x19~\x92a\x19CV[a\x19_V[\x82Ta\x19'V[\x90UV[\x90a\x19\xAC` _a\x19\xB2\x94a\x19\xA4\x82\x82\x01a\x19\x9E\x84\x88\x01a\x18\xADV[\x90a\x18\xF4V[\x01\x92\x01a\x19\x14V[\x90a\x19bV[V[\x91\x90a\x19\xC5Wa\x19\xC3\x91a\x19\x82V[V[a\x06RV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x19\xFAW\x82a\x19\xF2\x91`\x01a\x19\xF8\x95\x01\x81Ua\x18\x85V[\x90a\x19\xB4V[V[a\0\x9DV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x1A\x16a\x1A\x1B\x91a\x0C2V[a\x19\xFFV[\x90V[a\x1A(\x90Ta\x1A\nV[\x90V[\x90\x92\x91\x92a\x1A7a\x13\x10V[Pa\x1A@a\x13\x10V[Pa\x1AJ\x82a\x180V[\x80a\x1A]a\x1AW_a\x02\xE5V[\x91a\x02~V[\x11_\x14a\x1B-Wa\x1A\x83\x90a\x1A}\x84\x91a\x1Aw`\x01a\x16\xDBV[\x90a\x16\xF7V[\x90a\x18&V[\x90a\x1A\x8F_\x83\x01a\x1A\x1EV[\x92a\x1A\x9B_\x84\x01a\x17DV[\x93\x80a\x1A\xAFa\x1A\xA9\x85a\x18AV[\x91a\x18AV[\x11a\x1B\x11Wa\x1A\xC6a\x1A\xC0\x84a\x18AV[\x91a\x18AV[\x14_\x14a\x1A\xE1WPPa\x1A\xDC\x90_\x85\x91\x01a\x19bV[[\x91\x90V[a\x1B\x0C\x92Pa\x1B\x07\x86a\x1A\xFEa\x1A\xF5a\x184V[\x94_\x86\x01a\x18LV[` \x84\x01a\x18ZV[a\x19\xCAV[a\x1A\xDDV[_c% `\x1D`\xE0\x1B\x81R\x80a\x1B)`\x04\x82\x01a\x02vV[\x03\x90\xFD[Pa\x1BX\x91a\x1BS\x85a\x1BJa\x1BAa\x184V[\x94_\x86\x01a\x18LV[` \x84\x01a\x18ZV[a\x19\xCAV[a\x1Ba_a\x17QV[\x91\x90V[a\x1Bya\x1Bta\x1B~\x92a\x18AV[a\x02KV[a\x02~V[\x90V[\x90V[a\x1B\x98a\x1B\x93a\x1B\x9D\x92a\x1B\x81V[a\x02KV[a\x13UV[\x90V[a\x1B\xA9\x90a\x1B\x84V[\x90RV[\x91` a\x1B\xCE\x92\x94\x93a\x1B\xC7`@\x82\x01\x96_\x83\x01\x90a\x1B\xA0V[\x01\x90a\x0BgV[V[a\x1B\xE4a\x1B\xDFa\x1B\xE9\x92a\x02~V[a\x02KV[a\x18AV[\x90V[a\x1B\xF4a\x16\xBCV[P\x80a\x1C\x0Ea\x1C\x08e\xFF\xFF\xFF\xFF\xFF\xFFa\x1BeV[\x91a\x02~V[\x11a\x1C\x1FWa\x1C\x1C\x90a\x1B\xD0V[\x90V[`0a\x1C;_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a\x1B\xADV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`Q`\x04R`$_\xFD[\x91\x90\x91\x80`\x01\x14a\x1CrW`\x02\x03a\x1C?Wa\x1Cn\x91a\x14\x81V[\x90[V[Pa\x1C|\x91a\x13<V[\x90a\x1CpV\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x14\x8EV[a\0\x1D_5a\x02\xFCV[\x80c\x01\xFF\xC9\xA7\x14a\x02\xF7W\x80c\x06\xFD\xDE\x03\x14a\x02\xF2W\x80c\t^\xA7\xB3\x14a\x02\xEDW\x80c\x18\x16\r\xDD\x14a\x02\xE8W\x80c#\xB8r\xDD\x14a\x02\xE3W\x80c$\x8A\x9C\xA3\x14a\x02\xDEW\x80c//\xF1]\x14a\x02\xD9W\x80c1<\xE5g\x14a\x02\xD4W\x80c6D\xE5\x15\x14a\x02\xCFW\x80c6V\x8A\xBE\x14a\x02\xCAW\x80c:F\xB1\xA8\x14a\x02\xC5W\x80c@\xC1\x0F\x19\x14a\x02\xC0W\x80cB\x96lh\x14a\x02\xBBW\x80cK\xF5\xD7\xE9\x14a\x02\xB6W\x80cO\x1B\xFC\x9E\x14a\x02\xB1W\x80cX|\xDE\x1E\x14a\x02\xACW\x80c\\\x19\xA9\\\x14a\x02\xA7W\x80co\xCF\xFFE\x14a\x02\xA2W\x80cp\xA0\x821\x14a\x02\x9DW\x80cy\xCCg\x90\x14a\x02\x98W\x80cz\x8C\xD1V\x14a\x02\x93W\x80c~\xCE\xBE\0\x14a\x02\x8EW\x80c\x83\xF1!\x1B\x14a\x02\x89W\x80c\x84&\xAD\xF2\x14a\x02\x84W\x80c\x84L\x90&\x14a\x02\x7FW\x80c\x84\xB0\x19n\x14a\x02zW\x80c\x8AT%!\x14a\x02uW\x80c\x8D3C\xD6\x14a\x02pW\x80c\x8ES\x9E\x8C\x14a\x02kW\x80c\x90-U\xA5\x14a\x02fW\x80c\x91\xD1HT\x14a\x02aW\x80c\x91\xDD\xAD\xF4\x14a\x02\\W\x80c\x95\xD8\x9BA\x14a\x02WW\x80c\x9A\xB2N\xB0\x14a\x02RW\x80c\x9B~\xF6K\x14a\x02MW\x80c\xA2\x17\xFD\xDF\x14a\x02HW\x80c\xA9\x05\x9C\xBB\x14a\x02CW\x80c\xAA\x08*\x9D\x14a\x02>W\x80c\xB0\xCA%>\x14a\x029W\x80c\xBBMD6\x14a\x024W\x80c\xC0*\xE7T\x14a\x02/W\x80c\xC3\xCD\xA5 \x14a\x02*W\x80c\xD5\x05\xAC\xCF\x14a\x02%W\x80c\xD5Gt\x1F\x14a\x02 W\x80c\xDDb\xED>\x14a\x02\x1BWc\xF1\x12~\xD8\x03a\0\x0EWa\x14XV[a\x13tV[a\x13\x13V[a\x12\xD9V[a\x12/V[a\x11sV[a\x11>V[a\x11\x08V[a\x10\xD3V[a\x10aV[a\x10,V[a\x0F\xBCV[a\x0FEV[a\x0F\x10V[a\x0E\xDBV[a\x0ExV[a\x0ECV[a\r\xCCV[a\r\x97V[a\r3V[a\x0C\xC8V[a\x0B\x83V[a\x0BNV[a\n\xF5V[a\n\xC0V[a\n\x8BV[a\nWV[a\n\"V[a\t\xEDV[a\t\x8FV[a\tZV[a\x08\xE5V[a\x08tV[a\x08AV[a\x07\xEFV[a\x07\xB9V[a\x07\x85V[a\x07PV[a\x07\x1BV[a\x06\xBFV[a\x06XV[a\x05\xBCV[a\x05MV[a\x04\xF5V[a\x043V[a\x03\x84V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[c\xFF\xFF\xFF\xFF`\xE0\x1B\x16\x90V[a\x03%\x81a\x03\x10V[\x03a\x03,WV[_\x80\xFD[\x90P5\x90a\x03=\x82a\x03\x1CV[V[\x90` \x82\x82\x03\x12a\x03XWa\x03U\x91_\x01a\x030V[\x90V[a\x03\x0CV[\x15\x15\x90V[a\x03k\x90a\x03]V[\x90RV[\x91\x90a\x03\x82\x90_` \x85\x01\x94\x01\x90a\x03bV[V[4a\x03\xB4Wa\x03\xB0a\x03\x9Fa\x03\x9A6`\x04a\x03?V[a\x15+V[a\x03\xA7a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[_\x91\x03\x12a\x03\xC3WV[a\x03\x0CV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x04\ta\x04\x12` \x93a\x04\x17\x93a\x04\0\x81a\x03\xC8V[\x93\x84\x80\x93a\x03\xCCV[\x95\x86\x91\x01a\x03\xD5V[a\x03\xE0V[\x01\x90V[a\x040\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03\xEAV[\x90V[4a\x04cWa\x04C6`\x04a\x03\xB9V[a\x04_a\x04Na\x16\xC4V[a\x04Va\x03\x02V[\x91\x82\x91\x82a\x04\x1BV[\x03\x90\xF3[a\x03\x08V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04|\x90a\x04hV[\x90V[a\x04\x88\x81a\x04sV[\x03a\x04\x8FWV[_\x80\xFD[\x90P5\x90a\x04\xA0\x82a\x04\x7FV[V[\x90V[a\x04\xAE\x81a\x04\xA2V[\x03a\x04\xB5WV[_\x80\xFD[\x90P5\x90a\x04\xC6\x82a\x04\xA5V[V[\x91\x90`@\x83\x82\x03\x12a\x04\xF0W\x80a\x04\xE4a\x04\xED\x92_\x86\x01a\x04\x93V[\x93` \x01a\x04\xB9V[\x90V[a\x03\x0CV[4a\x05&Wa\x05\"a\x05\x11a\x05\x0B6`\x04a\x04\xC8V[\x90a\x16\xDAV[a\x05\x19a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[a\x054\x90a\x04\xA2V[\x90RV[\x91\x90a\x05K\x90_` \x85\x01\x94\x01\x90a\x05+V[V[4a\x05}Wa\x05]6`\x04a\x03\xB9V[a\x05ya\x05ha\x17&V[a\x05pa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90\x91``\x82\x84\x03\x12a\x05\xB7Wa\x05\xB4a\x05\x9D\x84_\x85\x01a\x04\x93V[\x93a\x05\xAB\x81` \x86\x01a\x04\x93V[\x93`@\x01a\x04\xB9V[\x90V[a\x03\x0CV[4a\x05\xEDWa\x05\xE9a\x05\xD8a\x05\xD26`\x04a\x05\x82V[\x91a\x17<V[a\x05\xE0a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[\x90V[a\x05\xFE\x81a\x05\xF2V[\x03a\x06\x05WV[_\x80\xFD[\x90P5\x90a\x06\x16\x82a\x05\xF5V[V[\x90` \x82\x82\x03\x12a\x061Wa\x06.\x91_\x01a\x06\tV[\x90V[a\x03\x0CV[a\x06?\x90a\x05\xF2V[\x90RV[\x91\x90a\x06V\x90_` \x85\x01\x94\x01\x90a\x066V[V[4a\x06\x88Wa\x06\x84a\x06sa\x06n6`\x04a\x06\x18V[a\x17\xB5V[a\x06{a\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[\x91\x90`@\x83\x82\x03\x12a\x06\xB5W\x80a\x06\xA9a\x06\xB2\x92_\x86\x01a\x06\tV[\x93` \x01a\x04\x93V[\x90V[a\x03\x0CV[_\x01\x90V[4a\x06\xEEWa\x06\xD8a\x06\xD26`\x04a\x06\x8DV[\x90a\x18\x01V[a\x06\xE0a\x03\x02V[\x80a\x06\xEA\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[`\xFF\x16\x90V[a\x07\x02\x90a\x06\xF3V[\x90RV[\x91\x90a\x07\x19\x90_` \x85\x01\x94\x01\x90a\x06\xF9V[V[4a\x07KWa\x07+6`\x04a\x03\xB9V[a\x07Ga\x076a\x180V[a\x07>a\x03\x02V[\x91\x82\x91\x82a\x07\x06V[\x03\x90\xF3[a\x03\x08V[4a\x07\x80Wa\x07`6`\x04a\x03\xB9V[a\x07|a\x07ka\x18FV[a\x07sa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[4a\x07\xB4Wa\x07\x9Ea\x07\x986`\x04a\x06\x8DV[\x90a\x18ZV[a\x07\xA6a\x03\x02V[\x80a\x07\xB0\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\x07\xEAWa\x07\xE6a\x07\xD5a\x07\xCF6`\x04a\x04\xC8V[\x90a\x19\x0BV[a\x07\xDDa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x08\x1EWa\x08\x08a\x08\x026`\x04a\x04\xC8V[\x90a\x1A\x92V[a\x08\x10a\x03\x02V[\x80a\x08\x1A\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[\x90` \x82\x82\x03\x12a\x08<Wa\x089\x91_\x01a\x04\xB9V[\x90V[a\x03\x0CV[4a\x08oWa\x08Ya\x08T6`\x04a\x08#V[a\x1A\x9EV[a\x08aa\x03\x02V[\x80a\x08k\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\x08\xA4Wa\x08\x846`\x04a\x03\xB9V[a\x08\xA0a\x08\x8Fa\x1BxV[a\x08\x97a\x03\x02V[\x91\x82\x91\x82a\x04\x1BV[\x03\x90\xF3[a\x03\x08V[\x90V[\x90V[a\x08\xC3a\x08\xBEa\x08\xC8\x92a\x08\xA9V[a\x08\xACV[a\x04\xA2V[\x90V[a\x08\xD7b\x9E4\0a\x08\xAFV[\x90V[a\x08\xE2a\x08\xCBV[\x90V[4a\t\x15Wa\x08\xF56`\x04a\x03\xB9V[a\t\x11a\t\0a\x08\xDAV[a\t\x08a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90` \x82\x82\x03\x12a\t3Wa\t0\x91_\x01a\x04\x93V[\x90V[a\x03\x0CV[a\tA\x90a\x04sV[\x90RV[\x91\x90a\tX\x90_` \x85\x01\x94\x01\x90a\t8V[V[4a\t\x8AWa\t\x86a\tua\tp6`\x04a\t\x1AV[a\x1C\x14V[a\t}a\x03\x02V[\x91\x82\x91\x82a\tEV[\x03\x90\xF3[a\x03\x08V[4a\t\xBDWa\t\xA7a\t\xA26`\x04a\t\x1AV[a\x1C3V[a\t\xAFa\x03\x02V[\x80a\t\xB9\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[c\xFF\xFF\xFF\xFF\x16\x90V[a\t\xD4\x90a\t\xC2V[\x90RV[\x91\x90a\t\xEB\x90_` \x85\x01\x94\x01\x90a\t\xCBV[V[4a\n\x1DWa\n\x19a\n\x08a\n\x036`\x04a\t\x1AV[a\x1CJV[a\n\x10a\x03\x02V[\x91\x82\x91\x82a\t\xD8V[\x03\x90\xF3[a\x03\x08V[4a\nRWa\nNa\n=a\n86`\x04a\t\x1AV[a\x1CuV[a\nEa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\n\x86Wa\npa\nj6`\x04a\x04\xC8V[\x90a\x1D\xAAV[a\nxa\x03\x02V[\x80a\n\x82\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\n\xBBWa\n\x9B6`\x04a\x03\xB9V[a\n\xB7a\n\xA6a\x1D\xDBV[a\n\xAEa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\n\xF0Wa\n\xECa\n\xDBa\n\xD66`\x04a\t\x1AV[a\x1ESV[a\n\xE3a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x0B%Wa\x0B\x056`\x04a\x03\xB9V[a\x0B!a\x0B\x10a\x1EhV[a\x0B\x18a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0B~Wa\x0B^6`\x04a\x03\xB9V[a\x0Bza\x0Bia\x0B*V[a\x0Bqa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x0B\xB1Wa\x0B\x9Ba\x0B\x966`\x04a\x08#V[a 3V[a\x0B\xA3a\x03\x02V[\x80a\x0B\xAD\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[`\xFF`\xF8\x1B\x16\x90V[a\x0B\xC8\x90a\x0B\xB6V[\x90RV[Q\x90V[` \x91\x81R\x01\x90V[` \x01\x90V[a\x0B\xE8\x90a\x04\xA2V[\x90RV[\x90a\x0B\xF9\x81` \x93a\x0B\xDFV[\x01\x90V[` \x01\x90V[\x90a\x0C a\x0C\x1Aa\x0C\x13\x84a\x0B\xCCV[\x80\x93a\x0B\xD0V[\x92a\x0B\xD9V[\x90_[\x81\x81\x10a\x0C0WPPP\x90V[\x90\x91\x92a\x0CIa\x0CC`\x01\x92\x86Qa\x0B\xECV[\x94a\x0B\xFDV[\x91\x01\x91\x90\x91a\x0C#V[\x93\x95\x91\x94a\x0C\xA4a\x0C\x99a\x0C\xB8\x95a\x0C\x8Ba\x0C\xAE\x95a\x0C\xC5\x9C\x9Aa\x0C~`\xE0\x8C\x01\x92_\x8D\x01\x90a\x0B\xBFV[\x8A\x82\x03` \x8C\x01Ra\x03\xEAV[\x90\x88\x82\x03`@\x8A\x01Ra\x03\xEAV[\x97``\x87\x01\x90a\x05+V[`\x80\x85\x01\x90a\t8V[`\xA0\x83\x01\x90a\x066V[`\xC0\x81\x84\x03\x91\x01Ra\x0C\x03V[\x90V[4a\x0C\xFFWa\x0C\xD86`\x04a\x03\xB9V[a\x0C\xFBa\x0C\xE3a \xBBV[\x93a\x0C\xF2\x97\x95\x97\x93\x91\x93a\x03\x02V[\x97\x88\x97\x88a\x0CSV[\x03\x90\xF3[a\x03\x08V[\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t\x90V[a\r0a\r\x04V[\x90V[4a\rcWa\rC6`\x04a\x03\xB9V[a\r_a\rNa\r(V[a\rVa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[\x7F\x9B\x12\xE0\xC5p~II\x15\xE5\x8B\x05d\xF1\x8A\xAA\xD9\xB7J\xC6\x9B\xFC\x81Z\x1E\xDA\xDC\x8EK\xD02\xEB\x90V[a\r\x94a\rhV[\x90V[4a\r\xC7Wa\r\xA76`\x04a\x03\xB9V[a\r\xC3a\r\xB2a\r\x8CV[a\r\xBAa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[4a\r\xFCWa\r\xF8a\r\xE7a\r\xE26`\x04a\x08#V[a!EV[a\r\xEFa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90V[a\x0E\x18a\x0E\x13a\x0E\x1D\x92a\x0E\x01V[a\x08\xACV[a\x04\xA2V[\x90V[a\x0E5k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x0E\x04V[\x90V[a\x0E@a\x0E V[\x90V[4a\x0EsWa\x0ES6`\x04a\x03\xB9V[a\x0Eoa\x0E^a\x0E8V[a\x0Efa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x0E\xA9Wa\x0E\xA5a\x0E\x94a\x0E\x8E6`\x04a\x06\x8DV[\x90a!\xB3V[a\x0E\x9Ca\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x0E\xC2\x90a\x0E\xAEV[\x90RV[\x91\x90a\x0E\xD9\x90_` \x85\x01\x94\x01\x90a\x0E\xB9V[V[4a\x0F\x0BWa\x0E\xEB6`\x04a\x03\xB9V[a\x0F\x07a\x0E\xF6a!\xE1V[a\x0E\xFEa\x03\x02V[\x91\x82\x91\x82a\x0E\xC6V[\x03\x90\xF3[a\x03\x08V[4a\x0F@Wa\x0F 6`\x04a\x03\xB9V[a\x0F<a\x0F+a!\xF5V[a\x0F3a\x03\x02V[\x91\x82\x91\x82a\x04\x1BV[\x03\x90\xF3[a\x03\x08V[4a\x0FuWa\x0Fqa\x0F`a\x0F[6`\x04a\t\x1AV[a\"\x0BV[a\x0Fha\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90V[a\x0F\x91a\x0F\x8Ca\x0F\x96\x92a\x0FzV[a\x08\xACV[a\x04\xA2V[\x90V[a\x0F\xAEk\x02\xE8vi\xC3\x08sj\x04\0\0\0a\x0F}V[\x90V[a\x0F\xB9a\x0F\x99V[\x90V[4a\x0F\xECWa\x0F\xCC6`\x04a\x03\xB9V[a\x0F\xE8a\x0F\xD7a\x0F\xB1V[a\x0F\xDFa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90V[_\x1B\x90V[a\x10\ra\x10\x08a\x10\x12\x92a\x0F\xF1V[a\x0F\xF4V[a\x05\xF2V[\x90V[a\x10\x1E_a\x0F\xF9V[\x90V[a\x10)a\x10\x15V[\x90V[4a\x10\\Wa\x10<6`\x04a\x03\xB9V[a\x10Xa\x10Ga\x10!V[a\x10Oa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[4a\x10\x92Wa\x10\x8Ea\x10}a\x10w6`\x04a\x04\xC8V[\x90a\":V[a\x10\x85a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[\x1C\x90V[\x90V[a\x10\xAE\x90`\x08a\x10\xB3\x93\x02a\x10\x97V[a\x10\x9BV[\x90V[\x90a\x10\xC1\x91Ta\x10\x9EV[\x90V[a\x10\xD0`\x0C_\x90a\x10\xB6V[\x90V[4a\x11\x03Wa\x10\xE36`\x04a\x03\xB9V[a\x10\xFFa\x10\xEEa\x10\xC4V[a\x10\xF6a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x119Wa\x115a\x11$a\x11\x1E6`\x04a\x04\xC8V[\x90a\"\\V[a\x11,a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x11nWa\x11ja\x11Ya\x11T6`\x04a\t\x1AV[a\"rV[a\x11aa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x11\xA3Wa\x11\x836`\x04a\x03\xB9V[a\x11\x9Fa\x11\x8Ea\"\x87V[a\x11\x96a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[a\x11\xB1\x81a\x06\xF3V[\x03a\x11\xB8WV[_\x80\xFD[\x90P5\x90a\x11\xC9\x82a\x11\xA8V[V[\x90\x91`\xC0\x82\x84\x03\x12a\x12*Wa\x11\xE3\x83_\x84\x01a\x04\x93V[\x92a\x11\xF1\x81` \x85\x01a\x04\xB9V[\x92a\x11\xFF\x82`@\x83\x01a\x04\xB9V[\x92a\x12'a\x12\x10\x84``\x85\x01a\x11\xBCV[\x93a\x12\x1E\x81`\x80\x86\x01a\x06\tV[\x93`\xA0\x01a\x06\tV[\x90V[a\x03\x0CV[4a\x12dWa\x12Na\x12B6`\x04a\x11\xCBV[\x94\x93\x90\x93\x92\x91\x92a#\x07V[a\x12Va\x03\x02V[\x80a\x12`\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[`\xE0\x81\x83\x03\x12a\x12\xD4Wa\x12\x7F\x82_\x83\x01a\x04\x93V[\x92a\x12\x8D\x83` \x84\x01a\x04\x93V[\x92a\x12\x9B\x81`@\x85\x01a\x04\xB9V[\x92a\x12\xA9\x82``\x83\x01a\x04\xB9V[\x92a\x12\xD1a\x12\xBA\x84`\x80\x85\x01a\x11\xBCV[\x93a\x12\xC8\x81`\xA0\x86\x01a\x06\tV[\x93`\xC0\x01a\x06\tV[\x90V[a\x03\x0CV[4a\x13\x0EWa\x12\xF8a\x12\xEC6`\x04a\x12iV[\x95\x94\x90\x94\x93\x91\x93a$[V[a\x13\0a\x03\x02V[\x80a\x13\n\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\x13BWa\x13,a\x13&6`\x04a\x06\x8DV[\x90a%yV[a\x134a\x03\x02V[\x80a\x13>\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[\x91\x90`@\x83\x82\x03\x12a\x13oW\x80a\x13ca\x13l\x92_\x86\x01a\x04\x93V[\x93` \x01a\x04\x93V[\x90V[a\x03\x0CV[4a\x13\xA5Wa\x13\xA1a\x13\x90a\x13\x8A6`\x04a\x13GV[\x90a%\x9BV[a\x13\x98a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[a\x13\xB3\x81a\t\xC2V[\x03a\x13\xBAWV[_\x80\xFD[\x90P5\x90a\x13\xCB\x82a\x13\xAAV[V[\x91\x90`@\x83\x82\x03\x12a\x13\xF5W\x80a\x13\xE9a\x13\xF2\x92_\x86\x01a\x04\x93V[\x93` \x01a\x13\xBEV[\x90V[a\x03\x0CV[a\x14\x03\x90a\x0E\xAEV[\x90RV[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x14\x1B\x90a\x14\x07V[\x90RV[\x90` \x80a\x14A\x93a\x147_\x82\x01Q_\x86\x01\x90a\x13\xFAV[\x01Q\x91\x01\x90a\x14\x12V[V[\x91\x90a\x14V\x90_`@\x85\x01\x94\x01\x90a\x14\x1FV[V[4a\x14\x89Wa\x14\x85a\x14ta\x14n6`\x04a\x13\xCDV[\x90a&\tV[a\x14|a\x03\x02V[\x91\x82\x91\x82a\x14CV[\x03\x90\xF3[a\x03\x08V[_\x80\xFD[_\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x14\xB6a\x14\xBC\x91a\x14\x07V[\x91a\x14\x07V[\x01\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x14\xCDWV[a\x14\x96V[\x90a\x14\xE5\x91a\x14\xDFa\x14\x92V[Pa\x14\xAAV[\x90V[a\x14\xF4a\x14\xFA\x91a\x14\x07V[\x91a\x14\x07V[\x90\x03\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x15\x0CWV[a\x14\x96V[\x90a\x15$\x91a\x15\x1Ea\x14\x92V[Pa\x14\xE8V[\x90V[_\x90V[a\x153a\x15'V[P\x80a\x15Na\x15Hcye\xDB\x0B`\xE0\x1Ba\x03\x10V[\x91a\x03\x10V[\x14\x90\x81\x15a\x15[W[P\x90V[a\x15e\x91Pa&\x1FV[_a\x15WV[``\x90V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x01`\x02\x83\x04\x92\x16\x80\x15a\x15\xA4W[` \x83\x10\x14a\x15\x9FWV[a\x15pV[\x91`\x7F\x16\x91a\x15\x94V[` \x91\x81R\x01\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a\x15\xDAa\x15\xD3\x83a\x15\x84V[\x80\x94a\x15\xAEV[\x91`\x01\x81\x16\x90\x81_\x14a\x161WP`\x01\x14a\x15\xF5W[PPPV[a\x16\x02\x91\x92\x93\x94Pa\x15\xB7V[\x91_\x92[\x81\x84\x10a\x16\x19WPP\x01\x90_\x80\x80a\x15\xF0V[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a\x16\x06V[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a\x15\xF0V[\x90a\x16V\x91a\x15\xC0V[\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x16w\x90a\x03\xE0V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x16\x91W`@RV[a\x16YV[\x90a\x16\xB6a\x16\xAF\x92a\x16\xA6a\x03\x02V[\x93\x84\x80\x92a\x16LV[\x03\x83a\x16mV[V[a\x16\xC1\x90a\x16\x96V[\x90V[a\x16\xCCa\x15kV[Pa\x16\xD7`\x03a\x16\xB8V[\x90V[a\x16\xF7\x91a\x16\xE6a\x15'V[Pa\x16\xEFa&EV[\x91\x90\x91a&RV[`\x01\x90V[_\x90V[_\x1C\x90V[a\x17\x11a\x17\x16\x91a\x17\0V[a\x10\x9BV[\x90V[a\x17#\x90Ta\x17\x05V[\x90V[a\x17.a\x16\xFCV[Pa\x179`\x02a\x17\x19V[\x90V[\x91a\x17f\x92a\x17Ia\x15'V[Pa\x17^a\x17Ua&EV[\x82\x90\x84\x91a&\xA2V[\x91\x90\x91a'.V[`\x01\x90V[_\x90V[a\x17x\x90a\x05\xF2V[\x90V[\x90a\x17\x85\x90a\x17oV[_R` R`@_ \x90V[\x90V[a\x17\xA0a\x17\xA5\x91a\x17\0V[a\x17\x91V[\x90V[a\x17\xB2\x90Ta\x17\x94V[\x90V[`\x01a\x17\xCEa\x17\xD4\x92a\x17\xC6a\x17kV[P`\x05a\x17{V[\x01a\x17\xA8V[\x90V[\x90a\x17\xF2\x91a\x17\xEDa\x17\xE8\x82a\x17\xB5V[a'\xCBV[a\x17\xF4V[V[\x90a\x17\xFE\x91a($V[PV[\x90a\x18\x0B\x91a\x17\xD7V[V[_\x90V[\x90V[a\x18(a\x18#a\x18-\x92a\x18\x11V[a\x08\xACV[a\x06\xF3V[\x90V[a\x188a\x18\rV[Pa\x18C`\x12a\x18\x14V[\x90V[a\x18Na\x17kV[Pa\x18Wa(\xD0V[\x90V[\x90\x80a\x18ua\x18oa\x18ja&EV[a\x04sV[\x91a\x04sV[\x03a\x18\x86Wa\x18\x83\x91a)\x8AV[PV[_c3K\xD9\x19`\xE1\x1B\x81R\x80a\x18\x9E`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[a\x18\xB6a\x18\xB1a\x18\xBB\x92a\x04hV[a\x08\xACV[a\x04hV[\x90V[a\x18\xC7\x90a\x18\xA2V[\x90V[a\x18\xD3\x90a\x18\xBEV[\x90V[\x90a\x18\xE0\x90a\x18\xCAV[_R` R`@_ \x90V[\x90V[a\x19\x03a\x18\xFEa\x19\x08\x92a\x14\x07V[a\x08\xACV[a\x04\xA2V[\x90V[a\x19B\x91a\x197a\x191a\x19,a\x19=\x94a\x19$a\x16\xFCV[P`\na\x18\xD6V[a\x18\xECV[\x91a*kV[\x90a+\x80V[a\x18\xEFV[\x90V[\x90a\x19_\x91a\x19Za\x19Ua\rhV[a'\xCBV[a\x19\xCAV[V[a\x19ua\x19pa\x19z\x92a\x0F\xF1V[a\x08\xACV[a\x04hV[\x90V[a\x19\x86\x90a\x19aV[\x90V[a\x19\x9Da\x19\x98a\x19\xA2\x92a\x0F\xF1V[a\x08\xACV[a\x04\xA2V[\x90V[a\x19\xB4a\x19\xBA\x91\x93\x92\x93a\x04\xA2V[\x92a\x04\xA2V[\x82\x01\x80\x92\x11a\x19\xC5WV[a\x14\x96V[\x90\x81a\x19\xE6a\x19\xE0a\x19\xDB_a\x19}V[a\x04sV[\x91a\x04sV[\x14a\x1AvW\x80a\x19\xFEa\x19\xF8_a\x19\x89V[\x91a\x04\xA2V[\x14a\x1AZWa\x1A\x15a\x1A\x0Ea\x17&V[\x82\x90a\x19\xA5V[a\x1A.a\x1A(a\x1A#a\x0E V[a\x04\xA2V[\x91a\x04\xA2V[\x11a\x1A>Wa\x1A<\x91a,\xA7V[V[_c\x17~?\xC3`\xE0\x1B\x81R\x80a\x1AV`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1Ar`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x1A\x8E`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x90a\x1A\x9C\x91a\x19EV[V[\x80a\x1A\xB1a\x1A\xAB_a\x19\x89V[\x91a\x04\xA2V[\x14a\x1A\xC2Wa\x1A\xC0\x903a-\x05V[V[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1A\xDA`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x90a\x1A\xF1a\x1A\xEAa\x03\x02V[\x92\x83a\x16mV[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1B\x11Wa\x1B\r` \x91a\x03\xE0V[\x01\x90V[a\x16YV[\x90a\x1B(a\x1B#\x83a\x1A\xF3V[a\x1A\xDEV[\x91\x82RV[_\x7Fmode=blocknumber&from=default\0\0\0\x91\x01RV[a\x1B^`\x1Da\x1B\x16V[\x90a\x1Bk` \x83\x01a\x1B-V[V[a\x1Bua\x1BTV[\x90V[a\x1B\x80a\x15kV[Pa\x1B\x89a!\xE1V[a\x1B\xA2a\x1B\x9Ca\x1B\x97a-dV[a\x0E\xAEV[\x91a\x0E\xAEV[\x03a\x1B\xB2Wa\x1B\xAFa\x1BmV[\x90V[_c\x01\xBF\xC1\xC5`\xE6\x1B\x81R\x80a\x1B\xCA`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_\x90V[\x90a\x1B\xDC\x90a\x18\xCAV[_R` R`@_ \x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x1B\xFFa\x1C\x04\x91a\x17\0V[a\x1B\xE8V[\x90V[a\x1C\x11\x90Ta\x1B\xF3V[\x90V[a\x1C+a\x1C0\x91a\x1C#a\x1B\xCEV[P`\ta\x1B\xD2V[a\x1C\x07V[\x90V[a\x1CD\x90a\x1C?a&EV[a-\xB7V[V[_\x90V[a\x1C\\\x90a\x1CVa\x1CFV[Pa.BV[\x90V[\x90a\x1Ci\x90a\x18\xCAV[_R` R`@_ \x90V[a\x1C\x8Ba\x1C\x90\x91a\x1C\x84a\x16\xFCV[P_a\x1C_V[a\x17\x19V[\x90V[\x90a\x1C\xAD\x91a\x1C\xA8a\x1C\xA3a\r\x04V[a'\xCBV[a\x1C\xAFV[V[\x80a\x1C\xCAa\x1C\xC4a\x1C\xBF_a\x19}V[a\x04sV[\x91a\x04sV[\x14a\x1D\x8EW\x81a\x1C\xE2a\x1C\xDC_a\x19\x89V[\x91a\x04\xA2V[\x14a\x1DrWa\x1C\xF8a\x1C\xF2a\x1EhV[\x15a\x03]V[a\x1DVWa\x1D\x07\x81\x83\x90a-\x05V[3\x90a\x1DQa\x1D?a\x1D9\x7F\xBE\xF4\xF8\x1C\x18\x14\xC6A\xED\xE8^\xBA\xAC\xF1\x9D\x04\x8B,[U\x98\n\xDF\xA6\xEF\x0F\x95le\x135\xA2\x93a\x18\xCAV[\x93a\x18\xCAV[\x93a\x1DHa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xA3V[_c\xB8\xB5\xCA-`\xE0\x1B\x81R\x80a\x1Dn`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1D\x8A`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x1D\xA6`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x90a\x1D\xB4\x91a\x1C\x93V[V[a\x1D\xC5a\x1D\xCB\x91\x93\x92\x93a\x04\xA2V[\x92a\x04\xA2V[\x82\x03\x91\x82\x11a\x1D\xD6WV[a\x14\x96V[a\x1D\xE3a\x16\xFCV[Pa\x1D\xEE`\x0Ca\x17\x19V[a\x1E\0a\x1D\xFA_a\x19\x89V[\x91a\x04\xA2V[\x14\x80\x15a\x1E/W[a\x1E#Wa\x1E a\x1E\x19`\x0Ca\x17\x19V[B\x90a\x1D\xB6V[\x90V[a\x1E,_a\x19\x89V[\x90V[PBa\x1ELa\x1EFa\x1EA`\x0Ca\x17\x19V[a\x04\xA2V[\x91a\x04\xA2V[\x10\x15a\x1E\x08V[a\x1Ee\x90a\x1E_a\x16\xFCV[Pa.qV[\x90V[a\x1Epa\x15'V[Pa\x1E{`\x0Ca\x17\x19V[a\x1E\x8Da\x1E\x87_a\x19\x89V[\x91a\x04\xA2V[\x14\x15\x80a\x1E\x98W[\x90V[PBa\x1E\xB5a\x1E\xAFa\x1E\xAA`\x0Ca\x17\x19V[a\x04\xA2V[\x91a\x04\xA2V[\x10a\x1E\x95V[a\x1E\xD4\x90a\x1E\xCFa\x1E\xCAa\x10\x15V[a'\xCBV[a\x1FNV[V[\x90a\x1E\xE2_\x19\x91a\x0F\xF4V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x1F\0a\x1E\xFBa\x1F\x05\x92a\x04\xA2V[a\x08\xACV[a\x04\xA2V[\x90V[\x90V[\x90a\x1F a\x1F\x1Ba\x1F'\x92a\x1E\xECV[a\x1F\x08V[\x82Ta\x1E\xD6V[\x90UV[\x91` a\x1FL\x92\x94\x93a\x1FE`@\x82\x01\x96_\x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[\x80a\x1Faa\x1F[Ba\x04\xA2V[\x91a\x04\xA2V[\x11\x15a \x17W\x80a\x1F\x9Aa\x1F\x94\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xA2V[\x91a\x04\xA2V[\x11a\x1F\xFBWa\x1F\xA9`\x0Ca\x17\x19V[a\x1F\xB4\x82`\x0Ca\x1F\x0BV[\x903\x90a\x1F\xE1\x7F\xDDh\x96\xDC\xF1\xD4\xB3\x11\xCC\xA8}\xD1\x9B\xBB\xA2\xEA\x9C\xE2\xF8g\xC1V\x88x\xA0C\x8Af\xA1\xAF\xEE\xEC\x92a\x18\xCAV[\x92a\x1F\xF6a\x1F\xEDa\x03\x02V[\x92\x83\x92\x83a\x1F+V[\x03\x90\xA2V[_c\xEFi\xAFe`\xE0\x1B\x81R\x80a \x13`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\xA5e\x83S`\xE0\x1B\x81R\x80a /`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[a <\x90a\x1E\xBBV[V[_\x90V[``\x90V[a P\x90a\x18\xBEV[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a kW` \x80\x91\x02\x01\x90V[a\x16YV[\x90a \x82a }\x83a SV[a\x1A\xDEV[\x91\x82RV[6\x907V[\x90a \xB1a \x99\x83a pV[\x92` \x80a \xA7\x86\x93a SV[\x92\x01\x91\x03\x90a \x87V[V[`\x0F`\xF8\x1B\x90V[a \xC3a >V[Pa \xCCa\x15kV[Pa \xD5a\x15kV[Pa \xDEa\x16\xFCV[Pa \xE7a\x1B\xCEV[Pa \xF0a\x17kV[Pa \xF9a BV[Pa!\x02a.\x89V[\x90a!\x0Ba.\xC9V[\x90F\x90a!\x170a GV[\x90a!!_a\x0F\xF9V[\x90a!3a!._a\x19\x89V[a \x8CV[\x90a!<a \xB3V[\x96\x95\x94\x93\x92\x91\x90V[a!na!s\x91a!Ta\x16\xFCV[Pa!ha!b`\x0Ba\x18\xECV[\x91a*kV[\x90a+\x80V[a\x18\xEFV[\x90V[\x90a!\x80\x90a\x18\xCAV[_R` R`@_ \x90V[`\xFF\x16\x90V[a!\x9Ea!\xA3\x91a\x17\0V[a!\x8CV[\x90V[a!\xB0\x90Ta!\x92V[\x90V[a!\xDA\x91_a!\xCFa!\xD5\x93a!\xC7a\x15'V[P`\x05a\x17{V[\x01a!vV[a!\xA6V[\x90V[_\x90V[a!\xE9a!\xDDV[Pa!\xF2a-dV[\x90V[a!\xFDa\x15kV[Pa\"\x08`\x04a\x16\xB8V[\x90V[a\"2a\"-a\"(a\"7\x93a\" a\x16\xFCV[P`\na\x18\xD6V[a\x18\xECV[a/\tV[a\x18\xEFV[\x90V[a\"W\x91a\"Fa\x15'V[Pa\"Oa&EV[\x91\x90\x91a'.V[`\x01\x90V[\x90a\"o\x91a\"ia\x16\xFCV[Pa\x19\x0BV[\x90V[a\"\x84\x90a\"~a\x16\xFCV[Pa\"\x0BV[\x90V[a\"\x8Fa\x16\xFCV[Pa\"\x98a\x17&V[\x90V[\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF\x90V[a\"\xF4a\"\xFB\x94a\"\xEA``\x94\x98\x97\x95a\"\xE0`\x80\x86\x01\x9A_\x87\x01\x90a\x066V[` \x85\x01\x90a\t8V[`@\x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[` \x01\x90V[Q\x90V[\x93\x95\x94\x90\x92\x91\x95Ba#!a#\x1B\x89a\x04\xA2V[\x91a\x04\xA2V[\x11a#\x9AW\x91a#\x8C\x91a#\x93\x93a#\x83a#\x98\x98\x99a#ka#Ba\"\x9BV[a#\\\x8B\x93\x8Ba#Pa\x03\x02V[\x95\x86\x94` \x86\x01a\"\xBFV[` \x82\x01\x81\x03\x82R\x03\x82a\x16mV[a#}a#w\x82a#\x03V[\x91a\"\xFDV[ a/~V[\x92\x90\x91\x92a/\x9BV[\x91\x82a/\xE5V[a-\xB7V[V[a#\xB5\x87_\x91\x82\x91c#A\xD7\x87`\xE1\x1B\x83R`\x04\x83\x01a\x058V[\x03\x90\xFD[\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x90V[\x91\x94a$%a$/\x92\x98\x97\x95a$\x1B`\xA0\x96a$\x11a$6\x9Aa$\x07`\xC0\x8A\x01\x9E_\x8B\x01\x90a\x066V[` \x89\x01\x90a\t8V[`@\x87\x01\x90a\t8V[``\x85\x01\x90a\x05+V[`\x80\x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[\x91` a$Y\x92\x94\x93a$R`@\x82\x01\x96_\x83\x01\x90a\t8V[\x01\x90a\t8V[V[\x96\x95\x91\x93\x92\x94\x90\x94Ba$va$p\x83a\x04\xA2V[\x91a\x04\xA2V[\x11a%0W\x90a$\xDFa$\xE8\x94\x93\x92a$\xC7a$\x90a#\xB9V[a$\xB8\x8C\x80\x94\x8C\x91a$\xA2\x8D\x91a07V[\x91\x92a$\xACa\x03\x02V[\x97\x88\x96` \x88\x01a#\xDDV[` \x82\x01\x81\x03\x82R\x03\x82a\x16mV[a$\xD9a$\xD3\x82a#\x03V[\x91a\"\xFDV[ a/~V[\x92\x90\x91\x92a/\x9BV[\x80a$\xFBa$\xF5\x87a\x04sV[\x91a\x04sV[\x03a%\x10WPa%\x0E\x92\x93\x91\x90\x91a&RV[V[\x84\x90a%,_\x92\x83\x92c%\xC0\x07#`\xE1\x1B\x84R`\x04\x84\x01a$8V[\x03\x90\xFD[a%K\x90_\x91\x82\x91c1<\x89\x81`\xE1\x1B\x83R`\x04\x83\x01a\x058V[\x03\x90\xFD[\x90a%j\x91a%ea%`\x82a\x17\xB5V[a'\xCBV[a%lV[V[\x90a%v\x91a)\x8AV[PV[\x90a%\x83\x91a%OV[V[\x90a%\x8F\x90a\x18\xCAV[_R` R`@_ \x90V[a%\xC0\x91a%\xB6a%\xBB\x92a%\xAEa\x16\xFCV[P`\x01a%\x85V[a\x1C_V[a\x17\x19V[\x90V[a%\xCD`@a\x1A\xDEV[\x90V[_\x90V[_\x90V[a%\xE0a%\xC3V[\x90` \x80\x83a%\xEDa%\xD0V[\x81R\x01a%\xF8a%\xD4V[\x81RPPV[a&\x06a%\xD8V[\x90V[\x90a&\x1C\x91a&\x16a%\xFEV[Pa0jV[\x90V[a&'a\x15'V[Pa&Aa&;c\x01\xFF\xC9\xA7`\xE0\x1Ba\x03\x10V[\x91a\x03\x10V[\x14\x90V[a&Ma\x1B\xCEV[P3\x90V[\x91a&`\x92\x91`\x01\x92a0\x92V[V[`@\x90a&\x8Ba&\x92\x94\x96\x95\x93\x96a&\x81``\x84\x01\x98_\x85\x01\x90a\t8V[` \x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[\x90a&\x9F\x91\x03a\x04\xA2V[\x90V[\x92\x91\x92a&\xB0\x81\x83\x90a%\x9BV[\x90\x81a&\xC5a&\xBF_\x19a\x04\xA2V[\x91a\x04\xA2V[\x10a&\xD2W[PPP\x90PV[\x81a&\xE5a&\xDF\x87a\x04\xA2V[\x91a\x04\xA2V[\x10a'\x0BWa'\x02\x93\x94a&\xFA\x91\x93\x92a&\x94V[\x90_\x92a0\x92V[\x80_\x80\x80a&\xCBV[Pa'*\x84\x92\x91\x92_\x93\x84\x93c}\xC7\xA0\xD9`\xE1\x1B\x85R`\x04\x85\x01a&bV[\x03\x90\xFD[\x91\x82a'Ja'Da'?_a\x19}V[a\x04sV[\x91a\x04sV[\x14a'\xA4W\x81a'ja'da'__a\x19}V[a\x04sV[\x91a\x04sV[\x14a'}Wa'{\x92\x91\x90\x91a1\xA1V[V[a'\xA0a'\x89_a\x19}V[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\tEV[\x03\x90\xFD[a'\xC7a'\xB0_a\x19}V[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\tEV[\x03\x90\xFD[a'\xDD\x90a'\xD7a&EV[\x90a2nV[V[\x90a'\xEB`\xFF\x91a\x0F\xF4V[\x91\x81\x19\x16\x91\x16\x17\x90V[a'\xFE\x90a\x03]V[\x90V[\x90V[\x90a(\x19a(\x14a( \x92a'\xF5V[a(\x01V[\x82Ta'\xDFV[\x90UV[a(,a\x15'V[Pa(Aa(;\x82\x84\x90a!\xB3V[\x15a\x03]V[_\x14a(\xCAWa(i`\x01a(d_a(\\`\x05\x86\x90a\x17{V[\x01\x85\x90a!vV[a(\x04V[\x90a(ra&EV[\x90a(\xAFa(\xA9a(\xA3\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x95a\x17oV[\x92a\x18\xCAV[\x92a\x18\xCAV[\x92a(\xB8a\x03\x02V[\x80a(\xC2\x81a\x06\xBAV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a(\xD8a\x17kV[Pa(\xE20a GV[a)\x14a)\x0E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04sV[\x91a\x04sV[\x14\x80a)PW[_\x14a)EW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a)Ma3\x1AV[\x90V[PFa)\x84a)~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xA2V[\x91a\x04\xA2V[\x14a)\x1BV[a)\x92a\x15'V[Pa)\x9E\x81\x83\x90a!\xB3V[_\x14a*&Wa)\xC5_a)\xC0_a)\xB8`\x05\x86\x90a\x17{V[\x01\x85\x90a!vV[a(\x04V[\x90a)\xCEa&EV[\x90a*\x0Ba*\x05a)\xFF\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x95a\x17oV[\x92a\x18\xCAV[\x92a\x18\xCAV[\x92a*\x14a\x03\x02V[\x80a*\x1E\x81a\x06\xBAV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a*@a*;a*E\x92a\x0E\xAEV[a\x08\xACV[a\x04\xA2V[\x90V[\x91` a*i\x92\x94\x93a*b`@\x82\x01\x96_\x83\x01\x90a\x05+V[\x01\x90a\x0E\xB9V[V[a*sa!\xDDV[Pa*|a!\xE1V[\x81a*\x8Fa*\x89\x83a*,V[\x91a\x04\xA2V[\x10\x15a*\xA2WPa*\x9F\x90a4#V[\x90V[\x90a*\xBD_\x92\x83\x92cvi\xFC\x0F`\xE1\x1B\x84R`\x04\x84\x01a*HV[\x03\x90\xFD[T\x90V[\x90V[a*\xDCa*\xD7a*\xE1\x92a*\xC5V[a\x08\xACV[a\x04\xA2V[\x90V[\x90V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a*\xFEa+\x03\x91a\x17\0V[a*\xE7V[\x90V[a+\x10\x90Ta*\xF2V[\x90V[\x90V[a+*a+%a+/\x92a+\x13V[a\x08\xACV[a\x04\xA2V[\x90V[`0\x1C\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a+Oa+T\x91a+2V[a+8V[\x90V[a+a\x90Ta+CV[\x90V[a+xa+sa+}\x92a\x0F\xF1V[a\x08\xACV[a\x14\x07V[\x90V[\x90a+\xD4\x90a+\x8Da\x14\x92V[Pa+\x99_\x84\x01a*\xC1V[a+\xA2_a\x19\x89V[\x90\x80\x80a+\xB8a+\xB2`\x05a*\xC8V[\x91a\x04\xA2V[\x11a,5W[P\x90a+\xCF_\x86\x01\x93\x91\x92\x93a*\xE4V[a:vV[\x80a+\xE7a+\xE1_a\x19\x89V[\x91a\x04\xA2V[\x14_\x14a+\xFDWPPa+\xF9_a+dV[[\x90V[a,*_\x91a,%a,\x1F\x84a,0\x96\x01\x92a,\x19`\x01a+\x16V[\x90a\x1D\xB6V[\x91a*\xE4V[a:lV[\x01a+WV[a+\xFAV[\x80a,Ca,I\x92\x91a7\x01V[\x90a\x1D\xB6V[\x90\x83a,{a,ua,p_a,j\x81\x8C\x01a,e\x89\x91a*\xE4V[a:lV[\x01a+\x06V[a\x0E\xAEV[\x91a\x0E\xAEV[\x10_\x14a,\x8CWP\x90[\x90_a+\xBEV[\x91Pa,\xA2\x90a,\x9C`\x01a+\x16V[\x90a\x19\xA5V[a,\x85V[\x80a,\xC2a,\xBCa,\xB7_a\x19}V[a\x04sV[\x91a\x04sV[\x14a,\xDEWa,\xDC\x91a,\xD4_a\x19}V[\x91\x90\x91a1\xA1V[V[a-\x01a,\xEA_a\x19}V[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\tEV[\x03\x90\xFD[\x90\x81a-!a-\x1Ba-\x16_a\x19}V[a\x04sV[\x91a\x04sV[\x14a-=Wa-;\x91\x90a-4_a\x19}V[\x90\x91a1\xA1V[V[a-`a-I_a\x19}V[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\tEV[\x03\x90\xFD[a-la!\xDDV[Pa-vCa4#V[\x90V[\x90a-\x8A`\x01\x80`\xA0\x1B\x03\x91a\x0F\xF4V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a-\xACa-\xA7a-\xB3\x92a\x18\xCAV[a-\x94V[\x82Ta-yV[\x90UV[\x90a.@\x91a.:a-\xC8\x82a\x1C\x14V[a-\xDD\x84a-\xD8`\t\x86\x90a\x1B\xD2V[a-\x97V[\x82\x81\x85\x90a.\x1Da.\x17a.\x11\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x95a\x18\xCAV[\x92a\x18\xCAV[\x92a\x18\xCAV[\x92a.&a\x03\x02V[\x80a.0\x81a\x06\xBAV[\x03\x90\xA4\x92\x91a;\x05V[\x91a;\x1DV[V[a.ia.da._a.n\x93a.Wa\x1CFV[P`\na\x18\xD6V[a\x18\xECV[a<\xCBV[a=JV[\x90V[a.\x83\x90a.}a\x16\xFCV[Pa=\x9BV[\x90V[\x90V[a.\x91a\x15kV[Pa.\xC6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a.\xC0`\x06a.\x86V[\x90a>\xB6V[\x90V[a.\xD1a\x15kV[Pa/\x06\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a/\0`\x07a.\x86V[\x90a>\xB6V[\x90V[a/\x11a\x14\x92V[Pa/\x1D_\x82\x01a*\xC1V[\x80a/0a/*_a\x19\x89V[\x91a\x04\xA2V[\x14_\x14a/FWPPa/B_a+dV[[\x90V[a/s_\x91a/na/h\x84a/y\x96\x01\x92a/b`\x01a+\x16V[\x90a\x1D\xB6V[\x91a*\xE4V[a:lV[\x01a+WV[a/CV[a/\x98\x90a/\x8Aa\x17kV[Pa/\x93a(\xD0V[a?\x04V[\x90V[\x92a/\xB6\x92a/\xBF\x94a/\xACa\x1B\xCEV[P\x92\x90\x91\x92a?\xCAV[\x90\x92\x91\x92a@\xF5V[\x90V[\x91` a/\xE3\x92\x94\x93a/\xDC`@\x82\x01\x96_\x83\x01\x90a\t8V[\x01\x90a\x05+V[V[a/\xEE\x81a07V[\x91a0\x01a/\xFB\x84a\x04\xA2V[\x91a\x04\xA2V[\x03a0\nWPPV[a0$_\x92\x83\x92c\x01\xD4\xB6#`\xE6\x1B\x84R`\x04\x84\x01a/\xC2V[\x03\x90\xFD[`\x01a04\x91\x01a\x04\xA2V[\x90V[a0K\x90a0Ca\x16\xFCV[P`\x08a\x1C_V[a0ga0W\x82a\x17\x19V[\x91a0a\x83a0(V[\x90a\x1F\x0BV[\x90V[\x90a0\x8Aa0\x85a0\x8F\x93a0}a%\xFEV[P`\na\x18\xD6V[a\x18\xECV[aBkV[\x90V[\x90\x92\x81a0\xAFa0\xA9a0\xA4_a\x19}V[a\x04sV[\x91a\x04sV[\x14a1zW\x83a0\xCFa0\xC9a0\xC4_a\x19}V[a\x04sV[\x91a\x04sV[\x14a1SWa0\xF3\x83a0\xEEa0\xE7`\x01\x86\x90a%\x85V[\x87\x90a\x1C_V[a\x1F\x0BV[a0\xFDW[PPPV[\x91\x90\x91a1Ha16a10\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x93a\x18\xCAV[\x93a\x18\xCAV[\x93a1?a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xA3_\x80\x80a0\xF8V[a1va1__a\x19}V[_\x91\x82\x91cJ\x14\x06\xB1`\xE1\x1B\x83R`\x04\x83\x01a\tEV[\x03\x90\xFD[a1\x9Da1\x86_a\x19}V[_\x91\x82\x91c\xE6\x02\xDF\x05`\xE0\x1B\x83R`\x04\x83\x01a\tEV[\x03\x90\xFD[\x91\x82a1\xBDa1\xB7a1\xB2_a\x19}V[a\x04sV[\x91a\x04sV[\x14\x15\x80a2(W[a1\xD8W[a1\xD6\x92\x91\x90\x91aB\x8CV[V[a1\xE0a\x1EhV[\x80a2\x07W[\x15a1\xCAW_c6\xE2x\xFD`\xE2\x1B\x81R\x80a2\x03`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[Pa2#a2\x1Da2\x16a\r\x04V[3\x90a!\xB3V[\x15a\x03]V[a1\xE6V[P\x81a2Da2>a29_a\x19}V[a\x04sV[\x91a\x04sV[\x14\x15a1\xC5V[\x91` a2l\x92\x94\x93a2e`@\x82\x01\x96_\x83\x01\x90a\t8V[\x01\x90a\x066V[V[\x90a2\x83a2}\x83\x83\x90a!\xB3V[\x15a\x03]V[a2\x8BWPPV[a2\xA5_\x92\x83\x92c\xE2Q}?`\xE0\x1B\x84R`\x04\x84\x01a2KV[\x03\x90\xFD[\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x90V[\x90\x95\x94\x92a3\x18\x94a3\x07a3\x11\x92a2\xFD`\x80\x96a2\xF3`\xA0\x88\x01\x9C_\x89\x01\x90a\x066V[` \x87\x01\x90a\x066V[`@\x85\x01\x90a\x066V[``\x83\x01\x90a\x05+V[\x01\x90a\t8V[V[a3\"a\x17kV[Pa3+a2\xA9V[a3\xA2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a3\x93\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Fa3~0a GV[\x91a3\x87a\x03\x02V[\x96\x87\x95` \x87\x01a2\xCDV[` \x82\x01\x81\x03\x82R\x03\x82a\x16mV[a3\xB4a3\xAE\x82a#\x03V[\x91a\"\xFDV[ \x90V[\x90V[a3\xCFa3\xCAa3\xD4\x92a3\xB8V[a\x08\xACV[a\x06\xF3V[\x90V[a3\xE0\x90a3\xBBV[\x90RV[\x91` a4\x05\x92\x94\x93a3\xFE`@\x82\x01\x96_\x83\x01\x90a3\xD7V[\x01\x90a\x05+V[V[a4\x1Ba4\x16a4 \x92a\x04\xA2V[a\x08\xACV[a\x0E\xAEV[\x90V[a4+a!\xDDV[P\x80a4Ea4?e\xFF\xFF\xFF\xFF\xFF\xFFa*,V[\x91a\x04\xA2V[\x11a4VWa4S\x90a4\x07V[\x90V[`0a4r_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a3\xE4V[\x03\x90\xFD[\x90V[a4\x8Da4\x88a4\x92\x92a4vV[a\x08\xACV[a\x04\xA2V[\x90V[\x90V[a4\xACa4\xA7a4\xB1\x92a4\x95V[a\x08\xACV[a\x06\xF3V[\x90V[a4\xD3\x90a4\xCDa4\xC7a4\xD8\x94a\x06\xF3V[\x91a\x04\xA2V[\x90a\x10\x97V[a\x04\xA2V[\x90V[\x90V[a4\xF2a4\xEDa4\xF7\x92a4\xDBV[a\x08\xACV[a\x06\xF3V[\x90V[\x1B\x90V[a5\x1D\x90a5\x17a5\x11a5\"\x94a\x06\xF3V[\x91a\x04\xA2V[\x90a4\xFAV[a\x04\xA2V[\x90V[\x90V[a5<a57a5A\x92a5%V[a\x08\xACV[a\x04\xA2V[\x90V[\x90V[a5[a5Va5`\x92a5DV[a\x08\xACV[a\x06\xF3V[\x90V[\x90V[a5za5ua5\x7F\x92a5cV[a\x08\xACV[a\x04\xA2V[\x90V[\x90V[a5\x99a5\x94a5\x9E\x92a5\x82V[a\x08\xACV[a\x06\xF3V[\x90V[\x90V[a5\xB8a5\xB3a5\xBD\x92a5\xA1V[a\x08\xACV[a\x04\xA2V[\x90V[\x90V[a5\xD7a5\xD2a5\xDC\x92a5\xC0V[a\x08\xACV[a\x06\xF3V[\x90V[\x90V[a5\xF6a5\xF1a5\xFB\x92a5\xDFV[a\x08\xACV[a\x04\xA2V[\x90V[\x90V[a6\x15a6\x10a6\x1A\x92a5\xFEV[a\x08\xACV[a\x06\xF3V[\x90V[a61a6,a66\x92a5\x82V[a\x08\xACV[a\x04\xA2V[\x90V[\x90V[a6Pa6Ka6U\x92a69V[a\x08\xACV[a\x06\xF3V[\x90V[a6la6ga6q\x92a5\xFEV[a\x08\xACV[a\x04\xA2V[\x90V[a6\x88a6\x83a6\x8D\x92a+\x13V[a\x08\xACV[a\x06\xF3V[\x90V[\x90V[a6\xA7a6\xA2a6\xAC\x92a6\x90V[a\x08\xACV[a\x04\xA2V[\x90V[\x90a6\xBA\x91\x02a\x04\xA2V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a6\xDDa6\xE3\x91a\x04\xA2V[\x91a\x04\xA2V[\x90\x81\x15a6\xEEW\x04\x90V[a6\xBDV[\x90a6\xFE\x91\x01a\x04\xA2V[\x90V[a7\ta\x16\xFCV[P\x80a7\x1Ea7\x18`\x01a+\x16V[\x91a\x04\xA2V[\x11\x15a:iW\x80a93a9\x10a9\0a8\xF0a8\xE0a8\xD0a8\xC0a8\xB0a8\xA0a8\x90a8\x80\x8Ba8za8sa99\x9Fa8Sa8Ca8c\x92a7e`\x01a+\x16V[\x90\x80a7}a7w`\x01`\x80\x1Ba4yV[\x91a\x04\xA2V[\x10\x15a:;W[\x80a7\xA0a7\x9Ah\x01\0\0\0\0\0\0\0\0a5(V[\x91a\x04\xA2V[\x10\x15a:\rW[\x80a7\xBFa7\xB9d\x01\0\0\0\0a5fV[\x91a\x04\xA2V[\x10\x15a9\xDFW[\x80a7\xDCa7\xD6b\x01\0\0a5\xA4V[\x91a\x04\xA2V[\x10\x15a9\xB1W[\x80a7\xF8a7\xF2a\x01\0a5\xE2V[\x91a\x04\xA2V[\x10\x15a9\x83W[\x80a8\x13a8\r`\x10a6\x1DV[\x91a\x04\xA2V[\x10\x15a9UW[a8-a8'`\x04a6XV[\x91a\x04\xA2V[\x10\x15a9<W[a8>`\x03a6\x93V[a6\xAFV[a8M`\x01a6tV[\x90a4\xB4V[a8]\x81\x86a6\xD1V[\x90a6\xF3V[a8m`\x01a6tV[\x90a4\xB4V[\x80\x92a6\xD1V[\x90a6\xF3V[a8\x8A`\x01a6tV[\x90a4\xB4V[a8\x9A\x81\x8Ca6\xD1V[\x90a6\xF3V[a8\xAA`\x01a6tV[\x90a4\xB4V[a8\xBA\x81\x8Aa6\xD1V[\x90a6\xF3V[a8\xCA`\x01a6tV[\x90a4\xB4V[a8\xDA\x81\x88a6\xD1V[\x90a6\xF3V[a8\xEA`\x01a6tV[\x90a4\xB4V[a8\xFA\x81\x86a6\xD1V[\x90a6\xF3V[a9\n`\x01a6tV[\x90a4\xB4V[\x91a9-a9'a9\"\x85\x80\x94a6\xD1V[a\x04\xA2V[\x91a\x04\xA2V[\x11aC\x1CV[\x90a&\x94V[\x90V[a9P\x90a9J`\x01a6tV[\x90a4\xFEV[a84V[a9la9}\x91a9f`\x04a6\x01V[\x90a4\xB4V[\x91a9w`\x02a6<V[\x90a4\xFEV[\x90a8\x1AV[a9\x9Aa9\xAB\x91a9\x94`\x08a5\xC3V[\x90a4\xB4V[\x91a9\xA5`\x04a6\x01V[\x90a4\xFEV[\x90a7\xFFV[a9\xC8a9\xD9\x91a9\xC2`\x10a5\x85V[\x90a4\xB4V[\x91a9\xD3`\x08a5\xC3V[\x90a4\xFEV[\x90a7\xE3V[a9\xF6a:\x07\x91a9\xF0` a5GV[\x90a4\xB4V[\x91a:\x01`\x10a5\x85V[\x90a4\xFEV[\x90a7\xC6V[a:$a:5\x91a:\x1E`@a4\xDEV[\x90a4\xB4V[\x91a:/` a5GV[\x90a4\xFEV[\x90a7\xA7V[a:Ra:c\x91a:L`\x80a4\x98V[\x90a4\xB4V[\x91a:]`@a4\xDEV[\x90a4\xFEV[\x90a7\x84V[\x90V[_R` _ \x01\x90V[\x93\x91\x90\x92a:\x82a\x16\xFCV[P[\x81a:\x97a:\x91\x83a\x04\xA2V[\x91a\x04\xA2V[\x10\x15a:\xFDWa:\xA8\x82\x82\x90aChV[\x90a:\xBE_a:\xB8\x88\x85\x90a:lV[\x01a+\x06V[a:\xD0a:\xCA\x87a\x0E\xAEV[\x91a\x0E\xAEV[\x11_\x14a:\xE0WP\x91[\x91a:\x84V[\x92\x91Pa:\xF7\x90a:\xF1`\x01a+\x16V[\x90a\x19\xA5V[\x90a:\xDAV[\x92PP\x91P\x90V[a;\x17\x90a;\x11a\x16\xFCV[Pa\x1CuV[\x90V[\x90V[\x91\x90\x91\x80a;3a;-\x85a\x04sV[\x91a\x04sV[\x14\x15\x80a<\xB1W[a;EW[PPPV[\x80a;`a;Za;U_a\x19}V[a\x04sV[\x91a\x04sV[\x03a<!W[P\x81a;\x82a;|a;w_a\x19}V[a\x04sV[\x91a\x04sV[\x03a;\x8EW[\x80a;@V[a;\xD5a;\xC8a;\xCF\x92a;\xA4`\n\x86\x90a\x18\xD6V[\x90a;\xC2a;\xBCa;\xB6`\x01\x93aD\x01V[\x93a\x18\xECV[\x91a;\x1AV[\x90aDTV[\x92\x90a\x18\xEFV[\x91a\x18\xEFV[\x91\x90\x91a<\x02\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x18\xCAV[\x92a<\x17a<\x0Ea\x03\x02V[\x92\x83\x92\x83a\x1F+V[\x03\x90\xA2_\x80a;\x88V[a<`a<fa<Ya<6`\n\x85\x90a\x18\xD6V[`\x02a<Sa<Ma<G\x89aD\x01V[\x93a\x18\xECV[\x91a;\x1AV[\x90aDTV[\x92\x90a\x18\xEFV[\x91a\x18\xEFV[\x91\x90\x91a<\x93\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x18\xCAV[\x92a<\xA8a<\x9Fa\x03\x02V[\x92\x83\x92\x83a\x1F+V[\x03\x90\xA2_a;fV[P\x81a<\xC5a<\xBF_a\x19\x89V[\x91a\x04\xA2V[\x11a;;V[_a<\xDF\x91a<\xD8a\x16\xFCV[P\x01a*\xC1V[\x90V[a<\xF6a<\xF1a<\xFB\x92a\t\xC2V[a\x08\xACV[a\x04\xA2V[\x90V[a=\x07\x90a5GV[\x90RV[\x91` a=,\x92\x94\x93a=%`@\x82\x01\x96_\x83\x01\x90a<\xFEV[\x01\x90a\x05+V[V[a=Ba==a=G\x92a\x04\xA2V[a\x08\xACV[a\t\xC2V[\x90V[a=Ra\x1CFV[P\x80a=ja=dc\xFF\xFF\xFF\xFFa<\xE2V[\x91a\x04\xA2V[\x11a={Wa=x\x90a=.V[\x90V[` a=\x97_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a=\x0BV[\x03\x90\xFD[a=\xB2a=\xB7\x91a=\xAAa\x16\xFCV[P`\x08a\x1C_V[a\x17\x19V[\x90V[\x90V[a=\xD1a=\xCCa=\xD6\x92a=\xBAV[a\x0F\xF4V[a\x05\xF2V[\x90V[a=\xE3`\xFFa=\xBDV[\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a>\ta>\x02\x83a\x15\x84V[\x80\x94a\x15\xAEV[\x91`\x01\x81\x16\x90\x81_\x14a>`WP`\x01\x14a>$W[PPPV[a>1\x91\x92\x93\x94Pa=\xE6V[\x91_\x92[\x81\x84\x10a>HWPP\x01\x90_\x80\x80a>\x1FV[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a>5V[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a>\x1FV[\x90a>\x85\x91a=\xEFV[\x90V[\x90a>\xA8a>\xA1\x92a>\x98a\x03\x02V[\x93\x84\x80\x92a>{V[\x03\x83a\x16mV[V[a>\xB3\x90a>\x88V[\x90V[\x90a>\xBFa\x15kV[Pa>\xC9\x82a\x17oV[a>\xE2a>\xDCa>\xD7a=\xD9V[a\x05\xF2V[\x91a\x05\xF2V[\x14\x15_\x14a>\xF7WPa>\xF4\x90aD\xDEV[\x90V[a?\x01\x91Pa>\xAAV[\x90V[`B\x91a?\x0Fa\x17kV[P`@Q\x91\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[_\x90V[a?Ua?Z\x91a\x17\0V[a\x1E\xECV[\x90V[\x90V[a?ta?oa?y\x92a?]V[a\x08\xACV[a\x04\xA2V[\x90V[a?\xB1a?\xB8\x94a?\xA7``\x94\x98\x97\x95a?\x9D`\x80\x86\x01\x9A_\x87\x01\x90a\x066V[` \x85\x01\x90a\x06\xF9V[`@\x83\x01\x90a\x066V[\x01\x90a\x066V[V[a?\xC2a\x03\x02V[=_\x82>=\x90\xFD[\x93\x92\x93a?\xD5a\x1B\xCEV[Pa?\xDEa?EV[Pa?\xE7a\x17kV[Pa?\xF1\x85a?IV[a@#a@\x1D\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0a?`V[\x91a\x04\xA2V[\x11a@\xB0W\x90a@F` \x94\x95_\x94\x93\x92\x93a@=a\x03\x02V[\x94\x85\x94\x85a?|V[\x83\x80R\x03\x90`\x01Z\xFA\x15a@\xABWa@^_Qa\x0F\xF4V[\x80a@ya@sa@n_a\x19}V[a\x04sV[\x91a\x04sV[\x14a@\x8FW_\x91a@\x89_a\x0F\xF9V[\x91\x92\x91\x90V[Pa@\x99_a\x19}V[`\x01\x91a@\xA5_a\x0F\xF9V[\x91\x92\x91\x90V[a?\xBAV[PPPa@\xBC_a\x19}V[\x90`\x03\x92\x91\x92\x91\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x04\x11\x15a@\xE4WV[a@\xC6V[\x90a@\xF3\x82a@\xDAV[V[\x80aA\x08aA\x02_a@\xE9V[\x91a@\xE9V[\x14_\x14aA\x13WPPV[\x80aA'aA!`\x01a@\xE9V[\x91a@\xE9V[\x14_\x14aAJW_c\xF6E\xEE\xDF`\xE0\x1B\x81R\x80aAF`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x80aA^aAX`\x02a@\xE9V[\x91a@\xE9V[\x14_\x14aA\x8CWaA\x88aAq\x83a?IV[_\x91\x82\x91c\xFC\xE6\x98\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x058V[\x03\x90\xFD[aA\x9FaA\x99`\x03a@\xE9V[\x91a@\xE9V[\x14aA\xA7WPV[aA\xC2\x90_\x91\x82\x91c5\xE2\xF3\x83`\xE2\x1B\x83R`\x04\x83\x01a\x06CV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_R` _ \x90V[aA\xEC\x81a*\xC1V[\x82\x10\x15aB\x06WaA\xFE`\x01\x91aA\xDAV[\x91\x02\x01\x90_\x90V[aA\xC6V[\x90aB\x15\x90a\x0E\xAEV[\x90RV[\x90aB#\x90a\x14\x07V[\x90RV[\x90aB]aBT_aB7a%\xC3V[\x94aBNaBF\x83\x83\x01a+\x06V[\x83\x88\x01aB\x0BV[\x01a+WV[` \x84\x01aB\x19V[V[aBh\x90aB'V[\x90V[aB\x89\x91_aB\x83\x92aB|a%\xFEV[P\x01aA\xE3V[PaB_V[\x90V[\x92\x91aB\x9A\x84\x83\x83\x91aE\x0EV[\x83aB\xB5aB\xAFaB\xAA_a\x19}V[a\x04sV[\x91a\x04sV[\x14aB\xCAW[aB\xC8\x92\x93\x91\x90\x91aF\x98V[V[aB\xD2a\x17&V[\x93aB\xDBaF}V[\x94\x80aB\xEFaB\xE9\x88a\x04\xA2V[\x91a\x04\xA2V[\x11aB\xFCWP\x93PaB\xBBV[\x85\x90aC\x18_\x92\x83\x92c\x0EX\xAE\x93`\xE1\x1B\x84R`\x04\x84\x01a\x1F+V[\x03\x90\xFD[aC$a\x16\xFCV[P\x15\x15\x90V[aC>aC9aCC\x92a69V[a\x08\xACV[a\x04\xA2V[\x90V[aCRaCX\x91a\x04\xA2V[\x91a\x04\xA2V[\x90\x81\x15aCcW\x04\x90V[a6\xBDV[aC\x8DaC\x93\x92aCwa\x16\xFCV[P\x82\x81\x16\x92\x18aC\x87`\x02aC*V[\x90aCFV[\x90a\x19\xA5V[\x90V[\x90V[aC\xADaC\xA8aC\xB2\x92aC\x96V[a\x08\xACV[a\x06\xF3V[\x90V[aC\xBE\x90aC\x99V[\x90RV[\x91` aC\xE3\x92\x94\x93aC\xDC`@\x82\x01\x96_\x83\x01\x90aC\xB5V[\x01\x90a\x05+V[V[aC\xF9aC\xF4aC\xFE\x92a\x04\xA2V[a\x08\xACV[a\x14\x07V[\x90V[aD\ta\x14\x92V[P\x80aD#aD\x1D`\x01\x80`\xD0\x1B\x03a\x18\xEFV[\x91a\x04\xA2V[\x11aD4WaD1\x90aC\xE5V[\x90V[`\xD0aDP_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01aC\xC2V[\x03\x90\xFD[\x90aD\x8AaD\x90\x93\x92aDea\x14\x92V[PaDna\x14\x92V[P\x80\x93aD\x83aD|a!\xE1V[\x94\x92a/\tV[\x90\x91aK\x13V[\x91aGWV[\x91\x90\x91\x90V[aD\xAAaD\xA5aD\xAF\x92a5DV[a\x08\xACV[a\x04\xA2V[\x90V[6\x907V[\x90aD\xDCaD\xC4\x83a\x1B\x16V[\x92` \x80aD\xD2\x86\x93a\x1A\xF3V[\x92\x01\x91\x03\x90aD\xB2V[V[aD\xE6a\x15kV[PaD\xF0\x81aG\xC1V[\x90aE\x03aD\xFE` aD\x96V[aD\xB7V[\x91\x82R` \x82\x01R\x90V[\x91\x90\x91\x80aE,aE&aE!_a\x19}V[a\x04sV[\x91a\x04sV[\x14_\x14aF\rWaEPaEI\x83aED`\x02a\x17\x19V[a\x19\xA5V[`\x02a\x1F\x0BV[[\x82aElaEfaEa_a\x19}V[a\x04sV[\x91a\x04sV[\x14_\x14aE\xE1WaE\x90aE\x89\x83aE\x84`\x02a\x17\x19V[a&\x94V[`\x02a\x1F\x0BV[[\x91\x90\x91aE\xDCaE\xCAaE\xC4\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93a\x18\xCAV[\x93a\x18\xCAV[\x93aE\xD3a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xA3V[aF\x08\x82aF\x02aE\xF3_\x87\x90a\x1C_V[\x91aE\xFD\x83a\x17\x19V[a6\xF3V[\x90a\x1F\x0BV[aE\x91V[aF aF\x1B_\x83\x90a\x1C_V[a\x17\x19V[\x80aF3aF-\x85a\x04\xA2V[\x91a\x04\xA2V[\x10aF[WaFFaFV\x91\x84\x90a&\x94V[aFQ_\x84\x90a\x1C_V[a\x1F\x0BV[aEQV[\x90aFy\x90\x91\x92_\x93\x84\x93c9\x144\xE3`\xE2\x1B\x85R`\x04\x85\x01a&bV[\x03\x90\xFD[aF\x85a\x16\xFCV[PaF\x95`\x01\x80`\xD0\x1B\x03a\x18\xEFV[\x90V[\x91aF\xF0aF\xEAaF\xF7\x94\x80aF\xBEaF\xB8aF\xB3_a\x19}V[a\x04sV[\x91a\x04sV[\x14aG(W[\x84aF\xDFaF\xD9aF\xD4_a\x19}V[a\x04sV[\x91a\x04sV[\x14aF\xF9W[a\x1C\x14V[\x92a\x1C\x14V[\x90\x91a;\x1DV[V[aG!`\x0B`\x02aG\x1BaG\x15aG\x0F\x89aD\x01V[\x93a\x18\xECV[\x91a;\x1AV[\x90aDTV[PPaF\xE5V[aGP`\x0B`\x01aGJaGDaG>\x89aD\x01V[\x93a\x18\xECV[\x91a;\x1AV[\x90aDTV[PPaF\xC4V[\x91aG{_aG\x80\x94aGha\x14\x92V[PaGqa\x14\x92V[P\x01\x92\x91\x92a*\xE4V[aI\xC5V[\x91\x90\x91\x90V[aG\x9AaG\x95aG\x9F\x92a=\xBAV[a\x08\xACV[a\x04\xA2V[\x90V[\x90V[aG\xB9aG\xB4aG\xBE\x92aG\xA2V[a\x08\xACV[a\x04\xA2V[\x90V[aG\xD6aG\xDB\x91aG\xD0a\x16\xFCV[Pa\x17oV[a?IV[aG\xE5`\xFFaG\x86V[\x16\x80aG\xFAaG\xF4`\x1FaG\xA5V[\x91a\x04\xA2V[\x11aH\x02W\x90V[_c,\xD4J\xC3`\xE2\x1B\x81R\x80aH\x1A`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[T\x90V[aH,`@a\x1A\xDEV[\x90V[_R` _ \x90V[aHA\x81aH\x1EV[\x82\x10\x15aH[WaHS`\x01\x91aH/V[\x91\x02\x01\x90_\x90V[aA\xC6V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[aH}\x90Qa\x0E\xAEV[\x90V[\x90aH\x91e\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x0F\xF4V[\x91\x81\x19\x16\x91\x16\x17\x90V[aH\xAFaH\xAAaH\xB4\x92a\x0E\xAEV[a\x08\xACV[a\x0E\xAEV[\x90V[\x90V[\x90aH\xCFaH\xCAaH\xD6\x92aH\x9BV[aH\xB7V[\x82TaH\x80V[\x90UV[aH\xE4\x90Qa\x14\x07V[\x90V[`0\x1B\x90V[\x90aH\xFFe\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91aH\xE7V[\x91\x81\x19\x16\x91\x16\x17\x90V[aI\x1DaI\x18aI\"\x92a\x14\x07V[a\x08\xACV[a\x14\x07V[\x90V[\x90V[\x90aI=aI8aID\x92aI\tV[aI%V[\x82TaH\xEDV[\x90UV[\x90aIr` _aIx\x94aIj\x82\x82\x01aId\x84\x88\x01aHsV[\x90aH\xBAV[\x01\x92\x01aH\xDAV[\x90aI(V[V[\x91\x90aI\x8BWaI\x89\x91aIHV[V[aH`V[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15aI\xC0W\x82aI\xB8\x91`\x01aI\xBE\x95\x01\x81UaH8V[\x90aIzV[V[a\x16YV[\x90\x92\x91\x92aI\xD1a\x14\x92V[PaI\xDAa\x14\x92V[PaI\xE4\x82aH\x1EV[\x80aI\xF7aI\xF1_a\x19\x89V[\x91a\x04\xA2V[\x11_\x14aJ\xC7WaJ\x1D\x90aJ\x17\x84\x91aJ\x11`\x01a+\x16V[\x90a\x1D\xB6V[\x90a:lV[\x90aJ)_\x83\x01a+\x06V[\x92aJ5_\x84\x01a+WV[\x93\x80aJIaJC\x85a\x0E\xAEV[\x91a\x0E\xAEV[\x11aJ\xABWaJ`aJZ\x84a\x0E\xAEV[\x91a\x0E\xAEV[\x14_\x14aJ{WPPaJv\x90_\x85\x91\x01aI(V[[\x91\x90V[aJ\xA6\x92PaJ\xA1\x86aJ\x98aJ\x8FaH\"V[\x94_\x86\x01aB\x0BV[` \x84\x01aB\x19V[aI\x90V[aJwV[_c% `\x1D`\xE0\x1B\x81R\x80aJ\xC3`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[PaJ\xF2\x91aJ\xED\x85aJ\xE4aJ\xDBaH\"V[\x94_\x86\x01aB\x0BV[` \x84\x01aB\x19V[aI\x90V[aJ\xFB_a+dV[\x91\x90V[cNH{q`\xE0\x1B_R`Q`\x04R`$_\xFD[\x91\x90\x91\x80`\x01\x14aK2W`\x02\x03aJ\xFFWaK.\x91a\x15\x11V[\x90[V[PaK<\x91a\x14\xD2V[\x90aK0V",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b61148e565b61001d5f356102fc565b806301ffc9a7146102f757806306fdde03146102f2578063095ea7b3146102ed57806318160ddd146102e857806323b872dd146102e3578063248a9ca3146102de5780632f2ff15d146102d9578063313ce567146102d45780633644e515146102cf57806336568abe146102ca5780633a46b1a8146102c557806340c10f19146102c057806342966c68146102bb5780634bf5d7e9146102b65780634f1bfc9e146102b1578063587cde1e146102ac5780635c19a95c146102a75780636fcfff45146102a257806370a082311461029d57806379cc6790146102985780637a8cd156146102935780637ecebe001461028e57806383f1211b146102895780638426adf214610284578063844c90261461027f57806384b0196e1461027a5780638a542521146102755780638d3343d6146102705780638e539e8c1461026b578063902d55a51461026657806391d148541461026157806391ddadf41461025c57806395d89b41146102575780639ab24eb0146102525780639b7ef64b1461024d578063a217fddf14610248578063a9059cbb14610243578063aa082a9d1461023e578063b0ca253e14610239578063bb4d443614610234578063c02ae7541461022f578063c3cda5201461022a578063d505accf14610225578063d547741f14610220578063dd62ed3e1461021b5763f1127ed80361000e57611458565b611374565b611313565b6112d9565b61122f565b611173565b61113e565b611108565b6110d3565b611061565b61102c565b610fbc565b610f45565b610f10565b610edb565b610e78565b610e43565b610dcc565b610d97565b610d33565b610cc8565b610b83565b610b4e565b610af5565b610ac0565b610a8b565b610a57565b610a22565b6109ed565b61098f565b61095a565b6108e5565b610874565b610841565b6107ef565b6107b9565b610785565b610750565b61071b565b6106bf565b610658565b6105bc565b61054d565b6104f5565b610433565b610384565b60e01c90565b60405190565b5f80fd5b5f80fd5b63ffffffff60e01b1690565b61032581610310565b0361032c57565b5f80fd5b9050359061033d8261031c565b565b9060208282031261035857610355915f01610330565b90565b61030c565b151590565b61036b9061035d565b9052565b9190610382905f60208501940190610362565b565b346103b4576103b061039f61039a36600461033f565b61152b565b6103a7610302565b9182918261036f565b0390f35b610308565b5f9103126103c357565b61030c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b61040961041260209361041793610400816103c8565b938480936103cc565b958691016103d5565b6103e0565b0190565b6104309160208201915f8184039101526103ea565b90565b34610463576104433660046103b9565b61045f61044e6116c4565b610456610302565b9182918261041b565b0390f35b610308565b60018060a01b031690565b61047c90610468565b90565b61048881610473565b0361048f57565b5f80fd5b905035906104a08261047f565b565b90565b6104ae816104a2565b036104b557565b5f80fd5b905035906104c6826104a5565b565b91906040838203126104f057806104e46104ed925f8601610493565b936020016104b9565b90565b61030c565b346105265761052261051161050b3660046104c8565b906116da565b610519610302565b9182918261036f565b0390f35b610308565b610534906104a2565b9052565b919061054b905f6020850194019061052b565b565b3461057d5761055d3660046103b9565b610579610568611726565b610570610302565b91829182610538565b0390f35b610308565b90916060828403126105b7576105b461059d845f8501610493565b936105ab8160208601610493565b936040016104b9565b90565b61030c565b346105ed576105e96105d86105d2366004610582565b9161173c565b6105e0610302565b9182918261036f565b0390f35b610308565b90565b6105fe816105f2565b0361060557565b5f80fd5b90503590610616826105f5565b565b906020828203126106315761062e915f01610609565b90565b61030c565b61063f906105f2565b9052565b9190610656905f60208501940190610636565b565b346106885761068461067361066e366004610618565b6117b5565b61067b610302565b91829182610643565b0390f35b610308565b91906040838203126106b557806106a96106b2925f8601610609565b93602001610493565b90565b61030c565b5f0190565b346106ee576106d86106d236600461068d565b90611801565b6106e0610302565b806106ea816106ba565b0390f35b610308565b60ff1690565b610702906106f3565b9052565b9190610719905f602085019401906106f9565b565b3461074b5761072b3660046103b9565b610747610736611830565b61073e610302565b91829182610706565b0390f35b610308565b34610780576107603660046103b9565b61077c61076b611846565b610773610302565b91829182610643565b0390f35b610308565b346107b45761079e61079836600461068d565b9061185a565b6107a6610302565b806107b0816106ba565b0390f35b610308565b346107ea576107e66107d56107cf3660046104c8565b9061190b565b6107dd610302565b91829182610538565b0390f35b610308565b3461081e576108086108023660046104c8565b90611a92565b610810610302565b8061081a816106ba565b0390f35b610308565b9060208282031261083c57610839915f016104b9565b90565b61030c565b3461086f57610859610854366004610823565b611a9e565b610861610302565b8061086b816106ba565b0390f35b610308565b346108a4576108843660046103b9565b6108a061088f611b78565b610897610302565b9182918261041b565b0390f35b610308565b90565b90565b6108c36108be6108c8926108a9565b6108ac565b6104a2565b90565b6108d7629e34006108af565b90565b6108e26108cb565b90565b34610915576108f53660046103b9565b6109116109006108da565b610908610302565b91829182610538565b0390f35b610308565b9060208282031261093357610930915f01610493565b90565b61030c565b61094190610473565b9052565b9190610958905f60208501940190610938565b565b3461098a5761098661097561097036600461091a565b611c14565b61097d610302565b91829182610945565b0390f35b610308565b346109bd576109a76109a236600461091a565b611c33565b6109af610302565b806109b9816106ba565b0390f35b610308565b63ffffffff1690565b6109d4906109c2565b9052565b91906109eb905f602085019401906109cb565b565b34610a1d57610a19610a08610a0336600461091a565b611c4a565b610a10610302565b918291826109d8565b0390f35b610308565b34610a5257610a4e610a3d610a3836600461091a565b611c75565b610a45610302565b91829182610538565b0390f35b610308565b34610a8657610a70610a6a3660046104c8565b90611daa565b610a78610302565b80610a82816106ba565b0390f35b610308565b34610abb57610a9b3660046103b9565b610ab7610aa6611ddb565b610aae610302565b91829182610538565b0390f35b610308565b34610af057610aec610adb610ad636600461091a565b611e53565b610ae3610302565b91829182610538565b0390f35b610308565b34610b2557610b053660046103b9565b610b21610b10611e68565b610b18610302565b9182918261036f565b0390f35b610308565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610b7e57610b5e3660046103b9565b610b7a610b69610b2a565b610b71610302565b91829182610538565b0390f35b610308565b34610bb157610b9b610b96366004610823565b612033565b610ba3610302565b80610bad816106ba565b0390f35b610308565b60ff60f81b1690565b610bc890610bb6565b9052565b5190565b60209181520190565b60200190565b610be8906104a2565b9052565b90610bf981602093610bdf565b0190565b60200190565b90610c20610c1a610c1384610bcc565b8093610bd0565b92610bd9565b905f5b818110610c305750505090565b909192610c49610c436001928651610bec565b94610bfd565b9101919091610c23565b93959194610ca4610c99610cb895610c8b610cae95610cc59c9a610c7e60e08c01925f8d0190610bbf565b8a820360208c01526103ea565b9088820360408a01526103ea565b97606087019061052b565b6080850190610938565b60a0830190610636565b60c0818403910152610c03565b90565b34610cff57610cd83660046103b9565b610cfb610ce36120bb565b93610cf2979597939193610302565b97889788610c53565b0390f35b610308565b7f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a67490565b610d30610d04565b90565b34610d6357610d433660046103b9565b610d5f610d4e610d28565b610d56610302565b91829182610643565b0390f35b610308565b7f9b12e0c5707e494915e58b0564f18aaad9b74ac69bfc815a1edadc8e4bd032eb90565b610d94610d68565b90565b34610dc757610da73660046103b9565b610dc3610db2610d8c565b610dba610302565b91829182610643565b0390f35b610308565b34610dfc57610df8610de7610de2366004610823565b612145565b610def610302565b91829182610538565b0390f35b610308565b90565b610e18610e13610e1d92610e01565b6108ac565b6104a2565b90565b610e356b033b2e3c9fd0803ce8000000610e04565b90565b610e40610e20565b90565b34610e7357610e533660046103b9565b610e6f610e5e610e38565b610e66610302565b91829182610538565b0390f35b610308565b34610ea957610ea5610e94610e8e36600461068d565b906121b3565b610e9c610302565b9182918261036f565b0390f35b610308565b65ffffffffffff1690565b610ec290610eae565b9052565b9190610ed9905f60208501940190610eb9565b565b34610f0b57610eeb3660046103b9565b610f07610ef66121e1565b610efe610302565b91829182610ec6565b0390f35b610308565b34610f4057610f203660046103b9565b610f3c610f2b6121f5565b610f33610302565b9182918261041b565b0390f35b610308565b34610f7557610f71610f60610f5b36600461091a565b61220b565b610f68610302565b91829182610538565b0390f35b610308565b90565b610f91610f8c610f9692610f7a565b6108ac565b6104a2565b90565b610fae6b02e87669c308736a04000000610f7d565b90565b610fb9610f99565b90565b34610fec57610fcc3660046103b9565b610fe8610fd7610fb1565b610fdf610302565b91829182610538565b0390f35b610308565b90565b5f1b90565b61100d61100861101292610ff1565b610ff4565b6105f2565b90565b61101e5f610ff9565b90565b611029611015565b90565b3461105c5761103c3660046103b9565b611058611047611021565b61104f610302565b91829182610643565b0390f35b610308565b346110925761108e61107d6110773660046104c8565b9061223a565b611085610302565b9182918261036f565b0390f35b610308565b1c90565b90565b6110ae9060086110b39302611097565b61109b565b90565b906110c1915461109e565b90565b6110d0600c5f906110b6565b90565b34611103576110e33660046103b9565b6110ff6110ee6110c4565b6110f6610302565b91829182610538565b0390f35b610308565b346111395761113561112461111e3660046104c8565b9061225c565b61112c610302565b91829182610538565b0390f35b610308565b3461116e5761116a61115961115436600461091a565b612272565b611161610302565b91829182610538565b0390f35b610308565b346111a3576111833660046103b9565b61119f61118e612287565b611196610302565b91829182610538565b0390f35b610308565b6111b1816106f3565b036111b857565b5f80fd5b905035906111c9826111a8565b565b909160c08284031261122a576111e3835f8401610493565b926111f181602085016104b9565b926111ff82604083016104b9565b9261122761121084606085016111bc565b9361121e8160808601610609565b9360a001610609565b90565b61030c565b346112645761124e6112423660046111cb565b94939093929192612307565b611256610302565b80611260816106ba565b0390f35b610308565b60e0818303126112d45761127f825f8301610493565b9261128d8360208401610493565b9261129b81604085016104b9565b926112a982606083016104b9565b926112d16112ba84608085016111bc565b936112c88160a08601610609565b9360c001610609565b90565b61030c565b3461130e576112f86112ec366004611269565b9594909493919361245b565b611300610302565b8061130a816106ba565b0390f35b610308565b346113425761132c61132636600461068d565b90612579565b611334610302565b8061133e816106ba565b0390f35b610308565b919060408382031261136f578061136361136c925f8601610493565b93602001610493565b90565b61030c565b346113a5576113a161139061138a366004611347565b9061259b565b611398610302565b91829182610538565b0390f35b610308565b6113b3816109c2565b036113ba57565b5f80fd5b905035906113cb826113aa565b565b91906040838203126113f557806113e96113f2925f8601610493565b936020016113be565b90565b61030c565b61140390610eae565b9052565b60018060d01b031690565b61141b90611407565b9052565b90602080611441936114375f8201515f8601906113fa565b0151910190611412565b565b9190611456905f6040850194019061141f565b565b346114895761148561147461146e3660046113cd565b90612609565b61147c610302565b91829182611443565b0390f35b610308565b5f80fd5b5f90565b634e487b7160e01b5f52601160045260245ffd5b6114b66114bc91611407565b91611407565b019060018060d01b0382116114cd57565b611496565b906114e5916114df611492565b506114aa565b90565b6114f46114fa91611407565b91611407565b90039060018060d01b03821161150c57565b611496565b906115249161151e611492565b506114e8565b90565b5f90565b611533611527565b508061154e611548637965db0b60e01b610310565b91610310565b1490811561155b575b5090565b611565915061261f565b5f611557565b606090565b634e487b7160e01b5f52602260045260245ffd5b90600160028304921680156115a4575b602083101461159f57565b611570565b91607f1691611594565b60209181520190565b5f5260205f2090565b905f92918054906115da6115d383611584565b80946115ae565b916001811690815f1461163157506001146115f5575b505050565b61160291929394506115b7565b915f925b81841061161957505001905f80806115f0565b60018160209295939554848601520191019290611606565b92949550505060ff19168252151560200201905f80806115f0565b90611656916115c0565b90565b634e487b7160e01b5f52604160045260245ffd5b90611677906103e0565b810190811067ffffffffffffffff82111761169157604052565b611659565b906116b66116af926116a6610302565b9384809261164c565b038361166d565b565b6116c190611696565b90565b6116cc61156b565b506116d760036116b8565b90565b6116f7916116e6611527565b506116ef612645565b919091612652565b600190565b5f90565b5f1c90565b61171161171691611700565b61109b565b90565b6117239054611705565b90565b61172e6116fc565b506117396002611719565b90565b9161176692611749611527565b5061175e611755612645565b829084916126a2565b91909161272e565b600190565b5f90565b611778906105f2565b90565b906117859061176f565b5f5260205260405f2090565b90565b6117a06117a591611700565b611791565b90565b6117b29054611794565b90565b60016117ce6117d4926117c661176b565b50600561177b565b016117a8565b90565b906117f2916117ed6117e8826117b5565b6127cb565b6117f4565b565b906117fe91612824565b50565b9061180b916117d7565b565b5f90565b90565b61182861182361182d92611811565b6108ac565b6106f3565b90565b61183861180d565b506118436012611814565b90565b61184e61176b565b506118576128d0565b90565b908061187561186f61186a612645565b610473565b91610473565b03611886576118839161298a565b50565b5f63334bd91960e11b81528061189e600482016106ba565b0390fd5b6118b66118b16118bb92610468565b6108ac565b610468565b90565b6118c7906118a2565b90565b6118d3906118be565b90565b906118e0906118ca565b5f5260205260405f2090565b90565b6119036118fe61190892611407565b6108ac565b6104a2565b90565b6119429161193761193161192c61193d946119246116fc565b50600a6118d6565b6118ec565b91612a6b565b90612b80565b6118ef565b90565b9061195f9161195a611955610d68565b6127cb565b6119ca565b565b61197561197061197a92610ff1565b6108ac565b610468565b90565b61198690611961565b90565b61199d6119986119a292610ff1565b6108ac565b6104a2565b90565b6119b46119ba919392936104a2565b926104a2565b82018092116119c557565b611496565b90816119e66119e06119db5f61197d565b610473565b91610473565b14611a7657806119fe6119f85f611989565b916104a2565b14611a5a57611a15611a0e611726565b82906119a5565b611a2e611a28611a23610e20565b6104a2565b916104a2565b11611a3e57611a3c91612ca7565b565b5f63177e3fc360e01b815280611a56600482016106ba565b0390fd5b5f631f2a200560e01b815280611a72600482016106ba565b0390fd5b5f63d92e233d60e01b815280611a8e600482016106ba565b0390fd5b90611a9c91611945565b565b80611ab1611aab5f611989565b916104a2565b14611ac257611ac09033612d05565b565b5f631f2a200560e01b815280611ada600482016106ba565b0390fd5b90611af1611aea610302565b928361166d565b565b67ffffffffffffffff8111611b1157611b0d6020916103e0565b0190565b611659565b90611b28611b2383611af3565b611ade565b918252565b5f7f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c74000000910152565b611b5e601d611b16565b90611b6b60208301611b2d565b565b611b75611b54565b90565b611b8061156b565b50611b896121e1565b611ba2611b9c611b97612d64565b610eae565b91610eae565b03611bb257611baf611b6d565b90565b5f6301bfc1c560e61b815280611bca600482016106ba565b0390fd5b5f90565b90611bdc906118ca565b5f5260205260405f2090565b60018060a01b031690565b611bff611c0491611700565b611be8565b90565b611c119054611bf3565b90565b611c2b611c3091611c23611bce565b506009611bd2565b611c07565b90565b611c4490611c3f612645565b612db7565b565b5f90565b611c5c90611c56611c46565b50612e42565b90565b90611c69906118ca565b5f5260205260405f2090565b611c8b611c9091611c846116fc565b505f611c5f565b611719565b90565b90611cad91611ca8611ca3610d04565b6127cb565b611caf565b565b80611cca611cc4611cbf5f61197d565b610473565b91610473565b14611d8e5781611ce2611cdc5f611989565b916104a2565b14611d7257611cf8611cf2611e68565b1561035d565b611d5657611d07818390612d05565b3390611d51611d3f611d397fbef4f81c1814c641ede85ebaacf19d048b2c5b55980adfa6ef0f956c651335a2936118ca565b936118ca565b93611d48610302565b91829182610538565b0390a3565b5f63b8b5ca2d60e01b815280611d6e600482016106ba565b0390fd5b5f631f2a200560e01b815280611d8a600482016106ba565b0390fd5b5f63d92e233d60e01b815280611da6600482016106ba565b0390fd5b90611db491611c93565b565b611dc5611dcb919392936104a2565b926104a2565b8203918211611dd657565b611496565b611de36116fc565b50611dee600c611719565b611e00611dfa5f611989565b916104a2565b148015611e2f575b611e2357611e20611e19600c611719565b4290611db6565b90565b611e2c5f611989565b90565b5042611e4c611e46611e41600c611719565b6104a2565b916104a2565b1015611e08565b611e6590611e5f6116fc565b50612e71565b90565b611e70611527565b50611e7b600c611719565b611e8d611e875f611989565b916104a2565b141580611e98575b90565b5042611eb5611eaf611eaa600c611719565b6104a2565b916104a2565b10611e95565b611ed490611ecf611eca611015565b6127cb565b611f4e565b565b90611ee25f1991610ff4565b9181191691161790565b611f00611efb611f05926104a2565b6108ac565b6104a2565b90565b90565b90611f20611f1b611f2792611eec565b611f08565b8254611ed6565b9055565b916020611f4c929493611f4560408201965f83019061052b565b019061052b565b565b80611f61611f5b426104a2565b916104a2565b11156120175780611f9a611f947f00000000000000000000000000000000000000000000000000000000000000006104a2565b916104a2565b11611ffb57611fa9600c611719565b611fb482600c611f0b565b903390611fe17fdd6896dcf1d4b311cca87dd19bbba2ea9ce2f867c1568878a0438a66a1afeeec926118ca565b92611ff6611fed610302565b92839283611f2b565b0390a2565b5f63ef69af6560e01b815280612013600482016106ba565b0390fd5b5f63a565835360e01b81528061202f600482016106ba565b0390fd5b61203c90611ebb565b565b5f90565b606090565b612050906118be565b90565b67ffffffffffffffff811161206b5760208091020190565b611659565b9061208261207d83612053565b611ade565b918252565b369037565b906120b161209983612070565b926020806120a78693612053565b9201910390612087565b565b600f60f81b90565b6120c361203e565b506120cc61156b565b506120d561156b565b506120de6116fc565b506120e7611bce565b506120f061176b565b506120f9612042565b50612102612e89565b9061210b612ec9565b90469061211730612047565b906121215f610ff9565b9061213361212e5f611989565b61208c565b9061213c6120b3565b96959493929190565b61216e612173916121546116fc565b50612168612162600b6118ec565b91612a6b565b90612b80565b6118ef565b90565b90612180906118ca565b5f5260205260405f2090565b60ff1690565b61219e6121a391611700565b61218c565b90565b6121b09054612192565b90565b6121da915f6121cf6121d5936121c7611527565b50600561177b565b01612176565b6121a6565b90565b5f90565b6121e96121dd565b506121f2612d64565b90565b6121fd61156b565b5061220860046116b8565b90565b61223261222d612228612237936122206116fc565b50600a6118d6565b6118ec565b612f09565b6118ef565b90565b61225791612246611527565b5061224f612645565b91909161272e565b600190565b9061226f916122696116fc565b5061190b565b90565b6122849061227e6116fc565b5061220b565b90565b61228f6116fc565b50612298611726565b90565b7fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf90565b6122f46122fb946122ea6060949897956122e0608086019a5f870190610636565b6020850190610938565b604083019061052b565b019061052b565b565b60200190565b5190565b939594909291954261232161231b896104a2565b916104a2565b1161239a579161238c9161239393612383612398989961236b61234261229b565b61235c8b938b612350610302565b958694602086016122bf565b6020820181038252038261166d565b61237d61237782612303565b916122fd565b20612f7e565b92909192612f9b565b9182612fe5565b612db7565b565b6123b5875f918291632341d78760e11b835260048301610538565b0390fd5b7f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c990565b919461242561242f9298979561241b60a0966124116124369a61240760c08a019e5f8b0190610636565b6020890190610938565b6040870190610938565b606085019061052b565b608083019061052b565b019061052b565b565b91602061245992949361245260408201965f830190610938565b0190610938565b565b969591939294909442612476612470836104a2565b916104a2565b1161253057906124df6124e89493926124c76124906123b9565b6124b88c80948c916124a28d91613037565b91926124ac610302565b978896602088016123dd565b6020820181038252038261166d565b6124d96124d382612303565b916122fd565b20612f7e565b92909192612f9b565b806124fb6124f587610473565b91610473565b03612510575061250e9293919091612652565b565b849061252c5f9283926325c0072360e11b845260048401612438565b0390fd5b61254b905f91829163313c898160e11b835260048301610538565b0390fd5b9061256a91612565612560826117b5565b6127cb565b61256c565b565b906125769161298a565b50565b906125839161254f565b565b9061258f906118ca565b5f5260205260405f2090565b6125c0916125b66125bb926125ae6116fc565b506001612585565b611c5f565b611719565b90565b6125cd6040611ade565b90565b5f90565b5f90565b6125e06125c3565b90602080836125ed6125d0565b8152016125f86125d4565b81525050565b6126066125d8565b90565b9061261c916126166125fe565b5061306a565b90565b612627611527565b5061264161263b6301ffc9a760e01b610310565b91610310565b1490565b61264d611bce565b503390565b916126609291600192613092565b565b60409061268b612692949695939661268160608401985f850190610938565b602083019061052b565b019061052b565b565b9061269f91036104a2565b90565b9291926126b081839061259b565b90816126c56126bf5f196104a2565b916104a2565b106126d2575b5050509050565b816126e56126df876104a2565b916104a2565b1061270b5761270293946126fa919392612694565b905f92613092565b805f80806126cb565b5061272a849291925f938493637dc7a0d960e11b855260048501612662565b0390fd5b918261274a61274461273f5f61197d565b610473565b91610473565b146127a4578161276a61276461275f5f61197d565b610473565b91610473565b1461277d5761277b929190916131a1565b565b6127a06127895f61197d565b5f91829163ec442f0560e01b835260048301610945565b0390fd5b6127c76127b05f61197d565b5f918291634b637e8f60e11b835260048301610945565b0390fd5b6127dd906127d7612645565b9061326e565b565b906127eb60ff91610ff4565b9181191691161790565b6127fe9061035d565b90565b90565b90612819612814612820926127f5565b612801565b82546127df565b9055565b61282c611527565b5061284161283b8284906121b3565b1561035d565b5f146128ca5761286960016128645f61285c6005869061177b565b018590612176565b612804565b90612872612645565b906128af6128a96128a37f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d9561176f565b926118ca565b926118ca565b926128b8610302565b806128c2816106ba565b0390a4600190565b50505f90565b6128d861176b565b506128e230612047565b61291461290e7f0000000000000000000000000000000000000000000000000000000000000000610473565b91610473565b1480612950575b5f14612945577f000000000000000000000000000000000000000000000000000000000000000090565b61294d61331a565b90565b504661298461297e7f00000000000000000000000000000000000000000000000000000000000000006104a2565b916104a2565b1461291b565b612992611527565b5061299e8183906121b3565b5f14612a26576129c55f6129c05f6129b86005869061177b565b018590612176565b612804565b906129ce612645565b90612a0b612a056129ff7ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b9561176f565b926118ca565b926118ca565b92612a14610302565b80612a1e816106ba565b0390a4600190565b50505f90565b612a40612a3b612a4592610eae565b6108ac565b6104a2565b90565b916020612a69929493612a6260408201965f83019061052b565b0190610eb9565b565b612a736121dd565b50612a7c6121e1565b81612a8f612a8983612a2c565b916104a2565b1015612aa25750612a9f90613423565b90565b90612abd5f928392637669fc0f60e11b845260048401612a48565b0390fd5b5490565b90565b612adc612ad7612ae192612ac5565b6108ac565b6104a2565b90565b90565b65ffffffffffff1690565b612afe612b0391611700565b612ae7565b90565b612b109054612af2565b90565b90565b612b2a612b25612b2f92612b13565b6108ac565b6104a2565b90565b60301c90565b60018060d01b031690565b612b4f612b5491612b32565b612b38565b90565b612b619054612b43565b90565b612b78612b73612b7d92610ff1565b6108ac565b611407565b90565b90612bd490612b8d611492565b50612b995f8401612ac1565b612ba25f611989565b908080612bb8612bb26005612ac8565b916104a2565b11612c35575b5090612bcf5f860193919293612ae4565b613a76565b80612be7612be15f611989565b916104a2565b145f14612bfd575050612bf95f612b64565b5b90565b612c2a5f91612c25612c1f84612c30960192612c196001612b16565b90611db6565b91612ae4565b613a6c565b01612b57565b612bfa565b80612c43612c499291613701565b90611db6565b9083612c7b612c75612c705f612c6a818c01612c658991612ae4565b613a6c565b01612b06565b610eae565b91610eae565b105f14612c8c5750905b905f612bbe565b9150612ca290612c9c6001612b16565b906119a5565b612c85565b80612cc2612cbc612cb75f61197d565b610473565b91610473565b14612cde57612cdc91612cd45f61197d565b9190916131a1565b565b612d01612cea5f61197d565b5f91829163ec442f0560e01b835260048301610945565b0390fd5b9081612d21612d1b612d165f61197d565b610473565b91610473565b14612d3d57612d3b9190612d345f61197d565b90916131a1565b565b612d60612d495f61197d565b5f918291634b637e8f60e11b835260048301610945565b0390fd5b612d6c6121dd565b50612d7643613423565b90565b90612d8a60018060a01b0391610ff4565b9181191691161790565b90565b90612dac612da7612db3926118ca565b612d94565b8254612d79565b9055565b90612e4091612e3a612dc882611c14565b612ddd84612dd860098690611bd2565b612d97565b82818590612e1d612e17612e117f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f956118ca565b926118ca565b926118ca565b92612e26610302565b80612e30816106ba565b0390a49291613b05565b91613b1d565b565b612e69612e64612e5f612e6e93612e57611c46565b50600a6118d6565b6118ec565b613ccb565b613d4a565b90565b612e8390612e7d6116fc565b50613d9b565b90565b90565b612e9161156b565b50612ec67f0000000000000000000000000000000000000000000000000000000000000000612ec06006612e86565b90613eb6565b90565b612ed161156b565b50612f067f0000000000000000000000000000000000000000000000000000000000000000612f006007612e86565b90613eb6565b90565b612f11611492565b50612f1d5f8201612ac1565b80612f30612f2a5f611989565b916104a2565b145f14612f46575050612f425f612b64565b5b90565b612f735f91612f6e612f6884612f79960192612f626001612b16565b90611db6565b91612ae4565b613a6c565b01612b57565b612f43565b612f9890612f8a61176b565b50612f936128d0565b613f04565b90565b92612fb692612fbf94612fac611bce565b5092909192613fca565b909291926140f5565b90565b916020612fe3929493612fdc60408201965f830190610938565b019061052b565b565b612fee81613037565b91613001612ffb846104a2565b916104a2565b0361300a575050565b6130245f9283926301d4b62360e61b845260048401612fc2565b0390fd5b600161303491016104a2565b90565b61304b906130436116fc565b506008611c5f565b61306761305782611719565b9161306183613028565b90611f0b565b90565b9061308a61308561308f9361307d6125fe565b50600a6118d6565b6118ec565b61426b565b90565b9092816130af6130a96130a45f61197d565b610473565b91610473565b1461317a57836130cf6130c96130c45f61197d565b610473565b91610473565b14613153576130f3836130ee6130e760018690612585565b8790611c5f565b611f0b565b6130fd575b505050565b9190916131486131366131307f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925936118ca565b936118ca565b9361313f610302565b91829182610538565b0390a35f80806130f8565b61317661315f5f61197d565b5f918291634a1406b160e11b835260048301610945565b0390fd5b61319d6131865f61197d565b5f91829163e602df0560e01b835260048301610945565b0390fd5b91826131bd6131b76131b25f61197d565b610473565b91610473565b141580613228575b6131d8575b6131d69291909161428c565b565b6131e0611e68565b80613207575b156131ca575f6336e278fd60e21b815280613203600482016106ba565b0390fd5b5061322361321d613216610d04565b33906121b3565b1561035d565b6131e6565b508161324461323e6132395f61197d565b610473565b91610473565b14156131c5565b91602061326c92949361326560408201965f830190610938565b0190610636565b565b9061328361327d8383906121b3565b1561035d565b61328b575050565b6132a55f92839263e2517d3f60e01b84526004840161324b565b0390fd5b7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f90565b9095949261331894613307613311926132fd6080966132f360a088019c5f890190610636565b6020870190610636565b6040850190610636565b606083019061052b565b0190610938565b565b61332261176b565b5061332b6132a9565b6133a27f0000000000000000000000000000000000000000000000000000000000000000916133937f00000000000000000000000000000000000000000000000000000000000000004661337e30612047565b91613387610302565b968795602087016132cd565b6020820181038252038261166d565b6133b46133ae82612303565b916122fd565b2090565b90565b6133cf6133ca6133d4926133b8565b6108ac565b6106f3565b90565b6133e0906133bb565b9052565b9160206134059294936133fe60408201965f8301906133d7565b019061052b565b565b61341b613416613420926104a2565b6108ac565b610eae565b90565b61342b6121dd565b508061344561343f65ffffffffffff612a2c565b916104a2565b116134565761345390613407565b90565b60306134725f9283926306dfcc6560e41b8452600484016133e4565b0390fd5b90565b61348d61348861349292613476565b6108ac565b6104a2565b90565b90565b6134ac6134a76134b192613495565b6108ac565b6106f3565b90565b6134d3906134cd6134c76134d8946106f3565b916104a2565b90611097565b6104a2565b90565b90565b6134f26134ed6134f7926134db565b6108ac565b6106f3565b90565b1b90565b61351d90613517613511613522946106f3565b916104a2565b906134fa565b6104a2565b90565b90565b61353c61353761354192613525565b6108ac565b6104a2565b90565b90565b61355b61355661356092613544565b6108ac565b6106f3565b90565b90565b61357a61357561357f92613563565b6108ac565b6104a2565b90565b90565b61359961359461359e92613582565b6108ac565b6106f3565b90565b90565b6135b86135b36135bd926135a1565b6108ac565b6104a2565b90565b90565b6135d76135d26135dc926135c0565b6108ac565b6106f3565b90565b90565b6135f66135f16135fb926135df565b6108ac565b6104a2565b90565b90565b61361561361061361a926135fe565b6108ac565b6106f3565b90565b61363161362c61363692613582565b6108ac565b6104a2565b90565b90565b61365061364b61365592613639565b6108ac565b6106f3565b90565b61366c613667613671926135fe565b6108ac565b6104a2565b90565b61368861368361368d92612b13565b6108ac565b6106f3565b90565b90565b6136a76136a26136ac92613690565b6108ac565b6104a2565b90565b906136ba91026104a2565b90565b634e487b7160e01b5f52601260045260245ffd5b6136dd6136e3916104a2565b916104a2565b9081156136ee570490565b6136bd565b906136fe91016104a2565b90565b6137096116fc565b508061371e6137186001612b16565b916104a2565b1115613a6957806139336139106139006138f06138e06138d06138c06138b06138a06138906138808b61387a6138736139399f613853613843613863926137656001612b16565b908061377d613777600160801b613479565b916104a2565b1015613a3b575b806137a061379a68010000000000000000613528565b916104a2565b1015613a0d575b806137bf6137b9640100000000613566565b916104a2565b10156139df575b806137dc6137d6620100006135a4565b916104a2565b10156139b1575b806137f86137f26101006135e2565b916104a2565b1015613983575b8061381361380d601061361d565b916104a2565b1015613955575b61382d6138276004613658565b916104a2565b101561393c575b61383e6003613693565b6136af565b61384d6001613674565b906134b4565b61385d81866136d1565b906136f3565b61386d6001613674565b906134b4565b80926136d1565b906136f3565b61388a6001613674565b906134b4565b61389a818c6136d1565b906136f3565b6138aa6001613674565b906134b4565b6138ba818a6136d1565b906136f3565b6138ca6001613674565b906134b4565b6138da81886136d1565b906136f3565b6138ea6001613674565b906134b4565b6138fa81866136d1565b906136f3565b61390a6001613674565b906134b4565b9161392d6139276139228580946136d1565b6104a2565b916104a2565b1161431c565b90612694565b90565b6139509061394a6001613674565b906134fe565b613834565b61396c61397d916139666004613601565b906134b4565b91613977600261363c565b906134fe565b9061381a565b61399a6139ab9161399460086135c3565b906134b4565b916139a56004613601565b906134fe565b906137ff565b6139c86139d9916139c26010613585565b906134b4565b916139d360086135c3565b906134fe565b906137e3565b6139f6613a07916139f06020613547565b906134b4565b91613a016010613585565b906134fe565b906137c6565b613a24613a3591613a1e60406134de565b906134b4565b91613a2f6020613547565b906134fe565b906137a7565b613a52613a6391613a4c6080613498565b906134b4565b91613a5d60406134de565b906134fe565b90613784565b90565b5f5260205f200190565b93919092613a826116fc565b505b81613a97613a91836104a2565b916104a2565b1015613afd57613aa8828290614368565b90613abe5f613ab8888590613a6c565b01612b06565b613ad0613aca87610eae565b91610eae565b115f14613ae05750915b91613a84565b929150613af790613af16001612b16565b906119a5565b90613ada565b925050915090565b613b1790613b116116fc565b50611c75565b90565b90565b91909180613b33613b2d85610473565b91610473565b141580613cb1575b613b45575b505050565b80613b60613b5a613b555f61197d565b610473565b91610473565b03613c21575b5081613b82613b7c613b775f61197d565b610473565b91610473565b03613b8e575b80613b40565b613bd5613bc8613bcf92613ba4600a86906118d6565b90613bc2613bbc613bb6600193614401565b936118ec565b91613b1a565b90614454565b92906118ef565b916118ef565b919091613c027fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926118ca565b92613c17613c0e610302565b92839283611f2b565b0390a25f80613b88565b613c60613c66613c59613c36600a85906118d6565b6002613c53613c4d613c4789614401565b936118ec565b91613b1a565b90614454565b92906118ef565b916118ef565b919091613c937fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926118ca565b92613ca8613c9f610302565b92839283611f2b565b0390a25f613b66565b5081613cc5613cbf5f611989565b916104a2565b11613b3b565b5f613cdf91613cd86116fc565b5001612ac1565b90565b613cf6613cf1613cfb926109c2565b6108ac565b6104a2565b90565b613d0790613547565b9052565b916020613d2c929493613d2560408201965f830190613cfe565b019061052b565b565b613d42613d3d613d47926104a2565b6108ac565b6109c2565b90565b613d52611c46565b5080613d6a613d6463ffffffff613ce2565b916104a2565b11613d7b57613d7890613d2e565b90565b6020613d975f9283926306dfcc6560e41b845260048401613d0b565b0390fd5b613db2613db791613daa6116fc565b506008611c5f565b611719565b90565b90565b613dd1613dcc613dd692613dba565b610ff4565b6105f2565b90565b613de360ff613dbd565b90565b5f5260205f2090565b905f9291805490613e09613e0283611584565b80946115ae565b916001811690815f14613e605750600114613e24575b505050565b613e319192939450613de6565b915f925b818410613e4857505001905f8080613e1f565b60018160209295939554848601520191019290613e35565b92949550505060ff19168252151560200201905f8080613e1f565b90613e8591613def565b90565b90613ea8613ea192613e98610302565b93848092613e7b565b038361166d565b565b613eb390613e88565b90565b90613ebf61156b565b50613ec98261176f565b613ee2613edc613ed7613dd9565b6105f2565b916105f2565b14155f14613ef75750613ef4906144de565b90565b613f019150613eaa565b90565b604291613f0f61176b565b50604051917f19010000000000000000000000000000000000000000000000000000000000008352600283015260228201522090565b5f90565b613f55613f5a91611700565b611eec565b90565b90565b613f74613f6f613f7992613f5d565b6108ac565b6104a2565b90565b613fb1613fb894613fa7606094989795613f9d608086019a5f870190610636565b60208501906106f9565b6040830190610636565b0190610636565b565b613fc2610302565b3d5f823e3d90fd5b939293613fd5611bce565b50613fde613f45565b50613fe761176b565b50613ff185613f49565b61402361401d7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0613f60565b916104a2565b116140b05790614046602094955f9493929361403d610302565b94859485613f7c565b838052039060015afa156140ab5761405e5f51610ff4565b8061407961407361406e5f61197d565b610473565b91610473565b1461408f575f916140895f610ff9565b91929190565b506140995f61197d565b6001916140a55f610ff9565b91929190565b613fba565b5050506140bc5f61197d565b9060039291929190565b634e487b7160e01b5f52602160045260245ffd5b600411156140e457565b6140c6565b906140f3826140da565b565b806141086141025f6140e9565b916140e9565b145f14614113575050565b8061412761412160016140e9565b916140e9565b145f1461414a575f63f645eedf60e01b815280614146600482016106ba565b0390fd5b8061415e61415860026140e9565b916140e9565b145f1461418c5761418861417183613f49565b5f91829163fce698f760e01b835260048301610538565b0390fd5b61419f61419960036140e9565b916140e9565b146141a75750565b6141c2905f9182916335e2f38360e21b835260048301610643565b0390fd5b634e487b7160e01b5f52603260045260245ffd5b5f5260205f2090565b6141ec81612ac1565b821015614206576141fe6001916141da565b910201905f90565b6141c6565b9061421590610eae565b9052565b9061422390611407565b9052565b9061425d6142545f6142376125c3565b9461424e614246838301612b06565b83880161420b565b01612b57565b60208401614219565b565b61426890614227565b90565b614289915f6142839261427c6125fe565b50016141e3565b5061425f565b90565b929161429a8483839161450e565b836142b56142af6142aa5f61197d565b610473565b91610473565b146142ca575b6142c89293919091614698565b565b6142d2611726565b936142db61467d565b94806142ef6142e9886104a2565b916104a2565b116142fc575093506142bb565b85906143185f928392630e58ae9360e11b845260048401611f2b565b0390fd5b6143246116fc565b50151590565b61433e61433961434392613639565b6108ac565b6104a2565b90565b614352614358916104a2565b916104a2565b908115614363570490565b6136bd565b61438d614393926143776116fc565b508281169218614387600261432a565b90614346565b906119a5565b90565b90565b6143ad6143a86143b292614396565b6108ac565b6106f3565b90565b6143be90614399565b9052565b9160206143e39294936143dc60408201965f8301906143b5565b019061052b565b565b6143f96143f46143fe926104a2565b6108ac565b611407565b90565b614409611492565b508061442361441d60018060d01b036118ef565b916104a2565b1161443457614431906143e5565b90565b60d06144505f9283926306dfcc6560e41b8452600484016143c2565b0390fd5b9061448a6144909392614465611492565b5061446e611492565b50809361448361447c6121e1565b9492612f09565b9091614b13565b91614757565b91909190565b6144aa6144a56144af92613544565b6108ac565b6104a2565b90565b369037565b906144dc6144c483611b16565b926020806144d28693611af3565b92019103906144b2565b565b6144e661156b565b506144f0816147c1565b906145036144fe6020614496565b6144b7565b918252602082015290565b9190918061452c6145266145215f61197d565b610473565b91610473565b145f1461460d57614550614549836145446002611719565b6119a5565b6002611f0b565b5b8261456c6145666145615f61197d565b610473565b91610473565b145f146145e157614590614589836145846002611719565b612694565b6002611f0b565b5b9190916145dc6145ca6145c47fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef936118ca565b936118ca565b936145d3610302565b91829182610538565b0390a3565b614608826146026145f35f8790611c5f565b916145fd83611719565b6136f3565b90611f0b565b614591565b61462061461b5f8390611c5f565b611719565b8061463361462d856104a2565b916104a2565b1061465b57614646614656918490612694565b6146515f8490611c5f565b611f0b565b614551565b906146799091925f93849363391434e360e21b855260048501612662565b0390fd5b6146856116fc565b5061469560018060d01b036118ef565b90565b916146f06146ea6146f794806146be6146b86146b35f61197d565b610473565b91610473565b14614728575b846146df6146d96146d45f61197d565b610473565b91610473565b146146f9575b611c14565b92611c14565b9091613b1d565b565b614721600b600261471b61471561470f89614401565b936118ec565b91613b1a565b90614454565b50506146e5565b614750600b600161474a61474461473e89614401565b936118ec565b91613b1a565b90614454565b50506146c4565b9161477b5f61478094614768611492565b50614771611492565b5001929192612ae4565b6149c5565b91909190565b61479a61479561479f92613dba565b6108ac565b6104a2565b90565b90565b6147b96147b46147be926147a2565b6108ac565b6104a2565b90565b6147d66147db916147d06116fc565b5061176f565b613f49565b6147e560ff614786565b16806147fa6147f4601f6147a5565b916104a2565b116148025790565b5f632cd44ac360e21b81528061481a600482016106ba565b0390fd5b5490565b61482c6040611ade565b90565b5f5260205f2090565b6148418161481e565b82101561485b5761485360019161482f565b910201905f90565b6141c6565b634e487b7160e01b5f525f60045260245ffd5b61487d9051610eae565b90565b9061489165ffffffffffff91610ff4565b9181191691161790565b6148af6148aa6148b492610eae565b6108ac565b610eae565b90565b90565b906148cf6148ca6148d69261489b565b6148b7565b8254614880565b9055565b6148e49051611407565b90565b60301b90565b906148ff65ffffffffffff19916148e7565b9181191691161790565b61491d61491861492292611407565b6108ac565b611407565b90565b90565b9061493d61493861494492614909565b614925565b82546148ed565b9055565b9061497260205f6149789461496a828201614964848801614873565b906148ba565b0192016148da565b90614928565b565b919061498b5761498991614948565b565b614860565b90815491680100000000000000008310156149c057826149b89160016149be95018155614838565b9061497a565b565b611659565b909291926149d1611492565b506149da611492565b506149e48261481e565b806149f76149f15f611989565b916104a2565b115f14614ac757614a1d90614a178491614a116001612b16565b90611db6565b90613a6c565b90614a295f8301612b06565b92614a355f8401612b57565b9380614a49614a4385610eae565b91610eae565b11614aab57614a60614a5a84610eae565b91610eae565b145f14614a7b575050614a76905f859101614928565b5b9190565b614aa69250614aa186614a98614a8f614822565b945f860161420b565b60208401614219565b614990565b614a77565b5f632520601d60e01b815280614ac3600482016106ba565b0390fd5b50614af291614aed85614ae4614adb614822565b945f860161420b565b60208401614219565b614990565b614afb5f612b64565b9190565b634e487b7160e01b5f52605160045260245ffd5b91909180600114614b3257600203614aff57614b2e91611511565b905b565b50614b3c916114d2565b90614b3056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\x14\x8EV[a\0\x1D_5a\x02\xFCV[\x80c\x01\xFF\xC9\xA7\x14a\x02\xF7W\x80c\x06\xFD\xDE\x03\x14a\x02\xF2W\x80c\t^\xA7\xB3\x14a\x02\xEDW\x80c\x18\x16\r\xDD\x14a\x02\xE8W\x80c#\xB8r\xDD\x14a\x02\xE3W\x80c$\x8A\x9C\xA3\x14a\x02\xDEW\x80c//\xF1]\x14a\x02\xD9W\x80c1<\xE5g\x14a\x02\xD4W\x80c6D\xE5\x15\x14a\x02\xCFW\x80c6V\x8A\xBE\x14a\x02\xCAW\x80c:F\xB1\xA8\x14a\x02\xC5W\x80c@\xC1\x0F\x19\x14a\x02\xC0W\x80cB\x96lh\x14a\x02\xBBW\x80cK\xF5\xD7\xE9\x14a\x02\xB6W\x80cO\x1B\xFC\x9E\x14a\x02\xB1W\x80cX|\xDE\x1E\x14a\x02\xACW\x80c\\\x19\xA9\\\x14a\x02\xA7W\x80co\xCF\xFFE\x14a\x02\xA2W\x80cp\xA0\x821\x14a\x02\x9DW\x80cy\xCCg\x90\x14a\x02\x98W\x80cz\x8C\xD1V\x14a\x02\x93W\x80c~\xCE\xBE\0\x14a\x02\x8EW\x80c\x83\xF1!\x1B\x14a\x02\x89W\x80c\x84&\xAD\xF2\x14a\x02\x84W\x80c\x84L\x90&\x14a\x02\x7FW\x80c\x84\xB0\x19n\x14a\x02zW\x80c\x8AT%!\x14a\x02uW\x80c\x8D3C\xD6\x14a\x02pW\x80c\x8ES\x9E\x8C\x14a\x02kW\x80c\x90-U\xA5\x14a\x02fW\x80c\x91\xD1HT\x14a\x02aW\x80c\x91\xDD\xAD\xF4\x14a\x02\\W\x80c\x95\xD8\x9BA\x14a\x02WW\x80c\x9A\xB2N\xB0\x14a\x02RW\x80c\x9B~\xF6K\x14a\x02MW\x80c\xA2\x17\xFD\xDF\x14a\x02HW\x80c\xA9\x05\x9C\xBB\x14a\x02CW\x80c\xAA\x08*\x9D\x14a\x02>W\x80c\xB0\xCA%>\x14a\x029W\x80c\xBBMD6\x14a\x024W\x80c\xC0*\xE7T\x14a\x02/W\x80c\xC3\xCD\xA5 \x14a\x02*W\x80c\xD5\x05\xAC\xCF\x14a\x02%W\x80c\xD5Gt\x1F\x14a\x02 W\x80c\xDDb\xED>\x14a\x02\x1BWc\xF1\x12~\xD8\x03a\0\x0EWa\x14XV[a\x13tV[a\x13\x13V[a\x12\xD9V[a\x12/V[a\x11sV[a\x11>V[a\x11\x08V[a\x10\xD3V[a\x10aV[a\x10,V[a\x0F\xBCV[a\x0FEV[a\x0F\x10V[a\x0E\xDBV[a\x0ExV[a\x0ECV[a\r\xCCV[a\r\x97V[a\r3V[a\x0C\xC8V[a\x0B\x83V[a\x0BNV[a\n\xF5V[a\n\xC0V[a\n\x8BV[a\nWV[a\n\"V[a\t\xEDV[a\t\x8FV[a\tZV[a\x08\xE5V[a\x08tV[a\x08AV[a\x07\xEFV[a\x07\xB9V[a\x07\x85V[a\x07PV[a\x07\x1BV[a\x06\xBFV[a\x06XV[a\x05\xBCV[a\x05MV[a\x04\xF5V[a\x043V[a\x03\x84V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[c\xFF\xFF\xFF\xFF`\xE0\x1B\x16\x90V[a\x03%\x81a\x03\x10V[\x03a\x03,WV[_\x80\xFD[\x90P5\x90a\x03=\x82a\x03\x1CV[V[\x90` \x82\x82\x03\x12a\x03XWa\x03U\x91_\x01a\x030V[\x90V[a\x03\x0CV[\x15\x15\x90V[a\x03k\x90a\x03]V[\x90RV[\x91\x90a\x03\x82\x90_` \x85\x01\x94\x01\x90a\x03bV[V[4a\x03\xB4Wa\x03\xB0a\x03\x9Fa\x03\x9A6`\x04a\x03?V[a\x15+V[a\x03\xA7a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[_\x91\x03\x12a\x03\xC3WV[a\x03\x0CV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x04\ta\x04\x12` \x93a\x04\x17\x93a\x04\0\x81a\x03\xC8V[\x93\x84\x80\x93a\x03\xCCV[\x95\x86\x91\x01a\x03\xD5V[a\x03\xE0V[\x01\x90V[a\x040\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03\xEAV[\x90V[4a\x04cWa\x04C6`\x04a\x03\xB9V[a\x04_a\x04Na\x16\xC4V[a\x04Va\x03\x02V[\x91\x82\x91\x82a\x04\x1BV[\x03\x90\xF3[a\x03\x08V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04|\x90a\x04hV[\x90V[a\x04\x88\x81a\x04sV[\x03a\x04\x8FWV[_\x80\xFD[\x90P5\x90a\x04\xA0\x82a\x04\x7FV[V[\x90V[a\x04\xAE\x81a\x04\xA2V[\x03a\x04\xB5WV[_\x80\xFD[\x90P5\x90a\x04\xC6\x82a\x04\xA5V[V[\x91\x90`@\x83\x82\x03\x12a\x04\xF0W\x80a\x04\xE4a\x04\xED\x92_\x86\x01a\x04\x93V[\x93` \x01a\x04\xB9V[\x90V[a\x03\x0CV[4a\x05&Wa\x05\"a\x05\x11a\x05\x0B6`\x04a\x04\xC8V[\x90a\x16\xDAV[a\x05\x19a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[a\x054\x90a\x04\xA2V[\x90RV[\x91\x90a\x05K\x90_` \x85\x01\x94\x01\x90a\x05+V[V[4a\x05}Wa\x05]6`\x04a\x03\xB9V[a\x05ya\x05ha\x17&V[a\x05pa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90\x91``\x82\x84\x03\x12a\x05\xB7Wa\x05\xB4a\x05\x9D\x84_\x85\x01a\x04\x93V[\x93a\x05\xAB\x81` \x86\x01a\x04\x93V[\x93`@\x01a\x04\xB9V[\x90V[a\x03\x0CV[4a\x05\xEDWa\x05\xE9a\x05\xD8a\x05\xD26`\x04a\x05\x82V[\x91a\x17<V[a\x05\xE0a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[\x90V[a\x05\xFE\x81a\x05\xF2V[\x03a\x06\x05WV[_\x80\xFD[\x90P5\x90a\x06\x16\x82a\x05\xF5V[V[\x90` \x82\x82\x03\x12a\x061Wa\x06.\x91_\x01a\x06\tV[\x90V[a\x03\x0CV[a\x06?\x90a\x05\xF2V[\x90RV[\x91\x90a\x06V\x90_` \x85\x01\x94\x01\x90a\x066V[V[4a\x06\x88Wa\x06\x84a\x06sa\x06n6`\x04a\x06\x18V[a\x17\xB5V[a\x06{a\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[\x91\x90`@\x83\x82\x03\x12a\x06\xB5W\x80a\x06\xA9a\x06\xB2\x92_\x86\x01a\x06\tV[\x93` \x01a\x04\x93V[\x90V[a\x03\x0CV[_\x01\x90V[4a\x06\xEEWa\x06\xD8a\x06\xD26`\x04a\x06\x8DV[\x90a\x18\x01V[a\x06\xE0a\x03\x02V[\x80a\x06\xEA\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[`\xFF\x16\x90V[a\x07\x02\x90a\x06\xF3V[\x90RV[\x91\x90a\x07\x19\x90_` \x85\x01\x94\x01\x90a\x06\xF9V[V[4a\x07KWa\x07+6`\x04a\x03\xB9V[a\x07Ga\x076a\x180V[a\x07>a\x03\x02V[\x91\x82\x91\x82a\x07\x06V[\x03\x90\xF3[a\x03\x08V[4a\x07\x80Wa\x07`6`\x04a\x03\xB9V[a\x07|a\x07ka\x18FV[a\x07sa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[4a\x07\xB4Wa\x07\x9Ea\x07\x986`\x04a\x06\x8DV[\x90a\x18ZV[a\x07\xA6a\x03\x02V[\x80a\x07\xB0\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\x07\xEAWa\x07\xE6a\x07\xD5a\x07\xCF6`\x04a\x04\xC8V[\x90a\x19\x0BV[a\x07\xDDa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x08\x1EWa\x08\x08a\x08\x026`\x04a\x04\xC8V[\x90a\x1A\x92V[a\x08\x10a\x03\x02V[\x80a\x08\x1A\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[\x90` \x82\x82\x03\x12a\x08<Wa\x089\x91_\x01a\x04\xB9V[\x90V[a\x03\x0CV[4a\x08oWa\x08Ya\x08T6`\x04a\x08#V[a\x1A\x9EV[a\x08aa\x03\x02V[\x80a\x08k\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\x08\xA4Wa\x08\x846`\x04a\x03\xB9V[a\x08\xA0a\x08\x8Fa\x1BxV[a\x08\x97a\x03\x02V[\x91\x82\x91\x82a\x04\x1BV[\x03\x90\xF3[a\x03\x08V[\x90V[\x90V[a\x08\xC3a\x08\xBEa\x08\xC8\x92a\x08\xA9V[a\x08\xACV[a\x04\xA2V[\x90V[a\x08\xD7b\x9E4\0a\x08\xAFV[\x90V[a\x08\xE2a\x08\xCBV[\x90V[4a\t\x15Wa\x08\xF56`\x04a\x03\xB9V[a\t\x11a\t\0a\x08\xDAV[a\t\x08a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90` \x82\x82\x03\x12a\t3Wa\t0\x91_\x01a\x04\x93V[\x90V[a\x03\x0CV[a\tA\x90a\x04sV[\x90RV[\x91\x90a\tX\x90_` \x85\x01\x94\x01\x90a\t8V[V[4a\t\x8AWa\t\x86a\tua\tp6`\x04a\t\x1AV[a\x1C\x14V[a\t}a\x03\x02V[\x91\x82\x91\x82a\tEV[\x03\x90\xF3[a\x03\x08V[4a\t\xBDWa\t\xA7a\t\xA26`\x04a\t\x1AV[a\x1C3V[a\t\xAFa\x03\x02V[\x80a\t\xB9\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[c\xFF\xFF\xFF\xFF\x16\x90V[a\t\xD4\x90a\t\xC2V[\x90RV[\x91\x90a\t\xEB\x90_` \x85\x01\x94\x01\x90a\t\xCBV[V[4a\n\x1DWa\n\x19a\n\x08a\n\x036`\x04a\t\x1AV[a\x1CJV[a\n\x10a\x03\x02V[\x91\x82\x91\x82a\t\xD8V[\x03\x90\xF3[a\x03\x08V[4a\nRWa\nNa\n=a\n86`\x04a\t\x1AV[a\x1CuV[a\nEa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\n\x86Wa\npa\nj6`\x04a\x04\xC8V[\x90a\x1D\xAAV[a\nxa\x03\x02V[\x80a\n\x82\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\n\xBBWa\n\x9B6`\x04a\x03\xB9V[a\n\xB7a\n\xA6a\x1D\xDBV[a\n\xAEa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\n\xF0Wa\n\xECa\n\xDBa\n\xD66`\x04a\t\x1AV[a\x1ESV[a\n\xE3a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x0B%Wa\x0B\x056`\x04a\x03\xB9V[a\x0B!a\x0B\x10a\x1EhV[a\x0B\x18a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0B~Wa\x0B^6`\x04a\x03\xB9V[a\x0Bza\x0Bia\x0B*V[a\x0Bqa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x0B\xB1Wa\x0B\x9Ba\x0B\x966`\x04a\x08#V[a 3V[a\x0B\xA3a\x03\x02V[\x80a\x0B\xAD\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[`\xFF`\xF8\x1B\x16\x90V[a\x0B\xC8\x90a\x0B\xB6V[\x90RV[Q\x90V[` \x91\x81R\x01\x90V[` \x01\x90V[a\x0B\xE8\x90a\x04\xA2V[\x90RV[\x90a\x0B\xF9\x81` \x93a\x0B\xDFV[\x01\x90V[` \x01\x90V[\x90a\x0C a\x0C\x1Aa\x0C\x13\x84a\x0B\xCCV[\x80\x93a\x0B\xD0V[\x92a\x0B\xD9V[\x90_[\x81\x81\x10a\x0C0WPPP\x90V[\x90\x91\x92a\x0CIa\x0CC`\x01\x92\x86Qa\x0B\xECV[\x94a\x0B\xFDV[\x91\x01\x91\x90\x91a\x0C#V[\x93\x95\x91\x94a\x0C\xA4a\x0C\x99a\x0C\xB8\x95a\x0C\x8Ba\x0C\xAE\x95a\x0C\xC5\x9C\x9Aa\x0C~`\xE0\x8C\x01\x92_\x8D\x01\x90a\x0B\xBFV[\x8A\x82\x03` \x8C\x01Ra\x03\xEAV[\x90\x88\x82\x03`@\x8A\x01Ra\x03\xEAV[\x97``\x87\x01\x90a\x05+V[`\x80\x85\x01\x90a\t8V[`\xA0\x83\x01\x90a\x066V[`\xC0\x81\x84\x03\x91\x01Ra\x0C\x03V[\x90V[4a\x0C\xFFWa\x0C\xD86`\x04a\x03\xB9V[a\x0C\xFBa\x0C\xE3a \xBBV[\x93a\x0C\xF2\x97\x95\x97\x93\x91\x93a\x03\x02V[\x97\x88\x97\x88a\x0CSV[\x03\x90\xF3[a\x03\x08V[\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t\x90V[a\r0a\r\x04V[\x90V[4a\rcWa\rC6`\x04a\x03\xB9V[a\r_a\rNa\r(V[a\rVa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[\x7F\x9B\x12\xE0\xC5p~II\x15\xE5\x8B\x05d\xF1\x8A\xAA\xD9\xB7J\xC6\x9B\xFC\x81Z\x1E\xDA\xDC\x8EK\xD02\xEB\x90V[a\r\x94a\rhV[\x90V[4a\r\xC7Wa\r\xA76`\x04a\x03\xB9V[a\r\xC3a\r\xB2a\r\x8CV[a\r\xBAa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[4a\r\xFCWa\r\xF8a\r\xE7a\r\xE26`\x04a\x08#V[a!EV[a\r\xEFa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90V[a\x0E\x18a\x0E\x13a\x0E\x1D\x92a\x0E\x01V[a\x08\xACV[a\x04\xA2V[\x90V[a\x0E5k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x0E\x04V[\x90V[a\x0E@a\x0E V[\x90V[4a\x0EsWa\x0ES6`\x04a\x03\xB9V[a\x0Eoa\x0E^a\x0E8V[a\x0Efa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x0E\xA9Wa\x0E\xA5a\x0E\x94a\x0E\x8E6`\x04a\x06\x8DV[\x90a!\xB3V[a\x0E\x9Ca\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x0E\xC2\x90a\x0E\xAEV[\x90RV[\x91\x90a\x0E\xD9\x90_` \x85\x01\x94\x01\x90a\x0E\xB9V[V[4a\x0F\x0BWa\x0E\xEB6`\x04a\x03\xB9V[a\x0F\x07a\x0E\xF6a!\xE1V[a\x0E\xFEa\x03\x02V[\x91\x82\x91\x82a\x0E\xC6V[\x03\x90\xF3[a\x03\x08V[4a\x0F@Wa\x0F 6`\x04a\x03\xB9V[a\x0F<a\x0F+a!\xF5V[a\x0F3a\x03\x02V[\x91\x82\x91\x82a\x04\x1BV[\x03\x90\xF3[a\x03\x08V[4a\x0FuWa\x0Fqa\x0F`a\x0F[6`\x04a\t\x1AV[a\"\x0BV[a\x0Fha\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90V[a\x0F\x91a\x0F\x8Ca\x0F\x96\x92a\x0FzV[a\x08\xACV[a\x04\xA2V[\x90V[a\x0F\xAEk\x02\xE8vi\xC3\x08sj\x04\0\0\0a\x0F}V[\x90V[a\x0F\xB9a\x0F\x99V[\x90V[4a\x0F\xECWa\x0F\xCC6`\x04a\x03\xB9V[a\x0F\xE8a\x0F\xD7a\x0F\xB1V[a\x0F\xDFa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90V[_\x1B\x90V[a\x10\ra\x10\x08a\x10\x12\x92a\x0F\xF1V[a\x0F\xF4V[a\x05\xF2V[\x90V[a\x10\x1E_a\x0F\xF9V[\x90V[a\x10)a\x10\x15V[\x90V[4a\x10\\Wa\x10<6`\x04a\x03\xB9V[a\x10Xa\x10Ga\x10!V[a\x10Oa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[4a\x10\x92Wa\x10\x8Ea\x10}a\x10w6`\x04a\x04\xC8V[\x90a\":V[a\x10\x85a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[\x1C\x90V[\x90V[a\x10\xAE\x90`\x08a\x10\xB3\x93\x02a\x10\x97V[a\x10\x9BV[\x90V[\x90a\x10\xC1\x91Ta\x10\x9EV[\x90V[a\x10\xD0`\x0C_\x90a\x10\xB6V[\x90V[4a\x11\x03Wa\x10\xE36`\x04a\x03\xB9V[a\x10\xFFa\x10\xEEa\x10\xC4V[a\x10\xF6a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x119Wa\x115a\x11$a\x11\x1E6`\x04a\x04\xC8V[\x90a\"\\V[a\x11,a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x11nWa\x11ja\x11Ya\x11T6`\x04a\t\x1AV[a\"rV[a\x11aa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x11\xA3Wa\x11\x836`\x04a\x03\xB9V[a\x11\x9Fa\x11\x8Ea\"\x87V[a\x11\x96a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[a\x11\xB1\x81a\x06\xF3V[\x03a\x11\xB8WV[_\x80\xFD[\x90P5\x90a\x11\xC9\x82a\x11\xA8V[V[\x90\x91`\xC0\x82\x84\x03\x12a\x12*Wa\x11\xE3\x83_\x84\x01a\x04\x93V[\x92a\x11\xF1\x81` \x85\x01a\x04\xB9V[\x92a\x11\xFF\x82`@\x83\x01a\x04\xB9V[\x92a\x12'a\x12\x10\x84``\x85\x01a\x11\xBCV[\x93a\x12\x1E\x81`\x80\x86\x01a\x06\tV[\x93`\xA0\x01a\x06\tV[\x90V[a\x03\x0CV[4a\x12dWa\x12Na\x12B6`\x04a\x11\xCBV[\x94\x93\x90\x93\x92\x91\x92a#\x07V[a\x12Va\x03\x02V[\x80a\x12`\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[`\xE0\x81\x83\x03\x12a\x12\xD4Wa\x12\x7F\x82_\x83\x01a\x04\x93V[\x92a\x12\x8D\x83` \x84\x01a\x04\x93V[\x92a\x12\x9B\x81`@\x85\x01a\x04\xB9V[\x92a\x12\xA9\x82``\x83\x01a\x04\xB9V[\x92a\x12\xD1a\x12\xBA\x84`\x80\x85\x01a\x11\xBCV[\x93a\x12\xC8\x81`\xA0\x86\x01a\x06\tV[\x93`\xC0\x01a\x06\tV[\x90V[a\x03\x0CV[4a\x13\x0EWa\x12\xF8a\x12\xEC6`\x04a\x12iV[\x95\x94\x90\x94\x93\x91\x93a$[V[a\x13\0a\x03\x02V[\x80a\x13\n\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\x13BWa\x13,a\x13&6`\x04a\x06\x8DV[\x90a%yV[a\x134a\x03\x02V[\x80a\x13>\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[\x91\x90`@\x83\x82\x03\x12a\x13oW\x80a\x13ca\x13l\x92_\x86\x01a\x04\x93V[\x93` \x01a\x04\x93V[\x90V[a\x03\x0CV[4a\x13\xA5Wa\x13\xA1a\x13\x90a\x13\x8A6`\x04a\x13GV[\x90a%\x9BV[a\x13\x98a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[a\x13\xB3\x81a\t\xC2V[\x03a\x13\xBAWV[_\x80\xFD[\x90P5\x90a\x13\xCB\x82a\x13\xAAV[V[\x91\x90`@\x83\x82\x03\x12a\x13\xF5W\x80a\x13\xE9a\x13\xF2\x92_\x86\x01a\x04\x93V[\x93` \x01a\x13\xBEV[\x90V[a\x03\x0CV[a\x14\x03\x90a\x0E\xAEV[\x90RV[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x14\x1B\x90a\x14\x07V[\x90RV[\x90` \x80a\x14A\x93a\x147_\x82\x01Q_\x86\x01\x90a\x13\xFAV[\x01Q\x91\x01\x90a\x14\x12V[V[\x91\x90a\x14V\x90_`@\x85\x01\x94\x01\x90a\x14\x1FV[V[4a\x14\x89Wa\x14\x85a\x14ta\x14n6`\x04a\x13\xCDV[\x90a&\tV[a\x14|a\x03\x02V[\x91\x82\x91\x82a\x14CV[\x03\x90\xF3[a\x03\x08V[_\x80\xFD[_\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x14\xB6a\x14\xBC\x91a\x14\x07V[\x91a\x14\x07V[\x01\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x14\xCDWV[a\x14\x96V[\x90a\x14\xE5\x91a\x14\xDFa\x14\x92V[Pa\x14\xAAV[\x90V[a\x14\xF4a\x14\xFA\x91a\x14\x07V[\x91a\x14\x07V[\x90\x03\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x15\x0CWV[a\x14\x96V[\x90a\x15$\x91a\x15\x1Ea\x14\x92V[Pa\x14\xE8V[\x90V[_\x90V[a\x153a\x15'V[P\x80a\x15Na\x15Hcye\xDB\x0B`\xE0\x1Ba\x03\x10V[\x91a\x03\x10V[\x14\x90\x81\x15a\x15[W[P\x90V[a\x15e\x91Pa&\x1FV[_a\x15WV[``\x90V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x01`\x02\x83\x04\x92\x16\x80\x15a\x15\xA4W[` \x83\x10\x14a\x15\x9FWV[a\x15pV[\x91`\x7F\x16\x91a\x15\x94V[` \x91\x81R\x01\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a\x15\xDAa\x15\xD3\x83a\x15\x84V[\x80\x94a\x15\xAEV[\x91`\x01\x81\x16\x90\x81_\x14a\x161WP`\x01\x14a\x15\xF5W[PPPV[a\x16\x02\x91\x92\x93\x94Pa\x15\xB7V[\x91_\x92[\x81\x84\x10a\x16\x19WPP\x01\x90_\x80\x80a\x15\xF0V[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a\x16\x06V[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a\x15\xF0V[\x90a\x16V\x91a\x15\xC0V[\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x16w\x90a\x03\xE0V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x16\x91W`@RV[a\x16YV[\x90a\x16\xB6a\x16\xAF\x92a\x16\xA6a\x03\x02V[\x93\x84\x80\x92a\x16LV[\x03\x83a\x16mV[V[a\x16\xC1\x90a\x16\x96V[\x90V[a\x16\xCCa\x15kV[Pa\x16\xD7`\x03a\x16\xB8V[\x90V[a\x16\xF7\x91a\x16\xE6a\x15'V[Pa\x16\xEFa&EV[\x91\x90\x91a&RV[`\x01\x90V[_\x90V[_\x1C\x90V[a\x17\x11a\x17\x16\x91a\x17\0V[a\x10\x9BV[\x90V[a\x17#\x90Ta\x17\x05V[\x90V[a\x17.a\x16\xFCV[Pa\x179`\x02a\x17\x19V[\x90V[\x91a\x17f\x92a\x17Ia\x15'V[Pa\x17^a\x17Ua&EV[\x82\x90\x84\x91a&\xA2V[\x91\x90\x91a'.V[`\x01\x90V[_\x90V[a\x17x\x90a\x05\xF2V[\x90V[\x90a\x17\x85\x90a\x17oV[_R` R`@_ \x90V[\x90V[a\x17\xA0a\x17\xA5\x91a\x17\0V[a\x17\x91V[\x90V[a\x17\xB2\x90Ta\x17\x94V[\x90V[`\x01a\x17\xCEa\x17\xD4\x92a\x17\xC6a\x17kV[P`\x05a\x17{V[\x01a\x17\xA8V[\x90V[\x90a\x17\xF2\x91a\x17\xEDa\x17\xE8\x82a\x17\xB5V[a'\xCBV[a\x17\xF4V[V[\x90a\x17\xFE\x91a($V[PV[\x90a\x18\x0B\x91a\x17\xD7V[V[_\x90V[\x90V[a\x18(a\x18#a\x18-\x92a\x18\x11V[a\x08\xACV[a\x06\xF3V[\x90V[a\x188a\x18\rV[Pa\x18C`\x12a\x18\x14V[\x90V[a\x18Na\x17kV[Pa\x18Wa(\xD0V[\x90V[\x90\x80a\x18ua\x18oa\x18ja&EV[a\x04sV[\x91a\x04sV[\x03a\x18\x86Wa\x18\x83\x91a)\x8AV[PV[_c3K\xD9\x19`\xE1\x1B\x81R\x80a\x18\x9E`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[a\x18\xB6a\x18\xB1a\x18\xBB\x92a\x04hV[a\x08\xACV[a\x04hV[\x90V[a\x18\xC7\x90a\x18\xA2V[\x90V[a\x18\xD3\x90a\x18\xBEV[\x90V[\x90a\x18\xE0\x90a\x18\xCAV[_R` R`@_ \x90V[\x90V[a\x19\x03a\x18\xFEa\x19\x08\x92a\x14\x07V[a\x08\xACV[a\x04\xA2V[\x90V[a\x19B\x91a\x197a\x191a\x19,a\x19=\x94a\x19$a\x16\xFCV[P`\na\x18\xD6V[a\x18\xECV[\x91a*kV[\x90a+\x80V[a\x18\xEFV[\x90V[\x90a\x19_\x91a\x19Za\x19Ua\rhV[a'\xCBV[a\x19\xCAV[V[a\x19ua\x19pa\x19z\x92a\x0F\xF1V[a\x08\xACV[a\x04hV[\x90V[a\x19\x86\x90a\x19aV[\x90V[a\x19\x9Da\x19\x98a\x19\xA2\x92a\x0F\xF1V[a\x08\xACV[a\x04\xA2V[\x90V[a\x19\xB4a\x19\xBA\x91\x93\x92\x93a\x04\xA2V[\x92a\x04\xA2V[\x82\x01\x80\x92\x11a\x19\xC5WV[a\x14\x96V[\x90\x81a\x19\xE6a\x19\xE0a\x19\xDB_a\x19}V[a\x04sV[\x91a\x04sV[\x14a\x1AvW\x80a\x19\xFEa\x19\xF8_a\x19\x89V[\x91a\x04\xA2V[\x14a\x1AZWa\x1A\x15a\x1A\x0Ea\x17&V[\x82\x90a\x19\xA5V[a\x1A.a\x1A(a\x1A#a\x0E V[a\x04\xA2V[\x91a\x04\xA2V[\x11a\x1A>Wa\x1A<\x91a,\xA7V[V[_c\x17~?\xC3`\xE0\x1B\x81R\x80a\x1AV`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1Ar`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x1A\x8E`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x90a\x1A\x9C\x91a\x19EV[V[\x80a\x1A\xB1a\x1A\xAB_a\x19\x89V[\x91a\x04\xA2V[\x14a\x1A\xC2Wa\x1A\xC0\x903a-\x05V[V[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1A\xDA`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x90a\x1A\xF1a\x1A\xEAa\x03\x02V[\x92\x83a\x16mV[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1B\x11Wa\x1B\r` \x91a\x03\xE0V[\x01\x90V[a\x16YV[\x90a\x1B(a\x1B#\x83a\x1A\xF3V[a\x1A\xDEV[\x91\x82RV[_\x7Fmode=blocknumber&from=default\0\0\0\x91\x01RV[a\x1B^`\x1Da\x1B\x16V[\x90a\x1Bk` \x83\x01a\x1B-V[V[a\x1Bua\x1BTV[\x90V[a\x1B\x80a\x15kV[Pa\x1B\x89a!\xE1V[a\x1B\xA2a\x1B\x9Ca\x1B\x97a-dV[a\x0E\xAEV[\x91a\x0E\xAEV[\x03a\x1B\xB2Wa\x1B\xAFa\x1BmV[\x90V[_c\x01\xBF\xC1\xC5`\xE6\x1B\x81R\x80a\x1B\xCA`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_\x90V[\x90a\x1B\xDC\x90a\x18\xCAV[_R` R`@_ \x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x1B\xFFa\x1C\x04\x91a\x17\0V[a\x1B\xE8V[\x90V[a\x1C\x11\x90Ta\x1B\xF3V[\x90V[a\x1C+a\x1C0\x91a\x1C#a\x1B\xCEV[P`\ta\x1B\xD2V[a\x1C\x07V[\x90V[a\x1CD\x90a\x1C?a&EV[a-\xB7V[V[_\x90V[a\x1C\\\x90a\x1CVa\x1CFV[Pa.BV[\x90V[\x90a\x1Ci\x90a\x18\xCAV[_R` R`@_ \x90V[a\x1C\x8Ba\x1C\x90\x91a\x1C\x84a\x16\xFCV[P_a\x1C_V[a\x17\x19V[\x90V[\x90a\x1C\xAD\x91a\x1C\xA8a\x1C\xA3a\r\x04V[a'\xCBV[a\x1C\xAFV[V[\x80a\x1C\xCAa\x1C\xC4a\x1C\xBF_a\x19}V[a\x04sV[\x91a\x04sV[\x14a\x1D\x8EW\x81a\x1C\xE2a\x1C\xDC_a\x19\x89V[\x91a\x04\xA2V[\x14a\x1DrWa\x1C\xF8a\x1C\xF2a\x1EhV[\x15a\x03]V[a\x1DVWa\x1D\x07\x81\x83\x90a-\x05V[3\x90a\x1DQa\x1D?a\x1D9\x7F\xBE\xF4\xF8\x1C\x18\x14\xC6A\xED\xE8^\xBA\xAC\xF1\x9D\x04\x8B,[U\x98\n\xDF\xA6\xEF\x0F\x95le\x135\xA2\x93a\x18\xCAV[\x93a\x18\xCAV[\x93a\x1DHa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xA3V[_c\xB8\xB5\xCA-`\xE0\x1B\x81R\x80a\x1Dn`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1D\x8A`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x1D\xA6`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x90a\x1D\xB4\x91a\x1C\x93V[V[a\x1D\xC5a\x1D\xCB\x91\x93\x92\x93a\x04\xA2V[\x92a\x04\xA2V[\x82\x03\x91\x82\x11a\x1D\xD6WV[a\x14\x96V[a\x1D\xE3a\x16\xFCV[Pa\x1D\xEE`\x0Ca\x17\x19V[a\x1E\0a\x1D\xFA_a\x19\x89V[\x91a\x04\xA2V[\x14\x80\x15a\x1E/W[a\x1E#Wa\x1E a\x1E\x19`\x0Ca\x17\x19V[B\x90a\x1D\xB6V[\x90V[a\x1E,_a\x19\x89V[\x90V[PBa\x1ELa\x1EFa\x1EA`\x0Ca\x17\x19V[a\x04\xA2V[\x91a\x04\xA2V[\x10\x15a\x1E\x08V[a\x1Ee\x90a\x1E_a\x16\xFCV[Pa.qV[\x90V[a\x1Epa\x15'V[Pa\x1E{`\x0Ca\x17\x19V[a\x1E\x8Da\x1E\x87_a\x19\x89V[\x91a\x04\xA2V[\x14\x15\x80a\x1E\x98W[\x90V[PBa\x1E\xB5a\x1E\xAFa\x1E\xAA`\x0Ca\x17\x19V[a\x04\xA2V[\x91a\x04\xA2V[\x10a\x1E\x95V[a\x1E\xD4\x90a\x1E\xCFa\x1E\xCAa\x10\x15V[a'\xCBV[a\x1FNV[V[\x90a\x1E\xE2_\x19\x91a\x0F\xF4V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x1F\0a\x1E\xFBa\x1F\x05\x92a\x04\xA2V[a\x08\xACV[a\x04\xA2V[\x90V[\x90V[\x90a\x1F a\x1F\x1Ba\x1F'\x92a\x1E\xECV[a\x1F\x08V[\x82Ta\x1E\xD6V[\x90UV[\x91` a\x1FL\x92\x94\x93a\x1FE`@\x82\x01\x96_\x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[\x80a\x1Faa\x1F[Ba\x04\xA2V[\x91a\x04\xA2V[\x11\x15a \x17W\x80a\x1F\x9Aa\x1F\x94\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xA2V[\x91a\x04\xA2V[\x11a\x1F\xFBWa\x1F\xA9`\x0Ca\x17\x19V[a\x1F\xB4\x82`\x0Ca\x1F\x0BV[\x903\x90a\x1F\xE1\x7F\xDDh\x96\xDC\xF1\xD4\xB3\x11\xCC\xA8}\xD1\x9B\xBB\xA2\xEA\x9C\xE2\xF8g\xC1V\x88x\xA0C\x8Af\xA1\xAF\xEE\xEC\x92a\x18\xCAV[\x92a\x1F\xF6a\x1F\xEDa\x03\x02V[\x92\x83\x92\x83a\x1F+V[\x03\x90\xA2V[_c\xEFi\xAFe`\xE0\x1B\x81R\x80a \x13`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\xA5e\x83S`\xE0\x1B\x81R\x80a /`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[a <\x90a\x1E\xBBV[V[_\x90V[``\x90V[a P\x90a\x18\xBEV[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a kW` \x80\x91\x02\x01\x90V[a\x16YV[\x90a \x82a }\x83a SV[a\x1A\xDEV[\x91\x82RV[6\x907V[\x90a \xB1a \x99\x83a pV[\x92` \x80a \xA7\x86\x93a SV[\x92\x01\x91\x03\x90a \x87V[V[`\x0F`\xF8\x1B\x90V[a \xC3a >V[Pa \xCCa\x15kV[Pa \xD5a\x15kV[Pa \xDEa\x16\xFCV[Pa \xE7a\x1B\xCEV[Pa \xF0a\x17kV[Pa \xF9a BV[Pa!\x02a.\x89V[\x90a!\x0Ba.\xC9V[\x90F\x90a!\x170a GV[\x90a!!_a\x0F\xF9V[\x90a!3a!._a\x19\x89V[a \x8CV[\x90a!<a \xB3V[\x96\x95\x94\x93\x92\x91\x90V[a!na!s\x91a!Ta\x16\xFCV[Pa!ha!b`\x0Ba\x18\xECV[\x91a*kV[\x90a+\x80V[a\x18\xEFV[\x90V[\x90a!\x80\x90a\x18\xCAV[_R` R`@_ \x90V[`\xFF\x16\x90V[a!\x9Ea!\xA3\x91a\x17\0V[a!\x8CV[\x90V[a!\xB0\x90Ta!\x92V[\x90V[a!\xDA\x91_a!\xCFa!\xD5\x93a!\xC7a\x15'V[P`\x05a\x17{V[\x01a!vV[a!\xA6V[\x90V[_\x90V[a!\xE9a!\xDDV[Pa!\xF2a-dV[\x90V[a!\xFDa\x15kV[Pa\"\x08`\x04a\x16\xB8V[\x90V[a\"2a\"-a\"(a\"7\x93a\" a\x16\xFCV[P`\na\x18\xD6V[a\x18\xECV[a/\tV[a\x18\xEFV[\x90V[a\"W\x91a\"Fa\x15'V[Pa\"Oa&EV[\x91\x90\x91a'.V[`\x01\x90V[\x90a\"o\x91a\"ia\x16\xFCV[Pa\x19\x0BV[\x90V[a\"\x84\x90a\"~a\x16\xFCV[Pa\"\x0BV[\x90V[a\"\x8Fa\x16\xFCV[Pa\"\x98a\x17&V[\x90V[\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF\x90V[a\"\xF4a\"\xFB\x94a\"\xEA``\x94\x98\x97\x95a\"\xE0`\x80\x86\x01\x9A_\x87\x01\x90a\x066V[` \x85\x01\x90a\t8V[`@\x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[` \x01\x90V[Q\x90V[\x93\x95\x94\x90\x92\x91\x95Ba#!a#\x1B\x89a\x04\xA2V[\x91a\x04\xA2V[\x11a#\x9AW\x91a#\x8C\x91a#\x93\x93a#\x83a#\x98\x98\x99a#ka#Ba\"\x9BV[a#\\\x8B\x93\x8Ba#Pa\x03\x02V[\x95\x86\x94` \x86\x01a\"\xBFV[` \x82\x01\x81\x03\x82R\x03\x82a\x16mV[a#}a#w\x82a#\x03V[\x91a\"\xFDV[ a/~V[\x92\x90\x91\x92a/\x9BV[\x91\x82a/\xE5V[a-\xB7V[V[a#\xB5\x87_\x91\x82\x91c#A\xD7\x87`\xE1\x1B\x83R`\x04\x83\x01a\x058V[\x03\x90\xFD[\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x90V[\x91\x94a$%a$/\x92\x98\x97\x95a$\x1B`\xA0\x96a$\x11a$6\x9Aa$\x07`\xC0\x8A\x01\x9E_\x8B\x01\x90a\x066V[` \x89\x01\x90a\t8V[`@\x87\x01\x90a\t8V[``\x85\x01\x90a\x05+V[`\x80\x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[\x91` a$Y\x92\x94\x93a$R`@\x82\x01\x96_\x83\x01\x90a\t8V[\x01\x90a\t8V[V[\x96\x95\x91\x93\x92\x94\x90\x94Ba$va$p\x83a\x04\xA2V[\x91a\x04\xA2V[\x11a%0W\x90a$\xDFa$\xE8\x94\x93\x92a$\xC7a$\x90a#\xB9V[a$\xB8\x8C\x80\x94\x8C\x91a$\xA2\x8D\x91a07V[\x91\x92a$\xACa\x03\x02V[\x97\x88\x96` \x88\x01a#\xDDV[` \x82\x01\x81\x03\x82R\x03\x82a\x16mV[a$\xD9a$\xD3\x82a#\x03V[\x91a\"\xFDV[ a/~V[\x92\x90\x91\x92a/\x9BV[\x80a$\xFBa$\xF5\x87a\x04sV[\x91a\x04sV[\x03a%\x10WPa%\x0E\x92\x93\x91\x90\x91a&RV[V[\x84\x90a%,_\x92\x83\x92c%\xC0\x07#`\xE1\x1B\x84R`\x04\x84\x01a$8V[\x03\x90\xFD[a%K\x90_\x91\x82\x91c1<\x89\x81`\xE1\x1B\x83R`\x04\x83\x01a\x058V[\x03\x90\xFD[\x90a%j\x91a%ea%`\x82a\x17\xB5V[a'\xCBV[a%lV[V[\x90a%v\x91a)\x8AV[PV[\x90a%\x83\x91a%OV[V[\x90a%\x8F\x90a\x18\xCAV[_R` R`@_ \x90V[a%\xC0\x91a%\xB6a%\xBB\x92a%\xAEa\x16\xFCV[P`\x01a%\x85V[a\x1C_V[a\x17\x19V[\x90V[a%\xCD`@a\x1A\xDEV[\x90V[_\x90V[_\x90V[a%\xE0a%\xC3V[\x90` \x80\x83a%\xEDa%\xD0V[\x81R\x01a%\xF8a%\xD4V[\x81RPPV[a&\x06a%\xD8V[\x90V[\x90a&\x1C\x91a&\x16a%\xFEV[Pa0jV[\x90V[a&'a\x15'V[Pa&Aa&;c\x01\xFF\xC9\xA7`\xE0\x1Ba\x03\x10V[\x91a\x03\x10V[\x14\x90V[a&Ma\x1B\xCEV[P3\x90V[\x91a&`\x92\x91`\x01\x92a0\x92V[V[`@\x90a&\x8Ba&\x92\x94\x96\x95\x93\x96a&\x81``\x84\x01\x98_\x85\x01\x90a\t8V[` \x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[\x90a&\x9F\x91\x03a\x04\xA2V[\x90V[\x92\x91\x92a&\xB0\x81\x83\x90a%\x9BV[\x90\x81a&\xC5a&\xBF_\x19a\x04\xA2V[\x91a\x04\xA2V[\x10a&\xD2W[PPP\x90PV[\x81a&\xE5a&\xDF\x87a\x04\xA2V[\x91a\x04\xA2V[\x10a'\x0BWa'\x02\x93\x94a&\xFA\x91\x93\x92a&\x94V[\x90_\x92a0\x92V[\x80_\x80\x80a&\xCBV[Pa'*\x84\x92\x91\x92_\x93\x84\x93c}\xC7\xA0\xD9`\xE1\x1B\x85R`\x04\x85\x01a&bV[\x03\x90\xFD[\x91\x82a'Ja'Da'?_a\x19}V[a\x04sV[\x91a\x04sV[\x14a'\xA4W\x81a'ja'da'__a\x19}V[a\x04sV[\x91a\x04sV[\x14a'}Wa'{\x92\x91\x90\x91a1\xA1V[V[a'\xA0a'\x89_a\x19}V[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\tEV[\x03\x90\xFD[a'\xC7a'\xB0_a\x19}V[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\tEV[\x03\x90\xFD[a'\xDD\x90a'\xD7a&EV[\x90a2nV[V[\x90a'\xEB`\xFF\x91a\x0F\xF4V[\x91\x81\x19\x16\x91\x16\x17\x90V[a'\xFE\x90a\x03]V[\x90V[\x90V[\x90a(\x19a(\x14a( \x92a'\xF5V[a(\x01V[\x82Ta'\xDFV[\x90UV[a(,a\x15'V[Pa(Aa(;\x82\x84\x90a!\xB3V[\x15a\x03]V[_\x14a(\xCAWa(i`\x01a(d_a(\\`\x05\x86\x90a\x17{V[\x01\x85\x90a!vV[a(\x04V[\x90a(ra&EV[\x90a(\xAFa(\xA9a(\xA3\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x95a\x17oV[\x92a\x18\xCAV[\x92a\x18\xCAV[\x92a(\xB8a\x03\x02V[\x80a(\xC2\x81a\x06\xBAV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a(\xD8a\x17kV[Pa(\xE20a GV[a)\x14a)\x0E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04sV[\x91a\x04sV[\x14\x80a)PW[_\x14a)EW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a)Ma3\x1AV[\x90V[PFa)\x84a)~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xA2V[\x91a\x04\xA2V[\x14a)\x1BV[a)\x92a\x15'V[Pa)\x9E\x81\x83\x90a!\xB3V[_\x14a*&Wa)\xC5_a)\xC0_a)\xB8`\x05\x86\x90a\x17{V[\x01\x85\x90a!vV[a(\x04V[\x90a)\xCEa&EV[\x90a*\x0Ba*\x05a)\xFF\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x95a\x17oV[\x92a\x18\xCAV[\x92a\x18\xCAV[\x92a*\x14a\x03\x02V[\x80a*\x1E\x81a\x06\xBAV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a*@a*;a*E\x92a\x0E\xAEV[a\x08\xACV[a\x04\xA2V[\x90V[\x91` a*i\x92\x94\x93a*b`@\x82\x01\x96_\x83\x01\x90a\x05+V[\x01\x90a\x0E\xB9V[V[a*sa!\xDDV[Pa*|a!\xE1V[\x81a*\x8Fa*\x89\x83a*,V[\x91a\x04\xA2V[\x10\x15a*\xA2WPa*\x9F\x90a4#V[\x90V[\x90a*\xBD_\x92\x83\x92cvi\xFC\x0F`\xE1\x1B\x84R`\x04\x84\x01a*HV[\x03\x90\xFD[T\x90V[\x90V[a*\xDCa*\xD7a*\xE1\x92a*\xC5V[a\x08\xACV[a\x04\xA2V[\x90V[\x90V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a*\xFEa+\x03\x91a\x17\0V[a*\xE7V[\x90V[a+\x10\x90Ta*\xF2V[\x90V[\x90V[a+*a+%a+/\x92a+\x13V[a\x08\xACV[a\x04\xA2V[\x90V[`0\x1C\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a+Oa+T\x91a+2V[a+8V[\x90V[a+a\x90Ta+CV[\x90V[a+xa+sa+}\x92a\x0F\xF1V[a\x08\xACV[a\x14\x07V[\x90V[\x90a+\xD4\x90a+\x8Da\x14\x92V[Pa+\x99_\x84\x01a*\xC1V[a+\xA2_a\x19\x89V[\x90\x80\x80a+\xB8a+\xB2`\x05a*\xC8V[\x91a\x04\xA2V[\x11a,5W[P\x90a+\xCF_\x86\x01\x93\x91\x92\x93a*\xE4V[a:vV[\x80a+\xE7a+\xE1_a\x19\x89V[\x91a\x04\xA2V[\x14_\x14a+\xFDWPPa+\xF9_a+dV[[\x90V[a,*_\x91a,%a,\x1F\x84a,0\x96\x01\x92a,\x19`\x01a+\x16V[\x90a\x1D\xB6V[\x91a*\xE4V[a:lV[\x01a+WV[a+\xFAV[\x80a,Ca,I\x92\x91a7\x01V[\x90a\x1D\xB6V[\x90\x83a,{a,ua,p_a,j\x81\x8C\x01a,e\x89\x91a*\xE4V[a:lV[\x01a+\x06V[a\x0E\xAEV[\x91a\x0E\xAEV[\x10_\x14a,\x8CWP\x90[\x90_a+\xBEV[\x91Pa,\xA2\x90a,\x9C`\x01a+\x16V[\x90a\x19\xA5V[a,\x85V[\x80a,\xC2a,\xBCa,\xB7_a\x19}V[a\x04sV[\x91a\x04sV[\x14a,\xDEWa,\xDC\x91a,\xD4_a\x19}V[\x91\x90\x91a1\xA1V[V[a-\x01a,\xEA_a\x19}V[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\tEV[\x03\x90\xFD[\x90\x81a-!a-\x1Ba-\x16_a\x19}V[a\x04sV[\x91a\x04sV[\x14a-=Wa-;\x91\x90a-4_a\x19}V[\x90\x91a1\xA1V[V[a-`a-I_a\x19}V[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\tEV[\x03\x90\xFD[a-la!\xDDV[Pa-vCa4#V[\x90V[\x90a-\x8A`\x01\x80`\xA0\x1B\x03\x91a\x0F\xF4V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a-\xACa-\xA7a-\xB3\x92a\x18\xCAV[a-\x94V[\x82Ta-yV[\x90UV[\x90a.@\x91a.:a-\xC8\x82a\x1C\x14V[a-\xDD\x84a-\xD8`\t\x86\x90a\x1B\xD2V[a-\x97V[\x82\x81\x85\x90a.\x1Da.\x17a.\x11\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x95a\x18\xCAV[\x92a\x18\xCAV[\x92a\x18\xCAV[\x92a.&a\x03\x02V[\x80a.0\x81a\x06\xBAV[\x03\x90\xA4\x92\x91a;\x05V[\x91a;\x1DV[V[a.ia.da._a.n\x93a.Wa\x1CFV[P`\na\x18\xD6V[a\x18\xECV[a<\xCBV[a=JV[\x90V[a.\x83\x90a.}a\x16\xFCV[Pa=\x9BV[\x90V[\x90V[a.\x91a\x15kV[Pa.\xC6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a.\xC0`\x06a.\x86V[\x90a>\xB6V[\x90V[a.\xD1a\x15kV[Pa/\x06\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a/\0`\x07a.\x86V[\x90a>\xB6V[\x90V[a/\x11a\x14\x92V[Pa/\x1D_\x82\x01a*\xC1V[\x80a/0a/*_a\x19\x89V[\x91a\x04\xA2V[\x14_\x14a/FWPPa/B_a+dV[[\x90V[a/s_\x91a/na/h\x84a/y\x96\x01\x92a/b`\x01a+\x16V[\x90a\x1D\xB6V[\x91a*\xE4V[a:lV[\x01a+WV[a/CV[a/\x98\x90a/\x8Aa\x17kV[Pa/\x93a(\xD0V[a?\x04V[\x90V[\x92a/\xB6\x92a/\xBF\x94a/\xACa\x1B\xCEV[P\x92\x90\x91\x92a?\xCAV[\x90\x92\x91\x92a@\xF5V[\x90V[\x91` a/\xE3\x92\x94\x93a/\xDC`@\x82\x01\x96_\x83\x01\x90a\t8V[\x01\x90a\x05+V[V[a/\xEE\x81a07V[\x91a0\x01a/\xFB\x84a\x04\xA2V[\x91a\x04\xA2V[\x03a0\nWPPV[a0$_\x92\x83\x92c\x01\xD4\xB6#`\xE6\x1B\x84R`\x04\x84\x01a/\xC2V[\x03\x90\xFD[`\x01a04\x91\x01a\x04\xA2V[\x90V[a0K\x90a0Ca\x16\xFCV[P`\x08a\x1C_V[a0ga0W\x82a\x17\x19V[\x91a0a\x83a0(V[\x90a\x1F\x0BV[\x90V[\x90a0\x8Aa0\x85a0\x8F\x93a0}a%\xFEV[P`\na\x18\xD6V[a\x18\xECV[aBkV[\x90V[\x90\x92\x81a0\xAFa0\xA9a0\xA4_a\x19}V[a\x04sV[\x91a\x04sV[\x14a1zW\x83a0\xCFa0\xC9a0\xC4_a\x19}V[a\x04sV[\x91a\x04sV[\x14a1SWa0\xF3\x83a0\xEEa0\xE7`\x01\x86\x90a%\x85V[\x87\x90a\x1C_V[a\x1F\x0BV[a0\xFDW[PPPV[\x91\x90\x91a1Ha16a10\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x93a\x18\xCAV[\x93a\x18\xCAV[\x93a1?a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xA3_\x80\x80a0\xF8V[a1va1__a\x19}V[_\x91\x82\x91cJ\x14\x06\xB1`\xE1\x1B\x83R`\x04\x83\x01a\tEV[\x03\x90\xFD[a1\x9Da1\x86_a\x19}V[_\x91\x82\x91c\xE6\x02\xDF\x05`\xE0\x1B\x83R`\x04\x83\x01a\tEV[\x03\x90\xFD[\x91\x82a1\xBDa1\xB7a1\xB2_a\x19}V[a\x04sV[\x91a\x04sV[\x14\x15\x80a2(W[a1\xD8W[a1\xD6\x92\x91\x90\x91aB\x8CV[V[a1\xE0a\x1EhV[\x80a2\x07W[\x15a1\xCAW_c6\xE2x\xFD`\xE2\x1B\x81R\x80a2\x03`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[Pa2#a2\x1Da2\x16a\r\x04V[3\x90a!\xB3V[\x15a\x03]V[a1\xE6V[P\x81a2Da2>a29_a\x19}V[a\x04sV[\x91a\x04sV[\x14\x15a1\xC5V[\x91` a2l\x92\x94\x93a2e`@\x82\x01\x96_\x83\x01\x90a\t8V[\x01\x90a\x066V[V[\x90a2\x83a2}\x83\x83\x90a!\xB3V[\x15a\x03]V[a2\x8BWPPV[a2\xA5_\x92\x83\x92c\xE2Q}?`\xE0\x1B\x84R`\x04\x84\x01a2KV[\x03\x90\xFD[\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x90V[\x90\x95\x94\x92a3\x18\x94a3\x07a3\x11\x92a2\xFD`\x80\x96a2\xF3`\xA0\x88\x01\x9C_\x89\x01\x90a\x066V[` \x87\x01\x90a\x066V[`@\x85\x01\x90a\x066V[``\x83\x01\x90a\x05+V[\x01\x90a\t8V[V[a3\"a\x17kV[Pa3+a2\xA9V[a3\xA2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a3\x93\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Fa3~0a GV[\x91a3\x87a\x03\x02V[\x96\x87\x95` \x87\x01a2\xCDV[` \x82\x01\x81\x03\x82R\x03\x82a\x16mV[a3\xB4a3\xAE\x82a#\x03V[\x91a\"\xFDV[ \x90V[\x90V[a3\xCFa3\xCAa3\xD4\x92a3\xB8V[a\x08\xACV[a\x06\xF3V[\x90V[a3\xE0\x90a3\xBBV[\x90RV[\x91` a4\x05\x92\x94\x93a3\xFE`@\x82\x01\x96_\x83\x01\x90a3\xD7V[\x01\x90a\x05+V[V[a4\x1Ba4\x16a4 \x92a\x04\xA2V[a\x08\xACV[a\x0E\xAEV[\x90V[a4+a!\xDDV[P\x80a4Ea4?e\xFF\xFF\xFF\xFF\xFF\xFFa*,V[\x91a\x04\xA2V[\x11a4VWa4S\x90a4\x07V[\x90V[`0a4r_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a3\xE4V[\x03\x90\xFD[\x90V[a4\x8Da4\x88a4\x92\x92a4vV[a\x08\xACV[a\x04\xA2V[\x90V[\x90V[a4\xACa4\xA7a4\xB1\x92a4\x95V[a\x08\xACV[a\x06\xF3V[\x90V[a4\xD3\x90a4\xCDa4\xC7a4\xD8\x94a\x06\xF3V[\x91a\x04\xA2V[\x90a\x10\x97V[a\x04\xA2V[\x90V[\x90V[a4\xF2a4\xEDa4\xF7\x92a4\xDBV[a\x08\xACV[a\x06\xF3V[\x90V[\x1B\x90V[a5\x1D\x90a5\x17a5\x11a5\"\x94a\x06\xF3V[\x91a\x04\xA2V[\x90a4\xFAV[a\x04\xA2V[\x90V[\x90V[a5<a57a5A\x92a5%V[a\x08\xACV[a\x04\xA2V[\x90V[\x90V[a5[a5Va5`\x92a5DV[a\x08\xACV[a\x06\xF3V[\x90V[\x90V[a5za5ua5\x7F\x92a5cV[a\x08\xACV[a\x04\xA2V[\x90V[\x90V[a5\x99a5\x94a5\x9E\x92a5\x82V[a\x08\xACV[a\x06\xF3V[\x90V[\x90V[a5\xB8a5\xB3a5\xBD\x92a5\xA1V[a\x08\xACV[a\x04\xA2V[\x90V[\x90V[a5\xD7a5\xD2a5\xDC\x92a5\xC0V[a\x08\xACV[a\x06\xF3V[\x90V[\x90V[a5\xF6a5\xF1a5\xFB\x92a5\xDFV[a\x08\xACV[a\x04\xA2V[\x90V[\x90V[a6\x15a6\x10a6\x1A\x92a5\xFEV[a\x08\xACV[a\x06\xF3V[\x90V[a61a6,a66\x92a5\x82V[a\x08\xACV[a\x04\xA2V[\x90V[\x90V[a6Pa6Ka6U\x92a69V[a\x08\xACV[a\x06\xF3V[\x90V[a6la6ga6q\x92a5\xFEV[a\x08\xACV[a\x04\xA2V[\x90V[a6\x88a6\x83a6\x8D\x92a+\x13V[a\x08\xACV[a\x06\xF3V[\x90V[\x90V[a6\xA7a6\xA2a6\xAC\x92a6\x90V[a\x08\xACV[a\x04\xA2V[\x90V[\x90a6\xBA\x91\x02a\x04\xA2V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a6\xDDa6\xE3\x91a\x04\xA2V[\x91a\x04\xA2V[\x90\x81\x15a6\xEEW\x04\x90V[a6\xBDV[\x90a6\xFE\x91\x01a\x04\xA2V[\x90V[a7\ta\x16\xFCV[P\x80a7\x1Ea7\x18`\x01a+\x16V[\x91a\x04\xA2V[\x11\x15a:iW\x80a93a9\x10a9\0a8\xF0a8\xE0a8\xD0a8\xC0a8\xB0a8\xA0a8\x90a8\x80\x8Ba8za8sa99\x9Fa8Sa8Ca8c\x92a7e`\x01a+\x16V[\x90\x80a7}a7w`\x01`\x80\x1Ba4yV[\x91a\x04\xA2V[\x10\x15a:;W[\x80a7\xA0a7\x9Ah\x01\0\0\0\0\0\0\0\0a5(V[\x91a\x04\xA2V[\x10\x15a:\rW[\x80a7\xBFa7\xB9d\x01\0\0\0\0a5fV[\x91a\x04\xA2V[\x10\x15a9\xDFW[\x80a7\xDCa7\xD6b\x01\0\0a5\xA4V[\x91a\x04\xA2V[\x10\x15a9\xB1W[\x80a7\xF8a7\xF2a\x01\0a5\xE2V[\x91a\x04\xA2V[\x10\x15a9\x83W[\x80a8\x13a8\r`\x10a6\x1DV[\x91a\x04\xA2V[\x10\x15a9UW[a8-a8'`\x04a6XV[\x91a\x04\xA2V[\x10\x15a9<W[a8>`\x03a6\x93V[a6\xAFV[a8M`\x01a6tV[\x90a4\xB4V[a8]\x81\x86a6\xD1V[\x90a6\xF3V[a8m`\x01a6tV[\x90a4\xB4V[\x80\x92a6\xD1V[\x90a6\xF3V[a8\x8A`\x01a6tV[\x90a4\xB4V[a8\x9A\x81\x8Ca6\xD1V[\x90a6\xF3V[a8\xAA`\x01a6tV[\x90a4\xB4V[a8\xBA\x81\x8Aa6\xD1V[\x90a6\xF3V[a8\xCA`\x01a6tV[\x90a4\xB4V[a8\xDA\x81\x88a6\xD1V[\x90a6\xF3V[a8\xEA`\x01a6tV[\x90a4\xB4V[a8\xFA\x81\x86a6\xD1V[\x90a6\xF3V[a9\n`\x01a6tV[\x90a4\xB4V[\x91a9-a9'a9\"\x85\x80\x94a6\xD1V[a\x04\xA2V[\x91a\x04\xA2V[\x11aC\x1CV[\x90a&\x94V[\x90V[a9P\x90a9J`\x01a6tV[\x90a4\xFEV[a84V[a9la9}\x91a9f`\x04a6\x01V[\x90a4\xB4V[\x91a9w`\x02a6<V[\x90a4\xFEV[\x90a8\x1AV[a9\x9Aa9\xAB\x91a9\x94`\x08a5\xC3V[\x90a4\xB4V[\x91a9\xA5`\x04a6\x01V[\x90a4\xFEV[\x90a7\xFFV[a9\xC8a9\xD9\x91a9\xC2`\x10a5\x85V[\x90a4\xB4V[\x91a9\xD3`\x08a5\xC3V[\x90a4\xFEV[\x90a7\xE3V[a9\xF6a:\x07\x91a9\xF0` a5GV[\x90a4\xB4V[\x91a:\x01`\x10a5\x85V[\x90a4\xFEV[\x90a7\xC6V[a:$a:5\x91a:\x1E`@a4\xDEV[\x90a4\xB4V[\x91a:/` a5GV[\x90a4\xFEV[\x90a7\xA7V[a:Ra:c\x91a:L`\x80a4\x98V[\x90a4\xB4V[\x91a:]`@a4\xDEV[\x90a4\xFEV[\x90a7\x84V[\x90V[_R` _ \x01\x90V[\x93\x91\x90\x92a:\x82a\x16\xFCV[P[\x81a:\x97a:\x91\x83a\x04\xA2V[\x91a\x04\xA2V[\x10\x15a:\xFDWa:\xA8\x82\x82\x90aChV[\x90a:\xBE_a:\xB8\x88\x85\x90a:lV[\x01a+\x06V[a:\xD0a:\xCA\x87a\x0E\xAEV[\x91a\x0E\xAEV[\x11_\x14a:\xE0WP\x91[\x91a:\x84V[\x92\x91Pa:\xF7\x90a:\xF1`\x01a+\x16V[\x90a\x19\xA5V[\x90a:\xDAV[\x92PP\x91P\x90V[a;\x17\x90a;\x11a\x16\xFCV[Pa\x1CuV[\x90V[\x90V[\x91\x90\x91\x80a;3a;-\x85a\x04sV[\x91a\x04sV[\x14\x15\x80a<\xB1W[a;EW[PPPV[\x80a;`a;Za;U_a\x19}V[a\x04sV[\x91a\x04sV[\x03a<!W[P\x81a;\x82a;|a;w_a\x19}V[a\x04sV[\x91a\x04sV[\x03a;\x8EW[\x80a;@V[a;\xD5a;\xC8a;\xCF\x92a;\xA4`\n\x86\x90a\x18\xD6V[\x90a;\xC2a;\xBCa;\xB6`\x01\x93aD\x01V[\x93a\x18\xECV[\x91a;\x1AV[\x90aDTV[\x92\x90a\x18\xEFV[\x91a\x18\xEFV[\x91\x90\x91a<\x02\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x18\xCAV[\x92a<\x17a<\x0Ea\x03\x02V[\x92\x83\x92\x83a\x1F+V[\x03\x90\xA2_\x80a;\x88V[a<`a<fa<Ya<6`\n\x85\x90a\x18\xD6V[`\x02a<Sa<Ma<G\x89aD\x01V[\x93a\x18\xECV[\x91a;\x1AV[\x90aDTV[\x92\x90a\x18\xEFV[\x91a\x18\xEFV[\x91\x90\x91a<\x93\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x18\xCAV[\x92a<\xA8a<\x9Fa\x03\x02V[\x92\x83\x92\x83a\x1F+V[\x03\x90\xA2_a;fV[P\x81a<\xC5a<\xBF_a\x19\x89V[\x91a\x04\xA2V[\x11a;;V[_a<\xDF\x91a<\xD8a\x16\xFCV[P\x01a*\xC1V[\x90V[a<\xF6a<\xF1a<\xFB\x92a\t\xC2V[a\x08\xACV[a\x04\xA2V[\x90V[a=\x07\x90a5GV[\x90RV[\x91` a=,\x92\x94\x93a=%`@\x82\x01\x96_\x83\x01\x90a<\xFEV[\x01\x90a\x05+V[V[a=Ba==a=G\x92a\x04\xA2V[a\x08\xACV[a\t\xC2V[\x90V[a=Ra\x1CFV[P\x80a=ja=dc\xFF\xFF\xFF\xFFa<\xE2V[\x91a\x04\xA2V[\x11a={Wa=x\x90a=.V[\x90V[` a=\x97_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a=\x0BV[\x03\x90\xFD[a=\xB2a=\xB7\x91a=\xAAa\x16\xFCV[P`\x08a\x1C_V[a\x17\x19V[\x90V[\x90V[a=\xD1a=\xCCa=\xD6\x92a=\xBAV[a\x0F\xF4V[a\x05\xF2V[\x90V[a=\xE3`\xFFa=\xBDV[\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a>\ta>\x02\x83a\x15\x84V[\x80\x94a\x15\xAEV[\x91`\x01\x81\x16\x90\x81_\x14a>`WP`\x01\x14a>$W[PPPV[a>1\x91\x92\x93\x94Pa=\xE6V[\x91_\x92[\x81\x84\x10a>HWPP\x01\x90_\x80\x80a>\x1FV[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a>5V[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a>\x1FV[\x90a>\x85\x91a=\xEFV[\x90V[\x90a>\xA8a>\xA1\x92a>\x98a\x03\x02V[\x93\x84\x80\x92a>{V[\x03\x83a\x16mV[V[a>\xB3\x90a>\x88V[\x90V[\x90a>\xBFa\x15kV[Pa>\xC9\x82a\x17oV[a>\xE2a>\xDCa>\xD7a=\xD9V[a\x05\xF2V[\x91a\x05\xF2V[\x14\x15_\x14a>\xF7WPa>\xF4\x90aD\xDEV[\x90V[a?\x01\x91Pa>\xAAV[\x90V[`B\x91a?\x0Fa\x17kV[P`@Q\x91\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[_\x90V[a?Ua?Z\x91a\x17\0V[a\x1E\xECV[\x90V[\x90V[a?ta?oa?y\x92a?]V[a\x08\xACV[a\x04\xA2V[\x90V[a?\xB1a?\xB8\x94a?\xA7``\x94\x98\x97\x95a?\x9D`\x80\x86\x01\x9A_\x87\x01\x90a\x066V[` \x85\x01\x90a\x06\xF9V[`@\x83\x01\x90a\x066V[\x01\x90a\x066V[V[a?\xC2a\x03\x02V[=_\x82>=\x90\xFD[\x93\x92\x93a?\xD5a\x1B\xCEV[Pa?\xDEa?EV[Pa?\xE7a\x17kV[Pa?\xF1\x85a?IV[a@#a@\x1D\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0a?`V[\x91a\x04\xA2V[\x11a@\xB0W\x90a@F` \x94\x95_\x94\x93\x92\x93a@=a\x03\x02V[\x94\x85\x94\x85a?|V[\x83\x80R\x03\x90`\x01Z\xFA\x15a@\xABWa@^_Qa\x0F\xF4V[\x80a@ya@sa@n_a\x19}V[a\x04sV[\x91a\x04sV[\x14a@\x8FW_\x91a@\x89_a\x0F\xF9V[\x91\x92\x91\x90V[Pa@\x99_a\x19}V[`\x01\x91a@\xA5_a\x0F\xF9V[\x91\x92\x91\x90V[a?\xBAV[PPPa@\xBC_a\x19}V[\x90`\x03\x92\x91\x92\x91\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x04\x11\x15a@\xE4WV[a@\xC6V[\x90a@\xF3\x82a@\xDAV[V[\x80aA\x08aA\x02_a@\xE9V[\x91a@\xE9V[\x14_\x14aA\x13WPPV[\x80aA'aA!`\x01a@\xE9V[\x91a@\xE9V[\x14_\x14aAJW_c\xF6E\xEE\xDF`\xE0\x1B\x81R\x80aAF`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x80aA^aAX`\x02a@\xE9V[\x91a@\xE9V[\x14_\x14aA\x8CWaA\x88aAq\x83a?IV[_\x91\x82\x91c\xFC\xE6\x98\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x058V[\x03\x90\xFD[aA\x9FaA\x99`\x03a@\xE9V[\x91a@\xE9V[\x14aA\xA7WPV[aA\xC2\x90_\x91\x82\x91c5\xE2\xF3\x83`\xE2\x1B\x83R`\x04\x83\x01a\x06CV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_R` _ \x90V[aA\xEC\x81a*\xC1V[\x82\x10\x15aB\x06WaA\xFE`\x01\x91aA\xDAV[\x91\x02\x01\x90_\x90V[aA\xC6V[\x90aB\x15\x90a\x0E\xAEV[\x90RV[\x90aB#\x90a\x14\x07V[\x90RV[\x90aB]aBT_aB7a%\xC3V[\x94aBNaBF\x83\x83\x01a+\x06V[\x83\x88\x01aB\x0BV[\x01a+WV[` \x84\x01aB\x19V[V[aBh\x90aB'V[\x90V[aB\x89\x91_aB\x83\x92aB|a%\xFEV[P\x01aA\xE3V[PaB_V[\x90V[\x92\x91aB\x9A\x84\x83\x83\x91aE\x0EV[\x83aB\xB5aB\xAFaB\xAA_a\x19}V[a\x04sV[\x91a\x04sV[\x14aB\xCAW[aB\xC8\x92\x93\x91\x90\x91aF\x98V[V[aB\xD2a\x17&V[\x93aB\xDBaF}V[\x94\x80aB\xEFaB\xE9\x88a\x04\xA2V[\x91a\x04\xA2V[\x11aB\xFCWP\x93PaB\xBBV[\x85\x90aC\x18_\x92\x83\x92c\x0EX\xAE\x93`\xE1\x1B\x84R`\x04\x84\x01a\x1F+V[\x03\x90\xFD[aC$a\x16\xFCV[P\x15\x15\x90V[aC>aC9aCC\x92a69V[a\x08\xACV[a\x04\xA2V[\x90V[aCRaCX\x91a\x04\xA2V[\x91a\x04\xA2V[\x90\x81\x15aCcW\x04\x90V[a6\xBDV[aC\x8DaC\x93\x92aCwa\x16\xFCV[P\x82\x81\x16\x92\x18aC\x87`\x02aC*V[\x90aCFV[\x90a\x19\xA5V[\x90V[\x90V[aC\xADaC\xA8aC\xB2\x92aC\x96V[a\x08\xACV[a\x06\xF3V[\x90V[aC\xBE\x90aC\x99V[\x90RV[\x91` aC\xE3\x92\x94\x93aC\xDC`@\x82\x01\x96_\x83\x01\x90aC\xB5V[\x01\x90a\x05+V[V[aC\xF9aC\xF4aC\xFE\x92a\x04\xA2V[a\x08\xACV[a\x14\x07V[\x90V[aD\ta\x14\x92V[P\x80aD#aD\x1D`\x01\x80`\xD0\x1B\x03a\x18\xEFV[\x91a\x04\xA2V[\x11aD4WaD1\x90aC\xE5V[\x90V[`\xD0aDP_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01aC\xC2V[\x03\x90\xFD[\x90aD\x8AaD\x90\x93\x92aDea\x14\x92V[PaDna\x14\x92V[P\x80\x93aD\x83aD|a!\xE1V[\x94\x92a/\tV[\x90\x91aK\x13V[\x91aGWV[\x91\x90\x91\x90V[aD\xAAaD\xA5aD\xAF\x92a5DV[a\x08\xACV[a\x04\xA2V[\x90V[6\x907V[\x90aD\xDCaD\xC4\x83a\x1B\x16V[\x92` \x80aD\xD2\x86\x93a\x1A\xF3V[\x92\x01\x91\x03\x90aD\xB2V[V[aD\xE6a\x15kV[PaD\xF0\x81aG\xC1V[\x90aE\x03aD\xFE` aD\x96V[aD\xB7V[\x91\x82R` \x82\x01R\x90V[\x91\x90\x91\x80aE,aE&aE!_a\x19}V[a\x04sV[\x91a\x04sV[\x14_\x14aF\rWaEPaEI\x83aED`\x02a\x17\x19V[a\x19\xA5V[`\x02a\x1F\x0BV[[\x82aElaEfaEa_a\x19}V[a\x04sV[\x91a\x04sV[\x14_\x14aE\xE1WaE\x90aE\x89\x83aE\x84`\x02a\x17\x19V[a&\x94V[`\x02a\x1F\x0BV[[\x91\x90\x91aE\xDCaE\xCAaE\xC4\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93a\x18\xCAV[\x93a\x18\xCAV[\x93aE\xD3a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xA3V[aF\x08\x82aF\x02aE\xF3_\x87\x90a\x1C_V[\x91aE\xFD\x83a\x17\x19V[a6\xF3V[\x90a\x1F\x0BV[aE\x91V[aF aF\x1B_\x83\x90a\x1C_V[a\x17\x19V[\x80aF3aF-\x85a\x04\xA2V[\x91a\x04\xA2V[\x10aF[WaFFaFV\x91\x84\x90a&\x94V[aFQ_\x84\x90a\x1C_V[a\x1F\x0BV[aEQV[\x90aFy\x90\x91\x92_\x93\x84\x93c9\x144\xE3`\xE2\x1B\x85R`\x04\x85\x01a&bV[\x03\x90\xFD[aF\x85a\x16\xFCV[PaF\x95`\x01\x80`\xD0\x1B\x03a\x18\xEFV[\x90V[\x91aF\xF0aF\xEAaF\xF7\x94\x80aF\xBEaF\xB8aF\xB3_a\x19}V[a\x04sV[\x91a\x04sV[\x14aG(W[\x84aF\xDFaF\xD9aF\xD4_a\x19}V[a\x04sV[\x91a\x04sV[\x14aF\xF9W[a\x1C\x14V[\x92a\x1C\x14V[\x90\x91a;\x1DV[V[aG!`\x0B`\x02aG\x1BaG\x15aG\x0F\x89aD\x01V[\x93a\x18\xECV[\x91a;\x1AV[\x90aDTV[PPaF\xE5V[aGP`\x0B`\x01aGJaGDaG>\x89aD\x01V[\x93a\x18\xECV[\x91a;\x1AV[\x90aDTV[PPaF\xC4V[\x91aG{_aG\x80\x94aGha\x14\x92V[PaGqa\x14\x92V[P\x01\x92\x91\x92a*\xE4V[aI\xC5V[\x91\x90\x91\x90V[aG\x9AaG\x95aG\x9F\x92a=\xBAV[a\x08\xACV[a\x04\xA2V[\x90V[\x90V[aG\xB9aG\xB4aG\xBE\x92aG\xA2V[a\x08\xACV[a\x04\xA2V[\x90V[aG\xD6aG\xDB\x91aG\xD0a\x16\xFCV[Pa\x17oV[a?IV[aG\xE5`\xFFaG\x86V[\x16\x80aG\xFAaG\xF4`\x1FaG\xA5V[\x91a\x04\xA2V[\x11aH\x02W\x90V[_c,\xD4J\xC3`\xE2\x1B\x81R\x80aH\x1A`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[T\x90V[aH,`@a\x1A\xDEV[\x90V[_R` _ \x90V[aHA\x81aH\x1EV[\x82\x10\x15aH[WaHS`\x01\x91aH/V[\x91\x02\x01\x90_\x90V[aA\xC6V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[aH}\x90Qa\x0E\xAEV[\x90V[\x90aH\x91e\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x0F\xF4V[\x91\x81\x19\x16\x91\x16\x17\x90V[aH\xAFaH\xAAaH\xB4\x92a\x0E\xAEV[a\x08\xACV[a\x0E\xAEV[\x90V[\x90V[\x90aH\xCFaH\xCAaH\xD6\x92aH\x9BV[aH\xB7V[\x82TaH\x80V[\x90UV[aH\xE4\x90Qa\x14\x07V[\x90V[`0\x1B\x90V[\x90aH\xFFe\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91aH\xE7V[\x91\x81\x19\x16\x91\x16\x17\x90V[aI\x1DaI\x18aI\"\x92a\x14\x07V[a\x08\xACV[a\x14\x07V[\x90V[\x90V[\x90aI=aI8aID\x92aI\tV[aI%V[\x82TaH\xEDV[\x90UV[\x90aIr` _aIx\x94aIj\x82\x82\x01aId\x84\x88\x01aHsV[\x90aH\xBAV[\x01\x92\x01aH\xDAV[\x90aI(V[V[\x91\x90aI\x8BWaI\x89\x91aIHV[V[aH`V[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15aI\xC0W\x82aI\xB8\x91`\x01aI\xBE\x95\x01\x81UaH8V[\x90aIzV[V[a\x16YV[\x90\x92\x91\x92aI\xD1a\x14\x92V[PaI\xDAa\x14\x92V[PaI\xE4\x82aH\x1EV[\x80aI\xF7aI\xF1_a\x19\x89V[\x91a\x04\xA2V[\x11_\x14aJ\xC7WaJ\x1D\x90aJ\x17\x84\x91aJ\x11`\x01a+\x16V[\x90a\x1D\xB6V[\x90a:lV[\x90aJ)_\x83\x01a+\x06V[\x92aJ5_\x84\x01a+WV[\x93\x80aJIaJC\x85a\x0E\xAEV[\x91a\x0E\xAEV[\x11aJ\xABWaJ`aJZ\x84a\x0E\xAEV[\x91a\x0E\xAEV[\x14_\x14aJ{WPPaJv\x90_\x85\x91\x01aI(V[[\x91\x90V[aJ\xA6\x92PaJ\xA1\x86aJ\x98aJ\x8FaH\"V[\x94_\x86\x01aB\x0BV[` \x84\x01aB\x19V[aI\x90V[aJwV[_c% `\x1D`\xE0\x1B\x81R\x80aJ\xC3`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[PaJ\xF2\x91aJ\xED\x85aJ\xE4aJ\xDBaH\"V[\x94_\x86\x01aB\x0BV[` \x84\x01aB\x19V[aI\x90V[aJ\xFB_a+dV[\x91\x90V[cNH{q`\xE0\x1B_R`Q`\x04R`$_\xFD[\x91\x90\x91\x80`\x01\x14aK2W`\x02\x03aJ\xFFWaK.\x91a\x15\x11V[\x90[V[PaK<\x91a\x14\xD2V[\x90aK0V",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AccessControlBadConfirmation()` and selector `0x6697b232`.
```solidity
error AccessControlBadConfirmation();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccessControlBadConfirmation;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AccessControlBadConfirmation>
        for UnderlyingRustTuple<'_> {
            fn from(value: AccessControlBadConfirmation) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AccessControlBadConfirmation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AccessControlBadConfirmation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AccessControlBadConfirmation()";
            const SELECTOR: [u8; 4] = [102u8, 151u8, 178u8, 50u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AccessControlUnauthorizedAccount(address,bytes32)` and selector `0xe2517d3f`.
```solidity
error AccessControlUnauthorizedAccount(address account, bytes32 neededRole);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccessControlUnauthorizedAccount {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub neededRole: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::FixedBytes<32>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AccessControlUnauthorizedAccount>
        for UnderlyingRustTuple<'_> {
            fn from(value: AccessControlUnauthorizedAccount) -> Self {
                (value.account, value.neededRole)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AccessControlUnauthorizedAccount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    account: tuple.0,
                    neededRole: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AccessControlUnauthorizedAccount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AccessControlUnauthorizedAccount(address,bytes32)";
            const SELECTOR: [u8; 4] = [226u8, 81u8, 125u8, 63u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.neededRole),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `BurnOnlyDuringLockPeriod()` and selector `0xb8b5ca2d`.
```solidity
error BurnOnlyDuringLockPeriod();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BurnOnlyDuringLockPeriod;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BurnOnlyDuringLockPeriod>
        for UnderlyingRustTuple<'_> {
            fn from(value: BurnOnlyDuringLockPeriod) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for BurnOnlyDuringLockPeriod {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BurnOnlyDuringLockPeriod {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BurnOnlyDuringLockPeriod()";
            const SELECTOR: [u8; 4] = [184u8, 181u8, 202u8, 45u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `CheckpointUnorderedInsertion()` and selector `0x2520601d`.
```solidity
error CheckpointUnorderedInsertion();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CheckpointUnorderedInsertion;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CheckpointUnorderedInsertion>
        for UnderlyingRustTuple<'_> {
            fn from(value: CheckpointUnorderedInsertion) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CheckpointUnorderedInsertion {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CheckpointUnorderedInsertion {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CheckpointUnorderedInsertion()";
            const SELECTOR: [u8; 4] = [37u8, 32u8, 96u8, 29u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`.
```solidity
error ECDSAInvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignature;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECDSAInvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignature()";
            const SELECTOR: [u8; 4] = [246u8, 69u8, 238u8, 223u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`.
```solidity
error ECDSAInvalidSignatureLength(uint256 length);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureLength {
        #[allow(missing_docs)]
        pub length: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECDSAInvalidSignatureLength>
        for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureLength) -> Self {
                (value.length,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ECDSAInvalidSignatureLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { length: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureLength(uint256)";
            const SELECTOR: [u8; 4] = [252u8, 230u8, 152u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`.
```solidity
error ECDSAInvalidSignatureS(bytes32 s);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureS {
        #[allow(missing_docs)]
        pub s: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECDSAInvalidSignatureS> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureS) -> Self {
                (value.s,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignatureS {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { s: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureS {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureS(bytes32)";
            const SELECTOR: [u8; 4] = [215u8, 139u8, 206u8, 12u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC20ExceededSafeSupply(uint256,uint256)` and selector `0x1cb15d26`.
```solidity
error ERC20ExceededSafeSupply(uint256 increasedSupply, uint256 cap);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20ExceededSafeSupply {
        #[allow(missing_docs)]
        pub increasedSupply: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub cap: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC20ExceededSafeSupply> for UnderlyingRustTuple<'_> {
            fn from(value: ERC20ExceededSafeSupply) -> Self {
                (value.increasedSupply, value.cap)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC20ExceededSafeSupply {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    increasedSupply: tuple.0,
                    cap: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20ExceededSafeSupply {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20ExceededSafeSupply(uint256,uint256)";
            const SELECTOR: [u8; 4] = [28u8, 177u8, 93u8, 38u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.increasedSupply),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.cap),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC20InsufficientAllowance(address,uint256,uint256)` and selector `0xfb8f41b2`.
```solidity
error ERC20InsufficientAllowance(address spender, uint256 allowance, uint256 needed);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20InsufficientAllowance {
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub allowance: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub needed: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC20InsufficientAllowance>
        for UnderlyingRustTuple<'_> {
            fn from(value: ERC20InsufficientAllowance) -> Self {
                (value.spender, value.allowance, value.needed)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ERC20InsufficientAllowance {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    spender: tuple.0,
                    allowance: tuple.1,
                    needed: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20InsufficientAllowance {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20InsufficientAllowance(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [251u8, 143u8, 65u8, 178u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.spender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.allowance),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.needed),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC20InsufficientBalance(address,uint256,uint256)` and selector `0xe450d38c`.
```solidity
error ERC20InsufficientBalance(address sender, uint256 balance, uint256 needed);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20InsufficientBalance {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub balance: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub needed: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC20InsufficientBalance>
        for UnderlyingRustTuple<'_> {
            fn from(value: ERC20InsufficientBalance) -> Self {
                (value.sender, value.balance, value.needed)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ERC20InsufficientBalance {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    sender: tuple.0,
                    balance: tuple.1,
                    needed: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20InsufficientBalance {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20InsufficientBalance(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [228u8, 80u8, 211u8, 140u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.balance),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.needed),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC20InvalidApprover(address)` and selector `0xe602df05`.
```solidity
error ERC20InvalidApprover(address approver);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20InvalidApprover {
        #[allow(missing_docs)]
        pub approver: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC20InvalidApprover> for UnderlyingRustTuple<'_> {
            fn from(value: ERC20InvalidApprover) -> Self {
                (value.approver,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC20InvalidApprover {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { approver: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20InvalidApprover {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20InvalidApprover(address)";
            const SELECTOR: [u8; 4] = [230u8, 2u8, 223u8, 5u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.approver,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC20InvalidReceiver(address)` and selector `0xec442f05`.
```solidity
error ERC20InvalidReceiver(address receiver);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20InvalidReceiver {
        #[allow(missing_docs)]
        pub receiver: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC20InvalidReceiver> for UnderlyingRustTuple<'_> {
            fn from(value: ERC20InvalidReceiver) -> Self {
                (value.receiver,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC20InvalidReceiver {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { receiver: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20InvalidReceiver {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20InvalidReceiver(address)";
            const SELECTOR: [u8; 4] = [236u8, 68u8, 47u8, 5u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.receiver,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC20InvalidSender(address)` and selector `0x96c6fd1e`.
```solidity
error ERC20InvalidSender(address sender);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20InvalidSender {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC20InvalidSender> for UnderlyingRustTuple<'_> {
            fn from(value: ERC20InvalidSender) -> Self {
                (value.sender,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC20InvalidSender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { sender: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20InvalidSender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20InvalidSender(address)";
            const SELECTOR: [u8; 4] = [150u8, 198u8, 253u8, 30u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC20InvalidSpender(address)` and selector `0x94280d62`.
```solidity
error ERC20InvalidSpender(address spender);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC20InvalidSpender {
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC20InvalidSpender> for UnderlyingRustTuple<'_> {
            fn from(value: ERC20InvalidSpender) -> Self {
                (value.spender,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC20InvalidSpender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { spender: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC20InvalidSpender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC20InvalidSpender(address)";
            const SELECTOR: [u8; 4] = [148u8, 40u8, 13u8, 98u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.spender,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC2612ExpiredSignature(uint256)` and selector `0x62791302`.
```solidity
error ERC2612ExpiredSignature(uint256 deadline);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC2612ExpiredSignature {
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC2612ExpiredSignature> for UnderlyingRustTuple<'_> {
            fn from(value: ERC2612ExpiredSignature) -> Self {
                (value.deadline,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC2612ExpiredSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { deadline: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC2612ExpiredSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC2612ExpiredSignature(uint256)";
            const SELECTOR: [u8; 4] = [98u8, 121u8, 19u8, 2u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC2612InvalidSigner(address,address)` and selector `0x4b800e46`.
```solidity
error ERC2612InvalidSigner(address signer, address owner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC2612InvalidSigner {
        #[allow(missing_docs)]
        pub signer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC2612InvalidSigner> for UnderlyingRustTuple<'_> {
            fn from(value: ERC2612InvalidSigner) -> Self {
                (value.signer, value.owner)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC2612InvalidSigner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signer: tuple.0,
                    owner: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC2612InvalidSigner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC2612InvalidSigner(address,address)";
            const SELECTOR: [u8; 4] = [75u8, 128u8, 14u8, 70u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signer,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC5805FutureLookup(uint256,uint48)` and selector `0xecd3f81e`.
```solidity
error ERC5805FutureLookup(uint256 timepoint, uint48 clock);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC5805FutureLookup {
        #[allow(missing_docs)]
        pub timepoint: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub clock: alloy::sol_types::private::primitives::aliases::U48,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<48>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U48,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC5805FutureLookup> for UnderlyingRustTuple<'_> {
            fn from(value: ERC5805FutureLookup) -> Self {
                (value.timepoint, value.clock)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC5805FutureLookup {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    timepoint: tuple.0,
                    clock: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC5805FutureLookup {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC5805FutureLookup(uint256,uint48)";
            const SELECTOR: [u8; 4] = [236u8, 211u8, 248u8, 30u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timepoint),
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::tokenize(&self.clock),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC6372InconsistentClock()` and selector `0x6ff07140`.
```solidity
error ERC6372InconsistentClock();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC6372InconsistentClock;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC6372InconsistentClock>
        for UnderlyingRustTuple<'_> {
            fn from(value: ERC6372InconsistentClock) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ERC6372InconsistentClock {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC6372InconsistentClock {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC6372InconsistentClock()";
            const SELECTOR: [u8; 4] = [111u8, 240u8, 113u8, 64u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ExceedsTotalSupply()` and selector `0x177e3fc3`.
```solidity
error ExceedsTotalSupply();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExceedsTotalSupply;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ExceedsTotalSupply> for UnderlyingRustTuple<'_> {
            fn from(value: ExceedsTotalSupply) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExceedsTotalSupply {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExceedsTotalSupply {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExceedsTotalSupply()";
            const SELECTOR: [u8; 4] = [23u8, 126u8, 63u8, 195u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidAccountNonce(address,uint256)` and selector `0x752d88c0`.
```solidity
error InvalidAccountNonce(address account, uint256 currentNonce);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidAccountNonce {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub currentNonce: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidAccountNonce> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidAccountNonce) -> Self {
                (value.account, value.currentNonce)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidAccountNonce {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    account: tuple.0,
                    currentNonce: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidAccountNonce {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidAccountNonce(address,uint256)";
            const SELECTOR: [u8; 4] = [117u8, 45u8, 136u8, 192u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.currentNonce),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidShortString()` and selector `0xb3512b0c`.
```solidity
error InvalidShortString();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidShortString;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidShortString> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidShortString) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidShortString {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidShortString {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidShortString()";
            const SELECTOR: [u8; 4] = [179u8, 81u8, 43u8, 12u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `SafeCastOverflowedUintDowncast(uint8,uint256)` and selector `0x6dfcc650`.
```solidity
error SafeCastOverflowedUintDowncast(uint8 bits, uint256 value);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SafeCastOverflowedUintDowncast {
        #[allow(missing_docs)]
        pub bits: u8,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u8,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SafeCastOverflowedUintDowncast>
        for UnderlyingRustTuple<'_> {
            fn from(value: SafeCastOverflowedUintDowncast) -> Self {
                (value.bits, value.value)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SafeCastOverflowedUintDowncast {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    bits: tuple.0,
                    value: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SafeCastOverflowedUintDowncast {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SafeCastOverflowedUintDowncast(uint8,uint256)";
            const SELECTOR: [u8; 4] = [109u8, 252u8, 198u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.bits),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `StringTooLong(string)` and selector `0x305a27a9`.
```solidity
error StringTooLong(string str);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StringTooLong {
        #[allow(missing_docs)]
        pub str: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StringTooLong> for UnderlyingRustTuple<'_> {
            fn from(value: StringTooLong) -> Self {
                (value.str,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StringTooLong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { str: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StringTooLong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StringTooLong(string)";
            const SELECTOR: [u8; 4] = [48u8, 90u8, 39u8, 169u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.str,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `TransfersLocked()` and selector `0xdb89e3f4`.
```solidity
error TransfersLocked();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TransfersLocked;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TransfersLocked> for UnderlyingRustTuple<'_> {
            fn from(value: TransfersLocked) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TransfersLocked {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TransfersLocked {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TransfersLocked()";
            const SELECTOR: [u8; 4] = [219u8, 137u8, 227u8, 244u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UnlockTimestampInPast()` and selector `0xa5658353`.
```solidity
error UnlockTimestampInPast();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnlockTimestampInPast;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnlockTimestampInPast> for UnderlyingRustTuple<'_> {
            fn from(value: UnlockTimestampInPast) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnlockTimestampInPast {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnlockTimestampInPast {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnlockTimestampInPast()";
            const SELECTOR: [u8; 4] = [165u8, 101u8, 131u8, 83u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UnlockTimestampTooLate()` and selector `0xef69af65`.
```solidity
error UnlockTimestampTooLate();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnlockTimestampTooLate;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnlockTimestampTooLate> for UnderlyingRustTuple<'_> {
            fn from(value: UnlockTimestampTooLate) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnlockTimestampTooLate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnlockTimestampTooLate {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnlockTimestampTooLate()";
            const SELECTOR: [u8; 4] = [239u8, 105u8, 175u8, 101u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `VotesExpiredSignature(uint256)` and selector `0x4683af0e`.
```solidity
error VotesExpiredSignature(uint256 expiry);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VotesExpiredSignature {
        #[allow(missing_docs)]
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<VotesExpiredSignature> for UnderlyingRustTuple<'_> {
            fn from(value: VotesExpiredSignature) -> Self {
                (value.expiry,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for VotesExpiredSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { expiry: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for VotesExpiredSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "VotesExpiredSignature(uint256)";
            const SELECTOR: [u8; 4] = [70u8, 131u8, 175u8, 14u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ZeroAddress()` and selector `0xd92e233d`.
```solidity
error ZeroAddress();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroAddress;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroAddress> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroAddress) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroAddress {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroAddress {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroAddress()";
            const SELECTOR: [u8; 4] = [217u8, 46u8, 35u8, 61u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ZeroAmount()` and selector `0x1f2a2005`.
```solidity
error ZeroAmount();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroAmount;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroAmount> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroAmount) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroAmount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroAmount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroAmount()";
            const SELECTOR: [u8; 4] = [31u8, 42u8, 32u8, 5u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Approval(address,address,uint256)` and selector `0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925`.
```solidity
event Approval(address indexed owner, address indexed spender, uint256 value);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Approval {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Approval {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Approval(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                140u8, 91u8, 225u8, 229u8, 235u8, 236u8, 125u8, 91u8, 209u8, 79u8, 113u8,
                66u8, 125u8, 30u8, 132u8, 243u8, 221u8, 3u8, 20u8, 192u8, 247u8, 178u8,
                41u8, 30u8, 91u8, 32u8, 10u8, 200u8, 199u8, 195u8, 185u8, 37u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    owner: topics.1,
                    spender: topics.2,
                    value: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.owner.clone(), self.spender.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.owner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.spender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Approval {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Approval> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Approval) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `DelegateChanged(address,address,address)` and selector `0x3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f`.
```solidity
event DelegateChanged(address indexed delegator, address indexed fromDelegate, address indexed toDelegate);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DelegateChanged {
        #[allow(missing_docs)]
        pub delegator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub fromDelegate: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub toDelegate: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for DelegateChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "DelegateChanged(address,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                49u8, 52u8, 232u8, 162u8, 230u8, 217u8, 126u8, 146u8, 154u8, 126u8, 84u8,
                1u8, 30u8, 165u8, 72u8, 93u8, 125u8, 25u8, 109u8, 213u8, 240u8, 186u8,
                77u8, 78u8, 249u8, 88u8, 3u8, 232u8, 227u8, 252u8, 37u8, 127u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    delegator: topics.1,
                    fromDelegate: topics.2,
                    toDelegate: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.delegator.clone(),
                    self.fromDelegate.clone(),
                    self.toDelegate.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.delegator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.fromDelegate,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.toDelegate,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DelegateChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DelegateChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DelegateChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `DelegateVotesChanged(address,uint256,uint256)` and selector `0xdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724`.
```solidity
event DelegateVotesChanged(address indexed delegate, uint256 previousVotes, uint256 newVotes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DelegateVotesChanged {
        #[allow(missing_docs)]
        pub delegate: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub previousVotes: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newVotes: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for DelegateVotesChanged {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "DelegateVotesChanged(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                222u8, 194u8, 186u8, 205u8, 210u8, 240u8, 91u8, 89u8, 222u8, 52u8, 218u8,
                155u8, 82u8, 61u8, 255u8, 139u8, 228u8, 46u8, 94u8, 56u8, 232u8, 24u8,
                200u8, 47u8, 219u8, 11u8, 174u8, 119u8, 67u8, 135u8, 167u8, 36u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    delegate: topics.1,
                    previousVotes: data.0,
                    newVotes: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.previousVotes),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newVotes),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.delegate.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.delegate,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DelegateVotesChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DelegateVotesChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DelegateVotesChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `EIP712DomainChanged()` and selector `0x0a6387c9ea3628b88a633bb4f3b151770f70085117a15f9bf3787cda53f13d31`.
```solidity
event EIP712DomainChanged();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EIP712DomainChanged;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for EIP712DomainChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EIP712DomainChanged()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                10u8, 99u8, 135u8, 201u8, 234u8, 54u8, 40u8, 184u8, 138u8, 99u8, 59u8,
                180u8, 243u8, 177u8, 81u8, 119u8, 15u8, 112u8, 8u8, 81u8, 23u8, 161u8,
                95u8, 155u8, 243u8, 120u8, 124u8, 218u8, 83u8, 241u8, 61u8, 49u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EIP712DomainChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EIP712DomainChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EIP712DomainChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RoleAdminChanged(bytes32,bytes32,bytes32)` and selector `0xbd79b86ffe0ab8e8776151514217cd7cacd52c909f66475c3af44e129f0b00ff`.
```solidity
event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleAdminChanged {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub previousAdminRole: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub newAdminRole: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RoleAdminChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "RoleAdminChanged(bytes32,bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                189u8, 121u8, 184u8, 111u8, 254u8, 10u8, 184u8, 232u8, 119u8, 97u8, 81u8,
                81u8, 66u8, 23u8, 205u8, 124u8, 172u8, 213u8, 44u8, 144u8, 159u8, 102u8,
                71u8, 92u8, 58u8, 244u8, 78u8, 18u8, 159u8, 11u8, 0u8, 255u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    previousAdminRole: topics.2,
                    newAdminRole: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.previousAdminRole.clone(),
                    self.newAdminRole.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.previousAdminRole);
                out[3usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.newAdminRole);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleAdminChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleAdminChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleAdminChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RoleGranted(bytes32,address,address)` and selector `0x2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d`.
```solidity
event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleGranted {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RoleGranted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RoleGranted(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8,
                236u8, 121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8,
                64u8, 48u8, 69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    account: topics.2,
                    sender: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.account.clone(),
                    self.sender.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleGranted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleGranted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleGranted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RoleRevoked(bytes32,address,address)` and selector `0xf6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b`.
```solidity
event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RoleRevoked {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RoleRevoked {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RoleRevoked(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8,
                103u8, 11u8, 68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8,
                253u8, 100u8, 235u8, 33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    role: topics.1,
                    account: topics.2,
                    sender: topics.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.account.clone(),
                    self.sender.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleRevoked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleRevoked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleRevoked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TokensBurnedByManager(address,uint256,address)` and selector `0xbef4f81c1814c641ede85ebaacf19d048b2c5b55980adfa6ef0f956c651335a2`.
```solidity
event TokensBurnedByManager(address indexed from, uint256 amount, address indexed burner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TokensBurnedByManager {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub burner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TokensBurnedByManager {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "TokensBurnedByManager(address,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                190u8, 244u8, 248u8, 28u8, 24u8, 20u8, 198u8, 65u8, 237u8, 232u8, 94u8,
                186u8, 172u8, 241u8, 157u8, 4u8, 139u8, 44u8, 91u8, 85u8, 152u8, 10u8,
                223u8, 166u8, 239u8, 15u8, 149u8, 108u8, 101u8, 19u8, 53u8, 162u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    from: topics.1,
                    amount: data.0,
                    burner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.from.clone(), self.burner.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.from,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.burner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TokensBurnedByManager {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TokensBurnedByManager> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TokensBurnedByManager) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Transfer(address,address,uint256)` and selector `0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef`.
```solidity
event Transfer(address indexed from, address indexed to, uint256 value);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Transfer {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Transfer {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Transfer(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                221u8, 242u8, 82u8, 173u8, 27u8, 226u8, 200u8, 155u8, 105u8, 194u8,
                176u8, 104u8, 252u8, 55u8, 141u8, 170u8, 149u8, 43u8, 167u8, 241u8, 99u8,
                196u8, 161u8, 22u8, 40u8, 245u8, 90u8, 77u8, 245u8, 35u8, 179u8, 239u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    from: topics.1,
                    to: topics.2,
                    value: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.from.clone(), self.to.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.from,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Transfer {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Transfer> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Transfer) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UnlockTimestampUpdated(uint256,uint256,address)` and selector `0xdd6896dcf1d4b311cca87dd19bbba2ea9ce2f867c1568878a0438a66a1afeeec`.
```solidity
event UnlockTimestampUpdated(uint256 oldTimestamp, uint256 newTimestamp, address indexed updatedBy);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UnlockTimestampUpdated {
        #[allow(missing_docs)]
        pub oldTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub updatedBy: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for UnlockTimestampUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "UnlockTimestampUpdated(uint256,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                221u8, 104u8, 150u8, 220u8, 241u8, 212u8, 179u8, 17u8, 204u8, 168u8,
                125u8, 209u8, 155u8, 187u8, 162u8, 234u8, 156u8, 226u8, 248u8, 103u8,
                193u8, 86u8, 136u8, 120u8, 160u8, 67u8, 138u8, 102u8, 161u8, 175u8,
                238u8, 236u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldTimestamp: data.0,
                    newTimestamp: data.1,
                    updatedBy: topics.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newTimestamp),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.updatedBy.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.updatedBy,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UnlockTimestampUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UnlockTimestampUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UnlockTimestampUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address defaultAdmin, address syndTreasuryAddress);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub defaultAdmin: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub syndTreasuryAddress: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.defaultAdmin, value.syndTreasuryAddress)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        defaultAdmin: tuple.0,
                        syndTreasuryAddress: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.defaultAdmin,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.syndTreasuryAddress,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `AIRDROP_MANAGER_ROLE()` and selector `0x8a542521`.
```solidity
function AIRDROP_MANAGER_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AIRDROP_MANAGER_ROLECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`AIRDROP_MANAGER_ROLE()`](AIRDROP_MANAGER_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AIRDROP_MANAGER_ROLEReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<AIRDROP_MANAGER_ROLECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: AIRDROP_MANAGER_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for AIRDROP_MANAGER_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<AIRDROP_MANAGER_ROLEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: AIRDROP_MANAGER_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for AIRDROP_MANAGER_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for AIRDROP_MANAGER_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AIRDROP_MANAGER_ROLE()";
            const SELECTOR: [u8; 4] = [138u8, 84u8, 37u8, 33u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: AIRDROP_MANAGER_ROLEReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: AIRDROP_MANAGER_ROLEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `CLOCK_MODE()` and selector `0x4bf5d7e9`.
```solidity
function CLOCK_MODE() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLOCK_MODECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`CLOCK_MODE()`](CLOCK_MODECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLOCK_MODEReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<CLOCK_MODECall> for UnderlyingRustTuple<'_> {
                fn from(value: CLOCK_MODECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CLOCK_MODECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<CLOCK_MODEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: CLOCK_MODEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CLOCK_MODEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for CLOCK_MODECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CLOCK_MODE()";
            const SELECTOR: [u8; 4] = [75u8, 245u8, 215u8, 233u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: CLOCK_MODEReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: CLOCK_MODEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`.
```solidity
function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`DEFAULT_ADMIN_ROLE()`](DEFAULT_ADMIN_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLEReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DEFAULT_ADMIN_ROLECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_ADMIN_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_ADMIN_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DEFAULT_ADMIN_ROLEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_ADMIN_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEFAULT_ADMIN_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DEFAULT_ADMIN_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DEFAULT_ADMIN_ROLE()";
            const SELECTOR: [u8; 4] = [162u8, 23u8, 253u8, 223u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: DEFAULT_ADMIN_ROLEReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: DEFAULT_ADMIN_ROLEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`.
```solidity
function DOMAIN_SEPARATOR() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DOMAIN_SEPARATORCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`DOMAIN_SEPARATOR()`](DOMAIN_SEPARATORCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DOMAIN_SEPARATORReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DOMAIN_SEPARATORCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DOMAIN_SEPARATORCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DOMAIN_SEPARATORCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DOMAIN_SEPARATORReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DOMAIN_SEPARATORReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DOMAIN_SEPARATORReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DOMAIN_SEPARATORCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DOMAIN_SEPARATOR()";
            const SELECTOR: [u8; 4] = [54u8, 68u8, 229u8, 21u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: DOMAIN_SEPARATORReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: DOMAIN_SEPARATORReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `EMISSION_MINTER_ROLE()` and selector `0x8d3343d6`.
```solidity
function EMISSION_MINTER_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EMISSION_MINTER_ROLECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`EMISSION_MINTER_ROLE()`](EMISSION_MINTER_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EMISSION_MINTER_ROLEReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<EMISSION_MINTER_ROLECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: EMISSION_MINTER_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for EMISSION_MINTER_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<EMISSION_MINTER_ROLEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: EMISSION_MINTER_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for EMISSION_MINTER_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for EMISSION_MINTER_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EMISSION_MINTER_ROLE()";
            const SELECTOR: [u8; 4] = [141u8, 51u8, 67u8, 214u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: EMISSION_MINTER_ROLEReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: EMISSION_MINTER_ROLEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `INITIAL_MINT_SUPPLY()` and selector `0x9b7ef64b`.
```solidity
function INITIAL_MINT_SUPPLY() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct INITIAL_MINT_SUPPLYCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`INITIAL_MINT_SUPPLY()`](INITIAL_MINT_SUPPLYCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct INITIAL_MINT_SUPPLYReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<INITIAL_MINT_SUPPLYCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: INITIAL_MINT_SUPPLYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for INITIAL_MINT_SUPPLYCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<INITIAL_MINT_SUPPLYReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: INITIAL_MINT_SUPPLYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for INITIAL_MINT_SUPPLYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for INITIAL_MINT_SUPPLYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "INITIAL_MINT_SUPPLY()";
            const SELECTOR: [u8; 4] = [155u8, 126u8, 246u8, 75u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: INITIAL_MINT_SUPPLYReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: INITIAL_MINT_SUPPLYReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `MAX_LOCK_DURATION()` and selector `0x4f1bfc9e`.
```solidity
function MAX_LOCK_DURATION() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_LOCK_DURATIONCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`MAX_LOCK_DURATION()`](MAX_LOCK_DURATIONCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_LOCK_DURATIONReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<MAX_LOCK_DURATIONCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_LOCK_DURATIONCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_LOCK_DURATIONCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<MAX_LOCK_DURATIONReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_LOCK_DURATIONReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_LOCK_DURATIONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_LOCK_DURATIONCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_LOCK_DURATION()";
            const SELECTOR: [u8; 4] = [79u8, 27u8, 252u8, 158u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: MAX_LOCK_DURATIONReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: MAX_LOCK_DURATIONReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `TOTAL_SUPPLY()` and selector `0x902d55a5`.
```solidity
function TOTAL_SUPPLY() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TOTAL_SUPPLYCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`TOTAL_SUPPLY()`](TOTAL_SUPPLYCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TOTAL_SUPPLYReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<TOTAL_SUPPLYCall> for UnderlyingRustTuple<'_> {
                fn from(value: TOTAL_SUPPLYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TOTAL_SUPPLYCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<TOTAL_SUPPLYReturn> for UnderlyingRustTuple<'_> {
                fn from(value: TOTAL_SUPPLYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TOTAL_SUPPLYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for TOTAL_SUPPLYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TOTAL_SUPPLY()";
            const SELECTOR: [u8; 4] = [144u8, 45u8, 85u8, 165u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: TOTAL_SUPPLYReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: TOTAL_SUPPLYReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `allowance(address,address)` and selector `0xdd62ed3e`.
```solidity
function allowance(address owner, address spender) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allowanceCall {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`allowance(address,address)`](allowanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allowanceReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allowanceCall> for UnderlyingRustTuple<'_> {
                fn from(value: allowanceCall) -> Self {
                    (value.owner, value.spender)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allowanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        owner: tuple.0,
                        spender: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allowanceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allowanceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allowanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allowanceCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allowance(address,address)";
            const SELECTOR: [u8; 4] = [221u8, 98u8, 237u8, 62u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.spender,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: allowanceReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: allowanceReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `approve(address,uint256)` and selector `0x095ea7b3`.
```solidity
function approve(address spender, uint256 value) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approveCall {
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`approve(address,uint256)`](approveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approveReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<approveCall> for UnderlyingRustTuple<'_> {
                fn from(value: approveCall) -> Self {
                    (value.spender, value.value)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        spender: tuple.0,
                        value: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<approveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: approveReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for approveCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "approve(address,uint256)";
            const SELECTOR: [u8; 4] = [9u8, 94u8, 167u8, 179u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.spender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: approveReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: approveReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `balanceOf(address)` and selector `0x70a08231`.
```solidity
function balanceOf(address account) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct balanceOfCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`balanceOf(address)`](balanceOfCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct balanceOfReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<balanceOfCall> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<balanceOfReturn> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for balanceOfCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "balanceOf(address)";
            const SELECTOR: [u8; 4] = [112u8, 160u8, 130u8, 49u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: balanceOfReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: balanceOfReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `burn(uint256)` and selector `0x42966c68`.
```solidity
function burn(uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct burnCall {
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`burn(uint256)`](burnCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct burnReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<burnCall> for UnderlyingRustTuple<'_> {
                fn from(value: burnCall) -> Self {
                    (value.amount,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { amount: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<burnReturn> for UnderlyingRustTuple<'_> {
                fn from(value: burnReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl burnReturn {
            fn _tokenize(
                &self,
            ) -> <burnCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for burnCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = burnReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "burn(uint256)";
            const SELECTOR: [u8; 4] = [66u8, 150u8, 108u8, 104u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                burnReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `burnFrom(address,uint256)` and selector `0x79cc6790`.
```solidity
function burnFrom(address from, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct burnFromCall {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`burnFrom(address,uint256)`](burnFromCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct burnFromReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<burnFromCall> for UnderlyingRustTuple<'_> {
                fn from(value: burnFromCall) -> Self {
                    (value.from, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnFromCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        from: tuple.0,
                        amount: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<burnFromReturn> for UnderlyingRustTuple<'_> {
                fn from(value: burnFromReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnFromReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl burnFromReturn {
            fn _tokenize(
                &self,
            ) -> <burnFromCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for burnFromCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = burnFromReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "burnFrom(address,uint256)";
            const SELECTOR: [u8; 4] = [121u8, 204u8, 103u8, 144u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                burnFromReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `checkpoints(address,uint32)` and selector `0xf1127ed8`.
```solidity
function checkpoints(address account, uint32 pos) external view returns (Checkpoints.Checkpoint208 memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkpointsCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub pos: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`checkpoints(address,uint32)`](checkpointsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkpointsReturn {
        #[allow(missing_docs)]
        pub _0: <Checkpoints::Checkpoint208 as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkpointsCall> for UnderlyingRustTuple<'_> {
                fn from(value: checkpointsCall) -> Self {
                    (value.account, value.pos)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkpointsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        pos: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Checkpoints::Checkpoint208,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Checkpoints::Checkpoint208 as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkpointsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: checkpointsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkpointsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkpointsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Checkpoints::Checkpoint208 as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Checkpoints::Checkpoint208,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkpoints(address,uint32)";
            const SELECTOR: [u8; 4] = [241u8, 18u8, 126u8, 216u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.pos),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <Checkpoints::Checkpoint208 as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: checkpointsReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: checkpointsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `clock()` and selector `0x91ddadf4`.
```solidity
function clock() external view returns (uint48);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clockCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`clock()`](clockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clockReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U48,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<clockCall> for UnderlyingRustTuple<'_> {
                fn from(value: clockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<48>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U48,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<clockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: clockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for clockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U48;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<48>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "clock()";
            const SELECTOR: [u8; 4] = [145u8, 221u8, 173u8, 244u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: clockReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: clockReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `decimals()` and selector `0x313ce567`.
```solidity
function decimals() external view returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decimalsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`decimals()`](decimalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decimalsReturn {
        #[allow(missing_docs)]
        pub _0: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<decimalsCall> for UnderlyingRustTuple<'_> {
                fn from(value: decimalsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for decimalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<decimalsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: decimalsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for decimalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for decimalsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u8;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "decimals()";
            const SELECTOR: [u8; 4] = [49u8, 60u8, 229u8, 103u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: decimalsReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: decimalsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `delegate(address)` and selector `0x5c19a95c`.
```solidity
function delegate(address delegatee) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateCall {
        #[allow(missing_docs)]
        pub delegatee: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`delegate(address)`](delegateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegateCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegateCall) -> Self {
                    (value.delegatee,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { delegatee: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegateReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl delegateReturn {
            fn _tokenize(
                &self,
            ) -> <delegateCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegateReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegate(address)";
            const SELECTOR: [u8; 4] = [92u8, 25u8, 169u8, 92u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delegatee,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                delegateReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xc3cda520`.
```solidity
function delegateBySig(address delegatee, uint256 nonce, uint256 expiry, uint8 v, bytes32 r, bytes32 s) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateBySigCall {
        #[allow(missing_docs)]
        pub delegatee: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub v: u8,
        #[allow(missing_docs)]
        pub r: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub s: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)`](delegateBySigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateBySigReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegateBySigCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegateBySigCall) -> Self {
                    (
                        value.delegatee,
                        value.nonce,
                        value.expiry,
                        value.v,
                        value.r,
                        value.s,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateBySigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        delegatee: tuple.0,
                        nonce: tuple.1,
                        expiry: tuple.2,
                        v: tuple.3,
                        r: tuple.4,
                        s: tuple.5,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegateBySigReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegateBySigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateBySigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl delegateBySigReturn {
            fn _tokenize(
                &self,
            ) -> <delegateBySigCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegateBySigCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegateBySigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [195u8, 205u8, 165u8, 32u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delegatee,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.v),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.r),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                delegateBySigReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `delegates(address)` and selector `0x587cde1e`.
```solidity
function delegates(address account) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatesCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`delegates(address)`](delegatesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegatesCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegatesCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegatesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegatesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegatesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegatesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegatesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegates(address)";
            const SELECTOR: [u8; 4] = [88u8, 124u8, 222u8, 30u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: delegatesReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: delegatesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `eip712Domain()` and selector `0x84b0196e`.
```solidity
function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`eip712Domain()`](eip712DomainCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainReturn {
        #[allow(missing_docs)]
        pub fields: alloy::sol_types::private::FixedBytes<1>,
        #[allow(missing_docs)]
        pub name: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub version: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub verifyingContract: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub extensions: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eip712DomainCall> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<1>,
                alloy::sol_types::private::String,
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eip712DomainReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainReturn) -> Self {
                    (
                        value.fields,
                        value.name,
                        value.version,
                        value.chainId,
                        value.verifyingContract,
                        value.salt,
                        value.extensions,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        fields: tuple.0,
                        name: tuple.1,
                        version: tuple.2,
                        chainId: tuple.3,
                        verifyingContract: tuple.4,
                        salt: tuple.5,
                        extensions: tuple.6,
                    }
                }
            }
        }
        impl eip712DomainReturn {
            fn _tokenize(
                &self,
            ) -> <eip712DomainCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        1,
                    > as alloy_sol_types::SolType>::tokenize(&self.fields),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.name,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.version,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.verifyingContract,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.extensions),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eip712DomainCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eip712DomainReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eip712Domain()";
            const SELECTOR: [u8; 4] = [132u8, 176u8, 25u8, 110u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                eip712DomainReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCurrentTotalSupply()` and selector `0xc02ae754`.
```solidity
function getCurrentTotalSupply() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentTotalSupplyCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCurrentTotalSupply()`](getCurrentTotalSupplyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentTotalSupplyReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentTotalSupplyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentTotalSupplyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentTotalSupplyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentTotalSupplyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentTotalSupplyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentTotalSupplyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentTotalSupplyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentTotalSupply()";
            const SELECTOR: [u8; 4] = [192u8, 42u8, 231u8, 84u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getCurrentTotalSupplyReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getCurrentTotalSupplyReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getPastTotalSupply(uint256)` and selector `0x8e539e8c`.
```solidity
function getPastTotalSupply(uint256 timepoint) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastTotalSupplyCall {
        #[allow(missing_docs)]
        pub timepoint: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getPastTotalSupply(uint256)`](getPastTotalSupplyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastTotalSupplyReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getPastTotalSupplyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPastTotalSupplyCall) -> Self {
                    (value.timepoint,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPastTotalSupplyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { timepoint: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getPastTotalSupplyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPastTotalSupplyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPastTotalSupplyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPastTotalSupplyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPastTotalSupply(uint256)";
            const SELECTOR: [u8; 4] = [142u8, 83u8, 158u8, 140u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timepoint),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getPastTotalSupplyReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getPastTotalSupplyReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getPastVotes(address,uint256)` and selector `0x3a46b1a8`.
```solidity
function getPastVotes(address account, uint256 timepoint) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastVotesCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub timepoint: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getPastVotes(address,uint256)`](getPastVotesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastVotesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getPastVotesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPastVotesCall) -> Self {
                    (value.account, value.timepoint)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPastVotesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        timepoint: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getPastVotesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getPastVotesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPastVotesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPastVotesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPastVotes(address,uint256)";
            const SELECTOR: [u8; 4] = [58u8, 70u8, 177u8, 168u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.timepoint),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getPastVotesReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getPastVotesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getPastVotingPower(address,uint256)` and selector `0xb0ca253e`.
```solidity
function getPastVotingPower(address account, uint256 blockNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastVotingPowerCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub blockNumber: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getPastVotingPower(address,uint256)`](getPastVotingPowerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastVotingPowerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getPastVotingPowerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPastVotingPowerCall) -> Self {
                    (value.account, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPastVotingPowerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        blockNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getPastVotingPowerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPastVotingPowerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPastVotingPowerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPastVotingPowerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPastVotingPower(address,uint256)";
            const SELECTOR: [u8; 4] = [176u8, 202u8, 37u8, 62u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getPastVotingPowerReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getPastVotingPowerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getRemainingLockTime()` and selector `0x7a8cd156`.
```solidity
function getRemainingLockTime() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRemainingLockTimeCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getRemainingLockTime()`](getRemainingLockTimeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRemainingLockTimeReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRemainingLockTimeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRemainingLockTimeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRemainingLockTimeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRemainingLockTimeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRemainingLockTimeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRemainingLockTimeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRemainingLockTimeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRemainingLockTime()";
            const SELECTOR: [u8; 4] = [122u8, 140u8, 209u8, 86u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getRemainingLockTimeReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getRemainingLockTimeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`.
```solidity
function getRoleAdmin(bytes32 role) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleAdminCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getRoleAdmin(bytes32)`](getRoleAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleAdminReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRoleAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleAdminCall) -> Self {
                    (value.role,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { role: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRoleAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRoleAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRoleAdmin(bytes32)";
            const SELECTOR: [u8; 4] = [36u8, 138u8, 156u8, 163u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getRoleAdminReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getRoleAdminReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getVotes(address)` and selector `0x9ab24eb0`.
```solidity
function getVotes(address account) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVotesCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getVotes(address)`](getVotesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVotesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getVotesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getVotesCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVotesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getVotesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getVotesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVotesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getVotesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getVotes(address)";
            const SELECTOR: [u8; 4] = [154u8, 178u8, 78u8, 176u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getVotesReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getVotesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getVotingPower(address)` and selector `0xbb4d4436`.
```solidity
function getVotingPower(address account) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVotingPowerCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getVotingPower(address)`](getVotingPowerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVotingPowerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getVotingPowerCall> for UnderlyingRustTuple<'_> {
                fn from(value: getVotingPowerCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVotingPowerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getVotingPowerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getVotingPowerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getVotingPowerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getVotingPowerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getVotingPower(address)";
            const SELECTOR: [u8; 4] = [187u8, 77u8, 68u8, 54u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getVotingPowerReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getVotingPowerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`.
```solidity
function grantRole(bytes32 role, address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct grantRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`grantRole(bytes32,address)`](grantRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct grantRoleReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<grantRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: grantRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<grantRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: grantRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl grantRoleReturn {
            fn _tokenize(
                &self,
            ) -> <grantRoleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for grantRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = grantRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "grantRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [47u8, 47u8, 241u8, 93u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                grantRoleReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `hasRole(bytes32,address)` and selector `0x91d14854`.
```solidity
function hasRole(bytes32 role, address account) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`hasRole(bytes32,address)`](hasRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRoleReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hasRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: hasRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hasRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hasRoleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hasRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hasRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [145u8, 209u8, 72u8, 84u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: hasRoleReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: hasRoleReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `maxLockTimestamp()` and selector `0x8426adf2`.
```solidity
function maxLockTimestamp() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxLockTimestampCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`maxLockTimestamp()`](maxLockTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxLockTimestampReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<maxLockTimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: maxLockTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for maxLockTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<maxLockTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: maxLockTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for maxLockTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for maxLockTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "maxLockTimestamp()";
            const SELECTOR: [u8; 4] = [132u8, 38u8, 173u8, 242u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: maxLockTimestampReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: maxLockTimestampReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `mint(address,uint256)` and selector `0x40c10f19`.
```solidity
function mint(address to, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mintCall {
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`mint(address,uint256)`](mintCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mintReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<mintCall> for UnderlyingRustTuple<'_> {
                fn from(value: mintCall) -> Self {
                    (value.to, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mintCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        amount: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<mintReturn> for UnderlyingRustTuple<'_> {
                fn from(value: mintReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mintReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl mintReturn {
            fn _tokenize(
                &self,
            ) -> <mintCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mintCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mintReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mint(address,uint256)";
            const SELECTOR: [u8; 4] = [64u8, 193u8, 15u8, 25u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                mintReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `name()` and selector `0x06fdde03`.
```solidity
function name() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nameCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`name()`](nameCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nameReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<nameCall> for UnderlyingRustTuple<'_> {
                fn from(value: nameCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nameCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<nameReturn> for UnderlyingRustTuple<'_> {
                fn from(value: nameReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nameReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nameCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "name()";
            const SELECTOR: [u8; 4] = [6u8, 253u8, 222u8, 3u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: nameReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: nameReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `nonces(address)` and selector `0x7ecebe00`.
```solidity
function nonces(address owner) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct noncesCall {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`nonces(address)`](noncesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct noncesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<noncesCall> for UnderlyingRustTuple<'_> {
                fn from(value: noncesCall) -> Self {
                    (value.owner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for noncesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { owner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<noncesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: noncesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for noncesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for noncesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nonces(address)";
            const SELECTOR: [u8; 4] = [126u8, 206u8, 190u8, 0u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: noncesReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: noncesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `numCheckpoints(address)` and selector `0x6fcfff45`.
```solidity
function numCheckpoints(address account) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct numCheckpointsCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`numCheckpoints(address)`](numCheckpointsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct numCheckpointsReturn {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<numCheckpointsCall> for UnderlyingRustTuple<'_> {
                fn from(value: numCheckpointsCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for numCheckpointsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<numCheckpointsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: numCheckpointsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for numCheckpointsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for numCheckpointsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "numCheckpoints(address)";
            const SELECTOR: [u8; 4] = [111u8, 207u8, 255u8, 69u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: numCheckpointsReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: numCheckpointsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`.
```solidity
function permit(address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permitCall {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub v: u8,
        #[allow(missing_docs)]
        pub r: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub s: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`permit(address,address,uint256,uint256,uint8,bytes32,bytes32)`](permitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permitReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<permitCall> for UnderlyingRustTuple<'_> {
                fn from(value: permitCall) -> Self {
                    (
                        value.owner,
                        value.spender,
                        value.value,
                        value.deadline,
                        value.v,
                        value.r,
                        value.s,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for permitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        owner: tuple.0,
                        spender: tuple.1,
                        value: tuple.2,
                        deadline: tuple.3,
                        v: tuple.4,
                        r: tuple.5,
                        s: tuple.6,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<permitReturn> for UnderlyingRustTuple<'_> {
                fn from(value: permitReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for permitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl permitReturn {
            fn _tokenize(
                &self,
            ) -> <permitCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for permitCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = permitReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [213u8, 5u8, 172u8, 207u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.spender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.v),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.r),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                permitReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`.
```solidity
function renounceRole(bytes32 role, address callerConfirmation) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub callerConfirmation: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`renounceRole(bytes32,address)`](renounceRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceRoleReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceRoleCall) -> Self {
                    (value.role, value.callerConfirmation)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        callerConfirmation: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl renounceRoleReturn {
            fn _tokenize(
                &self,
            ) -> <renounceRoleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [54u8, 86u8, 138u8, 190u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.callerConfirmation,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                renounceRoleReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`.
```solidity
function revokeRole(bytes32 role, address account) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeRoleCall {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`revokeRole(bytes32,address)`](revokeRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeRoleReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<revokeRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: revokeRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        role: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<revokeRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: revokeRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl revokeRoleReturn {
            fn _tokenize(
                &self,
            ) -> <revokeRoleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for revokeRoleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = revokeRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "revokeRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [213u8, 71u8, 116u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                revokeRoleReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setUnlockTimestamp(uint256)` and selector `0x844c9026`.
```solidity
function setUnlockTimestamp(uint256 newUnlockTimestamp) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUnlockTimestampCall {
        #[allow(missing_docs)]
        pub newUnlockTimestamp: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setUnlockTimestamp(uint256)`](setUnlockTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUnlockTimestampReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setUnlockTimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setUnlockTimestampCall) -> Self {
                    (value.newUnlockTimestamp,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setUnlockTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newUnlockTimestamp: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setUnlockTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setUnlockTimestampReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setUnlockTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setUnlockTimestampReturn {
            fn _tokenize(
                &self,
            ) -> <setUnlockTimestampCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setUnlockTimestampCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setUnlockTimestampReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setUnlockTimestamp(uint256)";
            const SELECTOR: [u8; 4] = [132u8, 76u8, 144u8, 38u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newUnlockTimestamp),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setUnlockTimestampReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`.
```solidity
function supportsInterface(bytes4 interfaceId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceCall {
        #[allow(missing_docs)]
        pub interfaceId: alloy::sol_types::private::FixedBytes<4>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`supportsInterface(bytes4)`](supportsInterfaceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsInterfaceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceCall) -> Self {
                    (value.interfaceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { interfaceId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsInterfaceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supportsInterfaceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "supportsInterface(bytes4)";
            const SELECTOR: [u8; 4] = [1u8, 255u8, 201u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.interfaceId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: supportsInterfaceReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: supportsInterfaceReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `symbol()` and selector `0x95d89b41`.
```solidity
function symbol() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct symbolCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`symbol()`](symbolCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct symbolReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<symbolCall> for UnderlyingRustTuple<'_> {
                fn from(value: symbolCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for symbolCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<symbolReturn> for UnderlyingRustTuple<'_> {
                fn from(value: symbolReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for symbolReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for symbolCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "symbol()";
            const SELECTOR: [u8; 4] = [149u8, 216u8, 155u8, 65u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: symbolReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: symbolReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `totalSupply()` and selector `0x18160ddd`.
```solidity
function totalSupply() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalSupplyCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`totalSupply()`](totalSupplyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalSupplyReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<totalSupplyCall> for UnderlyingRustTuple<'_> {
                fn from(value: totalSupplyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalSupplyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<totalSupplyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: totalSupplyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalSupplyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for totalSupplyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "totalSupply()";
            const SELECTOR: [u8; 4] = [24u8, 22u8, 13u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: totalSupplyReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: totalSupplyReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `transfer(address,uint256)` and selector `0xa9059cbb`.
```solidity
function transfer(address to, uint256 value) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferCall {
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`transfer(address,uint256)`](transferCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferCall) -> Self {
                    (value.to, value.value)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        value: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transfer(address,uint256)";
            const SELECTOR: [u8; 4] = [169u8, 5u8, 156u8, 187u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: transferReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: transferReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`.
```solidity
function transferFrom(address from, address to, uint256 value) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferFromCall {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`transferFrom(address,address,uint256)`](transferFromCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferFromReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferFromCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferFromCall) -> Self {
                    (value.from, value.to, value.value)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferFromCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        from: tuple.0,
                        to: tuple.1,
                        value: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferFromReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferFromReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferFromReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferFromCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferFrom(address,address,uint256)";
            const SELECTOR: [u8; 4] = [35u8, 184u8, 114u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: transferFromReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: transferFromReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `transfersLocked()` and selector `0x83f1211b`.
```solidity
function transfersLocked() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transfersLockedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`transfersLocked()`](transfersLockedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transfersLockedReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transfersLockedCall> for UnderlyingRustTuple<'_> {
                fn from(value: transfersLockedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transfersLockedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transfersLockedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transfersLockedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transfersLockedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transfersLockedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transfersLocked()";
            const SELECTOR: [u8; 4] = [131u8, 241u8, 33u8, 27u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: transfersLockedReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: transfersLockedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `unlockTimestamp()` and selector `0xaa082a9d`.
```solidity
function unlockTimestamp() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unlockTimestampCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`unlockTimestamp()`](unlockTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unlockTimestampReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unlockTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: unlockTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unlockTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unlockTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: unlockTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for unlockTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unlockTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unlockTimestamp()";
            const SELECTOR: [u8; 4] = [170u8, 8u8, 42u8, 157u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: unlockTimestampReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: unlockTimestampReturn = r.into();
                        r._0
                    })
            }
        }
    };
    ///Container for all the [`SyndicateToken`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum SyndicateTokenCalls {
        #[allow(missing_docs)]
        AIRDROP_MANAGER_ROLE(AIRDROP_MANAGER_ROLECall),
        #[allow(missing_docs)]
        CLOCK_MODE(CLOCK_MODECall),
        #[allow(missing_docs)]
        DEFAULT_ADMIN_ROLE(DEFAULT_ADMIN_ROLECall),
        #[allow(missing_docs)]
        DOMAIN_SEPARATOR(DOMAIN_SEPARATORCall),
        #[allow(missing_docs)]
        EMISSION_MINTER_ROLE(EMISSION_MINTER_ROLECall),
        #[allow(missing_docs)]
        INITIAL_MINT_SUPPLY(INITIAL_MINT_SUPPLYCall),
        #[allow(missing_docs)]
        MAX_LOCK_DURATION(MAX_LOCK_DURATIONCall),
        #[allow(missing_docs)]
        TOTAL_SUPPLY(TOTAL_SUPPLYCall),
        #[allow(missing_docs)]
        allowance(allowanceCall),
        #[allow(missing_docs)]
        approve(approveCall),
        #[allow(missing_docs)]
        balanceOf(balanceOfCall),
        #[allow(missing_docs)]
        burn(burnCall),
        #[allow(missing_docs)]
        burnFrom(burnFromCall),
        #[allow(missing_docs)]
        checkpoints(checkpointsCall),
        #[allow(missing_docs)]
        clock(clockCall),
        #[allow(missing_docs)]
        decimals(decimalsCall),
        #[allow(missing_docs)]
        delegate(delegateCall),
        #[allow(missing_docs)]
        delegateBySig(delegateBySigCall),
        #[allow(missing_docs)]
        delegates(delegatesCall),
        #[allow(missing_docs)]
        eip712Domain(eip712DomainCall),
        #[allow(missing_docs)]
        getCurrentTotalSupply(getCurrentTotalSupplyCall),
        #[allow(missing_docs)]
        getPastTotalSupply(getPastTotalSupplyCall),
        #[allow(missing_docs)]
        getPastVotes(getPastVotesCall),
        #[allow(missing_docs)]
        getPastVotingPower(getPastVotingPowerCall),
        #[allow(missing_docs)]
        getRemainingLockTime(getRemainingLockTimeCall),
        #[allow(missing_docs)]
        getRoleAdmin(getRoleAdminCall),
        #[allow(missing_docs)]
        getVotes(getVotesCall),
        #[allow(missing_docs)]
        getVotingPower(getVotingPowerCall),
        #[allow(missing_docs)]
        grantRole(grantRoleCall),
        #[allow(missing_docs)]
        hasRole(hasRoleCall),
        #[allow(missing_docs)]
        maxLockTimestamp(maxLockTimestampCall),
        #[allow(missing_docs)]
        mint(mintCall),
        #[allow(missing_docs)]
        name(nameCall),
        #[allow(missing_docs)]
        nonces(noncesCall),
        #[allow(missing_docs)]
        numCheckpoints(numCheckpointsCall),
        #[allow(missing_docs)]
        permit(permitCall),
        #[allow(missing_docs)]
        renounceRole(renounceRoleCall),
        #[allow(missing_docs)]
        revokeRole(revokeRoleCall),
        #[allow(missing_docs)]
        setUnlockTimestamp(setUnlockTimestampCall),
        #[allow(missing_docs)]
        supportsInterface(supportsInterfaceCall),
        #[allow(missing_docs)]
        symbol(symbolCall),
        #[allow(missing_docs)]
        totalSupply(totalSupplyCall),
        #[allow(missing_docs)]
        transfer(transferCall),
        #[allow(missing_docs)]
        transferFrom(transferFromCall),
        #[allow(missing_docs)]
        transfersLocked(transfersLockedCall),
        #[allow(missing_docs)]
        unlockTimestamp(unlockTimestampCall),
    }
    #[automatically_derived]
    impl SyndicateTokenCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 255u8, 201u8, 167u8],
            [6u8, 253u8, 222u8, 3u8],
            [9u8, 94u8, 167u8, 179u8],
            [24u8, 22u8, 13u8, 221u8],
            [35u8, 184u8, 114u8, 221u8],
            [36u8, 138u8, 156u8, 163u8],
            [47u8, 47u8, 241u8, 93u8],
            [49u8, 60u8, 229u8, 103u8],
            [54u8, 68u8, 229u8, 21u8],
            [54u8, 86u8, 138u8, 190u8],
            [58u8, 70u8, 177u8, 168u8],
            [64u8, 193u8, 15u8, 25u8],
            [66u8, 150u8, 108u8, 104u8],
            [75u8, 245u8, 215u8, 233u8],
            [79u8, 27u8, 252u8, 158u8],
            [88u8, 124u8, 222u8, 30u8],
            [92u8, 25u8, 169u8, 92u8],
            [111u8, 207u8, 255u8, 69u8],
            [112u8, 160u8, 130u8, 49u8],
            [121u8, 204u8, 103u8, 144u8],
            [122u8, 140u8, 209u8, 86u8],
            [126u8, 206u8, 190u8, 0u8],
            [131u8, 241u8, 33u8, 27u8],
            [132u8, 38u8, 173u8, 242u8],
            [132u8, 76u8, 144u8, 38u8],
            [132u8, 176u8, 25u8, 110u8],
            [138u8, 84u8, 37u8, 33u8],
            [141u8, 51u8, 67u8, 214u8],
            [142u8, 83u8, 158u8, 140u8],
            [144u8, 45u8, 85u8, 165u8],
            [145u8, 209u8, 72u8, 84u8],
            [145u8, 221u8, 173u8, 244u8],
            [149u8, 216u8, 155u8, 65u8],
            [154u8, 178u8, 78u8, 176u8],
            [155u8, 126u8, 246u8, 75u8],
            [162u8, 23u8, 253u8, 223u8],
            [169u8, 5u8, 156u8, 187u8],
            [170u8, 8u8, 42u8, 157u8],
            [176u8, 202u8, 37u8, 62u8],
            [187u8, 77u8, 68u8, 54u8],
            [192u8, 42u8, 231u8, 84u8],
            [195u8, 205u8, 165u8, 32u8],
            [213u8, 5u8, 172u8, 207u8],
            [213u8, 71u8, 116u8, 31u8],
            [221u8, 98u8, 237u8, 62u8],
            [241u8, 18u8, 126u8, 216u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SyndicateTokenCalls {
        const NAME: &'static str = "SyndicateTokenCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 46usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AIRDROP_MANAGER_ROLE(_) => {
                    <AIRDROP_MANAGER_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::CLOCK_MODE(_) => {
                    <CLOCK_MODECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DEFAULT_ADMIN_ROLE(_) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DOMAIN_SEPARATOR(_) => {
                    <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::EMISSION_MINTER_ROLE(_) => {
                    <EMISSION_MINTER_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::INITIAL_MINT_SUPPLY(_) => {
                    <INITIAL_MINT_SUPPLYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MAX_LOCK_DURATION(_) => {
                    <MAX_LOCK_DURATIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::TOTAL_SUPPLY(_) => {
                    <TOTAL_SUPPLYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allowance(_) => {
                    <allowanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::approve(_) => <approveCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::balanceOf(_) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::burn(_) => <burnCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::burnFrom(_) => <burnFromCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::checkpoints(_) => {
                    <checkpointsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::clock(_) => <clockCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::decimals(_) => <decimalsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::delegate(_) => <delegateCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::delegateBySig(_) => {
                    <delegateBySigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegates(_) => {
                    <delegatesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentTotalSupply(_) => {
                    <getCurrentTotalSupplyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPastTotalSupply(_) => {
                    <getPastTotalSupplyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPastVotes(_) => {
                    <getPastVotesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPastVotingPower(_) => {
                    <getPastVotingPowerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRemainingLockTime(_) => {
                    <getRemainingLockTimeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRoleAdmin(_) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getVotes(_) => <getVotesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getVotingPower(_) => {
                    <getVotingPowerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::grantRole(_) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hasRole(_) => <hasRoleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::maxLockTimestamp(_) => {
                    <maxLockTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mint(_) => <mintCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::name(_) => <nameCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::nonces(_) => <noncesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::numCheckpoints(_) => {
                    <numCheckpointsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::permit(_) => <permitCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::renounceRole(_) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::revokeRole(_) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setUnlockTimestamp(_) => {
                    <setUnlockTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::symbol(_) => <symbolCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::totalSupply(_) => {
                    <totalSupplyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transfer(_) => <transferCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::transferFrom(_) => {
                    <transferFromCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transfersLocked(_) => {
                    <transfersLockedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unlockTimestamp(_) => {
                    <unlockTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<SyndicateTokenCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn name(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <nameCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateTokenCalls::name)
                    }
                    name
                },
                {
                    fn approve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <approveCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateTokenCalls::approve)
                    }
                    approve
                },
                {
                    fn totalSupply(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <totalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::totalSupply)
                    }
                    totalSupply
                },
                {
                    fn transferFrom(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <transferFromCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::transferFrom)
                    }
                    transferFrom
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn grantRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateTokenCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn decimals(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <decimalsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateTokenCalls::decimals)
                    }
                    decimals
                },
                {
                    fn DOMAIN_SEPARATOR(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::DOMAIN_SEPARATOR)
                    }
                    DOMAIN_SEPARATOR
                },
                {
                    fn renounceRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn getPastVotes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getPastVotesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::getPastVotes)
                    }
                    getPastVotes
                },
                {
                    fn mint(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <mintCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateTokenCalls::mint)
                    }
                    mint
                },
                {
                    fn burn(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <burnCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateTokenCalls::burn)
                    }
                    burn
                },
                {
                    fn CLOCK_MODE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::CLOCK_MODE)
                    }
                    CLOCK_MODE
                },
                {
                    fn MAX_LOCK_DURATION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <MAX_LOCK_DURATIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::MAX_LOCK_DURATION)
                    }
                    MAX_LOCK_DURATION
                },
                {
                    fn delegates(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <delegatesCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateTokenCalls::delegates)
                    }
                    delegates
                },
                {
                    fn delegate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <delegateCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateTokenCalls::delegate)
                    }
                    delegate
                },
                {
                    fn numCheckpoints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <numCheckpointsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::numCheckpoints)
                    }
                    numCheckpoints
                },
                {
                    fn balanceOf(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <balanceOfCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateTokenCalls::balanceOf)
                    }
                    balanceOf
                },
                {
                    fn burnFrom(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <burnFromCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateTokenCalls::burnFrom)
                    }
                    burnFrom
                },
                {
                    fn getRemainingLockTime(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getRemainingLockTimeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::getRemainingLockTime)
                    }
                    getRemainingLockTime
                },
                {
                    fn nonces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <noncesCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateTokenCalls::nonces)
                    }
                    nonces
                },
                {
                    fn transfersLocked(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <transfersLockedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::transfersLocked)
                    }
                    transfersLocked
                },
                {
                    fn maxLockTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <maxLockTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::maxLockTimestamp)
                    }
                    maxLockTimestamp
                },
                {
                    fn setUnlockTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <setUnlockTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::setUnlockTimestamp)
                    }
                    setUnlockTimestamp
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn AIRDROP_MANAGER_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <AIRDROP_MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::AIRDROP_MANAGER_ROLE)
                    }
                    AIRDROP_MANAGER_ROLE
                },
                {
                    fn EMISSION_MINTER_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <EMISSION_MINTER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::EMISSION_MINTER_ROLE)
                    }
                    EMISSION_MINTER_ROLE
                },
                {
                    fn getPastTotalSupply(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getPastTotalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::getPastTotalSupply)
                    }
                    getPastTotalSupply
                },
                {
                    fn TOTAL_SUPPLY(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <TOTAL_SUPPLYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::TOTAL_SUPPLY)
                    }
                    TOTAL_SUPPLY
                },
                {
                    fn hasRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateTokenCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn clock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <clockCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateTokenCalls::clock)
                    }
                    clock
                },
                {
                    fn symbol(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <symbolCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateTokenCalls::symbol)
                    }
                    symbol
                },
                {
                    fn getVotes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getVotesCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateTokenCalls::getVotes)
                    }
                    getVotes
                },
                {
                    fn INITIAL_MINT_SUPPLY(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <INITIAL_MINT_SUPPLYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::INITIAL_MINT_SUPPLY)
                    }
                    INITIAL_MINT_SUPPLY
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn transfer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <transferCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateTokenCalls::transfer)
                    }
                    transfer
                },
                {
                    fn unlockTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <unlockTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::unlockTimestamp)
                    }
                    unlockTimestamp
                },
                {
                    fn getPastVotingPower(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getPastVotingPowerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::getPastVotingPower)
                    }
                    getPastVotingPower
                },
                {
                    fn getVotingPower(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getVotingPowerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::getVotingPower)
                    }
                    getVotingPower
                },
                {
                    fn getCurrentTotalSupply(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getCurrentTotalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::getCurrentTotalSupply)
                    }
                    getCurrentTotalSupply
                },
                {
                    fn delegateBySig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <delegateBySigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::delegateBySig)
                    }
                    delegateBySig
                },
                {
                    fn permit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <permitCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateTokenCalls::permit)
                    }
                    permit
                },
                {
                    fn revokeRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn allowance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <allowanceCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(SyndicateTokenCalls::allowance)
                    }
                    allowance
                },
                {
                    fn checkpoints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <checkpointsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::checkpoints)
                    }
                    checkpoints
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<SyndicateTokenCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn name(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <nameCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::name)
                    }
                    name
                },
                {
                    fn approve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <approveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::approve)
                    }
                    approve
                },
                {
                    fn totalSupply(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <totalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::totalSupply)
                    }
                    totalSupply
                },
                {
                    fn transferFrom(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <transferFromCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::transferFrom)
                    }
                    transferFrom
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn grantRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn decimals(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <decimalsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::decimals)
                    }
                    decimals
                },
                {
                    fn DOMAIN_SEPARATOR(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::DOMAIN_SEPARATOR)
                    }
                    DOMAIN_SEPARATOR
                },
                {
                    fn renounceRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn getPastVotes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getPastVotesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::getPastVotes)
                    }
                    getPastVotes
                },
                {
                    fn mint(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <mintCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::mint)
                    }
                    mint
                },
                {
                    fn burn(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <burnCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::burn)
                    }
                    burn
                },
                {
                    fn CLOCK_MODE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::CLOCK_MODE)
                    }
                    CLOCK_MODE
                },
                {
                    fn MAX_LOCK_DURATION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <MAX_LOCK_DURATIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::MAX_LOCK_DURATION)
                    }
                    MAX_LOCK_DURATION
                },
                {
                    fn delegates(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <delegatesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::delegates)
                    }
                    delegates
                },
                {
                    fn delegate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <delegateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::delegate)
                    }
                    delegate
                },
                {
                    fn numCheckpoints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <numCheckpointsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::numCheckpoints)
                    }
                    numCheckpoints
                },
                {
                    fn balanceOf(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <balanceOfCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::balanceOf)
                    }
                    balanceOf
                },
                {
                    fn burnFrom(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <burnFromCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::burnFrom)
                    }
                    burnFrom
                },
                {
                    fn getRemainingLockTime(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getRemainingLockTimeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::getRemainingLockTime)
                    }
                    getRemainingLockTime
                },
                {
                    fn nonces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <noncesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::nonces)
                    }
                    nonces
                },
                {
                    fn transfersLocked(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <transfersLockedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::transfersLocked)
                    }
                    transfersLocked
                },
                {
                    fn maxLockTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <maxLockTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::maxLockTimestamp)
                    }
                    maxLockTimestamp
                },
                {
                    fn setUnlockTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <setUnlockTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::setUnlockTimestamp)
                    }
                    setUnlockTimestamp
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn AIRDROP_MANAGER_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <AIRDROP_MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::AIRDROP_MANAGER_ROLE)
                    }
                    AIRDROP_MANAGER_ROLE
                },
                {
                    fn EMISSION_MINTER_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <EMISSION_MINTER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::EMISSION_MINTER_ROLE)
                    }
                    EMISSION_MINTER_ROLE
                },
                {
                    fn getPastTotalSupply(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getPastTotalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::getPastTotalSupply)
                    }
                    getPastTotalSupply
                },
                {
                    fn TOTAL_SUPPLY(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <TOTAL_SUPPLYCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::TOTAL_SUPPLY)
                    }
                    TOTAL_SUPPLY
                },
                {
                    fn hasRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn clock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <clockCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::clock)
                    }
                    clock
                },
                {
                    fn symbol(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <symbolCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::symbol)
                    }
                    symbol
                },
                {
                    fn getVotes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getVotesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::getVotes)
                    }
                    getVotes
                },
                {
                    fn INITIAL_MINT_SUPPLY(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <INITIAL_MINT_SUPPLYCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::INITIAL_MINT_SUPPLY)
                    }
                    INITIAL_MINT_SUPPLY
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn transfer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <transferCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::transfer)
                    }
                    transfer
                },
                {
                    fn unlockTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <unlockTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::unlockTimestamp)
                    }
                    unlockTimestamp
                },
                {
                    fn getPastVotingPower(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getPastVotingPowerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::getPastVotingPower)
                    }
                    getPastVotingPower
                },
                {
                    fn getVotingPower(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getVotingPowerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::getVotingPower)
                    }
                    getVotingPower
                },
                {
                    fn getCurrentTotalSupply(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getCurrentTotalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::getCurrentTotalSupply)
                    }
                    getCurrentTotalSupply
                },
                {
                    fn delegateBySig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <delegateBySigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::delegateBySig)
                    }
                    delegateBySig
                },
                {
                    fn permit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <permitCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::permit)
                    }
                    permit
                },
                {
                    fn revokeRole(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn allowance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <allowanceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::allowance)
                    }
                    allowance
                },
                {
                    fn checkpoints(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <checkpointsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::checkpoints)
                    }
                    checkpoints
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AIRDROP_MANAGER_ROLE(inner) => {
                    <AIRDROP_MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CLOCK_MODE(inner) => {
                    <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DOMAIN_SEPARATOR(inner) => {
                    <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EMISSION_MINTER_ROLE(inner) => {
                    <EMISSION_MINTER_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::INITIAL_MINT_SUPPLY(inner) => {
                    <INITIAL_MINT_SUPPLYCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MAX_LOCK_DURATION(inner) => {
                    <MAX_LOCK_DURATIONCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TOTAL_SUPPLY(inner) => {
                    <TOTAL_SUPPLYCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allowance(inner) => {
                    <allowanceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::approve(inner) => {
                    <approveCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::burn(inner) => {
                    <burnCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::burnFrom(inner) => {
                    <burnFromCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::checkpoints(inner) => {
                    <checkpointsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::clock(inner) => {
                    <clockCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::decimals(inner) => {
                    <decimalsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::delegate(inner) => {
                    <delegateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::delegateBySig(inner) => {
                    <delegateBySigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegates(inner) => {
                    <delegatesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentTotalSupply(inner) => {
                    <getCurrentTotalSupplyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPastTotalSupply(inner) => {
                    <getPastTotalSupplyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPastVotes(inner) => {
                    <getPastVotesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPastVotingPower(inner) => {
                    <getPastVotingPowerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRemainingLockTime(inner) => {
                    <getRemainingLockTimeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getVotes(inner) => {
                    <getVotesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getVotingPower(inner) => {
                    <getVotingPowerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::maxLockTimestamp(inner) => {
                    <maxLockTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::mint(inner) => {
                    <mintCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::name(inner) => {
                    <nameCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::nonces(inner) => {
                    <noncesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::numCheckpoints(inner) => {
                    <numCheckpointsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::permit(inner) => {
                    <permitCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setUnlockTimestamp(inner) => {
                    <setUnlockTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::symbol(inner) => {
                    <symbolCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::totalSupply(inner) => {
                    <totalSupplyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transfer(inner) => {
                    <transferCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::transferFrom(inner) => {
                    <transferFromCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transfersLocked(inner) => {
                    <transfersLockedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unlockTimestamp(inner) => {
                    <unlockTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AIRDROP_MANAGER_ROLE(inner) => {
                    <AIRDROP_MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CLOCK_MODE(inner) => {
                    <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DOMAIN_SEPARATOR(inner) => {
                    <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EMISSION_MINTER_ROLE(inner) => {
                    <EMISSION_MINTER_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::INITIAL_MINT_SUPPLY(inner) => {
                    <INITIAL_MINT_SUPPLYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MAX_LOCK_DURATION(inner) => {
                    <MAX_LOCK_DURATIONCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TOTAL_SUPPLY(inner) => {
                    <TOTAL_SUPPLYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allowance(inner) => {
                    <allowanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::approve(inner) => {
                    <approveCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::burn(inner) => {
                    <burnCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::burnFrom(inner) => {
                    <burnFromCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkpoints(inner) => {
                    <checkpointsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::clock(inner) => {
                    <clockCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::decimals(inner) => {
                    <decimalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegate(inner) => {
                    <delegateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegateBySig(inner) => {
                    <delegateBySigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegates(inner) => {
                    <delegatesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentTotalSupply(inner) => {
                    <getCurrentTotalSupplyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPastTotalSupply(inner) => {
                    <getPastTotalSupplyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPastVotes(inner) => {
                    <getPastVotesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPastVotingPower(inner) => {
                    <getPastVotingPowerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRemainingLockTime(inner) => {
                    <getRemainingLockTimeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getVotes(inner) => {
                    <getVotesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getVotingPower(inner) => {
                    <getVotingPowerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::maxLockTimestamp(inner) => {
                    <maxLockTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::mint(inner) => {
                    <mintCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::name(inner) => {
                    <nameCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::nonces(inner) => {
                    <noncesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::numCheckpoints(inner) => {
                    <numCheckpointsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::permit(inner) => {
                    <permitCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setUnlockTimestamp(inner) => {
                    <setUnlockTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::symbol(inner) => {
                    <symbolCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::totalSupply(inner) => {
                    <totalSupplyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transfer(inner) => {
                    <transferCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferFrom(inner) => {
                    <transferFromCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transfersLocked(inner) => {
                    <transfersLockedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unlockTimestamp(inner) => {
                    <unlockTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SyndicateToken`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum SyndicateTokenErrors {
        #[allow(missing_docs)]
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        #[allow(missing_docs)]
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        #[allow(missing_docs)]
        BurnOnlyDuringLockPeriod(BurnOnlyDuringLockPeriod),
        #[allow(missing_docs)]
        CheckpointUnorderedInsertion(CheckpointUnorderedInsertion),
        #[allow(missing_docs)]
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        #[allow(missing_docs)]
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        #[allow(missing_docs)]
        ERC20ExceededSafeSupply(ERC20ExceededSafeSupply),
        #[allow(missing_docs)]
        ERC20InsufficientAllowance(ERC20InsufficientAllowance),
        #[allow(missing_docs)]
        ERC20InsufficientBalance(ERC20InsufficientBalance),
        #[allow(missing_docs)]
        ERC20InvalidApprover(ERC20InvalidApprover),
        #[allow(missing_docs)]
        ERC20InvalidReceiver(ERC20InvalidReceiver),
        #[allow(missing_docs)]
        ERC20InvalidSender(ERC20InvalidSender),
        #[allow(missing_docs)]
        ERC20InvalidSpender(ERC20InvalidSpender),
        #[allow(missing_docs)]
        ERC2612ExpiredSignature(ERC2612ExpiredSignature),
        #[allow(missing_docs)]
        ERC2612InvalidSigner(ERC2612InvalidSigner),
        #[allow(missing_docs)]
        ERC5805FutureLookup(ERC5805FutureLookup),
        #[allow(missing_docs)]
        ERC6372InconsistentClock(ERC6372InconsistentClock),
        #[allow(missing_docs)]
        ExceedsTotalSupply(ExceedsTotalSupply),
        #[allow(missing_docs)]
        InvalidAccountNonce(InvalidAccountNonce),
        #[allow(missing_docs)]
        InvalidShortString(InvalidShortString),
        #[allow(missing_docs)]
        SafeCastOverflowedUintDowncast(SafeCastOverflowedUintDowncast),
        #[allow(missing_docs)]
        StringTooLong(StringTooLong),
        #[allow(missing_docs)]
        TransfersLocked(TransfersLocked),
        #[allow(missing_docs)]
        UnlockTimestampInPast(UnlockTimestampInPast),
        #[allow(missing_docs)]
        UnlockTimestampTooLate(UnlockTimestampTooLate),
        #[allow(missing_docs)]
        VotesExpiredSignature(VotesExpiredSignature),
        #[allow(missing_docs)]
        ZeroAddress(ZeroAddress),
        #[allow(missing_docs)]
        ZeroAmount(ZeroAmount),
    }
    #[automatically_derived]
    impl SyndicateTokenErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [23u8, 126u8, 63u8, 195u8],
            [28u8, 177u8, 93u8, 38u8],
            [31u8, 42u8, 32u8, 5u8],
            [37u8, 32u8, 96u8, 29u8],
            [48u8, 90u8, 39u8, 169u8],
            [70u8, 131u8, 175u8, 14u8],
            [75u8, 128u8, 14u8, 70u8],
            [98u8, 121u8, 19u8, 2u8],
            [102u8, 151u8, 178u8, 50u8],
            [109u8, 252u8, 198u8, 80u8],
            [111u8, 240u8, 113u8, 64u8],
            [117u8, 45u8, 136u8, 192u8],
            [148u8, 40u8, 13u8, 98u8],
            [150u8, 198u8, 253u8, 30u8],
            [165u8, 101u8, 131u8, 83u8],
            [179u8, 81u8, 43u8, 12u8],
            [184u8, 181u8, 202u8, 45u8],
            [215u8, 139u8, 206u8, 12u8],
            [217u8, 46u8, 35u8, 61u8],
            [219u8, 137u8, 227u8, 244u8],
            [226u8, 81u8, 125u8, 63u8],
            [228u8, 80u8, 211u8, 140u8],
            [230u8, 2u8, 223u8, 5u8],
            [236u8, 68u8, 47u8, 5u8],
            [236u8, 211u8, 248u8, 30u8],
            [239u8, 105u8, 175u8, 101u8],
            [246u8, 69u8, 238u8, 223u8],
            [251u8, 143u8, 65u8, 178u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SyndicateTokenErrors {
        const NAME: &'static str = "SyndicateTokenErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 29usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AccessControlBadConfirmation(_) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AccessControlUnauthorizedAccount(_) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BurnOnlyDuringLockPeriod(_) => {
                    <BurnOnlyDuringLockPeriod as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CheckpointUnorderedInsertion(_) => {
                    <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignature(_) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureLength(_) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureS(_) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20ExceededSafeSupply(_) => {
                    <ERC20ExceededSafeSupply as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20InsufficientAllowance(_) => {
                    <ERC20InsufficientAllowance as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20InsufficientBalance(_) => {
                    <ERC20InsufficientBalance as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20InvalidApprover(_) => {
                    <ERC20InvalidApprover as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20InvalidReceiver(_) => {
                    <ERC20InvalidReceiver as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20InvalidSender(_) => {
                    <ERC20InvalidSender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC20InvalidSpender(_) => {
                    <ERC20InvalidSpender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC2612ExpiredSignature(_) => {
                    <ERC2612ExpiredSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC2612InvalidSigner(_) => {
                    <ERC2612InvalidSigner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC5805FutureLookup(_) => {
                    <ERC5805FutureLookup as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC6372InconsistentClock(_) => {
                    <ERC6372InconsistentClock as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ExceedsTotalSupply(_) => {
                    <ExceedsTotalSupply as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidAccountNonce(_) => {
                    <InvalidAccountNonce as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidShortString(_) => {
                    <InvalidShortString as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SafeCastOverflowedUintDowncast(_) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StringTooLong(_) => {
                    <StringTooLong as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TransfersLocked(_) => {
                    <TransfersLocked as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnlockTimestampInPast(_) => {
                    <UnlockTimestampInPast as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnlockTimestampTooLate(_) => {
                    <UnlockTimestampTooLate as alloy_sol_types::SolError>::SELECTOR
                }
                Self::VotesExpiredSignature(_) => {
                    <VotesExpiredSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroAddress(_) => {
                    <ZeroAddress as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroAmount(_) => {
                    <ZeroAmount as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<SyndicateTokenErrors>] = &[
                {
                    fn ExceedsTotalSupply(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ExceedsTotalSupply as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::ExceedsTotalSupply)
                    }
                    ExceedsTotalSupply
                },
                {
                    fn ERC20ExceededSafeSupply(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20ExceededSafeSupply as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC20ExceededSafeSupply)
                    }
                    ERC20ExceededSafeSupply
                },
                {
                    fn ZeroAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ZeroAmount as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(SyndicateTokenErrors::ZeroAmount)
                    }
                    ZeroAmount
                },
                {
                    fn CheckpointUnorderedInsertion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::CheckpointUnorderedInsertion)
                    }
                    CheckpointUnorderedInsertion
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn VotesExpiredSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <VotesExpiredSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::VotesExpiredSignature)
                    }
                    VotesExpiredSignature
                },
                {
                    fn ERC2612InvalidSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC2612InvalidSigner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC2612InvalidSigner)
                    }
                    ERC2612InvalidSigner
                },
                {
                    fn ERC2612ExpiredSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC2612ExpiredSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC2612ExpiredSignature)
                    }
                    ERC2612ExpiredSignature
                },
                {
                    fn AccessControlBadConfirmation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::AccessControlBadConfirmation)
                    }
                    AccessControlBadConfirmation
                },
                {
                    fn SafeCastOverflowedUintDowncast(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::SafeCastOverflowedUintDowncast)
                    }
                    SafeCastOverflowedUintDowncast
                },
                {
                    fn ERC6372InconsistentClock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC6372InconsistentClock as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC6372InconsistentClock)
                    }
                    ERC6372InconsistentClock
                },
                {
                    fn InvalidAccountNonce(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <InvalidAccountNonce as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::InvalidAccountNonce)
                    }
                    InvalidAccountNonce
                },
                {
                    fn ERC20InvalidSpender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20InvalidSpender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC20InvalidSpender)
                    }
                    ERC20InvalidSpender
                },
                {
                    fn ERC20InvalidSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20InvalidSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC20InvalidSender)
                    }
                    ERC20InvalidSender
                },
                {
                    fn UnlockTimestampInPast(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <UnlockTimestampInPast as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::UnlockTimestampInPast)
                    }
                    UnlockTimestampInPast
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn BurnOnlyDuringLockPeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <BurnOnlyDuringLockPeriod as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::BurnOnlyDuringLockPeriod)
                    }
                    BurnOnlyDuringLockPeriod
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn ZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ZeroAddress as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(SyndicateTokenErrors::ZeroAddress)
                    }
                    ZeroAddress
                },
                {
                    fn TransfersLocked(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <TransfersLocked as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::TransfersLocked)
                    }
                    TransfersLocked
                },
                {
                    fn AccessControlUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::AccessControlUnauthorizedAccount)
                    }
                    AccessControlUnauthorizedAccount
                },
                {
                    fn ERC20InsufficientBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20InsufficientBalance as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC20InsufficientBalance)
                    }
                    ERC20InsufficientBalance
                },
                {
                    fn ERC20InvalidApprover(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20InvalidApprover as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC20InvalidApprover)
                    }
                    ERC20InvalidApprover
                },
                {
                    fn ERC20InvalidReceiver(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20InvalidReceiver as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC20InvalidReceiver)
                    }
                    ERC20InvalidReceiver
                },
                {
                    fn ERC5805FutureLookup(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC5805FutureLookup as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC5805FutureLookup)
                    }
                    ERC5805FutureLookup
                },
                {
                    fn UnlockTimestampTooLate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <UnlockTimestampTooLate as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::UnlockTimestampTooLate)
                    }
                    UnlockTimestampTooLate
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn ERC20InsufficientAllowance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20InsufficientAllowance as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC20InsufficientAllowance)
                    }
                    ERC20InsufficientAllowance
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<SyndicateTokenErrors>] = &[
                {
                    fn ExceedsTotalSupply(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ExceedsTotalSupply as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::ExceedsTotalSupply)
                    }
                    ExceedsTotalSupply
                },
                {
                    fn ERC20ExceededSafeSupply(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20ExceededSafeSupply as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC20ExceededSafeSupply)
                    }
                    ERC20ExceededSafeSupply
                },
                {
                    fn ZeroAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ZeroAmount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::ZeroAmount)
                    }
                    ZeroAmount
                },
                {
                    fn CheckpointUnorderedInsertion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::CheckpointUnorderedInsertion)
                    }
                    CheckpointUnorderedInsertion
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn VotesExpiredSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <VotesExpiredSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::VotesExpiredSignature)
                    }
                    VotesExpiredSignature
                },
                {
                    fn ERC2612InvalidSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC2612InvalidSigner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC2612InvalidSigner)
                    }
                    ERC2612InvalidSigner
                },
                {
                    fn ERC2612ExpiredSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC2612ExpiredSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC2612ExpiredSignature)
                    }
                    ERC2612ExpiredSignature
                },
                {
                    fn AccessControlBadConfirmation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::AccessControlBadConfirmation)
                    }
                    AccessControlBadConfirmation
                },
                {
                    fn SafeCastOverflowedUintDowncast(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::SafeCastOverflowedUintDowncast)
                    }
                    SafeCastOverflowedUintDowncast
                },
                {
                    fn ERC6372InconsistentClock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC6372InconsistentClock as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC6372InconsistentClock)
                    }
                    ERC6372InconsistentClock
                },
                {
                    fn InvalidAccountNonce(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <InvalidAccountNonce as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::InvalidAccountNonce)
                    }
                    InvalidAccountNonce
                },
                {
                    fn ERC20InvalidSpender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20InvalidSpender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC20InvalidSpender)
                    }
                    ERC20InvalidSpender
                },
                {
                    fn ERC20InvalidSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20InvalidSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC20InvalidSender)
                    }
                    ERC20InvalidSender
                },
                {
                    fn UnlockTimestampInPast(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <UnlockTimestampInPast as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::UnlockTimestampInPast)
                    }
                    UnlockTimestampInPast
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn BurnOnlyDuringLockPeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <BurnOnlyDuringLockPeriod as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::BurnOnlyDuringLockPeriod)
                    }
                    BurnOnlyDuringLockPeriod
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn ZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ZeroAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::ZeroAddress)
                    }
                    ZeroAddress
                },
                {
                    fn TransfersLocked(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <TransfersLocked as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::TransfersLocked)
                    }
                    TransfersLocked
                },
                {
                    fn AccessControlUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::AccessControlUnauthorizedAccount)
                    }
                    AccessControlUnauthorizedAccount
                },
                {
                    fn ERC20InsufficientBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20InsufficientBalance as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC20InsufficientBalance)
                    }
                    ERC20InsufficientBalance
                },
                {
                    fn ERC20InvalidApprover(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20InvalidApprover as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC20InvalidApprover)
                    }
                    ERC20InvalidApprover
                },
                {
                    fn ERC20InvalidReceiver(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20InvalidReceiver as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC20InvalidReceiver)
                    }
                    ERC20InvalidReceiver
                },
                {
                    fn ERC5805FutureLookup(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC5805FutureLookup as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC5805FutureLookup)
                    }
                    ERC5805FutureLookup
                },
                {
                    fn UnlockTimestampTooLate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <UnlockTimestampTooLate as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::UnlockTimestampTooLate)
                    }
                    UnlockTimestampTooLate
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn ERC20InsufficientAllowance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20InsufficientAllowance as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::ERC20InsufficientAllowance)
                    }
                    ERC20InsufficientAllowance
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AccessControlBadConfirmation(inner) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AccessControlUnauthorizedAccount(inner) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BurnOnlyDuringLockPeriod(inner) => {
                    <BurnOnlyDuringLockPeriod as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CheckpointUnorderedInsertion(inner) => {
                    <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20ExceededSafeSupply(inner) => {
                    <ERC20ExceededSafeSupply as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20InsufficientAllowance(inner) => {
                    <ERC20InsufficientAllowance as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20InsufficientBalance(inner) => {
                    <ERC20InsufficientBalance as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20InvalidApprover(inner) => {
                    <ERC20InvalidApprover as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20InvalidReceiver(inner) => {
                    <ERC20InvalidReceiver as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20InvalidSender(inner) => {
                    <ERC20InvalidSender as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC20InvalidSpender(inner) => {
                    <ERC20InvalidSpender as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC2612ExpiredSignature(inner) => {
                    <ERC2612ExpiredSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC2612InvalidSigner(inner) => {
                    <ERC2612InvalidSigner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC5805FutureLookup(inner) => {
                    <ERC5805FutureLookup as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC6372InconsistentClock(inner) => {
                    <ERC6372InconsistentClock as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ExceedsTotalSupply(inner) => {
                    <ExceedsTotalSupply as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidAccountNonce(inner) => {
                    <InvalidAccountNonce as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SafeCastOverflowedUintDowncast(inner) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::TransfersLocked(inner) => {
                    <TransfersLocked as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnlockTimestampInPast(inner) => {
                    <UnlockTimestampInPast as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnlockTimestampTooLate(inner) => {
                    <UnlockTimestampTooLate as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::VotesExpiredSignature(inner) => {
                    <VotesExpiredSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ZeroAddress(inner) => {
                    <ZeroAddress as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ZeroAmount(inner) => {
                    <ZeroAmount as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AccessControlBadConfirmation(inner) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AccessControlUnauthorizedAccount(inner) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BurnOnlyDuringLockPeriod(inner) => {
                    <BurnOnlyDuringLockPeriod as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CheckpointUnorderedInsertion(inner) => {
                    <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20ExceededSafeSupply(inner) => {
                    <ERC20ExceededSafeSupply as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20InsufficientAllowance(inner) => {
                    <ERC20InsufficientAllowance as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20InsufficientBalance(inner) => {
                    <ERC20InsufficientBalance as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20InvalidApprover(inner) => {
                    <ERC20InvalidApprover as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20InvalidReceiver(inner) => {
                    <ERC20InvalidReceiver as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20InvalidSender(inner) => {
                    <ERC20InvalidSender as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC20InvalidSpender(inner) => {
                    <ERC20InvalidSpender as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC2612ExpiredSignature(inner) => {
                    <ERC2612ExpiredSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC2612InvalidSigner(inner) => {
                    <ERC2612InvalidSigner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC5805FutureLookup(inner) => {
                    <ERC5805FutureLookup as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC6372InconsistentClock(inner) => {
                    <ERC6372InconsistentClock as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ExceedsTotalSupply(inner) => {
                    <ExceedsTotalSupply as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidAccountNonce(inner) => {
                    <InvalidAccountNonce as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SafeCastOverflowedUintDowncast(inner) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TransfersLocked(inner) => {
                    <TransfersLocked as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnlockTimestampInPast(inner) => {
                    <UnlockTimestampInPast as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnlockTimestampTooLate(inner) => {
                    <UnlockTimestampTooLate as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::VotesExpiredSignature(inner) => {
                    <VotesExpiredSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroAddress(inner) => {
                    <ZeroAddress as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroAmount(inner) => {
                    <ZeroAmount as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`SyndicateToken`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum SyndicateTokenEvents {
        #[allow(missing_docs)]
        Approval(Approval),
        #[allow(missing_docs)]
        DelegateChanged(DelegateChanged),
        #[allow(missing_docs)]
        DelegateVotesChanged(DelegateVotesChanged),
        #[allow(missing_docs)]
        EIP712DomainChanged(EIP712DomainChanged),
        #[allow(missing_docs)]
        RoleAdminChanged(RoleAdminChanged),
        #[allow(missing_docs)]
        RoleGranted(RoleGranted),
        #[allow(missing_docs)]
        RoleRevoked(RoleRevoked),
        #[allow(missing_docs)]
        TokensBurnedByManager(TokensBurnedByManager),
        #[allow(missing_docs)]
        Transfer(Transfer),
        #[allow(missing_docs)]
        UnlockTimestampUpdated(UnlockTimestampUpdated),
    }
    #[automatically_derived]
    impl SyndicateTokenEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                10u8, 99u8, 135u8, 201u8, 234u8, 54u8, 40u8, 184u8, 138u8, 99u8, 59u8,
                180u8, 243u8, 177u8, 81u8, 119u8, 15u8, 112u8, 8u8, 81u8, 23u8, 161u8,
                95u8, 155u8, 243u8, 120u8, 124u8, 218u8, 83u8, 241u8, 61u8, 49u8,
            ],
            [
                47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8,
                236u8, 121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8,
                64u8, 48u8, 69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
            ],
            [
                49u8, 52u8, 232u8, 162u8, 230u8, 217u8, 126u8, 146u8, 154u8, 126u8, 84u8,
                1u8, 30u8, 165u8, 72u8, 93u8, 125u8, 25u8, 109u8, 213u8, 240u8, 186u8,
                77u8, 78u8, 249u8, 88u8, 3u8, 232u8, 227u8, 252u8, 37u8, 127u8,
            ],
            [
                140u8, 91u8, 225u8, 229u8, 235u8, 236u8, 125u8, 91u8, 209u8, 79u8, 113u8,
                66u8, 125u8, 30u8, 132u8, 243u8, 221u8, 3u8, 20u8, 192u8, 247u8, 178u8,
                41u8, 30u8, 91u8, 32u8, 10u8, 200u8, 199u8, 195u8, 185u8, 37u8,
            ],
            [
                189u8, 121u8, 184u8, 111u8, 254u8, 10u8, 184u8, 232u8, 119u8, 97u8, 81u8,
                81u8, 66u8, 23u8, 205u8, 124u8, 172u8, 213u8, 44u8, 144u8, 159u8, 102u8,
                71u8, 92u8, 58u8, 244u8, 78u8, 18u8, 159u8, 11u8, 0u8, 255u8,
            ],
            [
                190u8, 244u8, 248u8, 28u8, 24u8, 20u8, 198u8, 65u8, 237u8, 232u8, 94u8,
                186u8, 172u8, 241u8, 157u8, 4u8, 139u8, 44u8, 91u8, 85u8, 152u8, 10u8,
                223u8, 166u8, 239u8, 15u8, 149u8, 108u8, 101u8, 19u8, 53u8, 162u8,
            ],
            [
                221u8, 104u8, 150u8, 220u8, 241u8, 212u8, 179u8, 17u8, 204u8, 168u8,
                125u8, 209u8, 155u8, 187u8, 162u8, 234u8, 156u8, 226u8, 248u8, 103u8,
                193u8, 86u8, 136u8, 120u8, 160u8, 67u8, 138u8, 102u8, 161u8, 175u8,
                238u8, 236u8,
            ],
            [
                221u8, 242u8, 82u8, 173u8, 27u8, 226u8, 200u8, 155u8, 105u8, 194u8,
                176u8, 104u8, 252u8, 55u8, 141u8, 170u8, 149u8, 43u8, 167u8, 241u8, 99u8,
                196u8, 161u8, 22u8, 40u8, 245u8, 90u8, 77u8, 245u8, 35u8, 179u8, 239u8,
            ],
            [
                222u8, 194u8, 186u8, 205u8, 210u8, 240u8, 91u8, 89u8, 222u8, 52u8, 218u8,
                155u8, 82u8, 61u8, 255u8, 139u8, 228u8, 46u8, 94u8, 56u8, 232u8, 24u8,
                200u8, 47u8, 219u8, 11u8, 174u8, 119u8, 67u8, 135u8, 167u8, 36u8,
            ],
            [
                246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8,
                103u8, 11u8, 68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8,
                253u8, 100u8, 235u8, 33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for SyndicateTokenEvents {
        const NAME: &'static str = "SyndicateTokenEvents";
        const COUNT: usize = 10usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Approval as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Approval as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Approval)
                }
                Some(<DelegateChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <DelegateChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::DelegateChanged)
                }
                Some(
                    <DelegateVotesChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DelegateVotesChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::DelegateVotesChanged)
                }
                Some(
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::EIP712DomainChanged)
                }
                Some(<RoleAdminChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleAdminChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RoleAdminChanged)
                }
                Some(<RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleGranted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RoleGranted)
                }
                Some(<RoleRevoked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleRevoked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RoleRevoked)
                }
                Some(
                    <TokensBurnedByManager as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TokensBurnedByManager as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::TokensBurnedByManager)
                }
                Some(<Transfer as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Transfer as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Transfer)
                }
                Some(
                    <UnlockTimestampUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UnlockTimestampUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UnlockTimestampUpdated)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for SyndicateTokenEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Approval(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DelegateChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DelegateVotesChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleAdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TokensBurnedByManager(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Transfer(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UnlockTimestampUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Approval(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DelegateChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DelegateVotesChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleAdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TokensBurnedByManager(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Transfer(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UnlockTimestampUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`SyndicateToken`](self) contract instance.

See the [wrapper's documentation](`SyndicateTokenInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SyndicateTokenInstance<P, N> {
        SyndicateTokenInstance::<P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        defaultAdmin: alloy::sol_types::private::Address,
        syndTreasuryAddress: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<SyndicateTokenInstance<P, N>>,
    > {
        SyndicateTokenInstance::<
            P,
            N,
        >::deploy(provider, defaultAdmin, syndTreasuryAddress)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        defaultAdmin: alloy::sol_types::private::Address,
        syndTreasuryAddress: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        SyndicateTokenInstance::<
            P,
            N,
        >::deploy_builder(provider, defaultAdmin, syndTreasuryAddress)
    }
    /**A [`SyndicateToken`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SyndicateToken`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SyndicateTokenInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for SyndicateTokenInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SyndicateTokenInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SyndicateTokenInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`SyndicateToken`](self) contract instance.

See the [wrapper's documentation](`SyndicateTokenInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            defaultAdmin: alloy::sol_types::private::Address,
            syndTreasuryAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<SyndicateTokenInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                defaultAdmin,
                syndTreasuryAddress,
            );
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            defaultAdmin: alloy::sol_types::private::Address,
            syndTreasuryAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            defaultAdmin,
                            syndTreasuryAddress,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> SyndicateTokenInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SyndicateTokenInstance<P, N> {
            SyndicateTokenInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SyndicateTokenInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`AIRDROP_MANAGER_ROLE`] function.
        pub fn AIRDROP_MANAGER_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, AIRDROP_MANAGER_ROLECall, N> {
            self.call_builder(&AIRDROP_MANAGER_ROLECall)
        }
        ///Creates a new call builder for the [`CLOCK_MODE`] function.
        pub fn CLOCK_MODE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, CLOCK_MODECall, N> {
            self.call_builder(&CLOCK_MODECall)
        }
        ///Creates a new call builder for the [`DEFAULT_ADMIN_ROLE`] function.
        pub fn DEFAULT_ADMIN_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, DEFAULT_ADMIN_ROLECall, N> {
            self.call_builder(&DEFAULT_ADMIN_ROLECall)
        }
        ///Creates a new call builder for the [`DOMAIN_SEPARATOR`] function.
        pub fn DOMAIN_SEPARATOR(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, DOMAIN_SEPARATORCall, N> {
            self.call_builder(&DOMAIN_SEPARATORCall)
        }
        ///Creates a new call builder for the [`EMISSION_MINTER_ROLE`] function.
        pub fn EMISSION_MINTER_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, EMISSION_MINTER_ROLECall, N> {
            self.call_builder(&EMISSION_MINTER_ROLECall)
        }
        ///Creates a new call builder for the [`INITIAL_MINT_SUPPLY`] function.
        pub fn INITIAL_MINT_SUPPLY(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, INITIAL_MINT_SUPPLYCall, N> {
            self.call_builder(&INITIAL_MINT_SUPPLYCall)
        }
        ///Creates a new call builder for the [`MAX_LOCK_DURATION`] function.
        pub fn MAX_LOCK_DURATION(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, MAX_LOCK_DURATIONCall, N> {
            self.call_builder(&MAX_LOCK_DURATIONCall)
        }
        ///Creates a new call builder for the [`TOTAL_SUPPLY`] function.
        pub fn TOTAL_SUPPLY(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, TOTAL_SUPPLYCall, N> {
            self.call_builder(&TOTAL_SUPPLYCall)
        }
        ///Creates a new call builder for the [`allowance`] function.
        pub fn allowance(
            &self,
            owner: alloy::sol_types::private::Address,
            spender: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, allowanceCall, N> {
            self.call_builder(&allowanceCall { owner, spender })
        }
        ///Creates a new call builder for the [`approve`] function.
        pub fn approve(
            &self,
            spender: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, approveCall, N> {
            self.call_builder(&approveCall { spender, value })
        }
        ///Creates a new call builder for the [`balanceOf`] function.
        pub fn balanceOf(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, balanceOfCall, N> {
            self.call_builder(&balanceOfCall { account })
        }
        ///Creates a new call builder for the [`burn`] function.
        pub fn burn(
            &self,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, burnCall, N> {
            self.call_builder(&burnCall { amount })
        }
        ///Creates a new call builder for the [`burnFrom`] function.
        pub fn burnFrom(
            &self,
            from: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, burnFromCall, N> {
            self.call_builder(&burnFromCall { from, amount })
        }
        ///Creates a new call builder for the [`checkpoints`] function.
        pub fn checkpoints(
            &self,
            account: alloy::sol_types::private::Address,
            pos: u32,
        ) -> alloy_contract::SolCallBuilder<&P, checkpointsCall, N> {
            self.call_builder(&checkpointsCall { account, pos })
        }
        ///Creates a new call builder for the [`clock`] function.
        pub fn clock(&self) -> alloy_contract::SolCallBuilder<&P, clockCall, N> {
            self.call_builder(&clockCall)
        }
        ///Creates a new call builder for the [`decimals`] function.
        pub fn decimals(&self) -> alloy_contract::SolCallBuilder<&P, decimalsCall, N> {
            self.call_builder(&decimalsCall)
        }
        ///Creates a new call builder for the [`delegate`] function.
        pub fn delegate(
            &self,
            delegatee: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, delegateCall, N> {
            self.call_builder(&delegateCall { delegatee })
        }
        ///Creates a new call builder for the [`delegateBySig`] function.
        pub fn delegateBySig(
            &self,
            delegatee: alloy::sol_types::private::Address,
            nonce: alloy::sol_types::private::primitives::aliases::U256,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
            v: u8,
            r: alloy::sol_types::private::FixedBytes<32>,
            s: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, delegateBySigCall, N> {
            self.call_builder(
                &delegateBySigCall {
                    delegatee,
                    nonce,
                    expiry,
                    v,
                    r,
                    s,
                },
            )
        }
        ///Creates a new call builder for the [`delegates`] function.
        pub fn delegates(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, delegatesCall, N> {
            self.call_builder(&delegatesCall { account })
        }
        ///Creates a new call builder for the [`eip712Domain`] function.
        pub fn eip712Domain(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, eip712DomainCall, N> {
            self.call_builder(&eip712DomainCall)
        }
        ///Creates a new call builder for the [`getCurrentTotalSupply`] function.
        pub fn getCurrentTotalSupply(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCurrentTotalSupplyCall, N> {
            self.call_builder(&getCurrentTotalSupplyCall)
        }
        ///Creates a new call builder for the [`getPastTotalSupply`] function.
        pub fn getPastTotalSupply(
            &self,
            timepoint: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getPastTotalSupplyCall, N> {
            self.call_builder(
                &getPastTotalSupplyCall {
                    timepoint,
                },
            )
        }
        ///Creates a new call builder for the [`getPastVotes`] function.
        pub fn getPastVotes(
            &self,
            account: alloy::sol_types::private::Address,
            timepoint: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getPastVotesCall, N> {
            self.call_builder(
                &getPastVotesCall {
                    account,
                    timepoint,
                },
            )
        }
        ///Creates a new call builder for the [`getPastVotingPower`] function.
        pub fn getPastVotingPower(
            &self,
            account: alloy::sol_types::private::Address,
            blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getPastVotingPowerCall, N> {
            self.call_builder(
                &getPastVotingPowerCall {
                    account,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getRemainingLockTime`] function.
        pub fn getRemainingLockTime(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getRemainingLockTimeCall, N> {
            self.call_builder(&getRemainingLockTimeCall)
        }
        ///Creates a new call builder for the [`getRoleAdmin`] function.
        pub fn getRoleAdmin(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getRoleAdminCall, N> {
            self.call_builder(&getRoleAdminCall { role })
        }
        ///Creates a new call builder for the [`getVotes`] function.
        pub fn getVotes(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, getVotesCall, N> {
            self.call_builder(&getVotesCall { account })
        }
        ///Creates a new call builder for the [`getVotingPower`] function.
        pub fn getVotingPower(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, getVotingPowerCall, N> {
            self.call_builder(&getVotingPowerCall { account })
        }
        ///Creates a new call builder for the [`grantRole`] function.
        pub fn grantRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, grantRoleCall, N> {
            self.call_builder(&grantRoleCall { role, account })
        }
        ///Creates a new call builder for the [`hasRole`] function.
        pub fn hasRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, hasRoleCall, N> {
            self.call_builder(&hasRoleCall { role, account })
        }
        ///Creates a new call builder for the [`maxLockTimestamp`] function.
        pub fn maxLockTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, maxLockTimestampCall, N> {
            self.call_builder(&maxLockTimestampCall)
        }
        ///Creates a new call builder for the [`mint`] function.
        pub fn mint(
            &self,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, mintCall, N> {
            self.call_builder(&mintCall { to, amount })
        }
        ///Creates a new call builder for the [`name`] function.
        pub fn name(&self) -> alloy_contract::SolCallBuilder<&P, nameCall, N> {
            self.call_builder(&nameCall)
        }
        ///Creates a new call builder for the [`nonces`] function.
        pub fn nonces(
            &self,
            owner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, noncesCall, N> {
            self.call_builder(&noncesCall { owner })
        }
        ///Creates a new call builder for the [`numCheckpoints`] function.
        pub fn numCheckpoints(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, numCheckpointsCall, N> {
            self.call_builder(&numCheckpointsCall { account })
        }
        ///Creates a new call builder for the [`permit`] function.
        pub fn permit(
            &self,
            owner: alloy::sol_types::private::Address,
            spender: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
            v: u8,
            r: alloy::sol_types::private::FixedBytes<32>,
            s: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, permitCall, N> {
            self.call_builder(
                &permitCall {
                    owner,
                    spender,
                    value,
                    deadline,
                    v,
                    r,
                    s,
                },
            )
        }
        ///Creates a new call builder for the [`renounceRole`] function.
        pub fn renounceRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            callerConfirmation: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, renounceRoleCall, N> {
            self.call_builder(
                &renounceRoleCall {
                    role,
                    callerConfirmation,
                },
            )
        }
        ///Creates a new call builder for the [`revokeRole`] function.
        pub fn revokeRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, revokeRoleCall, N> {
            self.call_builder(&revokeRoleCall { role, account })
        }
        ///Creates a new call builder for the [`setUnlockTimestamp`] function.
        pub fn setUnlockTimestamp(
            &self,
            newUnlockTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, setUnlockTimestampCall, N> {
            self.call_builder(
                &setUnlockTimestampCall {
                    newUnlockTimestamp,
                },
            )
        }
        ///Creates a new call builder for the [`supportsInterface`] function.
        pub fn supportsInterface(
            &self,
            interfaceId: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<&P, supportsInterfaceCall, N> {
            self.call_builder(
                &supportsInterfaceCall {
                    interfaceId,
                },
            )
        }
        ///Creates a new call builder for the [`symbol`] function.
        pub fn symbol(&self) -> alloy_contract::SolCallBuilder<&P, symbolCall, N> {
            self.call_builder(&symbolCall)
        }
        ///Creates a new call builder for the [`totalSupply`] function.
        pub fn totalSupply(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, totalSupplyCall, N> {
            self.call_builder(&totalSupplyCall)
        }
        ///Creates a new call builder for the [`transfer`] function.
        pub fn transfer(
            &self,
            to: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, transferCall, N> {
            self.call_builder(&transferCall { to, value })
        }
        ///Creates a new call builder for the [`transferFrom`] function.
        pub fn transferFrom(
            &self,
            from: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, transferFromCall, N> {
            self.call_builder(
                &transferFromCall {
                    from,
                    to,
                    value,
                },
            )
        }
        ///Creates a new call builder for the [`transfersLocked`] function.
        pub fn transfersLocked(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, transfersLockedCall, N> {
            self.call_builder(&transfersLockedCall)
        }
        ///Creates a new call builder for the [`unlockTimestamp`] function.
        pub fn unlockTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, unlockTimestampCall, N> {
            self.call_builder(&unlockTimestampCall)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > SyndicateTokenInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Approval`] event.
        pub fn Approval_filter(&self) -> alloy_contract::Event<&P, Approval, N> {
            self.event_filter::<Approval>()
        }
        ///Creates a new event filter for the [`DelegateChanged`] event.
        pub fn DelegateChanged_filter(
            &self,
        ) -> alloy_contract::Event<&P, DelegateChanged, N> {
            self.event_filter::<DelegateChanged>()
        }
        ///Creates a new event filter for the [`DelegateVotesChanged`] event.
        pub fn DelegateVotesChanged_filter(
            &self,
        ) -> alloy_contract::Event<&P, DelegateVotesChanged, N> {
            self.event_filter::<DelegateVotesChanged>()
        }
        ///Creates a new event filter for the [`EIP712DomainChanged`] event.
        pub fn EIP712DomainChanged_filter(
            &self,
        ) -> alloy_contract::Event<&P, EIP712DomainChanged, N> {
            self.event_filter::<EIP712DomainChanged>()
        }
        ///Creates a new event filter for the [`RoleAdminChanged`] event.
        pub fn RoleAdminChanged_filter(
            &self,
        ) -> alloy_contract::Event<&P, RoleAdminChanged, N> {
            self.event_filter::<RoleAdminChanged>()
        }
        ///Creates a new event filter for the [`RoleGranted`] event.
        pub fn RoleGranted_filter(&self) -> alloy_contract::Event<&P, RoleGranted, N> {
            self.event_filter::<RoleGranted>()
        }
        ///Creates a new event filter for the [`RoleRevoked`] event.
        pub fn RoleRevoked_filter(&self) -> alloy_contract::Event<&P, RoleRevoked, N> {
            self.event_filter::<RoleRevoked>()
        }
        ///Creates a new event filter for the [`TokensBurnedByManager`] event.
        pub fn TokensBurnedByManager_filter(
            &self,
        ) -> alloy_contract::Event<&P, TokensBurnedByManager, N> {
            self.event_filter::<TokensBurnedByManager>()
        }
        ///Creates a new event filter for the [`Transfer`] event.
        pub fn Transfer_filter(&self) -> alloy_contract::Event<&P, Transfer, N> {
            self.event_filter::<Transfer>()
        }
        ///Creates a new event filter for the [`UnlockTimestampUpdated`] event.
        pub fn UnlockTimestampUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, UnlockTimestampUpdated, N> {
            self.event_filter::<UnlockTimestampUpdated>()
        }
    }
}
