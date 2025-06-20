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
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> CheckpointsInstance<T, P, N> {
        CheckpointsInstance::<T, P, N>::new(address, provider)
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
    pub struct CheckpointsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for CheckpointsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("CheckpointsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > CheckpointsInstance<T, P, N> {
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
                _network_transport: ::core::marker::PhantomData,
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
    impl<T, P: ::core::clone::Clone, N> CheckpointsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> CheckpointsInstance<T, P, N> {
            CheckpointsInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > CheckpointsInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > CheckpointsInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
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
    error UnlockTimestampAlreadySet();
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
    function getRemainingEmissions() external view returns (uint256);
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
    "name": "getRemainingEmissions",
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
    "name": "UnlockTimestampAlreadySet",
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
    ///0x610180604052346100845761001b610015610158565b906103be565b610023610089565b614b67611c84823960805181612948015260a0518161297f015260c0518161290f015260e051816133550152610100518161337a01526101205181612ebc01526101405181612efc015261016051818181610b110152611f790152614b6790f35b61008f565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b906100bb90610093565b810190811060018060401b038211176100d357604052565b61009d565b906100eb6100e4610089565b92836100b1565b565b5f80fd5b60018060a01b031690565b610105906100f1565b90565b610111816100fc565b0361011857565b5f80fd5b9050519061012982610108565b565b91906040838203126101535780610147610150925f860161011c565b9360200161011c565b90565b6100ed565b6101766167eb8038038061016b816100d8565b92833981019061012b565b9091565b60018060401b03811161019657610192602091610093565b0190565b61009d565b906101ad6101a88361017a565b6100d8565b918252565b5f7f53796e6469636174650000000000000000000000000000000000000000000000910152565b6101e3600961019b565b906101f0602083016101b2565b565b6101fa6101d9565b90565b5f7f53594e4400000000000000000000000000000000000000000000000000000000910152565b61022e600461019b565b9061023b602083016101fd565b565b610245610224565b90565b90565b90565b61026261025d61026792610248565b61024b565b6100f1565b90565b6102739061024e565b90565b5f0190565b90565b90565b61029561029061029a9261027b565b61024b565b61027e565b90565b6102aa6301e13380610281565b90565b634e487b7160e01b5f52601160045260245ffd5b6102d06102d69193929361027e565b9261027e565b82018092116102e157565b6102ad565b6102fa6102f56102ff92610248565b61024b565b61027e565b90565b5f1b90565b906103135f1991610302565b9181191691161790565b61033161032c6103369261027e565b61024b565b61027e565b90565b90565b9061035161034c6103589261031d565b610339565b8254610307565b9055565b90565b61037361036e61037892610248565b610302565b61035c565b90565b6103845f61035f565b90565b90565b61039e6103996103a392610387565b61024b565b61027e565b90565b6103bb6b02e87669c308736a0400000061038a565b90565b906103e06103ca6101f2565b6103d26101f2565b6103da61023d565b916104a6565b816103fb6103f56103f05f61026a565b6100fc565b916100fc565b1461048a578061041b6104156104105f61026a565b6100fc565b916100fc565b1461046e5761045d61046c926104394261043361029d565b906102c1565b610160526104506104495f6102e6565b600c61033c565b61045861037b565b610942565b506104666103a6565b90610a10565b565b5f63d92e233d60e01b81528061048660048201610276565b0390fd5b5f63d92e233d60e01b8152806104a260048201610276565b0390fd5b906104b192916104b3565b565b906104be92916104c0565b565b906104cb92916104cd565b565b906104d892916104da565b565b906104e59291610532565b565b5f7f3100000000000000000000000000000000000000000000000000000000000000910152565b610518600161019b565b90610525602083016104e7565b565b61052f61050e565b90565b906105469291610540610527565b90610548565b565b9061055493929161059a565b565b90565b90565b60200190565b5190565b61057a61057561057f926100f1565b61024b565b6100f1565b90565b61058b90610566565b90565b61059790610582565b90565b6105ab6105fb946105e0939461062f565b6105bf816105b96006610556565b90610abd565b610120526105d7836105d16007610556565b90610abd565b61014052610559565b6105f26105ec82610562565b9161055c565b2060e052610559565b61060d61060782610562565b9161055c565b20610100524660a05261061e610bc2565b60805261062a3061058e565b60c052565b906106399161063b565b565b9061064591610647565b565b9061065191610898565b565b634e487b7160e01b5f525f60045260245ffd5b5190565b634e487b7160e01b5f52602260045260245ffd5b906001600283049216801561069e575b602083101461069957565b61066a565b91607f169161068e565b5f5260205f2090565b601f602091010490565b1b90565b919060086106da9102916106d45f19846106bb565b926106bb565b9181191691161790565b91906106fa6106f56107029361031d565b610339565b9083546106bf565b9055565b5f90565b61071c91610716610706565b916106e4565b565b5b81811061072a575050565b806107375f60019361070a565b0161071f565b9190601f811161074d575b505050565b61075961077e936106a8565b906020610765846106b1565b83019310610786575b610777906106b1565b019061071e565b5f8080610748565b91506107778192905061076e565b1c90565b906107a8905f1990600802610794565b191690565b816107b791610798565b906002021790565b906107c981610666565b9060018060401b038211610887576107eb826107e5855461067e565b8561073d565b602090601f831160011461081f5791809161080e935f92610813575b50506107ad565b90555b565b90915001515f80610807565b601f1983169161082e856106a8565b925f5b81811061086f57509160029391856001969410610855575b50505002019055610811565b610865910151601f841690610798565b90555f8080610849565b91936020600181928787015181550195019201610831565b61009d565b90610896916107bf565b565b906108a76108ae92600361088c565b600461088c565b565b5f90565b151590565b6108c29061035c565b90565b906108cf906108b9565b5f5260205260405f2090565b6108e490610582565b90565b906108f1906108db565b5f5260205260405f2090565b9061090960ff91610302565b9181191691161790565b61091c906108b4565b90565b90565b9061093761093261093e92610913565b61091f565b82546108fd565b9055565b61094a6108b0565b5061095f610959828490610c5f565b156108b4565b5f146109e85761098760016109825f61097a600586906108c5565b0185906108e7565b610922565b90610990610c8d565b906109cd6109c76109c17f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d956108b9565b926108db565b926108db565b926109d6610089565b806109e081610276565b0390a4600190565b50505f90565b6109f7906100fc565b9052565b9190610a0e905f602085019401906109ee565b565b80610a2b610a25610a205f61026a565b6100fc565b916100fc565b14610a4757610a4591610a3d5f61026a565b919091610cbe565b565b610a6a610a535f61026a565b5f91829163ec442f0560e01b8352600483016109fb565b0390fd5b5f90565b90565b610a89610a84610a8e92610a72565b61024b565b61027e565b90565b90565b610aa8610aa3610aad92610a91565b610302565b61035c565b90565b610aba60ff610a94565b90565b90610ac6610a6e565b50610ad8610ad383610559565b610562565b610aeb610ae56020610a75565b9161027e565b105f14610aff5750610afc90610e58565b90565b5f610b0d610b139392610d68565b0161088c565b610b23610b1e610ab0565b6108b9565b90565b5f90565b7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f90565b610b58905161035c565b90565b610b649061035c565b9052565b610b719061027e565b9052565b90959492610bc094610baf610bb992610ba5608096610b9b60a088019c5f890190610b5b565b6020870190610b5b565b6040850190610b5b565b6060830190610b68565b01906109ee565b565b610bca610b26565b50610bd3610b2a565b610c1d610be060e0610b4e565b91610c0e610bef610100610b4e565b46610bf93061058e565b91610c02610089565b96879560208701610b75565b602082018103825203826100b1565b610c2f610c2982610562565b9161055c565b2090565b5f1c90565b60ff1690565b610c4a610c4f91610c33565b610c38565b90565b610c5c9054610c3e565b90565b610c86915f610c7b610c8193610c736108b0565b5060056108c5565b016108e7565b610c52565b90565b5f90565b610c95610c89565b503390565b7f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a67490565b9182610cda610cd4610ccf5f61026a565b6100fc565b916100fc565b141580610d45575b610cf5575b610cf392919091610f7c565b565b610cfd610f06565b80610d24575b15610ce7575f6336e278fd60e21b815280610d2060048201610276565b0390fd5b50610d40610d3a610d33610c9a565b3390610c5f565b156108b4565b610d03565b5081610d61610d5b610d565f61026a565b6100fc565b916100fc565b1415610ce2565b90565b90565b610d82610d7d610d8792610d6b565b61024b565b61027e565b90565b60209181520190565b90825f9392825e0152565b610dbd610dc6602093610dcb93610db481610666565b93848093610d8a565b95869101610d93565b610093565b0190565b610de49160208201915f818403910152610d9e565b90565b610e01610dfc610df683610562565b9261055c565b610b4e565b9060208110610e0f575b5090565b610e21905f19906020036008026106bb565b165f610e0b565b610e34610e3991610c33565b61031d565b90565b610e50610e4b610e559261027e565b610302565b61035c565b90565b610e60610a6e565b50610e6a81610559565b90610e7482610562565b610e87610e81601f610d6e565b9161027e565b11610ebc5750610eb481610eae610ea8610ea3610eb995610de7565b610e28565b91610562565b17610e3c565b6108b9565b90565b610ede90610ec8610089565b91829163305a27a960e01b835260048301610dcf565b0390fd5b90565b610ef1610ef691610c33565b610ee2565b90565b610f039054610ee5565b90565b610f0e6108b0565b50610f19600c610ef9565b610f2b610f255f6102e6565b9161027e565b141580610f36575b90565b5042610f53610f4d610f48600c610ef9565b61027e565b9161027e565b10610f33565b916020610f7a929493610f7360408201965f830190610b68565b0190610b68565b565b9291610f8a84838391611085565b83610fa5610f9f610f9a5f61026a565b6100fc565b916100fc565b14610fba575b610fb89293919091611252565b565b610fc26111f4565b93610fcb611231565b9480610fdf610fd98861027e565b9161027e565b11610fec57509350610fab565b85906110085f928392630e58ae9360e11b845260048401610f59565b0390fd5b90611016906108db565b5f5260205260405f2090565b60409061104b611052949695939661104160608401985f8501906109ee565b6020830190610b68565b0190610b68565b565b9061105f910361027e565b90565b9061106d910161027e565b90565b9190611083905f60208501940190610b68565b565b919091806110a361109d6110985f61026a565b6100fc565b916100fc565b145f14611184576110c76110c0836110bb6002610ef9565b6102c1565b600261033c565b5b826110e36110dd6110d85f61026a565b6100fc565b916100fc565b145f1461115857611107611100836110fb6002610ef9565b611054565b600261033c565b5b91909161115361114161113b7fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef936108db565b936108db565b9361114a610089565b91829182611070565b0390a3565b61117f8261117961116a5f879061100c565b9161117483610ef9565b611062565b9061033c565b611108565b6111976111925f839061100c565b610ef9565b806111aa6111a48561027e565b9161027e565b106111d2576111bd6111cd918490611054565b6111c85f849061100c565b61033c565b6110c8565b906111f09091925f93849363391434e360e21b855260048501611022565b0390fd5b6111fc610706565b506112076002610ef9565b90565b60018060d01b031690565b61122961122461122e9261120a565b61024b565b61027e565b90565b611239610706565b5061124960018060d01b03611215565b90565b90565b90565b916112aa6112a46112b1948061127861127261126d5f61026a565b6100fc565b916100fc565b146112e2575b8461129961129361128e5f61026a565b6100fc565b916100fc565b146112b3575b6114da565b926114da565b909161150f565b565b6112db600b60026112d56112cf6112c9896113c4565b9361124c565b9161124f565b90611417565b505061129f565b61130a600b60016113046112fe6112f8896113c4565b9361124c565b9161124f565b90611417565b505061127e565b5f90565b6113216113279161120a565b9161120a565b019060018060d01b03821161133857565b6102ad565b906113509161134a611311565b50611315565b90565b90565b60ff1690565b61137061136b61137592611353565b61024b565b611356565b90565b6113819061135c565b9052565b9160206113a692949361139f60408201965f830190611378565b0190610b68565b565b6113bc6113b76113c19261027e565b61024b565b61120a565b90565b6113cc611311565b50806113e66113e060018060d01b03611215565b9161027e565b116113f7576113f4906113a8565b90565b60d06114135f9283926306dfcc6560e41b845260048401611385565b0390fd5b9061144d6114539392611428611311565b50611431611311565b50809361144661143f6116c1565b949261176e565b9091611c54565b916117e3565b91909190565b61146561146b9161120a565b9161120a565b90039060018060d01b03821161147d57565b6102ad565b906114959161148f611311565b50611459565b90565b906114a2906108db565b5f5260205260405f2090565b60018060a01b031690565b6114c56114ca91610c33565b6114ae565b90565b6114d790546114b9565b90565b6114f16114f6916114e9610c89565b506009611498565b6114cd565b90565b90611503906108db565b5f5260205260405f2090565b9190918061152561151f856100fc565b916100fc565b1415806116a3575b611537575b505050565b8061155261154c6115475f61026a565b6100fc565b916100fc565b03611613575b508161157461156e6115695f61026a565b6100fc565b916100fc565b03611580575b80611532565b6115c76115ba6115c192611596600a86906114f9565b906115b46115ae6115a86001936113c4565b9361124c565b9161124f565b90611417565b9290611215565b91611215565b9190916115f47fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926108db565b92611609611600610089565b92839283610f59565b0390a25f8061157a565b61165261165861164b611628600a85906114f9565b600261164561163f611639896113c4565b9361124c565b9161124f565b90611417565b9290611215565b91611215565b9190916116857fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926108db565b9261169a611691610089565b92839283610f59565b0390a25f611558565b50816116b76116b15f6102e6565b9161027e565b1161152d565b5f90565b6116c96116bd565b506116d2611812565b90565b5490565b90565b6116f06116eb6116f5926116d9565b61024b565b61027e565b90565b61170761170d9193929361027e565b9261027e565b820391821161171857565b6102ad565b90565b60301c90565b60018060d01b031690565b61173d61174291611720565b611726565b90565b61174f9054611731565b90565b61176661176161176b92610248565b61024b565b61120a565b90565b611776611311565b506117825f82016116d5565b8061179561178f5f6102e6565b9161027e565b145f146117ab5750506117a75f611752565b5b90565b6117d85f916117d36117cd846117de9601926117c760016116dc565b906116f8565b9161171d565b611827565b01611745565b6117a8565b916118075f61180c946117f4611311565b506117fd611311565b500192919261171d565b611a2c565b91909190565b61181a6116bd565b5061182443611bed565b90565b5f5260205f200190565b5490565b61183f60406100d8565b90565b65ffffffffffff1690565b9061185790611842565b9052565b906118659061120a565b9052565b5f5260205f2090565b634e487b7160e01b5f52603260045260245ffd5b61188f81611831565b8210156118a9576118a1600191611869565b910201905f90565b611872565b6118b89051611842565b90565b906118cc65ffffffffffff91610302565b9181191691161790565b6118ea6118e56118ef92611842565b61024b565b611842565b90565b90565b9061190a611905611911926118d6565b6118f2565b82546118bb565b9055565b61191f905161120a565b90565b60301b90565b9061193a65ffffffffffff1991611922565b9181191691161790565b61195861195361195d9261120a565b61024b565b61120a565b90565b90565b9061197861197361197f92611944565b611960565b8254611928565b9055565b906119ad60205f6119b3946119a582820161199f8488016118ae565b906118f5565b019201611915565b90611963565b565b91906119c6576119c491611983565b565b610653565b90815491680100000000000000008310156119fb57826119f39160016119f995018155611886565b906119b5565b565b61009d565b65ffffffffffff1690565b611a17611a1c91610c33565b611a00565b90565b611a299054611a0b565b90565b90929192611a38611311565b50611a41611311565b50611a4b82611831565b80611a5e611a585f6102e6565b9161027e565b115f14611b2e57611a8490611a7e8491611a7860016116dc565b906116f8565b90611827565b90611a905f8301611a1f565b92611a9c5f8401611745565b9380611ab0611aaa85611842565b91611842565b11611b1257611ac7611ac184611842565b91611842565b145f14611ae2575050611add905f859101611963565b5b9190565b611b0d9250611b0886611aff611af6611835565b945f860161184d565b6020840161185b565b6119cb565b611ade565b5f632520601d60e01b815280611b2a60048201610276565b0390fd5b50611b5991611b5485611b4b611b42611835565b945f860161184d565b6020840161185b565b6119cb565b611b625f611752565b9190565b611b7a611b75611b7f92611842565b61024b565b61027e565b90565b90565b611b99611b94611b9e92611b82565b61024b565b611356565b90565b611baa90611b85565b9052565b916020611bcf929493611bc860408201965f830190611ba1565b0190610b68565b565b611be5611be0611bea9261027e565b61024b565b611842565b90565b611bf56116bd565b5080611c0f611c0965ffffffffffff611b66565b9161027e565b11611c2057611c1d90611bd1565b90565b6030611c3c5f9283926306dfcc6560e41b845260048401611bae565b0390fd5b634e487b7160e01b5f52605160045260245ffd5b91909180600114611c7357600203611c4057611c6f91611482565b905b565b50611c7d9161133d565b90611c7156fe60806040526004361015610013575b611491565b61001d5f356102fc565b806301ffc9a7146102f757806306fdde03146102f2578063095ea7b3146102ed57806318160ddd146102e857806323b872dd146102e3578063248a9ca3146102de5780632f2ff15d146102d9578063313ce567146102d45780633644e515146102cf57806336568abe146102ca5780633a46b1a8146102c557806340c10f19146102c05780634bdd36ce146102bb5780634bf5d7e9146102b65780634f1bfc9e146102b1578063587cde1e146102ac5780635c19a95c146102a75780636fcfff45146102a257806370a082311461029d57806379cc6790146102985780637a8cd156146102935780637ecebe001461028e57806383f1211b146102895780638426adf214610284578063844c90261461027f57806384b0196e1461027a5780638a542521146102755780638d3343d6146102705780638e539e8c1461026b578063902d55a51461026657806391d148541461026157806391ddadf41461025c57806395d89b41146102575780639ab24eb0146102525780639b7ef64b1461024d578063a217fddf14610248578063a9059cbb14610243578063aa082a9d1461023e578063b0ca253e14610239578063bb4d443614610234578063c02ae7541461022f578063c3cda5201461022a578063d505accf14610225578063d547741f14610220578063dd62ed3e1461021b5763f1127ed80361000e5761145b565b611377565b611316565b6112dc565b611232565b611176565b611141565b61110b565b6110d6565b611064565b61102f565b610fbf565b610f48565b610f13565b610ede565b610e7b565b610e46565b610dcf565b610d9a565b610d36565b610ccb565b610b86565b610b33565b610ada565b610aa5565b610a70565b610a3c565b610a07565b6109d2565b610974565b61093f565b6108ca565b610858565b610823565b6107ef565b6107b9565b610785565b610750565b61071b565b6106bf565b610658565b6105bc565b61054d565b6104f5565b610433565b610384565b60e01c90565b60405190565b5f80fd5b5f80fd5b63ffffffff60e01b1690565b61032581610310565b0361032c57565b5f80fd5b9050359061033d8261031c565b565b9060208282031261035857610355915f01610330565b90565b61030c565b151590565b61036b9061035d565b9052565b9190610382905f60208501940190610362565b565b346103b4576103b061039f61039a36600461033f565b61152e565b6103a7610302565b9182918261036f565b0390f35b610308565b5f9103126103c357565b61030c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b61040961041260209361041793610400816103c8565b938480936103cc565b958691016103d5565b6103e0565b0190565b6104309160208201915f8184039101526103ea565b90565b34610463576104433660046103b9565b61045f61044e6116c7565b610456610302565b9182918261041b565b0390f35b610308565b60018060a01b031690565b61047c90610468565b90565b61048881610473565b0361048f57565b5f80fd5b905035906104a08261047f565b565b90565b6104ae816104a2565b036104b557565b5f80fd5b905035906104c6826104a5565b565b91906040838203126104f057806104e46104ed925f8601610493565b936020016104b9565b90565b61030c565b346105265761052261051161050b3660046104c8565b906116dd565b610519610302565b9182918261036f565b0390f35b610308565b610534906104a2565b9052565b919061054b905f6020850194019061052b565b565b3461057d5761055d3660046103b9565b610579610568611729565b610570610302565b91829182610538565b0390f35b610308565b90916060828403126105b7576105b461059d845f8501610493565b936105ab8160208601610493565b936040016104b9565b90565b61030c565b346105ed576105e96105d86105d2366004610582565b9161173f565b6105e0610302565b9182918261036f565b0390f35b610308565b90565b6105fe816105f2565b0361060557565b5f80fd5b90503590610616826105f5565b565b906020828203126106315761062e915f01610609565b90565b61030c565b61063f906105f2565b9052565b9190610656905f60208501940190610636565b565b346106885761068461067361066e366004610618565b6117b8565b61067b610302565b91829182610643565b0390f35b610308565b91906040838203126106b557806106a96106b2925f8601610609565b93602001610493565b90565b61030c565b5f0190565b346106ee576106d86106d236600461068d565b90611804565b6106e0610302565b806106ea816106ba565b0390f35b610308565b60ff1690565b610702906106f3565b9052565b9190610719905f602085019401906106f9565b565b3461074b5761072b3660046103b9565b610747610736611833565b61073e610302565b91829182610706565b0390f35b610308565b34610780576107603660046103b9565b61077c61076b611849565b610773610302565b91829182610643565b0390f35b610308565b346107b45761079e61079836600461068d565b9061185d565b6107a6610302565b806107b0816106ba565b0390f35b610308565b346107ea576107e66107d56107cf3660046104c8565b9061190e565b6107dd610302565b91829182610538565b0390f35b610308565b3461081e576108086108023660046104c8565b90611a95565b610810610302565b8061081a816106ba565b0390f35b610308565b34610853576108333660046103b9565b61084f61083e611ac6565b610846610302565b91829182610538565b0390f35b610308565b34610888576108683660046103b9565b610884610873611b85565b61087b610302565b9182918261041b565b0390f35b610308565b90565b90565b6108a76108a26108ac9261088d565b610890565b6104a2565b90565b6108bc6301e13380610893565b90565b6108c76108af565b90565b346108fa576108da3660046103b9565b6108f66108e56108bf565b6108ed610302565b91829182610538565b0390f35b610308565b9060208282031261091857610915915f01610493565b90565b61030c565b61092690610473565b9052565b919061093d905f6020850194019061091d565b565b3461096f5761096b61095a6109553660046108ff565b611c21565b610962610302565b9182918261092a565b0390f35b610308565b346109a25761098c6109873660046108ff565b611c40565b610994610302565b8061099e816106ba565b0390f35b610308565b63ffffffff1690565b6109b9906109a7565b9052565b91906109d0905f602085019401906109b0565b565b34610a02576109fe6109ed6109e83660046108ff565b611c57565b6109f5610302565b918291826109bd565b0390f35b610308565b34610a3757610a33610a22610a1d3660046108ff565b611c82565b610a2a610302565b91829182610538565b0390f35b610308565b34610a6b57610a55610a4f3660046104c8565b90611db7565b610a5d610302565b80610a67816106ba565b0390f35b610308565b34610aa057610a803660046103b9565b610a9c610a8b611dc3565b610a93610302565b91829182610538565b0390f35b610308565b34610ad557610ad1610ac0610abb3660046108ff565b611e3b565b610ac8610302565b91829182610538565b0390f35b610308565b34610b0a57610aea3660046103b9565b610b06610af5611e50565b610afd610302565b9182918261036f565b0390f35b610308565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610b6357610b433660046103b9565b610b5f610b4e610b0f565b610b56610302565b91829182610538565b0390f35b610308565b90602082820312610b8157610b7e915f016104b9565b90565b61030c565b34610bb457610b9e610b99366004610b68565b612058565b610ba6610302565b80610bb0816106ba565b0390f35b610308565b60ff60f81b1690565b610bcb90610bb9565b9052565b5190565b60209181520190565b60200190565b610beb906104a2565b9052565b90610bfc81602093610be2565b0190565b60200190565b90610c23610c1d610c1684610bcf565b8093610bd3565b92610bdc565b905f5b818110610c335750505090565b909192610c4c610c466001928651610bef565b94610c00565b9101919091610c26565b93959194610ca7610c9c610cbb95610c8e610cb195610cc89c9a610c8160e08c01925f8d0190610bc2565b8a820360208c01526103ea565b9088820360408a01526103ea565b97606087019061052b565b608085019061091d565b60a0830190610636565b60c0818403910152610c06565b90565b34610d0257610cdb3660046103b9565b610cfe610ce66120e0565b93610cf5979597939193610302565b97889788610c56565b0390f35b610308565b7f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a67490565b610d33610d07565b90565b34610d6657610d463660046103b9565b610d62610d51610d2b565b610d59610302565b91829182610643565b0390f35b610308565b7f9b12e0c5707e494915e58b0564f18aaad9b74ac69bfc815a1edadc8e4bd032eb90565b610d97610d6b565b90565b34610dca57610daa3660046103b9565b610dc6610db5610d8f565b610dbd610302565b91829182610643565b0390f35b610308565b34610dff57610dfb610dea610de5366004610b68565b61216a565b610df2610302565b91829182610538565b0390f35b610308565b90565b610e1b610e16610e2092610e04565b610890565b6104a2565b90565b610e386b033b2e3c9fd0803ce8000000610e07565b90565b610e43610e23565b90565b34610e7657610e563660046103b9565b610e72610e61610e3b565b610e69610302565b91829182610538565b0390f35b610308565b34610eac57610ea8610e97610e9136600461068d565b906121d8565b610e9f610302565b9182918261036f565b0390f35b610308565b65ffffffffffff1690565b610ec590610eb1565b9052565b9190610edc905f60208501940190610ebc565b565b34610f0e57610eee3660046103b9565b610f0a610ef9612206565b610f01610302565b91829182610ec9565b0390f35b610308565b34610f4357610f233660046103b9565b610f3f610f2e61221a565b610f36610302565b9182918261041b565b0390f35b610308565b34610f7857610f74610f63610f5e3660046108ff565b612230565b610f6b610302565b91829182610538565b0390f35b610308565b90565b610f94610f8f610f9992610f7d565b610890565b6104a2565b90565b610fb16b02e87669c308736a04000000610f80565b90565b610fbc610f9c565b90565b34610fef57610fcf3660046103b9565b610feb610fda610fb4565b610fe2610302565b91829182610538565b0390f35b610308565b90565b5f1b90565b61101061100b61101592610ff4565b610ff7565b6105f2565b90565b6110215f610ffc565b90565b61102c611018565b90565b3461105f5761103f3660046103b9565b61105b61104a611024565b611052610302565b91829182610643565b0390f35b610308565b346110955761109161108061107a3660046104c8565b9061225f565b611088610302565b9182918261036f565b0390f35b610308565b1c90565b90565b6110b19060086110b6930261109a565b61109e565b90565b906110c491546110a1565b90565b6110d3600c5f906110b9565b90565b34611106576110e63660046103b9565b6111026110f16110c7565b6110f9610302565b91829182610538565b0390f35b610308565b3461113c576111386111276111213660046104c8565b90612281565b61112f610302565b91829182610538565b0390f35b610308565b346111715761116d61115c6111573660046108ff565b612297565b611164610302565b91829182610538565b0390f35b610308565b346111a6576111863660046103b9565b6111a26111916122ac565b611199610302565b91829182610538565b0390f35b610308565b6111b4816106f3565b036111bb57565b5f80fd5b905035906111cc826111ab565b565b909160c08284031261122d576111e6835f8401610493565b926111f481602085016104b9565b9261120282604083016104b9565b9261122a61121384606085016111bf565b936112218160808601610609565b9360a001610609565b90565b61030c565b34611267576112516112453660046111ce565b9493909392919261232c565b611259610302565b80611263816106ba565b0390f35b610308565b60e0818303126112d757611282825f8301610493565b926112908360208401610493565b9261129e81604085016104b9565b926112ac82606083016104b9565b926112d46112bd84608085016111bf565b936112cb8160a08601610609565b9360c001610609565b90565b61030c565b34611311576112fb6112ef36600461126c565b95949094939193612480565b611303610302565b8061130d816106ba565b0390f35b610308565b346113455761132f61132936600461068d565b9061259e565b611337610302565b80611341816106ba565b0390f35b610308565b9190604083820312611372578061136661136f925f8601610493565b93602001610493565b90565b61030c565b346113a8576113a461139361138d36600461134a565b906125c0565b61139b610302565b91829182610538565b0390f35b610308565b6113b6816109a7565b036113bd57565b5f80fd5b905035906113ce826113ad565b565b91906040838203126113f857806113ec6113f5925f8601610493565b936020016113c1565b90565b61030c565b61140690610eb1565b9052565b60018060d01b031690565b61141e9061140a565b9052565b906020806114449361143a5f8201515f8601906113fd565b0151910190611415565b565b9190611459905f60408501940190611422565b565b3461148c576114886114776114713660046113d0565b9061262e565b61147f610302565b91829182611446565b0390f35b610308565b5f80fd5b5f90565b634e487b7160e01b5f52601160045260245ffd5b6114b96114bf9161140a565b9161140a565b019060018060d01b0382116114d057565b611499565b906114e8916114e2611495565b506114ad565b90565b6114f76114fd9161140a565b9161140a565b90039060018060d01b03821161150f57565b611499565b9061152791611521611495565b506114eb565b90565b5f90565b61153661152a565b508061155161154b637965db0b60e01b610310565b91610310565b1490811561155e575b5090565b6115689150612644565b5f61155a565b606090565b634e487b7160e01b5f52602260045260245ffd5b90600160028304921680156115a7575b60208310146115a257565b611573565b91607f1691611597565b60209181520190565b5f5260205f2090565b905f92918054906115dd6115d683611587565b80946115b1565b916001811690815f1461163457506001146115f8575b505050565b61160591929394506115ba565b915f925b81841061161c57505001905f80806115f3565b60018160209295939554848601520191019290611609565b92949550505060ff19168252151560200201905f80806115f3565b90611659916115c3565b90565b634e487b7160e01b5f52604160045260245ffd5b9061167a906103e0565b810190811067ffffffffffffffff82111761169457604052565b61165c565b906116b96116b2926116a9610302565b9384809261164f565b0383611670565b565b6116c490611699565b90565b6116cf61156e565b506116da60036116bb565b90565b6116fa916116e961152a565b506116f261266a565b919091612677565b600190565b5f90565b5f1c90565b61171461171991611703565b61109e565b90565b6117269054611708565b90565b6117316116ff565b5061173c600261171c565b90565b916117699261174c61152a565b5061176161175861266a565b829084916126c7565b919091612753565b600190565b5f90565b61177b906105f2565b90565b9061178890611772565b5f5260205260405f2090565b90565b6117a36117a891611703565b611794565b90565b6117b59054611797565b90565b60016117d16117d7926117c961176e565b50600561177e565b016117ab565b90565b906117f5916117f06117eb826117b8565b6127f0565b6117f7565b565b9061180191612849565b50565b9061180e916117da565b565b5f90565b90565b61182b61182661183092611814565b610890565b6106f3565b90565b61183b611810565b506118466012611817565b90565b61185161176e565b5061185a6128f5565b90565b908061187861187261186d61266a565b610473565b91610473565b0361188957611886916129af565b50565b5f63334bd91960e11b8152806118a1600482016106ba565b0390fd5b6118b96118b46118be92610468565b610890565b610468565b90565b6118ca906118a5565b90565b6118d6906118c1565b90565b906118e3906118cd565b5f5260205260405f2090565b90565b61190661190161190b9261140a565b610890565b6104a2565b90565b6119459161193a61193461192f611940946119276116ff565b50600a6118d9565b6118ef565b91612a90565b90612ba5565b6118f2565b90565b906119629161195d611958610d6b565b6127f0565b6119cd565b565b61197861197361197d92610ff4565b610890565b610468565b90565b61198990611964565b90565b6119a061199b6119a592610ff4565b610890565b6104a2565b90565b6119b76119bd919392936104a2565b926104a2565b82018092116119c857565b611499565b90816119e96119e36119de5f611980565b610473565b91610473565b14611a795780611a016119fb5f61198c565b916104a2565b14611a5d57611a18611a11611729565b82906119a8565b611a31611a2b611a26610e23565b6104a2565b916104a2565b11611a4157611a3f91612ccc565b565b5f63177e3fc360e01b815280611a59600482016106ba565b0390fd5b5f631f2a200560e01b815280611a75600482016106ba565b0390fd5b5f63d92e233d60e01b815280611a91600482016106ba565b0390fd5b90611a9f91611948565b565b611ab0611ab6919392936104a2565b926104a2565b8203918211611ac157565b611499565b611ace6116ff565b50611ae8611ada610e23565b611ae2611729565b90611aa1565b90565b90611afe611af7610302565b9283611670565b565b67ffffffffffffffff8111611b1e57611b1a6020916103e0565b0190565b61165c565b90611b35611b3083611b00565b611aeb565b918252565b5f7f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c74000000910152565b611b6b601d611b23565b90611b7860208301611b3a565b565b611b82611b61565b90565b611b8d61156e565b50611b96612206565b611baf611ba9611ba4612d2a565b610eb1565b91610eb1565b03611bbf57611bbc611b7a565b90565b5f6301bfc1c560e61b815280611bd7600482016106ba565b0390fd5b5f90565b90611be9906118cd565b5f5260205260405f2090565b60018060a01b031690565b611c0c611c1191611703565b611bf5565b90565b611c1e9054611c00565b90565b611c38611c3d91611c30611bdb565b506009611bdf565b611c14565b90565b611c5190611c4c61266a565b612d7d565b565b5f90565b611c6990611c63611c53565b50612e08565b90565b90611c76906118cd565b5f5260205260405f2090565b611c98611c9d91611c916116ff565b505f611c6c565b61171c565b90565b90611cba91611cb5611cb0610d07565b6127f0565b611cbc565b565b80611cd7611cd1611ccc5f611980565b610473565b91610473565b14611d9b5781611cef611ce95f61198c565b916104a2565b14611d7f57611d05611cff611e50565b1561035d565b611d6357611d14818390612e37565b3390611d5e611d4c611d467fbef4f81c1814c641ede85ebaacf19d048b2c5b55980adfa6ef0f956c651335a2936118cd565b936118cd565b93611d55610302565b91829182610538565b0390a3565b5f63b8b5ca2d60e01b815280611d7b600482016106ba565b0390fd5b5f631f2a200560e01b815280611d97600482016106ba565b0390fd5b5f63d92e233d60e01b815280611db3600482016106ba565b0390fd5b90611dc191611ca0565b565b611dcb6116ff565b50611dd6600c61171c565b611de8611de25f61198c565b916104a2565b148015611e17575b611e0b57611e08611e01600c61171c565b4290611aa1565b90565b611e145f61198c565b90565b5042611e34611e2e611e29600c61171c565b6104a2565b916104a2565b1015611df0565b611e4d90611e476116ff565b50612e96565b90565b611e5861152a565b50611e63600c61171c565b611e75611e6f5f61198c565b916104a2565b141580611e80575b90565b5042611e9d611e97611e92600c61171c565b6104a2565b916104a2565b10611e7d565b611ebc90611eb7611eb2611018565b6127f0565b611f36565b565b90611eca5f1991610ff7565b9181191691161790565b611ee8611ee3611eed926104a2565b610890565b6104a2565b90565b90565b90611f08611f03611f0f92611ed4565b611ef0565b8254611ebe565b9055565b916020611f34929493611f2d60408201965f83019061052b565b019061052b565b565b611f40600c61171c565b611f52611f4c5f61198c565b916104a2565b0361203c5780611f6a611f64426104a2565b916104a2565b11156120205780611fa3611f9d7f00000000000000000000000000000000000000000000000000000000000000006104a2565b916104a2565b1161200457611fb2600c61171c565b611fbd82600c611ef3565b903390611fea7fdd6896dcf1d4b311cca87dd19bbba2ea9ce2f867c1568878a0438a66a1afeeec926118cd565b92611fff611ff6610302565b92839283611f13565b0390a2565b5f63ef69af6560e01b81528061201c600482016106ba565b0390fd5b5f63a565835360e01b815280612038600482016106ba565b0390fd5b5f6337e978d360e11b815280612054600482016106ba565b0390fd5b61206190611ea3565b565b5f90565b606090565b612075906118c1565b90565b67ffffffffffffffff81116120905760208091020190565b61165c565b906120a76120a283612078565b611aeb565b918252565b369037565b906120d66120be83612095565b926020806120cc8693612078565b92019103906120ac565b565b600f60f81b90565b6120e8612063565b506120f161156e565b506120fa61156e565b506121036116ff565b5061210c611bdb565b5061211561176e565b5061211e612067565b50612127612eae565b90612130612eee565b90469061213c3061206c565b906121465f610ffc565b906121586121535f61198c565b6120b1565b906121616120d8565b96959493929190565b612193612198916121796116ff565b5061218d612187600b6118ef565b91612a90565b90612ba5565b6118f2565b90565b906121a5906118cd565b5f5260205260405f2090565b60ff1690565b6121c36121c891611703565b6121b1565b90565b6121d590546121b7565b90565b6121ff915f6121f46121fa936121ec61152a565b50600561177e565b0161219b565b6121cb565b90565b5f90565b61220e612202565b50612217612d2a565b90565b61222261156e565b5061222d60046116bb565b90565b61225761225261224d61225c936122456116ff565b50600a6118d9565b6118ef565b612f2e565b6118f2565b90565b61227c9161226b61152a565b5061227461266a565b919091612753565b600190565b906122949161228e6116ff565b5061190e565b90565b6122a9906122a36116ff565b50612230565b90565b6122b46116ff565b506122bd611729565b90565b7fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf90565b6123196123209461230f606094989795612305608086019a5f870190610636565b602085019061091d565b604083019061052b565b019061052b565b565b60200190565b5190565b9395949092919542612346612340896104a2565b916104a2565b116123bf57916123b1916123b8936123a86123bd98996123906123676122c0565b6123818b938b612375610302565b958694602086016122e4565b60208201810382520382611670565b6123a261239c82612328565b91612322565b20612fa3565b92909192612fc0565b918261300a565b612d7d565b565b6123da875f918291632341d78760e11b835260048301610538565b0390fd5b7f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c990565b919461244a6124549298979561244060a09661243661245b9a61242c60c08a019e5f8b0190610636565b602089019061091d565b604087019061091d565b606085019061052b565b608083019061052b565b019061052b565b565b91602061247e92949361247760408201965f83019061091d565b019061091d565b565b96959193929490944261249b612495836104a2565b916104a2565b11612555579061250461250d9493926124ec6124b56123de565b6124dd8c80948c916124c78d9161305c565b91926124d1610302565b97889660208801612402565b60208201810382520382611670565b6124fe6124f882612328565b91612322565b20612fa3565b92909192612fc0565b8061252061251a87610473565b91610473565b0361253557506125339293919091612677565b565b84906125515f9283926325c0072360e11b84526004840161245d565b0390fd5b612570905f91829163313c898160e11b835260048301610538565b0390fd5b9061258f9161258a612585826117b8565b6127f0565b612591565b565b9061259b916129af565b50565b906125a891612574565b565b906125b4906118cd565b5f5260205260405f2090565b6125e5916125db6125e0926125d36116ff565b5060016125aa565b611c6c565b61171c565b90565b6125f26040611aeb565b90565b5f90565b5f90565b6126056125e8565b90602080836126126125f5565b81520161261d6125f9565b81525050565b61262b6125fd565b90565b906126419161263b612623565b5061308f565b90565b61264c61152a565b506126666126606301ffc9a760e01b610310565b91610310565b1490565b612672611bdb565b503390565b9161268592916001926130b7565b565b6040906126b06126b794969593966126a660608401985f85019061091d565b602083019061052b565b019061052b565b565b906126c491036104a2565b90565b9291926126d58183906125c0565b90816126ea6126e45f196104a2565b916104a2565b106126f7575b5050509050565b8161270a612704876104a2565b916104a2565b1061273057612727939461271f9193926126b9565b905f926130b7565b805f80806126f0565b5061274f849291925f938493637dc7a0d960e11b855260048501612687565b0390fd5b918261276f6127696127645f611980565b610473565b91610473565b146127c9578161278f6127896127845f611980565b610473565b91610473565b146127a2576127a0929190916131c6565b565b6127c56127ae5f611980565b5f91829163ec442f0560e01b83526004830161092a565b0390fd5b6127ec6127d55f611980565b5f918291634b637e8f60e11b83526004830161092a565b0390fd5b612802906127fc61266a565b90613293565b565b9061281060ff91610ff7565b9181191691161790565b6128239061035d565b90565b90565b9061283e6128396128459261281a565b612826565b8254612804565b9055565b61285161152a565b506128666128608284906121d8565b1561035d565b5f146128ef5761288e60016128895f6128816005869061177e565b01859061219b565b612829565b9061289761266a565b906128d46128ce6128c87f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d95611772565b926118cd565b926118cd565b926128dd610302565b806128e7816106ba565b0390a4600190565b50505f90565b6128fd61176e565b506129073061206c565b6129396129337f0000000000000000000000000000000000000000000000000000000000000000610473565b91610473565b1480612975575b5f1461296a577f000000000000000000000000000000000000000000000000000000000000000090565b61297261333f565b90565b50466129a96129a37f00000000000000000000000000000000000000000000000000000000000000006104a2565b916104a2565b14612940565b6129b761152a565b506129c38183906121d8565b5f14612a4b576129ea5f6129e55f6129dd6005869061177e565b01859061219b565b612829565b906129f361266a565b90612a30612a2a612a247ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b95611772565b926118cd565b926118cd565b92612a39610302565b80612a43816106ba565b0390a4600190565b50505f90565b612a65612a60612a6a92610eb1565b610890565b6104a2565b90565b916020612a8e929493612a8760408201965f83019061052b565b0190610ebc565b565b612a98612202565b50612aa1612206565b81612ab4612aae83612a51565b916104a2565b1015612ac75750612ac490613448565b90565b90612ae25f928392637669fc0f60e11b845260048401612a6d565b0390fd5b5490565b90565b612b01612afc612b0692612aea565b610890565b6104a2565b90565b90565b65ffffffffffff1690565b612b23612b2891611703565b612b0c565b90565b612b359054612b17565b90565b90565b612b4f612b4a612b5492612b38565b610890565b6104a2565b90565b60301c90565b60018060d01b031690565b612b74612b7991612b57565b612b5d565b90565b612b869054612b68565b90565b612b9d612b98612ba292610ff4565b610890565b61140a565b90565b90612bf990612bb2611495565b50612bbe5f8401612ae6565b612bc75f61198c565b908080612bdd612bd76005612aed565b916104a2565b11612c5a575b5090612bf45f860193919293612b09565b613a9b565b80612c0c612c065f61198c565b916104a2565b145f14612c22575050612c1e5f612b89565b5b90565b612c4f5f91612c4a612c4484612c55960192612c3e6001612b3b565b90611aa1565b91612b09565b613a91565b01612b7c565b612c1f565b80612c68612c6e9291613726565b90611aa1565b9083612ca0612c9a612c955f612c8f818c01612c8a8991612b09565b613a91565b01612b2b565b610eb1565b91610eb1565b105f14612cb15750905b905f612be3565b9150612cc790612cc16001612b3b565b906119a8565b612caa565b80612ce7612ce1612cdc5f611980565b610473565b91610473565b14612d0357612d0191612cf95f611980565b9190916131c6565b565b612d26612d0f5f611980565b5f91829163ec442f0560e01b83526004830161092a565b0390fd5b612d32612202565b50612d3c43613448565b90565b90612d5060018060a01b0391610ff7565b9181191691161790565b90565b90612d72612d6d612d79926118cd565b612d5a565b8254612d3f565b9055565b90612e0691612e00612d8e82611c21565b612da384612d9e60098690611bdf565b612d5d565b82818590612de3612ddd612dd77f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f956118cd565b926118cd565b926118cd565b92612dec610302565b80612df6816106ba565b0390a49291613b2a565b91613b42565b565b612e2f612e2a612e25612e3493612e1d611c53565b50600a6118d9565b6118ef565b613cf0565b613d6f565b90565b9081612e53612e4d612e485f611980565b610473565b91610473565b14612e6f57612e6d9190612e665f611980565b90916131c6565b565b612e92612e7b5f611980565b5f918291634b637e8f60e11b83526004830161092a565b0390fd5b612ea890612ea26116ff565b50613dc0565b90565b90565b612eb661156e565b50612eeb7f0000000000000000000000000000000000000000000000000000000000000000612ee56006612eab565b90613edb565b90565b612ef661156e565b50612f2b7f0000000000000000000000000000000000000000000000000000000000000000612f256007612eab565b90613edb565b90565b612f36611495565b50612f425f8201612ae6565b80612f55612f4f5f61198c565b916104a2565b145f14612f6b575050612f675f612b89565b5b90565b612f985f91612f93612f8d84612f9e960192612f876001612b3b565b90611aa1565b91612b09565b613a91565b01612b7c565b612f68565b612fbd90612faf61176e565b50612fb86128f5565b613f29565b90565b92612fdb92612fe494612fd1611bdb565b5092909192613fef565b9092919261411a565b90565b91602061300892949361300160408201965f83019061091d565b019061052b565b565b6130138161305c565b91613026613020846104a2565b916104a2565b0361302f575050565b6130495f9283926301d4b62360e61b845260048401612fe7565b0390fd5b600161305991016104a2565b90565b613070906130686116ff565b506008611c6c565b61308c61307c8261171c565b916130868361304d565b90611ef3565b90565b906130af6130aa6130b4936130a2612623565b50600a6118d9565b6118ef565b614290565b90565b9092816130d46130ce6130c95f611980565b610473565b91610473565b1461319f57836130f46130ee6130e95f611980565b610473565b91610473565b14613178576131188361311361310c600186906125aa565b8790611c6c565b611ef3565b613122575b505050565b91909161316d61315b6131557f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925936118cd565b936118cd565b93613164610302565b91829182610538565b0390a35f808061311d565b61319b6131845f611980565b5f918291634a1406b160e11b83526004830161092a565b0390fd5b6131c26131ab5f611980565b5f91829163e602df0560e01b83526004830161092a565b0390fd5b91826131e26131dc6131d75f611980565b610473565b91610473565b14158061324d575b6131fd575b6131fb929190916142b1565b565b613205611e50565b8061322c575b156131ef575f6336e278fd60e21b815280613228600482016106ba565b0390fd5b5061324861324261323b610d07565b33906121d8565b1561035d565b61320b565b508161326961326361325e5f611980565b610473565b91610473565b14156131ea565b91602061329192949361328a60408201965f83019061091d565b0190610636565b565b906132a86132a28383906121d8565b1561035d565b6132b0575050565b6132ca5f92839263e2517d3f60e01b845260048401613270565b0390fd5b7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f90565b9095949261333d9461332c6133369261332260809661331860a088019c5f890190610636565b6020870190610636565b6040850190610636565b606083019061052b565b019061091d565b565b61334761176e565b506133506132ce565b6133c77f0000000000000000000000000000000000000000000000000000000000000000916133b87f0000000000000000000000000000000000000000000000000000000000000000466133a33061206c565b916133ac610302565b968795602087016132f2565b60208201810382520382611670565b6133d96133d382612328565b91612322565b2090565b90565b6133f46133ef6133f9926133dd565b610890565b6106f3565b90565b613405906133e0565b9052565b91602061342a92949361342360408201965f8301906133fc565b019061052b565b565b61344061343b613445926104a2565b610890565b610eb1565b90565b613450612202565b508061346a61346465ffffffffffff612a51565b916104a2565b1161347b576134789061342c565b90565b60306134975f9283926306dfcc6560e41b845260048401613409565b0390fd5b90565b6134b26134ad6134b79261349b565b610890565b6104a2565b90565b90565b6134d16134cc6134d6926134ba565b610890565b6106f3565b90565b6134f8906134f26134ec6134fd946106f3565b916104a2565b9061109a565b6104a2565b90565b90565b61351761351261351c92613500565b610890565b6106f3565b90565b1b90565b6135429061353c613536613547946106f3565b916104a2565b9061351f565b6104a2565b90565b90565b61356161355c6135669261354a565b610890565b6104a2565b90565b90565b61358061357b61358592613569565b610890565b6106f3565b90565b90565b61359f61359a6135a492613588565b610890565b6104a2565b90565b90565b6135be6135b96135c3926135a7565b610890565b6106f3565b90565b90565b6135dd6135d86135e2926135c6565b610890565b6104a2565b90565b90565b6135fc6135f7613601926135e5565b610890565b6106f3565b90565b90565b61361b61361661362092613604565b610890565b6104a2565b90565b90565b61363a61363561363f92613623565b610890565b6106f3565b90565b61365661365161365b926135a7565b610890565b6104a2565b90565b90565b61367561367061367a9261365e565b610890565b6106f3565b90565b61369161368c61369692613623565b610890565b6104a2565b90565b6136ad6136a86136b292612b38565b610890565b6106f3565b90565b90565b6136cc6136c76136d1926136b5565b610890565b6104a2565b90565b906136df91026104a2565b90565b634e487b7160e01b5f52601260045260245ffd5b613702613708916104a2565b916104a2565b908115613713570490565b6136e2565b9061372391016104a2565b90565b61372e6116ff565b508061374361373d6001612b3b565b916104a2565b1115613a8e57806139586139356139256139156139056138f56138e56138d56138c56138b56138a58b61389f61389861395e9f6138786138686138889261378a6001612b3b565b90806137a261379c600160801b61349e565b916104a2565b1015613a60575b806137c56137bf6801000000000000000061354d565b916104a2565b1015613a32575b806137e46137de64010000000061358b565b916104a2565b1015613a04575b806138016137fb620100006135c9565b916104a2565b10156139d6575b8061381d613817610100613607565b916104a2565b10156139a8575b806138386138326010613642565b916104a2565b101561397a575b61385261384c600461367d565b916104a2565b1015613961575b61386360036136b8565b6136d4565b6138726001613699565b906134d9565b61388281866136f6565b90613718565b6138926001613699565b906134d9565b80926136f6565b90613718565b6138af6001613699565b906134d9565b6138bf818c6136f6565b90613718565b6138cf6001613699565b906134d9565b6138df818a6136f6565b90613718565b6138ef6001613699565b906134d9565b6138ff81886136f6565b90613718565b61390f6001613699565b906134d9565b61391f81866136f6565b90613718565b61392f6001613699565b906134d9565b9161395261394c6139478580946136f6565b6104a2565b916104a2565b11614341565b906126b9565b90565b6139759061396f6001613699565b90613523565b613859565b6139916139a29161398b6004613626565b906134d9565b9161399c6002613661565b90613523565b9061383f565b6139bf6139d0916139b960086135e8565b906134d9565b916139ca6004613626565b90613523565b90613824565b6139ed6139fe916139e760106135aa565b906134d9565b916139f860086135e8565b90613523565b90613808565b613a1b613a2c91613a15602061356c565b906134d9565b91613a2660106135aa565b90613523565b906137eb565b613a49613a5a91613a436040613503565b906134d9565b91613a54602061356c565b90613523565b906137cc565b613a77613a8891613a7160806134bd565b906134d9565b91613a826040613503565b90613523565b906137a9565b90565b5f5260205f200190565b93919092613aa76116ff565b505b81613abc613ab6836104a2565b916104a2565b1015613b2257613acd82829061438d565b90613ae35f613add888590613a91565b01612b2b565b613af5613aef87610eb1565b91610eb1565b115f14613b055750915b91613aa9565b929150613b1c90613b166001612b3b565b906119a8565b90613aff565b925050915090565b613b3c90613b366116ff565b50611c82565b90565b90565b91909180613b58613b5285610473565b91610473565b141580613cd6575b613b6a575b505050565b80613b85613b7f613b7a5f611980565b610473565b91610473565b03613c46575b5081613ba7613ba1613b9c5f611980565b610473565b91610473565b03613bb3575b80613b65565b613bfa613bed613bf492613bc9600a86906118d9565b90613be7613be1613bdb600193614426565b936118ef565b91613b3f565b90614479565b92906118f2565b916118f2565b919091613c277fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926118cd565b92613c3c613c33610302565b92839283611f13565b0390a25f80613bad565b613c85613c8b613c7e613c5b600a85906118d9565b6002613c78613c72613c6c89614426565b936118ef565b91613b3f565b90614479565b92906118f2565b916118f2565b919091613cb87fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926118cd565b92613ccd613cc4610302565b92839283611f13565b0390a25f613b8b565b5081613cea613ce45f61198c565b916104a2565b11613b60565b5f613d0491613cfd6116ff565b5001612ae6565b90565b613d1b613d16613d20926109a7565b610890565b6104a2565b90565b613d2c9061356c565b9052565b916020613d51929493613d4a60408201965f830190613d23565b019061052b565b565b613d67613d62613d6c926104a2565b610890565b6109a7565b90565b613d77611c53565b5080613d8f613d8963ffffffff613d07565b916104a2565b11613da057613d9d90613d53565b90565b6020613dbc5f9283926306dfcc6560e41b845260048401613d30565b0390fd5b613dd7613ddc91613dcf6116ff565b506008611c6c565b61171c565b90565b90565b613df6613df1613dfb92613ddf565b610ff7565b6105f2565b90565b613e0860ff613de2565b90565b5f5260205f2090565b905f9291805490613e2e613e2783611587565b80946115b1565b916001811690815f14613e855750600114613e49575b505050565b613e569192939450613e0b565b915f925b818410613e6d57505001905f8080613e44565b60018160209295939554848601520191019290613e5a565b92949550505060ff19168252151560200201905f8080613e44565b90613eaa91613e14565b90565b90613ecd613ec692613ebd610302565b93848092613ea0565b0383611670565b565b613ed890613ead565b90565b90613ee461156e565b50613eee82611772565b613f07613f01613efc613dfe565b6105f2565b916105f2565b14155f14613f1c5750613f1990614503565b90565b613f269150613ecf565b90565b604291613f3461176e565b50604051917f19010000000000000000000000000000000000000000000000000000000000008352600283015260228201522090565b5f90565b613f7a613f7f91611703565b611ed4565b90565b90565b613f99613f94613f9e92613f82565b610890565b6104a2565b90565b613fd6613fdd94613fcc606094989795613fc2608086019a5f870190610636565b60208501906106f9565b6040830190610636565b0190610636565b565b613fe7610302565b3d5f823e3d90fd5b939293613ffa611bdb565b50614003613f6a565b5061400c61176e565b5061401685613f6e565b6140486140427f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0613f85565b916104a2565b116140d5579061406b602094955f94939293614062610302565b94859485613fa1565b838052039060015afa156140d0576140835f51610ff7565b8061409e6140986140935f611980565b610473565b91610473565b146140b4575f916140ae5f610ffc565b91929190565b506140be5f611980565b6001916140ca5f610ffc565b91929190565b613fdf565b5050506140e15f611980565b9060039291929190565b634e487b7160e01b5f52602160045260245ffd5b6004111561410957565b6140eb565b90614118826140ff565b565b8061412d6141275f61410e565b9161410e565b145f14614138575050565b8061414c614146600161410e565b9161410e565b145f1461416f575f63f645eedf60e01b81528061416b600482016106ba565b0390fd5b8061418361417d600261410e565b9161410e565b145f146141b1576141ad61419683613f6e565b5f91829163fce698f760e01b835260048301610538565b0390fd5b6141c46141be600361410e565b9161410e565b146141cc5750565b6141e7905f9182916335e2f38360e21b835260048301610643565b0390fd5b634e487b7160e01b5f52603260045260245ffd5b5f5260205f2090565b61421181612ae6565b82101561422b576142236001916141ff565b910201905f90565b6141eb565b9061423a90610eb1565b9052565b906142489061140a565b9052565b906142826142795f61425c6125e8565b9461427361426b838301612b2b565b838801614230565b01612b7c565b6020840161423e565b565b61428d9061424c565b90565b6142ae915f6142a8926142a1612623565b5001614208565b50614284565b90565b92916142bf84838391614533565b836142da6142d46142cf5f611980565b610473565b91610473565b146142ef575b6142ed92939190916146bd565b565b6142f7611729565b936143006146a2565b948061431461430e886104a2565b916104a2565b11614321575093506142e0565b859061433d5f928392630e58ae9360e11b845260048401611f13565b0390fd5b6143496116ff565b50151590565b61436361435e6143689261365e565b610890565b6104a2565b90565b61437761437d916104a2565b916104a2565b908115614388570490565b6136e2565b6143b26143b89261439c6116ff565b5082811692186143ac600261434f565b9061436b565b906119a8565b90565b90565b6143d26143cd6143d7926143bb565b610890565b6106f3565b90565b6143e3906143be565b9052565b91602061440892949361440160408201965f8301906143da565b019061052b565b565b61441e614419614423926104a2565b610890565b61140a565b90565b61442e611495565b508061444861444260018060d01b036118f2565b916104a2565b11614459576144569061440a565b90565b60d06144755f9283926306dfcc6560e41b8452600484016143e7565b0390fd5b906144af6144b5939261448a611495565b50614493611495565b5080936144a86144a1612206565b9492612f2e565b9091614b38565b9161477c565b91909190565b6144cf6144ca6144d492613569565b610890565b6104a2565b90565b369037565b906145016144e983611b23565b926020806144f78693611b00565b92019103906144d7565b565b61450b61156e565b50614515816147e6565b9061452861452360206144bb565b6144dc565b918252602082015290565b9190918061455161454b6145465f611980565b610473565b91610473565b145f146146325761457561456e83614569600261171c565b6119a8565b6002611ef3565b5b8261459161458b6145865f611980565b610473565b91610473565b145f14614606576145b56145ae836145a9600261171c565b6126b9565b6002611ef3565b5b9190916146016145ef6145e97fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef936118cd565b936118cd565b936145f8610302565b91829182610538565b0390a3565b61462d826146276146185f8790611c6c565b916146228361171c565b613718565b90611ef3565b6145b6565b6146456146405f8390611c6c565b61171c565b80614658614652856104a2565b916104a2565b106146805761466b61467b9184906126b9565b6146765f8490611c6c565b611ef3565b614576565b9061469e9091925f93849363391434e360e21b855260048501612687565b0390fd5b6146aa6116ff565b506146ba60018060d01b036118f2565b90565b9161471561470f61471c94806146e36146dd6146d85f611980565b610473565b91610473565b1461474d575b846147046146fe6146f95f611980565b610473565b91610473565b1461471e575b611c21565b92611c21565b9091613b42565b565b614746600b600261474061473a61473489614426565b936118ef565b91613b3f565b90614479565b505061470a565b614775600b600161476f61476961476389614426565b936118ef565b91613b3f565b90614479565b50506146e9565b916147a05f6147a59461478d611495565b50614796611495565b5001929192612b09565b6149ea565b91909190565b6147bf6147ba6147c492613ddf565b610890565b6104a2565b90565b90565b6147de6147d96147e3926147c7565b610890565b6104a2565b90565b6147fb614800916147f56116ff565b50611772565b613f6e565b61480a60ff6147ab565b168061481f614819601f6147ca565b916104a2565b116148275790565b5f632cd44ac360e21b81528061483f600482016106ba565b0390fd5b5490565b6148516040611aeb565b90565b5f5260205f2090565b61486681614843565b82101561488057614878600191614854565b910201905f90565b6141eb565b634e487b7160e01b5f525f60045260245ffd5b6148a29051610eb1565b90565b906148b665ffffffffffff91610ff7565b9181191691161790565b6148d46148cf6148d992610eb1565b610890565b610eb1565b90565b90565b906148f46148ef6148fb926148c0565b6148dc565b82546148a5565b9055565b614909905161140a565b90565b60301b90565b9061492465ffffffffffff199161490c565b9181191691161790565b61494261493d6149479261140a565b610890565b61140a565b90565b90565b9061496261495d6149699261492e565b61494a565b8254614912565b9055565b9061499760205f61499d9461498f828201614989848801614898565b906148df565b0192016148ff565b9061494d565b565b91906149b0576149ae9161496d565b565b614885565b90815491680100000000000000008310156149e557826149dd9160016149e39501815561485d565b9061499f565b565b61165c565b909291926149f6611495565b506149ff611495565b50614a0982614843565b80614a1c614a165f61198c565b916104a2565b115f14614aec57614a4290614a3c8491614a366001612b3b565b90611aa1565b90613a91565b90614a4e5f8301612b2b565b92614a5a5f8401612b7c565b9380614a6e614a6885610eb1565b91610eb1565b11614ad057614a85614a7f84610eb1565b91610eb1565b145f14614aa0575050614a9b905f85910161494d565b5b9190565b614acb9250614ac686614abd614ab4614847565b945f8601614230565b6020840161423e565b6149b5565b614a9c565b5f632520601d60e01b815280614ae8600482016106ba565b0390fd5b50614b1791614b1285614b09614b00614847565b945f8601614230565b6020840161423e565b6149b5565b614b205f612b89565b9190565b634e487b7160e01b5f52605160045260245ffd5b91909180600114614b5757600203614b2457614b5391611514565b905b565b50614b61916114d5565b90614b5556
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\x80`@R4a\0\x84Wa\0\x1Ba\0\x15a\x01XV[\x90a\x03\xBEV[a\0#a\0\x89V[aKga\x1C\x84\x829`\x80Q\x81a)H\x01R`\xA0Q\x81a)\x7F\x01R`\xC0Q\x81a)\x0F\x01R`\xE0Q\x81a3U\x01Ra\x01\0Q\x81a3z\x01Ra\x01 Q\x81a.\xBC\x01Ra\x01@Q\x81a.\xFC\x01Ra\x01`Q\x81\x81\x81a\x0B\x11\x01Ra\x1Fy\x01RaKg\x90\xF3[a\0\x8FV[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0\xBB\x90a\0\x93V[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\xD3W`@RV[a\0\x9DV[\x90a\0\xEBa\0\xE4a\0\x89V[\x92\x83a\0\xB1V[V[_\x80\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x01\x05\x90a\0\xF1V[\x90V[a\x01\x11\x81a\0\xFCV[\x03a\x01\x18WV[_\x80\xFD[\x90PQ\x90a\x01)\x82a\x01\x08V[V[\x91\x90`@\x83\x82\x03\x12a\x01SW\x80a\x01Ga\x01P\x92_\x86\x01a\x01\x1CV[\x93` \x01a\x01\x1CV[\x90V[a\0\xEDV[a\x01vag\xEB\x808\x03\x80a\x01k\x81a\0\xD8V[\x92\x839\x81\x01\x90a\x01+V[\x90\x91V[`\x01\x80`@\x1B\x03\x81\x11a\x01\x96Wa\x01\x92` \x91a\0\x93V[\x01\x90V[a\0\x9DV[\x90a\x01\xADa\x01\xA8\x83a\x01zV[a\0\xD8V[\x91\x82RV[_\x7FSyndicate\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x01\xE3`\ta\x01\x9BV[\x90a\x01\xF0` \x83\x01a\x01\xB2V[V[a\x01\xFAa\x01\xD9V[\x90V[_\x7FSYND\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x02.`\x04a\x01\x9BV[\x90a\x02;` \x83\x01a\x01\xFDV[V[a\x02Ea\x02$V[\x90V[\x90V[\x90V[a\x02ba\x02]a\x02g\x92a\x02HV[a\x02KV[a\0\xF1V[\x90V[a\x02s\x90a\x02NV[\x90V[_\x01\x90V[\x90V[\x90V[a\x02\x95a\x02\x90a\x02\x9A\x92a\x02{V[a\x02KV[a\x02~V[\x90V[a\x02\xAAc\x01\xE13\x80a\x02\x81V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x02\xD0a\x02\xD6\x91\x93\x92\x93a\x02~V[\x92a\x02~V[\x82\x01\x80\x92\x11a\x02\xE1WV[a\x02\xADV[a\x02\xFAa\x02\xF5a\x02\xFF\x92a\x02HV[a\x02KV[a\x02~V[\x90V[_\x1B\x90V[\x90a\x03\x13_\x19\x91a\x03\x02V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x031a\x03,a\x036\x92a\x02~V[a\x02KV[a\x02~V[\x90V[\x90V[\x90a\x03Qa\x03La\x03X\x92a\x03\x1DV[a\x039V[\x82Ta\x03\x07V[\x90UV[\x90V[a\x03sa\x03na\x03x\x92a\x02HV[a\x03\x02V[a\x03\\V[\x90V[a\x03\x84_a\x03_V[\x90V[\x90V[a\x03\x9Ea\x03\x99a\x03\xA3\x92a\x03\x87V[a\x02KV[a\x02~V[\x90V[a\x03\xBBk\x02\xE8vi\xC3\x08sj\x04\0\0\0a\x03\x8AV[\x90V[\x90a\x03\xE0a\x03\xCAa\x01\xF2V[a\x03\xD2a\x01\xF2V[a\x03\xDAa\x02=V[\x91a\x04\xA6V[\x81a\x03\xFBa\x03\xF5a\x03\xF0_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x04\x8AW\x80a\x04\x1Ba\x04\x15a\x04\x10_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x04nWa\x04]a\x04l\x92a\x049Ba\x043a\x02\x9DV[\x90a\x02\xC1V[a\x01`Ra\x04Pa\x04I_a\x02\xE6V[`\x0Ca\x03<V[a\x04Xa\x03{V[a\tBV[Pa\x04fa\x03\xA6V[\x90a\n\x10V[V[_c\xD9.#=`\xE0\x1B\x81R\x80a\x04\x86`\x04\x82\x01a\x02vV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x04\xA2`\x04\x82\x01a\x02vV[\x03\x90\xFD[\x90a\x04\xB1\x92\x91a\x04\xB3V[V[\x90a\x04\xBE\x92\x91a\x04\xC0V[V[\x90a\x04\xCB\x92\x91a\x04\xCDV[V[\x90a\x04\xD8\x92\x91a\x04\xDAV[V[\x90a\x04\xE5\x92\x91a\x052V[V[_\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x05\x18`\x01a\x01\x9BV[\x90a\x05%` \x83\x01a\x04\xE7V[V[a\x05/a\x05\x0EV[\x90V[\x90a\x05F\x92\x91a\x05@a\x05'V[\x90a\x05HV[V[\x90a\x05T\x93\x92\x91a\x05\x9AV[V[\x90V[\x90V[` \x01\x90V[Q\x90V[a\x05za\x05ua\x05\x7F\x92a\0\xF1V[a\x02KV[a\0\xF1V[\x90V[a\x05\x8B\x90a\x05fV[\x90V[a\x05\x97\x90a\x05\x82V[\x90V[a\x05\xABa\x05\xFB\x94a\x05\xE0\x93\x94a\x06/V[a\x05\xBF\x81a\x05\xB9`\x06a\x05VV[\x90a\n\xBDV[a\x01 Ra\x05\xD7\x83a\x05\xD1`\x07a\x05VV[\x90a\n\xBDV[a\x01@Ra\x05YV[a\x05\xF2a\x05\xEC\x82a\x05bV[\x91a\x05\\V[ `\xE0Ra\x05YV[a\x06\ra\x06\x07\x82a\x05bV[\x91a\x05\\V[ a\x01\0RF`\xA0Ra\x06\x1Ea\x0B\xC2V[`\x80Ra\x06*0a\x05\x8EV[`\xC0RV[\x90a\x069\x91a\x06;V[V[\x90a\x06E\x91a\x06GV[V[\x90a\x06Q\x91a\x08\x98V[V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[Q\x90V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x01`\x02\x83\x04\x92\x16\x80\x15a\x06\x9EW[` \x83\x10\x14a\x06\x99WV[a\x06jV[\x91`\x7F\x16\x91a\x06\x8EV[_R` _ \x90V[`\x1F` \x91\x01\x04\x90V[\x1B\x90V[\x91\x90`\x08a\x06\xDA\x91\x02\x91a\x06\xD4_\x19\x84a\x06\xBBV[\x92a\x06\xBBV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x91\x90a\x06\xFAa\x06\xF5a\x07\x02\x93a\x03\x1DV[a\x039V[\x90\x83Ta\x06\xBFV[\x90UV[_\x90V[a\x07\x1C\x91a\x07\x16a\x07\x06V[\x91a\x06\xE4V[V[[\x81\x81\x10a\x07*WPPV[\x80a\x077_`\x01\x93a\x07\nV[\x01a\x07\x1FV[\x91\x90`\x1F\x81\x11a\x07MW[PPPV[a\x07Ya\x07~\x93a\x06\xA8V[\x90` a\x07e\x84a\x06\xB1V[\x83\x01\x93\x10a\x07\x86W[a\x07w\x90a\x06\xB1V[\x01\x90a\x07\x1EV[_\x80\x80a\x07HV[\x91Pa\x07w\x81\x92\x90Pa\x07nV[\x1C\x90V[\x90a\x07\xA8\x90_\x19\x90`\x08\x02a\x07\x94V[\x19\x16\x90V[\x81a\x07\xB7\x91a\x07\x98V[\x90`\x02\x02\x17\x90V[\x90a\x07\xC9\x81a\x06fV[\x90`\x01\x80`@\x1B\x03\x82\x11a\x08\x87Wa\x07\xEB\x82a\x07\xE5\x85Ta\x06~V[\x85a\x07=V[` \x90`\x1F\x83\x11`\x01\x14a\x08\x1FW\x91\x80\x91a\x08\x0E\x93_\x92a\x08\x13W[PPa\x07\xADV[\x90U[V[\x90\x91P\x01Q_\x80a\x08\x07V[`\x1F\x19\x83\x16\x91a\x08.\x85a\x06\xA8V[\x92_[\x81\x81\x10a\x08oWP\x91`\x02\x93\x91\x85`\x01\x96\x94\x10a\x08UW[PPP\x02\x01\x90Ua\x08\x11V[a\x08e\x91\x01Q`\x1F\x84\x16\x90a\x07\x98V[\x90U_\x80\x80a\x08IV[\x91\x93` `\x01\x81\x92\x87\x87\x01Q\x81U\x01\x95\x01\x92\x01a\x081V[a\0\x9DV[\x90a\x08\x96\x91a\x07\xBFV[V[\x90a\x08\xA7a\x08\xAE\x92`\x03a\x08\x8CV[`\x04a\x08\x8CV[V[_\x90V[\x15\x15\x90V[a\x08\xC2\x90a\x03\\V[\x90V[\x90a\x08\xCF\x90a\x08\xB9V[_R` R`@_ \x90V[a\x08\xE4\x90a\x05\x82V[\x90V[\x90a\x08\xF1\x90a\x08\xDBV[_R` R`@_ \x90V[\x90a\t\t`\xFF\x91a\x03\x02V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\t\x1C\x90a\x08\xB4V[\x90V[\x90V[\x90a\t7a\t2a\t>\x92a\t\x13V[a\t\x1FV[\x82Ta\x08\xFDV[\x90UV[a\tJa\x08\xB0V[Pa\t_a\tY\x82\x84\x90a\x0C_V[\x15a\x08\xB4V[_\x14a\t\xE8Wa\t\x87`\x01a\t\x82_a\tz`\x05\x86\x90a\x08\xC5V[\x01\x85\x90a\x08\xE7V[a\t\"V[\x90a\t\x90a\x0C\x8DV[\x90a\t\xCDa\t\xC7a\t\xC1\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x95a\x08\xB9V[\x92a\x08\xDBV[\x92a\x08\xDBV[\x92a\t\xD6a\0\x89V[\x80a\t\xE0\x81a\x02vV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a\t\xF7\x90a\0\xFCV[\x90RV[\x91\x90a\n\x0E\x90_` \x85\x01\x94\x01\x90a\t\xEEV[V[\x80a\n+a\n%a\n _a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\nGWa\nE\x91a\n=_a\x02jV[\x91\x90\x91a\x0C\xBEV[V[a\nja\nS_a\x02jV[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\t\xFBV[\x03\x90\xFD[_\x90V[\x90V[a\n\x89a\n\x84a\n\x8E\x92a\nrV[a\x02KV[a\x02~V[\x90V[\x90V[a\n\xA8a\n\xA3a\n\xAD\x92a\n\x91V[a\x03\x02V[a\x03\\V[\x90V[a\n\xBA`\xFFa\n\x94V[\x90V[\x90a\n\xC6a\nnV[Pa\n\xD8a\n\xD3\x83a\x05YV[a\x05bV[a\n\xEBa\n\xE5` a\nuV[\x91a\x02~V[\x10_\x14a\n\xFFWPa\n\xFC\x90a\x0EXV[\x90V[_a\x0B\ra\x0B\x13\x93\x92a\rhV[\x01a\x08\x8CV[a\x0B#a\x0B\x1Ea\n\xB0V[a\x08\xB9V[\x90V[_\x90V[\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x90V[a\x0BX\x90Qa\x03\\V[\x90V[a\x0Bd\x90a\x03\\V[\x90RV[a\x0Bq\x90a\x02~V[\x90RV[\x90\x95\x94\x92a\x0B\xC0\x94a\x0B\xAFa\x0B\xB9\x92a\x0B\xA5`\x80\x96a\x0B\x9B`\xA0\x88\x01\x9C_\x89\x01\x90a\x0B[V[` \x87\x01\x90a\x0B[V[`@\x85\x01\x90a\x0B[V[``\x83\x01\x90a\x0BhV[\x01\x90a\t\xEEV[V[a\x0B\xCAa\x0B&V[Pa\x0B\xD3a\x0B*V[a\x0C\x1Da\x0B\xE0`\xE0a\x0BNV[\x91a\x0C\x0Ea\x0B\xEFa\x01\0a\x0BNV[Fa\x0B\xF90a\x05\x8EV[\x91a\x0C\x02a\0\x89V[\x96\x87\x95` \x87\x01a\x0BuV[` \x82\x01\x81\x03\x82R\x03\x82a\0\xB1V[a\x0C/a\x0C)\x82a\x05bV[\x91a\x05\\V[ \x90V[_\x1C\x90V[`\xFF\x16\x90V[a\x0CJa\x0CO\x91a\x0C3V[a\x0C8V[\x90V[a\x0C\\\x90Ta\x0C>V[\x90V[a\x0C\x86\x91_a\x0C{a\x0C\x81\x93a\x0Csa\x08\xB0V[P`\x05a\x08\xC5V[\x01a\x08\xE7V[a\x0CRV[\x90V[_\x90V[a\x0C\x95a\x0C\x89V[P3\x90V[\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t\x90V[\x91\x82a\x0C\xDAa\x0C\xD4a\x0C\xCF_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14\x15\x80a\rEW[a\x0C\xF5W[a\x0C\xF3\x92\x91\x90\x91a\x0F|V[V[a\x0C\xFDa\x0F\x06V[\x80a\r$W[\x15a\x0C\xE7W_c6\xE2x\xFD`\xE2\x1B\x81R\x80a\r `\x04\x82\x01a\x02vV[\x03\x90\xFD[Pa\r@a\r:a\r3a\x0C\x9AV[3\x90a\x0C_V[\x15a\x08\xB4V[a\r\x03V[P\x81a\raa\r[a\rV_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14\x15a\x0C\xE2V[\x90V[\x90V[a\r\x82a\r}a\r\x87\x92a\rkV[a\x02KV[a\x02~V[\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[a\r\xBDa\r\xC6` \x93a\r\xCB\x93a\r\xB4\x81a\x06fV[\x93\x84\x80\x93a\r\x8AV[\x95\x86\x91\x01a\r\x93V[a\0\x93V[\x01\x90V[a\r\xE4\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\r\x9EV[\x90V[a\x0E\x01a\r\xFCa\r\xF6\x83a\x05bV[\x92a\x05\\V[a\x0BNV[\x90` \x81\x10a\x0E\x0FW[P\x90V[a\x0E!\x90_\x19\x90` \x03`\x08\x02a\x06\xBBV[\x16_a\x0E\x0BV[a\x0E4a\x0E9\x91a\x0C3V[a\x03\x1DV[\x90V[a\x0EPa\x0EKa\x0EU\x92a\x02~V[a\x03\x02V[a\x03\\V[\x90V[a\x0E`a\nnV[Pa\x0Ej\x81a\x05YV[\x90a\x0Et\x82a\x05bV[a\x0E\x87a\x0E\x81`\x1Fa\rnV[\x91a\x02~V[\x11a\x0E\xBCWPa\x0E\xB4\x81a\x0E\xAEa\x0E\xA8a\x0E\xA3a\x0E\xB9\x95a\r\xE7V[a\x0E(V[\x91a\x05bV[\x17a\x0E<V[a\x08\xB9V[\x90V[a\x0E\xDE\x90a\x0E\xC8a\0\x89V[\x91\x82\x91c0Z'\xA9`\xE0\x1B\x83R`\x04\x83\x01a\r\xCFV[\x03\x90\xFD[\x90V[a\x0E\xF1a\x0E\xF6\x91a\x0C3V[a\x0E\xE2V[\x90V[a\x0F\x03\x90Ta\x0E\xE5V[\x90V[a\x0F\x0Ea\x08\xB0V[Pa\x0F\x19`\x0Ca\x0E\xF9V[a\x0F+a\x0F%_a\x02\xE6V[\x91a\x02~V[\x14\x15\x80a\x0F6W[\x90V[PBa\x0FSa\x0FMa\x0FH`\x0Ca\x0E\xF9V[a\x02~V[\x91a\x02~V[\x10a\x0F3V[\x91` a\x0Fz\x92\x94\x93a\x0Fs`@\x82\x01\x96_\x83\x01\x90a\x0BhV[\x01\x90a\x0BhV[V[\x92\x91a\x0F\x8A\x84\x83\x83\x91a\x10\x85V[\x83a\x0F\xA5a\x0F\x9Fa\x0F\x9A_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x0F\xBAW[a\x0F\xB8\x92\x93\x91\x90\x91a\x12RV[V[a\x0F\xC2a\x11\xF4V[\x93a\x0F\xCBa\x121V[\x94\x80a\x0F\xDFa\x0F\xD9\x88a\x02~V[\x91a\x02~V[\x11a\x0F\xECWP\x93Pa\x0F\xABV[\x85\x90a\x10\x08_\x92\x83\x92c\x0EX\xAE\x93`\xE1\x1B\x84R`\x04\x84\x01a\x0FYV[\x03\x90\xFD[\x90a\x10\x16\x90a\x08\xDBV[_R` R`@_ \x90V[`@\x90a\x10Ka\x10R\x94\x96\x95\x93\x96a\x10A``\x84\x01\x98_\x85\x01\x90a\t\xEEV[` \x83\x01\x90a\x0BhV[\x01\x90a\x0BhV[V[\x90a\x10_\x91\x03a\x02~V[\x90V[\x90a\x10m\x91\x01a\x02~V[\x90V[\x91\x90a\x10\x83\x90_` \x85\x01\x94\x01\x90a\x0BhV[V[\x91\x90\x91\x80a\x10\xA3a\x10\x9Da\x10\x98_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14_\x14a\x11\x84Wa\x10\xC7a\x10\xC0\x83a\x10\xBB`\x02a\x0E\xF9V[a\x02\xC1V[`\x02a\x03<V[[\x82a\x10\xE3a\x10\xDDa\x10\xD8_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14_\x14a\x11XWa\x11\x07a\x11\0\x83a\x10\xFB`\x02a\x0E\xF9V[a\x10TV[`\x02a\x03<V[[\x91\x90\x91a\x11Sa\x11Aa\x11;\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93a\x08\xDBV[\x93a\x08\xDBV[\x93a\x11Ja\0\x89V[\x91\x82\x91\x82a\x10pV[\x03\x90\xA3V[a\x11\x7F\x82a\x11ya\x11j_\x87\x90a\x10\x0CV[\x91a\x11t\x83a\x0E\xF9V[a\x10bV[\x90a\x03<V[a\x11\x08V[a\x11\x97a\x11\x92_\x83\x90a\x10\x0CV[a\x0E\xF9V[\x80a\x11\xAAa\x11\xA4\x85a\x02~V[\x91a\x02~V[\x10a\x11\xD2Wa\x11\xBDa\x11\xCD\x91\x84\x90a\x10TV[a\x11\xC8_\x84\x90a\x10\x0CV[a\x03<V[a\x10\xC8V[\x90a\x11\xF0\x90\x91\x92_\x93\x84\x93c9\x144\xE3`\xE2\x1B\x85R`\x04\x85\x01a\x10\"V[\x03\x90\xFD[a\x11\xFCa\x07\x06V[Pa\x12\x07`\x02a\x0E\xF9V[\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x12)a\x12$a\x12.\x92a\x12\nV[a\x02KV[a\x02~V[\x90V[a\x129a\x07\x06V[Pa\x12I`\x01\x80`\xD0\x1B\x03a\x12\x15V[\x90V[\x90V[\x90V[\x91a\x12\xAAa\x12\xA4a\x12\xB1\x94\x80a\x12xa\x12ra\x12m_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x12\xE2W[\x84a\x12\x99a\x12\x93a\x12\x8E_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x12\xB3W[a\x14\xDAV[\x92a\x14\xDAV[\x90\x91a\x15\x0FV[V[a\x12\xDB`\x0B`\x02a\x12\xD5a\x12\xCFa\x12\xC9\x89a\x13\xC4V[\x93a\x12LV[\x91a\x12OV[\x90a\x14\x17V[PPa\x12\x9FV[a\x13\n`\x0B`\x01a\x13\x04a\x12\xFEa\x12\xF8\x89a\x13\xC4V[\x93a\x12LV[\x91a\x12OV[\x90a\x14\x17V[PPa\x12~V[_\x90V[a\x13!a\x13'\x91a\x12\nV[\x91a\x12\nV[\x01\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x138WV[a\x02\xADV[\x90a\x13P\x91a\x13Ja\x13\x11V[Pa\x13\x15V[\x90V[\x90V[`\xFF\x16\x90V[a\x13pa\x13ka\x13u\x92a\x13SV[a\x02KV[a\x13VV[\x90V[a\x13\x81\x90a\x13\\V[\x90RV[\x91` a\x13\xA6\x92\x94\x93a\x13\x9F`@\x82\x01\x96_\x83\x01\x90a\x13xV[\x01\x90a\x0BhV[V[a\x13\xBCa\x13\xB7a\x13\xC1\x92a\x02~V[a\x02KV[a\x12\nV[\x90V[a\x13\xCCa\x13\x11V[P\x80a\x13\xE6a\x13\xE0`\x01\x80`\xD0\x1B\x03a\x12\x15V[\x91a\x02~V[\x11a\x13\xF7Wa\x13\xF4\x90a\x13\xA8V[\x90V[`\xD0a\x14\x13_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a\x13\x85V[\x03\x90\xFD[\x90a\x14Ma\x14S\x93\x92a\x14(a\x13\x11V[Pa\x141a\x13\x11V[P\x80\x93a\x14Fa\x14?a\x16\xC1V[\x94\x92a\x17nV[\x90\x91a\x1CTV[\x91a\x17\xE3V[\x91\x90\x91\x90V[a\x14ea\x14k\x91a\x12\nV[\x91a\x12\nV[\x90\x03\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x14}WV[a\x02\xADV[\x90a\x14\x95\x91a\x14\x8Fa\x13\x11V[Pa\x14YV[\x90V[\x90a\x14\xA2\x90a\x08\xDBV[_R` R`@_ \x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x14\xC5a\x14\xCA\x91a\x0C3V[a\x14\xAEV[\x90V[a\x14\xD7\x90Ta\x14\xB9V[\x90V[a\x14\xF1a\x14\xF6\x91a\x14\xE9a\x0C\x89V[P`\ta\x14\x98V[a\x14\xCDV[\x90V[\x90a\x15\x03\x90a\x08\xDBV[_R` R`@_ \x90V[\x91\x90\x91\x80a\x15%a\x15\x1F\x85a\0\xFCV[\x91a\0\xFCV[\x14\x15\x80a\x16\xA3W[a\x157W[PPPV[\x80a\x15Ra\x15La\x15G_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x03a\x16\x13W[P\x81a\x15ta\x15na\x15i_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x03a\x15\x80W[\x80a\x152V[a\x15\xC7a\x15\xBAa\x15\xC1\x92a\x15\x96`\n\x86\x90a\x14\xF9V[\x90a\x15\xB4a\x15\xAEa\x15\xA8`\x01\x93a\x13\xC4V[\x93a\x12LV[\x91a\x12OV[\x90a\x14\x17V[\x92\x90a\x12\x15V[\x91a\x12\x15V[\x91\x90\x91a\x15\xF4\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x08\xDBV[\x92a\x16\ta\x16\0a\0\x89V[\x92\x83\x92\x83a\x0FYV[\x03\x90\xA2_\x80a\x15zV[a\x16Ra\x16Xa\x16Ka\x16(`\n\x85\x90a\x14\xF9V[`\x02a\x16Ea\x16?a\x169\x89a\x13\xC4V[\x93a\x12LV[\x91a\x12OV[\x90a\x14\x17V[\x92\x90a\x12\x15V[\x91a\x12\x15V[\x91\x90\x91a\x16\x85\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x08\xDBV[\x92a\x16\x9Aa\x16\x91a\0\x89V[\x92\x83\x92\x83a\x0FYV[\x03\x90\xA2_a\x15XV[P\x81a\x16\xB7a\x16\xB1_a\x02\xE6V[\x91a\x02~V[\x11a\x15-V[_\x90V[a\x16\xC9a\x16\xBDV[Pa\x16\xD2a\x18\x12V[\x90V[T\x90V[\x90V[a\x16\xF0a\x16\xEBa\x16\xF5\x92a\x16\xD9V[a\x02KV[a\x02~V[\x90V[a\x17\x07a\x17\r\x91\x93\x92\x93a\x02~V[\x92a\x02~V[\x82\x03\x91\x82\x11a\x17\x18WV[a\x02\xADV[\x90V[`0\x1C\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x17=a\x17B\x91a\x17 V[a\x17&V[\x90V[a\x17O\x90Ta\x171V[\x90V[a\x17fa\x17aa\x17k\x92a\x02HV[a\x02KV[a\x12\nV[\x90V[a\x17va\x13\x11V[Pa\x17\x82_\x82\x01a\x16\xD5V[\x80a\x17\x95a\x17\x8F_a\x02\xE6V[\x91a\x02~V[\x14_\x14a\x17\xABWPPa\x17\xA7_a\x17RV[[\x90V[a\x17\xD8_\x91a\x17\xD3a\x17\xCD\x84a\x17\xDE\x96\x01\x92a\x17\xC7`\x01a\x16\xDCV[\x90a\x16\xF8V[\x91a\x17\x1DV[a\x18'V[\x01a\x17EV[a\x17\xA8V[\x91a\x18\x07_a\x18\x0C\x94a\x17\xF4a\x13\x11V[Pa\x17\xFDa\x13\x11V[P\x01\x92\x91\x92a\x17\x1DV[a\x1A,V[\x91\x90\x91\x90V[a\x18\x1Aa\x16\xBDV[Pa\x18$Ca\x1B\xEDV[\x90V[_R` _ \x01\x90V[T\x90V[a\x18?`@a\0\xD8V[\x90V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90a\x18W\x90a\x18BV[\x90RV[\x90a\x18e\x90a\x12\nV[\x90RV[_R` _ \x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[a\x18\x8F\x81a\x181V[\x82\x10\x15a\x18\xA9Wa\x18\xA1`\x01\x91a\x18iV[\x91\x02\x01\x90_\x90V[a\x18rV[a\x18\xB8\x90Qa\x18BV[\x90V[\x90a\x18\xCCe\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x03\x02V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x18\xEAa\x18\xE5a\x18\xEF\x92a\x18BV[a\x02KV[a\x18BV[\x90V[\x90V[\x90a\x19\na\x19\x05a\x19\x11\x92a\x18\xD6V[a\x18\xF2V[\x82Ta\x18\xBBV[\x90UV[a\x19\x1F\x90Qa\x12\nV[\x90V[`0\x1B\x90V[\x90a\x19:e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91a\x19\"V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x19Xa\x19Sa\x19]\x92a\x12\nV[a\x02KV[a\x12\nV[\x90V[\x90V[\x90a\x19xa\x19sa\x19\x7F\x92a\x19DV[a\x19`V[\x82Ta\x19(V[\x90UV[\x90a\x19\xAD` _a\x19\xB3\x94a\x19\xA5\x82\x82\x01a\x19\x9F\x84\x88\x01a\x18\xAEV[\x90a\x18\xF5V[\x01\x92\x01a\x19\x15V[\x90a\x19cV[V[\x91\x90a\x19\xC6Wa\x19\xC4\x91a\x19\x83V[V[a\x06SV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x19\xFBW\x82a\x19\xF3\x91`\x01a\x19\xF9\x95\x01\x81Ua\x18\x86V[\x90a\x19\xB5V[V[a\0\x9DV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x1A\x17a\x1A\x1C\x91a\x0C3V[a\x1A\0V[\x90V[a\x1A)\x90Ta\x1A\x0BV[\x90V[\x90\x92\x91\x92a\x1A8a\x13\x11V[Pa\x1AAa\x13\x11V[Pa\x1AK\x82a\x181V[\x80a\x1A^a\x1AX_a\x02\xE6V[\x91a\x02~V[\x11_\x14a\x1B.Wa\x1A\x84\x90a\x1A~\x84\x91a\x1Ax`\x01a\x16\xDCV[\x90a\x16\xF8V[\x90a\x18'V[\x90a\x1A\x90_\x83\x01a\x1A\x1FV[\x92a\x1A\x9C_\x84\x01a\x17EV[\x93\x80a\x1A\xB0a\x1A\xAA\x85a\x18BV[\x91a\x18BV[\x11a\x1B\x12Wa\x1A\xC7a\x1A\xC1\x84a\x18BV[\x91a\x18BV[\x14_\x14a\x1A\xE2WPPa\x1A\xDD\x90_\x85\x91\x01a\x19cV[[\x91\x90V[a\x1B\r\x92Pa\x1B\x08\x86a\x1A\xFFa\x1A\xF6a\x185V[\x94_\x86\x01a\x18MV[` \x84\x01a\x18[V[a\x19\xCBV[a\x1A\xDEV[_c% `\x1D`\xE0\x1B\x81R\x80a\x1B*`\x04\x82\x01a\x02vV[\x03\x90\xFD[Pa\x1BY\x91a\x1BT\x85a\x1BKa\x1BBa\x185V[\x94_\x86\x01a\x18MV[` \x84\x01a\x18[V[a\x19\xCBV[a\x1Bb_a\x17RV[\x91\x90V[a\x1Bza\x1Bua\x1B\x7F\x92a\x18BV[a\x02KV[a\x02~V[\x90V[\x90V[a\x1B\x99a\x1B\x94a\x1B\x9E\x92a\x1B\x82V[a\x02KV[a\x13VV[\x90V[a\x1B\xAA\x90a\x1B\x85V[\x90RV[\x91` a\x1B\xCF\x92\x94\x93a\x1B\xC8`@\x82\x01\x96_\x83\x01\x90a\x1B\xA1V[\x01\x90a\x0BhV[V[a\x1B\xE5a\x1B\xE0a\x1B\xEA\x92a\x02~V[a\x02KV[a\x18BV[\x90V[a\x1B\xF5a\x16\xBDV[P\x80a\x1C\x0Fa\x1C\te\xFF\xFF\xFF\xFF\xFF\xFFa\x1BfV[\x91a\x02~V[\x11a\x1C Wa\x1C\x1D\x90a\x1B\xD1V[\x90V[`0a\x1C<_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a\x1B\xAEV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`Q`\x04R`$_\xFD[\x91\x90\x91\x80`\x01\x14a\x1CsW`\x02\x03a\x1C@Wa\x1Co\x91a\x14\x82V[\x90[V[Pa\x1C}\x91a\x13=V[\x90a\x1CqV\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x14\x91V[a\0\x1D_5a\x02\xFCV[\x80c\x01\xFF\xC9\xA7\x14a\x02\xF7W\x80c\x06\xFD\xDE\x03\x14a\x02\xF2W\x80c\t^\xA7\xB3\x14a\x02\xEDW\x80c\x18\x16\r\xDD\x14a\x02\xE8W\x80c#\xB8r\xDD\x14a\x02\xE3W\x80c$\x8A\x9C\xA3\x14a\x02\xDEW\x80c//\xF1]\x14a\x02\xD9W\x80c1<\xE5g\x14a\x02\xD4W\x80c6D\xE5\x15\x14a\x02\xCFW\x80c6V\x8A\xBE\x14a\x02\xCAW\x80c:F\xB1\xA8\x14a\x02\xC5W\x80c@\xC1\x0F\x19\x14a\x02\xC0W\x80cK\xDD6\xCE\x14a\x02\xBBW\x80cK\xF5\xD7\xE9\x14a\x02\xB6W\x80cO\x1B\xFC\x9E\x14a\x02\xB1W\x80cX|\xDE\x1E\x14a\x02\xACW\x80c\\\x19\xA9\\\x14a\x02\xA7W\x80co\xCF\xFFE\x14a\x02\xA2W\x80cp\xA0\x821\x14a\x02\x9DW\x80cy\xCCg\x90\x14a\x02\x98W\x80cz\x8C\xD1V\x14a\x02\x93W\x80c~\xCE\xBE\0\x14a\x02\x8EW\x80c\x83\xF1!\x1B\x14a\x02\x89W\x80c\x84&\xAD\xF2\x14a\x02\x84W\x80c\x84L\x90&\x14a\x02\x7FW\x80c\x84\xB0\x19n\x14a\x02zW\x80c\x8AT%!\x14a\x02uW\x80c\x8D3C\xD6\x14a\x02pW\x80c\x8ES\x9E\x8C\x14a\x02kW\x80c\x90-U\xA5\x14a\x02fW\x80c\x91\xD1HT\x14a\x02aW\x80c\x91\xDD\xAD\xF4\x14a\x02\\W\x80c\x95\xD8\x9BA\x14a\x02WW\x80c\x9A\xB2N\xB0\x14a\x02RW\x80c\x9B~\xF6K\x14a\x02MW\x80c\xA2\x17\xFD\xDF\x14a\x02HW\x80c\xA9\x05\x9C\xBB\x14a\x02CW\x80c\xAA\x08*\x9D\x14a\x02>W\x80c\xB0\xCA%>\x14a\x029W\x80c\xBBMD6\x14a\x024W\x80c\xC0*\xE7T\x14a\x02/W\x80c\xC3\xCD\xA5 \x14a\x02*W\x80c\xD5\x05\xAC\xCF\x14a\x02%W\x80c\xD5Gt\x1F\x14a\x02 W\x80c\xDDb\xED>\x14a\x02\x1BWc\xF1\x12~\xD8\x03a\0\x0EWa\x14[V[a\x13wV[a\x13\x16V[a\x12\xDCV[a\x122V[a\x11vV[a\x11AV[a\x11\x0BV[a\x10\xD6V[a\x10dV[a\x10/V[a\x0F\xBFV[a\x0FHV[a\x0F\x13V[a\x0E\xDEV[a\x0E{V[a\x0EFV[a\r\xCFV[a\r\x9AV[a\r6V[a\x0C\xCBV[a\x0B\x86V[a\x0B3V[a\n\xDAV[a\n\xA5V[a\npV[a\n<V[a\n\x07V[a\t\xD2V[a\ttV[a\t?V[a\x08\xCAV[a\x08XV[a\x08#V[a\x07\xEFV[a\x07\xB9V[a\x07\x85V[a\x07PV[a\x07\x1BV[a\x06\xBFV[a\x06XV[a\x05\xBCV[a\x05MV[a\x04\xF5V[a\x043V[a\x03\x84V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[c\xFF\xFF\xFF\xFF`\xE0\x1B\x16\x90V[a\x03%\x81a\x03\x10V[\x03a\x03,WV[_\x80\xFD[\x90P5\x90a\x03=\x82a\x03\x1CV[V[\x90` \x82\x82\x03\x12a\x03XWa\x03U\x91_\x01a\x030V[\x90V[a\x03\x0CV[\x15\x15\x90V[a\x03k\x90a\x03]V[\x90RV[\x91\x90a\x03\x82\x90_` \x85\x01\x94\x01\x90a\x03bV[V[4a\x03\xB4Wa\x03\xB0a\x03\x9Fa\x03\x9A6`\x04a\x03?V[a\x15.V[a\x03\xA7a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[_\x91\x03\x12a\x03\xC3WV[a\x03\x0CV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x04\ta\x04\x12` \x93a\x04\x17\x93a\x04\0\x81a\x03\xC8V[\x93\x84\x80\x93a\x03\xCCV[\x95\x86\x91\x01a\x03\xD5V[a\x03\xE0V[\x01\x90V[a\x040\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03\xEAV[\x90V[4a\x04cWa\x04C6`\x04a\x03\xB9V[a\x04_a\x04Na\x16\xC7V[a\x04Va\x03\x02V[\x91\x82\x91\x82a\x04\x1BV[\x03\x90\xF3[a\x03\x08V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04|\x90a\x04hV[\x90V[a\x04\x88\x81a\x04sV[\x03a\x04\x8FWV[_\x80\xFD[\x90P5\x90a\x04\xA0\x82a\x04\x7FV[V[\x90V[a\x04\xAE\x81a\x04\xA2V[\x03a\x04\xB5WV[_\x80\xFD[\x90P5\x90a\x04\xC6\x82a\x04\xA5V[V[\x91\x90`@\x83\x82\x03\x12a\x04\xF0W\x80a\x04\xE4a\x04\xED\x92_\x86\x01a\x04\x93V[\x93` \x01a\x04\xB9V[\x90V[a\x03\x0CV[4a\x05&Wa\x05\"a\x05\x11a\x05\x0B6`\x04a\x04\xC8V[\x90a\x16\xDDV[a\x05\x19a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[a\x054\x90a\x04\xA2V[\x90RV[\x91\x90a\x05K\x90_` \x85\x01\x94\x01\x90a\x05+V[V[4a\x05}Wa\x05]6`\x04a\x03\xB9V[a\x05ya\x05ha\x17)V[a\x05pa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90\x91``\x82\x84\x03\x12a\x05\xB7Wa\x05\xB4a\x05\x9D\x84_\x85\x01a\x04\x93V[\x93a\x05\xAB\x81` \x86\x01a\x04\x93V[\x93`@\x01a\x04\xB9V[\x90V[a\x03\x0CV[4a\x05\xEDWa\x05\xE9a\x05\xD8a\x05\xD26`\x04a\x05\x82V[\x91a\x17?V[a\x05\xE0a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[\x90V[a\x05\xFE\x81a\x05\xF2V[\x03a\x06\x05WV[_\x80\xFD[\x90P5\x90a\x06\x16\x82a\x05\xF5V[V[\x90` \x82\x82\x03\x12a\x061Wa\x06.\x91_\x01a\x06\tV[\x90V[a\x03\x0CV[a\x06?\x90a\x05\xF2V[\x90RV[\x91\x90a\x06V\x90_` \x85\x01\x94\x01\x90a\x066V[V[4a\x06\x88Wa\x06\x84a\x06sa\x06n6`\x04a\x06\x18V[a\x17\xB8V[a\x06{a\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[\x91\x90`@\x83\x82\x03\x12a\x06\xB5W\x80a\x06\xA9a\x06\xB2\x92_\x86\x01a\x06\tV[\x93` \x01a\x04\x93V[\x90V[a\x03\x0CV[_\x01\x90V[4a\x06\xEEWa\x06\xD8a\x06\xD26`\x04a\x06\x8DV[\x90a\x18\x04V[a\x06\xE0a\x03\x02V[\x80a\x06\xEA\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[`\xFF\x16\x90V[a\x07\x02\x90a\x06\xF3V[\x90RV[\x91\x90a\x07\x19\x90_` \x85\x01\x94\x01\x90a\x06\xF9V[V[4a\x07KWa\x07+6`\x04a\x03\xB9V[a\x07Ga\x076a\x183V[a\x07>a\x03\x02V[\x91\x82\x91\x82a\x07\x06V[\x03\x90\xF3[a\x03\x08V[4a\x07\x80Wa\x07`6`\x04a\x03\xB9V[a\x07|a\x07ka\x18IV[a\x07sa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[4a\x07\xB4Wa\x07\x9Ea\x07\x986`\x04a\x06\x8DV[\x90a\x18]V[a\x07\xA6a\x03\x02V[\x80a\x07\xB0\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\x07\xEAWa\x07\xE6a\x07\xD5a\x07\xCF6`\x04a\x04\xC8V[\x90a\x19\x0EV[a\x07\xDDa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x08\x1EWa\x08\x08a\x08\x026`\x04a\x04\xC8V[\x90a\x1A\x95V[a\x08\x10a\x03\x02V[\x80a\x08\x1A\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\x08SWa\x0836`\x04a\x03\xB9V[a\x08Oa\x08>a\x1A\xC6V[a\x08Fa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x08\x88Wa\x08h6`\x04a\x03\xB9V[a\x08\x84a\x08sa\x1B\x85V[a\x08{a\x03\x02V[\x91\x82\x91\x82a\x04\x1BV[\x03\x90\xF3[a\x03\x08V[\x90V[\x90V[a\x08\xA7a\x08\xA2a\x08\xAC\x92a\x08\x8DV[a\x08\x90V[a\x04\xA2V[\x90V[a\x08\xBCc\x01\xE13\x80a\x08\x93V[\x90V[a\x08\xC7a\x08\xAFV[\x90V[4a\x08\xFAWa\x08\xDA6`\x04a\x03\xB9V[a\x08\xF6a\x08\xE5a\x08\xBFV[a\x08\xEDa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90` \x82\x82\x03\x12a\t\x18Wa\t\x15\x91_\x01a\x04\x93V[\x90V[a\x03\x0CV[a\t&\x90a\x04sV[\x90RV[\x91\x90a\t=\x90_` \x85\x01\x94\x01\x90a\t\x1DV[V[4a\toWa\tka\tZa\tU6`\x04a\x08\xFFV[a\x1C!V[a\tba\x03\x02V[\x91\x82\x91\x82a\t*V[\x03\x90\xF3[a\x03\x08V[4a\t\xA2Wa\t\x8Ca\t\x876`\x04a\x08\xFFV[a\x1C@V[a\t\x94a\x03\x02V[\x80a\t\x9E\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[c\xFF\xFF\xFF\xFF\x16\x90V[a\t\xB9\x90a\t\xA7V[\x90RV[\x91\x90a\t\xD0\x90_` \x85\x01\x94\x01\x90a\t\xB0V[V[4a\n\x02Wa\t\xFEa\t\xEDa\t\xE86`\x04a\x08\xFFV[a\x1CWV[a\t\xF5a\x03\x02V[\x91\x82\x91\x82a\t\xBDV[\x03\x90\xF3[a\x03\x08V[4a\n7Wa\n3a\n\"a\n\x1D6`\x04a\x08\xFFV[a\x1C\x82V[a\n*a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\nkWa\nUa\nO6`\x04a\x04\xC8V[\x90a\x1D\xB7V[a\n]a\x03\x02V[\x80a\ng\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\n\xA0Wa\n\x806`\x04a\x03\xB9V[a\n\x9Ca\n\x8Ba\x1D\xC3V[a\n\x93a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\n\xD5Wa\n\xD1a\n\xC0a\n\xBB6`\x04a\x08\xFFV[a\x1E;V[a\n\xC8a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x0B\nWa\n\xEA6`\x04a\x03\xB9V[a\x0B\x06a\n\xF5a\x1EPV[a\n\xFDa\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0BcWa\x0BC6`\x04a\x03\xB9V[a\x0B_a\x0BNa\x0B\x0FV[a\x0BVa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90` \x82\x82\x03\x12a\x0B\x81Wa\x0B~\x91_\x01a\x04\xB9V[\x90V[a\x03\x0CV[4a\x0B\xB4Wa\x0B\x9Ea\x0B\x996`\x04a\x0BhV[a XV[a\x0B\xA6a\x03\x02V[\x80a\x0B\xB0\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[`\xFF`\xF8\x1B\x16\x90V[a\x0B\xCB\x90a\x0B\xB9V[\x90RV[Q\x90V[` \x91\x81R\x01\x90V[` \x01\x90V[a\x0B\xEB\x90a\x04\xA2V[\x90RV[\x90a\x0B\xFC\x81` \x93a\x0B\xE2V[\x01\x90V[` \x01\x90V[\x90a\x0C#a\x0C\x1Da\x0C\x16\x84a\x0B\xCFV[\x80\x93a\x0B\xD3V[\x92a\x0B\xDCV[\x90_[\x81\x81\x10a\x0C3WPPP\x90V[\x90\x91\x92a\x0CLa\x0CF`\x01\x92\x86Qa\x0B\xEFV[\x94a\x0C\0V[\x91\x01\x91\x90\x91a\x0C&V[\x93\x95\x91\x94a\x0C\xA7a\x0C\x9Ca\x0C\xBB\x95a\x0C\x8Ea\x0C\xB1\x95a\x0C\xC8\x9C\x9Aa\x0C\x81`\xE0\x8C\x01\x92_\x8D\x01\x90a\x0B\xC2V[\x8A\x82\x03` \x8C\x01Ra\x03\xEAV[\x90\x88\x82\x03`@\x8A\x01Ra\x03\xEAV[\x97``\x87\x01\x90a\x05+V[`\x80\x85\x01\x90a\t\x1DV[`\xA0\x83\x01\x90a\x066V[`\xC0\x81\x84\x03\x91\x01Ra\x0C\x06V[\x90V[4a\r\x02Wa\x0C\xDB6`\x04a\x03\xB9V[a\x0C\xFEa\x0C\xE6a \xE0V[\x93a\x0C\xF5\x97\x95\x97\x93\x91\x93a\x03\x02V[\x97\x88\x97\x88a\x0CVV[\x03\x90\xF3[a\x03\x08V[\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t\x90V[a\r3a\r\x07V[\x90V[4a\rfWa\rF6`\x04a\x03\xB9V[a\rba\rQa\r+V[a\rYa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[\x7F\x9B\x12\xE0\xC5p~II\x15\xE5\x8B\x05d\xF1\x8A\xAA\xD9\xB7J\xC6\x9B\xFC\x81Z\x1E\xDA\xDC\x8EK\xD02\xEB\x90V[a\r\x97a\rkV[\x90V[4a\r\xCAWa\r\xAA6`\x04a\x03\xB9V[a\r\xC6a\r\xB5a\r\x8FV[a\r\xBDa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[4a\r\xFFWa\r\xFBa\r\xEAa\r\xE56`\x04a\x0BhV[a!jV[a\r\xF2a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90V[a\x0E\x1Ba\x0E\x16a\x0E \x92a\x0E\x04V[a\x08\x90V[a\x04\xA2V[\x90V[a\x0E8k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x0E\x07V[\x90V[a\x0ECa\x0E#V[\x90V[4a\x0EvWa\x0EV6`\x04a\x03\xB9V[a\x0Era\x0Eaa\x0E;V[a\x0Eia\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x0E\xACWa\x0E\xA8a\x0E\x97a\x0E\x916`\x04a\x06\x8DV[\x90a!\xD8V[a\x0E\x9Fa\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x0E\xC5\x90a\x0E\xB1V[\x90RV[\x91\x90a\x0E\xDC\x90_` \x85\x01\x94\x01\x90a\x0E\xBCV[V[4a\x0F\x0EWa\x0E\xEE6`\x04a\x03\xB9V[a\x0F\na\x0E\xF9a\"\x06V[a\x0F\x01a\x03\x02V[\x91\x82\x91\x82a\x0E\xC9V[\x03\x90\xF3[a\x03\x08V[4a\x0FCWa\x0F#6`\x04a\x03\xB9V[a\x0F?a\x0F.a\"\x1AV[a\x0F6a\x03\x02V[\x91\x82\x91\x82a\x04\x1BV[\x03\x90\xF3[a\x03\x08V[4a\x0FxWa\x0Fta\x0Fca\x0F^6`\x04a\x08\xFFV[a\"0V[a\x0Fka\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90V[a\x0F\x94a\x0F\x8Fa\x0F\x99\x92a\x0F}V[a\x08\x90V[a\x04\xA2V[\x90V[a\x0F\xB1k\x02\xE8vi\xC3\x08sj\x04\0\0\0a\x0F\x80V[\x90V[a\x0F\xBCa\x0F\x9CV[\x90V[4a\x0F\xEFWa\x0F\xCF6`\x04a\x03\xB9V[a\x0F\xEBa\x0F\xDAa\x0F\xB4V[a\x0F\xE2a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90V[_\x1B\x90V[a\x10\x10a\x10\x0Ba\x10\x15\x92a\x0F\xF4V[a\x0F\xF7V[a\x05\xF2V[\x90V[a\x10!_a\x0F\xFCV[\x90V[a\x10,a\x10\x18V[\x90V[4a\x10_Wa\x10?6`\x04a\x03\xB9V[a\x10[a\x10Ja\x10$V[a\x10Ra\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[4a\x10\x95Wa\x10\x91a\x10\x80a\x10z6`\x04a\x04\xC8V[\x90a\"_V[a\x10\x88a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[\x1C\x90V[\x90V[a\x10\xB1\x90`\x08a\x10\xB6\x93\x02a\x10\x9AV[a\x10\x9EV[\x90V[\x90a\x10\xC4\x91Ta\x10\xA1V[\x90V[a\x10\xD3`\x0C_\x90a\x10\xB9V[\x90V[4a\x11\x06Wa\x10\xE66`\x04a\x03\xB9V[a\x11\x02a\x10\xF1a\x10\xC7V[a\x10\xF9a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x11<Wa\x118a\x11'a\x11!6`\x04a\x04\xC8V[\x90a\"\x81V[a\x11/a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x11qWa\x11ma\x11\\a\x11W6`\x04a\x08\xFFV[a\"\x97V[a\x11da\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x11\xA6Wa\x11\x866`\x04a\x03\xB9V[a\x11\xA2a\x11\x91a\"\xACV[a\x11\x99a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[a\x11\xB4\x81a\x06\xF3V[\x03a\x11\xBBWV[_\x80\xFD[\x90P5\x90a\x11\xCC\x82a\x11\xABV[V[\x90\x91`\xC0\x82\x84\x03\x12a\x12-Wa\x11\xE6\x83_\x84\x01a\x04\x93V[\x92a\x11\xF4\x81` \x85\x01a\x04\xB9V[\x92a\x12\x02\x82`@\x83\x01a\x04\xB9V[\x92a\x12*a\x12\x13\x84``\x85\x01a\x11\xBFV[\x93a\x12!\x81`\x80\x86\x01a\x06\tV[\x93`\xA0\x01a\x06\tV[\x90V[a\x03\x0CV[4a\x12gWa\x12Qa\x12E6`\x04a\x11\xCEV[\x94\x93\x90\x93\x92\x91\x92a#,V[a\x12Ya\x03\x02V[\x80a\x12c\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[`\xE0\x81\x83\x03\x12a\x12\xD7Wa\x12\x82\x82_\x83\x01a\x04\x93V[\x92a\x12\x90\x83` \x84\x01a\x04\x93V[\x92a\x12\x9E\x81`@\x85\x01a\x04\xB9V[\x92a\x12\xAC\x82``\x83\x01a\x04\xB9V[\x92a\x12\xD4a\x12\xBD\x84`\x80\x85\x01a\x11\xBFV[\x93a\x12\xCB\x81`\xA0\x86\x01a\x06\tV[\x93`\xC0\x01a\x06\tV[\x90V[a\x03\x0CV[4a\x13\x11Wa\x12\xFBa\x12\xEF6`\x04a\x12lV[\x95\x94\x90\x94\x93\x91\x93a$\x80V[a\x13\x03a\x03\x02V[\x80a\x13\r\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\x13EWa\x13/a\x13)6`\x04a\x06\x8DV[\x90a%\x9EV[a\x137a\x03\x02V[\x80a\x13A\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[\x91\x90`@\x83\x82\x03\x12a\x13rW\x80a\x13fa\x13o\x92_\x86\x01a\x04\x93V[\x93` \x01a\x04\x93V[\x90V[a\x03\x0CV[4a\x13\xA8Wa\x13\xA4a\x13\x93a\x13\x8D6`\x04a\x13JV[\x90a%\xC0V[a\x13\x9Ba\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[a\x13\xB6\x81a\t\xA7V[\x03a\x13\xBDWV[_\x80\xFD[\x90P5\x90a\x13\xCE\x82a\x13\xADV[V[\x91\x90`@\x83\x82\x03\x12a\x13\xF8W\x80a\x13\xECa\x13\xF5\x92_\x86\x01a\x04\x93V[\x93` \x01a\x13\xC1V[\x90V[a\x03\x0CV[a\x14\x06\x90a\x0E\xB1V[\x90RV[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x14\x1E\x90a\x14\nV[\x90RV[\x90` \x80a\x14D\x93a\x14:_\x82\x01Q_\x86\x01\x90a\x13\xFDV[\x01Q\x91\x01\x90a\x14\x15V[V[\x91\x90a\x14Y\x90_`@\x85\x01\x94\x01\x90a\x14\"V[V[4a\x14\x8CWa\x14\x88a\x14wa\x14q6`\x04a\x13\xD0V[\x90a&.V[a\x14\x7Fa\x03\x02V[\x91\x82\x91\x82a\x14FV[\x03\x90\xF3[a\x03\x08V[_\x80\xFD[_\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x14\xB9a\x14\xBF\x91a\x14\nV[\x91a\x14\nV[\x01\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x14\xD0WV[a\x14\x99V[\x90a\x14\xE8\x91a\x14\xE2a\x14\x95V[Pa\x14\xADV[\x90V[a\x14\xF7a\x14\xFD\x91a\x14\nV[\x91a\x14\nV[\x90\x03\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x15\x0FWV[a\x14\x99V[\x90a\x15'\x91a\x15!a\x14\x95V[Pa\x14\xEBV[\x90V[_\x90V[a\x156a\x15*V[P\x80a\x15Qa\x15Kcye\xDB\x0B`\xE0\x1Ba\x03\x10V[\x91a\x03\x10V[\x14\x90\x81\x15a\x15^W[P\x90V[a\x15h\x91Pa&DV[_a\x15ZV[``\x90V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x01`\x02\x83\x04\x92\x16\x80\x15a\x15\xA7W[` \x83\x10\x14a\x15\xA2WV[a\x15sV[\x91`\x7F\x16\x91a\x15\x97V[` \x91\x81R\x01\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a\x15\xDDa\x15\xD6\x83a\x15\x87V[\x80\x94a\x15\xB1V[\x91`\x01\x81\x16\x90\x81_\x14a\x164WP`\x01\x14a\x15\xF8W[PPPV[a\x16\x05\x91\x92\x93\x94Pa\x15\xBAV[\x91_\x92[\x81\x84\x10a\x16\x1CWPP\x01\x90_\x80\x80a\x15\xF3V[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a\x16\tV[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a\x15\xF3V[\x90a\x16Y\x91a\x15\xC3V[\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x16z\x90a\x03\xE0V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x16\x94W`@RV[a\x16\\V[\x90a\x16\xB9a\x16\xB2\x92a\x16\xA9a\x03\x02V[\x93\x84\x80\x92a\x16OV[\x03\x83a\x16pV[V[a\x16\xC4\x90a\x16\x99V[\x90V[a\x16\xCFa\x15nV[Pa\x16\xDA`\x03a\x16\xBBV[\x90V[a\x16\xFA\x91a\x16\xE9a\x15*V[Pa\x16\xF2a&jV[\x91\x90\x91a&wV[`\x01\x90V[_\x90V[_\x1C\x90V[a\x17\x14a\x17\x19\x91a\x17\x03V[a\x10\x9EV[\x90V[a\x17&\x90Ta\x17\x08V[\x90V[a\x171a\x16\xFFV[Pa\x17<`\x02a\x17\x1CV[\x90V[\x91a\x17i\x92a\x17La\x15*V[Pa\x17aa\x17Xa&jV[\x82\x90\x84\x91a&\xC7V[\x91\x90\x91a'SV[`\x01\x90V[_\x90V[a\x17{\x90a\x05\xF2V[\x90V[\x90a\x17\x88\x90a\x17rV[_R` R`@_ \x90V[\x90V[a\x17\xA3a\x17\xA8\x91a\x17\x03V[a\x17\x94V[\x90V[a\x17\xB5\x90Ta\x17\x97V[\x90V[`\x01a\x17\xD1a\x17\xD7\x92a\x17\xC9a\x17nV[P`\x05a\x17~V[\x01a\x17\xABV[\x90V[\x90a\x17\xF5\x91a\x17\xF0a\x17\xEB\x82a\x17\xB8V[a'\xF0V[a\x17\xF7V[V[\x90a\x18\x01\x91a(IV[PV[\x90a\x18\x0E\x91a\x17\xDAV[V[_\x90V[\x90V[a\x18+a\x18&a\x180\x92a\x18\x14V[a\x08\x90V[a\x06\xF3V[\x90V[a\x18;a\x18\x10V[Pa\x18F`\x12a\x18\x17V[\x90V[a\x18Qa\x17nV[Pa\x18Za(\xF5V[\x90V[\x90\x80a\x18xa\x18ra\x18ma&jV[a\x04sV[\x91a\x04sV[\x03a\x18\x89Wa\x18\x86\x91a)\xAFV[PV[_c3K\xD9\x19`\xE1\x1B\x81R\x80a\x18\xA1`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[a\x18\xB9a\x18\xB4a\x18\xBE\x92a\x04hV[a\x08\x90V[a\x04hV[\x90V[a\x18\xCA\x90a\x18\xA5V[\x90V[a\x18\xD6\x90a\x18\xC1V[\x90V[\x90a\x18\xE3\x90a\x18\xCDV[_R` R`@_ \x90V[\x90V[a\x19\x06a\x19\x01a\x19\x0B\x92a\x14\nV[a\x08\x90V[a\x04\xA2V[\x90V[a\x19E\x91a\x19:a\x194a\x19/a\x19@\x94a\x19'a\x16\xFFV[P`\na\x18\xD9V[a\x18\xEFV[\x91a*\x90V[\x90a+\xA5V[a\x18\xF2V[\x90V[\x90a\x19b\x91a\x19]a\x19Xa\rkV[a'\xF0V[a\x19\xCDV[V[a\x19xa\x19sa\x19}\x92a\x0F\xF4V[a\x08\x90V[a\x04hV[\x90V[a\x19\x89\x90a\x19dV[\x90V[a\x19\xA0a\x19\x9Ba\x19\xA5\x92a\x0F\xF4V[a\x08\x90V[a\x04\xA2V[\x90V[a\x19\xB7a\x19\xBD\x91\x93\x92\x93a\x04\xA2V[\x92a\x04\xA2V[\x82\x01\x80\x92\x11a\x19\xC8WV[a\x14\x99V[\x90\x81a\x19\xE9a\x19\xE3a\x19\xDE_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14a\x1AyW\x80a\x1A\x01a\x19\xFB_a\x19\x8CV[\x91a\x04\xA2V[\x14a\x1A]Wa\x1A\x18a\x1A\x11a\x17)V[\x82\x90a\x19\xA8V[a\x1A1a\x1A+a\x1A&a\x0E#V[a\x04\xA2V[\x91a\x04\xA2V[\x11a\x1AAWa\x1A?\x91a,\xCCV[V[_c\x17~?\xC3`\xE0\x1B\x81R\x80a\x1AY`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1Au`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x1A\x91`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x90a\x1A\x9F\x91a\x19HV[V[a\x1A\xB0a\x1A\xB6\x91\x93\x92\x93a\x04\xA2V[\x92a\x04\xA2V[\x82\x03\x91\x82\x11a\x1A\xC1WV[a\x14\x99V[a\x1A\xCEa\x16\xFFV[Pa\x1A\xE8a\x1A\xDAa\x0E#V[a\x1A\xE2a\x17)V[\x90a\x1A\xA1V[\x90V[\x90a\x1A\xFEa\x1A\xF7a\x03\x02V[\x92\x83a\x16pV[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1B\x1EWa\x1B\x1A` \x91a\x03\xE0V[\x01\x90V[a\x16\\V[\x90a\x1B5a\x1B0\x83a\x1B\0V[a\x1A\xEBV[\x91\x82RV[_\x7Fmode=blocknumber&from=default\0\0\0\x91\x01RV[a\x1Bk`\x1Da\x1B#V[\x90a\x1Bx` \x83\x01a\x1B:V[V[a\x1B\x82a\x1BaV[\x90V[a\x1B\x8Da\x15nV[Pa\x1B\x96a\"\x06V[a\x1B\xAFa\x1B\xA9a\x1B\xA4a-*V[a\x0E\xB1V[\x91a\x0E\xB1V[\x03a\x1B\xBFWa\x1B\xBCa\x1BzV[\x90V[_c\x01\xBF\xC1\xC5`\xE6\x1B\x81R\x80a\x1B\xD7`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_\x90V[\x90a\x1B\xE9\x90a\x18\xCDV[_R` R`@_ \x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x1C\x0Ca\x1C\x11\x91a\x17\x03V[a\x1B\xF5V[\x90V[a\x1C\x1E\x90Ta\x1C\0V[\x90V[a\x1C8a\x1C=\x91a\x1C0a\x1B\xDBV[P`\ta\x1B\xDFV[a\x1C\x14V[\x90V[a\x1CQ\x90a\x1CLa&jV[a-}V[V[_\x90V[a\x1Ci\x90a\x1Cca\x1CSV[Pa.\x08V[\x90V[\x90a\x1Cv\x90a\x18\xCDV[_R` R`@_ \x90V[a\x1C\x98a\x1C\x9D\x91a\x1C\x91a\x16\xFFV[P_a\x1ClV[a\x17\x1CV[\x90V[\x90a\x1C\xBA\x91a\x1C\xB5a\x1C\xB0a\r\x07V[a'\xF0V[a\x1C\xBCV[V[\x80a\x1C\xD7a\x1C\xD1a\x1C\xCC_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14a\x1D\x9BW\x81a\x1C\xEFa\x1C\xE9_a\x19\x8CV[\x91a\x04\xA2V[\x14a\x1D\x7FWa\x1D\x05a\x1C\xFFa\x1EPV[\x15a\x03]V[a\x1DcWa\x1D\x14\x81\x83\x90a.7V[3\x90a\x1D^a\x1DLa\x1DF\x7F\xBE\xF4\xF8\x1C\x18\x14\xC6A\xED\xE8^\xBA\xAC\xF1\x9D\x04\x8B,[U\x98\n\xDF\xA6\xEF\x0F\x95le\x135\xA2\x93a\x18\xCDV[\x93a\x18\xCDV[\x93a\x1DUa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xA3V[_c\xB8\xB5\xCA-`\xE0\x1B\x81R\x80a\x1D{`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1D\x97`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x1D\xB3`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x90a\x1D\xC1\x91a\x1C\xA0V[V[a\x1D\xCBa\x16\xFFV[Pa\x1D\xD6`\x0Ca\x17\x1CV[a\x1D\xE8a\x1D\xE2_a\x19\x8CV[\x91a\x04\xA2V[\x14\x80\x15a\x1E\x17W[a\x1E\x0BWa\x1E\x08a\x1E\x01`\x0Ca\x17\x1CV[B\x90a\x1A\xA1V[\x90V[a\x1E\x14_a\x19\x8CV[\x90V[PBa\x1E4a\x1E.a\x1E)`\x0Ca\x17\x1CV[a\x04\xA2V[\x91a\x04\xA2V[\x10\x15a\x1D\xF0V[a\x1EM\x90a\x1EGa\x16\xFFV[Pa.\x96V[\x90V[a\x1EXa\x15*V[Pa\x1Ec`\x0Ca\x17\x1CV[a\x1Eua\x1Eo_a\x19\x8CV[\x91a\x04\xA2V[\x14\x15\x80a\x1E\x80W[\x90V[PBa\x1E\x9Da\x1E\x97a\x1E\x92`\x0Ca\x17\x1CV[a\x04\xA2V[\x91a\x04\xA2V[\x10a\x1E}V[a\x1E\xBC\x90a\x1E\xB7a\x1E\xB2a\x10\x18V[a'\xF0V[a\x1F6V[V[\x90a\x1E\xCA_\x19\x91a\x0F\xF7V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x1E\xE8a\x1E\xE3a\x1E\xED\x92a\x04\xA2V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[\x90a\x1F\x08a\x1F\x03a\x1F\x0F\x92a\x1E\xD4V[a\x1E\xF0V[\x82Ta\x1E\xBEV[\x90UV[\x91` a\x1F4\x92\x94\x93a\x1F-`@\x82\x01\x96_\x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[a\x1F@`\x0Ca\x17\x1CV[a\x1FRa\x1FL_a\x19\x8CV[\x91a\x04\xA2V[\x03a <W\x80a\x1Fja\x1FdBa\x04\xA2V[\x91a\x04\xA2V[\x11\x15a  W\x80a\x1F\xA3a\x1F\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xA2V[\x91a\x04\xA2V[\x11a \x04Wa\x1F\xB2`\x0Ca\x17\x1CV[a\x1F\xBD\x82`\x0Ca\x1E\xF3V[\x903\x90a\x1F\xEA\x7F\xDDh\x96\xDC\xF1\xD4\xB3\x11\xCC\xA8}\xD1\x9B\xBB\xA2\xEA\x9C\xE2\xF8g\xC1V\x88x\xA0C\x8Af\xA1\xAF\xEE\xEC\x92a\x18\xCDV[\x92a\x1F\xFFa\x1F\xF6a\x03\x02V[\x92\x83\x92\x83a\x1F\x13V[\x03\x90\xA2V[_c\xEFi\xAFe`\xE0\x1B\x81R\x80a \x1C`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\xA5e\x83S`\xE0\x1B\x81R\x80a 8`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c7\xE9x\xD3`\xE1\x1B\x81R\x80a T`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[a a\x90a\x1E\xA3V[V[_\x90V[``\x90V[a u\x90a\x18\xC1V[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a \x90W` \x80\x91\x02\x01\x90V[a\x16\\V[\x90a \xA7a \xA2\x83a xV[a\x1A\xEBV[\x91\x82RV[6\x907V[\x90a \xD6a \xBE\x83a \x95V[\x92` \x80a \xCC\x86\x93a xV[\x92\x01\x91\x03\x90a \xACV[V[`\x0F`\xF8\x1B\x90V[a \xE8a cV[Pa \xF1a\x15nV[Pa \xFAa\x15nV[Pa!\x03a\x16\xFFV[Pa!\x0Ca\x1B\xDBV[Pa!\x15a\x17nV[Pa!\x1Ea gV[Pa!'a.\xAEV[\x90a!0a.\xEEV[\x90F\x90a!<0a lV[\x90a!F_a\x0F\xFCV[\x90a!Xa!S_a\x19\x8CV[a \xB1V[\x90a!aa \xD8V[\x96\x95\x94\x93\x92\x91\x90V[a!\x93a!\x98\x91a!ya\x16\xFFV[Pa!\x8Da!\x87`\x0Ba\x18\xEFV[\x91a*\x90V[\x90a+\xA5V[a\x18\xF2V[\x90V[\x90a!\xA5\x90a\x18\xCDV[_R` R`@_ \x90V[`\xFF\x16\x90V[a!\xC3a!\xC8\x91a\x17\x03V[a!\xB1V[\x90V[a!\xD5\x90Ta!\xB7V[\x90V[a!\xFF\x91_a!\xF4a!\xFA\x93a!\xECa\x15*V[P`\x05a\x17~V[\x01a!\x9BV[a!\xCBV[\x90V[_\x90V[a\"\x0Ea\"\x02V[Pa\"\x17a-*V[\x90V[a\"\"a\x15nV[Pa\"-`\x04a\x16\xBBV[\x90V[a\"Wa\"Ra\"Ma\"\\\x93a\"Ea\x16\xFFV[P`\na\x18\xD9V[a\x18\xEFV[a/.V[a\x18\xF2V[\x90V[a\"|\x91a\"ka\x15*V[Pa\"ta&jV[\x91\x90\x91a'SV[`\x01\x90V[\x90a\"\x94\x91a\"\x8Ea\x16\xFFV[Pa\x19\x0EV[\x90V[a\"\xA9\x90a\"\xA3a\x16\xFFV[Pa\"0V[\x90V[a\"\xB4a\x16\xFFV[Pa\"\xBDa\x17)V[\x90V[\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF\x90V[a#\x19a# \x94a#\x0F``\x94\x98\x97\x95a#\x05`\x80\x86\x01\x9A_\x87\x01\x90a\x066V[` \x85\x01\x90a\t\x1DV[`@\x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[` \x01\x90V[Q\x90V[\x93\x95\x94\x90\x92\x91\x95Ba#Fa#@\x89a\x04\xA2V[\x91a\x04\xA2V[\x11a#\xBFW\x91a#\xB1\x91a#\xB8\x93a#\xA8a#\xBD\x98\x99a#\x90a#ga\"\xC0V[a#\x81\x8B\x93\x8Ba#ua\x03\x02V[\x95\x86\x94` \x86\x01a\"\xE4V[` \x82\x01\x81\x03\x82R\x03\x82a\x16pV[a#\xA2a#\x9C\x82a#(V[\x91a#\"V[ a/\xA3V[\x92\x90\x91\x92a/\xC0V[\x91\x82a0\nV[a-}V[V[a#\xDA\x87_\x91\x82\x91c#A\xD7\x87`\xE1\x1B\x83R`\x04\x83\x01a\x058V[\x03\x90\xFD[\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x90V[\x91\x94a$Ja$T\x92\x98\x97\x95a$@`\xA0\x96a$6a$[\x9Aa$,`\xC0\x8A\x01\x9E_\x8B\x01\x90a\x066V[` \x89\x01\x90a\t\x1DV[`@\x87\x01\x90a\t\x1DV[``\x85\x01\x90a\x05+V[`\x80\x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[\x91` a$~\x92\x94\x93a$w`@\x82\x01\x96_\x83\x01\x90a\t\x1DV[\x01\x90a\t\x1DV[V[\x96\x95\x91\x93\x92\x94\x90\x94Ba$\x9Ba$\x95\x83a\x04\xA2V[\x91a\x04\xA2V[\x11a%UW\x90a%\x04a%\r\x94\x93\x92a$\xECa$\xB5a#\xDEV[a$\xDD\x8C\x80\x94\x8C\x91a$\xC7\x8D\x91a0\\V[\x91\x92a$\xD1a\x03\x02V[\x97\x88\x96` \x88\x01a$\x02V[` \x82\x01\x81\x03\x82R\x03\x82a\x16pV[a$\xFEa$\xF8\x82a#(V[\x91a#\"V[ a/\xA3V[\x92\x90\x91\x92a/\xC0V[\x80a% a%\x1A\x87a\x04sV[\x91a\x04sV[\x03a%5WPa%3\x92\x93\x91\x90\x91a&wV[V[\x84\x90a%Q_\x92\x83\x92c%\xC0\x07#`\xE1\x1B\x84R`\x04\x84\x01a$]V[\x03\x90\xFD[a%p\x90_\x91\x82\x91c1<\x89\x81`\xE1\x1B\x83R`\x04\x83\x01a\x058V[\x03\x90\xFD[\x90a%\x8F\x91a%\x8Aa%\x85\x82a\x17\xB8V[a'\xF0V[a%\x91V[V[\x90a%\x9B\x91a)\xAFV[PV[\x90a%\xA8\x91a%tV[V[\x90a%\xB4\x90a\x18\xCDV[_R` R`@_ \x90V[a%\xE5\x91a%\xDBa%\xE0\x92a%\xD3a\x16\xFFV[P`\x01a%\xAAV[a\x1ClV[a\x17\x1CV[\x90V[a%\xF2`@a\x1A\xEBV[\x90V[_\x90V[_\x90V[a&\x05a%\xE8V[\x90` \x80\x83a&\x12a%\xF5V[\x81R\x01a&\x1Da%\xF9V[\x81RPPV[a&+a%\xFDV[\x90V[\x90a&A\x91a&;a&#V[Pa0\x8FV[\x90V[a&La\x15*V[Pa&fa&`c\x01\xFF\xC9\xA7`\xE0\x1Ba\x03\x10V[\x91a\x03\x10V[\x14\x90V[a&ra\x1B\xDBV[P3\x90V[\x91a&\x85\x92\x91`\x01\x92a0\xB7V[V[`@\x90a&\xB0a&\xB7\x94\x96\x95\x93\x96a&\xA6``\x84\x01\x98_\x85\x01\x90a\t\x1DV[` \x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[\x90a&\xC4\x91\x03a\x04\xA2V[\x90V[\x92\x91\x92a&\xD5\x81\x83\x90a%\xC0V[\x90\x81a&\xEAa&\xE4_\x19a\x04\xA2V[\x91a\x04\xA2V[\x10a&\xF7W[PPP\x90PV[\x81a'\na'\x04\x87a\x04\xA2V[\x91a\x04\xA2V[\x10a'0Wa''\x93\x94a'\x1F\x91\x93\x92a&\xB9V[\x90_\x92a0\xB7V[\x80_\x80\x80a&\xF0V[Pa'O\x84\x92\x91\x92_\x93\x84\x93c}\xC7\xA0\xD9`\xE1\x1B\x85R`\x04\x85\x01a&\x87V[\x03\x90\xFD[\x91\x82a'oa'ia'd_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14a'\xC9W\x81a'\x8Fa'\x89a'\x84_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14a'\xA2Wa'\xA0\x92\x91\x90\x91a1\xC6V[V[a'\xC5a'\xAE_a\x19\x80V[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\t*V[\x03\x90\xFD[a'\xECa'\xD5_a\x19\x80V[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\t*V[\x03\x90\xFD[a(\x02\x90a'\xFCa&jV[\x90a2\x93V[V[\x90a(\x10`\xFF\x91a\x0F\xF7V[\x91\x81\x19\x16\x91\x16\x17\x90V[a(#\x90a\x03]V[\x90V[\x90V[\x90a(>a(9a(E\x92a(\x1AV[a(&V[\x82Ta(\x04V[\x90UV[a(Qa\x15*V[Pa(fa(`\x82\x84\x90a!\xD8V[\x15a\x03]V[_\x14a(\xEFWa(\x8E`\x01a(\x89_a(\x81`\x05\x86\x90a\x17~V[\x01\x85\x90a!\x9BV[a()V[\x90a(\x97a&jV[\x90a(\xD4a(\xCEa(\xC8\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x95a\x17rV[\x92a\x18\xCDV[\x92a\x18\xCDV[\x92a(\xDDa\x03\x02V[\x80a(\xE7\x81a\x06\xBAV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a(\xFDa\x17nV[Pa)\x070a lV[a)9a)3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04sV[\x91a\x04sV[\x14\x80a)uW[_\x14a)jW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a)ra3?V[\x90V[PFa)\xA9a)\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xA2V[\x91a\x04\xA2V[\x14a)@V[a)\xB7a\x15*V[Pa)\xC3\x81\x83\x90a!\xD8V[_\x14a*KWa)\xEA_a)\xE5_a)\xDD`\x05\x86\x90a\x17~V[\x01\x85\x90a!\x9BV[a()V[\x90a)\xF3a&jV[\x90a*0a**a*$\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x95a\x17rV[\x92a\x18\xCDV[\x92a\x18\xCDV[\x92a*9a\x03\x02V[\x80a*C\x81a\x06\xBAV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a*ea*`a*j\x92a\x0E\xB1V[a\x08\x90V[a\x04\xA2V[\x90V[\x91` a*\x8E\x92\x94\x93a*\x87`@\x82\x01\x96_\x83\x01\x90a\x05+V[\x01\x90a\x0E\xBCV[V[a*\x98a\"\x02V[Pa*\xA1a\"\x06V[\x81a*\xB4a*\xAE\x83a*QV[\x91a\x04\xA2V[\x10\x15a*\xC7WPa*\xC4\x90a4HV[\x90V[\x90a*\xE2_\x92\x83\x92cvi\xFC\x0F`\xE1\x1B\x84R`\x04\x84\x01a*mV[\x03\x90\xFD[T\x90V[\x90V[a+\x01a*\xFCa+\x06\x92a*\xEAV[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a+#a+(\x91a\x17\x03V[a+\x0CV[\x90V[a+5\x90Ta+\x17V[\x90V[\x90V[a+Oa+Ja+T\x92a+8V[a\x08\x90V[a\x04\xA2V[\x90V[`0\x1C\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a+ta+y\x91a+WV[a+]V[\x90V[a+\x86\x90Ta+hV[\x90V[a+\x9Da+\x98a+\xA2\x92a\x0F\xF4V[a\x08\x90V[a\x14\nV[\x90V[\x90a+\xF9\x90a+\xB2a\x14\x95V[Pa+\xBE_\x84\x01a*\xE6V[a+\xC7_a\x19\x8CV[\x90\x80\x80a+\xDDa+\xD7`\x05a*\xEDV[\x91a\x04\xA2V[\x11a,ZW[P\x90a+\xF4_\x86\x01\x93\x91\x92\x93a+\tV[a:\x9BV[\x80a,\x0Ca,\x06_a\x19\x8CV[\x91a\x04\xA2V[\x14_\x14a,\"WPPa,\x1E_a+\x89V[[\x90V[a,O_\x91a,Ja,D\x84a,U\x96\x01\x92a,>`\x01a+;V[\x90a\x1A\xA1V[\x91a+\tV[a:\x91V[\x01a+|V[a,\x1FV[\x80a,ha,n\x92\x91a7&V[\x90a\x1A\xA1V[\x90\x83a,\xA0a,\x9Aa,\x95_a,\x8F\x81\x8C\x01a,\x8A\x89\x91a+\tV[a:\x91V[\x01a++V[a\x0E\xB1V[\x91a\x0E\xB1V[\x10_\x14a,\xB1WP\x90[\x90_a+\xE3V[\x91Pa,\xC7\x90a,\xC1`\x01a+;V[\x90a\x19\xA8V[a,\xAAV[\x80a,\xE7a,\xE1a,\xDC_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14a-\x03Wa-\x01\x91a,\xF9_a\x19\x80V[\x91\x90\x91a1\xC6V[V[a-&a-\x0F_a\x19\x80V[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\t*V[\x03\x90\xFD[a-2a\"\x02V[Pa-<Ca4HV[\x90V[\x90a-P`\x01\x80`\xA0\x1B\x03\x91a\x0F\xF7V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a-ra-ma-y\x92a\x18\xCDV[a-ZV[\x82Ta-?V[\x90UV[\x90a.\x06\x91a.\0a-\x8E\x82a\x1C!V[a-\xA3\x84a-\x9E`\t\x86\x90a\x1B\xDFV[a-]V[\x82\x81\x85\x90a-\xE3a-\xDDa-\xD7\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x95a\x18\xCDV[\x92a\x18\xCDV[\x92a\x18\xCDV[\x92a-\xECa\x03\x02V[\x80a-\xF6\x81a\x06\xBAV[\x03\x90\xA4\x92\x91a;*V[\x91a;BV[V[a./a.*a.%a.4\x93a.\x1Da\x1CSV[P`\na\x18\xD9V[a\x18\xEFV[a<\xF0V[a=oV[\x90V[\x90\x81a.Sa.Ma.H_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14a.oWa.m\x91\x90a.f_a\x19\x80V[\x90\x91a1\xC6V[V[a.\x92a.{_a\x19\x80V[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\t*V[\x03\x90\xFD[a.\xA8\x90a.\xA2a\x16\xFFV[Pa=\xC0V[\x90V[\x90V[a.\xB6a\x15nV[Pa.\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a.\xE5`\x06a.\xABV[\x90a>\xDBV[\x90V[a.\xF6a\x15nV[Pa/+\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a/%`\x07a.\xABV[\x90a>\xDBV[\x90V[a/6a\x14\x95V[Pa/B_\x82\x01a*\xE6V[\x80a/Ua/O_a\x19\x8CV[\x91a\x04\xA2V[\x14_\x14a/kWPPa/g_a+\x89V[[\x90V[a/\x98_\x91a/\x93a/\x8D\x84a/\x9E\x96\x01\x92a/\x87`\x01a+;V[\x90a\x1A\xA1V[\x91a+\tV[a:\x91V[\x01a+|V[a/hV[a/\xBD\x90a/\xAFa\x17nV[Pa/\xB8a(\xF5V[a?)V[\x90V[\x92a/\xDB\x92a/\xE4\x94a/\xD1a\x1B\xDBV[P\x92\x90\x91\x92a?\xEFV[\x90\x92\x91\x92aA\x1AV[\x90V[\x91` a0\x08\x92\x94\x93a0\x01`@\x82\x01\x96_\x83\x01\x90a\t\x1DV[\x01\x90a\x05+V[V[a0\x13\x81a0\\V[\x91a0&a0 \x84a\x04\xA2V[\x91a\x04\xA2V[\x03a0/WPPV[a0I_\x92\x83\x92c\x01\xD4\xB6#`\xE6\x1B\x84R`\x04\x84\x01a/\xE7V[\x03\x90\xFD[`\x01a0Y\x91\x01a\x04\xA2V[\x90V[a0p\x90a0ha\x16\xFFV[P`\x08a\x1ClV[a0\x8Ca0|\x82a\x17\x1CV[\x91a0\x86\x83a0MV[\x90a\x1E\xF3V[\x90V[\x90a0\xAFa0\xAAa0\xB4\x93a0\xA2a&#V[P`\na\x18\xD9V[a\x18\xEFV[aB\x90V[\x90V[\x90\x92\x81a0\xD4a0\xCEa0\xC9_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14a1\x9FW\x83a0\xF4a0\xEEa0\xE9_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14a1xWa1\x18\x83a1\x13a1\x0C`\x01\x86\x90a%\xAAV[\x87\x90a\x1ClV[a\x1E\xF3V[a1\"W[PPPV[\x91\x90\x91a1ma1[a1U\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x93a\x18\xCDV[\x93a\x18\xCDV[\x93a1da\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xA3_\x80\x80a1\x1DV[a1\x9Ba1\x84_a\x19\x80V[_\x91\x82\x91cJ\x14\x06\xB1`\xE1\x1B\x83R`\x04\x83\x01a\t*V[\x03\x90\xFD[a1\xC2a1\xAB_a\x19\x80V[_\x91\x82\x91c\xE6\x02\xDF\x05`\xE0\x1B\x83R`\x04\x83\x01a\t*V[\x03\x90\xFD[\x91\x82a1\xE2a1\xDCa1\xD7_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14\x15\x80a2MW[a1\xFDW[a1\xFB\x92\x91\x90\x91aB\xB1V[V[a2\x05a\x1EPV[\x80a2,W[\x15a1\xEFW_c6\xE2x\xFD`\xE2\x1B\x81R\x80a2(`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[Pa2Ha2Ba2;a\r\x07V[3\x90a!\xD8V[\x15a\x03]V[a2\x0BV[P\x81a2ia2ca2^_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14\x15a1\xEAV[\x91` a2\x91\x92\x94\x93a2\x8A`@\x82\x01\x96_\x83\x01\x90a\t\x1DV[\x01\x90a\x066V[V[\x90a2\xA8a2\xA2\x83\x83\x90a!\xD8V[\x15a\x03]V[a2\xB0WPPV[a2\xCA_\x92\x83\x92c\xE2Q}?`\xE0\x1B\x84R`\x04\x84\x01a2pV[\x03\x90\xFD[\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x90V[\x90\x95\x94\x92a3=\x94a3,a36\x92a3\"`\x80\x96a3\x18`\xA0\x88\x01\x9C_\x89\x01\x90a\x066V[` \x87\x01\x90a\x066V[`@\x85\x01\x90a\x066V[``\x83\x01\x90a\x05+V[\x01\x90a\t\x1DV[V[a3Ga\x17nV[Pa3Pa2\xCEV[a3\xC7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a3\xB8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Fa3\xA30a lV[\x91a3\xACa\x03\x02V[\x96\x87\x95` \x87\x01a2\xF2V[` \x82\x01\x81\x03\x82R\x03\x82a\x16pV[a3\xD9a3\xD3\x82a#(V[\x91a#\"V[ \x90V[\x90V[a3\xF4a3\xEFa3\xF9\x92a3\xDDV[a\x08\x90V[a\x06\xF3V[\x90V[a4\x05\x90a3\xE0V[\x90RV[\x91` a4*\x92\x94\x93a4#`@\x82\x01\x96_\x83\x01\x90a3\xFCV[\x01\x90a\x05+V[V[a4@a4;a4E\x92a\x04\xA2V[a\x08\x90V[a\x0E\xB1V[\x90V[a4Pa\"\x02V[P\x80a4ja4de\xFF\xFF\xFF\xFF\xFF\xFFa*QV[\x91a\x04\xA2V[\x11a4{Wa4x\x90a4,V[\x90V[`0a4\x97_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a4\tV[\x03\x90\xFD[\x90V[a4\xB2a4\xADa4\xB7\x92a4\x9BV[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a4\xD1a4\xCCa4\xD6\x92a4\xBAV[a\x08\x90V[a\x06\xF3V[\x90V[a4\xF8\x90a4\xF2a4\xECa4\xFD\x94a\x06\xF3V[\x91a\x04\xA2V[\x90a\x10\x9AV[a\x04\xA2V[\x90V[\x90V[a5\x17a5\x12a5\x1C\x92a5\0V[a\x08\x90V[a\x06\xF3V[\x90V[\x1B\x90V[a5B\x90a5<a56a5G\x94a\x06\xF3V[\x91a\x04\xA2V[\x90a5\x1FV[a\x04\xA2V[\x90V[\x90V[a5aa5\\a5f\x92a5JV[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a5\x80a5{a5\x85\x92a5iV[a\x08\x90V[a\x06\xF3V[\x90V[\x90V[a5\x9Fa5\x9Aa5\xA4\x92a5\x88V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a5\xBEa5\xB9a5\xC3\x92a5\xA7V[a\x08\x90V[a\x06\xF3V[\x90V[\x90V[a5\xDDa5\xD8a5\xE2\x92a5\xC6V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a5\xFCa5\xF7a6\x01\x92a5\xE5V[a\x08\x90V[a\x06\xF3V[\x90V[\x90V[a6\x1Ba6\x16a6 \x92a6\x04V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a6:a65a6?\x92a6#V[a\x08\x90V[a\x06\xF3V[\x90V[a6Va6Qa6[\x92a5\xA7V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a6ua6pa6z\x92a6^V[a\x08\x90V[a\x06\xF3V[\x90V[a6\x91a6\x8Ca6\x96\x92a6#V[a\x08\x90V[a\x04\xA2V[\x90V[a6\xADa6\xA8a6\xB2\x92a+8V[a\x08\x90V[a\x06\xF3V[\x90V[\x90V[a6\xCCa6\xC7a6\xD1\x92a6\xB5V[a\x08\x90V[a\x04\xA2V[\x90V[\x90a6\xDF\x91\x02a\x04\xA2V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a7\x02a7\x08\x91a\x04\xA2V[\x91a\x04\xA2V[\x90\x81\x15a7\x13W\x04\x90V[a6\xE2V[\x90a7#\x91\x01a\x04\xA2V[\x90V[a7.a\x16\xFFV[P\x80a7Ca7=`\x01a+;V[\x91a\x04\xA2V[\x11\x15a:\x8EW\x80a9Xa95a9%a9\x15a9\x05a8\xF5a8\xE5a8\xD5a8\xC5a8\xB5a8\xA5\x8Ba8\x9Fa8\x98a9^\x9Fa8xa8ha8\x88\x92a7\x8A`\x01a+;V[\x90\x80a7\xA2a7\x9C`\x01`\x80\x1Ba4\x9EV[\x91a\x04\xA2V[\x10\x15a:`W[\x80a7\xC5a7\xBFh\x01\0\0\0\0\0\0\0\0a5MV[\x91a\x04\xA2V[\x10\x15a:2W[\x80a7\xE4a7\xDEd\x01\0\0\0\0a5\x8BV[\x91a\x04\xA2V[\x10\x15a:\x04W[\x80a8\x01a7\xFBb\x01\0\0a5\xC9V[\x91a\x04\xA2V[\x10\x15a9\xD6W[\x80a8\x1Da8\x17a\x01\0a6\x07V[\x91a\x04\xA2V[\x10\x15a9\xA8W[\x80a88a82`\x10a6BV[\x91a\x04\xA2V[\x10\x15a9zW[a8Ra8L`\x04a6}V[\x91a\x04\xA2V[\x10\x15a9aW[a8c`\x03a6\xB8V[a6\xD4V[a8r`\x01a6\x99V[\x90a4\xD9V[a8\x82\x81\x86a6\xF6V[\x90a7\x18V[a8\x92`\x01a6\x99V[\x90a4\xD9V[\x80\x92a6\xF6V[\x90a7\x18V[a8\xAF`\x01a6\x99V[\x90a4\xD9V[a8\xBF\x81\x8Ca6\xF6V[\x90a7\x18V[a8\xCF`\x01a6\x99V[\x90a4\xD9V[a8\xDF\x81\x8Aa6\xF6V[\x90a7\x18V[a8\xEF`\x01a6\x99V[\x90a4\xD9V[a8\xFF\x81\x88a6\xF6V[\x90a7\x18V[a9\x0F`\x01a6\x99V[\x90a4\xD9V[a9\x1F\x81\x86a6\xF6V[\x90a7\x18V[a9/`\x01a6\x99V[\x90a4\xD9V[\x91a9Ra9La9G\x85\x80\x94a6\xF6V[a\x04\xA2V[\x91a\x04\xA2V[\x11aCAV[\x90a&\xB9V[\x90V[a9u\x90a9o`\x01a6\x99V[\x90a5#V[a8YV[a9\x91a9\xA2\x91a9\x8B`\x04a6&V[\x90a4\xD9V[\x91a9\x9C`\x02a6aV[\x90a5#V[\x90a8?V[a9\xBFa9\xD0\x91a9\xB9`\x08a5\xE8V[\x90a4\xD9V[\x91a9\xCA`\x04a6&V[\x90a5#V[\x90a8$V[a9\xEDa9\xFE\x91a9\xE7`\x10a5\xAAV[\x90a4\xD9V[\x91a9\xF8`\x08a5\xE8V[\x90a5#V[\x90a8\x08V[a:\x1Ba:,\x91a:\x15` a5lV[\x90a4\xD9V[\x91a:&`\x10a5\xAAV[\x90a5#V[\x90a7\xEBV[a:Ia:Z\x91a:C`@a5\x03V[\x90a4\xD9V[\x91a:T` a5lV[\x90a5#V[\x90a7\xCCV[a:wa:\x88\x91a:q`\x80a4\xBDV[\x90a4\xD9V[\x91a:\x82`@a5\x03V[\x90a5#V[\x90a7\xA9V[\x90V[_R` _ \x01\x90V[\x93\x91\x90\x92a:\xA7a\x16\xFFV[P[\x81a:\xBCa:\xB6\x83a\x04\xA2V[\x91a\x04\xA2V[\x10\x15a;\"Wa:\xCD\x82\x82\x90aC\x8DV[\x90a:\xE3_a:\xDD\x88\x85\x90a:\x91V[\x01a++V[a:\xF5a:\xEF\x87a\x0E\xB1V[\x91a\x0E\xB1V[\x11_\x14a;\x05WP\x91[\x91a:\xA9V[\x92\x91Pa;\x1C\x90a;\x16`\x01a+;V[\x90a\x19\xA8V[\x90a:\xFFV[\x92PP\x91P\x90V[a;<\x90a;6a\x16\xFFV[Pa\x1C\x82V[\x90V[\x90V[\x91\x90\x91\x80a;Xa;R\x85a\x04sV[\x91a\x04sV[\x14\x15\x80a<\xD6W[a;jW[PPPV[\x80a;\x85a;\x7Fa;z_a\x19\x80V[a\x04sV[\x91a\x04sV[\x03a<FW[P\x81a;\xA7a;\xA1a;\x9C_a\x19\x80V[a\x04sV[\x91a\x04sV[\x03a;\xB3W[\x80a;eV[a;\xFAa;\xEDa;\xF4\x92a;\xC9`\n\x86\x90a\x18\xD9V[\x90a;\xE7a;\xE1a;\xDB`\x01\x93aD&V[\x93a\x18\xEFV[\x91a;?V[\x90aDyV[\x92\x90a\x18\xF2V[\x91a\x18\xF2V[\x91\x90\x91a<'\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x18\xCDV[\x92a<<a<3a\x03\x02V[\x92\x83\x92\x83a\x1F\x13V[\x03\x90\xA2_\x80a;\xADV[a<\x85a<\x8Ba<~a<[`\n\x85\x90a\x18\xD9V[`\x02a<xa<ra<l\x89aD&V[\x93a\x18\xEFV[\x91a;?V[\x90aDyV[\x92\x90a\x18\xF2V[\x91a\x18\xF2V[\x91\x90\x91a<\xB8\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x18\xCDV[\x92a<\xCDa<\xC4a\x03\x02V[\x92\x83\x92\x83a\x1F\x13V[\x03\x90\xA2_a;\x8BV[P\x81a<\xEAa<\xE4_a\x19\x8CV[\x91a\x04\xA2V[\x11a;`V[_a=\x04\x91a<\xFDa\x16\xFFV[P\x01a*\xE6V[\x90V[a=\x1Ba=\x16a= \x92a\t\xA7V[a\x08\x90V[a\x04\xA2V[\x90V[a=,\x90a5lV[\x90RV[\x91` a=Q\x92\x94\x93a=J`@\x82\x01\x96_\x83\x01\x90a=#V[\x01\x90a\x05+V[V[a=ga=ba=l\x92a\x04\xA2V[a\x08\x90V[a\t\xA7V[\x90V[a=wa\x1CSV[P\x80a=\x8Fa=\x89c\xFF\xFF\xFF\xFFa=\x07V[\x91a\x04\xA2V[\x11a=\xA0Wa=\x9D\x90a=SV[\x90V[` a=\xBC_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a=0V[\x03\x90\xFD[a=\xD7a=\xDC\x91a=\xCFa\x16\xFFV[P`\x08a\x1ClV[a\x17\x1CV[\x90V[\x90V[a=\xF6a=\xF1a=\xFB\x92a=\xDFV[a\x0F\xF7V[a\x05\xF2V[\x90V[a>\x08`\xFFa=\xE2V[\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a>.a>'\x83a\x15\x87V[\x80\x94a\x15\xB1V[\x91`\x01\x81\x16\x90\x81_\x14a>\x85WP`\x01\x14a>IW[PPPV[a>V\x91\x92\x93\x94Pa>\x0BV[\x91_\x92[\x81\x84\x10a>mWPP\x01\x90_\x80\x80a>DV[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a>ZV[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a>DV[\x90a>\xAA\x91a>\x14V[\x90V[\x90a>\xCDa>\xC6\x92a>\xBDa\x03\x02V[\x93\x84\x80\x92a>\xA0V[\x03\x83a\x16pV[V[a>\xD8\x90a>\xADV[\x90V[\x90a>\xE4a\x15nV[Pa>\xEE\x82a\x17rV[a?\x07a?\x01a>\xFCa=\xFEV[a\x05\xF2V[\x91a\x05\xF2V[\x14\x15_\x14a?\x1CWPa?\x19\x90aE\x03V[\x90V[a?&\x91Pa>\xCFV[\x90V[`B\x91a?4a\x17nV[P`@Q\x91\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[_\x90V[a?za?\x7F\x91a\x17\x03V[a\x1E\xD4V[\x90V[\x90V[a?\x99a?\x94a?\x9E\x92a?\x82V[a\x08\x90V[a\x04\xA2V[\x90V[a?\xD6a?\xDD\x94a?\xCC``\x94\x98\x97\x95a?\xC2`\x80\x86\x01\x9A_\x87\x01\x90a\x066V[` \x85\x01\x90a\x06\xF9V[`@\x83\x01\x90a\x066V[\x01\x90a\x066V[V[a?\xE7a\x03\x02V[=_\x82>=\x90\xFD[\x93\x92\x93a?\xFAa\x1B\xDBV[Pa@\x03a?jV[Pa@\x0Ca\x17nV[Pa@\x16\x85a?nV[a@Ha@B\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0a?\x85V[\x91a\x04\xA2V[\x11a@\xD5W\x90a@k` \x94\x95_\x94\x93\x92\x93a@ba\x03\x02V[\x94\x85\x94\x85a?\xA1V[\x83\x80R\x03\x90`\x01Z\xFA\x15a@\xD0Wa@\x83_Qa\x0F\xF7V[\x80a@\x9Ea@\x98a@\x93_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14a@\xB4W_\x91a@\xAE_a\x0F\xFCV[\x91\x92\x91\x90V[Pa@\xBE_a\x19\x80V[`\x01\x91a@\xCA_a\x0F\xFCV[\x91\x92\x91\x90V[a?\xDFV[PPPa@\xE1_a\x19\x80V[\x90`\x03\x92\x91\x92\x91\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x04\x11\x15aA\tWV[a@\xEBV[\x90aA\x18\x82a@\xFFV[V[\x80aA-aA'_aA\x0EV[\x91aA\x0EV[\x14_\x14aA8WPPV[\x80aALaAF`\x01aA\x0EV[\x91aA\x0EV[\x14_\x14aAoW_c\xF6E\xEE\xDF`\xE0\x1B\x81R\x80aAk`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x80aA\x83aA}`\x02aA\x0EV[\x91aA\x0EV[\x14_\x14aA\xB1WaA\xADaA\x96\x83a?nV[_\x91\x82\x91c\xFC\xE6\x98\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x058V[\x03\x90\xFD[aA\xC4aA\xBE`\x03aA\x0EV[\x91aA\x0EV[\x14aA\xCCWPV[aA\xE7\x90_\x91\x82\x91c5\xE2\xF3\x83`\xE2\x1B\x83R`\x04\x83\x01a\x06CV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_R` _ \x90V[aB\x11\x81a*\xE6V[\x82\x10\x15aB+WaB#`\x01\x91aA\xFFV[\x91\x02\x01\x90_\x90V[aA\xEBV[\x90aB:\x90a\x0E\xB1V[\x90RV[\x90aBH\x90a\x14\nV[\x90RV[\x90aB\x82aBy_aB\\a%\xE8V[\x94aBsaBk\x83\x83\x01a++V[\x83\x88\x01aB0V[\x01a+|V[` \x84\x01aB>V[V[aB\x8D\x90aBLV[\x90V[aB\xAE\x91_aB\xA8\x92aB\xA1a&#V[P\x01aB\x08V[PaB\x84V[\x90V[\x92\x91aB\xBF\x84\x83\x83\x91aE3V[\x83aB\xDAaB\xD4aB\xCF_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14aB\xEFW[aB\xED\x92\x93\x91\x90\x91aF\xBDV[V[aB\xF7a\x17)V[\x93aC\0aF\xA2V[\x94\x80aC\x14aC\x0E\x88a\x04\xA2V[\x91a\x04\xA2V[\x11aC!WP\x93PaB\xE0V[\x85\x90aC=_\x92\x83\x92c\x0EX\xAE\x93`\xE1\x1B\x84R`\x04\x84\x01a\x1F\x13V[\x03\x90\xFD[aCIa\x16\xFFV[P\x15\x15\x90V[aCcaC^aCh\x92a6^V[a\x08\x90V[a\x04\xA2V[\x90V[aCwaC}\x91a\x04\xA2V[\x91a\x04\xA2V[\x90\x81\x15aC\x88W\x04\x90V[a6\xE2V[aC\xB2aC\xB8\x92aC\x9Ca\x16\xFFV[P\x82\x81\x16\x92\x18aC\xAC`\x02aCOV[\x90aCkV[\x90a\x19\xA8V[\x90V[\x90V[aC\xD2aC\xCDaC\xD7\x92aC\xBBV[a\x08\x90V[a\x06\xF3V[\x90V[aC\xE3\x90aC\xBEV[\x90RV[\x91` aD\x08\x92\x94\x93aD\x01`@\x82\x01\x96_\x83\x01\x90aC\xDAV[\x01\x90a\x05+V[V[aD\x1EaD\x19aD#\x92a\x04\xA2V[a\x08\x90V[a\x14\nV[\x90V[aD.a\x14\x95V[P\x80aDHaDB`\x01\x80`\xD0\x1B\x03a\x18\xF2V[\x91a\x04\xA2V[\x11aDYWaDV\x90aD\nV[\x90V[`\xD0aDu_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01aC\xE7V[\x03\x90\xFD[\x90aD\xAFaD\xB5\x93\x92aD\x8Aa\x14\x95V[PaD\x93a\x14\x95V[P\x80\x93aD\xA8aD\xA1a\"\x06V[\x94\x92a/.V[\x90\x91aK8V[\x91aG|V[\x91\x90\x91\x90V[aD\xCFaD\xCAaD\xD4\x92a5iV[a\x08\x90V[a\x04\xA2V[\x90V[6\x907V[\x90aE\x01aD\xE9\x83a\x1B#V[\x92` \x80aD\xF7\x86\x93a\x1B\0V[\x92\x01\x91\x03\x90aD\xD7V[V[aE\x0Ba\x15nV[PaE\x15\x81aG\xE6V[\x90aE(aE#` aD\xBBV[aD\xDCV[\x91\x82R` \x82\x01R\x90V[\x91\x90\x91\x80aEQaEKaEF_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14_\x14aF2WaEuaEn\x83aEi`\x02a\x17\x1CV[a\x19\xA8V[`\x02a\x1E\xF3V[[\x82aE\x91aE\x8BaE\x86_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14_\x14aF\x06WaE\xB5aE\xAE\x83aE\xA9`\x02a\x17\x1CV[a&\xB9V[`\x02a\x1E\xF3V[[\x91\x90\x91aF\x01aE\xEFaE\xE9\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93a\x18\xCDV[\x93a\x18\xCDV[\x93aE\xF8a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xA3V[aF-\x82aF'aF\x18_\x87\x90a\x1ClV[\x91aF\"\x83a\x17\x1CV[a7\x18V[\x90a\x1E\xF3V[aE\xB6V[aFEaF@_\x83\x90a\x1ClV[a\x17\x1CV[\x80aFXaFR\x85a\x04\xA2V[\x91a\x04\xA2V[\x10aF\x80WaFkaF{\x91\x84\x90a&\xB9V[aFv_\x84\x90a\x1ClV[a\x1E\xF3V[aEvV[\x90aF\x9E\x90\x91\x92_\x93\x84\x93c9\x144\xE3`\xE2\x1B\x85R`\x04\x85\x01a&\x87V[\x03\x90\xFD[aF\xAAa\x16\xFFV[PaF\xBA`\x01\x80`\xD0\x1B\x03a\x18\xF2V[\x90V[\x91aG\x15aG\x0FaG\x1C\x94\x80aF\xE3aF\xDDaF\xD8_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14aGMW[\x84aG\x04aF\xFEaF\xF9_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14aG\x1EW[a\x1C!V[\x92a\x1C!V[\x90\x91a;BV[V[aGF`\x0B`\x02aG@aG:aG4\x89aD&V[\x93a\x18\xEFV[\x91a;?V[\x90aDyV[PPaG\nV[aGu`\x0B`\x01aGoaGiaGc\x89aD&V[\x93a\x18\xEFV[\x91a;?V[\x90aDyV[PPaF\xE9V[\x91aG\xA0_aG\xA5\x94aG\x8Da\x14\x95V[PaG\x96a\x14\x95V[P\x01\x92\x91\x92a+\tV[aI\xEAV[\x91\x90\x91\x90V[aG\xBFaG\xBAaG\xC4\x92a=\xDFV[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[aG\xDEaG\xD9aG\xE3\x92aG\xC7V[a\x08\x90V[a\x04\xA2V[\x90V[aG\xFBaH\0\x91aG\xF5a\x16\xFFV[Pa\x17rV[a?nV[aH\n`\xFFaG\xABV[\x16\x80aH\x1FaH\x19`\x1FaG\xCAV[\x91a\x04\xA2V[\x11aH'W\x90V[_c,\xD4J\xC3`\xE2\x1B\x81R\x80aH?`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[T\x90V[aHQ`@a\x1A\xEBV[\x90V[_R` _ \x90V[aHf\x81aHCV[\x82\x10\x15aH\x80WaHx`\x01\x91aHTV[\x91\x02\x01\x90_\x90V[aA\xEBV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[aH\xA2\x90Qa\x0E\xB1V[\x90V[\x90aH\xB6e\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x0F\xF7V[\x91\x81\x19\x16\x91\x16\x17\x90V[aH\xD4aH\xCFaH\xD9\x92a\x0E\xB1V[a\x08\x90V[a\x0E\xB1V[\x90V[\x90V[\x90aH\xF4aH\xEFaH\xFB\x92aH\xC0V[aH\xDCV[\x82TaH\xA5V[\x90UV[aI\t\x90Qa\x14\nV[\x90V[`0\x1B\x90V[\x90aI$e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91aI\x0CV[\x91\x81\x19\x16\x91\x16\x17\x90V[aIBaI=aIG\x92a\x14\nV[a\x08\x90V[a\x14\nV[\x90V[\x90V[\x90aIbaI]aIi\x92aI.V[aIJV[\x82TaI\x12V[\x90UV[\x90aI\x97` _aI\x9D\x94aI\x8F\x82\x82\x01aI\x89\x84\x88\x01aH\x98V[\x90aH\xDFV[\x01\x92\x01aH\xFFV[\x90aIMV[V[\x91\x90aI\xB0WaI\xAE\x91aImV[V[aH\x85V[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15aI\xE5W\x82aI\xDD\x91`\x01aI\xE3\x95\x01\x81UaH]V[\x90aI\x9FV[V[a\x16\\V[\x90\x92\x91\x92aI\xF6a\x14\x95V[PaI\xFFa\x14\x95V[PaJ\t\x82aHCV[\x80aJ\x1CaJ\x16_a\x19\x8CV[\x91a\x04\xA2V[\x11_\x14aJ\xECWaJB\x90aJ<\x84\x91aJ6`\x01a+;V[\x90a\x1A\xA1V[\x90a:\x91V[\x90aJN_\x83\x01a++V[\x92aJZ_\x84\x01a+|V[\x93\x80aJnaJh\x85a\x0E\xB1V[\x91a\x0E\xB1V[\x11aJ\xD0WaJ\x85aJ\x7F\x84a\x0E\xB1V[\x91a\x0E\xB1V[\x14_\x14aJ\xA0WPPaJ\x9B\x90_\x85\x91\x01aIMV[[\x91\x90V[aJ\xCB\x92PaJ\xC6\x86aJ\xBDaJ\xB4aHGV[\x94_\x86\x01aB0V[` \x84\x01aB>V[aI\xB5V[aJ\x9CV[_c% `\x1D`\xE0\x1B\x81R\x80aJ\xE8`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[PaK\x17\x91aK\x12\x85aK\taK\0aHGV[\x94_\x86\x01aB0V[` \x84\x01aB>V[aI\xB5V[aK _a+\x89V[\x91\x90V[cNH{q`\xE0\x1B_R`Q`\x04R`$_\xFD[\x91\x90\x91\x80`\x01\x14aKWW`\x02\x03aK$WaKS\x91a\x15\x14V[\x90[V[PaKa\x91a\x14\xD5V[\x90aKUV",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b611491565b61001d5f356102fc565b806301ffc9a7146102f757806306fdde03146102f2578063095ea7b3146102ed57806318160ddd146102e857806323b872dd146102e3578063248a9ca3146102de5780632f2ff15d146102d9578063313ce567146102d45780633644e515146102cf57806336568abe146102ca5780633a46b1a8146102c557806340c10f19146102c05780634bdd36ce146102bb5780634bf5d7e9146102b65780634f1bfc9e146102b1578063587cde1e146102ac5780635c19a95c146102a75780636fcfff45146102a257806370a082311461029d57806379cc6790146102985780637a8cd156146102935780637ecebe001461028e57806383f1211b146102895780638426adf214610284578063844c90261461027f57806384b0196e1461027a5780638a542521146102755780638d3343d6146102705780638e539e8c1461026b578063902d55a51461026657806391d148541461026157806391ddadf41461025c57806395d89b41146102575780639ab24eb0146102525780639b7ef64b1461024d578063a217fddf14610248578063a9059cbb14610243578063aa082a9d1461023e578063b0ca253e14610239578063bb4d443614610234578063c02ae7541461022f578063c3cda5201461022a578063d505accf14610225578063d547741f14610220578063dd62ed3e1461021b5763f1127ed80361000e5761145b565b611377565b611316565b6112dc565b611232565b611176565b611141565b61110b565b6110d6565b611064565b61102f565b610fbf565b610f48565b610f13565b610ede565b610e7b565b610e46565b610dcf565b610d9a565b610d36565b610ccb565b610b86565b610b33565b610ada565b610aa5565b610a70565b610a3c565b610a07565b6109d2565b610974565b61093f565b6108ca565b610858565b610823565b6107ef565b6107b9565b610785565b610750565b61071b565b6106bf565b610658565b6105bc565b61054d565b6104f5565b610433565b610384565b60e01c90565b60405190565b5f80fd5b5f80fd5b63ffffffff60e01b1690565b61032581610310565b0361032c57565b5f80fd5b9050359061033d8261031c565b565b9060208282031261035857610355915f01610330565b90565b61030c565b151590565b61036b9061035d565b9052565b9190610382905f60208501940190610362565b565b346103b4576103b061039f61039a36600461033f565b61152e565b6103a7610302565b9182918261036f565b0390f35b610308565b5f9103126103c357565b61030c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b61040961041260209361041793610400816103c8565b938480936103cc565b958691016103d5565b6103e0565b0190565b6104309160208201915f8184039101526103ea565b90565b34610463576104433660046103b9565b61045f61044e6116c7565b610456610302565b9182918261041b565b0390f35b610308565b60018060a01b031690565b61047c90610468565b90565b61048881610473565b0361048f57565b5f80fd5b905035906104a08261047f565b565b90565b6104ae816104a2565b036104b557565b5f80fd5b905035906104c6826104a5565b565b91906040838203126104f057806104e46104ed925f8601610493565b936020016104b9565b90565b61030c565b346105265761052261051161050b3660046104c8565b906116dd565b610519610302565b9182918261036f565b0390f35b610308565b610534906104a2565b9052565b919061054b905f6020850194019061052b565b565b3461057d5761055d3660046103b9565b610579610568611729565b610570610302565b91829182610538565b0390f35b610308565b90916060828403126105b7576105b461059d845f8501610493565b936105ab8160208601610493565b936040016104b9565b90565b61030c565b346105ed576105e96105d86105d2366004610582565b9161173f565b6105e0610302565b9182918261036f565b0390f35b610308565b90565b6105fe816105f2565b0361060557565b5f80fd5b90503590610616826105f5565b565b906020828203126106315761062e915f01610609565b90565b61030c565b61063f906105f2565b9052565b9190610656905f60208501940190610636565b565b346106885761068461067361066e366004610618565b6117b8565b61067b610302565b91829182610643565b0390f35b610308565b91906040838203126106b557806106a96106b2925f8601610609565b93602001610493565b90565b61030c565b5f0190565b346106ee576106d86106d236600461068d565b90611804565b6106e0610302565b806106ea816106ba565b0390f35b610308565b60ff1690565b610702906106f3565b9052565b9190610719905f602085019401906106f9565b565b3461074b5761072b3660046103b9565b610747610736611833565b61073e610302565b91829182610706565b0390f35b610308565b34610780576107603660046103b9565b61077c61076b611849565b610773610302565b91829182610643565b0390f35b610308565b346107b45761079e61079836600461068d565b9061185d565b6107a6610302565b806107b0816106ba565b0390f35b610308565b346107ea576107e66107d56107cf3660046104c8565b9061190e565b6107dd610302565b91829182610538565b0390f35b610308565b3461081e576108086108023660046104c8565b90611a95565b610810610302565b8061081a816106ba565b0390f35b610308565b34610853576108333660046103b9565b61084f61083e611ac6565b610846610302565b91829182610538565b0390f35b610308565b34610888576108683660046103b9565b610884610873611b85565b61087b610302565b9182918261041b565b0390f35b610308565b90565b90565b6108a76108a26108ac9261088d565b610890565b6104a2565b90565b6108bc6301e13380610893565b90565b6108c76108af565b90565b346108fa576108da3660046103b9565b6108f66108e56108bf565b6108ed610302565b91829182610538565b0390f35b610308565b9060208282031261091857610915915f01610493565b90565b61030c565b61092690610473565b9052565b919061093d905f6020850194019061091d565b565b3461096f5761096b61095a6109553660046108ff565b611c21565b610962610302565b9182918261092a565b0390f35b610308565b346109a25761098c6109873660046108ff565b611c40565b610994610302565b8061099e816106ba565b0390f35b610308565b63ffffffff1690565b6109b9906109a7565b9052565b91906109d0905f602085019401906109b0565b565b34610a02576109fe6109ed6109e83660046108ff565b611c57565b6109f5610302565b918291826109bd565b0390f35b610308565b34610a3757610a33610a22610a1d3660046108ff565b611c82565b610a2a610302565b91829182610538565b0390f35b610308565b34610a6b57610a55610a4f3660046104c8565b90611db7565b610a5d610302565b80610a67816106ba565b0390f35b610308565b34610aa057610a803660046103b9565b610a9c610a8b611dc3565b610a93610302565b91829182610538565b0390f35b610308565b34610ad557610ad1610ac0610abb3660046108ff565b611e3b565b610ac8610302565b91829182610538565b0390f35b610308565b34610b0a57610aea3660046103b9565b610b06610af5611e50565b610afd610302565b9182918261036f565b0390f35b610308565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610b6357610b433660046103b9565b610b5f610b4e610b0f565b610b56610302565b91829182610538565b0390f35b610308565b90602082820312610b8157610b7e915f016104b9565b90565b61030c565b34610bb457610b9e610b99366004610b68565b612058565b610ba6610302565b80610bb0816106ba565b0390f35b610308565b60ff60f81b1690565b610bcb90610bb9565b9052565b5190565b60209181520190565b60200190565b610beb906104a2565b9052565b90610bfc81602093610be2565b0190565b60200190565b90610c23610c1d610c1684610bcf565b8093610bd3565b92610bdc565b905f5b818110610c335750505090565b909192610c4c610c466001928651610bef565b94610c00565b9101919091610c26565b93959194610ca7610c9c610cbb95610c8e610cb195610cc89c9a610c8160e08c01925f8d0190610bc2565b8a820360208c01526103ea565b9088820360408a01526103ea565b97606087019061052b565b608085019061091d565b60a0830190610636565b60c0818403910152610c06565b90565b34610d0257610cdb3660046103b9565b610cfe610ce66120e0565b93610cf5979597939193610302565b97889788610c56565b0390f35b610308565b7f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a67490565b610d33610d07565b90565b34610d6657610d463660046103b9565b610d62610d51610d2b565b610d59610302565b91829182610643565b0390f35b610308565b7f9b12e0c5707e494915e58b0564f18aaad9b74ac69bfc815a1edadc8e4bd032eb90565b610d97610d6b565b90565b34610dca57610daa3660046103b9565b610dc6610db5610d8f565b610dbd610302565b91829182610643565b0390f35b610308565b34610dff57610dfb610dea610de5366004610b68565b61216a565b610df2610302565b91829182610538565b0390f35b610308565b90565b610e1b610e16610e2092610e04565b610890565b6104a2565b90565b610e386b033b2e3c9fd0803ce8000000610e07565b90565b610e43610e23565b90565b34610e7657610e563660046103b9565b610e72610e61610e3b565b610e69610302565b91829182610538565b0390f35b610308565b34610eac57610ea8610e97610e9136600461068d565b906121d8565b610e9f610302565b9182918261036f565b0390f35b610308565b65ffffffffffff1690565b610ec590610eb1565b9052565b9190610edc905f60208501940190610ebc565b565b34610f0e57610eee3660046103b9565b610f0a610ef9612206565b610f01610302565b91829182610ec9565b0390f35b610308565b34610f4357610f233660046103b9565b610f3f610f2e61221a565b610f36610302565b9182918261041b565b0390f35b610308565b34610f7857610f74610f63610f5e3660046108ff565b612230565b610f6b610302565b91829182610538565b0390f35b610308565b90565b610f94610f8f610f9992610f7d565b610890565b6104a2565b90565b610fb16b02e87669c308736a04000000610f80565b90565b610fbc610f9c565b90565b34610fef57610fcf3660046103b9565b610feb610fda610fb4565b610fe2610302565b91829182610538565b0390f35b610308565b90565b5f1b90565b61101061100b61101592610ff4565b610ff7565b6105f2565b90565b6110215f610ffc565b90565b61102c611018565b90565b3461105f5761103f3660046103b9565b61105b61104a611024565b611052610302565b91829182610643565b0390f35b610308565b346110955761109161108061107a3660046104c8565b9061225f565b611088610302565b9182918261036f565b0390f35b610308565b1c90565b90565b6110b19060086110b6930261109a565b61109e565b90565b906110c491546110a1565b90565b6110d3600c5f906110b9565b90565b34611106576110e63660046103b9565b6111026110f16110c7565b6110f9610302565b91829182610538565b0390f35b610308565b3461113c576111386111276111213660046104c8565b90612281565b61112f610302565b91829182610538565b0390f35b610308565b346111715761116d61115c6111573660046108ff565b612297565b611164610302565b91829182610538565b0390f35b610308565b346111a6576111863660046103b9565b6111a26111916122ac565b611199610302565b91829182610538565b0390f35b610308565b6111b4816106f3565b036111bb57565b5f80fd5b905035906111cc826111ab565b565b909160c08284031261122d576111e6835f8401610493565b926111f481602085016104b9565b9261120282604083016104b9565b9261122a61121384606085016111bf565b936112218160808601610609565b9360a001610609565b90565b61030c565b34611267576112516112453660046111ce565b9493909392919261232c565b611259610302565b80611263816106ba565b0390f35b610308565b60e0818303126112d757611282825f8301610493565b926112908360208401610493565b9261129e81604085016104b9565b926112ac82606083016104b9565b926112d46112bd84608085016111bf565b936112cb8160a08601610609565b9360c001610609565b90565b61030c565b34611311576112fb6112ef36600461126c565b95949094939193612480565b611303610302565b8061130d816106ba565b0390f35b610308565b346113455761132f61132936600461068d565b9061259e565b611337610302565b80611341816106ba565b0390f35b610308565b9190604083820312611372578061136661136f925f8601610493565b93602001610493565b90565b61030c565b346113a8576113a461139361138d36600461134a565b906125c0565b61139b610302565b91829182610538565b0390f35b610308565b6113b6816109a7565b036113bd57565b5f80fd5b905035906113ce826113ad565b565b91906040838203126113f857806113ec6113f5925f8601610493565b936020016113c1565b90565b61030c565b61140690610eb1565b9052565b60018060d01b031690565b61141e9061140a565b9052565b906020806114449361143a5f8201515f8601906113fd565b0151910190611415565b565b9190611459905f60408501940190611422565b565b3461148c576114886114776114713660046113d0565b9061262e565b61147f610302565b91829182611446565b0390f35b610308565b5f80fd5b5f90565b634e487b7160e01b5f52601160045260245ffd5b6114b96114bf9161140a565b9161140a565b019060018060d01b0382116114d057565b611499565b906114e8916114e2611495565b506114ad565b90565b6114f76114fd9161140a565b9161140a565b90039060018060d01b03821161150f57565b611499565b9061152791611521611495565b506114eb565b90565b5f90565b61153661152a565b508061155161154b637965db0b60e01b610310565b91610310565b1490811561155e575b5090565b6115689150612644565b5f61155a565b606090565b634e487b7160e01b5f52602260045260245ffd5b90600160028304921680156115a7575b60208310146115a257565b611573565b91607f1691611597565b60209181520190565b5f5260205f2090565b905f92918054906115dd6115d683611587565b80946115b1565b916001811690815f1461163457506001146115f8575b505050565b61160591929394506115ba565b915f925b81841061161c57505001905f80806115f3565b60018160209295939554848601520191019290611609565b92949550505060ff19168252151560200201905f80806115f3565b90611659916115c3565b90565b634e487b7160e01b5f52604160045260245ffd5b9061167a906103e0565b810190811067ffffffffffffffff82111761169457604052565b61165c565b906116b96116b2926116a9610302565b9384809261164f565b0383611670565b565b6116c490611699565b90565b6116cf61156e565b506116da60036116bb565b90565b6116fa916116e961152a565b506116f261266a565b919091612677565b600190565b5f90565b5f1c90565b61171461171991611703565b61109e565b90565b6117269054611708565b90565b6117316116ff565b5061173c600261171c565b90565b916117699261174c61152a565b5061176161175861266a565b829084916126c7565b919091612753565b600190565b5f90565b61177b906105f2565b90565b9061178890611772565b5f5260205260405f2090565b90565b6117a36117a891611703565b611794565b90565b6117b59054611797565b90565b60016117d16117d7926117c961176e565b50600561177e565b016117ab565b90565b906117f5916117f06117eb826117b8565b6127f0565b6117f7565b565b9061180191612849565b50565b9061180e916117da565b565b5f90565b90565b61182b61182661183092611814565b610890565b6106f3565b90565b61183b611810565b506118466012611817565b90565b61185161176e565b5061185a6128f5565b90565b908061187861187261186d61266a565b610473565b91610473565b0361188957611886916129af565b50565b5f63334bd91960e11b8152806118a1600482016106ba565b0390fd5b6118b96118b46118be92610468565b610890565b610468565b90565b6118ca906118a5565b90565b6118d6906118c1565b90565b906118e3906118cd565b5f5260205260405f2090565b90565b61190661190161190b9261140a565b610890565b6104a2565b90565b6119459161193a61193461192f611940946119276116ff565b50600a6118d9565b6118ef565b91612a90565b90612ba5565b6118f2565b90565b906119629161195d611958610d6b565b6127f0565b6119cd565b565b61197861197361197d92610ff4565b610890565b610468565b90565b61198990611964565b90565b6119a061199b6119a592610ff4565b610890565b6104a2565b90565b6119b76119bd919392936104a2565b926104a2565b82018092116119c857565b611499565b90816119e96119e36119de5f611980565b610473565b91610473565b14611a795780611a016119fb5f61198c565b916104a2565b14611a5d57611a18611a11611729565b82906119a8565b611a31611a2b611a26610e23565b6104a2565b916104a2565b11611a4157611a3f91612ccc565b565b5f63177e3fc360e01b815280611a59600482016106ba565b0390fd5b5f631f2a200560e01b815280611a75600482016106ba565b0390fd5b5f63d92e233d60e01b815280611a91600482016106ba565b0390fd5b90611a9f91611948565b565b611ab0611ab6919392936104a2565b926104a2565b8203918211611ac157565b611499565b611ace6116ff565b50611ae8611ada610e23565b611ae2611729565b90611aa1565b90565b90611afe611af7610302565b9283611670565b565b67ffffffffffffffff8111611b1e57611b1a6020916103e0565b0190565b61165c565b90611b35611b3083611b00565b611aeb565b918252565b5f7f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c74000000910152565b611b6b601d611b23565b90611b7860208301611b3a565b565b611b82611b61565b90565b611b8d61156e565b50611b96612206565b611baf611ba9611ba4612d2a565b610eb1565b91610eb1565b03611bbf57611bbc611b7a565b90565b5f6301bfc1c560e61b815280611bd7600482016106ba565b0390fd5b5f90565b90611be9906118cd565b5f5260205260405f2090565b60018060a01b031690565b611c0c611c1191611703565b611bf5565b90565b611c1e9054611c00565b90565b611c38611c3d91611c30611bdb565b506009611bdf565b611c14565b90565b611c5190611c4c61266a565b612d7d565b565b5f90565b611c6990611c63611c53565b50612e08565b90565b90611c76906118cd565b5f5260205260405f2090565b611c98611c9d91611c916116ff565b505f611c6c565b61171c565b90565b90611cba91611cb5611cb0610d07565b6127f0565b611cbc565b565b80611cd7611cd1611ccc5f611980565b610473565b91610473565b14611d9b5781611cef611ce95f61198c565b916104a2565b14611d7f57611d05611cff611e50565b1561035d565b611d6357611d14818390612e37565b3390611d5e611d4c611d467fbef4f81c1814c641ede85ebaacf19d048b2c5b55980adfa6ef0f956c651335a2936118cd565b936118cd565b93611d55610302565b91829182610538565b0390a3565b5f63b8b5ca2d60e01b815280611d7b600482016106ba565b0390fd5b5f631f2a200560e01b815280611d97600482016106ba565b0390fd5b5f63d92e233d60e01b815280611db3600482016106ba565b0390fd5b90611dc191611ca0565b565b611dcb6116ff565b50611dd6600c61171c565b611de8611de25f61198c565b916104a2565b148015611e17575b611e0b57611e08611e01600c61171c565b4290611aa1565b90565b611e145f61198c565b90565b5042611e34611e2e611e29600c61171c565b6104a2565b916104a2565b1015611df0565b611e4d90611e476116ff565b50612e96565b90565b611e5861152a565b50611e63600c61171c565b611e75611e6f5f61198c565b916104a2565b141580611e80575b90565b5042611e9d611e97611e92600c61171c565b6104a2565b916104a2565b10611e7d565b611ebc90611eb7611eb2611018565b6127f0565b611f36565b565b90611eca5f1991610ff7565b9181191691161790565b611ee8611ee3611eed926104a2565b610890565b6104a2565b90565b90565b90611f08611f03611f0f92611ed4565b611ef0565b8254611ebe565b9055565b916020611f34929493611f2d60408201965f83019061052b565b019061052b565b565b611f40600c61171c565b611f52611f4c5f61198c565b916104a2565b0361203c5780611f6a611f64426104a2565b916104a2565b11156120205780611fa3611f9d7f00000000000000000000000000000000000000000000000000000000000000006104a2565b916104a2565b1161200457611fb2600c61171c565b611fbd82600c611ef3565b903390611fea7fdd6896dcf1d4b311cca87dd19bbba2ea9ce2f867c1568878a0438a66a1afeeec926118cd565b92611fff611ff6610302565b92839283611f13565b0390a2565b5f63ef69af6560e01b81528061201c600482016106ba565b0390fd5b5f63a565835360e01b815280612038600482016106ba565b0390fd5b5f6337e978d360e11b815280612054600482016106ba565b0390fd5b61206190611ea3565b565b5f90565b606090565b612075906118c1565b90565b67ffffffffffffffff81116120905760208091020190565b61165c565b906120a76120a283612078565b611aeb565b918252565b369037565b906120d66120be83612095565b926020806120cc8693612078565b92019103906120ac565b565b600f60f81b90565b6120e8612063565b506120f161156e565b506120fa61156e565b506121036116ff565b5061210c611bdb565b5061211561176e565b5061211e612067565b50612127612eae565b90612130612eee565b90469061213c3061206c565b906121465f610ffc565b906121586121535f61198c565b6120b1565b906121616120d8565b96959493929190565b612193612198916121796116ff565b5061218d612187600b6118ef565b91612a90565b90612ba5565b6118f2565b90565b906121a5906118cd565b5f5260205260405f2090565b60ff1690565b6121c36121c891611703565b6121b1565b90565b6121d590546121b7565b90565b6121ff915f6121f46121fa936121ec61152a565b50600561177e565b0161219b565b6121cb565b90565b5f90565b61220e612202565b50612217612d2a565b90565b61222261156e565b5061222d60046116bb565b90565b61225761225261224d61225c936122456116ff565b50600a6118d9565b6118ef565b612f2e565b6118f2565b90565b61227c9161226b61152a565b5061227461266a565b919091612753565b600190565b906122949161228e6116ff565b5061190e565b90565b6122a9906122a36116ff565b50612230565b90565b6122b46116ff565b506122bd611729565b90565b7fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf90565b6123196123209461230f606094989795612305608086019a5f870190610636565b602085019061091d565b604083019061052b565b019061052b565b565b60200190565b5190565b9395949092919542612346612340896104a2565b916104a2565b116123bf57916123b1916123b8936123a86123bd98996123906123676122c0565b6123818b938b612375610302565b958694602086016122e4565b60208201810382520382611670565b6123a261239c82612328565b91612322565b20612fa3565b92909192612fc0565b918261300a565b612d7d565b565b6123da875f918291632341d78760e11b835260048301610538565b0390fd5b7f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c990565b919461244a6124549298979561244060a09661243661245b9a61242c60c08a019e5f8b0190610636565b602089019061091d565b604087019061091d565b606085019061052b565b608083019061052b565b019061052b565b565b91602061247e92949361247760408201965f83019061091d565b019061091d565b565b96959193929490944261249b612495836104a2565b916104a2565b11612555579061250461250d9493926124ec6124b56123de565b6124dd8c80948c916124c78d9161305c565b91926124d1610302565b97889660208801612402565b60208201810382520382611670565b6124fe6124f882612328565b91612322565b20612fa3565b92909192612fc0565b8061252061251a87610473565b91610473565b0361253557506125339293919091612677565b565b84906125515f9283926325c0072360e11b84526004840161245d565b0390fd5b612570905f91829163313c898160e11b835260048301610538565b0390fd5b9061258f9161258a612585826117b8565b6127f0565b612591565b565b9061259b916129af565b50565b906125a891612574565b565b906125b4906118cd565b5f5260205260405f2090565b6125e5916125db6125e0926125d36116ff565b5060016125aa565b611c6c565b61171c565b90565b6125f26040611aeb565b90565b5f90565b5f90565b6126056125e8565b90602080836126126125f5565b81520161261d6125f9565b81525050565b61262b6125fd565b90565b906126419161263b612623565b5061308f565b90565b61264c61152a565b506126666126606301ffc9a760e01b610310565b91610310565b1490565b612672611bdb565b503390565b9161268592916001926130b7565b565b6040906126b06126b794969593966126a660608401985f85019061091d565b602083019061052b565b019061052b565b565b906126c491036104a2565b90565b9291926126d58183906125c0565b90816126ea6126e45f196104a2565b916104a2565b106126f7575b5050509050565b8161270a612704876104a2565b916104a2565b1061273057612727939461271f9193926126b9565b905f926130b7565b805f80806126f0565b5061274f849291925f938493637dc7a0d960e11b855260048501612687565b0390fd5b918261276f6127696127645f611980565b610473565b91610473565b146127c9578161278f6127896127845f611980565b610473565b91610473565b146127a2576127a0929190916131c6565b565b6127c56127ae5f611980565b5f91829163ec442f0560e01b83526004830161092a565b0390fd5b6127ec6127d55f611980565b5f918291634b637e8f60e11b83526004830161092a565b0390fd5b612802906127fc61266a565b90613293565b565b9061281060ff91610ff7565b9181191691161790565b6128239061035d565b90565b90565b9061283e6128396128459261281a565b612826565b8254612804565b9055565b61285161152a565b506128666128608284906121d8565b1561035d565b5f146128ef5761288e60016128895f6128816005869061177e565b01859061219b565b612829565b9061289761266a565b906128d46128ce6128c87f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d95611772565b926118cd565b926118cd565b926128dd610302565b806128e7816106ba565b0390a4600190565b50505f90565b6128fd61176e565b506129073061206c565b6129396129337f0000000000000000000000000000000000000000000000000000000000000000610473565b91610473565b1480612975575b5f1461296a577f000000000000000000000000000000000000000000000000000000000000000090565b61297261333f565b90565b50466129a96129a37f00000000000000000000000000000000000000000000000000000000000000006104a2565b916104a2565b14612940565b6129b761152a565b506129c38183906121d8565b5f14612a4b576129ea5f6129e55f6129dd6005869061177e565b01859061219b565b612829565b906129f361266a565b90612a30612a2a612a247ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b95611772565b926118cd565b926118cd565b92612a39610302565b80612a43816106ba565b0390a4600190565b50505f90565b612a65612a60612a6a92610eb1565b610890565b6104a2565b90565b916020612a8e929493612a8760408201965f83019061052b565b0190610ebc565b565b612a98612202565b50612aa1612206565b81612ab4612aae83612a51565b916104a2565b1015612ac75750612ac490613448565b90565b90612ae25f928392637669fc0f60e11b845260048401612a6d565b0390fd5b5490565b90565b612b01612afc612b0692612aea565b610890565b6104a2565b90565b90565b65ffffffffffff1690565b612b23612b2891611703565b612b0c565b90565b612b359054612b17565b90565b90565b612b4f612b4a612b5492612b38565b610890565b6104a2565b90565b60301c90565b60018060d01b031690565b612b74612b7991612b57565b612b5d565b90565b612b869054612b68565b90565b612b9d612b98612ba292610ff4565b610890565b61140a565b90565b90612bf990612bb2611495565b50612bbe5f8401612ae6565b612bc75f61198c565b908080612bdd612bd76005612aed565b916104a2565b11612c5a575b5090612bf45f860193919293612b09565b613a9b565b80612c0c612c065f61198c565b916104a2565b145f14612c22575050612c1e5f612b89565b5b90565b612c4f5f91612c4a612c4484612c55960192612c3e6001612b3b565b90611aa1565b91612b09565b613a91565b01612b7c565b612c1f565b80612c68612c6e9291613726565b90611aa1565b9083612ca0612c9a612c955f612c8f818c01612c8a8991612b09565b613a91565b01612b2b565b610eb1565b91610eb1565b105f14612cb15750905b905f612be3565b9150612cc790612cc16001612b3b565b906119a8565b612caa565b80612ce7612ce1612cdc5f611980565b610473565b91610473565b14612d0357612d0191612cf95f611980565b9190916131c6565b565b612d26612d0f5f611980565b5f91829163ec442f0560e01b83526004830161092a565b0390fd5b612d32612202565b50612d3c43613448565b90565b90612d5060018060a01b0391610ff7565b9181191691161790565b90565b90612d72612d6d612d79926118cd565b612d5a565b8254612d3f565b9055565b90612e0691612e00612d8e82611c21565b612da384612d9e60098690611bdf565b612d5d565b82818590612de3612ddd612dd77f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f956118cd565b926118cd565b926118cd565b92612dec610302565b80612df6816106ba565b0390a49291613b2a565b91613b42565b565b612e2f612e2a612e25612e3493612e1d611c53565b50600a6118d9565b6118ef565b613cf0565b613d6f565b90565b9081612e53612e4d612e485f611980565b610473565b91610473565b14612e6f57612e6d9190612e665f611980565b90916131c6565b565b612e92612e7b5f611980565b5f918291634b637e8f60e11b83526004830161092a565b0390fd5b612ea890612ea26116ff565b50613dc0565b90565b90565b612eb661156e565b50612eeb7f0000000000000000000000000000000000000000000000000000000000000000612ee56006612eab565b90613edb565b90565b612ef661156e565b50612f2b7f0000000000000000000000000000000000000000000000000000000000000000612f256007612eab565b90613edb565b90565b612f36611495565b50612f425f8201612ae6565b80612f55612f4f5f61198c565b916104a2565b145f14612f6b575050612f675f612b89565b5b90565b612f985f91612f93612f8d84612f9e960192612f876001612b3b565b90611aa1565b91612b09565b613a91565b01612b7c565b612f68565b612fbd90612faf61176e565b50612fb86128f5565b613f29565b90565b92612fdb92612fe494612fd1611bdb565b5092909192613fef565b9092919261411a565b90565b91602061300892949361300160408201965f83019061091d565b019061052b565b565b6130138161305c565b91613026613020846104a2565b916104a2565b0361302f575050565b6130495f9283926301d4b62360e61b845260048401612fe7565b0390fd5b600161305991016104a2565b90565b613070906130686116ff565b506008611c6c565b61308c61307c8261171c565b916130868361304d565b90611ef3565b90565b906130af6130aa6130b4936130a2612623565b50600a6118d9565b6118ef565b614290565b90565b9092816130d46130ce6130c95f611980565b610473565b91610473565b1461319f57836130f46130ee6130e95f611980565b610473565b91610473565b14613178576131188361311361310c600186906125aa565b8790611c6c565b611ef3565b613122575b505050565b91909161316d61315b6131557f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925936118cd565b936118cd565b93613164610302565b91829182610538565b0390a35f808061311d565b61319b6131845f611980565b5f918291634a1406b160e11b83526004830161092a565b0390fd5b6131c26131ab5f611980565b5f91829163e602df0560e01b83526004830161092a565b0390fd5b91826131e26131dc6131d75f611980565b610473565b91610473565b14158061324d575b6131fd575b6131fb929190916142b1565b565b613205611e50565b8061322c575b156131ef575f6336e278fd60e21b815280613228600482016106ba565b0390fd5b5061324861324261323b610d07565b33906121d8565b1561035d565b61320b565b508161326961326361325e5f611980565b610473565b91610473565b14156131ea565b91602061329192949361328a60408201965f83019061091d565b0190610636565b565b906132a86132a28383906121d8565b1561035d565b6132b0575050565b6132ca5f92839263e2517d3f60e01b845260048401613270565b0390fd5b7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f90565b9095949261333d9461332c6133369261332260809661331860a088019c5f890190610636565b6020870190610636565b6040850190610636565b606083019061052b565b019061091d565b565b61334761176e565b506133506132ce565b6133c77f0000000000000000000000000000000000000000000000000000000000000000916133b87f0000000000000000000000000000000000000000000000000000000000000000466133a33061206c565b916133ac610302565b968795602087016132f2565b60208201810382520382611670565b6133d96133d382612328565b91612322565b2090565b90565b6133f46133ef6133f9926133dd565b610890565b6106f3565b90565b613405906133e0565b9052565b91602061342a92949361342360408201965f8301906133fc565b019061052b565b565b61344061343b613445926104a2565b610890565b610eb1565b90565b613450612202565b508061346a61346465ffffffffffff612a51565b916104a2565b1161347b576134789061342c565b90565b60306134975f9283926306dfcc6560e41b845260048401613409565b0390fd5b90565b6134b26134ad6134b79261349b565b610890565b6104a2565b90565b90565b6134d16134cc6134d6926134ba565b610890565b6106f3565b90565b6134f8906134f26134ec6134fd946106f3565b916104a2565b9061109a565b6104a2565b90565b90565b61351761351261351c92613500565b610890565b6106f3565b90565b1b90565b6135429061353c613536613547946106f3565b916104a2565b9061351f565b6104a2565b90565b90565b61356161355c6135669261354a565b610890565b6104a2565b90565b90565b61358061357b61358592613569565b610890565b6106f3565b90565b90565b61359f61359a6135a492613588565b610890565b6104a2565b90565b90565b6135be6135b96135c3926135a7565b610890565b6106f3565b90565b90565b6135dd6135d86135e2926135c6565b610890565b6104a2565b90565b90565b6135fc6135f7613601926135e5565b610890565b6106f3565b90565b90565b61361b61361661362092613604565b610890565b6104a2565b90565b90565b61363a61363561363f92613623565b610890565b6106f3565b90565b61365661365161365b926135a7565b610890565b6104a2565b90565b90565b61367561367061367a9261365e565b610890565b6106f3565b90565b61369161368c61369692613623565b610890565b6104a2565b90565b6136ad6136a86136b292612b38565b610890565b6106f3565b90565b90565b6136cc6136c76136d1926136b5565b610890565b6104a2565b90565b906136df91026104a2565b90565b634e487b7160e01b5f52601260045260245ffd5b613702613708916104a2565b916104a2565b908115613713570490565b6136e2565b9061372391016104a2565b90565b61372e6116ff565b508061374361373d6001612b3b565b916104a2565b1115613a8e57806139586139356139256139156139056138f56138e56138d56138c56138b56138a58b61389f61389861395e9f6138786138686138889261378a6001612b3b565b90806137a261379c600160801b61349e565b916104a2565b1015613a60575b806137c56137bf6801000000000000000061354d565b916104a2565b1015613a32575b806137e46137de64010000000061358b565b916104a2565b1015613a04575b806138016137fb620100006135c9565b916104a2565b10156139d6575b8061381d613817610100613607565b916104a2565b10156139a8575b806138386138326010613642565b916104a2565b101561397a575b61385261384c600461367d565b916104a2565b1015613961575b61386360036136b8565b6136d4565b6138726001613699565b906134d9565b61388281866136f6565b90613718565b6138926001613699565b906134d9565b80926136f6565b90613718565b6138af6001613699565b906134d9565b6138bf818c6136f6565b90613718565b6138cf6001613699565b906134d9565b6138df818a6136f6565b90613718565b6138ef6001613699565b906134d9565b6138ff81886136f6565b90613718565b61390f6001613699565b906134d9565b61391f81866136f6565b90613718565b61392f6001613699565b906134d9565b9161395261394c6139478580946136f6565b6104a2565b916104a2565b11614341565b906126b9565b90565b6139759061396f6001613699565b90613523565b613859565b6139916139a29161398b6004613626565b906134d9565b9161399c6002613661565b90613523565b9061383f565b6139bf6139d0916139b960086135e8565b906134d9565b916139ca6004613626565b90613523565b90613824565b6139ed6139fe916139e760106135aa565b906134d9565b916139f860086135e8565b90613523565b90613808565b613a1b613a2c91613a15602061356c565b906134d9565b91613a2660106135aa565b90613523565b906137eb565b613a49613a5a91613a436040613503565b906134d9565b91613a54602061356c565b90613523565b906137cc565b613a77613a8891613a7160806134bd565b906134d9565b91613a826040613503565b90613523565b906137a9565b90565b5f5260205f200190565b93919092613aa76116ff565b505b81613abc613ab6836104a2565b916104a2565b1015613b2257613acd82829061438d565b90613ae35f613add888590613a91565b01612b2b565b613af5613aef87610eb1565b91610eb1565b115f14613b055750915b91613aa9565b929150613b1c90613b166001612b3b565b906119a8565b90613aff565b925050915090565b613b3c90613b366116ff565b50611c82565b90565b90565b91909180613b58613b5285610473565b91610473565b141580613cd6575b613b6a575b505050565b80613b85613b7f613b7a5f611980565b610473565b91610473565b03613c46575b5081613ba7613ba1613b9c5f611980565b610473565b91610473565b03613bb3575b80613b65565b613bfa613bed613bf492613bc9600a86906118d9565b90613be7613be1613bdb600193614426565b936118ef565b91613b3f565b90614479565b92906118f2565b916118f2565b919091613c277fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926118cd565b92613c3c613c33610302565b92839283611f13565b0390a25f80613bad565b613c85613c8b613c7e613c5b600a85906118d9565b6002613c78613c72613c6c89614426565b936118ef565b91613b3f565b90614479565b92906118f2565b916118f2565b919091613cb87fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926118cd565b92613ccd613cc4610302565b92839283611f13565b0390a25f613b8b565b5081613cea613ce45f61198c565b916104a2565b11613b60565b5f613d0491613cfd6116ff565b5001612ae6565b90565b613d1b613d16613d20926109a7565b610890565b6104a2565b90565b613d2c9061356c565b9052565b916020613d51929493613d4a60408201965f830190613d23565b019061052b565b565b613d67613d62613d6c926104a2565b610890565b6109a7565b90565b613d77611c53565b5080613d8f613d8963ffffffff613d07565b916104a2565b11613da057613d9d90613d53565b90565b6020613dbc5f9283926306dfcc6560e41b845260048401613d30565b0390fd5b613dd7613ddc91613dcf6116ff565b506008611c6c565b61171c565b90565b90565b613df6613df1613dfb92613ddf565b610ff7565b6105f2565b90565b613e0860ff613de2565b90565b5f5260205f2090565b905f9291805490613e2e613e2783611587565b80946115b1565b916001811690815f14613e855750600114613e49575b505050565b613e569192939450613e0b565b915f925b818410613e6d57505001905f8080613e44565b60018160209295939554848601520191019290613e5a565b92949550505060ff19168252151560200201905f8080613e44565b90613eaa91613e14565b90565b90613ecd613ec692613ebd610302565b93848092613ea0565b0383611670565b565b613ed890613ead565b90565b90613ee461156e565b50613eee82611772565b613f07613f01613efc613dfe565b6105f2565b916105f2565b14155f14613f1c5750613f1990614503565b90565b613f269150613ecf565b90565b604291613f3461176e565b50604051917f19010000000000000000000000000000000000000000000000000000000000008352600283015260228201522090565b5f90565b613f7a613f7f91611703565b611ed4565b90565b90565b613f99613f94613f9e92613f82565b610890565b6104a2565b90565b613fd6613fdd94613fcc606094989795613fc2608086019a5f870190610636565b60208501906106f9565b6040830190610636565b0190610636565b565b613fe7610302565b3d5f823e3d90fd5b939293613ffa611bdb565b50614003613f6a565b5061400c61176e565b5061401685613f6e565b6140486140427f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0613f85565b916104a2565b116140d5579061406b602094955f94939293614062610302565b94859485613fa1565b838052039060015afa156140d0576140835f51610ff7565b8061409e6140986140935f611980565b610473565b91610473565b146140b4575f916140ae5f610ffc565b91929190565b506140be5f611980565b6001916140ca5f610ffc565b91929190565b613fdf565b5050506140e15f611980565b9060039291929190565b634e487b7160e01b5f52602160045260245ffd5b6004111561410957565b6140eb565b90614118826140ff565b565b8061412d6141275f61410e565b9161410e565b145f14614138575050565b8061414c614146600161410e565b9161410e565b145f1461416f575f63f645eedf60e01b81528061416b600482016106ba565b0390fd5b8061418361417d600261410e565b9161410e565b145f146141b1576141ad61419683613f6e565b5f91829163fce698f760e01b835260048301610538565b0390fd5b6141c46141be600361410e565b9161410e565b146141cc5750565b6141e7905f9182916335e2f38360e21b835260048301610643565b0390fd5b634e487b7160e01b5f52603260045260245ffd5b5f5260205f2090565b61421181612ae6565b82101561422b576142236001916141ff565b910201905f90565b6141eb565b9061423a90610eb1565b9052565b906142489061140a565b9052565b906142826142795f61425c6125e8565b9461427361426b838301612b2b565b838801614230565b01612b7c565b6020840161423e565b565b61428d9061424c565b90565b6142ae915f6142a8926142a1612623565b5001614208565b50614284565b90565b92916142bf84838391614533565b836142da6142d46142cf5f611980565b610473565b91610473565b146142ef575b6142ed92939190916146bd565b565b6142f7611729565b936143006146a2565b948061431461430e886104a2565b916104a2565b11614321575093506142e0565b859061433d5f928392630e58ae9360e11b845260048401611f13565b0390fd5b6143496116ff565b50151590565b61436361435e6143689261365e565b610890565b6104a2565b90565b61437761437d916104a2565b916104a2565b908115614388570490565b6136e2565b6143b26143b89261439c6116ff565b5082811692186143ac600261434f565b9061436b565b906119a8565b90565b90565b6143d26143cd6143d7926143bb565b610890565b6106f3565b90565b6143e3906143be565b9052565b91602061440892949361440160408201965f8301906143da565b019061052b565b565b61441e614419614423926104a2565b610890565b61140a565b90565b61442e611495565b508061444861444260018060d01b036118f2565b916104a2565b11614459576144569061440a565b90565b60d06144755f9283926306dfcc6560e41b8452600484016143e7565b0390fd5b906144af6144b5939261448a611495565b50614493611495565b5080936144a86144a1612206565b9492612f2e565b9091614b38565b9161477c565b91909190565b6144cf6144ca6144d492613569565b610890565b6104a2565b90565b369037565b906145016144e983611b23565b926020806144f78693611b00565b92019103906144d7565b565b61450b61156e565b50614515816147e6565b9061452861452360206144bb565b6144dc565b918252602082015290565b9190918061455161454b6145465f611980565b610473565b91610473565b145f146146325761457561456e83614569600261171c565b6119a8565b6002611ef3565b5b8261459161458b6145865f611980565b610473565b91610473565b145f14614606576145b56145ae836145a9600261171c565b6126b9565b6002611ef3565b5b9190916146016145ef6145e97fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef936118cd565b936118cd565b936145f8610302565b91829182610538565b0390a3565b61462d826146276146185f8790611c6c565b916146228361171c565b613718565b90611ef3565b6145b6565b6146456146405f8390611c6c565b61171c565b80614658614652856104a2565b916104a2565b106146805761466b61467b9184906126b9565b6146765f8490611c6c565b611ef3565b614576565b9061469e9091925f93849363391434e360e21b855260048501612687565b0390fd5b6146aa6116ff565b506146ba60018060d01b036118f2565b90565b9161471561470f61471c94806146e36146dd6146d85f611980565b610473565b91610473565b1461474d575b846147046146fe6146f95f611980565b610473565b91610473565b1461471e575b611c21565b92611c21565b9091613b42565b565b614746600b600261474061473a61473489614426565b936118ef565b91613b3f565b90614479565b505061470a565b614775600b600161476f61476961476389614426565b936118ef565b91613b3f565b90614479565b50506146e9565b916147a05f6147a59461478d611495565b50614796611495565b5001929192612b09565b6149ea565b91909190565b6147bf6147ba6147c492613ddf565b610890565b6104a2565b90565b90565b6147de6147d96147e3926147c7565b610890565b6104a2565b90565b6147fb614800916147f56116ff565b50611772565b613f6e565b61480a60ff6147ab565b168061481f614819601f6147ca565b916104a2565b116148275790565b5f632cd44ac360e21b81528061483f600482016106ba565b0390fd5b5490565b6148516040611aeb565b90565b5f5260205f2090565b61486681614843565b82101561488057614878600191614854565b910201905f90565b6141eb565b634e487b7160e01b5f525f60045260245ffd5b6148a29051610eb1565b90565b906148b665ffffffffffff91610ff7565b9181191691161790565b6148d46148cf6148d992610eb1565b610890565b610eb1565b90565b90565b906148f46148ef6148fb926148c0565b6148dc565b82546148a5565b9055565b614909905161140a565b90565b60301b90565b9061492465ffffffffffff199161490c565b9181191691161790565b61494261493d6149479261140a565b610890565b61140a565b90565b90565b9061496261495d6149699261492e565b61494a565b8254614912565b9055565b9061499760205f61499d9461498f828201614989848801614898565b906148df565b0192016148ff565b9061494d565b565b91906149b0576149ae9161496d565b565b614885565b90815491680100000000000000008310156149e557826149dd9160016149e39501815561485d565b9061499f565b565b61165c565b909291926149f6611495565b506149ff611495565b50614a0982614843565b80614a1c614a165f61198c565b916104a2565b115f14614aec57614a4290614a3c8491614a366001612b3b565b90611aa1565b90613a91565b90614a4e5f8301612b2b565b92614a5a5f8401612b7c565b9380614a6e614a6885610eb1565b91610eb1565b11614ad057614a85614a7f84610eb1565b91610eb1565b145f14614aa0575050614a9b905f85910161494d565b5b9190565b614acb9250614ac686614abd614ab4614847565b945f8601614230565b6020840161423e565b6149b5565b614a9c565b5f632520601d60e01b815280614ae8600482016106ba565b0390fd5b50614b1791614b1285614b09614b00614847565b945f8601614230565b6020840161423e565b6149b5565b614b205f612b89565b9190565b634e487b7160e01b5f52605160045260245ffd5b91909180600114614b5757600203614b2457614b5391611514565b905b565b50614b61916114d5565b90614b5556
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\x14\x91V[a\0\x1D_5a\x02\xFCV[\x80c\x01\xFF\xC9\xA7\x14a\x02\xF7W\x80c\x06\xFD\xDE\x03\x14a\x02\xF2W\x80c\t^\xA7\xB3\x14a\x02\xEDW\x80c\x18\x16\r\xDD\x14a\x02\xE8W\x80c#\xB8r\xDD\x14a\x02\xE3W\x80c$\x8A\x9C\xA3\x14a\x02\xDEW\x80c//\xF1]\x14a\x02\xD9W\x80c1<\xE5g\x14a\x02\xD4W\x80c6D\xE5\x15\x14a\x02\xCFW\x80c6V\x8A\xBE\x14a\x02\xCAW\x80c:F\xB1\xA8\x14a\x02\xC5W\x80c@\xC1\x0F\x19\x14a\x02\xC0W\x80cK\xDD6\xCE\x14a\x02\xBBW\x80cK\xF5\xD7\xE9\x14a\x02\xB6W\x80cO\x1B\xFC\x9E\x14a\x02\xB1W\x80cX|\xDE\x1E\x14a\x02\xACW\x80c\\\x19\xA9\\\x14a\x02\xA7W\x80co\xCF\xFFE\x14a\x02\xA2W\x80cp\xA0\x821\x14a\x02\x9DW\x80cy\xCCg\x90\x14a\x02\x98W\x80cz\x8C\xD1V\x14a\x02\x93W\x80c~\xCE\xBE\0\x14a\x02\x8EW\x80c\x83\xF1!\x1B\x14a\x02\x89W\x80c\x84&\xAD\xF2\x14a\x02\x84W\x80c\x84L\x90&\x14a\x02\x7FW\x80c\x84\xB0\x19n\x14a\x02zW\x80c\x8AT%!\x14a\x02uW\x80c\x8D3C\xD6\x14a\x02pW\x80c\x8ES\x9E\x8C\x14a\x02kW\x80c\x90-U\xA5\x14a\x02fW\x80c\x91\xD1HT\x14a\x02aW\x80c\x91\xDD\xAD\xF4\x14a\x02\\W\x80c\x95\xD8\x9BA\x14a\x02WW\x80c\x9A\xB2N\xB0\x14a\x02RW\x80c\x9B~\xF6K\x14a\x02MW\x80c\xA2\x17\xFD\xDF\x14a\x02HW\x80c\xA9\x05\x9C\xBB\x14a\x02CW\x80c\xAA\x08*\x9D\x14a\x02>W\x80c\xB0\xCA%>\x14a\x029W\x80c\xBBMD6\x14a\x024W\x80c\xC0*\xE7T\x14a\x02/W\x80c\xC3\xCD\xA5 \x14a\x02*W\x80c\xD5\x05\xAC\xCF\x14a\x02%W\x80c\xD5Gt\x1F\x14a\x02 W\x80c\xDDb\xED>\x14a\x02\x1BWc\xF1\x12~\xD8\x03a\0\x0EWa\x14[V[a\x13wV[a\x13\x16V[a\x12\xDCV[a\x122V[a\x11vV[a\x11AV[a\x11\x0BV[a\x10\xD6V[a\x10dV[a\x10/V[a\x0F\xBFV[a\x0FHV[a\x0F\x13V[a\x0E\xDEV[a\x0E{V[a\x0EFV[a\r\xCFV[a\r\x9AV[a\r6V[a\x0C\xCBV[a\x0B\x86V[a\x0B3V[a\n\xDAV[a\n\xA5V[a\npV[a\n<V[a\n\x07V[a\t\xD2V[a\ttV[a\t?V[a\x08\xCAV[a\x08XV[a\x08#V[a\x07\xEFV[a\x07\xB9V[a\x07\x85V[a\x07PV[a\x07\x1BV[a\x06\xBFV[a\x06XV[a\x05\xBCV[a\x05MV[a\x04\xF5V[a\x043V[a\x03\x84V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[c\xFF\xFF\xFF\xFF`\xE0\x1B\x16\x90V[a\x03%\x81a\x03\x10V[\x03a\x03,WV[_\x80\xFD[\x90P5\x90a\x03=\x82a\x03\x1CV[V[\x90` \x82\x82\x03\x12a\x03XWa\x03U\x91_\x01a\x030V[\x90V[a\x03\x0CV[\x15\x15\x90V[a\x03k\x90a\x03]V[\x90RV[\x91\x90a\x03\x82\x90_` \x85\x01\x94\x01\x90a\x03bV[V[4a\x03\xB4Wa\x03\xB0a\x03\x9Fa\x03\x9A6`\x04a\x03?V[a\x15.V[a\x03\xA7a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[_\x91\x03\x12a\x03\xC3WV[a\x03\x0CV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x04\ta\x04\x12` \x93a\x04\x17\x93a\x04\0\x81a\x03\xC8V[\x93\x84\x80\x93a\x03\xCCV[\x95\x86\x91\x01a\x03\xD5V[a\x03\xE0V[\x01\x90V[a\x040\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03\xEAV[\x90V[4a\x04cWa\x04C6`\x04a\x03\xB9V[a\x04_a\x04Na\x16\xC7V[a\x04Va\x03\x02V[\x91\x82\x91\x82a\x04\x1BV[\x03\x90\xF3[a\x03\x08V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04|\x90a\x04hV[\x90V[a\x04\x88\x81a\x04sV[\x03a\x04\x8FWV[_\x80\xFD[\x90P5\x90a\x04\xA0\x82a\x04\x7FV[V[\x90V[a\x04\xAE\x81a\x04\xA2V[\x03a\x04\xB5WV[_\x80\xFD[\x90P5\x90a\x04\xC6\x82a\x04\xA5V[V[\x91\x90`@\x83\x82\x03\x12a\x04\xF0W\x80a\x04\xE4a\x04\xED\x92_\x86\x01a\x04\x93V[\x93` \x01a\x04\xB9V[\x90V[a\x03\x0CV[4a\x05&Wa\x05\"a\x05\x11a\x05\x0B6`\x04a\x04\xC8V[\x90a\x16\xDDV[a\x05\x19a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[a\x054\x90a\x04\xA2V[\x90RV[\x91\x90a\x05K\x90_` \x85\x01\x94\x01\x90a\x05+V[V[4a\x05}Wa\x05]6`\x04a\x03\xB9V[a\x05ya\x05ha\x17)V[a\x05pa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90\x91``\x82\x84\x03\x12a\x05\xB7Wa\x05\xB4a\x05\x9D\x84_\x85\x01a\x04\x93V[\x93a\x05\xAB\x81` \x86\x01a\x04\x93V[\x93`@\x01a\x04\xB9V[\x90V[a\x03\x0CV[4a\x05\xEDWa\x05\xE9a\x05\xD8a\x05\xD26`\x04a\x05\x82V[\x91a\x17?V[a\x05\xE0a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[\x90V[a\x05\xFE\x81a\x05\xF2V[\x03a\x06\x05WV[_\x80\xFD[\x90P5\x90a\x06\x16\x82a\x05\xF5V[V[\x90` \x82\x82\x03\x12a\x061Wa\x06.\x91_\x01a\x06\tV[\x90V[a\x03\x0CV[a\x06?\x90a\x05\xF2V[\x90RV[\x91\x90a\x06V\x90_` \x85\x01\x94\x01\x90a\x066V[V[4a\x06\x88Wa\x06\x84a\x06sa\x06n6`\x04a\x06\x18V[a\x17\xB8V[a\x06{a\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[\x91\x90`@\x83\x82\x03\x12a\x06\xB5W\x80a\x06\xA9a\x06\xB2\x92_\x86\x01a\x06\tV[\x93` \x01a\x04\x93V[\x90V[a\x03\x0CV[_\x01\x90V[4a\x06\xEEWa\x06\xD8a\x06\xD26`\x04a\x06\x8DV[\x90a\x18\x04V[a\x06\xE0a\x03\x02V[\x80a\x06\xEA\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[`\xFF\x16\x90V[a\x07\x02\x90a\x06\xF3V[\x90RV[\x91\x90a\x07\x19\x90_` \x85\x01\x94\x01\x90a\x06\xF9V[V[4a\x07KWa\x07+6`\x04a\x03\xB9V[a\x07Ga\x076a\x183V[a\x07>a\x03\x02V[\x91\x82\x91\x82a\x07\x06V[\x03\x90\xF3[a\x03\x08V[4a\x07\x80Wa\x07`6`\x04a\x03\xB9V[a\x07|a\x07ka\x18IV[a\x07sa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[4a\x07\xB4Wa\x07\x9Ea\x07\x986`\x04a\x06\x8DV[\x90a\x18]V[a\x07\xA6a\x03\x02V[\x80a\x07\xB0\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\x07\xEAWa\x07\xE6a\x07\xD5a\x07\xCF6`\x04a\x04\xC8V[\x90a\x19\x0EV[a\x07\xDDa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x08\x1EWa\x08\x08a\x08\x026`\x04a\x04\xC8V[\x90a\x1A\x95V[a\x08\x10a\x03\x02V[\x80a\x08\x1A\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\x08SWa\x0836`\x04a\x03\xB9V[a\x08Oa\x08>a\x1A\xC6V[a\x08Fa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x08\x88Wa\x08h6`\x04a\x03\xB9V[a\x08\x84a\x08sa\x1B\x85V[a\x08{a\x03\x02V[\x91\x82\x91\x82a\x04\x1BV[\x03\x90\xF3[a\x03\x08V[\x90V[\x90V[a\x08\xA7a\x08\xA2a\x08\xAC\x92a\x08\x8DV[a\x08\x90V[a\x04\xA2V[\x90V[a\x08\xBCc\x01\xE13\x80a\x08\x93V[\x90V[a\x08\xC7a\x08\xAFV[\x90V[4a\x08\xFAWa\x08\xDA6`\x04a\x03\xB9V[a\x08\xF6a\x08\xE5a\x08\xBFV[a\x08\xEDa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90` \x82\x82\x03\x12a\t\x18Wa\t\x15\x91_\x01a\x04\x93V[\x90V[a\x03\x0CV[a\t&\x90a\x04sV[\x90RV[\x91\x90a\t=\x90_` \x85\x01\x94\x01\x90a\t\x1DV[V[4a\toWa\tka\tZa\tU6`\x04a\x08\xFFV[a\x1C!V[a\tba\x03\x02V[\x91\x82\x91\x82a\t*V[\x03\x90\xF3[a\x03\x08V[4a\t\xA2Wa\t\x8Ca\t\x876`\x04a\x08\xFFV[a\x1C@V[a\t\x94a\x03\x02V[\x80a\t\x9E\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[c\xFF\xFF\xFF\xFF\x16\x90V[a\t\xB9\x90a\t\xA7V[\x90RV[\x91\x90a\t\xD0\x90_` \x85\x01\x94\x01\x90a\t\xB0V[V[4a\n\x02Wa\t\xFEa\t\xEDa\t\xE86`\x04a\x08\xFFV[a\x1CWV[a\t\xF5a\x03\x02V[\x91\x82\x91\x82a\t\xBDV[\x03\x90\xF3[a\x03\x08V[4a\n7Wa\n3a\n\"a\n\x1D6`\x04a\x08\xFFV[a\x1C\x82V[a\n*a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\nkWa\nUa\nO6`\x04a\x04\xC8V[\x90a\x1D\xB7V[a\n]a\x03\x02V[\x80a\ng\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\n\xA0Wa\n\x806`\x04a\x03\xB9V[a\n\x9Ca\n\x8Ba\x1D\xC3V[a\n\x93a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\n\xD5Wa\n\xD1a\n\xC0a\n\xBB6`\x04a\x08\xFFV[a\x1E;V[a\n\xC8a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x0B\nWa\n\xEA6`\x04a\x03\xB9V[a\x0B\x06a\n\xF5a\x1EPV[a\n\xFDa\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0BcWa\x0BC6`\x04a\x03\xB9V[a\x0B_a\x0BNa\x0B\x0FV[a\x0BVa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90` \x82\x82\x03\x12a\x0B\x81Wa\x0B~\x91_\x01a\x04\xB9V[\x90V[a\x03\x0CV[4a\x0B\xB4Wa\x0B\x9Ea\x0B\x996`\x04a\x0BhV[a XV[a\x0B\xA6a\x03\x02V[\x80a\x0B\xB0\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[`\xFF`\xF8\x1B\x16\x90V[a\x0B\xCB\x90a\x0B\xB9V[\x90RV[Q\x90V[` \x91\x81R\x01\x90V[` \x01\x90V[a\x0B\xEB\x90a\x04\xA2V[\x90RV[\x90a\x0B\xFC\x81` \x93a\x0B\xE2V[\x01\x90V[` \x01\x90V[\x90a\x0C#a\x0C\x1Da\x0C\x16\x84a\x0B\xCFV[\x80\x93a\x0B\xD3V[\x92a\x0B\xDCV[\x90_[\x81\x81\x10a\x0C3WPPP\x90V[\x90\x91\x92a\x0CLa\x0CF`\x01\x92\x86Qa\x0B\xEFV[\x94a\x0C\0V[\x91\x01\x91\x90\x91a\x0C&V[\x93\x95\x91\x94a\x0C\xA7a\x0C\x9Ca\x0C\xBB\x95a\x0C\x8Ea\x0C\xB1\x95a\x0C\xC8\x9C\x9Aa\x0C\x81`\xE0\x8C\x01\x92_\x8D\x01\x90a\x0B\xC2V[\x8A\x82\x03` \x8C\x01Ra\x03\xEAV[\x90\x88\x82\x03`@\x8A\x01Ra\x03\xEAV[\x97``\x87\x01\x90a\x05+V[`\x80\x85\x01\x90a\t\x1DV[`\xA0\x83\x01\x90a\x066V[`\xC0\x81\x84\x03\x91\x01Ra\x0C\x06V[\x90V[4a\r\x02Wa\x0C\xDB6`\x04a\x03\xB9V[a\x0C\xFEa\x0C\xE6a \xE0V[\x93a\x0C\xF5\x97\x95\x97\x93\x91\x93a\x03\x02V[\x97\x88\x97\x88a\x0CVV[\x03\x90\xF3[a\x03\x08V[\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t\x90V[a\r3a\r\x07V[\x90V[4a\rfWa\rF6`\x04a\x03\xB9V[a\rba\rQa\r+V[a\rYa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[\x7F\x9B\x12\xE0\xC5p~II\x15\xE5\x8B\x05d\xF1\x8A\xAA\xD9\xB7J\xC6\x9B\xFC\x81Z\x1E\xDA\xDC\x8EK\xD02\xEB\x90V[a\r\x97a\rkV[\x90V[4a\r\xCAWa\r\xAA6`\x04a\x03\xB9V[a\r\xC6a\r\xB5a\r\x8FV[a\r\xBDa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[4a\r\xFFWa\r\xFBa\r\xEAa\r\xE56`\x04a\x0BhV[a!jV[a\r\xF2a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90V[a\x0E\x1Ba\x0E\x16a\x0E \x92a\x0E\x04V[a\x08\x90V[a\x04\xA2V[\x90V[a\x0E8k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x0E\x07V[\x90V[a\x0ECa\x0E#V[\x90V[4a\x0EvWa\x0EV6`\x04a\x03\xB9V[a\x0Era\x0Eaa\x0E;V[a\x0Eia\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x0E\xACWa\x0E\xA8a\x0E\x97a\x0E\x916`\x04a\x06\x8DV[\x90a!\xD8V[a\x0E\x9Fa\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x0E\xC5\x90a\x0E\xB1V[\x90RV[\x91\x90a\x0E\xDC\x90_` \x85\x01\x94\x01\x90a\x0E\xBCV[V[4a\x0F\x0EWa\x0E\xEE6`\x04a\x03\xB9V[a\x0F\na\x0E\xF9a\"\x06V[a\x0F\x01a\x03\x02V[\x91\x82\x91\x82a\x0E\xC9V[\x03\x90\xF3[a\x03\x08V[4a\x0FCWa\x0F#6`\x04a\x03\xB9V[a\x0F?a\x0F.a\"\x1AV[a\x0F6a\x03\x02V[\x91\x82\x91\x82a\x04\x1BV[\x03\x90\xF3[a\x03\x08V[4a\x0FxWa\x0Fta\x0Fca\x0F^6`\x04a\x08\xFFV[a\"0V[a\x0Fka\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90V[a\x0F\x94a\x0F\x8Fa\x0F\x99\x92a\x0F}V[a\x08\x90V[a\x04\xA2V[\x90V[a\x0F\xB1k\x02\xE8vi\xC3\x08sj\x04\0\0\0a\x0F\x80V[\x90V[a\x0F\xBCa\x0F\x9CV[\x90V[4a\x0F\xEFWa\x0F\xCF6`\x04a\x03\xB9V[a\x0F\xEBa\x0F\xDAa\x0F\xB4V[a\x0F\xE2a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90V[_\x1B\x90V[a\x10\x10a\x10\x0Ba\x10\x15\x92a\x0F\xF4V[a\x0F\xF7V[a\x05\xF2V[\x90V[a\x10!_a\x0F\xFCV[\x90V[a\x10,a\x10\x18V[\x90V[4a\x10_Wa\x10?6`\x04a\x03\xB9V[a\x10[a\x10Ja\x10$V[a\x10Ra\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[4a\x10\x95Wa\x10\x91a\x10\x80a\x10z6`\x04a\x04\xC8V[\x90a\"_V[a\x10\x88a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[\x1C\x90V[\x90V[a\x10\xB1\x90`\x08a\x10\xB6\x93\x02a\x10\x9AV[a\x10\x9EV[\x90V[\x90a\x10\xC4\x91Ta\x10\xA1V[\x90V[a\x10\xD3`\x0C_\x90a\x10\xB9V[\x90V[4a\x11\x06Wa\x10\xE66`\x04a\x03\xB9V[a\x11\x02a\x10\xF1a\x10\xC7V[a\x10\xF9a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x11<Wa\x118a\x11'a\x11!6`\x04a\x04\xC8V[\x90a\"\x81V[a\x11/a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x11qWa\x11ma\x11\\a\x11W6`\x04a\x08\xFFV[a\"\x97V[a\x11da\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x11\xA6Wa\x11\x866`\x04a\x03\xB9V[a\x11\xA2a\x11\x91a\"\xACV[a\x11\x99a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[a\x11\xB4\x81a\x06\xF3V[\x03a\x11\xBBWV[_\x80\xFD[\x90P5\x90a\x11\xCC\x82a\x11\xABV[V[\x90\x91`\xC0\x82\x84\x03\x12a\x12-Wa\x11\xE6\x83_\x84\x01a\x04\x93V[\x92a\x11\xF4\x81` \x85\x01a\x04\xB9V[\x92a\x12\x02\x82`@\x83\x01a\x04\xB9V[\x92a\x12*a\x12\x13\x84``\x85\x01a\x11\xBFV[\x93a\x12!\x81`\x80\x86\x01a\x06\tV[\x93`\xA0\x01a\x06\tV[\x90V[a\x03\x0CV[4a\x12gWa\x12Qa\x12E6`\x04a\x11\xCEV[\x94\x93\x90\x93\x92\x91\x92a#,V[a\x12Ya\x03\x02V[\x80a\x12c\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[`\xE0\x81\x83\x03\x12a\x12\xD7Wa\x12\x82\x82_\x83\x01a\x04\x93V[\x92a\x12\x90\x83` \x84\x01a\x04\x93V[\x92a\x12\x9E\x81`@\x85\x01a\x04\xB9V[\x92a\x12\xAC\x82``\x83\x01a\x04\xB9V[\x92a\x12\xD4a\x12\xBD\x84`\x80\x85\x01a\x11\xBFV[\x93a\x12\xCB\x81`\xA0\x86\x01a\x06\tV[\x93`\xC0\x01a\x06\tV[\x90V[a\x03\x0CV[4a\x13\x11Wa\x12\xFBa\x12\xEF6`\x04a\x12lV[\x95\x94\x90\x94\x93\x91\x93a$\x80V[a\x13\x03a\x03\x02V[\x80a\x13\r\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\x13EWa\x13/a\x13)6`\x04a\x06\x8DV[\x90a%\x9EV[a\x137a\x03\x02V[\x80a\x13A\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[\x91\x90`@\x83\x82\x03\x12a\x13rW\x80a\x13fa\x13o\x92_\x86\x01a\x04\x93V[\x93` \x01a\x04\x93V[\x90V[a\x03\x0CV[4a\x13\xA8Wa\x13\xA4a\x13\x93a\x13\x8D6`\x04a\x13JV[\x90a%\xC0V[a\x13\x9Ba\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[a\x13\xB6\x81a\t\xA7V[\x03a\x13\xBDWV[_\x80\xFD[\x90P5\x90a\x13\xCE\x82a\x13\xADV[V[\x91\x90`@\x83\x82\x03\x12a\x13\xF8W\x80a\x13\xECa\x13\xF5\x92_\x86\x01a\x04\x93V[\x93` \x01a\x13\xC1V[\x90V[a\x03\x0CV[a\x14\x06\x90a\x0E\xB1V[\x90RV[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x14\x1E\x90a\x14\nV[\x90RV[\x90` \x80a\x14D\x93a\x14:_\x82\x01Q_\x86\x01\x90a\x13\xFDV[\x01Q\x91\x01\x90a\x14\x15V[V[\x91\x90a\x14Y\x90_`@\x85\x01\x94\x01\x90a\x14\"V[V[4a\x14\x8CWa\x14\x88a\x14wa\x14q6`\x04a\x13\xD0V[\x90a&.V[a\x14\x7Fa\x03\x02V[\x91\x82\x91\x82a\x14FV[\x03\x90\xF3[a\x03\x08V[_\x80\xFD[_\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x14\xB9a\x14\xBF\x91a\x14\nV[\x91a\x14\nV[\x01\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x14\xD0WV[a\x14\x99V[\x90a\x14\xE8\x91a\x14\xE2a\x14\x95V[Pa\x14\xADV[\x90V[a\x14\xF7a\x14\xFD\x91a\x14\nV[\x91a\x14\nV[\x90\x03\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x15\x0FWV[a\x14\x99V[\x90a\x15'\x91a\x15!a\x14\x95V[Pa\x14\xEBV[\x90V[_\x90V[a\x156a\x15*V[P\x80a\x15Qa\x15Kcye\xDB\x0B`\xE0\x1Ba\x03\x10V[\x91a\x03\x10V[\x14\x90\x81\x15a\x15^W[P\x90V[a\x15h\x91Pa&DV[_a\x15ZV[``\x90V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x01`\x02\x83\x04\x92\x16\x80\x15a\x15\xA7W[` \x83\x10\x14a\x15\xA2WV[a\x15sV[\x91`\x7F\x16\x91a\x15\x97V[` \x91\x81R\x01\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a\x15\xDDa\x15\xD6\x83a\x15\x87V[\x80\x94a\x15\xB1V[\x91`\x01\x81\x16\x90\x81_\x14a\x164WP`\x01\x14a\x15\xF8W[PPPV[a\x16\x05\x91\x92\x93\x94Pa\x15\xBAV[\x91_\x92[\x81\x84\x10a\x16\x1CWPP\x01\x90_\x80\x80a\x15\xF3V[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a\x16\tV[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a\x15\xF3V[\x90a\x16Y\x91a\x15\xC3V[\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x16z\x90a\x03\xE0V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x16\x94W`@RV[a\x16\\V[\x90a\x16\xB9a\x16\xB2\x92a\x16\xA9a\x03\x02V[\x93\x84\x80\x92a\x16OV[\x03\x83a\x16pV[V[a\x16\xC4\x90a\x16\x99V[\x90V[a\x16\xCFa\x15nV[Pa\x16\xDA`\x03a\x16\xBBV[\x90V[a\x16\xFA\x91a\x16\xE9a\x15*V[Pa\x16\xF2a&jV[\x91\x90\x91a&wV[`\x01\x90V[_\x90V[_\x1C\x90V[a\x17\x14a\x17\x19\x91a\x17\x03V[a\x10\x9EV[\x90V[a\x17&\x90Ta\x17\x08V[\x90V[a\x171a\x16\xFFV[Pa\x17<`\x02a\x17\x1CV[\x90V[\x91a\x17i\x92a\x17La\x15*V[Pa\x17aa\x17Xa&jV[\x82\x90\x84\x91a&\xC7V[\x91\x90\x91a'SV[`\x01\x90V[_\x90V[a\x17{\x90a\x05\xF2V[\x90V[\x90a\x17\x88\x90a\x17rV[_R` R`@_ \x90V[\x90V[a\x17\xA3a\x17\xA8\x91a\x17\x03V[a\x17\x94V[\x90V[a\x17\xB5\x90Ta\x17\x97V[\x90V[`\x01a\x17\xD1a\x17\xD7\x92a\x17\xC9a\x17nV[P`\x05a\x17~V[\x01a\x17\xABV[\x90V[\x90a\x17\xF5\x91a\x17\xF0a\x17\xEB\x82a\x17\xB8V[a'\xF0V[a\x17\xF7V[V[\x90a\x18\x01\x91a(IV[PV[\x90a\x18\x0E\x91a\x17\xDAV[V[_\x90V[\x90V[a\x18+a\x18&a\x180\x92a\x18\x14V[a\x08\x90V[a\x06\xF3V[\x90V[a\x18;a\x18\x10V[Pa\x18F`\x12a\x18\x17V[\x90V[a\x18Qa\x17nV[Pa\x18Za(\xF5V[\x90V[\x90\x80a\x18xa\x18ra\x18ma&jV[a\x04sV[\x91a\x04sV[\x03a\x18\x89Wa\x18\x86\x91a)\xAFV[PV[_c3K\xD9\x19`\xE1\x1B\x81R\x80a\x18\xA1`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[a\x18\xB9a\x18\xB4a\x18\xBE\x92a\x04hV[a\x08\x90V[a\x04hV[\x90V[a\x18\xCA\x90a\x18\xA5V[\x90V[a\x18\xD6\x90a\x18\xC1V[\x90V[\x90a\x18\xE3\x90a\x18\xCDV[_R` R`@_ \x90V[\x90V[a\x19\x06a\x19\x01a\x19\x0B\x92a\x14\nV[a\x08\x90V[a\x04\xA2V[\x90V[a\x19E\x91a\x19:a\x194a\x19/a\x19@\x94a\x19'a\x16\xFFV[P`\na\x18\xD9V[a\x18\xEFV[\x91a*\x90V[\x90a+\xA5V[a\x18\xF2V[\x90V[\x90a\x19b\x91a\x19]a\x19Xa\rkV[a'\xF0V[a\x19\xCDV[V[a\x19xa\x19sa\x19}\x92a\x0F\xF4V[a\x08\x90V[a\x04hV[\x90V[a\x19\x89\x90a\x19dV[\x90V[a\x19\xA0a\x19\x9Ba\x19\xA5\x92a\x0F\xF4V[a\x08\x90V[a\x04\xA2V[\x90V[a\x19\xB7a\x19\xBD\x91\x93\x92\x93a\x04\xA2V[\x92a\x04\xA2V[\x82\x01\x80\x92\x11a\x19\xC8WV[a\x14\x99V[\x90\x81a\x19\xE9a\x19\xE3a\x19\xDE_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14a\x1AyW\x80a\x1A\x01a\x19\xFB_a\x19\x8CV[\x91a\x04\xA2V[\x14a\x1A]Wa\x1A\x18a\x1A\x11a\x17)V[\x82\x90a\x19\xA8V[a\x1A1a\x1A+a\x1A&a\x0E#V[a\x04\xA2V[\x91a\x04\xA2V[\x11a\x1AAWa\x1A?\x91a,\xCCV[V[_c\x17~?\xC3`\xE0\x1B\x81R\x80a\x1AY`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1Au`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x1A\x91`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x90a\x1A\x9F\x91a\x19HV[V[a\x1A\xB0a\x1A\xB6\x91\x93\x92\x93a\x04\xA2V[\x92a\x04\xA2V[\x82\x03\x91\x82\x11a\x1A\xC1WV[a\x14\x99V[a\x1A\xCEa\x16\xFFV[Pa\x1A\xE8a\x1A\xDAa\x0E#V[a\x1A\xE2a\x17)V[\x90a\x1A\xA1V[\x90V[\x90a\x1A\xFEa\x1A\xF7a\x03\x02V[\x92\x83a\x16pV[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1B\x1EWa\x1B\x1A` \x91a\x03\xE0V[\x01\x90V[a\x16\\V[\x90a\x1B5a\x1B0\x83a\x1B\0V[a\x1A\xEBV[\x91\x82RV[_\x7Fmode=blocknumber&from=default\0\0\0\x91\x01RV[a\x1Bk`\x1Da\x1B#V[\x90a\x1Bx` \x83\x01a\x1B:V[V[a\x1B\x82a\x1BaV[\x90V[a\x1B\x8Da\x15nV[Pa\x1B\x96a\"\x06V[a\x1B\xAFa\x1B\xA9a\x1B\xA4a-*V[a\x0E\xB1V[\x91a\x0E\xB1V[\x03a\x1B\xBFWa\x1B\xBCa\x1BzV[\x90V[_c\x01\xBF\xC1\xC5`\xE6\x1B\x81R\x80a\x1B\xD7`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_\x90V[\x90a\x1B\xE9\x90a\x18\xCDV[_R` R`@_ \x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x1C\x0Ca\x1C\x11\x91a\x17\x03V[a\x1B\xF5V[\x90V[a\x1C\x1E\x90Ta\x1C\0V[\x90V[a\x1C8a\x1C=\x91a\x1C0a\x1B\xDBV[P`\ta\x1B\xDFV[a\x1C\x14V[\x90V[a\x1CQ\x90a\x1CLa&jV[a-}V[V[_\x90V[a\x1Ci\x90a\x1Cca\x1CSV[Pa.\x08V[\x90V[\x90a\x1Cv\x90a\x18\xCDV[_R` R`@_ \x90V[a\x1C\x98a\x1C\x9D\x91a\x1C\x91a\x16\xFFV[P_a\x1ClV[a\x17\x1CV[\x90V[\x90a\x1C\xBA\x91a\x1C\xB5a\x1C\xB0a\r\x07V[a'\xF0V[a\x1C\xBCV[V[\x80a\x1C\xD7a\x1C\xD1a\x1C\xCC_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14a\x1D\x9BW\x81a\x1C\xEFa\x1C\xE9_a\x19\x8CV[\x91a\x04\xA2V[\x14a\x1D\x7FWa\x1D\x05a\x1C\xFFa\x1EPV[\x15a\x03]V[a\x1DcWa\x1D\x14\x81\x83\x90a.7V[3\x90a\x1D^a\x1DLa\x1DF\x7F\xBE\xF4\xF8\x1C\x18\x14\xC6A\xED\xE8^\xBA\xAC\xF1\x9D\x04\x8B,[U\x98\n\xDF\xA6\xEF\x0F\x95le\x135\xA2\x93a\x18\xCDV[\x93a\x18\xCDV[\x93a\x1DUa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xA3V[_c\xB8\xB5\xCA-`\xE0\x1B\x81R\x80a\x1D{`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1D\x97`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x1D\xB3`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x90a\x1D\xC1\x91a\x1C\xA0V[V[a\x1D\xCBa\x16\xFFV[Pa\x1D\xD6`\x0Ca\x17\x1CV[a\x1D\xE8a\x1D\xE2_a\x19\x8CV[\x91a\x04\xA2V[\x14\x80\x15a\x1E\x17W[a\x1E\x0BWa\x1E\x08a\x1E\x01`\x0Ca\x17\x1CV[B\x90a\x1A\xA1V[\x90V[a\x1E\x14_a\x19\x8CV[\x90V[PBa\x1E4a\x1E.a\x1E)`\x0Ca\x17\x1CV[a\x04\xA2V[\x91a\x04\xA2V[\x10\x15a\x1D\xF0V[a\x1EM\x90a\x1EGa\x16\xFFV[Pa.\x96V[\x90V[a\x1EXa\x15*V[Pa\x1Ec`\x0Ca\x17\x1CV[a\x1Eua\x1Eo_a\x19\x8CV[\x91a\x04\xA2V[\x14\x15\x80a\x1E\x80W[\x90V[PBa\x1E\x9Da\x1E\x97a\x1E\x92`\x0Ca\x17\x1CV[a\x04\xA2V[\x91a\x04\xA2V[\x10a\x1E}V[a\x1E\xBC\x90a\x1E\xB7a\x1E\xB2a\x10\x18V[a'\xF0V[a\x1F6V[V[\x90a\x1E\xCA_\x19\x91a\x0F\xF7V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x1E\xE8a\x1E\xE3a\x1E\xED\x92a\x04\xA2V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[\x90a\x1F\x08a\x1F\x03a\x1F\x0F\x92a\x1E\xD4V[a\x1E\xF0V[\x82Ta\x1E\xBEV[\x90UV[\x91` a\x1F4\x92\x94\x93a\x1F-`@\x82\x01\x96_\x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[a\x1F@`\x0Ca\x17\x1CV[a\x1FRa\x1FL_a\x19\x8CV[\x91a\x04\xA2V[\x03a <W\x80a\x1Fja\x1FdBa\x04\xA2V[\x91a\x04\xA2V[\x11\x15a  W\x80a\x1F\xA3a\x1F\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xA2V[\x91a\x04\xA2V[\x11a \x04Wa\x1F\xB2`\x0Ca\x17\x1CV[a\x1F\xBD\x82`\x0Ca\x1E\xF3V[\x903\x90a\x1F\xEA\x7F\xDDh\x96\xDC\xF1\xD4\xB3\x11\xCC\xA8}\xD1\x9B\xBB\xA2\xEA\x9C\xE2\xF8g\xC1V\x88x\xA0C\x8Af\xA1\xAF\xEE\xEC\x92a\x18\xCDV[\x92a\x1F\xFFa\x1F\xF6a\x03\x02V[\x92\x83\x92\x83a\x1F\x13V[\x03\x90\xA2V[_c\xEFi\xAFe`\xE0\x1B\x81R\x80a \x1C`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\xA5e\x83S`\xE0\x1B\x81R\x80a 8`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c7\xE9x\xD3`\xE1\x1B\x81R\x80a T`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[a a\x90a\x1E\xA3V[V[_\x90V[``\x90V[a u\x90a\x18\xC1V[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a \x90W` \x80\x91\x02\x01\x90V[a\x16\\V[\x90a \xA7a \xA2\x83a xV[a\x1A\xEBV[\x91\x82RV[6\x907V[\x90a \xD6a \xBE\x83a \x95V[\x92` \x80a \xCC\x86\x93a xV[\x92\x01\x91\x03\x90a \xACV[V[`\x0F`\xF8\x1B\x90V[a \xE8a cV[Pa \xF1a\x15nV[Pa \xFAa\x15nV[Pa!\x03a\x16\xFFV[Pa!\x0Ca\x1B\xDBV[Pa!\x15a\x17nV[Pa!\x1Ea gV[Pa!'a.\xAEV[\x90a!0a.\xEEV[\x90F\x90a!<0a lV[\x90a!F_a\x0F\xFCV[\x90a!Xa!S_a\x19\x8CV[a \xB1V[\x90a!aa \xD8V[\x96\x95\x94\x93\x92\x91\x90V[a!\x93a!\x98\x91a!ya\x16\xFFV[Pa!\x8Da!\x87`\x0Ba\x18\xEFV[\x91a*\x90V[\x90a+\xA5V[a\x18\xF2V[\x90V[\x90a!\xA5\x90a\x18\xCDV[_R` R`@_ \x90V[`\xFF\x16\x90V[a!\xC3a!\xC8\x91a\x17\x03V[a!\xB1V[\x90V[a!\xD5\x90Ta!\xB7V[\x90V[a!\xFF\x91_a!\xF4a!\xFA\x93a!\xECa\x15*V[P`\x05a\x17~V[\x01a!\x9BV[a!\xCBV[\x90V[_\x90V[a\"\x0Ea\"\x02V[Pa\"\x17a-*V[\x90V[a\"\"a\x15nV[Pa\"-`\x04a\x16\xBBV[\x90V[a\"Wa\"Ra\"Ma\"\\\x93a\"Ea\x16\xFFV[P`\na\x18\xD9V[a\x18\xEFV[a/.V[a\x18\xF2V[\x90V[a\"|\x91a\"ka\x15*V[Pa\"ta&jV[\x91\x90\x91a'SV[`\x01\x90V[\x90a\"\x94\x91a\"\x8Ea\x16\xFFV[Pa\x19\x0EV[\x90V[a\"\xA9\x90a\"\xA3a\x16\xFFV[Pa\"0V[\x90V[a\"\xB4a\x16\xFFV[Pa\"\xBDa\x17)V[\x90V[\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF\x90V[a#\x19a# \x94a#\x0F``\x94\x98\x97\x95a#\x05`\x80\x86\x01\x9A_\x87\x01\x90a\x066V[` \x85\x01\x90a\t\x1DV[`@\x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[` \x01\x90V[Q\x90V[\x93\x95\x94\x90\x92\x91\x95Ba#Fa#@\x89a\x04\xA2V[\x91a\x04\xA2V[\x11a#\xBFW\x91a#\xB1\x91a#\xB8\x93a#\xA8a#\xBD\x98\x99a#\x90a#ga\"\xC0V[a#\x81\x8B\x93\x8Ba#ua\x03\x02V[\x95\x86\x94` \x86\x01a\"\xE4V[` \x82\x01\x81\x03\x82R\x03\x82a\x16pV[a#\xA2a#\x9C\x82a#(V[\x91a#\"V[ a/\xA3V[\x92\x90\x91\x92a/\xC0V[\x91\x82a0\nV[a-}V[V[a#\xDA\x87_\x91\x82\x91c#A\xD7\x87`\xE1\x1B\x83R`\x04\x83\x01a\x058V[\x03\x90\xFD[\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x90V[\x91\x94a$Ja$T\x92\x98\x97\x95a$@`\xA0\x96a$6a$[\x9Aa$,`\xC0\x8A\x01\x9E_\x8B\x01\x90a\x066V[` \x89\x01\x90a\t\x1DV[`@\x87\x01\x90a\t\x1DV[``\x85\x01\x90a\x05+V[`\x80\x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[\x91` a$~\x92\x94\x93a$w`@\x82\x01\x96_\x83\x01\x90a\t\x1DV[\x01\x90a\t\x1DV[V[\x96\x95\x91\x93\x92\x94\x90\x94Ba$\x9Ba$\x95\x83a\x04\xA2V[\x91a\x04\xA2V[\x11a%UW\x90a%\x04a%\r\x94\x93\x92a$\xECa$\xB5a#\xDEV[a$\xDD\x8C\x80\x94\x8C\x91a$\xC7\x8D\x91a0\\V[\x91\x92a$\xD1a\x03\x02V[\x97\x88\x96` \x88\x01a$\x02V[` \x82\x01\x81\x03\x82R\x03\x82a\x16pV[a$\xFEa$\xF8\x82a#(V[\x91a#\"V[ a/\xA3V[\x92\x90\x91\x92a/\xC0V[\x80a% a%\x1A\x87a\x04sV[\x91a\x04sV[\x03a%5WPa%3\x92\x93\x91\x90\x91a&wV[V[\x84\x90a%Q_\x92\x83\x92c%\xC0\x07#`\xE1\x1B\x84R`\x04\x84\x01a$]V[\x03\x90\xFD[a%p\x90_\x91\x82\x91c1<\x89\x81`\xE1\x1B\x83R`\x04\x83\x01a\x058V[\x03\x90\xFD[\x90a%\x8F\x91a%\x8Aa%\x85\x82a\x17\xB8V[a'\xF0V[a%\x91V[V[\x90a%\x9B\x91a)\xAFV[PV[\x90a%\xA8\x91a%tV[V[\x90a%\xB4\x90a\x18\xCDV[_R` R`@_ \x90V[a%\xE5\x91a%\xDBa%\xE0\x92a%\xD3a\x16\xFFV[P`\x01a%\xAAV[a\x1ClV[a\x17\x1CV[\x90V[a%\xF2`@a\x1A\xEBV[\x90V[_\x90V[_\x90V[a&\x05a%\xE8V[\x90` \x80\x83a&\x12a%\xF5V[\x81R\x01a&\x1Da%\xF9V[\x81RPPV[a&+a%\xFDV[\x90V[\x90a&A\x91a&;a&#V[Pa0\x8FV[\x90V[a&La\x15*V[Pa&fa&`c\x01\xFF\xC9\xA7`\xE0\x1Ba\x03\x10V[\x91a\x03\x10V[\x14\x90V[a&ra\x1B\xDBV[P3\x90V[\x91a&\x85\x92\x91`\x01\x92a0\xB7V[V[`@\x90a&\xB0a&\xB7\x94\x96\x95\x93\x96a&\xA6``\x84\x01\x98_\x85\x01\x90a\t\x1DV[` \x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[\x90a&\xC4\x91\x03a\x04\xA2V[\x90V[\x92\x91\x92a&\xD5\x81\x83\x90a%\xC0V[\x90\x81a&\xEAa&\xE4_\x19a\x04\xA2V[\x91a\x04\xA2V[\x10a&\xF7W[PPP\x90PV[\x81a'\na'\x04\x87a\x04\xA2V[\x91a\x04\xA2V[\x10a'0Wa''\x93\x94a'\x1F\x91\x93\x92a&\xB9V[\x90_\x92a0\xB7V[\x80_\x80\x80a&\xF0V[Pa'O\x84\x92\x91\x92_\x93\x84\x93c}\xC7\xA0\xD9`\xE1\x1B\x85R`\x04\x85\x01a&\x87V[\x03\x90\xFD[\x91\x82a'oa'ia'd_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14a'\xC9W\x81a'\x8Fa'\x89a'\x84_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14a'\xA2Wa'\xA0\x92\x91\x90\x91a1\xC6V[V[a'\xC5a'\xAE_a\x19\x80V[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\t*V[\x03\x90\xFD[a'\xECa'\xD5_a\x19\x80V[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\t*V[\x03\x90\xFD[a(\x02\x90a'\xFCa&jV[\x90a2\x93V[V[\x90a(\x10`\xFF\x91a\x0F\xF7V[\x91\x81\x19\x16\x91\x16\x17\x90V[a(#\x90a\x03]V[\x90V[\x90V[\x90a(>a(9a(E\x92a(\x1AV[a(&V[\x82Ta(\x04V[\x90UV[a(Qa\x15*V[Pa(fa(`\x82\x84\x90a!\xD8V[\x15a\x03]V[_\x14a(\xEFWa(\x8E`\x01a(\x89_a(\x81`\x05\x86\x90a\x17~V[\x01\x85\x90a!\x9BV[a()V[\x90a(\x97a&jV[\x90a(\xD4a(\xCEa(\xC8\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x95a\x17rV[\x92a\x18\xCDV[\x92a\x18\xCDV[\x92a(\xDDa\x03\x02V[\x80a(\xE7\x81a\x06\xBAV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a(\xFDa\x17nV[Pa)\x070a lV[a)9a)3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04sV[\x91a\x04sV[\x14\x80a)uW[_\x14a)jW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a)ra3?V[\x90V[PFa)\xA9a)\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xA2V[\x91a\x04\xA2V[\x14a)@V[a)\xB7a\x15*V[Pa)\xC3\x81\x83\x90a!\xD8V[_\x14a*KWa)\xEA_a)\xE5_a)\xDD`\x05\x86\x90a\x17~V[\x01\x85\x90a!\x9BV[a()V[\x90a)\xF3a&jV[\x90a*0a**a*$\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x95a\x17rV[\x92a\x18\xCDV[\x92a\x18\xCDV[\x92a*9a\x03\x02V[\x80a*C\x81a\x06\xBAV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a*ea*`a*j\x92a\x0E\xB1V[a\x08\x90V[a\x04\xA2V[\x90V[\x91` a*\x8E\x92\x94\x93a*\x87`@\x82\x01\x96_\x83\x01\x90a\x05+V[\x01\x90a\x0E\xBCV[V[a*\x98a\"\x02V[Pa*\xA1a\"\x06V[\x81a*\xB4a*\xAE\x83a*QV[\x91a\x04\xA2V[\x10\x15a*\xC7WPa*\xC4\x90a4HV[\x90V[\x90a*\xE2_\x92\x83\x92cvi\xFC\x0F`\xE1\x1B\x84R`\x04\x84\x01a*mV[\x03\x90\xFD[T\x90V[\x90V[a+\x01a*\xFCa+\x06\x92a*\xEAV[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a+#a+(\x91a\x17\x03V[a+\x0CV[\x90V[a+5\x90Ta+\x17V[\x90V[\x90V[a+Oa+Ja+T\x92a+8V[a\x08\x90V[a\x04\xA2V[\x90V[`0\x1C\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a+ta+y\x91a+WV[a+]V[\x90V[a+\x86\x90Ta+hV[\x90V[a+\x9Da+\x98a+\xA2\x92a\x0F\xF4V[a\x08\x90V[a\x14\nV[\x90V[\x90a+\xF9\x90a+\xB2a\x14\x95V[Pa+\xBE_\x84\x01a*\xE6V[a+\xC7_a\x19\x8CV[\x90\x80\x80a+\xDDa+\xD7`\x05a*\xEDV[\x91a\x04\xA2V[\x11a,ZW[P\x90a+\xF4_\x86\x01\x93\x91\x92\x93a+\tV[a:\x9BV[\x80a,\x0Ca,\x06_a\x19\x8CV[\x91a\x04\xA2V[\x14_\x14a,\"WPPa,\x1E_a+\x89V[[\x90V[a,O_\x91a,Ja,D\x84a,U\x96\x01\x92a,>`\x01a+;V[\x90a\x1A\xA1V[\x91a+\tV[a:\x91V[\x01a+|V[a,\x1FV[\x80a,ha,n\x92\x91a7&V[\x90a\x1A\xA1V[\x90\x83a,\xA0a,\x9Aa,\x95_a,\x8F\x81\x8C\x01a,\x8A\x89\x91a+\tV[a:\x91V[\x01a++V[a\x0E\xB1V[\x91a\x0E\xB1V[\x10_\x14a,\xB1WP\x90[\x90_a+\xE3V[\x91Pa,\xC7\x90a,\xC1`\x01a+;V[\x90a\x19\xA8V[a,\xAAV[\x80a,\xE7a,\xE1a,\xDC_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14a-\x03Wa-\x01\x91a,\xF9_a\x19\x80V[\x91\x90\x91a1\xC6V[V[a-&a-\x0F_a\x19\x80V[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\t*V[\x03\x90\xFD[a-2a\"\x02V[Pa-<Ca4HV[\x90V[\x90a-P`\x01\x80`\xA0\x1B\x03\x91a\x0F\xF7V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a-ra-ma-y\x92a\x18\xCDV[a-ZV[\x82Ta-?V[\x90UV[\x90a.\x06\x91a.\0a-\x8E\x82a\x1C!V[a-\xA3\x84a-\x9E`\t\x86\x90a\x1B\xDFV[a-]V[\x82\x81\x85\x90a-\xE3a-\xDDa-\xD7\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x95a\x18\xCDV[\x92a\x18\xCDV[\x92a\x18\xCDV[\x92a-\xECa\x03\x02V[\x80a-\xF6\x81a\x06\xBAV[\x03\x90\xA4\x92\x91a;*V[\x91a;BV[V[a./a.*a.%a.4\x93a.\x1Da\x1CSV[P`\na\x18\xD9V[a\x18\xEFV[a<\xF0V[a=oV[\x90V[\x90\x81a.Sa.Ma.H_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14a.oWa.m\x91\x90a.f_a\x19\x80V[\x90\x91a1\xC6V[V[a.\x92a.{_a\x19\x80V[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\t*V[\x03\x90\xFD[a.\xA8\x90a.\xA2a\x16\xFFV[Pa=\xC0V[\x90V[\x90V[a.\xB6a\x15nV[Pa.\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a.\xE5`\x06a.\xABV[\x90a>\xDBV[\x90V[a.\xF6a\x15nV[Pa/+\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a/%`\x07a.\xABV[\x90a>\xDBV[\x90V[a/6a\x14\x95V[Pa/B_\x82\x01a*\xE6V[\x80a/Ua/O_a\x19\x8CV[\x91a\x04\xA2V[\x14_\x14a/kWPPa/g_a+\x89V[[\x90V[a/\x98_\x91a/\x93a/\x8D\x84a/\x9E\x96\x01\x92a/\x87`\x01a+;V[\x90a\x1A\xA1V[\x91a+\tV[a:\x91V[\x01a+|V[a/hV[a/\xBD\x90a/\xAFa\x17nV[Pa/\xB8a(\xF5V[a?)V[\x90V[\x92a/\xDB\x92a/\xE4\x94a/\xD1a\x1B\xDBV[P\x92\x90\x91\x92a?\xEFV[\x90\x92\x91\x92aA\x1AV[\x90V[\x91` a0\x08\x92\x94\x93a0\x01`@\x82\x01\x96_\x83\x01\x90a\t\x1DV[\x01\x90a\x05+V[V[a0\x13\x81a0\\V[\x91a0&a0 \x84a\x04\xA2V[\x91a\x04\xA2V[\x03a0/WPPV[a0I_\x92\x83\x92c\x01\xD4\xB6#`\xE6\x1B\x84R`\x04\x84\x01a/\xE7V[\x03\x90\xFD[`\x01a0Y\x91\x01a\x04\xA2V[\x90V[a0p\x90a0ha\x16\xFFV[P`\x08a\x1ClV[a0\x8Ca0|\x82a\x17\x1CV[\x91a0\x86\x83a0MV[\x90a\x1E\xF3V[\x90V[\x90a0\xAFa0\xAAa0\xB4\x93a0\xA2a&#V[P`\na\x18\xD9V[a\x18\xEFV[aB\x90V[\x90V[\x90\x92\x81a0\xD4a0\xCEa0\xC9_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14a1\x9FW\x83a0\xF4a0\xEEa0\xE9_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14a1xWa1\x18\x83a1\x13a1\x0C`\x01\x86\x90a%\xAAV[\x87\x90a\x1ClV[a\x1E\xF3V[a1\"W[PPPV[\x91\x90\x91a1ma1[a1U\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x93a\x18\xCDV[\x93a\x18\xCDV[\x93a1da\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xA3_\x80\x80a1\x1DV[a1\x9Ba1\x84_a\x19\x80V[_\x91\x82\x91cJ\x14\x06\xB1`\xE1\x1B\x83R`\x04\x83\x01a\t*V[\x03\x90\xFD[a1\xC2a1\xAB_a\x19\x80V[_\x91\x82\x91c\xE6\x02\xDF\x05`\xE0\x1B\x83R`\x04\x83\x01a\t*V[\x03\x90\xFD[\x91\x82a1\xE2a1\xDCa1\xD7_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14\x15\x80a2MW[a1\xFDW[a1\xFB\x92\x91\x90\x91aB\xB1V[V[a2\x05a\x1EPV[\x80a2,W[\x15a1\xEFW_c6\xE2x\xFD`\xE2\x1B\x81R\x80a2(`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[Pa2Ha2Ba2;a\r\x07V[3\x90a!\xD8V[\x15a\x03]V[a2\x0BV[P\x81a2ia2ca2^_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14\x15a1\xEAV[\x91` a2\x91\x92\x94\x93a2\x8A`@\x82\x01\x96_\x83\x01\x90a\t\x1DV[\x01\x90a\x066V[V[\x90a2\xA8a2\xA2\x83\x83\x90a!\xD8V[\x15a\x03]V[a2\xB0WPPV[a2\xCA_\x92\x83\x92c\xE2Q}?`\xE0\x1B\x84R`\x04\x84\x01a2pV[\x03\x90\xFD[\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x90V[\x90\x95\x94\x92a3=\x94a3,a36\x92a3\"`\x80\x96a3\x18`\xA0\x88\x01\x9C_\x89\x01\x90a\x066V[` \x87\x01\x90a\x066V[`@\x85\x01\x90a\x066V[``\x83\x01\x90a\x05+V[\x01\x90a\t\x1DV[V[a3Ga\x17nV[Pa3Pa2\xCEV[a3\xC7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a3\xB8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Fa3\xA30a lV[\x91a3\xACa\x03\x02V[\x96\x87\x95` \x87\x01a2\xF2V[` \x82\x01\x81\x03\x82R\x03\x82a\x16pV[a3\xD9a3\xD3\x82a#(V[\x91a#\"V[ \x90V[\x90V[a3\xF4a3\xEFa3\xF9\x92a3\xDDV[a\x08\x90V[a\x06\xF3V[\x90V[a4\x05\x90a3\xE0V[\x90RV[\x91` a4*\x92\x94\x93a4#`@\x82\x01\x96_\x83\x01\x90a3\xFCV[\x01\x90a\x05+V[V[a4@a4;a4E\x92a\x04\xA2V[a\x08\x90V[a\x0E\xB1V[\x90V[a4Pa\"\x02V[P\x80a4ja4de\xFF\xFF\xFF\xFF\xFF\xFFa*QV[\x91a\x04\xA2V[\x11a4{Wa4x\x90a4,V[\x90V[`0a4\x97_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a4\tV[\x03\x90\xFD[\x90V[a4\xB2a4\xADa4\xB7\x92a4\x9BV[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a4\xD1a4\xCCa4\xD6\x92a4\xBAV[a\x08\x90V[a\x06\xF3V[\x90V[a4\xF8\x90a4\xF2a4\xECa4\xFD\x94a\x06\xF3V[\x91a\x04\xA2V[\x90a\x10\x9AV[a\x04\xA2V[\x90V[\x90V[a5\x17a5\x12a5\x1C\x92a5\0V[a\x08\x90V[a\x06\xF3V[\x90V[\x1B\x90V[a5B\x90a5<a56a5G\x94a\x06\xF3V[\x91a\x04\xA2V[\x90a5\x1FV[a\x04\xA2V[\x90V[\x90V[a5aa5\\a5f\x92a5JV[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a5\x80a5{a5\x85\x92a5iV[a\x08\x90V[a\x06\xF3V[\x90V[\x90V[a5\x9Fa5\x9Aa5\xA4\x92a5\x88V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a5\xBEa5\xB9a5\xC3\x92a5\xA7V[a\x08\x90V[a\x06\xF3V[\x90V[\x90V[a5\xDDa5\xD8a5\xE2\x92a5\xC6V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a5\xFCa5\xF7a6\x01\x92a5\xE5V[a\x08\x90V[a\x06\xF3V[\x90V[\x90V[a6\x1Ba6\x16a6 \x92a6\x04V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a6:a65a6?\x92a6#V[a\x08\x90V[a\x06\xF3V[\x90V[a6Va6Qa6[\x92a5\xA7V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a6ua6pa6z\x92a6^V[a\x08\x90V[a\x06\xF3V[\x90V[a6\x91a6\x8Ca6\x96\x92a6#V[a\x08\x90V[a\x04\xA2V[\x90V[a6\xADa6\xA8a6\xB2\x92a+8V[a\x08\x90V[a\x06\xF3V[\x90V[\x90V[a6\xCCa6\xC7a6\xD1\x92a6\xB5V[a\x08\x90V[a\x04\xA2V[\x90V[\x90a6\xDF\x91\x02a\x04\xA2V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a7\x02a7\x08\x91a\x04\xA2V[\x91a\x04\xA2V[\x90\x81\x15a7\x13W\x04\x90V[a6\xE2V[\x90a7#\x91\x01a\x04\xA2V[\x90V[a7.a\x16\xFFV[P\x80a7Ca7=`\x01a+;V[\x91a\x04\xA2V[\x11\x15a:\x8EW\x80a9Xa95a9%a9\x15a9\x05a8\xF5a8\xE5a8\xD5a8\xC5a8\xB5a8\xA5\x8Ba8\x9Fa8\x98a9^\x9Fa8xa8ha8\x88\x92a7\x8A`\x01a+;V[\x90\x80a7\xA2a7\x9C`\x01`\x80\x1Ba4\x9EV[\x91a\x04\xA2V[\x10\x15a:`W[\x80a7\xC5a7\xBFh\x01\0\0\0\0\0\0\0\0a5MV[\x91a\x04\xA2V[\x10\x15a:2W[\x80a7\xE4a7\xDEd\x01\0\0\0\0a5\x8BV[\x91a\x04\xA2V[\x10\x15a:\x04W[\x80a8\x01a7\xFBb\x01\0\0a5\xC9V[\x91a\x04\xA2V[\x10\x15a9\xD6W[\x80a8\x1Da8\x17a\x01\0a6\x07V[\x91a\x04\xA2V[\x10\x15a9\xA8W[\x80a88a82`\x10a6BV[\x91a\x04\xA2V[\x10\x15a9zW[a8Ra8L`\x04a6}V[\x91a\x04\xA2V[\x10\x15a9aW[a8c`\x03a6\xB8V[a6\xD4V[a8r`\x01a6\x99V[\x90a4\xD9V[a8\x82\x81\x86a6\xF6V[\x90a7\x18V[a8\x92`\x01a6\x99V[\x90a4\xD9V[\x80\x92a6\xF6V[\x90a7\x18V[a8\xAF`\x01a6\x99V[\x90a4\xD9V[a8\xBF\x81\x8Ca6\xF6V[\x90a7\x18V[a8\xCF`\x01a6\x99V[\x90a4\xD9V[a8\xDF\x81\x8Aa6\xF6V[\x90a7\x18V[a8\xEF`\x01a6\x99V[\x90a4\xD9V[a8\xFF\x81\x88a6\xF6V[\x90a7\x18V[a9\x0F`\x01a6\x99V[\x90a4\xD9V[a9\x1F\x81\x86a6\xF6V[\x90a7\x18V[a9/`\x01a6\x99V[\x90a4\xD9V[\x91a9Ra9La9G\x85\x80\x94a6\xF6V[a\x04\xA2V[\x91a\x04\xA2V[\x11aCAV[\x90a&\xB9V[\x90V[a9u\x90a9o`\x01a6\x99V[\x90a5#V[a8YV[a9\x91a9\xA2\x91a9\x8B`\x04a6&V[\x90a4\xD9V[\x91a9\x9C`\x02a6aV[\x90a5#V[\x90a8?V[a9\xBFa9\xD0\x91a9\xB9`\x08a5\xE8V[\x90a4\xD9V[\x91a9\xCA`\x04a6&V[\x90a5#V[\x90a8$V[a9\xEDa9\xFE\x91a9\xE7`\x10a5\xAAV[\x90a4\xD9V[\x91a9\xF8`\x08a5\xE8V[\x90a5#V[\x90a8\x08V[a:\x1Ba:,\x91a:\x15` a5lV[\x90a4\xD9V[\x91a:&`\x10a5\xAAV[\x90a5#V[\x90a7\xEBV[a:Ia:Z\x91a:C`@a5\x03V[\x90a4\xD9V[\x91a:T` a5lV[\x90a5#V[\x90a7\xCCV[a:wa:\x88\x91a:q`\x80a4\xBDV[\x90a4\xD9V[\x91a:\x82`@a5\x03V[\x90a5#V[\x90a7\xA9V[\x90V[_R` _ \x01\x90V[\x93\x91\x90\x92a:\xA7a\x16\xFFV[P[\x81a:\xBCa:\xB6\x83a\x04\xA2V[\x91a\x04\xA2V[\x10\x15a;\"Wa:\xCD\x82\x82\x90aC\x8DV[\x90a:\xE3_a:\xDD\x88\x85\x90a:\x91V[\x01a++V[a:\xF5a:\xEF\x87a\x0E\xB1V[\x91a\x0E\xB1V[\x11_\x14a;\x05WP\x91[\x91a:\xA9V[\x92\x91Pa;\x1C\x90a;\x16`\x01a+;V[\x90a\x19\xA8V[\x90a:\xFFV[\x92PP\x91P\x90V[a;<\x90a;6a\x16\xFFV[Pa\x1C\x82V[\x90V[\x90V[\x91\x90\x91\x80a;Xa;R\x85a\x04sV[\x91a\x04sV[\x14\x15\x80a<\xD6W[a;jW[PPPV[\x80a;\x85a;\x7Fa;z_a\x19\x80V[a\x04sV[\x91a\x04sV[\x03a<FW[P\x81a;\xA7a;\xA1a;\x9C_a\x19\x80V[a\x04sV[\x91a\x04sV[\x03a;\xB3W[\x80a;eV[a;\xFAa;\xEDa;\xF4\x92a;\xC9`\n\x86\x90a\x18\xD9V[\x90a;\xE7a;\xE1a;\xDB`\x01\x93aD&V[\x93a\x18\xEFV[\x91a;?V[\x90aDyV[\x92\x90a\x18\xF2V[\x91a\x18\xF2V[\x91\x90\x91a<'\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x18\xCDV[\x92a<<a<3a\x03\x02V[\x92\x83\x92\x83a\x1F\x13V[\x03\x90\xA2_\x80a;\xADV[a<\x85a<\x8Ba<~a<[`\n\x85\x90a\x18\xD9V[`\x02a<xa<ra<l\x89aD&V[\x93a\x18\xEFV[\x91a;?V[\x90aDyV[\x92\x90a\x18\xF2V[\x91a\x18\xF2V[\x91\x90\x91a<\xB8\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x18\xCDV[\x92a<\xCDa<\xC4a\x03\x02V[\x92\x83\x92\x83a\x1F\x13V[\x03\x90\xA2_a;\x8BV[P\x81a<\xEAa<\xE4_a\x19\x8CV[\x91a\x04\xA2V[\x11a;`V[_a=\x04\x91a<\xFDa\x16\xFFV[P\x01a*\xE6V[\x90V[a=\x1Ba=\x16a= \x92a\t\xA7V[a\x08\x90V[a\x04\xA2V[\x90V[a=,\x90a5lV[\x90RV[\x91` a=Q\x92\x94\x93a=J`@\x82\x01\x96_\x83\x01\x90a=#V[\x01\x90a\x05+V[V[a=ga=ba=l\x92a\x04\xA2V[a\x08\x90V[a\t\xA7V[\x90V[a=wa\x1CSV[P\x80a=\x8Fa=\x89c\xFF\xFF\xFF\xFFa=\x07V[\x91a\x04\xA2V[\x11a=\xA0Wa=\x9D\x90a=SV[\x90V[` a=\xBC_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a=0V[\x03\x90\xFD[a=\xD7a=\xDC\x91a=\xCFa\x16\xFFV[P`\x08a\x1ClV[a\x17\x1CV[\x90V[\x90V[a=\xF6a=\xF1a=\xFB\x92a=\xDFV[a\x0F\xF7V[a\x05\xF2V[\x90V[a>\x08`\xFFa=\xE2V[\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a>.a>'\x83a\x15\x87V[\x80\x94a\x15\xB1V[\x91`\x01\x81\x16\x90\x81_\x14a>\x85WP`\x01\x14a>IW[PPPV[a>V\x91\x92\x93\x94Pa>\x0BV[\x91_\x92[\x81\x84\x10a>mWPP\x01\x90_\x80\x80a>DV[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a>ZV[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a>DV[\x90a>\xAA\x91a>\x14V[\x90V[\x90a>\xCDa>\xC6\x92a>\xBDa\x03\x02V[\x93\x84\x80\x92a>\xA0V[\x03\x83a\x16pV[V[a>\xD8\x90a>\xADV[\x90V[\x90a>\xE4a\x15nV[Pa>\xEE\x82a\x17rV[a?\x07a?\x01a>\xFCa=\xFEV[a\x05\xF2V[\x91a\x05\xF2V[\x14\x15_\x14a?\x1CWPa?\x19\x90aE\x03V[\x90V[a?&\x91Pa>\xCFV[\x90V[`B\x91a?4a\x17nV[P`@Q\x91\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[_\x90V[a?za?\x7F\x91a\x17\x03V[a\x1E\xD4V[\x90V[\x90V[a?\x99a?\x94a?\x9E\x92a?\x82V[a\x08\x90V[a\x04\xA2V[\x90V[a?\xD6a?\xDD\x94a?\xCC``\x94\x98\x97\x95a?\xC2`\x80\x86\x01\x9A_\x87\x01\x90a\x066V[` \x85\x01\x90a\x06\xF9V[`@\x83\x01\x90a\x066V[\x01\x90a\x066V[V[a?\xE7a\x03\x02V[=_\x82>=\x90\xFD[\x93\x92\x93a?\xFAa\x1B\xDBV[Pa@\x03a?jV[Pa@\x0Ca\x17nV[Pa@\x16\x85a?nV[a@Ha@B\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0a?\x85V[\x91a\x04\xA2V[\x11a@\xD5W\x90a@k` \x94\x95_\x94\x93\x92\x93a@ba\x03\x02V[\x94\x85\x94\x85a?\xA1V[\x83\x80R\x03\x90`\x01Z\xFA\x15a@\xD0Wa@\x83_Qa\x0F\xF7V[\x80a@\x9Ea@\x98a@\x93_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14a@\xB4W_\x91a@\xAE_a\x0F\xFCV[\x91\x92\x91\x90V[Pa@\xBE_a\x19\x80V[`\x01\x91a@\xCA_a\x0F\xFCV[\x91\x92\x91\x90V[a?\xDFV[PPPa@\xE1_a\x19\x80V[\x90`\x03\x92\x91\x92\x91\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x04\x11\x15aA\tWV[a@\xEBV[\x90aA\x18\x82a@\xFFV[V[\x80aA-aA'_aA\x0EV[\x91aA\x0EV[\x14_\x14aA8WPPV[\x80aALaAF`\x01aA\x0EV[\x91aA\x0EV[\x14_\x14aAoW_c\xF6E\xEE\xDF`\xE0\x1B\x81R\x80aAk`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x80aA\x83aA}`\x02aA\x0EV[\x91aA\x0EV[\x14_\x14aA\xB1WaA\xADaA\x96\x83a?nV[_\x91\x82\x91c\xFC\xE6\x98\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x058V[\x03\x90\xFD[aA\xC4aA\xBE`\x03aA\x0EV[\x91aA\x0EV[\x14aA\xCCWPV[aA\xE7\x90_\x91\x82\x91c5\xE2\xF3\x83`\xE2\x1B\x83R`\x04\x83\x01a\x06CV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_R` _ \x90V[aB\x11\x81a*\xE6V[\x82\x10\x15aB+WaB#`\x01\x91aA\xFFV[\x91\x02\x01\x90_\x90V[aA\xEBV[\x90aB:\x90a\x0E\xB1V[\x90RV[\x90aBH\x90a\x14\nV[\x90RV[\x90aB\x82aBy_aB\\a%\xE8V[\x94aBsaBk\x83\x83\x01a++V[\x83\x88\x01aB0V[\x01a+|V[` \x84\x01aB>V[V[aB\x8D\x90aBLV[\x90V[aB\xAE\x91_aB\xA8\x92aB\xA1a&#V[P\x01aB\x08V[PaB\x84V[\x90V[\x92\x91aB\xBF\x84\x83\x83\x91aE3V[\x83aB\xDAaB\xD4aB\xCF_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14aB\xEFW[aB\xED\x92\x93\x91\x90\x91aF\xBDV[V[aB\xF7a\x17)V[\x93aC\0aF\xA2V[\x94\x80aC\x14aC\x0E\x88a\x04\xA2V[\x91a\x04\xA2V[\x11aC!WP\x93PaB\xE0V[\x85\x90aC=_\x92\x83\x92c\x0EX\xAE\x93`\xE1\x1B\x84R`\x04\x84\x01a\x1F\x13V[\x03\x90\xFD[aCIa\x16\xFFV[P\x15\x15\x90V[aCcaC^aCh\x92a6^V[a\x08\x90V[a\x04\xA2V[\x90V[aCwaC}\x91a\x04\xA2V[\x91a\x04\xA2V[\x90\x81\x15aC\x88W\x04\x90V[a6\xE2V[aC\xB2aC\xB8\x92aC\x9Ca\x16\xFFV[P\x82\x81\x16\x92\x18aC\xAC`\x02aCOV[\x90aCkV[\x90a\x19\xA8V[\x90V[\x90V[aC\xD2aC\xCDaC\xD7\x92aC\xBBV[a\x08\x90V[a\x06\xF3V[\x90V[aC\xE3\x90aC\xBEV[\x90RV[\x91` aD\x08\x92\x94\x93aD\x01`@\x82\x01\x96_\x83\x01\x90aC\xDAV[\x01\x90a\x05+V[V[aD\x1EaD\x19aD#\x92a\x04\xA2V[a\x08\x90V[a\x14\nV[\x90V[aD.a\x14\x95V[P\x80aDHaDB`\x01\x80`\xD0\x1B\x03a\x18\xF2V[\x91a\x04\xA2V[\x11aDYWaDV\x90aD\nV[\x90V[`\xD0aDu_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01aC\xE7V[\x03\x90\xFD[\x90aD\xAFaD\xB5\x93\x92aD\x8Aa\x14\x95V[PaD\x93a\x14\x95V[P\x80\x93aD\xA8aD\xA1a\"\x06V[\x94\x92a/.V[\x90\x91aK8V[\x91aG|V[\x91\x90\x91\x90V[aD\xCFaD\xCAaD\xD4\x92a5iV[a\x08\x90V[a\x04\xA2V[\x90V[6\x907V[\x90aE\x01aD\xE9\x83a\x1B#V[\x92` \x80aD\xF7\x86\x93a\x1B\0V[\x92\x01\x91\x03\x90aD\xD7V[V[aE\x0Ba\x15nV[PaE\x15\x81aG\xE6V[\x90aE(aE#` aD\xBBV[aD\xDCV[\x91\x82R` \x82\x01R\x90V[\x91\x90\x91\x80aEQaEKaEF_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14_\x14aF2WaEuaEn\x83aEi`\x02a\x17\x1CV[a\x19\xA8V[`\x02a\x1E\xF3V[[\x82aE\x91aE\x8BaE\x86_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14_\x14aF\x06WaE\xB5aE\xAE\x83aE\xA9`\x02a\x17\x1CV[a&\xB9V[`\x02a\x1E\xF3V[[\x91\x90\x91aF\x01aE\xEFaE\xE9\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93a\x18\xCDV[\x93a\x18\xCDV[\x93aE\xF8a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xA3V[aF-\x82aF'aF\x18_\x87\x90a\x1ClV[\x91aF\"\x83a\x17\x1CV[a7\x18V[\x90a\x1E\xF3V[aE\xB6V[aFEaF@_\x83\x90a\x1ClV[a\x17\x1CV[\x80aFXaFR\x85a\x04\xA2V[\x91a\x04\xA2V[\x10aF\x80WaFkaF{\x91\x84\x90a&\xB9V[aFv_\x84\x90a\x1ClV[a\x1E\xF3V[aEvV[\x90aF\x9E\x90\x91\x92_\x93\x84\x93c9\x144\xE3`\xE2\x1B\x85R`\x04\x85\x01a&\x87V[\x03\x90\xFD[aF\xAAa\x16\xFFV[PaF\xBA`\x01\x80`\xD0\x1B\x03a\x18\xF2V[\x90V[\x91aG\x15aG\x0FaG\x1C\x94\x80aF\xE3aF\xDDaF\xD8_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14aGMW[\x84aG\x04aF\xFEaF\xF9_a\x19\x80V[a\x04sV[\x91a\x04sV[\x14aG\x1EW[a\x1C!V[\x92a\x1C!V[\x90\x91a;BV[V[aGF`\x0B`\x02aG@aG:aG4\x89aD&V[\x93a\x18\xEFV[\x91a;?V[\x90aDyV[PPaG\nV[aGu`\x0B`\x01aGoaGiaGc\x89aD&V[\x93a\x18\xEFV[\x91a;?V[\x90aDyV[PPaF\xE9V[\x91aG\xA0_aG\xA5\x94aG\x8Da\x14\x95V[PaG\x96a\x14\x95V[P\x01\x92\x91\x92a+\tV[aI\xEAV[\x91\x90\x91\x90V[aG\xBFaG\xBAaG\xC4\x92a=\xDFV[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[aG\xDEaG\xD9aG\xE3\x92aG\xC7V[a\x08\x90V[a\x04\xA2V[\x90V[aG\xFBaH\0\x91aG\xF5a\x16\xFFV[Pa\x17rV[a?nV[aH\n`\xFFaG\xABV[\x16\x80aH\x1FaH\x19`\x1FaG\xCAV[\x91a\x04\xA2V[\x11aH'W\x90V[_c,\xD4J\xC3`\xE2\x1B\x81R\x80aH?`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[T\x90V[aHQ`@a\x1A\xEBV[\x90V[_R` _ \x90V[aHf\x81aHCV[\x82\x10\x15aH\x80WaHx`\x01\x91aHTV[\x91\x02\x01\x90_\x90V[aA\xEBV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[aH\xA2\x90Qa\x0E\xB1V[\x90V[\x90aH\xB6e\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x0F\xF7V[\x91\x81\x19\x16\x91\x16\x17\x90V[aH\xD4aH\xCFaH\xD9\x92a\x0E\xB1V[a\x08\x90V[a\x0E\xB1V[\x90V[\x90V[\x90aH\xF4aH\xEFaH\xFB\x92aH\xC0V[aH\xDCV[\x82TaH\xA5V[\x90UV[aI\t\x90Qa\x14\nV[\x90V[`0\x1B\x90V[\x90aI$e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91aI\x0CV[\x91\x81\x19\x16\x91\x16\x17\x90V[aIBaI=aIG\x92a\x14\nV[a\x08\x90V[a\x14\nV[\x90V[\x90V[\x90aIbaI]aIi\x92aI.V[aIJV[\x82TaI\x12V[\x90UV[\x90aI\x97` _aI\x9D\x94aI\x8F\x82\x82\x01aI\x89\x84\x88\x01aH\x98V[\x90aH\xDFV[\x01\x92\x01aH\xFFV[\x90aIMV[V[\x91\x90aI\xB0WaI\xAE\x91aImV[V[aH\x85V[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15aI\xE5W\x82aI\xDD\x91`\x01aI\xE3\x95\x01\x81UaH]V[\x90aI\x9FV[V[a\x16\\V[\x90\x92\x91\x92aI\xF6a\x14\x95V[PaI\xFFa\x14\x95V[PaJ\t\x82aHCV[\x80aJ\x1CaJ\x16_a\x19\x8CV[\x91a\x04\xA2V[\x11_\x14aJ\xECWaJB\x90aJ<\x84\x91aJ6`\x01a+;V[\x90a\x1A\xA1V[\x90a:\x91V[\x90aJN_\x83\x01a++V[\x92aJZ_\x84\x01a+|V[\x93\x80aJnaJh\x85a\x0E\xB1V[\x91a\x0E\xB1V[\x11aJ\xD0WaJ\x85aJ\x7F\x84a\x0E\xB1V[\x91a\x0E\xB1V[\x14_\x14aJ\xA0WPPaJ\x9B\x90_\x85\x91\x01aIMV[[\x91\x90V[aJ\xCB\x92PaJ\xC6\x86aJ\xBDaJ\xB4aHGV[\x94_\x86\x01aB0V[` \x84\x01aB>V[aI\xB5V[aJ\x9CV[_c% `\x1D`\xE0\x1B\x81R\x80aJ\xE8`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[PaK\x17\x91aK\x12\x85aK\taK\0aHGV[\x94_\x86\x01aB0V[` \x84\x01aB>V[aI\xB5V[aK _a+\x89V[\x91\x90V[cNH{q`\xE0\x1B_R`Q`\x04R`$_\xFD[\x91\x90\x91\x80`\x01\x14aKWW`\x02\x03aK$WaKS\x91a\x15\x14V[\x90[V[PaKa\x91a\x14\xD5V[\x90aKUV",
    );
    /**Custom error with signature `AccessControlBadConfirmation()` and selector `0x6697b232`.
```solidity
error AccessControlBadConfirmation();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccessControlBadConfirmation {}
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
                Self {}
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
        }
    };
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
        }
    };
    /**Custom error with signature `BurnOnlyDuringLockPeriod()` and selector `0xb8b5ca2d`.
```solidity
error BurnOnlyDuringLockPeriod();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BurnOnlyDuringLockPeriod {}
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
                Self {}
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
        }
    };
    /**Custom error with signature `CheckpointUnorderedInsertion()` and selector `0x2520601d`.
```solidity
error CheckpointUnorderedInsertion();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CheckpointUnorderedInsertion {}
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
                Self {}
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
        }
    };
    /**Custom error with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`.
```solidity
error ECDSAInvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignature {}
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
                Self {}
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
        }
    };
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
        }
    };
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
        }
    };
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
        }
    };
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
        }
    };
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
        }
    };
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
        }
    };
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
        }
    };
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
        }
    };
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
        }
    };
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
        }
    };
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
        }
    };
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
        }
    };
    /**Custom error with signature `ERC6372InconsistentClock()` and selector `0x6ff07140`.
```solidity
error ERC6372InconsistentClock();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC6372InconsistentClock {}
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
                Self {}
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
        }
    };
    /**Custom error with signature `ExceedsTotalSupply()` and selector `0x177e3fc3`.
```solidity
error ExceedsTotalSupply();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExceedsTotalSupply {}
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
                Self {}
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
        }
    };
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
        }
    };
    /**Custom error with signature `InvalidShortString()` and selector `0xb3512b0c`.
```solidity
error InvalidShortString();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidShortString {}
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
                Self {}
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
        }
    };
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
        }
    };
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
        }
    };
    /**Custom error with signature `TransfersLocked()` and selector `0xdb89e3f4`.
```solidity
error TransfersLocked();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TransfersLocked {}
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
                Self {}
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
        }
    };
    /**Custom error with signature `UnlockTimestampAlreadySet()` and selector `0x6fd2f1a6`.
```solidity
error UnlockTimestampAlreadySet();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnlockTimestampAlreadySet {}
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
        impl ::core::convert::From<UnlockTimestampAlreadySet>
        for UnderlyingRustTuple<'_> {
            fn from(value: UnlockTimestampAlreadySet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UnlockTimestampAlreadySet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnlockTimestampAlreadySet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnlockTimestampAlreadySet()";
            const SELECTOR: [u8; 4] = [111u8, 210u8, 241u8, 166u8];
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
        }
    };
    /**Custom error with signature `UnlockTimestampInPast()` and selector `0xa5658353`.
```solidity
error UnlockTimestampInPast();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnlockTimestampInPast {}
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
                Self {}
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
        }
    };
    /**Custom error with signature `UnlockTimestampTooLate()` and selector `0xef69af65`.
```solidity
error UnlockTimestampTooLate();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnlockTimestampTooLate {}
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
                Self {}
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
        }
    };
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
        }
    };
    /**Custom error with signature `ZeroAddress()` and selector `0xd92e233d`.
```solidity
error ZeroAddress();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroAddress {}
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
                Self {}
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
        }
    };
    /**Custom error with signature `ZeroAmount()` and selector `0x1f2a2005`.
```solidity
error ZeroAmount();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroAmount {}
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
                Self {}
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
        }
    };
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
                140u8,
                91u8,
                225u8,
                229u8,
                235u8,
                236u8,
                125u8,
                91u8,
                209u8,
                79u8,
                113u8,
                66u8,
                125u8,
                30u8,
                132u8,
                243u8,
                221u8,
                3u8,
                20u8,
                192u8,
                247u8,
                178u8,
                41u8,
                30u8,
                91u8,
                32u8,
                10u8,
                200u8,
                199u8,
                195u8,
                185u8,
                37u8,
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
                49u8,
                52u8,
                232u8,
                162u8,
                230u8,
                217u8,
                126u8,
                146u8,
                154u8,
                126u8,
                84u8,
                1u8,
                30u8,
                165u8,
                72u8,
                93u8,
                125u8,
                25u8,
                109u8,
                213u8,
                240u8,
                186u8,
                77u8,
                78u8,
                249u8,
                88u8,
                3u8,
                232u8,
                227u8,
                252u8,
                37u8,
                127u8,
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
                222u8,
                194u8,
                186u8,
                205u8,
                210u8,
                240u8,
                91u8,
                89u8,
                222u8,
                52u8,
                218u8,
                155u8,
                82u8,
                61u8,
                255u8,
                139u8,
                228u8,
                46u8,
                94u8,
                56u8,
                232u8,
                24u8,
                200u8,
                47u8,
                219u8,
                11u8,
                174u8,
                119u8,
                67u8,
                135u8,
                167u8,
                36u8,
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
    pub struct EIP712DomainChanged {}
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
                10u8,
                99u8,
                135u8,
                201u8,
                234u8,
                54u8,
                40u8,
                184u8,
                138u8,
                99u8,
                59u8,
                180u8,
                243u8,
                177u8,
                81u8,
                119u8,
                15u8,
                112u8,
                8u8,
                81u8,
                23u8,
                161u8,
                95u8,
                155u8,
                243u8,
                120u8,
                124u8,
                218u8,
                83u8,
                241u8,
                61u8,
                49u8,
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
                189u8,
                121u8,
                184u8,
                111u8,
                254u8,
                10u8,
                184u8,
                232u8,
                119u8,
                97u8,
                81u8,
                81u8,
                66u8,
                23u8,
                205u8,
                124u8,
                172u8,
                213u8,
                44u8,
                144u8,
                159u8,
                102u8,
                71u8,
                92u8,
                58u8,
                244u8,
                78u8,
                18u8,
                159u8,
                11u8,
                0u8,
                255u8,
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
                47u8,
                135u8,
                136u8,
                17u8,
                126u8,
                126u8,
                255u8,
                29u8,
                130u8,
                233u8,
                38u8,
                236u8,
                121u8,
                73u8,
                1u8,
                209u8,
                124u8,
                120u8,
                2u8,
                74u8,
                80u8,
                39u8,
                9u8,
                64u8,
                48u8,
                69u8,
                64u8,
                167u8,
                51u8,
                101u8,
                111u8,
                13u8,
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
                246u8,
                57u8,
                31u8,
                92u8,
                50u8,
                217u8,
                198u8,
                157u8,
                42u8,
                71u8,
                234u8,
                103u8,
                11u8,
                68u8,
                41u8,
                116u8,
                181u8,
                57u8,
                53u8,
                209u8,
                237u8,
                199u8,
                253u8,
                100u8,
                235u8,
                33u8,
                224u8,
                71u8,
                168u8,
                57u8,
                23u8,
                27u8,
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
                190u8,
                244u8,
                248u8,
                28u8,
                24u8,
                20u8,
                198u8,
                65u8,
                237u8,
                232u8,
                94u8,
                186u8,
                172u8,
                241u8,
                157u8,
                4u8,
                139u8,
                44u8,
                91u8,
                85u8,
                152u8,
                10u8,
                223u8,
                166u8,
                239u8,
                15u8,
                149u8,
                108u8,
                101u8,
                19u8,
                53u8,
                162u8,
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
                221u8,
                242u8,
                82u8,
                173u8,
                27u8,
                226u8,
                200u8,
                155u8,
                105u8,
                194u8,
                176u8,
                104u8,
                252u8,
                55u8,
                141u8,
                170u8,
                149u8,
                43u8,
                167u8,
                241u8,
                99u8,
                196u8,
                161u8,
                22u8,
                40u8,
                245u8,
                90u8,
                77u8,
                245u8,
                35u8,
                179u8,
                239u8,
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
                221u8,
                104u8,
                150u8,
                220u8,
                241u8,
                212u8,
                179u8,
                17u8,
                204u8,
                168u8,
                125u8,
                209u8,
                155u8,
                187u8,
                162u8,
                234u8,
                156u8,
                226u8,
                248u8,
                103u8,
                193u8,
                86u8,
                136u8,
                120u8,
                160u8,
                67u8,
                138u8,
                102u8,
                161u8,
                175u8,
                238u8,
                236u8,
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
    /**Function with signature `AIRDROP_MANAGER_ROLE()` and selector `0x8a542521`.
```solidity
function AIRDROP_MANAGER_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AIRDROP_MANAGER_ROLECall {}
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
                    Self {}
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
            type Return = AIRDROP_MANAGER_ROLEReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `CLOCK_MODE()` and selector `0x4bf5d7e9`.
```solidity
function CLOCK_MODE() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLOCK_MODECall {}
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
                    Self {}
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
            type Return = CLOCK_MODEReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`.
```solidity
function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLECall {}
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
                    Self {}
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
            type Return = DEFAULT_ADMIN_ROLEReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`.
```solidity
function DOMAIN_SEPARATOR() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DOMAIN_SEPARATORCall {}
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
                    Self {}
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
            type Return = DOMAIN_SEPARATORReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `EMISSION_MINTER_ROLE()` and selector `0x8d3343d6`.
```solidity
function EMISSION_MINTER_ROLE() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EMISSION_MINTER_ROLECall {}
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
                    Self {}
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
            type Return = EMISSION_MINTER_ROLEReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `INITIAL_MINT_SUPPLY()` and selector `0x9b7ef64b`.
```solidity
function INITIAL_MINT_SUPPLY() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct INITIAL_MINT_SUPPLYCall {}
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
                    Self {}
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
            type Return = INITIAL_MINT_SUPPLYReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `MAX_LOCK_DURATION()` and selector `0x4f1bfc9e`.
```solidity
function MAX_LOCK_DURATION() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_LOCK_DURATIONCall {}
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
                    Self {}
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
            type Return = MAX_LOCK_DURATIONReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `TOTAL_SUPPLY()` and selector `0x902d55a5`.
```solidity
function TOTAL_SUPPLY() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TOTAL_SUPPLYCall {}
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
                    Self {}
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
            type Return = TOTAL_SUPPLYReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            type Return = allowanceReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            type Return = approveReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            type Return = balanceOfReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            type Return = checkpointsReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `clock()` and selector `0x91ddadf4`.
```solidity
function clock() external view returns (uint48);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clockCall {}
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
                    Self {}
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
            type Return = clockReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `decimals()` and selector `0x313ce567`.
```solidity
function decimals() external view returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decimalsCall {}
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
                    Self {}
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
            type Return = decimalsReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            type Return = delegatesReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `eip712Domain()` and selector `0x84b0196e`.
```solidity
function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainCall {}
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
                    Self {}
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getCurrentTotalSupply()` and selector `0xc02ae754`.
```solidity
function getCurrentTotalSupply() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentTotalSupplyCall {}
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
                    Self {}
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
            type Return = getCurrentTotalSupplyReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            type Return = getPastTotalSupplyReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            type Return = getPastVotesReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            type Return = getPastVotingPowerReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getRemainingEmissions()` and selector `0x4bdd36ce`.
```solidity
function getRemainingEmissions() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRemainingEmissionsCall {}
    ///Container type for the return parameters of the [`getRemainingEmissions()`](getRemainingEmissionsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRemainingEmissionsReturn {
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
            impl ::core::convert::From<getRemainingEmissionsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRemainingEmissionsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRemainingEmissionsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getRemainingEmissionsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRemainingEmissionsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRemainingEmissionsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRemainingEmissionsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRemainingEmissionsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRemainingEmissions()";
            const SELECTOR: [u8; 4] = [75u8, 221u8, 54u8, 206u8];
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getRemainingLockTime()` and selector `0x7a8cd156`.
```solidity
function getRemainingLockTime() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRemainingLockTimeCall {}
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
                    Self {}
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
            type Return = getRemainingLockTimeReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            type Return = getRoleAdminReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            type Return = getVotesReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            type Return = getVotingPowerReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            type Return = hasRoleReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `maxLockTimestamp()` and selector `0x8426adf2`.
```solidity
function maxLockTimestamp() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxLockTimestampCall {}
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
                    Self {}
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
            type Return = maxLockTimestampReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `name()` and selector `0x06fdde03`.
```solidity
function name() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nameCall {}
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
                    Self {}
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
            type Return = nameReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            type Return = noncesReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            type Return = numCheckpointsReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            type Return = supportsInterfaceReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `symbol()` and selector `0x95d89b41`.
```solidity
function symbol() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct symbolCall {}
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
                    Self {}
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
            type Return = symbolReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `totalSupply()` and selector `0x18160ddd`.
```solidity
function totalSupply() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalSupplyCall {}
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
                    Self {}
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
            type Return = totalSupplyReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            type Return = transferReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            type Return = transferFromReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `transfersLocked()` and selector `0x83f1211b`.
```solidity
function transfersLocked() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transfersLockedCall {}
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
                    Self {}
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
            type Return = transfersLockedReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `unlockTimestamp()` and selector `0xaa082a9d`.
```solidity
function unlockTimestamp() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unlockTimestampCall {}
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
                    Self {}
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
            type Return = unlockTimestampReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`SyndicateToken`](self) function calls.
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
        getRemainingEmissions(getRemainingEmissionsCall),
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
            [75u8, 221u8, 54u8, 206u8],
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
                Self::getRemainingEmissions(_) => {
                    <getRemainingEmissionsCall as alloy_sol_types::SolCall>::SELECTOR
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
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<SyndicateTokenCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn name(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <nameCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::name)
                    }
                    name
                },
                {
                    fn approve(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <approveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::approve)
                    }
                    approve
                },
                {
                    fn totalSupply(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <totalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::totalSupply)
                    }
                    totalSupply
                },
                {
                    fn transferFrom(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <transferFromCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::transferFrom)
                    }
                    transferFrom
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn grantRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn decimals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <decimalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::decimals)
                    }
                    decimals
                },
                {
                    fn DOMAIN_SEPARATOR(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::DOMAIN_SEPARATOR)
                    }
                    DOMAIN_SEPARATOR
                },
                {
                    fn renounceRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn getPastVotes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getPastVotesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::getPastVotes)
                    }
                    getPastVotes
                },
                {
                    fn mint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <mintCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::mint)
                    }
                    mint
                },
                {
                    fn getRemainingEmissions(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getRemainingEmissionsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::getRemainingEmissions)
                    }
                    getRemainingEmissions
                },
                {
                    fn CLOCK_MODE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::CLOCK_MODE)
                    }
                    CLOCK_MODE
                },
                {
                    fn MAX_LOCK_DURATION(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <MAX_LOCK_DURATIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::MAX_LOCK_DURATION)
                    }
                    MAX_LOCK_DURATION
                },
                {
                    fn delegates(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <delegatesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::delegates)
                    }
                    delegates
                },
                {
                    fn delegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <delegateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::delegate)
                    }
                    delegate
                },
                {
                    fn numCheckpoints(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <numCheckpointsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::numCheckpoints)
                    }
                    numCheckpoints
                },
                {
                    fn balanceOf(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <balanceOfCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::balanceOf)
                    }
                    balanceOf
                },
                {
                    fn burnFrom(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <burnFromCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::burnFrom)
                    }
                    burnFrom
                },
                {
                    fn getRemainingLockTime(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getRemainingLockTimeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::getRemainingLockTime)
                    }
                    getRemainingLockTime
                },
                {
                    fn nonces(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <noncesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::nonces)
                    }
                    nonces
                },
                {
                    fn transfersLocked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <transfersLockedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::transfersLocked)
                    }
                    transfersLocked
                },
                {
                    fn maxLockTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <maxLockTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::maxLockTimestamp)
                    }
                    maxLockTimestamp
                },
                {
                    fn setUnlockTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <setUnlockTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::setUnlockTimestamp)
                    }
                    setUnlockTimestamp
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn AIRDROP_MANAGER_ROLE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <AIRDROP_MANAGER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::AIRDROP_MANAGER_ROLE)
                    }
                    AIRDROP_MANAGER_ROLE
                },
                {
                    fn EMISSION_MINTER_ROLE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <EMISSION_MINTER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::EMISSION_MINTER_ROLE)
                    }
                    EMISSION_MINTER_ROLE
                },
                {
                    fn getPastTotalSupply(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getPastTotalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::getPastTotalSupply)
                    }
                    getPastTotalSupply
                },
                {
                    fn TOTAL_SUPPLY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <TOTAL_SUPPLYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::TOTAL_SUPPLY)
                    }
                    TOTAL_SUPPLY
                },
                {
                    fn hasRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn clock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <clockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::clock)
                    }
                    clock
                },
                {
                    fn symbol(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <symbolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::symbol)
                    }
                    symbol
                },
                {
                    fn getVotes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getVotesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::getVotes)
                    }
                    getVotes
                },
                {
                    fn INITIAL_MINT_SUPPLY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <INITIAL_MINT_SUPPLYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::INITIAL_MINT_SUPPLY)
                    }
                    INITIAL_MINT_SUPPLY
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn transfer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <transferCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::transfer)
                    }
                    transfer
                },
                {
                    fn unlockTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <unlockTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::unlockTimestamp)
                    }
                    unlockTimestamp
                },
                {
                    fn getPastVotingPower(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getPastVotingPowerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::getPastVotingPower)
                    }
                    getPastVotingPower
                },
                {
                    fn getVotingPower(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getVotingPowerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::getVotingPower)
                    }
                    getVotingPower
                },
                {
                    fn getCurrentTotalSupply(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getCurrentTotalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::getCurrentTotalSupply)
                    }
                    getCurrentTotalSupply
                },
                {
                    fn delegateBySig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <delegateBySigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::delegateBySig)
                    }
                    delegateBySig
                },
                {
                    fn permit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <permitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::permit)
                    }
                    permit
                },
                {
                    fn revokeRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn allowance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <allowanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::allowance)
                    }
                    allowance
                },
                {
                    fn checkpoints(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <checkpointsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
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
            DECODE_SHIMS[idx](data, validate)
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
                Self::getRemainingEmissions(inner) => {
                    <getRemainingEmissionsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getRemainingEmissions(inner) => {
                    <getRemainingEmissionsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        UnlockTimestampAlreadySet(UnlockTimestampAlreadySet),
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
            [111u8, 210u8, 241u8, 166u8],
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
        const COUNT: usize = 30usize;
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
                Self::UnlockTimestampAlreadySet(_) => {
                    <UnlockTimestampAlreadySet as alloy_sol_types::SolError>::SELECTOR
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
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<SyndicateTokenErrors>] = &[
                {
                    fn ExceedsTotalSupply(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ExceedsTotalSupply as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::ExceedsTotalSupply)
                    }
                    ExceedsTotalSupply
                },
                {
                    fn ERC20ExceededSafeSupply(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20ExceededSafeSupply as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::ERC20ExceededSafeSupply)
                    }
                    ERC20ExceededSafeSupply
                },
                {
                    fn ZeroAmount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ZeroAmount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::ZeroAmount)
                    }
                    ZeroAmount
                },
                {
                    fn CheckpointUnorderedInsertion(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::CheckpointUnorderedInsertion)
                    }
                    CheckpointUnorderedInsertion
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn VotesExpiredSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <VotesExpiredSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::VotesExpiredSignature)
                    }
                    VotesExpiredSignature
                },
                {
                    fn ERC2612InvalidSigner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC2612InvalidSigner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::ERC2612InvalidSigner)
                    }
                    ERC2612InvalidSigner
                },
                {
                    fn ERC2612ExpiredSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC2612ExpiredSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::ERC2612ExpiredSignature)
                    }
                    ERC2612ExpiredSignature
                },
                {
                    fn AccessControlBadConfirmation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::AccessControlBadConfirmation)
                    }
                    AccessControlBadConfirmation
                },
                {
                    fn SafeCastOverflowedUintDowncast(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::SafeCastOverflowedUintDowncast)
                    }
                    SafeCastOverflowedUintDowncast
                },
                {
                    fn UnlockTimestampAlreadySet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <UnlockTimestampAlreadySet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::UnlockTimestampAlreadySet)
                    }
                    UnlockTimestampAlreadySet
                },
                {
                    fn ERC6372InconsistentClock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC6372InconsistentClock as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::ERC6372InconsistentClock)
                    }
                    ERC6372InconsistentClock
                },
                {
                    fn InvalidAccountNonce(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <InvalidAccountNonce as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::InvalidAccountNonce)
                    }
                    InvalidAccountNonce
                },
                {
                    fn ERC20InvalidSpender(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20InvalidSpender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::ERC20InvalidSpender)
                    }
                    ERC20InvalidSpender
                },
                {
                    fn ERC20InvalidSender(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20InvalidSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::ERC20InvalidSender)
                    }
                    ERC20InvalidSender
                },
                {
                    fn UnlockTimestampInPast(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <UnlockTimestampInPast as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::UnlockTimestampInPast)
                    }
                    UnlockTimestampInPast
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn BurnOnlyDuringLockPeriod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <BurnOnlyDuringLockPeriod as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::BurnOnlyDuringLockPeriod)
                    }
                    BurnOnlyDuringLockPeriod
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn ZeroAddress(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ZeroAddress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::ZeroAddress)
                    }
                    ZeroAddress
                },
                {
                    fn TransfersLocked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <TransfersLocked as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::TransfersLocked)
                    }
                    TransfersLocked
                },
                {
                    fn AccessControlUnauthorizedAccount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::AccessControlUnauthorizedAccount)
                    }
                    AccessControlUnauthorizedAccount
                },
                {
                    fn ERC20InsufficientBalance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20InsufficientBalance as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::ERC20InsufficientBalance)
                    }
                    ERC20InsufficientBalance
                },
                {
                    fn ERC20InvalidApprover(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20InvalidApprover as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::ERC20InvalidApprover)
                    }
                    ERC20InvalidApprover
                },
                {
                    fn ERC20InvalidReceiver(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20InvalidReceiver as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::ERC20InvalidReceiver)
                    }
                    ERC20InvalidReceiver
                },
                {
                    fn ERC5805FutureLookup(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC5805FutureLookup as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::ERC5805FutureLookup)
                    }
                    ERC5805FutureLookup
                },
                {
                    fn UnlockTimestampTooLate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <UnlockTimestampTooLate as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::UnlockTimestampTooLate)
                    }
                    UnlockTimestampTooLate
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn ERC20InsufficientAllowance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ERC20InsufficientAllowance as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenErrors::ERC20InsufficientAllowance)
                    }
                    ERC20InsufficientAllowance
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
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
            DECODE_SHIMS[idx](data, validate)
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
                Self::UnlockTimestampAlreadySet(inner) => {
                    <UnlockTimestampAlreadySet as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::UnlockTimestampAlreadySet(inner) => {
                    <UnlockTimestampAlreadySet as alloy_sol_types::SolError>::abi_encode_raw(
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
                10u8,
                99u8,
                135u8,
                201u8,
                234u8,
                54u8,
                40u8,
                184u8,
                138u8,
                99u8,
                59u8,
                180u8,
                243u8,
                177u8,
                81u8,
                119u8,
                15u8,
                112u8,
                8u8,
                81u8,
                23u8,
                161u8,
                95u8,
                155u8,
                243u8,
                120u8,
                124u8,
                218u8,
                83u8,
                241u8,
                61u8,
                49u8,
            ],
            [
                47u8,
                135u8,
                136u8,
                17u8,
                126u8,
                126u8,
                255u8,
                29u8,
                130u8,
                233u8,
                38u8,
                236u8,
                121u8,
                73u8,
                1u8,
                209u8,
                124u8,
                120u8,
                2u8,
                74u8,
                80u8,
                39u8,
                9u8,
                64u8,
                48u8,
                69u8,
                64u8,
                167u8,
                51u8,
                101u8,
                111u8,
                13u8,
            ],
            [
                49u8,
                52u8,
                232u8,
                162u8,
                230u8,
                217u8,
                126u8,
                146u8,
                154u8,
                126u8,
                84u8,
                1u8,
                30u8,
                165u8,
                72u8,
                93u8,
                125u8,
                25u8,
                109u8,
                213u8,
                240u8,
                186u8,
                77u8,
                78u8,
                249u8,
                88u8,
                3u8,
                232u8,
                227u8,
                252u8,
                37u8,
                127u8,
            ],
            [
                140u8,
                91u8,
                225u8,
                229u8,
                235u8,
                236u8,
                125u8,
                91u8,
                209u8,
                79u8,
                113u8,
                66u8,
                125u8,
                30u8,
                132u8,
                243u8,
                221u8,
                3u8,
                20u8,
                192u8,
                247u8,
                178u8,
                41u8,
                30u8,
                91u8,
                32u8,
                10u8,
                200u8,
                199u8,
                195u8,
                185u8,
                37u8,
            ],
            [
                189u8,
                121u8,
                184u8,
                111u8,
                254u8,
                10u8,
                184u8,
                232u8,
                119u8,
                97u8,
                81u8,
                81u8,
                66u8,
                23u8,
                205u8,
                124u8,
                172u8,
                213u8,
                44u8,
                144u8,
                159u8,
                102u8,
                71u8,
                92u8,
                58u8,
                244u8,
                78u8,
                18u8,
                159u8,
                11u8,
                0u8,
                255u8,
            ],
            [
                190u8,
                244u8,
                248u8,
                28u8,
                24u8,
                20u8,
                198u8,
                65u8,
                237u8,
                232u8,
                94u8,
                186u8,
                172u8,
                241u8,
                157u8,
                4u8,
                139u8,
                44u8,
                91u8,
                85u8,
                152u8,
                10u8,
                223u8,
                166u8,
                239u8,
                15u8,
                149u8,
                108u8,
                101u8,
                19u8,
                53u8,
                162u8,
            ],
            [
                221u8,
                104u8,
                150u8,
                220u8,
                241u8,
                212u8,
                179u8,
                17u8,
                204u8,
                168u8,
                125u8,
                209u8,
                155u8,
                187u8,
                162u8,
                234u8,
                156u8,
                226u8,
                248u8,
                103u8,
                193u8,
                86u8,
                136u8,
                120u8,
                160u8,
                67u8,
                138u8,
                102u8,
                161u8,
                175u8,
                238u8,
                236u8,
            ],
            [
                221u8,
                242u8,
                82u8,
                173u8,
                27u8,
                226u8,
                200u8,
                155u8,
                105u8,
                194u8,
                176u8,
                104u8,
                252u8,
                55u8,
                141u8,
                170u8,
                149u8,
                43u8,
                167u8,
                241u8,
                99u8,
                196u8,
                161u8,
                22u8,
                40u8,
                245u8,
                90u8,
                77u8,
                245u8,
                35u8,
                179u8,
                239u8,
            ],
            [
                222u8,
                194u8,
                186u8,
                205u8,
                210u8,
                240u8,
                91u8,
                89u8,
                222u8,
                52u8,
                218u8,
                155u8,
                82u8,
                61u8,
                255u8,
                139u8,
                228u8,
                46u8,
                94u8,
                56u8,
                232u8,
                24u8,
                200u8,
                47u8,
                219u8,
                11u8,
                174u8,
                119u8,
                67u8,
                135u8,
                167u8,
                36u8,
            ],
            [
                246u8,
                57u8,
                31u8,
                92u8,
                50u8,
                217u8,
                198u8,
                157u8,
                42u8,
                71u8,
                234u8,
                103u8,
                11u8,
                68u8,
                41u8,
                116u8,
                181u8,
                57u8,
                53u8,
                209u8,
                237u8,
                199u8,
                253u8,
                100u8,
                235u8,
                33u8,
                224u8,
                71u8,
                168u8,
                57u8,
                23u8,
                27u8,
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
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Approval as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Approval as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Approval)
                }
                Some(<DelegateChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <DelegateChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DelegateChanged)
                }
                Some(
                    <DelegateVotesChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DelegateVotesChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DelegateVotesChanged)
                }
                Some(
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::EIP712DomainChanged)
                }
                Some(<RoleAdminChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleAdminChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RoleAdminChanged)
                }
                Some(<RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleGranted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RoleGranted)
                }
                Some(<RoleRevoked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleRevoked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RoleRevoked)
                }
                Some(
                    <TokensBurnedByManager as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TokensBurnedByManager as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TokensBurnedByManager)
                }
                Some(<Transfer as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Transfer as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Transfer)
                }
                Some(
                    <UnlockTimestampUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UnlockTimestampUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
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
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SyndicateTokenInstance<T, P, N> {
        SyndicateTokenInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        defaultAdmin: alloy::sol_types::private::Address,
        syndTreasuryAddress: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<SyndicateTokenInstance<T, P, N>>,
    > {
        SyndicateTokenInstance::<
            T,
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
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        defaultAdmin: alloy::sol_types::private::Address,
        syndTreasuryAddress: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        SyndicateTokenInstance::<
            T,
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
    pub struct SyndicateTokenInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for SyndicateTokenInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SyndicateTokenInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SyndicateTokenInstance<T, P, N> {
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
                _network_transport: ::core::marker::PhantomData,
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
        ) -> alloy_contract::Result<SyndicateTokenInstance<T, P, N>> {
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
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
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
    impl<T, P: ::core::clone::Clone, N> SyndicateTokenInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SyndicateTokenInstance<T, P, N> {
            SyndicateTokenInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SyndicateTokenInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`AIRDROP_MANAGER_ROLE`] function.
        pub fn AIRDROP_MANAGER_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, AIRDROP_MANAGER_ROLECall, N> {
            self.call_builder(&AIRDROP_MANAGER_ROLECall {})
        }
        ///Creates a new call builder for the [`CLOCK_MODE`] function.
        pub fn CLOCK_MODE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, CLOCK_MODECall, N> {
            self.call_builder(&CLOCK_MODECall {})
        }
        ///Creates a new call builder for the [`DEFAULT_ADMIN_ROLE`] function.
        pub fn DEFAULT_ADMIN_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DEFAULT_ADMIN_ROLECall, N> {
            self.call_builder(&DEFAULT_ADMIN_ROLECall {})
        }
        ///Creates a new call builder for the [`DOMAIN_SEPARATOR`] function.
        pub fn DOMAIN_SEPARATOR(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DOMAIN_SEPARATORCall, N> {
            self.call_builder(&DOMAIN_SEPARATORCall {})
        }
        ///Creates a new call builder for the [`EMISSION_MINTER_ROLE`] function.
        pub fn EMISSION_MINTER_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, EMISSION_MINTER_ROLECall, N> {
            self.call_builder(&EMISSION_MINTER_ROLECall {})
        }
        ///Creates a new call builder for the [`INITIAL_MINT_SUPPLY`] function.
        pub fn INITIAL_MINT_SUPPLY(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, INITIAL_MINT_SUPPLYCall, N> {
            self.call_builder(&INITIAL_MINT_SUPPLYCall {})
        }
        ///Creates a new call builder for the [`MAX_LOCK_DURATION`] function.
        pub fn MAX_LOCK_DURATION(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, MAX_LOCK_DURATIONCall, N> {
            self.call_builder(&MAX_LOCK_DURATIONCall {})
        }
        ///Creates a new call builder for the [`TOTAL_SUPPLY`] function.
        pub fn TOTAL_SUPPLY(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, TOTAL_SUPPLYCall, N> {
            self.call_builder(&TOTAL_SUPPLYCall {})
        }
        ///Creates a new call builder for the [`allowance`] function.
        pub fn allowance(
            &self,
            owner: alloy::sol_types::private::Address,
            spender: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, allowanceCall, N> {
            self.call_builder(&allowanceCall { owner, spender })
        }
        ///Creates a new call builder for the [`approve`] function.
        pub fn approve(
            &self,
            spender: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, approveCall, N> {
            self.call_builder(&approveCall { spender, value })
        }
        ///Creates a new call builder for the [`balanceOf`] function.
        pub fn balanceOf(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, balanceOfCall, N> {
            self.call_builder(&balanceOfCall { account })
        }
        ///Creates a new call builder for the [`burnFrom`] function.
        pub fn burnFrom(
            &self,
            from: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, burnFromCall, N> {
            self.call_builder(&burnFromCall { from, amount })
        }
        ///Creates a new call builder for the [`checkpoints`] function.
        pub fn checkpoints(
            &self,
            account: alloy::sol_types::private::Address,
            pos: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkpointsCall, N> {
            self.call_builder(&checkpointsCall { account, pos })
        }
        ///Creates a new call builder for the [`clock`] function.
        pub fn clock(&self) -> alloy_contract::SolCallBuilder<T, &P, clockCall, N> {
            self.call_builder(&clockCall {})
        }
        ///Creates a new call builder for the [`decimals`] function.
        pub fn decimals(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, decimalsCall, N> {
            self.call_builder(&decimalsCall {})
        }
        ///Creates a new call builder for the [`delegate`] function.
        pub fn delegate(
            &self,
            delegatee: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegateCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, delegateBySigCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, delegatesCall, N> {
            self.call_builder(&delegatesCall { account })
        }
        ///Creates a new call builder for the [`eip712Domain`] function.
        pub fn eip712Domain(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eip712DomainCall, N> {
            self.call_builder(&eip712DomainCall {})
        }
        ///Creates a new call builder for the [`getCurrentTotalSupply`] function.
        pub fn getCurrentTotalSupply(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCurrentTotalSupplyCall, N> {
            self.call_builder(&getCurrentTotalSupplyCall {})
        }
        ///Creates a new call builder for the [`getPastTotalSupply`] function.
        pub fn getPastTotalSupply(
            &self,
            timepoint: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPastTotalSupplyCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, getPastVotesCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, getPastVotingPowerCall, N> {
            self.call_builder(
                &getPastVotingPowerCall {
                    account,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getRemainingEmissions`] function.
        pub fn getRemainingEmissions(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRemainingEmissionsCall, N> {
            self.call_builder(&getRemainingEmissionsCall {})
        }
        ///Creates a new call builder for the [`getRemainingLockTime`] function.
        pub fn getRemainingLockTime(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRemainingLockTimeCall, N> {
            self.call_builder(&getRemainingLockTimeCall {})
        }
        ///Creates a new call builder for the [`getRoleAdmin`] function.
        pub fn getRoleAdmin(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRoleAdminCall, N> {
            self.call_builder(&getRoleAdminCall { role })
        }
        ///Creates a new call builder for the [`getVotes`] function.
        pub fn getVotes(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getVotesCall, N> {
            self.call_builder(&getVotesCall { account })
        }
        ///Creates a new call builder for the [`getVotingPower`] function.
        pub fn getVotingPower(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getVotingPowerCall, N> {
            self.call_builder(&getVotingPowerCall { account })
        }
        ///Creates a new call builder for the [`grantRole`] function.
        pub fn grantRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, grantRoleCall, N> {
            self.call_builder(&grantRoleCall { role, account })
        }
        ///Creates a new call builder for the [`hasRole`] function.
        pub fn hasRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, hasRoleCall, N> {
            self.call_builder(&hasRoleCall { role, account })
        }
        ///Creates a new call builder for the [`maxLockTimestamp`] function.
        pub fn maxLockTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, maxLockTimestampCall, N> {
            self.call_builder(&maxLockTimestampCall {})
        }
        ///Creates a new call builder for the [`mint`] function.
        pub fn mint(
            &self,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, mintCall, N> {
            self.call_builder(&mintCall { to, amount })
        }
        ///Creates a new call builder for the [`name`] function.
        pub fn name(&self) -> alloy_contract::SolCallBuilder<T, &P, nameCall, N> {
            self.call_builder(&nameCall {})
        }
        ///Creates a new call builder for the [`nonces`] function.
        pub fn nonces(
            &self,
            owner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, noncesCall, N> {
            self.call_builder(&noncesCall { owner })
        }
        ///Creates a new call builder for the [`numCheckpoints`] function.
        pub fn numCheckpoints(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, numCheckpointsCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, permitCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceRoleCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, revokeRoleCall, N> {
            self.call_builder(&revokeRoleCall { role, account })
        }
        ///Creates a new call builder for the [`setUnlockTimestamp`] function.
        pub fn setUnlockTimestamp(
            &self,
            newUnlockTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, setUnlockTimestampCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, supportsInterfaceCall, N> {
            self.call_builder(
                &supportsInterfaceCall {
                    interfaceId,
                },
            )
        }
        ///Creates a new call builder for the [`symbol`] function.
        pub fn symbol(&self) -> alloy_contract::SolCallBuilder<T, &P, symbolCall, N> {
            self.call_builder(&symbolCall {})
        }
        ///Creates a new call builder for the [`totalSupply`] function.
        pub fn totalSupply(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, totalSupplyCall, N> {
            self.call_builder(&totalSupplyCall {})
        }
        ///Creates a new call builder for the [`transfer`] function.
        pub fn transfer(
            &self,
            to: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferCall, N> {
            self.call_builder(&transferCall { to, value })
        }
        ///Creates a new call builder for the [`transferFrom`] function.
        pub fn transferFrom(
            &self,
            from: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferFromCall, N> {
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
        ) -> alloy_contract::SolCallBuilder<T, &P, transfersLockedCall, N> {
            self.call_builder(&transfersLockedCall {})
        }
        ///Creates a new call builder for the [`unlockTimestamp`] function.
        pub fn unlockTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, unlockTimestampCall, N> {
            self.call_builder(&unlockTimestampCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SyndicateTokenInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Approval`] event.
        pub fn Approval_filter(&self) -> alloy_contract::Event<T, &P, Approval, N> {
            self.event_filter::<Approval>()
        }
        ///Creates a new event filter for the [`DelegateChanged`] event.
        pub fn DelegateChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DelegateChanged, N> {
            self.event_filter::<DelegateChanged>()
        }
        ///Creates a new event filter for the [`DelegateVotesChanged`] event.
        pub fn DelegateVotesChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DelegateVotesChanged, N> {
            self.event_filter::<DelegateVotesChanged>()
        }
        ///Creates a new event filter for the [`EIP712DomainChanged`] event.
        pub fn EIP712DomainChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EIP712DomainChanged, N> {
            self.event_filter::<EIP712DomainChanged>()
        }
        ///Creates a new event filter for the [`RoleAdminChanged`] event.
        pub fn RoleAdminChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RoleAdminChanged, N> {
            self.event_filter::<RoleAdminChanged>()
        }
        ///Creates a new event filter for the [`RoleGranted`] event.
        pub fn RoleGranted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RoleGranted, N> {
            self.event_filter::<RoleGranted>()
        }
        ///Creates a new event filter for the [`RoleRevoked`] event.
        pub fn RoleRevoked_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RoleRevoked, N> {
            self.event_filter::<RoleRevoked>()
        }
        ///Creates a new event filter for the [`TokensBurnedByManager`] event.
        pub fn TokensBurnedByManager_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TokensBurnedByManager, N> {
            self.event_filter::<TokensBurnedByManager>()
        }
        ///Creates a new event filter for the [`Transfer`] event.
        pub fn Transfer_filter(&self) -> alloy_contract::Event<T, &P, Transfer, N> {
            self.event_filter::<Transfer>()
        }
        ///Creates a new event filter for the [`UnlockTimestampUpdated`] event.
        pub fn UnlockTimestampUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, UnlockTimestampUpdated, N> {
            self.event_filter::<UnlockTimestampUpdated>()
        }
    }
}
