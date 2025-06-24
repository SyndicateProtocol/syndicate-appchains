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
    ///0x610180604052346100845761001b610015610158565b906103bd565b610023610089565b614b29611c8382396080518161290a015260a05181612941015260c051816128d1015260e051816133170152610100518161333c01526101205181612e7e01526101405181612ebe015261016051818181610b100152611f570152614b2990f35b61008f565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b906100bb90610093565b810190811060018060401b038211176100d357604052565b61009d565b906100eb6100e4610089565b92836100b1565b565b5f80fd5b60018060a01b031690565b610105906100f1565b90565b610111816100fc565b0361011857565b5f80fd5b9050519061012982610108565b565b91906040838203126101535780610147610150925f860161011c565b9360200161011c565b90565b6100ed565b6101766167ac8038038061016b816100d8565b92833981019061012b565b9091565b60018060401b03811161019657610192602091610093565b0190565b61009d565b906101ad6101a88361017a565b6100d8565b918252565b5f7f53796e6469636174650000000000000000000000000000000000000000000000910152565b6101e3600961019b565b906101f0602083016101b2565b565b6101fa6101d9565b90565b5f7f53594e4400000000000000000000000000000000000000000000000000000000910152565b61022e600461019b565b9061023b602083016101fd565b565b610245610224565b90565b90565b90565b61026261025d61026792610248565b61024b565b6100f1565b90565b6102739061024e565b90565b5f0190565b90565b90565b61029561029061029a9261027b565b61024b565b61027e565b90565b6102a96276a700610281565b90565b634e487b7160e01b5f52601160045260245ffd5b6102cf6102d59193929361027e565b9261027e565b82018092116102e057565b6102ac565b6102f96102f46102fe92610248565b61024b565b61027e565b90565b5f1b90565b906103125f1991610301565b9181191691161790565b61033061032b6103359261027e565b61024b565b61027e565b90565b90565b9061035061034b6103579261031c565b610338565b8254610306565b9055565b90565b61037261036d61037792610248565b610301565b61035b565b90565b6103835f61035e565b90565b90565b61039d6103986103a292610386565b61024b565b61027e565b90565b6103ba6b02e87669c308736a04000000610389565b90565b906103df6103c96101f2565b6103d16101f2565b6103d961023d565b916104a5565b816103fa6103f46103ef5f61026a565b6100fc565b916100fc565b14610489578061041a61041461040f5f61026a565b6100fc565b916100fc565b1461046d5761045c61046b926104384261043261029d565b906102c0565b6101605261044f6104485f6102e5565b600c61033b565b61045761037a565b610941565b506104656103a5565b90610a0f565b565b5f63d92e233d60e01b81528061048560048201610276565b0390fd5b5f63d92e233d60e01b8152806104a160048201610276565b0390fd5b906104b092916104b2565b565b906104bd92916104bf565b565b906104ca92916104cc565b565b906104d792916104d9565b565b906104e49291610531565b565b5f7f3100000000000000000000000000000000000000000000000000000000000000910152565b610517600161019b565b90610524602083016104e6565b565b61052e61050d565b90565b90610545929161053f610526565b90610547565b565b90610553939291610599565b565b90565b90565b60200190565b5190565b61057961057461057e926100f1565b61024b565b6100f1565b90565b61058a90610565565b90565b61059690610581565b90565b6105aa6105fa946105df939461062e565b6105be816105b86006610555565b90610abc565b610120526105d6836105d06007610555565b90610abc565b61014052610558565b6105f16105eb82610561565b9161055b565b2060e052610558565b61060c61060682610561565b9161055b565b20610100524660a05261061d610bc1565b6080526106293061058d565b60c052565b906106389161063a565b565b9061064491610646565b565b9061065091610897565b565b634e487b7160e01b5f525f60045260245ffd5b5190565b634e487b7160e01b5f52602260045260245ffd5b906001600283049216801561069d575b602083101461069857565b610669565b91607f169161068d565b5f5260205f2090565b601f602091010490565b1b90565b919060086106d99102916106d35f19846106ba565b926106ba565b9181191691161790565b91906106f96106f46107019361031c565b610338565b9083546106be565b9055565b5f90565b61071b91610715610705565b916106e3565b565b5b818110610729575050565b806107365f600193610709565b0161071e565b9190601f811161074c575b505050565b61075861077d936106a7565b906020610764846106b0565b83019310610785575b610776906106b0565b019061071d565b5f8080610747565b91506107768192905061076d565b1c90565b906107a7905f1990600802610793565b191690565b816107b691610797565b906002021790565b906107c881610665565b9060018060401b038211610886576107ea826107e4855461067d565b8561073c565b602090601f831160011461081e5791809161080d935f92610812575b50506107ac565b90555b565b90915001515f80610806565b601f1983169161082d856106a7565b925f5b81811061086e57509160029391856001969410610854575b50505002019055610810565b610864910151601f841690610797565b90555f8080610848565b91936020600181928787015181550195019201610830565b61009d565b90610895916107be565b565b906108a66108ad92600361088b565b600461088b565b565b5f90565b151590565b6108c19061035b565b90565b906108ce906108b8565b5f5260205260405f2090565b6108e390610581565b90565b906108f0906108da565b5f5260205260405f2090565b9061090860ff91610301565b9181191691161790565b61091b906108b3565b90565b90565b9061093661093161093d92610912565b61091e565b82546108fc565b9055565b6109496108af565b5061095e610958828490610c5e565b156108b3565b5f146109e75761098660016109815f610979600586906108c4565b0185906108e6565b610921565b9061098f610c8c565b906109cc6109c66109c07f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d956108b8565b926108da565b926108da565b926109d5610089565b806109df81610276565b0390a4600190565b50505f90565b6109f6906100fc565b9052565b9190610a0d905f602085019401906109ed565b565b80610a2a610a24610a1f5f61026a565b6100fc565b916100fc565b14610a4657610a4491610a3c5f61026a565b919091610cbd565b565b610a69610a525f61026a565b5f91829163ec442f0560e01b8352600483016109fa565b0390fd5b5f90565b90565b610a88610a83610a8d92610a71565b61024b565b61027e565b90565b90565b610aa7610aa2610aac92610a90565b610301565b61035b565b90565b610ab960ff610a93565b90565b90610ac5610a6d565b50610ad7610ad283610558565b610561565b610aea610ae46020610a74565b9161027e565b105f14610afe5750610afb90610e57565b90565b5f610b0c610b129392610d67565b0161088b565b610b22610b1d610aaf565b6108b8565b90565b5f90565b7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f90565b610b57905161035b565b90565b610b639061035b565b9052565b610b709061027e565b9052565b90959492610bbf94610bae610bb892610ba4608096610b9a60a088019c5f890190610b5a565b6020870190610b5a565b6040850190610b5a565b6060830190610b67565b01906109ed565b565b610bc9610b25565b50610bd2610b29565b610c1c610bdf60e0610b4d565b91610c0d610bee610100610b4d565b46610bf83061058d565b91610c01610089565b96879560208701610b74565b602082018103825203826100b1565b610c2e610c2882610561565b9161055b565b2090565b5f1c90565b60ff1690565b610c49610c4e91610c32565b610c37565b90565b610c5b9054610c3d565b90565b610c85915f610c7a610c8093610c726108af565b5060056108c4565b016108e6565b610c51565b90565b5f90565b610c94610c88565b503390565b7f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a67490565b9182610cd9610cd3610cce5f61026a565b6100fc565b916100fc565b141580610d44575b610cf4575b610cf292919091610f7b565b565b610cfc610f05565b80610d23575b15610ce6575f6336e278fd60e21b815280610d1f60048201610276565b0390fd5b50610d3f610d39610d32610c99565b3390610c5e565b156108b3565b610d02565b5081610d60610d5a610d555f61026a565b6100fc565b916100fc565b1415610ce1565b90565b90565b610d81610d7c610d8692610d6a565b61024b565b61027e565b90565b60209181520190565b90825f9392825e0152565b610dbc610dc5602093610dca93610db381610665565b93848093610d89565b95869101610d92565b610093565b0190565b610de39160208201915f818403910152610d9d565b90565b610e00610dfb610df583610561565b9261055b565b610b4d565b9060208110610e0e575b5090565b610e20905f19906020036008026106ba565b165f610e0a565b610e33610e3891610c32565b61031c565b90565b610e4f610e4a610e549261027e565b610301565b61035b565b90565b610e5f610a6d565b50610e6981610558565b90610e7382610561565b610e86610e80601f610d6d565b9161027e565b11610ebb5750610eb381610ead610ea7610ea2610eb895610de6565b610e27565b91610561565b17610e3b565b6108b8565b90565b610edd90610ec7610089565b91829163305a27a960e01b835260048301610dce565b0390fd5b90565b610ef0610ef591610c32565b610ee1565b90565b610f029054610ee4565b90565b610f0d6108af565b50610f18600c610ef8565b610f2a610f245f6102e5565b9161027e565b141580610f35575b90565b5042610f52610f4c610f47600c610ef8565b61027e565b9161027e565b10610f32565b916020610f79929493610f7260408201965f830190610b67565b0190610b67565b565b9291610f8984838391611084565b83610fa4610f9e610f995f61026a565b6100fc565b916100fc565b14610fb9575b610fb79293919091611251565b565b610fc16111f3565b93610fca611230565b9480610fde610fd88861027e565b9161027e565b11610feb57509350610faa565b85906110075f928392630e58ae9360e11b845260048401610f58565b0390fd5b90611015906108da565b5f5260205260405f2090565b60409061104a611051949695939661104060608401985f8501906109ed565b6020830190610b67565b0190610b67565b565b9061105e910361027e565b90565b9061106c910161027e565b90565b9190611082905f60208501940190610b67565b565b919091806110a261109c6110975f61026a565b6100fc565b916100fc565b145f14611183576110c66110bf836110ba6002610ef8565b6102c0565b600261033b565b5b826110e26110dc6110d75f61026a565b6100fc565b916100fc565b145f14611157576111066110ff836110fa6002610ef8565b611053565b600261033b565b5b91909161115261114061113a7fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef936108da565b936108da565b93611149610089565b9182918261106f565b0390a3565b61117e826111786111695f879061100b565b9161117383610ef8565b611061565b9061033b565b611107565b6111966111915f839061100b565b610ef8565b806111a96111a38561027e565b9161027e565b106111d1576111bc6111cc918490611053565b6111c75f849061100b565b61033b565b6110c7565b906111ef9091925f93849363391434e360e21b855260048501611021565b0390fd5b6111fb610705565b506112066002610ef8565b90565b60018060d01b031690565b61122861122361122d92611209565b61024b565b61027e565b90565b611238610705565b5061124860018060d01b03611214565b90565b90565b90565b916112a96112a36112b0948061127761127161126c5f61026a565b6100fc565b916100fc565b146112e1575b8461129861129261128d5f61026a565b6100fc565b916100fc565b146112b2575b6114d9565b926114d9565b909161150e565b565b6112da600b60026112d46112ce6112c8896113c3565b9361124b565b9161124e565b90611416565b505061129e565b611309600b60016113036112fd6112f7896113c3565b9361124b565b9161124e565b90611416565b505061127d565b5f90565b61132061132691611209565b91611209565b019060018060d01b03821161133757565b6102ac565b9061134f91611349611310565b50611314565b90565b90565b60ff1690565b61136f61136a61137492611352565b61024b565b611355565b90565b6113809061135b565b9052565b9160206113a592949361139e60408201965f830190611377565b0190610b67565b565b6113bb6113b66113c09261027e565b61024b565b611209565b90565b6113cb611310565b50806113e56113df60018060d01b03611214565b9161027e565b116113f6576113f3906113a7565b90565b60d06114125f9283926306dfcc6560e41b845260048401611384565b0390fd5b9061144c6114529392611427611310565b50611430611310565b50809361144561143e6116c0565b949261176d565b9091611c53565b916117e2565b91909190565b61146461146a91611209565b91611209565b90039060018060d01b03821161147c57565b6102ac565b906114949161148e611310565b50611458565b90565b906114a1906108da565b5f5260205260405f2090565b60018060a01b031690565b6114c46114c991610c32565b6114ad565b90565b6114d690546114b8565b90565b6114f06114f5916114e8610c88565b506009611497565b6114cc565b90565b90611502906108da565b5f5260205260405f2090565b9190918061152461151e856100fc565b916100fc565b1415806116a2575b611536575b505050565b8061155161154b6115465f61026a565b6100fc565b916100fc565b03611612575b508161157361156d6115685f61026a565b6100fc565b916100fc565b0361157f575b80611531565b6115c66115b96115c092611595600a86906114f8565b906115b36115ad6115a76001936113c3565b9361124b565b9161124e565b90611416565b9290611214565b91611214565b9190916115f37fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926108da565b926116086115ff610089565b92839283610f58565b0390a25f80611579565b61165161165761164a611627600a85906114f8565b600261164461163e611638896113c3565b9361124b565b9161124e565b90611416565b9290611214565b91611214565b9190916116847fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926108da565b92611699611690610089565b92839283610f58565b0390a25f611557565b50816116b66116b05f6102e5565b9161027e565b1161152c565b5f90565b6116c86116bc565b506116d1611811565b90565b5490565b90565b6116ef6116ea6116f4926116d8565b61024b565b61027e565b90565b61170661170c9193929361027e565b9261027e565b820391821161171757565b6102ac565b90565b60301c90565b60018060d01b031690565b61173c6117419161171f565b611725565b90565b61174e9054611730565b90565b61176561176061176a92610248565b61024b565b611209565b90565b611775611310565b506117815f82016116d4565b8061179461178e5f6102e5565b9161027e565b145f146117aa5750506117a65f611751565b5b90565b6117d75f916117d26117cc846117dd9601926117c660016116db565b906116f7565b9161171c565b611826565b01611744565b6117a7565b916118065f61180b946117f3611310565b506117fc611310565b500192919261171c565b611a2b565b91909190565b6118196116bc565b5061182343611bec565b90565b5f5260205f200190565b5490565b61183e60406100d8565b90565b65ffffffffffff1690565b9061185690611841565b9052565b9061186490611209565b9052565b5f5260205f2090565b634e487b7160e01b5f52603260045260245ffd5b61188e81611830565b8210156118a8576118a0600191611868565b910201905f90565b611871565b6118b79051611841565b90565b906118cb65ffffffffffff91610301565b9181191691161790565b6118e96118e46118ee92611841565b61024b565b611841565b90565b90565b90611909611904611910926118d5565b6118f1565b82546118ba565b9055565b61191e9051611209565b90565b60301b90565b9061193965ffffffffffff1991611921565b9181191691161790565b61195761195261195c92611209565b61024b565b611209565b90565b90565b9061197761197261197e92611943565b61195f565b8254611927565b9055565b906119ac60205f6119b2946119a482820161199e8488016118ad565b906118f4565b019201611914565b90611962565b565b91906119c5576119c391611982565b565b610652565b90815491680100000000000000008310156119fa57826119f29160016119f895018155611885565b906119b4565b565b61009d565b65ffffffffffff1690565b611a16611a1b91610c32565b6119ff565b90565b611a289054611a0a565b90565b90929192611a37611310565b50611a40611310565b50611a4a82611830565b80611a5d611a575f6102e5565b9161027e565b115f14611b2d57611a8390611a7d8491611a7760016116db565b906116f7565b90611826565b90611a8f5f8301611a1e565b92611a9b5f8401611744565b9380611aaf611aa985611841565b91611841565b11611b1157611ac6611ac084611841565b91611841565b145f14611ae1575050611adc905f859101611962565b5b9190565b611b0c9250611b0786611afe611af5611834565b945f860161184c565b6020840161185a565b6119ca565b611add565b5f632520601d60e01b815280611b2960048201610276565b0390fd5b50611b5891611b5385611b4a611b41611834565b945f860161184c565b6020840161185a565b6119ca565b611b615f611751565b9190565b611b79611b74611b7e92611841565b61024b565b61027e565b90565b90565b611b98611b93611b9d92611b81565b61024b565b611355565b90565b611ba990611b84565b9052565b916020611bce929493611bc760408201965f830190611ba0565b0190610b67565b565b611be4611bdf611be99261027e565b61024b565b611841565b90565b611bf46116bc565b5080611c0e611c0865ffffffffffff611b65565b9161027e565b11611c1f57611c1c90611bd0565b90565b6030611c3b5f9283926306dfcc6560e41b845260048401611bad565b0390fd5b634e487b7160e01b5f52605160045260245ffd5b91909180600114611c7257600203611c3f57611c6e91611481565b905b565b50611c7c9161133c565b90611c7056fe60806040526004361015610013575b611490565b61001d5f356102fc565b806301ffc9a7146102f757806306fdde03146102f2578063095ea7b3146102ed57806318160ddd146102e857806323b872dd146102e3578063248a9ca3146102de5780632f2ff15d146102d9578063313ce567146102d45780633644e515146102cf57806336568abe146102ca5780633a46b1a8146102c557806340c10f19146102c05780634bdd36ce146102bb5780634bf5d7e9146102b65780634f1bfc9e146102b1578063587cde1e146102ac5780635c19a95c146102a75780636fcfff45146102a257806370a082311461029d57806379cc6790146102985780637a8cd156146102935780637ecebe001461028e57806383f1211b146102895780638426adf214610284578063844c90261461027f57806384b0196e1461027a5780638a542521146102755780638d3343d6146102705780638e539e8c1461026b578063902d55a51461026657806391d148541461026157806391ddadf41461025c57806395d89b41146102575780639ab24eb0146102525780639b7ef64b1461024d578063a217fddf14610248578063a9059cbb14610243578063aa082a9d1461023e578063b0ca253e14610239578063bb4d443614610234578063c02ae7541461022f578063c3cda5201461022a578063d505accf14610225578063d547741f14610220578063dd62ed3e1461021b5763f1127ed80361000e5761145a565b611376565b611315565b6112db565b611231565b611175565b611140565b61110a565b6110d5565b611063565b61102e565b610fbe565b610f47565b610f12565b610edd565b610e7a565b610e45565b610dce565b610d99565b610d35565b610cca565b610b85565b610b32565b610ad9565b610aa4565b610a6f565b610a3b565b610a06565b6109d1565b610973565b61093e565b6108c9565b610858565b610823565b6107ef565b6107b9565b610785565b610750565b61071b565b6106bf565b610658565b6105bc565b61054d565b6104f5565b610433565b610384565b60e01c90565b60405190565b5f80fd5b5f80fd5b63ffffffff60e01b1690565b61032581610310565b0361032c57565b5f80fd5b9050359061033d8261031c565b565b9060208282031261035857610355915f01610330565b90565b61030c565b151590565b61036b9061035d565b9052565b9190610382905f60208501940190610362565b565b346103b4576103b061039f61039a36600461033f565b61152d565b6103a7610302565b9182918261036f565b0390f35b610308565b5f9103126103c357565b61030c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b61040961041260209361041793610400816103c8565b938480936103cc565b958691016103d5565b6103e0565b0190565b6104309160208201915f8184039101526103ea565b90565b34610463576104433660046103b9565b61045f61044e6116c6565b610456610302565b9182918261041b565b0390f35b610308565b60018060a01b031690565b61047c90610468565b90565b61048881610473565b0361048f57565b5f80fd5b905035906104a08261047f565b565b90565b6104ae816104a2565b036104b557565b5f80fd5b905035906104c6826104a5565b565b91906040838203126104f057806104e46104ed925f8601610493565b936020016104b9565b90565b61030c565b346105265761052261051161050b3660046104c8565b906116dc565b610519610302565b9182918261036f565b0390f35b610308565b610534906104a2565b9052565b919061054b905f6020850194019061052b565b565b3461057d5761055d3660046103b9565b610579610568611728565b610570610302565b91829182610538565b0390f35b610308565b90916060828403126105b7576105b461059d845f8501610493565b936105ab8160208601610493565b936040016104b9565b90565b61030c565b346105ed576105e96105d86105d2366004610582565b9161173e565b6105e0610302565b9182918261036f565b0390f35b610308565b90565b6105fe816105f2565b0361060557565b5f80fd5b90503590610616826105f5565b565b906020828203126106315761062e915f01610609565b90565b61030c565b61063f906105f2565b9052565b9190610656905f60208501940190610636565b565b346106885761068461067361066e366004610618565b6117b7565b61067b610302565b91829182610643565b0390f35b610308565b91906040838203126106b557806106a96106b2925f8601610609565b93602001610493565b90565b61030c565b5f0190565b346106ee576106d86106d236600461068d565b90611803565b6106e0610302565b806106ea816106ba565b0390f35b610308565b60ff1690565b610702906106f3565b9052565b9190610719905f602085019401906106f9565b565b3461074b5761072b3660046103b9565b610747610736611832565b61073e610302565b91829182610706565b0390f35b610308565b34610780576107603660046103b9565b61077c61076b611848565b610773610302565b91829182610643565b0390f35b610308565b346107b45761079e61079836600461068d565b9061185c565b6107a6610302565b806107b0816106ba565b0390f35b610308565b346107ea576107e66107d56107cf3660046104c8565b9061190d565b6107dd610302565b91829182610538565b0390f35b610308565b3461081e576108086108023660046104c8565b90611a94565b610810610302565b8061081a816106ba565b0390f35b610308565b34610853576108333660046103b9565b61084f61083e611ac5565b610846610302565b91829182610538565b0390f35b610308565b34610888576108683660046103b9565b610884610873611b84565b61087b610302565b9182918261041b565b0390f35b610308565b90565b90565b6108a76108a26108ac9261088d565b610890565b6104a2565b90565b6108bb6276a700610893565b90565b6108c66108af565b90565b346108f9576108d93660046103b9565b6108f56108e46108be565b6108ec610302565b91829182610538565b0390f35b610308565b9060208282031261091757610914915f01610493565b90565b61030c565b61092590610473565b9052565b919061093c905f6020850194019061091c565b565b3461096e5761096a6109596109543660046108fe565b611c20565b610961610302565b91829182610929565b0390f35b610308565b346109a15761098b6109863660046108fe565b611c3f565b610993610302565b8061099d816106ba565b0390f35b610308565b63ffffffff1690565b6109b8906109a6565b9052565b91906109cf905f602085019401906109af565b565b34610a01576109fd6109ec6109e73660046108fe565b611c56565b6109f4610302565b918291826109bc565b0390f35b610308565b34610a3657610a32610a21610a1c3660046108fe565b611c81565b610a29610302565b91829182610538565b0390f35b610308565b34610a6a57610a54610a4e3660046104c8565b90611db6565b610a5c610302565b80610a66816106ba565b0390f35b610308565b34610a9f57610a7f3660046103b9565b610a9b610a8a611dc2565b610a92610302565b91829182610538565b0390f35b610308565b34610ad457610ad0610abf610aba3660046108fe565b611e3a565b610ac7610302565b91829182610538565b0390f35b610308565b34610b0957610ae93660046103b9565b610b05610af4611e4f565b610afc610302565b9182918261036f565b0390f35b610308565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610b6257610b423660046103b9565b610b5e610b4d610b0e565b610b55610302565b91829182610538565b0390f35b610308565b90602082820312610b8057610b7d915f016104b9565b90565b61030c565b34610bb357610b9d610b98366004610b67565b61201a565b610ba5610302565b80610baf816106ba565b0390f35b610308565b60ff60f81b1690565b610bca90610bb8565b9052565b5190565b60209181520190565b60200190565b610bea906104a2565b9052565b90610bfb81602093610be1565b0190565b60200190565b90610c22610c1c610c1584610bce565b8093610bd2565b92610bdb565b905f5b818110610c325750505090565b909192610c4b610c456001928651610bee565b94610bff565b9101919091610c25565b93959194610ca6610c9b610cba95610c8d610cb095610cc79c9a610c8060e08c01925f8d0190610bc1565b8a820360208c01526103ea565b9088820360408a01526103ea565b97606087019061052b565b608085019061091c565b60a0830190610636565b60c0818403910152610c05565b90565b34610d0157610cda3660046103b9565b610cfd610ce56120a2565b93610cf4979597939193610302565b97889788610c55565b0390f35b610308565b7f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a67490565b610d32610d06565b90565b34610d6557610d453660046103b9565b610d61610d50610d2a565b610d58610302565b91829182610643565b0390f35b610308565b7f9b12e0c5707e494915e58b0564f18aaad9b74ac69bfc815a1edadc8e4bd032eb90565b610d96610d6a565b90565b34610dc957610da93660046103b9565b610dc5610db4610d8e565b610dbc610302565b91829182610643565b0390f35b610308565b34610dfe57610dfa610de9610de4366004610b67565b61212c565b610df1610302565b91829182610538565b0390f35b610308565b90565b610e1a610e15610e1f92610e03565b610890565b6104a2565b90565b610e376b033b2e3c9fd0803ce8000000610e06565b90565b610e42610e22565b90565b34610e7557610e553660046103b9565b610e71610e60610e3a565b610e68610302565b91829182610538565b0390f35b610308565b34610eab57610ea7610e96610e9036600461068d565b9061219a565b610e9e610302565b9182918261036f565b0390f35b610308565b65ffffffffffff1690565b610ec490610eb0565b9052565b9190610edb905f60208501940190610ebb565b565b34610f0d57610eed3660046103b9565b610f09610ef86121c8565b610f00610302565b91829182610ec8565b0390f35b610308565b34610f4257610f223660046103b9565b610f3e610f2d6121dc565b610f35610302565b9182918261041b565b0390f35b610308565b34610f7757610f73610f62610f5d3660046108fe565b6121f2565b610f6a610302565b91829182610538565b0390f35b610308565b90565b610f93610f8e610f9892610f7c565b610890565b6104a2565b90565b610fb06b02e87669c308736a04000000610f7f565b90565b610fbb610f9b565b90565b34610fee57610fce3660046103b9565b610fea610fd9610fb3565b610fe1610302565b91829182610538565b0390f35b610308565b90565b5f1b90565b61100f61100a61101492610ff3565b610ff6565b6105f2565b90565b6110205f610ffb565b90565b61102b611017565b90565b3461105e5761103e3660046103b9565b61105a611049611023565b611051610302565b91829182610643565b0390f35b610308565b346110945761109061107f6110793660046104c8565b90612221565b611087610302565b9182918261036f565b0390f35b610308565b1c90565b90565b6110b09060086110b59302611099565b61109d565b90565b906110c391546110a0565b90565b6110d2600c5f906110b8565b90565b34611105576110e53660046103b9565b6111016110f06110c6565b6110f8610302565b91829182610538565b0390f35b610308565b3461113b576111376111266111203660046104c8565b90612243565b61112e610302565b91829182610538565b0390f35b610308565b346111705761116c61115b6111563660046108fe565b612259565b611163610302565b91829182610538565b0390f35b610308565b346111a5576111853660046103b9565b6111a161119061226e565b611198610302565b91829182610538565b0390f35b610308565b6111b3816106f3565b036111ba57565b5f80fd5b905035906111cb826111aa565b565b909160c08284031261122c576111e5835f8401610493565b926111f381602085016104b9565b9261120182604083016104b9565b9261122961121284606085016111be565b936112208160808601610609565b9360a001610609565b90565b61030c565b34611266576112506112443660046111cd565b949390939291926122ee565b611258610302565b80611262816106ba565b0390f35b610308565b60e0818303126112d657611281825f8301610493565b9261128f8360208401610493565b9261129d81604085016104b9565b926112ab82606083016104b9565b926112d36112bc84608085016111be565b936112ca8160a08601610609565b9360c001610609565b90565b61030c565b34611310576112fa6112ee36600461126b565b95949094939193612442565b611302610302565b8061130c816106ba565b0390f35b610308565b346113445761132e61132836600461068d565b90612560565b611336610302565b80611340816106ba565b0390f35b610308565b9190604083820312611371578061136561136e925f8601610493565b93602001610493565b90565b61030c565b346113a7576113a361139261138c366004611349565b90612582565b61139a610302565b91829182610538565b0390f35b610308565b6113b5816109a6565b036113bc57565b5f80fd5b905035906113cd826113ac565b565b91906040838203126113f757806113eb6113f4925f8601610493565b936020016113c0565b90565b61030c565b61140590610eb0565b9052565b60018060d01b031690565b61141d90611409565b9052565b90602080611443936114395f8201515f8601906113fc565b0151910190611414565b565b9190611458905f60408501940190611421565b565b3461148b576114876114766114703660046113cf565b906125f0565b61147e610302565b91829182611445565b0390f35b610308565b5f80fd5b5f90565b634e487b7160e01b5f52601160045260245ffd5b6114b86114be91611409565b91611409565b019060018060d01b0382116114cf57565b611498565b906114e7916114e1611494565b506114ac565b90565b6114f66114fc91611409565b91611409565b90039060018060d01b03821161150e57565b611498565b9061152691611520611494565b506114ea565b90565b5f90565b611535611529565b508061155061154a637965db0b60e01b610310565b91610310565b1490811561155d575b5090565b6115679150612606565b5f611559565b606090565b634e487b7160e01b5f52602260045260245ffd5b90600160028304921680156115a6575b60208310146115a157565b611572565b91607f1691611596565b60209181520190565b5f5260205f2090565b905f92918054906115dc6115d583611586565b80946115b0565b916001811690815f1461163357506001146115f7575b505050565b61160491929394506115b9565b915f925b81841061161b57505001905f80806115f2565b60018160209295939554848601520191019290611608565b92949550505060ff19168252151560200201905f80806115f2565b90611658916115c2565b90565b634e487b7160e01b5f52604160045260245ffd5b90611679906103e0565b810190811067ffffffffffffffff82111761169357604052565b61165b565b906116b86116b1926116a8610302565b9384809261164e565b038361166f565b565b6116c390611698565b90565b6116ce61156d565b506116d960036116ba565b90565b6116f9916116e8611529565b506116f161262c565b919091612639565b600190565b5f90565b5f1c90565b61171361171891611702565b61109d565b90565b6117259054611707565b90565b6117306116fe565b5061173b600261171b565b90565b916117689261174b611529565b5061176061175761262c565b82908491612689565b919091612715565b600190565b5f90565b61177a906105f2565b90565b9061178790611771565b5f5260205260405f2090565b90565b6117a26117a791611702565b611793565b90565b6117b49054611796565b90565b60016117d06117d6926117c861176d565b50600561177d565b016117aa565b90565b906117f4916117ef6117ea826117b7565b6127b2565b6117f6565b565b906118009161280b565b50565b9061180d916117d9565b565b5f90565b90565b61182a61182561182f92611813565b610890565b6106f3565b90565b61183a61180f565b506118456012611816565b90565b61185061176d565b506118596128b7565b90565b908061187761187161186c61262c565b610473565b91610473565b036118885761188591612971565b50565b5f63334bd91960e11b8152806118a0600482016106ba565b0390fd5b6118b86118b36118bd92610468565b610890565b610468565b90565b6118c9906118a4565b90565b6118d5906118c0565b90565b906118e2906118cc565b5f5260205260405f2090565b90565b61190561190061190a92611409565b610890565b6104a2565b90565b6119449161193961193361192e61193f946119266116fe565b50600a6118d8565b6118ee565b91612a52565b90612b67565b6118f1565b90565b906119619161195c611957610d6a565b6127b2565b6119cc565b565b61197761197261197c92610ff3565b610890565b610468565b90565b61198890611963565b90565b61199f61199a6119a492610ff3565b610890565b6104a2565b90565b6119b66119bc919392936104a2565b926104a2565b82018092116119c757565b611498565b90816119e86119e26119dd5f61197f565b610473565b91610473565b14611a785780611a006119fa5f61198b565b916104a2565b14611a5c57611a17611a10611728565b82906119a7565b611a30611a2a611a25610e22565b6104a2565b916104a2565b11611a4057611a3e91612c8e565b565b5f63177e3fc360e01b815280611a58600482016106ba565b0390fd5b5f631f2a200560e01b815280611a74600482016106ba565b0390fd5b5f63d92e233d60e01b815280611a90600482016106ba565b0390fd5b90611a9e91611947565b565b611aaf611ab5919392936104a2565b926104a2565b8203918211611ac057565b611498565b611acd6116fe565b50611ae7611ad9610e22565b611ae1611728565b90611aa0565b90565b90611afd611af6610302565b928361166f565b565b67ffffffffffffffff8111611b1d57611b196020916103e0565b0190565b61165b565b90611b34611b2f83611aff565b611aea565b918252565b5f7f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c74000000910152565b611b6a601d611b22565b90611b7760208301611b39565b565b611b81611b60565b90565b611b8c61156d565b50611b956121c8565b611bae611ba8611ba3612cec565b610eb0565b91610eb0565b03611bbe57611bbb611b79565b90565b5f6301bfc1c560e61b815280611bd6600482016106ba565b0390fd5b5f90565b90611be8906118cc565b5f5260205260405f2090565b60018060a01b031690565b611c0b611c1091611702565b611bf4565b90565b611c1d9054611bff565b90565b611c37611c3c91611c2f611bda565b506009611bde565b611c13565b90565b611c5090611c4b61262c565b612d3f565b565b5f90565b611c6890611c62611c52565b50612dca565b90565b90611c75906118cc565b5f5260205260405f2090565b611c97611c9c91611c906116fe565b505f611c6b565b61171b565b90565b90611cb991611cb4611caf610d06565b6127b2565b611cbb565b565b80611cd6611cd0611ccb5f61197f565b610473565b91610473565b14611d9a5781611cee611ce85f61198b565b916104a2565b14611d7e57611d04611cfe611e4f565b1561035d565b611d6257611d13818390612df9565b3390611d5d611d4b611d457fbef4f81c1814c641ede85ebaacf19d048b2c5b55980adfa6ef0f956c651335a2936118cc565b936118cc565b93611d54610302565b91829182610538565b0390a3565b5f63b8b5ca2d60e01b815280611d7a600482016106ba565b0390fd5b5f631f2a200560e01b815280611d96600482016106ba565b0390fd5b5f63d92e233d60e01b815280611db2600482016106ba565b0390fd5b90611dc091611c9f565b565b611dca6116fe565b50611dd5600c61171b565b611de7611de15f61198b565b916104a2565b148015611e16575b611e0a57611e07611e00600c61171b565b4290611aa0565b90565b611e135f61198b565b90565b5042611e33611e2d611e28600c61171b565b6104a2565b916104a2565b1015611def565b611e4c90611e466116fe565b50612e58565b90565b611e57611529565b50611e62600c61171b565b611e74611e6e5f61198b565b916104a2565b141580611e7f575b90565b5042611e9c611e96611e91600c61171b565b6104a2565b916104a2565b10611e7c565b611ebb90611eb6611eb1611017565b6127b2565b611f35565b565b90611ec95f1991610ff6565b9181191691161790565b611ee7611ee2611eec926104a2565b610890565b6104a2565b90565b90565b90611f07611f02611f0e92611ed3565b611eef565b8254611ebd565b9055565b916020611f33929493611f2c60408201965f83019061052b565b019061052b565b565b80611f48611f42426104a2565b916104a2565b1115611ffe5780611f81611f7b7f00000000000000000000000000000000000000000000000000000000000000006104a2565b916104a2565b11611fe257611f90600c61171b565b611f9b82600c611ef2565b903390611fc87fdd6896dcf1d4b311cca87dd19bbba2ea9ce2f867c1568878a0438a66a1afeeec926118cc565b92611fdd611fd4610302565b92839283611f12565b0390a2565b5f63ef69af6560e01b815280611ffa600482016106ba565b0390fd5b5f63a565835360e01b815280612016600482016106ba565b0390fd5b61202390611ea2565b565b5f90565b606090565b612037906118c0565b90565b67ffffffffffffffff81116120525760208091020190565b61165b565b906120696120648361203a565b611aea565b918252565b369037565b9061209861208083612057565b9260208061208e869361203a565b920191039061206e565b565b600f60f81b90565b6120aa612025565b506120b361156d565b506120bc61156d565b506120c56116fe565b506120ce611bda565b506120d761176d565b506120e0612029565b506120e9612e70565b906120f2612eb0565b9046906120fe3061202e565b906121085f610ffb565b9061211a6121155f61198b565b612073565b9061212361209a565b96959493929190565b61215561215a9161213b6116fe565b5061214f612149600b6118ee565b91612a52565b90612b67565b6118f1565b90565b90612167906118cc565b5f5260205260405f2090565b60ff1690565b61218561218a91611702565b612173565b90565b6121979054612179565b90565b6121c1915f6121b66121bc936121ae611529565b50600561177d565b0161215d565b61218d565b90565b5f90565b6121d06121c4565b506121d9612cec565b90565b6121e461156d565b506121ef60046116ba565b90565b61221961221461220f61221e936122076116fe565b50600a6118d8565b6118ee565b612ef0565b6118f1565b90565b61223e9161222d611529565b5061223661262c565b919091612715565b600190565b90612256916122506116fe565b5061190d565b90565b61226b906122656116fe565b506121f2565b90565b6122766116fe565b5061227f611728565b90565b7fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf90565b6122db6122e2946122d16060949897956122c7608086019a5f870190610636565b602085019061091c565b604083019061052b565b019061052b565b565b60200190565b5190565b9395949092919542612308612302896104a2565b916104a2565b1161238157916123739161237a9361236a61237f9899612352612329612282565b6123438b938b612337610302565b958694602086016122a6565b6020820181038252038261166f565b61236461235e826122ea565b916122e4565b20612f65565b92909192612f82565b9182612fcc565b612d3f565b565b61239c875f918291632341d78760e11b835260048301610538565b0390fd5b7f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c990565b919461240c6124169298979561240260a0966123f861241d9a6123ee60c08a019e5f8b0190610636565b602089019061091c565b604087019061091c565b606085019061052b565b608083019061052b565b019061052b565b565b91602061244092949361243960408201965f83019061091c565b019061091c565b565b96959193929490944261245d612457836104a2565b916104a2565b1161251757906124c66124cf9493926124ae6124776123a0565b61249f8c80948c916124898d9161301e565b9192612493610302565b978896602088016123c4565b6020820181038252038261166f565b6124c06124ba826122ea565b916122e4565b20612f65565b92909192612f82565b806124e26124dc87610473565b91610473565b036124f757506124f59293919091612639565b565b84906125135f9283926325c0072360e11b84526004840161241f565b0390fd5b612532905f91829163313c898160e11b835260048301610538565b0390fd5b906125519161254c612547826117b7565b6127b2565b612553565b565b9061255d91612971565b50565b9061256a91612536565b565b90612576906118cc565b5f5260205260405f2090565b6125a79161259d6125a2926125956116fe565b50600161256c565b611c6b565b61171b565b90565b6125b46040611aea565b90565b5f90565b5f90565b6125c76125aa565b90602080836125d46125b7565b8152016125df6125bb565b81525050565b6125ed6125bf565b90565b90612603916125fd6125e5565b50613051565b90565b61260e611529565b506126286126226301ffc9a760e01b610310565b91610310565b1490565b612634611bda565b503390565b916126479291600192613079565b565b604090612672612679949695939661266860608401985f85019061091c565b602083019061052b565b019061052b565b565b9061268691036104a2565b90565b929192612697818390612582565b90816126ac6126a65f196104a2565b916104a2565b106126b9575b5050509050565b816126cc6126c6876104a2565b916104a2565b106126f2576126e993946126e191939261267b565b905f92613079565b805f80806126b2565b50612711849291925f938493637dc7a0d960e11b855260048501612649565b0390fd5b918261273161272b6127265f61197f565b610473565b91610473565b1461278b578161275161274b6127465f61197f565b610473565b91610473565b146127645761276292919091613188565b565b6127876127705f61197f565b5f91829163ec442f0560e01b835260048301610929565b0390fd5b6127ae6127975f61197f565b5f918291634b637e8f60e11b835260048301610929565b0390fd5b6127c4906127be61262c565b90613255565b565b906127d260ff91610ff6565b9181191691161790565b6127e59061035d565b90565b90565b906128006127fb612807926127dc565b6127e8565b82546127c6565b9055565b612813611529565b5061282861282282849061219a565b1561035d565b5f146128b157612850600161284b5f6128436005869061177d565b01859061215d565b6127eb565b9061285961262c565b9061289661289061288a7f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d95611771565b926118cc565b926118cc565b9261289f610302565b806128a9816106ba565b0390a4600190565b50505f90565b6128bf61176d565b506128c93061202e565b6128fb6128f57f0000000000000000000000000000000000000000000000000000000000000000610473565b91610473565b1480612937575b5f1461292c577f000000000000000000000000000000000000000000000000000000000000000090565b612934613301565b90565b504661296b6129657f00000000000000000000000000000000000000000000000000000000000000006104a2565b916104a2565b14612902565b612979611529565b5061298581839061219a565b5f14612a0d576129ac5f6129a75f61299f6005869061177d565b01859061215d565b6127eb565b906129b561262c565b906129f26129ec6129e67ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b95611771565b926118cc565b926118cc565b926129fb610302565b80612a05816106ba565b0390a4600190565b50505f90565b612a27612a22612a2c92610eb0565b610890565b6104a2565b90565b916020612a50929493612a4960408201965f83019061052b565b0190610ebb565b565b612a5a6121c4565b50612a636121c8565b81612a76612a7083612a13565b916104a2565b1015612a895750612a869061340a565b90565b90612aa45f928392637669fc0f60e11b845260048401612a2f565b0390fd5b5490565b90565b612ac3612abe612ac892612aac565b610890565b6104a2565b90565b90565b65ffffffffffff1690565b612ae5612aea91611702565b612ace565b90565b612af79054612ad9565b90565b90565b612b11612b0c612b1692612afa565b610890565b6104a2565b90565b60301c90565b60018060d01b031690565b612b36612b3b91612b19565b612b1f565b90565b612b489054612b2a565b90565b612b5f612b5a612b6492610ff3565b610890565b611409565b90565b90612bbb90612b74611494565b50612b805f8401612aa8565b612b895f61198b565b908080612b9f612b996005612aaf565b916104a2565b11612c1c575b5090612bb65f860193919293612acb565b613a5d565b80612bce612bc85f61198b565b916104a2565b145f14612be4575050612be05f612b4b565b5b90565b612c115f91612c0c612c0684612c17960192612c006001612afd565b90611aa0565b91612acb565b613a53565b01612b3e565b612be1565b80612c2a612c3092916136e8565b90611aa0565b9083612c62612c5c612c575f612c51818c01612c4c8991612acb565b613a53565b01612aed565b610eb0565b91610eb0565b105f14612c735750905b905f612ba5565b9150612c8990612c836001612afd565b906119a7565b612c6c565b80612ca9612ca3612c9e5f61197f565b610473565b91610473565b14612cc557612cc391612cbb5f61197f565b919091613188565b565b612ce8612cd15f61197f565b5f91829163ec442f0560e01b835260048301610929565b0390fd5b612cf46121c4565b50612cfe4361340a565b90565b90612d1260018060a01b0391610ff6565b9181191691161790565b90565b90612d34612d2f612d3b926118cc565b612d1c565b8254612d01565b9055565b90612dc891612dc2612d5082611c20565b612d6584612d6060098690611bde565b612d1f565b82818590612da5612d9f612d997f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f956118cc565b926118cc565b926118cc565b92612dae610302565b80612db8816106ba565b0390a49291613aec565b91613b04565b565b612df1612dec612de7612df693612ddf611c52565b50600a6118d8565b6118ee565b613cb2565b613d31565b90565b9081612e15612e0f612e0a5f61197f565b610473565b91610473565b14612e3157612e2f9190612e285f61197f565b9091613188565b565b612e54612e3d5f61197f565b5f918291634b637e8f60e11b835260048301610929565b0390fd5b612e6a90612e646116fe565b50613d82565b90565b90565b612e7861156d565b50612ead7f0000000000000000000000000000000000000000000000000000000000000000612ea76006612e6d565b90613e9d565b90565b612eb861156d565b50612eed7f0000000000000000000000000000000000000000000000000000000000000000612ee76007612e6d565b90613e9d565b90565b612ef8611494565b50612f045f8201612aa8565b80612f17612f115f61198b565b916104a2565b145f14612f2d575050612f295f612b4b565b5b90565b612f5a5f91612f55612f4f84612f60960192612f496001612afd565b90611aa0565b91612acb565b613a53565b01612b3e565b612f2a565b612f7f90612f7161176d565b50612f7a6128b7565b613eeb565b90565b92612f9d92612fa694612f93611bda565b5092909192613fb1565b909291926140dc565b90565b916020612fca929493612fc360408201965f83019061091c565b019061052b565b565b612fd58161301e565b91612fe8612fe2846104a2565b916104a2565b03612ff1575050565b61300b5f9283926301d4b62360e61b845260048401612fa9565b0390fd5b600161301b91016104a2565b90565b6130329061302a6116fe565b506008611c6b565b61304e61303e8261171b565b916130488361300f565b90611ef2565b90565b9061307161306c613076936130646125e5565b50600a6118d8565b6118ee565b614252565b90565b90928161309661309061308b5f61197f565b610473565b91610473565b1461316157836130b66130b06130ab5f61197f565b610473565b91610473565b1461313a576130da836130d56130ce6001869061256c565b8790611c6b565b611ef2565b6130e4575b505050565b91909161312f61311d6131177f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925936118cc565b936118cc565b93613126610302565b91829182610538565b0390a35f80806130df565b61315d6131465f61197f565b5f918291634a1406b160e11b835260048301610929565b0390fd5b61318461316d5f61197f565b5f91829163e602df0560e01b835260048301610929565b0390fd5b91826131a461319e6131995f61197f565b610473565b91610473565b14158061320f575b6131bf575b6131bd92919091614273565b565b6131c7611e4f565b806131ee575b156131b1575f6336e278fd60e21b8152806131ea600482016106ba565b0390fd5b5061320a6132046131fd610d06565b339061219a565b1561035d565b6131cd565b508161322b6132256132205f61197f565b610473565b91610473565b14156131ac565b91602061325392949361324c60408201965f83019061091c565b0190610636565b565b9061326a61326483839061219a565b1561035d565b613272575050565b61328c5f92839263e2517d3f60e01b845260048401613232565b0390fd5b7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f90565b909594926132ff946132ee6132f8926132e46080966132da60a088019c5f890190610636565b6020870190610636565b6040850190610636565b606083019061052b565b019061091c565b565b61330961176d565b50613312613290565b6133897f00000000000000000000000000000000000000000000000000000000000000009161337a7f0000000000000000000000000000000000000000000000000000000000000000466133653061202e565b9161336e610302565b968795602087016132b4565b6020820181038252038261166f565b61339b613395826122ea565b916122e4565b2090565b90565b6133b66133b16133bb9261339f565b610890565b6106f3565b90565b6133c7906133a2565b9052565b9160206133ec9294936133e560408201965f8301906133be565b019061052b565b565b6134026133fd613407926104a2565b610890565b610eb0565b90565b6134126121c4565b508061342c61342665ffffffffffff612a13565b916104a2565b1161343d5761343a906133ee565b90565b60306134595f9283926306dfcc6560e41b8452600484016133cb565b0390fd5b90565b61347461346f6134799261345d565b610890565b6104a2565b90565b90565b61349361348e6134989261347c565b610890565b6106f3565b90565b6134ba906134b46134ae6134bf946106f3565b916104a2565b90611099565b6104a2565b90565b90565b6134d96134d46134de926134c2565b610890565b6106f3565b90565b1b90565b613504906134fe6134f8613509946106f3565b916104a2565b906134e1565b6104a2565b90565b90565b61352361351e6135289261350c565b610890565b6104a2565b90565b90565b61354261353d6135479261352b565b610890565b6106f3565b90565b90565b61356161355c6135669261354a565b610890565b6104a2565b90565b90565b61358061357b61358592613569565b610890565b6106f3565b90565b90565b61359f61359a6135a492613588565b610890565b6104a2565b90565b90565b6135be6135b96135c3926135a7565b610890565b6106f3565b90565b90565b6135dd6135d86135e2926135c6565b610890565b6104a2565b90565b90565b6135fc6135f7613601926135e5565b610890565b6106f3565b90565b61361861361361361d92613569565b610890565b6104a2565b90565b90565b61363761363261363c92613620565b610890565b6106f3565b90565b61365361364e613658926135e5565b610890565b6104a2565b90565b61366f61366a61367492612afa565b610890565b6106f3565b90565b90565b61368e61368961369392613677565b610890565b6104a2565b90565b906136a191026104a2565b90565b634e487b7160e01b5f52601260045260245ffd5b6136c46136ca916104a2565b916104a2565b9081156136d5570490565b6136a4565b906136e591016104a2565b90565b6136f06116fe565b50806137056136ff6001612afd565b916104a2565b1115613a50578061391a6138f76138e76138d76138c76138b76138a76138976138876138776138678b61386161385a6139209f61383a61382a61384a9261374c6001612afd565b908061376461375e600160801b613460565b916104a2565b1015613a22575b806137876137816801000000000000000061350f565b916104a2565b10156139f4575b806137a66137a064010000000061354d565b916104a2565b10156139c6575b806137c36137bd6201000061358b565b916104a2565b1015613998575b806137df6137d96101006135c9565b916104a2565b101561396a575b806137fa6137f46010613604565b916104a2565b101561393c575b61381461380e600461363f565b916104a2565b1015613923575b613825600361367a565b613696565b613834600161365b565b9061349b565b61384481866136b8565b906136da565b613854600161365b565b9061349b565b80926136b8565b906136da565b613871600161365b565b9061349b565b613881818c6136b8565b906136da565b613891600161365b565b9061349b565b6138a1818a6136b8565b906136da565b6138b1600161365b565b9061349b565b6138c181886136b8565b906136da565b6138d1600161365b565b9061349b565b6138e181866136b8565b906136da565b6138f1600161365b565b9061349b565b9161391461390e6139098580946136b8565b6104a2565b916104a2565b11614303565b9061267b565b90565b61393790613931600161365b565b906134e5565b61381b565b6139536139649161394d60046135e8565b9061349b565b9161395e6002613623565b906134e5565b90613801565b6139816139929161397b60086135aa565b9061349b565b9161398c60046135e8565b906134e5565b906137e6565b6139af6139c0916139a9601061356c565b9061349b565b916139ba60086135aa565b906134e5565b906137ca565b6139dd6139ee916139d7602061352e565b9061349b565b916139e8601061356c565b906134e5565b906137ad565b613a0b613a1c91613a0560406134c5565b9061349b565b91613a16602061352e565b906134e5565b9061378e565b613a39613a4a91613a33608061347f565b9061349b565b91613a4460406134c5565b906134e5565b9061376b565b90565b5f5260205f200190565b93919092613a696116fe565b505b81613a7e613a78836104a2565b916104a2565b1015613ae457613a8f82829061434f565b90613aa55f613a9f888590613a53565b01612aed565b613ab7613ab187610eb0565b91610eb0565b115f14613ac75750915b91613a6b565b929150613ade90613ad86001612afd565b906119a7565b90613ac1565b925050915090565b613afe90613af86116fe565b50611c81565b90565b90565b91909180613b1a613b1485610473565b91610473565b141580613c98575b613b2c575b505050565b80613b47613b41613b3c5f61197f565b610473565b91610473565b03613c08575b5081613b69613b63613b5e5f61197f565b610473565b91610473565b03613b75575b80613b27565b613bbc613baf613bb692613b8b600a86906118d8565b90613ba9613ba3613b9d6001936143e8565b936118ee565b91613b01565b9061443b565b92906118f1565b916118f1565b919091613be97fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926118cc565b92613bfe613bf5610302565b92839283611f12565b0390a25f80613b6f565b613c47613c4d613c40613c1d600a85906118d8565b6002613c3a613c34613c2e896143e8565b936118ee565b91613b01565b9061443b565b92906118f1565b916118f1565b919091613c7a7fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926118cc565b92613c8f613c86610302565b92839283611f12565b0390a25f613b4d565b5081613cac613ca65f61198b565b916104a2565b11613b22565b5f613cc691613cbf6116fe565b5001612aa8565b90565b613cdd613cd8613ce2926109a6565b610890565b6104a2565b90565b613cee9061352e565b9052565b916020613d13929493613d0c60408201965f830190613ce5565b019061052b565b565b613d29613d24613d2e926104a2565b610890565b6109a6565b90565b613d39611c52565b5080613d51613d4b63ffffffff613cc9565b916104a2565b11613d6257613d5f90613d15565b90565b6020613d7e5f9283926306dfcc6560e41b845260048401613cf2565b0390fd5b613d99613d9e91613d916116fe565b506008611c6b565b61171b565b90565b90565b613db8613db3613dbd92613da1565b610ff6565b6105f2565b90565b613dca60ff613da4565b90565b5f5260205f2090565b905f9291805490613df0613de983611586565b80946115b0565b916001811690815f14613e475750600114613e0b575b505050565b613e189192939450613dcd565b915f925b818410613e2f57505001905f8080613e06565b60018160209295939554848601520191019290613e1c565b92949550505060ff19168252151560200201905f8080613e06565b90613e6c91613dd6565b90565b90613e8f613e8892613e7f610302565b93848092613e62565b038361166f565b565b613e9a90613e6f565b90565b90613ea661156d565b50613eb082611771565b613ec9613ec3613ebe613dc0565b6105f2565b916105f2565b14155f14613ede5750613edb906144c5565b90565b613ee89150613e91565b90565b604291613ef661176d565b50604051917f19010000000000000000000000000000000000000000000000000000000000008352600283015260228201522090565b5f90565b613f3c613f4191611702565b611ed3565b90565b90565b613f5b613f56613f6092613f44565b610890565b6104a2565b90565b613f98613f9f94613f8e606094989795613f84608086019a5f870190610636565b60208501906106f9565b6040830190610636565b0190610636565b565b613fa9610302565b3d5f823e3d90fd5b939293613fbc611bda565b50613fc5613f2c565b50613fce61176d565b50613fd885613f30565b61400a6140047f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0613f47565b916104a2565b11614097579061402d602094955f94939293614024610302565b94859485613f63565b838052039060015afa15614092576140455f51610ff6565b8061406061405a6140555f61197f565b610473565b91610473565b14614076575f916140705f610ffb565b91929190565b506140805f61197f565b60019161408c5f610ffb565b91929190565b613fa1565b5050506140a35f61197f565b9060039291929190565b634e487b7160e01b5f52602160045260245ffd5b600411156140cb57565b6140ad565b906140da826140c1565b565b806140ef6140e95f6140d0565b916140d0565b145f146140fa575050565b8061410e61410860016140d0565b916140d0565b145f14614131575f63f645eedf60e01b81528061412d600482016106ba565b0390fd5b8061414561413f60026140d0565b916140d0565b145f146141735761416f61415883613f30565b5f91829163fce698f760e01b835260048301610538565b0390fd5b61418661418060036140d0565b916140d0565b1461418e5750565b6141a9905f9182916335e2f38360e21b835260048301610643565b0390fd5b634e487b7160e01b5f52603260045260245ffd5b5f5260205f2090565b6141d381612aa8565b8210156141ed576141e56001916141c1565b910201905f90565b6141ad565b906141fc90610eb0565b9052565b9061420a90611409565b9052565b9061424461423b5f61421e6125aa565b9461423561422d838301612aed565b8388016141f2565b01612b3e565b60208401614200565b565b61424f9061420e565b90565b614270915f61426a926142636125e5565b50016141ca565b50614246565b90565b9291614281848383916144f5565b8361429c6142966142915f61197f565b610473565b91610473565b146142b1575b6142af929391909161467f565b565b6142b9611728565b936142c2614664565b94806142d66142d0886104a2565b916104a2565b116142e3575093506142a2565b85906142ff5f928392630e58ae9360e11b845260048401611f12565b0390fd5b61430b6116fe565b50151590565b61432561432061432a92613620565b610890565b6104a2565b90565b61433961433f916104a2565b916104a2565b90811561434a570490565b6136a4565b61437461437a9261435e6116fe565b50828116921861436e6002614311565b9061432d565b906119a7565b90565b90565b61439461438f6143999261437d565b610890565b6106f3565b90565b6143a590614380565b9052565b9160206143ca9294936143c360408201965f83019061439c565b019061052b565b565b6143e06143db6143e5926104a2565b610890565b611409565b90565b6143f0611494565b508061440a61440460018060d01b036118f1565b916104a2565b1161441b57614418906143cc565b90565b60d06144375f9283926306dfcc6560e41b8452600484016143a9565b0390fd5b90614471614477939261444c611494565b50614455611494565b50809361446a6144636121c8565b9492612ef0565b9091614afa565b9161473e565b91909190565b61449161448c6144969261352b565b610890565b6104a2565b90565b369037565b906144c36144ab83611b22565b926020806144b98693611aff565b9201910390614499565b565b6144cd61156d565b506144d7816147a8565b906144ea6144e5602061447d565b61449e565b918252602082015290565b9190918061451361450d6145085f61197f565b610473565b91610473565b145f146145f4576145376145308361452b600261171b565b6119a7565b6002611ef2565b5b8261455361454d6145485f61197f565b610473565b91610473565b145f146145c8576145776145708361456b600261171b565b61267b565b6002611ef2565b5b9190916145c36145b16145ab7fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef936118cc565b936118cc565b936145ba610302565b91829182610538565b0390a3565b6145ef826145e96145da5f8790611c6b565b916145e48361171b565b6136da565b90611ef2565b614578565b6146076146025f8390611c6b565b61171b565b8061461a614614856104a2565b916104a2565b106146425761462d61463d91849061267b565b6146385f8490611c6b565b611ef2565b614538565b906146609091925f93849363391434e360e21b855260048501612649565b0390fd5b61466c6116fe565b5061467c60018060d01b036118f1565b90565b916146d76146d16146de94806146a561469f61469a5f61197f565b610473565b91610473565b1461470f575b846146c66146c06146bb5f61197f565b610473565b91610473565b146146e0575b611c20565b92611c20565b9091613b04565b565b614708600b60026147026146fc6146f6896143e8565b936118ee565b91613b01565b9061443b565b50506146cc565b614737600b600161473161472b614725896143e8565b936118ee565b91613b01565b9061443b565b50506146ab565b916147625f6147679461474f611494565b50614758611494565b5001929192612acb565b6149ac565b91909190565b61478161477c61478692613da1565b610890565b6104a2565b90565b90565b6147a061479b6147a592614789565b610890565b6104a2565b90565b6147bd6147c2916147b76116fe565b50611771565b613f30565b6147cc60ff61476d565b16806147e16147db601f61478c565b916104a2565b116147e95790565b5f632cd44ac360e21b815280614801600482016106ba565b0390fd5b5490565b6148136040611aea565b90565b5f5260205f2090565b61482881614805565b8210156148425761483a600191614816565b910201905f90565b6141ad565b634e487b7160e01b5f525f60045260245ffd5b6148649051610eb0565b90565b9061487865ffffffffffff91610ff6565b9181191691161790565b61489661489161489b92610eb0565b610890565b610eb0565b90565b90565b906148b66148b16148bd92614882565b61489e565b8254614867565b9055565b6148cb9051611409565b90565b60301b90565b906148e665ffffffffffff19916148ce565b9181191691161790565b6149046148ff61490992611409565b610890565b611409565b90565b90565b9061492461491f61492b926148f0565b61490c565b82546148d4565b9055565b9061495960205f61495f9461495182820161494b84880161485a565b906148a1565b0192016148c1565b9061490f565b565b9190614972576149709161492f565b565b614847565b90815491680100000000000000008310156149a7578261499f9160016149a59501815561481f565b90614961565b565b61165b565b909291926149b8611494565b506149c1611494565b506149cb82614805565b806149de6149d85f61198b565b916104a2565b115f14614aae57614a04906149fe84916149f86001612afd565b90611aa0565b90613a53565b90614a105f8301612aed565b92614a1c5f8401612b3e565b9380614a30614a2a85610eb0565b91610eb0565b11614a9257614a47614a4184610eb0565b91610eb0565b145f14614a62575050614a5d905f85910161490f565b5b9190565b614a8d9250614a8886614a7f614a76614809565b945f86016141f2565b60208401614200565b614977565b614a5e565b5f632520601d60e01b815280614aaa600482016106ba565b0390fd5b50614ad991614ad485614acb614ac2614809565b945f86016141f2565b60208401614200565b614977565b614ae25f612b4b565b9190565b634e487b7160e01b5f52605160045260245ffd5b91909180600114614b1957600203614ae657614b1591611513565b905b565b50614b23916114d4565b90614b1756
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\x80`@R4a\0\x84Wa\0\x1Ba\0\x15a\x01XV[\x90a\x03\xBDV[a\0#a\0\x89V[aK)a\x1C\x83\x829`\x80Q\x81a)\n\x01R`\xA0Q\x81a)A\x01R`\xC0Q\x81a(\xD1\x01R`\xE0Q\x81a3\x17\x01Ra\x01\0Q\x81a3<\x01Ra\x01 Q\x81a.~\x01Ra\x01@Q\x81a.\xBE\x01Ra\x01`Q\x81\x81\x81a\x0B\x10\x01Ra\x1FW\x01RaK)\x90\xF3[a\0\x8FV[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0\xBB\x90a\0\x93V[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\xD3W`@RV[a\0\x9DV[\x90a\0\xEBa\0\xE4a\0\x89V[\x92\x83a\0\xB1V[V[_\x80\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x01\x05\x90a\0\xF1V[\x90V[a\x01\x11\x81a\0\xFCV[\x03a\x01\x18WV[_\x80\xFD[\x90PQ\x90a\x01)\x82a\x01\x08V[V[\x91\x90`@\x83\x82\x03\x12a\x01SW\x80a\x01Ga\x01P\x92_\x86\x01a\x01\x1CV[\x93` \x01a\x01\x1CV[\x90V[a\0\xEDV[a\x01vag\xAC\x808\x03\x80a\x01k\x81a\0\xD8V[\x92\x839\x81\x01\x90a\x01+V[\x90\x91V[`\x01\x80`@\x1B\x03\x81\x11a\x01\x96Wa\x01\x92` \x91a\0\x93V[\x01\x90V[a\0\x9DV[\x90a\x01\xADa\x01\xA8\x83a\x01zV[a\0\xD8V[\x91\x82RV[_\x7FSyndicate\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x01\xE3`\ta\x01\x9BV[\x90a\x01\xF0` \x83\x01a\x01\xB2V[V[a\x01\xFAa\x01\xD9V[\x90V[_\x7FSYND\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x02.`\x04a\x01\x9BV[\x90a\x02;` \x83\x01a\x01\xFDV[V[a\x02Ea\x02$V[\x90V[\x90V[\x90V[a\x02ba\x02]a\x02g\x92a\x02HV[a\x02KV[a\0\xF1V[\x90V[a\x02s\x90a\x02NV[\x90V[_\x01\x90V[\x90V[\x90V[a\x02\x95a\x02\x90a\x02\x9A\x92a\x02{V[a\x02KV[a\x02~V[\x90V[a\x02\xA9bv\xA7\0a\x02\x81V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x02\xCFa\x02\xD5\x91\x93\x92\x93a\x02~V[\x92a\x02~V[\x82\x01\x80\x92\x11a\x02\xE0WV[a\x02\xACV[a\x02\xF9a\x02\xF4a\x02\xFE\x92a\x02HV[a\x02KV[a\x02~V[\x90V[_\x1B\x90V[\x90a\x03\x12_\x19\x91a\x03\x01V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x030a\x03+a\x035\x92a\x02~V[a\x02KV[a\x02~V[\x90V[\x90V[\x90a\x03Pa\x03Ka\x03W\x92a\x03\x1CV[a\x038V[\x82Ta\x03\x06V[\x90UV[\x90V[a\x03ra\x03ma\x03w\x92a\x02HV[a\x03\x01V[a\x03[V[\x90V[a\x03\x83_a\x03^V[\x90V[\x90V[a\x03\x9Da\x03\x98a\x03\xA2\x92a\x03\x86V[a\x02KV[a\x02~V[\x90V[a\x03\xBAk\x02\xE8vi\xC3\x08sj\x04\0\0\0a\x03\x89V[\x90V[\x90a\x03\xDFa\x03\xC9a\x01\xF2V[a\x03\xD1a\x01\xF2V[a\x03\xD9a\x02=V[\x91a\x04\xA5V[\x81a\x03\xFAa\x03\xF4a\x03\xEF_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x04\x89W\x80a\x04\x1Aa\x04\x14a\x04\x0F_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x04mWa\x04\\a\x04k\x92a\x048Ba\x042a\x02\x9DV[\x90a\x02\xC0V[a\x01`Ra\x04Oa\x04H_a\x02\xE5V[`\x0Ca\x03;V[a\x04Wa\x03zV[a\tAV[Pa\x04ea\x03\xA5V[\x90a\n\x0FV[V[_c\xD9.#=`\xE0\x1B\x81R\x80a\x04\x85`\x04\x82\x01a\x02vV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x04\xA1`\x04\x82\x01a\x02vV[\x03\x90\xFD[\x90a\x04\xB0\x92\x91a\x04\xB2V[V[\x90a\x04\xBD\x92\x91a\x04\xBFV[V[\x90a\x04\xCA\x92\x91a\x04\xCCV[V[\x90a\x04\xD7\x92\x91a\x04\xD9V[V[\x90a\x04\xE4\x92\x91a\x051V[V[_\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x05\x17`\x01a\x01\x9BV[\x90a\x05$` \x83\x01a\x04\xE6V[V[a\x05.a\x05\rV[\x90V[\x90a\x05E\x92\x91a\x05?a\x05&V[\x90a\x05GV[V[\x90a\x05S\x93\x92\x91a\x05\x99V[V[\x90V[\x90V[` \x01\x90V[Q\x90V[a\x05ya\x05ta\x05~\x92a\0\xF1V[a\x02KV[a\0\xF1V[\x90V[a\x05\x8A\x90a\x05eV[\x90V[a\x05\x96\x90a\x05\x81V[\x90V[a\x05\xAAa\x05\xFA\x94a\x05\xDF\x93\x94a\x06.V[a\x05\xBE\x81a\x05\xB8`\x06a\x05UV[\x90a\n\xBCV[a\x01 Ra\x05\xD6\x83a\x05\xD0`\x07a\x05UV[\x90a\n\xBCV[a\x01@Ra\x05XV[a\x05\xF1a\x05\xEB\x82a\x05aV[\x91a\x05[V[ `\xE0Ra\x05XV[a\x06\x0Ca\x06\x06\x82a\x05aV[\x91a\x05[V[ a\x01\0RF`\xA0Ra\x06\x1Da\x0B\xC1V[`\x80Ra\x06)0a\x05\x8DV[`\xC0RV[\x90a\x068\x91a\x06:V[V[\x90a\x06D\x91a\x06FV[V[\x90a\x06P\x91a\x08\x97V[V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[Q\x90V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x01`\x02\x83\x04\x92\x16\x80\x15a\x06\x9DW[` \x83\x10\x14a\x06\x98WV[a\x06iV[\x91`\x7F\x16\x91a\x06\x8DV[_R` _ \x90V[`\x1F` \x91\x01\x04\x90V[\x1B\x90V[\x91\x90`\x08a\x06\xD9\x91\x02\x91a\x06\xD3_\x19\x84a\x06\xBAV[\x92a\x06\xBAV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x91\x90a\x06\xF9a\x06\xF4a\x07\x01\x93a\x03\x1CV[a\x038V[\x90\x83Ta\x06\xBEV[\x90UV[_\x90V[a\x07\x1B\x91a\x07\x15a\x07\x05V[\x91a\x06\xE3V[V[[\x81\x81\x10a\x07)WPPV[\x80a\x076_`\x01\x93a\x07\tV[\x01a\x07\x1EV[\x91\x90`\x1F\x81\x11a\x07LW[PPPV[a\x07Xa\x07}\x93a\x06\xA7V[\x90` a\x07d\x84a\x06\xB0V[\x83\x01\x93\x10a\x07\x85W[a\x07v\x90a\x06\xB0V[\x01\x90a\x07\x1DV[_\x80\x80a\x07GV[\x91Pa\x07v\x81\x92\x90Pa\x07mV[\x1C\x90V[\x90a\x07\xA7\x90_\x19\x90`\x08\x02a\x07\x93V[\x19\x16\x90V[\x81a\x07\xB6\x91a\x07\x97V[\x90`\x02\x02\x17\x90V[\x90a\x07\xC8\x81a\x06eV[\x90`\x01\x80`@\x1B\x03\x82\x11a\x08\x86Wa\x07\xEA\x82a\x07\xE4\x85Ta\x06}V[\x85a\x07<V[` \x90`\x1F\x83\x11`\x01\x14a\x08\x1EW\x91\x80\x91a\x08\r\x93_\x92a\x08\x12W[PPa\x07\xACV[\x90U[V[\x90\x91P\x01Q_\x80a\x08\x06V[`\x1F\x19\x83\x16\x91a\x08-\x85a\x06\xA7V[\x92_[\x81\x81\x10a\x08nWP\x91`\x02\x93\x91\x85`\x01\x96\x94\x10a\x08TW[PPP\x02\x01\x90Ua\x08\x10V[a\x08d\x91\x01Q`\x1F\x84\x16\x90a\x07\x97V[\x90U_\x80\x80a\x08HV[\x91\x93` `\x01\x81\x92\x87\x87\x01Q\x81U\x01\x95\x01\x92\x01a\x080V[a\0\x9DV[\x90a\x08\x95\x91a\x07\xBEV[V[\x90a\x08\xA6a\x08\xAD\x92`\x03a\x08\x8BV[`\x04a\x08\x8BV[V[_\x90V[\x15\x15\x90V[a\x08\xC1\x90a\x03[V[\x90V[\x90a\x08\xCE\x90a\x08\xB8V[_R` R`@_ \x90V[a\x08\xE3\x90a\x05\x81V[\x90V[\x90a\x08\xF0\x90a\x08\xDAV[_R` R`@_ \x90V[\x90a\t\x08`\xFF\x91a\x03\x01V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\t\x1B\x90a\x08\xB3V[\x90V[\x90V[\x90a\t6a\t1a\t=\x92a\t\x12V[a\t\x1EV[\x82Ta\x08\xFCV[\x90UV[a\tIa\x08\xAFV[Pa\t^a\tX\x82\x84\x90a\x0C^V[\x15a\x08\xB3V[_\x14a\t\xE7Wa\t\x86`\x01a\t\x81_a\ty`\x05\x86\x90a\x08\xC4V[\x01\x85\x90a\x08\xE6V[a\t!V[\x90a\t\x8Fa\x0C\x8CV[\x90a\t\xCCa\t\xC6a\t\xC0\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x95a\x08\xB8V[\x92a\x08\xDAV[\x92a\x08\xDAV[\x92a\t\xD5a\0\x89V[\x80a\t\xDF\x81a\x02vV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a\t\xF6\x90a\0\xFCV[\x90RV[\x91\x90a\n\r\x90_` \x85\x01\x94\x01\x90a\t\xEDV[V[\x80a\n*a\n$a\n\x1F_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\nFWa\nD\x91a\n<_a\x02jV[\x91\x90\x91a\x0C\xBDV[V[a\nia\nR_a\x02jV[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\t\xFAV[\x03\x90\xFD[_\x90V[\x90V[a\n\x88a\n\x83a\n\x8D\x92a\nqV[a\x02KV[a\x02~V[\x90V[\x90V[a\n\xA7a\n\xA2a\n\xAC\x92a\n\x90V[a\x03\x01V[a\x03[V[\x90V[a\n\xB9`\xFFa\n\x93V[\x90V[\x90a\n\xC5a\nmV[Pa\n\xD7a\n\xD2\x83a\x05XV[a\x05aV[a\n\xEAa\n\xE4` a\ntV[\x91a\x02~V[\x10_\x14a\n\xFEWPa\n\xFB\x90a\x0EWV[\x90V[_a\x0B\x0Ca\x0B\x12\x93\x92a\rgV[\x01a\x08\x8BV[a\x0B\"a\x0B\x1Da\n\xAFV[a\x08\xB8V[\x90V[_\x90V[\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x90V[a\x0BW\x90Qa\x03[V[\x90V[a\x0Bc\x90a\x03[V[\x90RV[a\x0Bp\x90a\x02~V[\x90RV[\x90\x95\x94\x92a\x0B\xBF\x94a\x0B\xAEa\x0B\xB8\x92a\x0B\xA4`\x80\x96a\x0B\x9A`\xA0\x88\x01\x9C_\x89\x01\x90a\x0BZV[` \x87\x01\x90a\x0BZV[`@\x85\x01\x90a\x0BZV[``\x83\x01\x90a\x0BgV[\x01\x90a\t\xEDV[V[a\x0B\xC9a\x0B%V[Pa\x0B\xD2a\x0B)V[a\x0C\x1Ca\x0B\xDF`\xE0a\x0BMV[\x91a\x0C\ra\x0B\xEEa\x01\0a\x0BMV[Fa\x0B\xF80a\x05\x8DV[\x91a\x0C\x01a\0\x89V[\x96\x87\x95` \x87\x01a\x0BtV[` \x82\x01\x81\x03\x82R\x03\x82a\0\xB1V[a\x0C.a\x0C(\x82a\x05aV[\x91a\x05[V[ \x90V[_\x1C\x90V[`\xFF\x16\x90V[a\x0CIa\x0CN\x91a\x0C2V[a\x0C7V[\x90V[a\x0C[\x90Ta\x0C=V[\x90V[a\x0C\x85\x91_a\x0Cza\x0C\x80\x93a\x0Cra\x08\xAFV[P`\x05a\x08\xC4V[\x01a\x08\xE6V[a\x0CQV[\x90V[_\x90V[a\x0C\x94a\x0C\x88V[P3\x90V[\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t\x90V[\x91\x82a\x0C\xD9a\x0C\xD3a\x0C\xCE_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14\x15\x80a\rDW[a\x0C\xF4W[a\x0C\xF2\x92\x91\x90\x91a\x0F{V[V[a\x0C\xFCa\x0F\x05V[\x80a\r#W[\x15a\x0C\xE6W_c6\xE2x\xFD`\xE2\x1B\x81R\x80a\r\x1F`\x04\x82\x01a\x02vV[\x03\x90\xFD[Pa\r?a\r9a\r2a\x0C\x99V[3\x90a\x0C^V[\x15a\x08\xB3V[a\r\x02V[P\x81a\r`a\rZa\rU_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14\x15a\x0C\xE1V[\x90V[\x90V[a\r\x81a\r|a\r\x86\x92a\rjV[a\x02KV[a\x02~V[\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[a\r\xBCa\r\xC5` \x93a\r\xCA\x93a\r\xB3\x81a\x06eV[\x93\x84\x80\x93a\r\x89V[\x95\x86\x91\x01a\r\x92V[a\0\x93V[\x01\x90V[a\r\xE3\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\r\x9DV[\x90V[a\x0E\0a\r\xFBa\r\xF5\x83a\x05aV[\x92a\x05[V[a\x0BMV[\x90` \x81\x10a\x0E\x0EW[P\x90V[a\x0E \x90_\x19\x90` \x03`\x08\x02a\x06\xBAV[\x16_a\x0E\nV[a\x0E3a\x0E8\x91a\x0C2V[a\x03\x1CV[\x90V[a\x0EOa\x0EJa\x0ET\x92a\x02~V[a\x03\x01V[a\x03[V[\x90V[a\x0E_a\nmV[Pa\x0Ei\x81a\x05XV[\x90a\x0Es\x82a\x05aV[a\x0E\x86a\x0E\x80`\x1Fa\rmV[\x91a\x02~V[\x11a\x0E\xBBWPa\x0E\xB3\x81a\x0E\xADa\x0E\xA7a\x0E\xA2a\x0E\xB8\x95a\r\xE6V[a\x0E'V[\x91a\x05aV[\x17a\x0E;V[a\x08\xB8V[\x90V[a\x0E\xDD\x90a\x0E\xC7a\0\x89V[\x91\x82\x91c0Z'\xA9`\xE0\x1B\x83R`\x04\x83\x01a\r\xCEV[\x03\x90\xFD[\x90V[a\x0E\xF0a\x0E\xF5\x91a\x0C2V[a\x0E\xE1V[\x90V[a\x0F\x02\x90Ta\x0E\xE4V[\x90V[a\x0F\ra\x08\xAFV[Pa\x0F\x18`\x0Ca\x0E\xF8V[a\x0F*a\x0F$_a\x02\xE5V[\x91a\x02~V[\x14\x15\x80a\x0F5W[\x90V[PBa\x0FRa\x0FLa\x0FG`\x0Ca\x0E\xF8V[a\x02~V[\x91a\x02~V[\x10a\x0F2V[\x91` a\x0Fy\x92\x94\x93a\x0Fr`@\x82\x01\x96_\x83\x01\x90a\x0BgV[\x01\x90a\x0BgV[V[\x92\x91a\x0F\x89\x84\x83\x83\x91a\x10\x84V[\x83a\x0F\xA4a\x0F\x9Ea\x0F\x99_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x0F\xB9W[a\x0F\xB7\x92\x93\x91\x90\x91a\x12QV[V[a\x0F\xC1a\x11\xF3V[\x93a\x0F\xCAa\x120V[\x94\x80a\x0F\xDEa\x0F\xD8\x88a\x02~V[\x91a\x02~V[\x11a\x0F\xEBWP\x93Pa\x0F\xAAV[\x85\x90a\x10\x07_\x92\x83\x92c\x0EX\xAE\x93`\xE1\x1B\x84R`\x04\x84\x01a\x0FXV[\x03\x90\xFD[\x90a\x10\x15\x90a\x08\xDAV[_R` R`@_ \x90V[`@\x90a\x10Ja\x10Q\x94\x96\x95\x93\x96a\x10@``\x84\x01\x98_\x85\x01\x90a\t\xEDV[` \x83\x01\x90a\x0BgV[\x01\x90a\x0BgV[V[\x90a\x10^\x91\x03a\x02~V[\x90V[\x90a\x10l\x91\x01a\x02~V[\x90V[\x91\x90a\x10\x82\x90_` \x85\x01\x94\x01\x90a\x0BgV[V[\x91\x90\x91\x80a\x10\xA2a\x10\x9Ca\x10\x97_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14_\x14a\x11\x83Wa\x10\xC6a\x10\xBF\x83a\x10\xBA`\x02a\x0E\xF8V[a\x02\xC0V[`\x02a\x03;V[[\x82a\x10\xE2a\x10\xDCa\x10\xD7_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14_\x14a\x11WWa\x11\x06a\x10\xFF\x83a\x10\xFA`\x02a\x0E\xF8V[a\x10SV[`\x02a\x03;V[[\x91\x90\x91a\x11Ra\x11@a\x11:\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93a\x08\xDAV[\x93a\x08\xDAV[\x93a\x11Ia\0\x89V[\x91\x82\x91\x82a\x10oV[\x03\x90\xA3V[a\x11~\x82a\x11xa\x11i_\x87\x90a\x10\x0BV[\x91a\x11s\x83a\x0E\xF8V[a\x10aV[\x90a\x03;V[a\x11\x07V[a\x11\x96a\x11\x91_\x83\x90a\x10\x0BV[a\x0E\xF8V[\x80a\x11\xA9a\x11\xA3\x85a\x02~V[\x91a\x02~V[\x10a\x11\xD1Wa\x11\xBCa\x11\xCC\x91\x84\x90a\x10SV[a\x11\xC7_\x84\x90a\x10\x0BV[a\x03;V[a\x10\xC7V[\x90a\x11\xEF\x90\x91\x92_\x93\x84\x93c9\x144\xE3`\xE2\x1B\x85R`\x04\x85\x01a\x10!V[\x03\x90\xFD[a\x11\xFBa\x07\x05V[Pa\x12\x06`\x02a\x0E\xF8V[\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x12(a\x12#a\x12-\x92a\x12\tV[a\x02KV[a\x02~V[\x90V[a\x128a\x07\x05V[Pa\x12H`\x01\x80`\xD0\x1B\x03a\x12\x14V[\x90V[\x90V[\x90V[\x91a\x12\xA9a\x12\xA3a\x12\xB0\x94\x80a\x12wa\x12qa\x12l_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x12\xE1W[\x84a\x12\x98a\x12\x92a\x12\x8D_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x12\xB2W[a\x14\xD9V[\x92a\x14\xD9V[\x90\x91a\x15\x0EV[V[a\x12\xDA`\x0B`\x02a\x12\xD4a\x12\xCEa\x12\xC8\x89a\x13\xC3V[\x93a\x12KV[\x91a\x12NV[\x90a\x14\x16V[PPa\x12\x9EV[a\x13\t`\x0B`\x01a\x13\x03a\x12\xFDa\x12\xF7\x89a\x13\xC3V[\x93a\x12KV[\x91a\x12NV[\x90a\x14\x16V[PPa\x12}V[_\x90V[a\x13 a\x13&\x91a\x12\tV[\x91a\x12\tV[\x01\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x137WV[a\x02\xACV[\x90a\x13O\x91a\x13Ia\x13\x10V[Pa\x13\x14V[\x90V[\x90V[`\xFF\x16\x90V[a\x13oa\x13ja\x13t\x92a\x13RV[a\x02KV[a\x13UV[\x90V[a\x13\x80\x90a\x13[V[\x90RV[\x91` a\x13\xA5\x92\x94\x93a\x13\x9E`@\x82\x01\x96_\x83\x01\x90a\x13wV[\x01\x90a\x0BgV[V[a\x13\xBBa\x13\xB6a\x13\xC0\x92a\x02~V[a\x02KV[a\x12\tV[\x90V[a\x13\xCBa\x13\x10V[P\x80a\x13\xE5a\x13\xDF`\x01\x80`\xD0\x1B\x03a\x12\x14V[\x91a\x02~V[\x11a\x13\xF6Wa\x13\xF3\x90a\x13\xA7V[\x90V[`\xD0a\x14\x12_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a\x13\x84V[\x03\x90\xFD[\x90a\x14La\x14R\x93\x92a\x14'a\x13\x10V[Pa\x140a\x13\x10V[P\x80\x93a\x14Ea\x14>a\x16\xC0V[\x94\x92a\x17mV[\x90\x91a\x1CSV[\x91a\x17\xE2V[\x91\x90\x91\x90V[a\x14da\x14j\x91a\x12\tV[\x91a\x12\tV[\x90\x03\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x14|WV[a\x02\xACV[\x90a\x14\x94\x91a\x14\x8Ea\x13\x10V[Pa\x14XV[\x90V[\x90a\x14\xA1\x90a\x08\xDAV[_R` R`@_ \x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x14\xC4a\x14\xC9\x91a\x0C2V[a\x14\xADV[\x90V[a\x14\xD6\x90Ta\x14\xB8V[\x90V[a\x14\xF0a\x14\xF5\x91a\x14\xE8a\x0C\x88V[P`\ta\x14\x97V[a\x14\xCCV[\x90V[\x90a\x15\x02\x90a\x08\xDAV[_R` R`@_ \x90V[\x91\x90\x91\x80a\x15$a\x15\x1E\x85a\0\xFCV[\x91a\0\xFCV[\x14\x15\x80a\x16\xA2W[a\x156W[PPPV[\x80a\x15Qa\x15Ka\x15F_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x03a\x16\x12W[P\x81a\x15sa\x15ma\x15h_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x03a\x15\x7FW[\x80a\x151V[a\x15\xC6a\x15\xB9a\x15\xC0\x92a\x15\x95`\n\x86\x90a\x14\xF8V[\x90a\x15\xB3a\x15\xADa\x15\xA7`\x01\x93a\x13\xC3V[\x93a\x12KV[\x91a\x12NV[\x90a\x14\x16V[\x92\x90a\x12\x14V[\x91a\x12\x14V[\x91\x90\x91a\x15\xF3\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x08\xDAV[\x92a\x16\x08a\x15\xFFa\0\x89V[\x92\x83\x92\x83a\x0FXV[\x03\x90\xA2_\x80a\x15yV[a\x16Qa\x16Wa\x16Ja\x16'`\n\x85\x90a\x14\xF8V[`\x02a\x16Da\x16>a\x168\x89a\x13\xC3V[\x93a\x12KV[\x91a\x12NV[\x90a\x14\x16V[\x92\x90a\x12\x14V[\x91a\x12\x14V[\x91\x90\x91a\x16\x84\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x08\xDAV[\x92a\x16\x99a\x16\x90a\0\x89V[\x92\x83\x92\x83a\x0FXV[\x03\x90\xA2_a\x15WV[P\x81a\x16\xB6a\x16\xB0_a\x02\xE5V[\x91a\x02~V[\x11a\x15,V[_\x90V[a\x16\xC8a\x16\xBCV[Pa\x16\xD1a\x18\x11V[\x90V[T\x90V[\x90V[a\x16\xEFa\x16\xEAa\x16\xF4\x92a\x16\xD8V[a\x02KV[a\x02~V[\x90V[a\x17\x06a\x17\x0C\x91\x93\x92\x93a\x02~V[\x92a\x02~V[\x82\x03\x91\x82\x11a\x17\x17WV[a\x02\xACV[\x90V[`0\x1C\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x17<a\x17A\x91a\x17\x1FV[a\x17%V[\x90V[a\x17N\x90Ta\x170V[\x90V[a\x17ea\x17`a\x17j\x92a\x02HV[a\x02KV[a\x12\tV[\x90V[a\x17ua\x13\x10V[Pa\x17\x81_\x82\x01a\x16\xD4V[\x80a\x17\x94a\x17\x8E_a\x02\xE5V[\x91a\x02~V[\x14_\x14a\x17\xAAWPPa\x17\xA6_a\x17QV[[\x90V[a\x17\xD7_\x91a\x17\xD2a\x17\xCC\x84a\x17\xDD\x96\x01\x92a\x17\xC6`\x01a\x16\xDBV[\x90a\x16\xF7V[\x91a\x17\x1CV[a\x18&V[\x01a\x17DV[a\x17\xA7V[\x91a\x18\x06_a\x18\x0B\x94a\x17\xF3a\x13\x10V[Pa\x17\xFCa\x13\x10V[P\x01\x92\x91\x92a\x17\x1CV[a\x1A+V[\x91\x90\x91\x90V[a\x18\x19a\x16\xBCV[Pa\x18#Ca\x1B\xECV[\x90V[_R` _ \x01\x90V[T\x90V[a\x18>`@a\0\xD8V[\x90V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90a\x18V\x90a\x18AV[\x90RV[\x90a\x18d\x90a\x12\tV[\x90RV[_R` _ \x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[a\x18\x8E\x81a\x180V[\x82\x10\x15a\x18\xA8Wa\x18\xA0`\x01\x91a\x18hV[\x91\x02\x01\x90_\x90V[a\x18qV[a\x18\xB7\x90Qa\x18AV[\x90V[\x90a\x18\xCBe\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x03\x01V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x18\xE9a\x18\xE4a\x18\xEE\x92a\x18AV[a\x02KV[a\x18AV[\x90V[\x90V[\x90a\x19\ta\x19\x04a\x19\x10\x92a\x18\xD5V[a\x18\xF1V[\x82Ta\x18\xBAV[\x90UV[a\x19\x1E\x90Qa\x12\tV[\x90V[`0\x1B\x90V[\x90a\x199e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91a\x19!V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x19Wa\x19Ra\x19\\\x92a\x12\tV[a\x02KV[a\x12\tV[\x90V[\x90V[\x90a\x19wa\x19ra\x19~\x92a\x19CV[a\x19_V[\x82Ta\x19'V[\x90UV[\x90a\x19\xAC` _a\x19\xB2\x94a\x19\xA4\x82\x82\x01a\x19\x9E\x84\x88\x01a\x18\xADV[\x90a\x18\xF4V[\x01\x92\x01a\x19\x14V[\x90a\x19bV[V[\x91\x90a\x19\xC5Wa\x19\xC3\x91a\x19\x82V[V[a\x06RV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x19\xFAW\x82a\x19\xF2\x91`\x01a\x19\xF8\x95\x01\x81Ua\x18\x85V[\x90a\x19\xB4V[V[a\0\x9DV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x1A\x16a\x1A\x1B\x91a\x0C2V[a\x19\xFFV[\x90V[a\x1A(\x90Ta\x1A\nV[\x90V[\x90\x92\x91\x92a\x1A7a\x13\x10V[Pa\x1A@a\x13\x10V[Pa\x1AJ\x82a\x180V[\x80a\x1A]a\x1AW_a\x02\xE5V[\x91a\x02~V[\x11_\x14a\x1B-Wa\x1A\x83\x90a\x1A}\x84\x91a\x1Aw`\x01a\x16\xDBV[\x90a\x16\xF7V[\x90a\x18&V[\x90a\x1A\x8F_\x83\x01a\x1A\x1EV[\x92a\x1A\x9B_\x84\x01a\x17DV[\x93\x80a\x1A\xAFa\x1A\xA9\x85a\x18AV[\x91a\x18AV[\x11a\x1B\x11Wa\x1A\xC6a\x1A\xC0\x84a\x18AV[\x91a\x18AV[\x14_\x14a\x1A\xE1WPPa\x1A\xDC\x90_\x85\x91\x01a\x19bV[[\x91\x90V[a\x1B\x0C\x92Pa\x1B\x07\x86a\x1A\xFEa\x1A\xF5a\x184V[\x94_\x86\x01a\x18LV[` \x84\x01a\x18ZV[a\x19\xCAV[a\x1A\xDDV[_c% `\x1D`\xE0\x1B\x81R\x80a\x1B)`\x04\x82\x01a\x02vV[\x03\x90\xFD[Pa\x1BX\x91a\x1BS\x85a\x1BJa\x1BAa\x184V[\x94_\x86\x01a\x18LV[` \x84\x01a\x18ZV[a\x19\xCAV[a\x1Ba_a\x17QV[\x91\x90V[a\x1Bya\x1Bta\x1B~\x92a\x18AV[a\x02KV[a\x02~V[\x90V[\x90V[a\x1B\x98a\x1B\x93a\x1B\x9D\x92a\x1B\x81V[a\x02KV[a\x13UV[\x90V[a\x1B\xA9\x90a\x1B\x84V[\x90RV[\x91` a\x1B\xCE\x92\x94\x93a\x1B\xC7`@\x82\x01\x96_\x83\x01\x90a\x1B\xA0V[\x01\x90a\x0BgV[V[a\x1B\xE4a\x1B\xDFa\x1B\xE9\x92a\x02~V[a\x02KV[a\x18AV[\x90V[a\x1B\xF4a\x16\xBCV[P\x80a\x1C\x0Ea\x1C\x08e\xFF\xFF\xFF\xFF\xFF\xFFa\x1BeV[\x91a\x02~V[\x11a\x1C\x1FWa\x1C\x1C\x90a\x1B\xD0V[\x90V[`0a\x1C;_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a\x1B\xADV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`Q`\x04R`$_\xFD[\x91\x90\x91\x80`\x01\x14a\x1CrW`\x02\x03a\x1C?Wa\x1Cn\x91a\x14\x81V[\x90[V[Pa\x1C|\x91a\x13<V[\x90a\x1CpV\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x14\x90V[a\0\x1D_5a\x02\xFCV[\x80c\x01\xFF\xC9\xA7\x14a\x02\xF7W\x80c\x06\xFD\xDE\x03\x14a\x02\xF2W\x80c\t^\xA7\xB3\x14a\x02\xEDW\x80c\x18\x16\r\xDD\x14a\x02\xE8W\x80c#\xB8r\xDD\x14a\x02\xE3W\x80c$\x8A\x9C\xA3\x14a\x02\xDEW\x80c//\xF1]\x14a\x02\xD9W\x80c1<\xE5g\x14a\x02\xD4W\x80c6D\xE5\x15\x14a\x02\xCFW\x80c6V\x8A\xBE\x14a\x02\xCAW\x80c:F\xB1\xA8\x14a\x02\xC5W\x80c@\xC1\x0F\x19\x14a\x02\xC0W\x80cK\xDD6\xCE\x14a\x02\xBBW\x80cK\xF5\xD7\xE9\x14a\x02\xB6W\x80cO\x1B\xFC\x9E\x14a\x02\xB1W\x80cX|\xDE\x1E\x14a\x02\xACW\x80c\\\x19\xA9\\\x14a\x02\xA7W\x80co\xCF\xFFE\x14a\x02\xA2W\x80cp\xA0\x821\x14a\x02\x9DW\x80cy\xCCg\x90\x14a\x02\x98W\x80cz\x8C\xD1V\x14a\x02\x93W\x80c~\xCE\xBE\0\x14a\x02\x8EW\x80c\x83\xF1!\x1B\x14a\x02\x89W\x80c\x84&\xAD\xF2\x14a\x02\x84W\x80c\x84L\x90&\x14a\x02\x7FW\x80c\x84\xB0\x19n\x14a\x02zW\x80c\x8AT%!\x14a\x02uW\x80c\x8D3C\xD6\x14a\x02pW\x80c\x8ES\x9E\x8C\x14a\x02kW\x80c\x90-U\xA5\x14a\x02fW\x80c\x91\xD1HT\x14a\x02aW\x80c\x91\xDD\xAD\xF4\x14a\x02\\W\x80c\x95\xD8\x9BA\x14a\x02WW\x80c\x9A\xB2N\xB0\x14a\x02RW\x80c\x9B~\xF6K\x14a\x02MW\x80c\xA2\x17\xFD\xDF\x14a\x02HW\x80c\xA9\x05\x9C\xBB\x14a\x02CW\x80c\xAA\x08*\x9D\x14a\x02>W\x80c\xB0\xCA%>\x14a\x029W\x80c\xBBMD6\x14a\x024W\x80c\xC0*\xE7T\x14a\x02/W\x80c\xC3\xCD\xA5 \x14a\x02*W\x80c\xD5\x05\xAC\xCF\x14a\x02%W\x80c\xD5Gt\x1F\x14a\x02 W\x80c\xDDb\xED>\x14a\x02\x1BWc\xF1\x12~\xD8\x03a\0\x0EWa\x14ZV[a\x13vV[a\x13\x15V[a\x12\xDBV[a\x121V[a\x11uV[a\x11@V[a\x11\nV[a\x10\xD5V[a\x10cV[a\x10.V[a\x0F\xBEV[a\x0FGV[a\x0F\x12V[a\x0E\xDDV[a\x0EzV[a\x0EEV[a\r\xCEV[a\r\x99V[a\r5V[a\x0C\xCAV[a\x0B\x85V[a\x0B2V[a\n\xD9V[a\n\xA4V[a\noV[a\n;V[a\n\x06V[a\t\xD1V[a\tsV[a\t>V[a\x08\xC9V[a\x08XV[a\x08#V[a\x07\xEFV[a\x07\xB9V[a\x07\x85V[a\x07PV[a\x07\x1BV[a\x06\xBFV[a\x06XV[a\x05\xBCV[a\x05MV[a\x04\xF5V[a\x043V[a\x03\x84V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[c\xFF\xFF\xFF\xFF`\xE0\x1B\x16\x90V[a\x03%\x81a\x03\x10V[\x03a\x03,WV[_\x80\xFD[\x90P5\x90a\x03=\x82a\x03\x1CV[V[\x90` \x82\x82\x03\x12a\x03XWa\x03U\x91_\x01a\x030V[\x90V[a\x03\x0CV[\x15\x15\x90V[a\x03k\x90a\x03]V[\x90RV[\x91\x90a\x03\x82\x90_` \x85\x01\x94\x01\x90a\x03bV[V[4a\x03\xB4Wa\x03\xB0a\x03\x9Fa\x03\x9A6`\x04a\x03?V[a\x15-V[a\x03\xA7a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[_\x91\x03\x12a\x03\xC3WV[a\x03\x0CV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x04\ta\x04\x12` \x93a\x04\x17\x93a\x04\0\x81a\x03\xC8V[\x93\x84\x80\x93a\x03\xCCV[\x95\x86\x91\x01a\x03\xD5V[a\x03\xE0V[\x01\x90V[a\x040\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03\xEAV[\x90V[4a\x04cWa\x04C6`\x04a\x03\xB9V[a\x04_a\x04Na\x16\xC6V[a\x04Va\x03\x02V[\x91\x82\x91\x82a\x04\x1BV[\x03\x90\xF3[a\x03\x08V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04|\x90a\x04hV[\x90V[a\x04\x88\x81a\x04sV[\x03a\x04\x8FWV[_\x80\xFD[\x90P5\x90a\x04\xA0\x82a\x04\x7FV[V[\x90V[a\x04\xAE\x81a\x04\xA2V[\x03a\x04\xB5WV[_\x80\xFD[\x90P5\x90a\x04\xC6\x82a\x04\xA5V[V[\x91\x90`@\x83\x82\x03\x12a\x04\xF0W\x80a\x04\xE4a\x04\xED\x92_\x86\x01a\x04\x93V[\x93` \x01a\x04\xB9V[\x90V[a\x03\x0CV[4a\x05&Wa\x05\"a\x05\x11a\x05\x0B6`\x04a\x04\xC8V[\x90a\x16\xDCV[a\x05\x19a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[a\x054\x90a\x04\xA2V[\x90RV[\x91\x90a\x05K\x90_` \x85\x01\x94\x01\x90a\x05+V[V[4a\x05}Wa\x05]6`\x04a\x03\xB9V[a\x05ya\x05ha\x17(V[a\x05pa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90\x91``\x82\x84\x03\x12a\x05\xB7Wa\x05\xB4a\x05\x9D\x84_\x85\x01a\x04\x93V[\x93a\x05\xAB\x81` \x86\x01a\x04\x93V[\x93`@\x01a\x04\xB9V[\x90V[a\x03\x0CV[4a\x05\xEDWa\x05\xE9a\x05\xD8a\x05\xD26`\x04a\x05\x82V[\x91a\x17>V[a\x05\xE0a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[\x90V[a\x05\xFE\x81a\x05\xF2V[\x03a\x06\x05WV[_\x80\xFD[\x90P5\x90a\x06\x16\x82a\x05\xF5V[V[\x90` \x82\x82\x03\x12a\x061Wa\x06.\x91_\x01a\x06\tV[\x90V[a\x03\x0CV[a\x06?\x90a\x05\xF2V[\x90RV[\x91\x90a\x06V\x90_` \x85\x01\x94\x01\x90a\x066V[V[4a\x06\x88Wa\x06\x84a\x06sa\x06n6`\x04a\x06\x18V[a\x17\xB7V[a\x06{a\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[\x91\x90`@\x83\x82\x03\x12a\x06\xB5W\x80a\x06\xA9a\x06\xB2\x92_\x86\x01a\x06\tV[\x93` \x01a\x04\x93V[\x90V[a\x03\x0CV[_\x01\x90V[4a\x06\xEEWa\x06\xD8a\x06\xD26`\x04a\x06\x8DV[\x90a\x18\x03V[a\x06\xE0a\x03\x02V[\x80a\x06\xEA\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[`\xFF\x16\x90V[a\x07\x02\x90a\x06\xF3V[\x90RV[\x91\x90a\x07\x19\x90_` \x85\x01\x94\x01\x90a\x06\xF9V[V[4a\x07KWa\x07+6`\x04a\x03\xB9V[a\x07Ga\x076a\x182V[a\x07>a\x03\x02V[\x91\x82\x91\x82a\x07\x06V[\x03\x90\xF3[a\x03\x08V[4a\x07\x80Wa\x07`6`\x04a\x03\xB9V[a\x07|a\x07ka\x18HV[a\x07sa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[4a\x07\xB4Wa\x07\x9Ea\x07\x986`\x04a\x06\x8DV[\x90a\x18\\V[a\x07\xA6a\x03\x02V[\x80a\x07\xB0\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\x07\xEAWa\x07\xE6a\x07\xD5a\x07\xCF6`\x04a\x04\xC8V[\x90a\x19\rV[a\x07\xDDa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x08\x1EWa\x08\x08a\x08\x026`\x04a\x04\xC8V[\x90a\x1A\x94V[a\x08\x10a\x03\x02V[\x80a\x08\x1A\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\x08SWa\x0836`\x04a\x03\xB9V[a\x08Oa\x08>a\x1A\xC5V[a\x08Fa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x08\x88Wa\x08h6`\x04a\x03\xB9V[a\x08\x84a\x08sa\x1B\x84V[a\x08{a\x03\x02V[\x91\x82\x91\x82a\x04\x1BV[\x03\x90\xF3[a\x03\x08V[\x90V[\x90V[a\x08\xA7a\x08\xA2a\x08\xAC\x92a\x08\x8DV[a\x08\x90V[a\x04\xA2V[\x90V[a\x08\xBBbv\xA7\0a\x08\x93V[\x90V[a\x08\xC6a\x08\xAFV[\x90V[4a\x08\xF9Wa\x08\xD96`\x04a\x03\xB9V[a\x08\xF5a\x08\xE4a\x08\xBEV[a\x08\xECa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90` \x82\x82\x03\x12a\t\x17Wa\t\x14\x91_\x01a\x04\x93V[\x90V[a\x03\x0CV[a\t%\x90a\x04sV[\x90RV[\x91\x90a\t<\x90_` \x85\x01\x94\x01\x90a\t\x1CV[V[4a\tnWa\tja\tYa\tT6`\x04a\x08\xFEV[a\x1C V[a\taa\x03\x02V[\x91\x82\x91\x82a\t)V[\x03\x90\xF3[a\x03\x08V[4a\t\xA1Wa\t\x8Ba\t\x866`\x04a\x08\xFEV[a\x1C?V[a\t\x93a\x03\x02V[\x80a\t\x9D\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[c\xFF\xFF\xFF\xFF\x16\x90V[a\t\xB8\x90a\t\xA6V[\x90RV[\x91\x90a\t\xCF\x90_` \x85\x01\x94\x01\x90a\t\xAFV[V[4a\n\x01Wa\t\xFDa\t\xECa\t\xE76`\x04a\x08\xFEV[a\x1CVV[a\t\xF4a\x03\x02V[\x91\x82\x91\x82a\t\xBCV[\x03\x90\xF3[a\x03\x08V[4a\n6Wa\n2a\n!a\n\x1C6`\x04a\x08\xFEV[a\x1C\x81V[a\n)a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\njWa\nTa\nN6`\x04a\x04\xC8V[\x90a\x1D\xB6V[a\n\\a\x03\x02V[\x80a\nf\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\n\x9FWa\n\x7F6`\x04a\x03\xB9V[a\n\x9Ba\n\x8Aa\x1D\xC2V[a\n\x92a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\n\xD4Wa\n\xD0a\n\xBFa\n\xBA6`\x04a\x08\xFEV[a\x1E:V[a\n\xC7a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x0B\tWa\n\xE96`\x04a\x03\xB9V[a\x0B\x05a\n\xF4a\x1EOV[a\n\xFCa\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0BbWa\x0BB6`\x04a\x03\xB9V[a\x0B^a\x0BMa\x0B\x0EV[a\x0BUa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90` \x82\x82\x03\x12a\x0B\x80Wa\x0B}\x91_\x01a\x04\xB9V[\x90V[a\x03\x0CV[4a\x0B\xB3Wa\x0B\x9Da\x0B\x986`\x04a\x0BgV[a \x1AV[a\x0B\xA5a\x03\x02V[\x80a\x0B\xAF\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[`\xFF`\xF8\x1B\x16\x90V[a\x0B\xCA\x90a\x0B\xB8V[\x90RV[Q\x90V[` \x91\x81R\x01\x90V[` \x01\x90V[a\x0B\xEA\x90a\x04\xA2V[\x90RV[\x90a\x0B\xFB\x81` \x93a\x0B\xE1V[\x01\x90V[` \x01\x90V[\x90a\x0C\"a\x0C\x1Ca\x0C\x15\x84a\x0B\xCEV[\x80\x93a\x0B\xD2V[\x92a\x0B\xDBV[\x90_[\x81\x81\x10a\x0C2WPPP\x90V[\x90\x91\x92a\x0CKa\x0CE`\x01\x92\x86Qa\x0B\xEEV[\x94a\x0B\xFFV[\x91\x01\x91\x90\x91a\x0C%V[\x93\x95\x91\x94a\x0C\xA6a\x0C\x9Ba\x0C\xBA\x95a\x0C\x8Da\x0C\xB0\x95a\x0C\xC7\x9C\x9Aa\x0C\x80`\xE0\x8C\x01\x92_\x8D\x01\x90a\x0B\xC1V[\x8A\x82\x03` \x8C\x01Ra\x03\xEAV[\x90\x88\x82\x03`@\x8A\x01Ra\x03\xEAV[\x97``\x87\x01\x90a\x05+V[`\x80\x85\x01\x90a\t\x1CV[`\xA0\x83\x01\x90a\x066V[`\xC0\x81\x84\x03\x91\x01Ra\x0C\x05V[\x90V[4a\r\x01Wa\x0C\xDA6`\x04a\x03\xB9V[a\x0C\xFDa\x0C\xE5a \xA2V[\x93a\x0C\xF4\x97\x95\x97\x93\x91\x93a\x03\x02V[\x97\x88\x97\x88a\x0CUV[\x03\x90\xF3[a\x03\x08V[\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t\x90V[a\r2a\r\x06V[\x90V[4a\reWa\rE6`\x04a\x03\xB9V[a\raa\rPa\r*V[a\rXa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[\x7F\x9B\x12\xE0\xC5p~II\x15\xE5\x8B\x05d\xF1\x8A\xAA\xD9\xB7J\xC6\x9B\xFC\x81Z\x1E\xDA\xDC\x8EK\xD02\xEB\x90V[a\r\x96a\rjV[\x90V[4a\r\xC9Wa\r\xA96`\x04a\x03\xB9V[a\r\xC5a\r\xB4a\r\x8EV[a\r\xBCa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[4a\r\xFEWa\r\xFAa\r\xE9a\r\xE46`\x04a\x0BgV[a!,V[a\r\xF1a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90V[a\x0E\x1Aa\x0E\x15a\x0E\x1F\x92a\x0E\x03V[a\x08\x90V[a\x04\xA2V[\x90V[a\x0E7k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x0E\x06V[\x90V[a\x0EBa\x0E\"V[\x90V[4a\x0EuWa\x0EU6`\x04a\x03\xB9V[a\x0Eqa\x0E`a\x0E:V[a\x0Eha\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x0E\xABWa\x0E\xA7a\x0E\x96a\x0E\x906`\x04a\x06\x8DV[\x90a!\x9AV[a\x0E\x9Ea\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x0E\xC4\x90a\x0E\xB0V[\x90RV[\x91\x90a\x0E\xDB\x90_` \x85\x01\x94\x01\x90a\x0E\xBBV[V[4a\x0F\rWa\x0E\xED6`\x04a\x03\xB9V[a\x0F\ta\x0E\xF8a!\xC8V[a\x0F\0a\x03\x02V[\x91\x82\x91\x82a\x0E\xC8V[\x03\x90\xF3[a\x03\x08V[4a\x0FBWa\x0F\"6`\x04a\x03\xB9V[a\x0F>a\x0F-a!\xDCV[a\x0F5a\x03\x02V[\x91\x82\x91\x82a\x04\x1BV[\x03\x90\xF3[a\x03\x08V[4a\x0FwWa\x0Fsa\x0Fba\x0F]6`\x04a\x08\xFEV[a!\xF2V[a\x0Fja\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90V[a\x0F\x93a\x0F\x8Ea\x0F\x98\x92a\x0F|V[a\x08\x90V[a\x04\xA2V[\x90V[a\x0F\xB0k\x02\xE8vi\xC3\x08sj\x04\0\0\0a\x0F\x7FV[\x90V[a\x0F\xBBa\x0F\x9BV[\x90V[4a\x0F\xEEWa\x0F\xCE6`\x04a\x03\xB9V[a\x0F\xEAa\x0F\xD9a\x0F\xB3V[a\x0F\xE1a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90V[_\x1B\x90V[a\x10\x0Fa\x10\na\x10\x14\x92a\x0F\xF3V[a\x0F\xF6V[a\x05\xF2V[\x90V[a\x10 _a\x0F\xFBV[\x90V[a\x10+a\x10\x17V[\x90V[4a\x10^Wa\x10>6`\x04a\x03\xB9V[a\x10Za\x10Ia\x10#V[a\x10Qa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[4a\x10\x94Wa\x10\x90a\x10\x7Fa\x10y6`\x04a\x04\xC8V[\x90a\"!V[a\x10\x87a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[\x1C\x90V[\x90V[a\x10\xB0\x90`\x08a\x10\xB5\x93\x02a\x10\x99V[a\x10\x9DV[\x90V[\x90a\x10\xC3\x91Ta\x10\xA0V[\x90V[a\x10\xD2`\x0C_\x90a\x10\xB8V[\x90V[4a\x11\x05Wa\x10\xE56`\x04a\x03\xB9V[a\x11\x01a\x10\xF0a\x10\xC6V[a\x10\xF8a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x11;Wa\x117a\x11&a\x11 6`\x04a\x04\xC8V[\x90a\"CV[a\x11.a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x11pWa\x11la\x11[a\x11V6`\x04a\x08\xFEV[a\"YV[a\x11ca\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x11\xA5Wa\x11\x856`\x04a\x03\xB9V[a\x11\xA1a\x11\x90a\"nV[a\x11\x98a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[a\x11\xB3\x81a\x06\xF3V[\x03a\x11\xBAWV[_\x80\xFD[\x90P5\x90a\x11\xCB\x82a\x11\xAAV[V[\x90\x91`\xC0\x82\x84\x03\x12a\x12,Wa\x11\xE5\x83_\x84\x01a\x04\x93V[\x92a\x11\xF3\x81` \x85\x01a\x04\xB9V[\x92a\x12\x01\x82`@\x83\x01a\x04\xB9V[\x92a\x12)a\x12\x12\x84``\x85\x01a\x11\xBEV[\x93a\x12 \x81`\x80\x86\x01a\x06\tV[\x93`\xA0\x01a\x06\tV[\x90V[a\x03\x0CV[4a\x12fWa\x12Pa\x12D6`\x04a\x11\xCDV[\x94\x93\x90\x93\x92\x91\x92a\"\xEEV[a\x12Xa\x03\x02V[\x80a\x12b\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[`\xE0\x81\x83\x03\x12a\x12\xD6Wa\x12\x81\x82_\x83\x01a\x04\x93V[\x92a\x12\x8F\x83` \x84\x01a\x04\x93V[\x92a\x12\x9D\x81`@\x85\x01a\x04\xB9V[\x92a\x12\xAB\x82``\x83\x01a\x04\xB9V[\x92a\x12\xD3a\x12\xBC\x84`\x80\x85\x01a\x11\xBEV[\x93a\x12\xCA\x81`\xA0\x86\x01a\x06\tV[\x93`\xC0\x01a\x06\tV[\x90V[a\x03\x0CV[4a\x13\x10Wa\x12\xFAa\x12\xEE6`\x04a\x12kV[\x95\x94\x90\x94\x93\x91\x93a$BV[a\x13\x02a\x03\x02V[\x80a\x13\x0C\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\x13DWa\x13.a\x13(6`\x04a\x06\x8DV[\x90a%`V[a\x136a\x03\x02V[\x80a\x13@\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[\x91\x90`@\x83\x82\x03\x12a\x13qW\x80a\x13ea\x13n\x92_\x86\x01a\x04\x93V[\x93` \x01a\x04\x93V[\x90V[a\x03\x0CV[4a\x13\xA7Wa\x13\xA3a\x13\x92a\x13\x8C6`\x04a\x13IV[\x90a%\x82V[a\x13\x9Aa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[a\x13\xB5\x81a\t\xA6V[\x03a\x13\xBCWV[_\x80\xFD[\x90P5\x90a\x13\xCD\x82a\x13\xACV[V[\x91\x90`@\x83\x82\x03\x12a\x13\xF7W\x80a\x13\xEBa\x13\xF4\x92_\x86\x01a\x04\x93V[\x93` \x01a\x13\xC0V[\x90V[a\x03\x0CV[a\x14\x05\x90a\x0E\xB0V[\x90RV[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x14\x1D\x90a\x14\tV[\x90RV[\x90` \x80a\x14C\x93a\x149_\x82\x01Q_\x86\x01\x90a\x13\xFCV[\x01Q\x91\x01\x90a\x14\x14V[V[\x91\x90a\x14X\x90_`@\x85\x01\x94\x01\x90a\x14!V[V[4a\x14\x8BWa\x14\x87a\x14va\x14p6`\x04a\x13\xCFV[\x90a%\xF0V[a\x14~a\x03\x02V[\x91\x82\x91\x82a\x14EV[\x03\x90\xF3[a\x03\x08V[_\x80\xFD[_\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x14\xB8a\x14\xBE\x91a\x14\tV[\x91a\x14\tV[\x01\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x14\xCFWV[a\x14\x98V[\x90a\x14\xE7\x91a\x14\xE1a\x14\x94V[Pa\x14\xACV[\x90V[a\x14\xF6a\x14\xFC\x91a\x14\tV[\x91a\x14\tV[\x90\x03\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x15\x0EWV[a\x14\x98V[\x90a\x15&\x91a\x15 a\x14\x94V[Pa\x14\xEAV[\x90V[_\x90V[a\x155a\x15)V[P\x80a\x15Pa\x15Jcye\xDB\x0B`\xE0\x1Ba\x03\x10V[\x91a\x03\x10V[\x14\x90\x81\x15a\x15]W[P\x90V[a\x15g\x91Pa&\x06V[_a\x15YV[``\x90V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x01`\x02\x83\x04\x92\x16\x80\x15a\x15\xA6W[` \x83\x10\x14a\x15\xA1WV[a\x15rV[\x91`\x7F\x16\x91a\x15\x96V[` \x91\x81R\x01\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a\x15\xDCa\x15\xD5\x83a\x15\x86V[\x80\x94a\x15\xB0V[\x91`\x01\x81\x16\x90\x81_\x14a\x163WP`\x01\x14a\x15\xF7W[PPPV[a\x16\x04\x91\x92\x93\x94Pa\x15\xB9V[\x91_\x92[\x81\x84\x10a\x16\x1BWPP\x01\x90_\x80\x80a\x15\xF2V[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a\x16\x08V[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a\x15\xF2V[\x90a\x16X\x91a\x15\xC2V[\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x16y\x90a\x03\xE0V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x16\x93W`@RV[a\x16[V[\x90a\x16\xB8a\x16\xB1\x92a\x16\xA8a\x03\x02V[\x93\x84\x80\x92a\x16NV[\x03\x83a\x16oV[V[a\x16\xC3\x90a\x16\x98V[\x90V[a\x16\xCEa\x15mV[Pa\x16\xD9`\x03a\x16\xBAV[\x90V[a\x16\xF9\x91a\x16\xE8a\x15)V[Pa\x16\xF1a&,V[\x91\x90\x91a&9V[`\x01\x90V[_\x90V[_\x1C\x90V[a\x17\x13a\x17\x18\x91a\x17\x02V[a\x10\x9DV[\x90V[a\x17%\x90Ta\x17\x07V[\x90V[a\x170a\x16\xFEV[Pa\x17;`\x02a\x17\x1BV[\x90V[\x91a\x17h\x92a\x17Ka\x15)V[Pa\x17`a\x17Wa&,V[\x82\x90\x84\x91a&\x89V[\x91\x90\x91a'\x15V[`\x01\x90V[_\x90V[a\x17z\x90a\x05\xF2V[\x90V[\x90a\x17\x87\x90a\x17qV[_R` R`@_ \x90V[\x90V[a\x17\xA2a\x17\xA7\x91a\x17\x02V[a\x17\x93V[\x90V[a\x17\xB4\x90Ta\x17\x96V[\x90V[`\x01a\x17\xD0a\x17\xD6\x92a\x17\xC8a\x17mV[P`\x05a\x17}V[\x01a\x17\xAAV[\x90V[\x90a\x17\xF4\x91a\x17\xEFa\x17\xEA\x82a\x17\xB7V[a'\xB2V[a\x17\xF6V[V[\x90a\x18\0\x91a(\x0BV[PV[\x90a\x18\r\x91a\x17\xD9V[V[_\x90V[\x90V[a\x18*a\x18%a\x18/\x92a\x18\x13V[a\x08\x90V[a\x06\xF3V[\x90V[a\x18:a\x18\x0FV[Pa\x18E`\x12a\x18\x16V[\x90V[a\x18Pa\x17mV[Pa\x18Ya(\xB7V[\x90V[\x90\x80a\x18wa\x18qa\x18la&,V[a\x04sV[\x91a\x04sV[\x03a\x18\x88Wa\x18\x85\x91a)qV[PV[_c3K\xD9\x19`\xE1\x1B\x81R\x80a\x18\xA0`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[a\x18\xB8a\x18\xB3a\x18\xBD\x92a\x04hV[a\x08\x90V[a\x04hV[\x90V[a\x18\xC9\x90a\x18\xA4V[\x90V[a\x18\xD5\x90a\x18\xC0V[\x90V[\x90a\x18\xE2\x90a\x18\xCCV[_R` R`@_ \x90V[\x90V[a\x19\x05a\x19\0a\x19\n\x92a\x14\tV[a\x08\x90V[a\x04\xA2V[\x90V[a\x19D\x91a\x199a\x193a\x19.a\x19?\x94a\x19&a\x16\xFEV[P`\na\x18\xD8V[a\x18\xEEV[\x91a*RV[\x90a+gV[a\x18\xF1V[\x90V[\x90a\x19a\x91a\x19\\a\x19Wa\rjV[a'\xB2V[a\x19\xCCV[V[a\x19wa\x19ra\x19|\x92a\x0F\xF3V[a\x08\x90V[a\x04hV[\x90V[a\x19\x88\x90a\x19cV[\x90V[a\x19\x9Fa\x19\x9Aa\x19\xA4\x92a\x0F\xF3V[a\x08\x90V[a\x04\xA2V[\x90V[a\x19\xB6a\x19\xBC\x91\x93\x92\x93a\x04\xA2V[\x92a\x04\xA2V[\x82\x01\x80\x92\x11a\x19\xC7WV[a\x14\x98V[\x90\x81a\x19\xE8a\x19\xE2a\x19\xDD_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14a\x1AxW\x80a\x1A\0a\x19\xFA_a\x19\x8BV[\x91a\x04\xA2V[\x14a\x1A\\Wa\x1A\x17a\x1A\x10a\x17(V[\x82\x90a\x19\xA7V[a\x1A0a\x1A*a\x1A%a\x0E\"V[a\x04\xA2V[\x91a\x04\xA2V[\x11a\x1A@Wa\x1A>\x91a,\x8EV[V[_c\x17~?\xC3`\xE0\x1B\x81R\x80a\x1AX`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1At`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x1A\x90`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x90a\x1A\x9E\x91a\x19GV[V[a\x1A\xAFa\x1A\xB5\x91\x93\x92\x93a\x04\xA2V[\x92a\x04\xA2V[\x82\x03\x91\x82\x11a\x1A\xC0WV[a\x14\x98V[a\x1A\xCDa\x16\xFEV[Pa\x1A\xE7a\x1A\xD9a\x0E\"V[a\x1A\xE1a\x17(V[\x90a\x1A\xA0V[\x90V[\x90a\x1A\xFDa\x1A\xF6a\x03\x02V[\x92\x83a\x16oV[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1B\x1DWa\x1B\x19` \x91a\x03\xE0V[\x01\x90V[a\x16[V[\x90a\x1B4a\x1B/\x83a\x1A\xFFV[a\x1A\xEAV[\x91\x82RV[_\x7Fmode=blocknumber&from=default\0\0\0\x91\x01RV[a\x1Bj`\x1Da\x1B\"V[\x90a\x1Bw` \x83\x01a\x1B9V[V[a\x1B\x81a\x1B`V[\x90V[a\x1B\x8Ca\x15mV[Pa\x1B\x95a!\xC8V[a\x1B\xAEa\x1B\xA8a\x1B\xA3a,\xECV[a\x0E\xB0V[\x91a\x0E\xB0V[\x03a\x1B\xBEWa\x1B\xBBa\x1ByV[\x90V[_c\x01\xBF\xC1\xC5`\xE6\x1B\x81R\x80a\x1B\xD6`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_\x90V[\x90a\x1B\xE8\x90a\x18\xCCV[_R` R`@_ \x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x1C\x0Ba\x1C\x10\x91a\x17\x02V[a\x1B\xF4V[\x90V[a\x1C\x1D\x90Ta\x1B\xFFV[\x90V[a\x1C7a\x1C<\x91a\x1C/a\x1B\xDAV[P`\ta\x1B\xDEV[a\x1C\x13V[\x90V[a\x1CP\x90a\x1CKa&,V[a-?V[V[_\x90V[a\x1Ch\x90a\x1Cba\x1CRV[Pa-\xCAV[\x90V[\x90a\x1Cu\x90a\x18\xCCV[_R` R`@_ \x90V[a\x1C\x97a\x1C\x9C\x91a\x1C\x90a\x16\xFEV[P_a\x1CkV[a\x17\x1BV[\x90V[\x90a\x1C\xB9\x91a\x1C\xB4a\x1C\xAFa\r\x06V[a'\xB2V[a\x1C\xBBV[V[\x80a\x1C\xD6a\x1C\xD0a\x1C\xCB_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14a\x1D\x9AW\x81a\x1C\xEEa\x1C\xE8_a\x19\x8BV[\x91a\x04\xA2V[\x14a\x1D~Wa\x1D\x04a\x1C\xFEa\x1EOV[\x15a\x03]V[a\x1DbWa\x1D\x13\x81\x83\x90a-\xF9V[3\x90a\x1D]a\x1DKa\x1DE\x7F\xBE\xF4\xF8\x1C\x18\x14\xC6A\xED\xE8^\xBA\xAC\xF1\x9D\x04\x8B,[U\x98\n\xDF\xA6\xEF\x0F\x95le\x135\xA2\x93a\x18\xCCV[\x93a\x18\xCCV[\x93a\x1DTa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xA3V[_c\xB8\xB5\xCA-`\xE0\x1B\x81R\x80a\x1Dz`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1D\x96`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x1D\xB2`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x90a\x1D\xC0\x91a\x1C\x9FV[V[a\x1D\xCAa\x16\xFEV[Pa\x1D\xD5`\x0Ca\x17\x1BV[a\x1D\xE7a\x1D\xE1_a\x19\x8BV[\x91a\x04\xA2V[\x14\x80\x15a\x1E\x16W[a\x1E\nWa\x1E\x07a\x1E\0`\x0Ca\x17\x1BV[B\x90a\x1A\xA0V[\x90V[a\x1E\x13_a\x19\x8BV[\x90V[PBa\x1E3a\x1E-a\x1E(`\x0Ca\x17\x1BV[a\x04\xA2V[\x91a\x04\xA2V[\x10\x15a\x1D\xEFV[a\x1EL\x90a\x1EFa\x16\xFEV[Pa.XV[\x90V[a\x1EWa\x15)V[Pa\x1Eb`\x0Ca\x17\x1BV[a\x1Eta\x1En_a\x19\x8BV[\x91a\x04\xA2V[\x14\x15\x80a\x1E\x7FW[\x90V[PBa\x1E\x9Ca\x1E\x96a\x1E\x91`\x0Ca\x17\x1BV[a\x04\xA2V[\x91a\x04\xA2V[\x10a\x1E|V[a\x1E\xBB\x90a\x1E\xB6a\x1E\xB1a\x10\x17V[a'\xB2V[a\x1F5V[V[\x90a\x1E\xC9_\x19\x91a\x0F\xF6V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x1E\xE7a\x1E\xE2a\x1E\xEC\x92a\x04\xA2V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[\x90a\x1F\x07a\x1F\x02a\x1F\x0E\x92a\x1E\xD3V[a\x1E\xEFV[\x82Ta\x1E\xBDV[\x90UV[\x91` a\x1F3\x92\x94\x93a\x1F,`@\x82\x01\x96_\x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[\x80a\x1FHa\x1FBBa\x04\xA2V[\x91a\x04\xA2V[\x11\x15a\x1F\xFEW\x80a\x1F\x81a\x1F{\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xA2V[\x91a\x04\xA2V[\x11a\x1F\xE2Wa\x1F\x90`\x0Ca\x17\x1BV[a\x1F\x9B\x82`\x0Ca\x1E\xF2V[\x903\x90a\x1F\xC8\x7F\xDDh\x96\xDC\xF1\xD4\xB3\x11\xCC\xA8}\xD1\x9B\xBB\xA2\xEA\x9C\xE2\xF8g\xC1V\x88x\xA0C\x8Af\xA1\xAF\xEE\xEC\x92a\x18\xCCV[\x92a\x1F\xDDa\x1F\xD4a\x03\x02V[\x92\x83\x92\x83a\x1F\x12V[\x03\x90\xA2V[_c\xEFi\xAFe`\xE0\x1B\x81R\x80a\x1F\xFA`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\xA5e\x83S`\xE0\x1B\x81R\x80a \x16`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[a #\x90a\x1E\xA2V[V[_\x90V[``\x90V[a 7\x90a\x18\xC0V[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a RW` \x80\x91\x02\x01\x90V[a\x16[V[\x90a ia d\x83a :V[a\x1A\xEAV[\x91\x82RV[6\x907V[\x90a \x98a \x80\x83a WV[\x92` \x80a \x8E\x86\x93a :V[\x92\x01\x91\x03\x90a nV[V[`\x0F`\xF8\x1B\x90V[a \xAAa %V[Pa \xB3a\x15mV[Pa \xBCa\x15mV[Pa \xC5a\x16\xFEV[Pa \xCEa\x1B\xDAV[Pa \xD7a\x17mV[Pa \xE0a )V[Pa \xE9a.pV[\x90a \xF2a.\xB0V[\x90F\x90a \xFE0a .V[\x90a!\x08_a\x0F\xFBV[\x90a!\x1Aa!\x15_a\x19\x8BV[a sV[\x90a!#a \x9AV[\x96\x95\x94\x93\x92\x91\x90V[a!Ua!Z\x91a!;a\x16\xFEV[Pa!Oa!I`\x0Ba\x18\xEEV[\x91a*RV[\x90a+gV[a\x18\xF1V[\x90V[\x90a!g\x90a\x18\xCCV[_R` R`@_ \x90V[`\xFF\x16\x90V[a!\x85a!\x8A\x91a\x17\x02V[a!sV[\x90V[a!\x97\x90Ta!yV[\x90V[a!\xC1\x91_a!\xB6a!\xBC\x93a!\xAEa\x15)V[P`\x05a\x17}V[\x01a!]V[a!\x8DV[\x90V[_\x90V[a!\xD0a!\xC4V[Pa!\xD9a,\xECV[\x90V[a!\xE4a\x15mV[Pa!\xEF`\x04a\x16\xBAV[\x90V[a\"\x19a\"\x14a\"\x0Fa\"\x1E\x93a\"\x07a\x16\xFEV[P`\na\x18\xD8V[a\x18\xEEV[a.\xF0V[a\x18\xF1V[\x90V[a\">\x91a\"-a\x15)V[Pa\"6a&,V[\x91\x90\x91a'\x15V[`\x01\x90V[\x90a\"V\x91a\"Pa\x16\xFEV[Pa\x19\rV[\x90V[a\"k\x90a\"ea\x16\xFEV[Pa!\xF2V[\x90V[a\"va\x16\xFEV[Pa\"\x7Fa\x17(V[\x90V[\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF\x90V[a\"\xDBa\"\xE2\x94a\"\xD1``\x94\x98\x97\x95a\"\xC7`\x80\x86\x01\x9A_\x87\x01\x90a\x066V[` \x85\x01\x90a\t\x1CV[`@\x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[` \x01\x90V[Q\x90V[\x93\x95\x94\x90\x92\x91\x95Ba#\x08a#\x02\x89a\x04\xA2V[\x91a\x04\xA2V[\x11a#\x81W\x91a#s\x91a#z\x93a#ja#\x7F\x98\x99a#Ra#)a\"\x82V[a#C\x8B\x93\x8Ba#7a\x03\x02V[\x95\x86\x94` \x86\x01a\"\xA6V[` \x82\x01\x81\x03\x82R\x03\x82a\x16oV[a#da#^\x82a\"\xEAV[\x91a\"\xE4V[ a/eV[\x92\x90\x91\x92a/\x82V[\x91\x82a/\xCCV[a-?V[V[a#\x9C\x87_\x91\x82\x91c#A\xD7\x87`\xE1\x1B\x83R`\x04\x83\x01a\x058V[\x03\x90\xFD[\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x90V[\x91\x94a$\x0Ca$\x16\x92\x98\x97\x95a$\x02`\xA0\x96a#\xF8a$\x1D\x9Aa#\xEE`\xC0\x8A\x01\x9E_\x8B\x01\x90a\x066V[` \x89\x01\x90a\t\x1CV[`@\x87\x01\x90a\t\x1CV[``\x85\x01\x90a\x05+V[`\x80\x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[\x91` a$@\x92\x94\x93a$9`@\x82\x01\x96_\x83\x01\x90a\t\x1CV[\x01\x90a\t\x1CV[V[\x96\x95\x91\x93\x92\x94\x90\x94Ba$]a$W\x83a\x04\xA2V[\x91a\x04\xA2V[\x11a%\x17W\x90a$\xC6a$\xCF\x94\x93\x92a$\xAEa$wa#\xA0V[a$\x9F\x8C\x80\x94\x8C\x91a$\x89\x8D\x91a0\x1EV[\x91\x92a$\x93a\x03\x02V[\x97\x88\x96` \x88\x01a#\xC4V[` \x82\x01\x81\x03\x82R\x03\x82a\x16oV[a$\xC0a$\xBA\x82a\"\xEAV[\x91a\"\xE4V[ a/eV[\x92\x90\x91\x92a/\x82V[\x80a$\xE2a$\xDC\x87a\x04sV[\x91a\x04sV[\x03a$\xF7WPa$\xF5\x92\x93\x91\x90\x91a&9V[V[\x84\x90a%\x13_\x92\x83\x92c%\xC0\x07#`\xE1\x1B\x84R`\x04\x84\x01a$\x1FV[\x03\x90\xFD[a%2\x90_\x91\x82\x91c1<\x89\x81`\xE1\x1B\x83R`\x04\x83\x01a\x058V[\x03\x90\xFD[\x90a%Q\x91a%La%G\x82a\x17\xB7V[a'\xB2V[a%SV[V[\x90a%]\x91a)qV[PV[\x90a%j\x91a%6V[V[\x90a%v\x90a\x18\xCCV[_R` R`@_ \x90V[a%\xA7\x91a%\x9Da%\xA2\x92a%\x95a\x16\xFEV[P`\x01a%lV[a\x1CkV[a\x17\x1BV[\x90V[a%\xB4`@a\x1A\xEAV[\x90V[_\x90V[_\x90V[a%\xC7a%\xAAV[\x90` \x80\x83a%\xD4a%\xB7V[\x81R\x01a%\xDFa%\xBBV[\x81RPPV[a%\xEDa%\xBFV[\x90V[\x90a&\x03\x91a%\xFDa%\xE5V[Pa0QV[\x90V[a&\x0Ea\x15)V[Pa&(a&\"c\x01\xFF\xC9\xA7`\xE0\x1Ba\x03\x10V[\x91a\x03\x10V[\x14\x90V[a&4a\x1B\xDAV[P3\x90V[\x91a&G\x92\x91`\x01\x92a0yV[V[`@\x90a&ra&y\x94\x96\x95\x93\x96a&h``\x84\x01\x98_\x85\x01\x90a\t\x1CV[` \x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[\x90a&\x86\x91\x03a\x04\xA2V[\x90V[\x92\x91\x92a&\x97\x81\x83\x90a%\x82V[\x90\x81a&\xACa&\xA6_\x19a\x04\xA2V[\x91a\x04\xA2V[\x10a&\xB9W[PPP\x90PV[\x81a&\xCCa&\xC6\x87a\x04\xA2V[\x91a\x04\xA2V[\x10a&\xF2Wa&\xE9\x93\x94a&\xE1\x91\x93\x92a&{V[\x90_\x92a0yV[\x80_\x80\x80a&\xB2V[Pa'\x11\x84\x92\x91\x92_\x93\x84\x93c}\xC7\xA0\xD9`\xE1\x1B\x85R`\x04\x85\x01a&IV[\x03\x90\xFD[\x91\x82a'1a'+a'&_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14a'\x8BW\x81a'Qa'Ka'F_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14a'dWa'b\x92\x91\x90\x91a1\x88V[V[a'\x87a'p_a\x19\x7FV[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\t)V[\x03\x90\xFD[a'\xAEa'\x97_a\x19\x7FV[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\t)V[\x03\x90\xFD[a'\xC4\x90a'\xBEa&,V[\x90a2UV[V[\x90a'\xD2`\xFF\x91a\x0F\xF6V[\x91\x81\x19\x16\x91\x16\x17\x90V[a'\xE5\x90a\x03]V[\x90V[\x90V[\x90a(\0a'\xFBa(\x07\x92a'\xDCV[a'\xE8V[\x82Ta'\xC6V[\x90UV[a(\x13a\x15)V[Pa((a(\"\x82\x84\x90a!\x9AV[\x15a\x03]V[_\x14a(\xB1Wa(P`\x01a(K_a(C`\x05\x86\x90a\x17}V[\x01\x85\x90a!]V[a'\xEBV[\x90a(Ya&,V[\x90a(\x96a(\x90a(\x8A\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x95a\x17qV[\x92a\x18\xCCV[\x92a\x18\xCCV[\x92a(\x9Fa\x03\x02V[\x80a(\xA9\x81a\x06\xBAV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a(\xBFa\x17mV[Pa(\xC90a .V[a(\xFBa(\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04sV[\x91a\x04sV[\x14\x80a)7W[_\x14a),W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a)4a3\x01V[\x90V[PFa)ka)e\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xA2V[\x91a\x04\xA2V[\x14a)\x02V[a)ya\x15)V[Pa)\x85\x81\x83\x90a!\x9AV[_\x14a*\rWa)\xAC_a)\xA7_a)\x9F`\x05\x86\x90a\x17}V[\x01\x85\x90a!]V[a'\xEBV[\x90a)\xB5a&,V[\x90a)\xF2a)\xECa)\xE6\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x95a\x17qV[\x92a\x18\xCCV[\x92a\x18\xCCV[\x92a)\xFBa\x03\x02V[\x80a*\x05\x81a\x06\xBAV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a*'a*\"a*,\x92a\x0E\xB0V[a\x08\x90V[a\x04\xA2V[\x90V[\x91` a*P\x92\x94\x93a*I`@\x82\x01\x96_\x83\x01\x90a\x05+V[\x01\x90a\x0E\xBBV[V[a*Za!\xC4V[Pa*ca!\xC8V[\x81a*va*p\x83a*\x13V[\x91a\x04\xA2V[\x10\x15a*\x89WPa*\x86\x90a4\nV[\x90V[\x90a*\xA4_\x92\x83\x92cvi\xFC\x0F`\xE1\x1B\x84R`\x04\x84\x01a*/V[\x03\x90\xFD[T\x90V[\x90V[a*\xC3a*\xBEa*\xC8\x92a*\xACV[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a*\xE5a*\xEA\x91a\x17\x02V[a*\xCEV[\x90V[a*\xF7\x90Ta*\xD9V[\x90V[\x90V[a+\x11a+\x0Ca+\x16\x92a*\xFAV[a\x08\x90V[a\x04\xA2V[\x90V[`0\x1C\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a+6a+;\x91a+\x19V[a+\x1FV[\x90V[a+H\x90Ta+*V[\x90V[a+_a+Za+d\x92a\x0F\xF3V[a\x08\x90V[a\x14\tV[\x90V[\x90a+\xBB\x90a+ta\x14\x94V[Pa+\x80_\x84\x01a*\xA8V[a+\x89_a\x19\x8BV[\x90\x80\x80a+\x9Fa+\x99`\x05a*\xAFV[\x91a\x04\xA2V[\x11a,\x1CW[P\x90a+\xB6_\x86\x01\x93\x91\x92\x93a*\xCBV[a:]V[\x80a+\xCEa+\xC8_a\x19\x8BV[\x91a\x04\xA2V[\x14_\x14a+\xE4WPPa+\xE0_a+KV[[\x90V[a,\x11_\x91a,\x0Ca,\x06\x84a,\x17\x96\x01\x92a,\0`\x01a*\xFDV[\x90a\x1A\xA0V[\x91a*\xCBV[a:SV[\x01a+>V[a+\xE1V[\x80a,*a,0\x92\x91a6\xE8V[\x90a\x1A\xA0V[\x90\x83a,ba,\\a,W_a,Q\x81\x8C\x01a,L\x89\x91a*\xCBV[a:SV[\x01a*\xEDV[a\x0E\xB0V[\x91a\x0E\xB0V[\x10_\x14a,sWP\x90[\x90_a+\xA5V[\x91Pa,\x89\x90a,\x83`\x01a*\xFDV[\x90a\x19\xA7V[a,lV[\x80a,\xA9a,\xA3a,\x9E_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14a,\xC5Wa,\xC3\x91a,\xBB_a\x19\x7FV[\x91\x90\x91a1\x88V[V[a,\xE8a,\xD1_a\x19\x7FV[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\t)V[\x03\x90\xFD[a,\xF4a!\xC4V[Pa,\xFECa4\nV[\x90V[\x90a-\x12`\x01\x80`\xA0\x1B\x03\x91a\x0F\xF6V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a-4a-/a-;\x92a\x18\xCCV[a-\x1CV[\x82Ta-\x01V[\x90UV[\x90a-\xC8\x91a-\xC2a-P\x82a\x1C V[a-e\x84a-``\t\x86\x90a\x1B\xDEV[a-\x1FV[\x82\x81\x85\x90a-\xA5a-\x9Fa-\x99\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x95a\x18\xCCV[\x92a\x18\xCCV[\x92a\x18\xCCV[\x92a-\xAEa\x03\x02V[\x80a-\xB8\x81a\x06\xBAV[\x03\x90\xA4\x92\x91a:\xECV[\x91a;\x04V[V[a-\xF1a-\xECa-\xE7a-\xF6\x93a-\xDFa\x1CRV[P`\na\x18\xD8V[a\x18\xEEV[a<\xB2V[a=1V[\x90V[\x90\x81a.\x15a.\x0Fa.\n_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14a.1Wa./\x91\x90a.(_a\x19\x7FV[\x90\x91a1\x88V[V[a.Ta.=_a\x19\x7FV[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\t)V[\x03\x90\xFD[a.j\x90a.da\x16\xFEV[Pa=\x82V[\x90V[\x90V[a.xa\x15mV[Pa.\xAD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a.\xA7`\x06a.mV[\x90a>\x9DV[\x90V[a.\xB8a\x15mV[Pa.\xED\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a.\xE7`\x07a.mV[\x90a>\x9DV[\x90V[a.\xF8a\x14\x94V[Pa/\x04_\x82\x01a*\xA8V[\x80a/\x17a/\x11_a\x19\x8BV[\x91a\x04\xA2V[\x14_\x14a/-WPPa/)_a+KV[[\x90V[a/Z_\x91a/Ua/O\x84a/`\x96\x01\x92a/I`\x01a*\xFDV[\x90a\x1A\xA0V[\x91a*\xCBV[a:SV[\x01a+>V[a/*V[a/\x7F\x90a/qa\x17mV[Pa/za(\xB7V[a>\xEBV[\x90V[\x92a/\x9D\x92a/\xA6\x94a/\x93a\x1B\xDAV[P\x92\x90\x91\x92a?\xB1V[\x90\x92\x91\x92a@\xDCV[\x90V[\x91` a/\xCA\x92\x94\x93a/\xC3`@\x82\x01\x96_\x83\x01\x90a\t\x1CV[\x01\x90a\x05+V[V[a/\xD5\x81a0\x1EV[\x91a/\xE8a/\xE2\x84a\x04\xA2V[\x91a\x04\xA2V[\x03a/\xF1WPPV[a0\x0B_\x92\x83\x92c\x01\xD4\xB6#`\xE6\x1B\x84R`\x04\x84\x01a/\xA9V[\x03\x90\xFD[`\x01a0\x1B\x91\x01a\x04\xA2V[\x90V[a02\x90a0*a\x16\xFEV[P`\x08a\x1CkV[a0Na0>\x82a\x17\x1BV[\x91a0H\x83a0\x0FV[\x90a\x1E\xF2V[\x90V[\x90a0qa0la0v\x93a0da%\xE5V[P`\na\x18\xD8V[a\x18\xEEV[aBRV[\x90V[\x90\x92\x81a0\x96a0\x90a0\x8B_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14a1aW\x83a0\xB6a0\xB0a0\xAB_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14a1:Wa0\xDA\x83a0\xD5a0\xCE`\x01\x86\x90a%lV[\x87\x90a\x1CkV[a\x1E\xF2V[a0\xE4W[PPPV[\x91\x90\x91a1/a1\x1Da1\x17\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x93a\x18\xCCV[\x93a\x18\xCCV[\x93a1&a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xA3_\x80\x80a0\xDFV[a1]a1F_a\x19\x7FV[_\x91\x82\x91cJ\x14\x06\xB1`\xE1\x1B\x83R`\x04\x83\x01a\t)V[\x03\x90\xFD[a1\x84a1m_a\x19\x7FV[_\x91\x82\x91c\xE6\x02\xDF\x05`\xE0\x1B\x83R`\x04\x83\x01a\t)V[\x03\x90\xFD[\x91\x82a1\xA4a1\x9Ea1\x99_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14\x15\x80a2\x0FW[a1\xBFW[a1\xBD\x92\x91\x90\x91aBsV[V[a1\xC7a\x1EOV[\x80a1\xEEW[\x15a1\xB1W_c6\xE2x\xFD`\xE2\x1B\x81R\x80a1\xEA`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[Pa2\na2\x04a1\xFDa\r\x06V[3\x90a!\x9AV[\x15a\x03]V[a1\xCDV[P\x81a2+a2%a2 _a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14\x15a1\xACV[\x91` a2S\x92\x94\x93a2L`@\x82\x01\x96_\x83\x01\x90a\t\x1CV[\x01\x90a\x066V[V[\x90a2ja2d\x83\x83\x90a!\x9AV[\x15a\x03]V[a2rWPPV[a2\x8C_\x92\x83\x92c\xE2Q}?`\xE0\x1B\x84R`\x04\x84\x01a22V[\x03\x90\xFD[\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x90V[\x90\x95\x94\x92a2\xFF\x94a2\xEEa2\xF8\x92a2\xE4`\x80\x96a2\xDA`\xA0\x88\x01\x9C_\x89\x01\x90a\x066V[` \x87\x01\x90a\x066V[`@\x85\x01\x90a\x066V[``\x83\x01\x90a\x05+V[\x01\x90a\t\x1CV[V[a3\ta\x17mV[Pa3\x12a2\x90V[a3\x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a3z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Fa3e0a .V[\x91a3na\x03\x02V[\x96\x87\x95` \x87\x01a2\xB4V[` \x82\x01\x81\x03\x82R\x03\x82a\x16oV[a3\x9Ba3\x95\x82a\"\xEAV[\x91a\"\xE4V[ \x90V[\x90V[a3\xB6a3\xB1a3\xBB\x92a3\x9FV[a\x08\x90V[a\x06\xF3V[\x90V[a3\xC7\x90a3\xA2V[\x90RV[\x91` a3\xEC\x92\x94\x93a3\xE5`@\x82\x01\x96_\x83\x01\x90a3\xBEV[\x01\x90a\x05+V[V[a4\x02a3\xFDa4\x07\x92a\x04\xA2V[a\x08\x90V[a\x0E\xB0V[\x90V[a4\x12a!\xC4V[P\x80a4,a4&e\xFF\xFF\xFF\xFF\xFF\xFFa*\x13V[\x91a\x04\xA2V[\x11a4=Wa4:\x90a3\xEEV[\x90V[`0a4Y_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a3\xCBV[\x03\x90\xFD[\x90V[a4ta4oa4y\x92a4]V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a4\x93a4\x8Ea4\x98\x92a4|V[a\x08\x90V[a\x06\xF3V[\x90V[a4\xBA\x90a4\xB4a4\xAEa4\xBF\x94a\x06\xF3V[\x91a\x04\xA2V[\x90a\x10\x99V[a\x04\xA2V[\x90V[\x90V[a4\xD9a4\xD4a4\xDE\x92a4\xC2V[a\x08\x90V[a\x06\xF3V[\x90V[\x1B\x90V[a5\x04\x90a4\xFEa4\xF8a5\t\x94a\x06\xF3V[\x91a\x04\xA2V[\x90a4\xE1V[a\x04\xA2V[\x90V[\x90V[a5#a5\x1Ea5(\x92a5\x0CV[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a5Ba5=a5G\x92a5+V[a\x08\x90V[a\x06\xF3V[\x90V[\x90V[a5aa5\\a5f\x92a5JV[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a5\x80a5{a5\x85\x92a5iV[a\x08\x90V[a\x06\xF3V[\x90V[\x90V[a5\x9Fa5\x9Aa5\xA4\x92a5\x88V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a5\xBEa5\xB9a5\xC3\x92a5\xA7V[a\x08\x90V[a\x06\xF3V[\x90V[\x90V[a5\xDDa5\xD8a5\xE2\x92a5\xC6V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a5\xFCa5\xF7a6\x01\x92a5\xE5V[a\x08\x90V[a\x06\xF3V[\x90V[a6\x18a6\x13a6\x1D\x92a5iV[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a67a62a6<\x92a6 V[a\x08\x90V[a\x06\xF3V[\x90V[a6Sa6Na6X\x92a5\xE5V[a\x08\x90V[a\x04\xA2V[\x90V[a6oa6ja6t\x92a*\xFAV[a\x08\x90V[a\x06\xF3V[\x90V[\x90V[a6\x8Ea6\x89a6\x93\x92a6wV[a\x08\x90V[a\x04\xA2V[\x90V[\x90a6\xA1\x91\x02a\x04\xA2V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a6\xC4a6\xCA\x91a\x04\xA2V[\x91a\x04\xA2V[\x90\x81\x15a6\xD5W\x04\x90V[a6\xA4V[\x90a6\xE5\x91\x01a\x04\xA2V[\x90V[a6\xF0a\x16\xFEV[P\x80a7\x05a6\xFF`\x01a*\xFDV[\x91a\x04\xA2V[\x11\x15a:PW\x80a9\x1Aa8\xF7a8\xE7a8\xD7a8\xC7a8\xB7a8\xA7a8\x97a8\x87a8wa8g\x8Ba8aa8Za9 \x9Fa8:a8*a8J\x92a7L`\x01a*\xFDV[\x90\x80a7da7^`\x01`\x80\x1Ba4`V[\x91a\x04\xA2V[\x10\x15a:\"W[\x80a7\x87a7\x81h\x01\0\0\0\0\0\0\0\0a5\x0FV[\x91a\x04\xA2V[\x10\x15a9\xF4W[\x80a7\xA6a7\xA0d\x01\0\0\0\0a5MV[\x91a\x04\xA2V[\x10\x15a9\xC6W[\x80a7\xC3a7\xBDb\x01\0\0a5\x8BV[\x91a\x04\xA2V[\x10\x15a9\x98W[\x80a7\xDFa7\xD9a\x01\0a5\xC9V[\x91a\x04\xA2V[\x10\x15a9jW[\x80a7\xFAa7\xF4`\x10a6\x04V[\x91a\x04\xA2V[\x10\x15a9<W[a8\x14a8\x0E`\x04a6?V[\x91a\x04\xA2V[\x10\x15a9#W[a8%`\x03a6zV[a6\x96V[a84`\x01a6[V[\x90a4\x9BV[a8D\x81\x86a6\xB8V[\x90a6\xDAV[a8T`\x01a6[V[\x90a4\x9BV[\x80\x92a6\xB8V[\x90a6\xDAV[a8q`\x01a6[V[\x90a4\x9BV[a8\x81\x81\x8Ca6\xB8V[\x90a6\xDAV[a8\x91`\x01a6[V[\x90a4\x9BV[a8\xA1\x81\x8Aa6\xB8V[\x90a6\xDAV[a8\xB1`\x01a6[V[\x90a4\x9BV[a8\xC1\x81\x88a6\xB8V[\x90a6\xDAV[a8\xD1`\x01a6[V[\x90a4\x9BV[a8\xE1\x81\x86a6\xB8V[\x90a6\xDAV[a8\xF1`\x01a6[V[\x90a4\x9BV[\x91a9\x14a9\x0Ea9\t\x85\x80\x94a6\xB8V[a\x04\xA2V[\x91a\x04\xA2V[\x11aC\x03V[\x90a&{V[\x90V[a97\x90a91`\x01a6[V[\x90a4\xE5V[a8\x1BV[a9Sa9d\x91a9M`\x04a5\xE8V[\x90a4\x9BV[\x91a9^`\x02a6#V[\x90a4\xE5V[\x90a8\x01V[a9\x81a9\x92\x91a9{`\x08a5\xAAV[\x90a4\x9BV[\x91a9\x8C`\x04a5\xE8V[\x90a4\xE5V[\x90a7\xE6V[a9\xAFa9\xC0\x91a9\xA9`\x10a5lV[\x90a4\x9BV[\x91a9\xBA`\x08a5\xAAV[\x90a4\xE5V[\x90a7\xCAV[a9\xDDa9\xEE\x91a9\xD7` a5.V[\x90a4\x9BV[\x91a9\xE8`\x10a5lV[\x90a4\xE5V[\x90a7\xADV[a:\x0Ba:\x1C\x91a:\x05`@a4\xC5V[\x90a4\x9BV[\x91a:\x16` a5.V[\x90a4\xE5V[\x90a7\x8EV[a:9a:J\x91a:3`\x80a4\x7FV[\x90a4\x9BV[\x91a:D`@a4\xC5V[\x90a4\xE5V[\x90a7kV[\x90V[_R` _ \x01\x90V[\x93\x91\x90\x92a:ia\x16\xFEV[P[\x81a:~a:x\x83a\x04\xA2V[\x91a\x04\xA2V[\x10\x15a:\xE4Wa:\x8F\x82\x82\x90aCOV[\x90a:\xA5_a:\x9F\x88\x85\x90a:SV[\x01a*\xEDV[a:\xB7a:\xB1\x87a\x0E\xB0V[\x91a\x0E\xB0V[\x11_\x14a:\xC7WP\x91[\x91a:kV[\x92\x91Pa:\xDE\x90a:\xD8`\x01a*\xFDV[\x90a\x19\xA7V[\x90a:\xC1V[\x92PP\x91P\x90V[a:\xFE\x90a:\xF8a\x16\xFEV[Pa\x1C\x81V[\x90V[\x90V[\x91\x90\x91\x80a;\x1Aa;\x14\x85a\x04sV[\x91a\x04sV[\x14\x15\x80a<\x98W[a;,W[PPPV[\x80a;Ga;Aa;<_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x03a<\x08W[P\x81a;ia;ca;^_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x03a;uW[\x80a;'V[a;\xBCa;\xAFa;\xB6\x92a;\x8B`\n\x86\x90a\x18\xD8V[\x90a;\xA9a;\xA3a;\x9D`\x01\x93aC\xE8V[\x93a\x18\xEEV[\x91a;\x01V[\x90aD;V[\x92\x90a\x18\xF1V[\x91a\x18\xF1V[\x91\x90\x91a;\xE9\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x18\xCCV[\x92a;\xFEa;\xF5a\x03\x02V[\x92\x83\x92\x83a\x1F\x12V[\x03\x90\xA2_\x80a;oV[a<Ga<Ma<@a<\x1D`\n\x85\x90a\x18\xD8V[`\x02a<:a<4a<.\x89aC\xE8V[\x93a\x18\xEEV[\x91a;\x01V[\x90aD;V[\x92\x90a\x18\xF1V[\x91a\x18\xF1V[\x91\x90\x91a<z\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x18\xCCV[\x92a<\x8Fa<\x86a\x03\x02V[\x92\x83\x92\x83a\x1F\x12V[\x03\x90\xA2_a;MV[P\x81a<\xACa<\xA6_a\x19\x8BV[\x91a\x04\xA2V[\x11a;\"V[_a<\xC6\x91a<\xBFa\x16\xFEV[P\x01a*\xA8V[\x90V[a<\xDDa<\xD8a<\xE2\x92a\t\xA6V[a\x08\x90V[a\x04\xA2V[\x90V[a<\xEE\x90a5.V[\x90RV[\x91` a=\x13\x92\x94\x93a=\x0C`@\x82\x01\x96_\x83\x01\x90a<\xE5V[\x01\x90a\x05+V[V[a=)a=$a=.\x92a\x04\xA2V[a\x08\x90V[a\t\xA6V[\x90V[a=9a\x1CRV[P\x80a=Qa=Kc\xFF\xFF\xFF\xFFa<\xC9V[\x91a\x04\xA2V[\x11a=bWa=_\x90a=\x15V[\x90V[` a=~_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a<\xF2V[\x03\x90\xFD[a=\x99a=\x9E\x91a=\x91a\x16\xFEV[P`\x08a\x1CkV[a\x17\x1BV[\x90V[\x90V[a=\xB8a=\xB3a=\xBD\x92a=\xA1V[a\x0F\xF6V[a\x05\xF2V[\x90V[a=\xCA`\xFFa=\xA4V[\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a=\xF0a=\xE9\x83a\x15\x86V[\x80\x94a\x15\xB0V[\x91`\x01\x81\x16\x90\x81_\x14a>GWP`\x01\x14a>\x0BW[PPPV[a>\x18\x91\x92\x93\x94Pa=\xCDV[\x91_\x92[\x81\x84\x10a>/WPP\x01\x90_\x80\x80a>\x06V[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a>\x1CV[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a>\x06V[\x90a>l\x91a=\xD6V[\x90V[\x90a>\x8Fa>\x88\x92a>\x7Fa\x03\x02V[\x93\x84\x80\x92a>bV[\x03\x83a\x16oV[V[a>\x9A\x90a>oV[\x90V[\x90a>\xA6a\x15mV[Pa>\xB0\x82a\x17qV[a>\xC9a>\xC3a>\xBEa=\xC0V[a\x05\xF2V[\x91a\x05\xF2V[\x14\x15_\x14a>\xDEWPa>\xDB\x90aD\xC5V[\x90V[a>\xE8\x91Pa>\x91V[\x90V[`B\x91a>\xF6a\x17mV[P`@Q\x91\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[_\x90V[a?<a?A\x91a\x17\x02V[a\x1E\xD3V[\x90V[\x90V[a?[a?Va?`\x92a?DV[a\x08\x90V[a\x04\xA2V[\x90V[a?\x98a?\x9F\x94a?\x8E``\x94\x98\x97\x95a?\x84`\x80\x86\x01\x9A_\x87\x01\x90a\x066V[` \x85\x01\x90a\x06\xF9V[`@\x83\x01\x90a\x066V[\x01\x90a\x066V[V[a?\xA9a\x03\x02V[=_\x82>=\x90\xFD[\x93\x92\x93a?\xBCa\x1B\xDAV[Pa?\xC5a?,V[Pa?\xCEa\x17mV[Pa?\xD8\x85a?0V[a@\na@\x04\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0a?GV[\x91a\x04\xA2V[\x11a@\x97W\x90a@-` \x94\x95_\x94\x93\x92\x93a@$a\x03\x02V[\x94\x85\x94\x85a?cV[\x83\x80R\x03\x90`\x01Z\xFA\x15a@\x92Wa@E_Qa\x0F\xF6V[\x80a@`a@Za@U_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14a@vW_\x91a@p_a\x0F\xFBV[\x91\x92\x91\x90V[Pa@\x80_a\x19\x7FV[`\x01\x91a@\x8C_a\x0F\xFBV[\x91\x92\x91\x90V[a?\xA1V[PPPa@\xA3_a\x19\x7FV[\x90`\x03\x92\x91\x92\x91\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x04\x11\x15a@\xCBWV[a@\xADV[\x90a@\xDA\x82a@\xC1V[V[\x80a@\xEFa@\xE9_a@\xD0V[\x91a@\xD0V[\x14_\x14a@\xFAWPPV[\x80aA\x0EaA\x08`\x01a@\xD0V[\x91a@\xD0V[\x14_\x14aA1W_c\xF6E\xEE\xDF`\xE0\x1B\x81R\x80aA-`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x80aAEaA?`\x02a@\xD0V[\x91a@\xD0V[\x14_\x14aAsWaAoaAX\x83a?0V[_\x91\x82\x91c\xFC\xE6\x98\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x058V[\x03\x90\xFD[aA\x86aA\x80`\x03a@\xD0V[\x91a@\xD0V[\x14aA\x8EWPV[aA\xA9\x90_\x91\x82\x91c5\xE2\xF3\x83`\xE2\x1B\x83R`\x04\x83\x01a\x06CV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_R` _ \x90V[aA\xD3\x81a*\xA8V[\x82\x10\x15aA\xEDWaA\xE5`\x01\x91aA\xC1V[\x91\x02\x01\x90_\x90V[aA\xADV[\x90aA\xFC\x90a\x0E\xB0V[\x90RV[\x90aB\n\x90a\x14\tV[\x90RV[\x90aBDaB;_aB\x1Ea%\xAAV[\x94aB5aB-\x83\x83\x01a*\xEDV[\x83\x88\x01aA\xF2V[\x01a+>V[` \x84\x01aB\0V[V[aBO\x90aB\x0EV[\x90V[aBp\x91_aBj\x92aBca%\xE5V[P\x01aA\xCAV[PaBFV[\x90V[\x92\x91aB\x81\x84\x83\x83\x91aD\xF5V[\x83aB\x9CaB\x96aB\x91_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14aB\xB1W[aB\xAF\x92\x93\x91\x90\x91aF\x7FV[V[aB\xB9a\x17(V[\x93aB\xC2aFdV[\x94\x80aB\xD6aB\xD0\x88a\x04\xA2V[\x91a\x04\xA2V[\x11aB\xE3WP\x93PaB\xA2V[\x85\x90aB\xFF_\x92\x83\x92c\x0EX\xAE\x93`\xE1\x1B\x84R`\x04\x84\x01a\x1F\x12V[\x03\x90\xFD[aC\x0Ba\x16\xFEV[P\x15\x15\x90V[aC%aC aC*\x92a6 V[a\x08\x90V[a\x04\xA2V[\x90V[aC9aC?\x91a\x04\xA2V[\x91a\x04\xA2V[\x90\x81\x15aCJW\x04\x90V[a6\xA4V[aCtaCz\x92aC^a\x16\xFEV[P\x82\x81\x16\x92\x18aCn`\x02aC\x11V[\x90aC-V[\x90a\x19\xA7V[\x90V[\x90V[aC\x94aC\x8FaC\x99\x92aC}V[a\x08\x90V[a\x06\xF3V[\x90V[aC\xA5\x90aC\x80V[\x90RV[\x91` aC\xCA\x92\x94\x93aC\xC3`@\x82\x01\x96_\x83\x01\x90aC\x9CV[\x01\x90a\x05+V[V[aC\xE0aC\xDBaC\xE5\x92a\x04\xA2V[a\x08\x90V[a\x14\tV[\x90V[aC\xF0a\x14\x94V[P\x80aD\naD\x04`\x01\x80`\xD0\x1B\x03a\x18\xF1V[\x91a\x04\xA2V[\x11aD\x1BWaD\x18\x90aC\xCCV[\x90V[`\xD0aD7_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01aC\xA9V[\x03\x90\xFD[\x90aDqaDw\x93\x92aDLa\x14\x94V[PaDUa\x14\x94V[P\x80\x93aDjaDca!\xC8V[\x94\x92a.\xF0V[\x90\x91aJ\xFAV[\x91aG>V[\x91\x90\x91\x90V[aD\x91aD\x8CaD\x96\x92a5+V[a\x08\x90V[a\x04\xA2V[\x90V[6\x907V[\x90aD\xC3aD\xAB\x83a\x1B\"V[\x92` \x80aD\xB9\x86\x93a\x1A\xFFV[\x92\x01\x91\x03\x90aD\x99V[V[aD\xCDa\x15mV[PaD\xD7\x81aG\xA8V[\x90aD\xEAaD\xE5` aD}V[aD\x9EV[\x91\x82R` \x82\x01R\x90V[\x91\x90\x91\x80aE\x13aE\raE\x08_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14_\x14aE\xF4WaE7aE0\x83aE+`\x02a\x17\x1BV[a\x19\xA7V[`\x02a\x1E\xF2V[[\x82aESaEMaEH_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14_\x14aE\xC8WaEwaEp\x83aEk`\x02a\x17\x1BV[a&{V[`\x02a\x1E\xF2V[[\x91\x90\x91aE\xC3aE\xB1aE\xAB\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93a\x18\xCCV[\x93a\x18\xCCV[\x93aE\xBAa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xA3V[aE\xEF\x82aE\xE9aE\xDA_\x87\x90a\x1CkV[\x91aE\xE4\x83a\x17\x1BV[a6\xDAV[\x90a\x1E\xF2V[aExV[aF\x07aF\x02_\x83\x90a\x1CkV[a\x17\x1BV[\x80aF\x1AaF\x14\x85a\x04\xA2V[\x91a\x04\xA2V[\x10aFBWaF-aF=\x91\x84\x90a&{V[aF8_\x84\x90a\x1CkV[a\x1E\xF2V[aE8V[\x90aF`\x90\x91\x92_\x93\x84\x93c9\x144\xE3`\xE2\x1B\x85R`\x04\x85\x01a&IV[\x03\x90\xFD[aFla\x16\xFEV[PaF|`\x01\x80`\xD0\x1B\x03a\x18\xF1V[\x90V[\x91aF\xD7aF\xD1aF\xDE\x94\x80aF\xA5aF\x9FaF\x9A_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14aG\x0FW[\x84aF\xC6aF\xC0aF\xBB_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14aF\xE0W[a\x1C V[\x92a\x1C V[\x90\x91a;\x04V[V[aG\x08`\x0B`\x02aG\x02aF\xFCaF\xF6\x89aC\xE8V[\x93a\x18\xEEV[\x91a;\x01V[\x90aD;V[PPaF\xCCV[aG7`\x0B`\x01aG1aG+aG%\x89aC\xE8V[\x93a\x18\xEEV[\x91a;\x01V[\x90aD;V[PPaF\xABV[\x91aGb_aGg\x94aGOa\x14\x94V[PaGXa\x14\x94V[P\x01\x92\x91\x92a*\xCBV[aI\xACV[\x91\x90\x91\x90V[aG\x81aG|aG\x86\x92a=\xA1V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[aG\xA0aG\x9BaG\xA5\x92aG\x89V[a\x08\x90V[a\x04\xA2V[\x90V[aG\xBDaG\xC2\x91aG\xB7a\x16\xFEV[Pa\x17qV[a?0V[aG\xCC`\xFFaGmV[\x16\x80aG\xE1aG\xDB`\x1FaG\x8CV[\x91a\x04\xA2V[\x11aG\xE9W\x90V[_c,\xD4J\xC3`\xE2\x1B\x81R\x80aH\x01`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[T\x90V[aH\x13`@a\x1A\xEAV[\x90V[_R` _ \x90V[aH(\x81aH\x05V[\x82\x10\x15aHBWaH:`\x01\x91aH\x16V[\x91\x02\x01\x90_\x90V[aA\xADV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[aHd\x90Qa\x0E\xB0V[\x90V[\x90aHxe\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x0F\xF6V[\x91\x81\x19\x16\x91\x16\x17\x90V[aH\x96aH\x91aH\x9B\x92a\x0E\xB0V[a\x08\x90V[a\x0E\xB0V[\x90V[\x90V[\x90aH\xB6aH\xB1aH\xBD\x92aH\x82V[aH\x9EV[\x82TaHgV[\x90UV[aH\xCB\x90Qa\x14\tV[\x90V[`0\x1B\x90V[\x90aH\xE6e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91aH\xCEV[\x91\x81\x19\x16\x91\x16\x17\x90V[aI\x04aH\xFFaI\t\x92a\x14\tV[a\x08\x90V[a\x14\tV[\x90V[\x90V[\x90aI$aI\x1FaI+\x92aH\xF0V[aI\x0CV[\x82TaH\xD4V[\x90UV[\x90aIY` _aI_\x94aIQ\x82\x82\x01aIK\x84\x88\x01aHZV[\x90aH\xA1V[\x01\x92\x01aH\xC1V[\x90aI\x0FV[V[\x91\x90aIrWaIp\x91aI/V[V[aHGV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15aI\xA7W\x82aI\x9F\x91`\x01aI\xA5\x95\x01\x81UaH\x1FV[\x90aIaV[V[a\x16[V[\x90\x92\x91\x92aI\xB8a\x14\x94V[PaI\xC1a\x14\x94V[PaI\xCB\x82aH\x05V[\x80aI\xDEaI\xD8_a\x19\x8BV[\x91a\x04\xA2V[\x11_\x14aJ\xAEWaJ\x04\x90aI\xFE\x84\x91aI\xF8`\x01a*\xFDV[\x90a\x1A\xA0V[\x90a:SV[\x90aJ\x10_\x83\x01a*\xEDV[\x92aJ\x1C_\x84\x01a+>V[\x93\x80aJ0aJ*\x85a\x0E\xB0V[\x91a\x0E\xB0V[\x11aJ\x92WaJGaJA\x84a\x0E\xB0V[\x91a\x0E\xB0V[\x14_\x14aJbWPPaJ]\x90_\x85\x91\x01aI\x0FV[[\x91\x90V[aJ\x8D\x92PaJ\x88\x86aJ\x7FaJvaH\tV[\x94_\x86\x01aA\xF2V[` \x84\x01aB\0V[aIwV[aJ^V[_c% `\x1D`\xE0\x1B\x81R\x80aJ\xAA`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[PaJ\xD9\x91aJ\xD4\x85aJ\xCBaJ\xC2aH\tV[\x94_\x86\x01aA\xF2V[` \x84\x01aB\0V[aIwV[aJ\xE2_a+KV[\x91\x90V[cNH{q`\xE0\x1B_R`Q`\x04R`$_\xFD[\x91\x90\x91\x80`\x01\x14aK\x19W`\x02\x03aJ\xE6WaK\x15\x91a\x15\x13V[\x90[V[PaK#\x91a\x14\xD4V[\x90aK\x17V",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b611490565b61001d5f356102fc565b806301ffc9a7146102f757806306fdde03146102f2578063095ea7b3146102ed57806318160ddd146102e857806323b872dd146102e3578063248a9ca3146102de5780632f2ff15d146102d9578063313ce567146102d45780633644e515146102cf57806336568abe146102ca5780633a46b1a8146102c557806340c10f19146102c05780634bdd36ce146102bb5780634bf5d7e9146102b65780634f1bfc9e146102b1578063587cde1e146102ac5780635c19a95c146102a75780636fcfff45146102a257806370a082311461029d57806379cc6790146102985780637a8cd156146102935780637ecebe001461028e57806383f1211b146102895780638426adf214610284578063844c90261461027f57806384b0196e1461027a5780638a542521146102755780638d3343d6146102705780638e539e8c1461026b578063902d55a51461026657806391d148541461026157806391ddadf41461025c57806395d89b41146102575780639ab24eb0146102525780639b7ef64b1461024d578063a217fddf14610248578063a9059cbb14610243578063aa082a9d1461023e578063b0ca253e14610239578063bb4d443614610234578063c02ae7541461022f578063c3cda5201461022a578063d505accf14610225578063d547741f14610220578063dd62ed3e1461021b5763f1127ed80361000e5761145a565b611376565b611315565b6112db565b611231565b611175565b611140565b61110a565b6110d5565b611063565b61102e565b610fbe565b610f47565b610f12565b610edd565b610e7a565b610e45565b610dce565b610d99565b610d35565b610cca565b610b85565b610b32565b610ad9565b610aa4565b610a6f565b610a3b565b610a06565b6109d1565b610973565b61093e565b6108c9565b610858565b610823565b6107ef565b6107b9565b610785565b610750565b61071b565b6106bf565b610658565b6105bc565b61054d565b6104f5565b610433565b610384565b60e01c90565b60405190565b5f80fd5b5f80fd5b63ffffffff60e01b1690565b61032581610310565b0361032c57565b5f80fd5b9050359061033d8261031c565b565b9060208282031261035857610355915f01610330565b90565b61030c565b151590565b61036b9061035d565b9052565b9190610382905f60208501940190610362565b565b346103b4576103b061039f61039a36600461033f565b61152d565b6103a7610302565b9182918261036f565b0390f35b610308565b5f9103126103c357565b61030c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b61040961041260209361041793610400816103c8565b938480936103cc565b958691016103d5565b6103e0565b0190565b6104309160208201915f8184039101526103ea565b90565b34610463576104433660046103b9565b61045f61044e6116c6565b610456610302565b9182918261041b565b0390f35b610308565b60018060a01b031690565b61047c90610468565b90565b61048881610473565b0361048f57565b5f80fd5b905035906104a08261047f565b565b90565b6104ae816104a2565b036104b557565b5f80fd5b905035906104c6826104a5565b565b91906040838203126104f057806104e46104ed925f8601610493565b936020016104b9565b90565b61030c565b346105265761052261051161050b3660046104c8565b906116dc565b610519610302565b9182918261036f565b0390f35b610308565b610534906104a2565b9052565b919061054b905f6020850194019061052b565b565b3461057d5761055d3660046103b9565b610579610568611728565b610570610302565b91829182610538565b0390f35b610308565b90916060828403126105b7576105b461059d845f8501610493565b936105ab8160208601610493565b936040016104b9565b90565b61030c565b346105ed576105e96105d86105d2366004610582565b9161173e565b6105e0610302565b9182918261036f565b0390f35b610308565b90565b6105fe816105f2565b0361060557565b5f80fd5b90503590610616826105f5565b565b906020828203126106315761062e915f01610609565b90565b61030c565b61063f906105f2565b9052565b9190610656905f60208501940190610636565b565b346106885761068461067361066e366004610618565b6117b7565b61067b610302565b91829182610643565b0390f35b610308565b91906040838203126106b557806106a96106b2925f8601610609565b93602001610493565b90565b61030c565b5f0190565b346106ee576106d86106d236600461068d565b90611803565b6106e0610302565b806106ea816106ba565b0390f35b610308565b60ff1690565b610702906106f3565b9052565b9190610719905f602085019401906106f9565b565b3461074b5761072b3660046103b9565b610747610736611832565b61073e610302565b91829182610706565b0390f35b610308565b34610780576107603660046103b9565b61077c61076b611848565b610773610302565b91829182610643565b0390f35b610308565b346107b45761079e61079836600461068d565b9061185c565b6107a6610302565b806107b0816106ba565b0390f35b610308565b346107ea576107e66107d56107cf3660046104c8565b9061190d565b6107dd610302565b91829182610538565b0390f35b610308565b3461081e576108086108023660046104c8565b90611a94565b610810610302565b8061081a816106ba565b0390f35b610308565b34610853576108333660046103b9565b61084f61083e611ac5565b610846610302565b91829182610538565b0390f35b610308565b34610888576108683660046103b9565b610884610873611b84565b61087b610302565b9182918261041b565b0390f35b610308565b90565b90565b6108a76108a26108ac9261088d565b610890565b6104a2565b90565b6108bb6276a700610893565b90565b6108c66108af565b90565b346108f9576108d93660046103b9565b6108f56108e46108be565b6108ec610302565b91829182610538565b0390f35b610308565b9060208282031261091757610914915f01610493565b90565b61030c565b61092590610473565b9052565b919061093c905f6020850194019061091c565b565b3461096e5761096a6109596109543660046108fe565b611c20565b610961610302565b91829182610929565b0390f35b610308565b346109a15761098b6109863660046108fe565b611c3f565b610993610302565b8061099d816106ba565b0390f35b610308565b63ffffffff1690565b6109b8906109a6565b9052565b91906109cf905f602085019401906109af565b565b34610a01576109fd6109ec6109e73660046108fe565b611c56565b6109f4610302565b918291826109bc565b0390f35b610308565b34610a3657610a32610a21610a1c3660046108fe565b611c81565b610a29610302565b91829182610538565b0390f35b610308565b34610a6a57610a54610a4e3660046104c8565b90611db6565b610a5c610302565b80610a66816106ba565b0390f35b610308565b34610a9f57610a7f3660046103b9565b610a9b610a8a611dc2565b610a92610302565b91829182610538565b0390f35b610308565b34610ad457610ad0610abf610aba3660046108fe565b611e3a565b610ac7610302565b91829182610538565b0390f35b610308565b34610b0957610ae93660046103b9565b610b05610af4611e4f565b610afc610302565b9182918261036f565b0390f35b610308565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610b6257610b423660046103b9565b610b5e610b4d610b0e565b610b55610302565b91829182610538565b0390f35b610308565b90602082820312610b8057610b7d915f016104b9565b90565b61030c565b34610bb357610b9d610b98366004610b67565b61201a565b610ba5610302565b80610baf816106ba565b0390f35b610308565b60ff60f81b1690565b610bca90610bb8565b9052565b5190565b60209181520190565b60200190565b610bea906104a2565b9052565b90610bfb81602093610be1565b0190565b60200190565b90610c22610c1c610c1584610bce565b8093610bd2565b92610bdb565b905f5b818110610c325750505090565b909192610c4b610c456001928651610bee565b94610bff565b9101919091610c25565b93959194610ca6610c9b610cba95610c8d610cb095610cc79c9a610c8060e08c01925f8d0190610bc1565b8a820360208c01526103ea565b9088820360408a01526103ea565b97606087019061052b565b608085019061091c565b60a0830190610636565b60c0818403910152610c05565b90565b34610d0157610cda3660046103b9565b610cfd610ce56120a2565b93610cf4979597939193610302565b97889788610c55565b0390f35b610308565b7f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a67490565b610d32610d06565b90565b34610d6557610d453660046103b9565b610d61610d50610d2a565b610d58610302565b91829182610643565b0390f35b610308565b7f9b12e0c5707e494915e58b0564f18aaad9b74ac69bfc815a1edadc8e4bd032eb90565b610d96610d6a565b90565b34610dc957610da93660046103b9565b610dc5610db4610d8e565b610dbc610302565b91829182610643565b0390f35b610308565b34610dfe57610dfa610de9610de4366004610b67565b61212c565b610df1610302565b91829182610538565b0390f35b610308565b90565b610e1a610e15610e1f92610e03565b610890565b6104a2565b90565b610e376b033b2e3c9fd0803ce8000000610e06565b90565b610e42610e22565b90565b34610e7557610e553660046103b9565b610e71610e60610e3a565b610e68610302565b91829182610538565b0390f35b610308565b34610eab57610ea7610e96610e9036600461068d565b9061219a565b610e9e610302565b9182918261036f565b0390f35b610308565b65ffffffffffff1690565b610ec490610eb0565b9052565b9190610edb905f60208501940190610ebb565b565b34610f0d57610eed3660046103b9565b610f09610ef86121c8565b610f00610302565b91829182610ec8565b0390f35b610308565b34610f4257610f223660046103b9565b610f3e610f2d6121dc565b610f35610302565b9182918261041b565b0390f35b610308565b34610f7757610f73610f62610f5d3660046108fe565b6121f2565b610f6a610302565b91829182610538565b0390f35b610308565b90565b610f93610f8e610f9892610f7c565b610890565b6104a2565b90565b610fb06b02e87669c308736a04000000610f7f565b90565b610fbb610f9b565b90565b34610fee57610fce3660046103b9565b610fea610fd9610fb3565b610fe1610302565b91829182610538565b0390f35b610308565b90565b5f1b90565b61100f61100a61101492610ff3565b610ff6565b6105f2565b90565b6110205f610ffb565b90565b61102b611017565b90565b3461105e5761103e3660046103b9565b61105a611049611023565b611051610302565b91829182610643565b0390f35b610308565b346110945761109061107f6110793660046104c8565b90612221565b611087610302565b9182918261036f565b0390f35b610308565b1c90565b90565b6110b09060086110b59302611099565b61109d565b90565b906110c391546110a0565b90565b6110d2600c5f906110b8565b90565b34611105576110e53660046103b9565b6111016110f06110c6565b6110f8610302565b91829182610538565b0390f35b610308565b3461113b576111376111266111203660046104c8565b90612243565b61112e610302565b91829182610538565b0390f35b610308565b346111705761116c61115b6111563660046108fe565b612259565b611163610302565b91829182610538565b0390f35b610308565b346111a5576111853660046103b9565b6111a161119061226e565b611198610302565b91829182610538565b0390f35b610308565b6111b3816106f3565b036111ba57565b5f80fd5b905035906111cb826111aa565b565b909160c08284031261122c576111e5835f8401610493565b926111f381602085016104b9565b9261120182604083016104b9565b9261122961121284606085016111be565b936112208160808601610609565b9360a001610609565b90565b61030c565b34611266576112506112443660046111cd565b949390939291926122ee565b611258610302565b80611262816106ba565b0390f35b610308565b60e0818303126112d657611281825f8301610493565b9261128f8360208401610493565b9261129d81604085016104b9565b926112ab82606083016104b9565b926112d36112bc84608085016111be565b936112ca8160a08601610609565b9360c001610609565b90565b61030c565b34611310576112fa6112ee36600461126b565b95949094939193612442565b611302610302565b8061130c816106ba565b0390f35b610308565b346113445761132e61132836600461068d565b90612560565b611336610302565b80611340816106ba565b0390f35b610308565b9190604083820312611371578061136561136e925f8601610493565b93602001610493565b90565b61030c565b346113a7576113a361139261138c366004611349565b90612582565b61139a610302565b91829182610538565b0390f35b610308565b6113b5816109a6565b036113bc57565b5f80fd5b905035906113cd826113ac565b565b91906040838203126113f757806113eb6113f4925f8601610493565b936020016113c0565b90565b61030c565b61140590610eb0565b9052565b60018060d01b031690565b61141d90611409565b9052565b90602080611443936114395f8201515f8601906113fc565b0151910190611414565b565b9190611458905f60408501940190611421565b565b3461148b576114876114766114703660046113cf565b906125f0565b61147e610302565b91829182611445565b0390f35b610308565b5f80fd5b5f90565b634e487b7160e01b5f52601160045260245ffd5b6114b86114be91611409565b91611409565b019060018060d01b0382116114cf57565b611498565b906114e7916114e1611494565b506114ac565b90565b6114f66114fc91611409565b91611409565b90039060018060d01b03821161150e57565b611498565b9061152691611520611494565b506114ea565b90565b5f90565b611535611529565b508061155061154a637965db0b60e01b610310565b91610310565b1490811561155d575b5090565b6115679150612606565b5f611559565b606090565b634e487b7160e01b5f52602260045260245ffd5b90600160028304921680156115a6575b60208310146115a157565b611572565b91607f1691611596565b60209181520190565b5f5260205f2090565b905f92918054906115dc6115d583611586565b80946115b0565b916001811690815f1461163357506001146115f7575b505050565b61160491929394506115b9565b915f925b81841061161b57505001905f80806115f2565b60018160209295939554848601520191019290611608565b92949550505060ff19168252151560200201905f80806115f2565b90611658916115c2565b90565b634e487b7160e01b5f52604160045260245ffd5b90611679906103e0565b810190811067ffffffffffffffff82111761169357604052565b61165b565b906116b86116b1926116a8610302565b9384809261164e565b038361166f565b565b6116c390611698565b90565b6116ce61156d565b506116d960036116ba565b90565b6116f9916116e8611529565b506116f161262c565b919091612639565b600190565b5f90565b5f1c90565b61171361171891611702565b61109d565b90565b6117259054611707565b90565b6117306116fe565b5061173b600261171b565b90565b916117689261174b611529565b5061176061175761262c565b82908491612689565b919091612715565b600190565b5f90565b61177a906105f2565b90565b9061178790611771565b5f5260205260405f2090565b90565b6117a26117a791611702565b611793565b90565b6117b49054611796565b90565b60016117d06117d6926117c861176d565b50600561177d565b016117aa565b90565b906117f4916117ef6117ea826117b7565b6127b2565b6117f6565b565b906118009161280b565b50565b9061180d916117d9565b565b5f90565b90565b61182a61182561182f92611813565b610890565b6106f3565b90565b61183a61180f565b506118456012611816565b90565b61185061176d565b506118596128b7565b90565b908061187761187161186c61262c565b610473565b91610473565b036118885761188591612971565b50565b5f63334bd91960e11b8152806118a0600482016106ba565b0390fd5b6118b86118b36118bd92610468565b610890565b610468565b90565b6118c9906118a4565b90565b6118d5906118c0565b90565b906118e2906118cc565b5f5260205260405f2090565b90565b61190561190061190a92611409565b610890565b6104a2565b90565b6119449161193961193361192e61193f946119266116fe565b50600a6118d8565b6118ee565b91612a52565b90612b67565b6118f1565b90565b906119619161195c611957610d6a565b6127b2565b6119cc565b565b61197761197261197c92610ff3565b610890565b610468565b90565b61198890611963565b90565b61199f61199a6119a492610ff3565b610890565b6104a2565b90565b6119b66119bc919392936104a2565b926104a2565b82018092116119c757565b611498565b90816119e86119e26119dd5f61197f565b610473565b91610473565b14611a785780611a006119fa5f61198b565b916104a2565b14611a5c57611a17611a10611728565b82906119a7565b611a30611a2a611a25610e22565b6104a2565b916104a2565b11611a4057611a3e91612c8e565b565b5f63177e3fc360e01b815280611a58600482016106ba565b0390fd5b5f631f2a200560e01b815280611a74600482016106ba565b0390fd5b5f63d92e233d60e01b815280611a90600482016106ba565b0390fd5b90611a9e91611947565b565b611aaf611ab5919392936104a2565b926104a2565b8203918211611ac057565b611498565b611acd6116fe565b50611ae7611ad9610e22565b611ae1611728565b90611aa0565b90565b90611afd611af6610302565b928361166f565b565b67ffffffffffffffff8111611b1d57611b196020916103e0565b0190565b61165b565b90611b34611b2f83611aff565b611aea565b918252565b5f7f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c74000000910152565b611b6a601d611b22565b90611b7760208301611b39565b565b611b81611b60565b90565b611b8c61156d565b50611b956121c8565b611bae611ba8611ba3612cec565b610eb0565b91610eb0565b03611bbe57611bbb611b79565b90565b5f6301bfc1c560e61b815280611bd6600482016106ba565b0390fd5b5f90565b90611be8906118cc565b5f5260205260405f2090565b60018060a01b031690565b611c0b611c1091611702565b611bf4565b90565b611c1d9054611bff565b90565b611c37611c3c91611c2f611bda565b506009611bde565b611c13565b90565b611c5090611c4b61262c565b612d3f565b565b5f90565b611c6890611c62611c52565b50612dca565b90565b90611c75906118cc565b5f5260205260405f2090565b611c97611c9c91611c906116fe565b505f611c6b565b61171b565b90565b90611cb991611cb4611caf610d06565b6127b2565b611cbb565b565b80611cd6611cd0611ccb5f61197f565b610473565b91610473565b14611d9a5781611cee611ce85f61198b565b916104a2565b14611d7e57611d04611cfe611e4f565b1561035d565b611d6257611d13818390612df9565b3390611d5d611d4b611d457fbef4f81c1814c641ede85ebaacf19d048b2c5b55980adfa6ef0f956c651335a2936118cc565b936118cc565b93611d54610302565b91829182610538565b0390a3565b5f63b8b5ca2d60e01b815280611d7a600482016106ba565b0390fd5b5f631f2a200560e01b815280611d96600482016106ba565b0390fd5b5f63d92e233d60e01b815280611db2600482016106ba565b0390fd5b90611dc091611c9f565b565b611dca6116fe565b50611dd5600c61171b565b611de7611de15f61198b565b916104a2565b148015611e16575b611e0a57611e07611e00600c61171b565b4290611aa0565b90565b611e135f61198b565b90565b5042611e33611e2d611e28600c61171b565b6104a2565b916104a2565b1015611def565b611e4c90611e466116fe565b50612e58565b90565b611e57611529565b50611e62600c61171b565b611e74611e6e5f61198b565b916104a2565b141580611e7f575b90565b5042611e9c611e96611e91600c61171b565b6104a2565b916104a2565b10611e7c565b611ebb90611eb6611eb1611017565b6127b2565b611f35565b565b90611ec95f1991610ff6565b9181191691161790565b611ee7611ee2611eec926104a2565b610890565b6104a2565b90565b90565b90611f07611f02611f0e92611ed3565b611eef565b8254611ebd565b9055565b916020611f33929493611f2c60408201965f83019061052b565b019061052b565b565b80611f48611f42426104a2565b916104a2565b1115611ffe5780611f81611f7b7f00000000000000000000000000000000000000000000000000000000000000006104a2565b916104a2565b11611fe257611f90600c61171b565b611f9b82600c611ef2565b903390611fc87fdd6896dcf1d4b311cca87dd19bbba2ea9ce2f867c1568878a0438a66a1afeeec926118cc565b92611fdd611fd4610302565b92839283611f12565b0390a2565b5f63ef69af6560e01b815280611ffa600482016106ba565b0390fd5b5f63a565835360e01b815280612016600482016106ba565b0390fd5b61202390611ea2565b565b5f90565b606090565b612037906118c0565b90565b67ffffffffffffffff81116120525760208091020190565b61165b565b906120696120648361203a565b611aea565b918252565b369037565b9061209861208083612057565b9260208061208e869361203a565b920191039061206e565b565b600f60f81b90565b6120aa612025565b506120b361156d565b506120bc61156d565b506120c56116fe565b506120ce611bda565b506120d761176d565b506120e0612029565b506120e9612e70565b906120f2612eb0565b9046906120fe3061202e565b906121085f610ffb565b9061211a6121155f61198b565b612073565b9061212361209a565b96959493929190565b61215561215a9161213b6116fe565b5061214f612149600b6118ee565b91612a52565b90612b67565b6118f1565b90565b90612167906118cc565b5f5260205260405f2090565b60ff1690565b61218561218a91611702565b612173565b90565b6121979054612179565b90565b6121c1915f6121b66121bc936121ae611529565b50600561177d565b0161215d565b61218d565b90565b5f90565b6121d06121c4565b506121d9612cec565b90565b6121e461156d565b506121ef60046116ba565b90565b61221961221461220f61221e936122076116fe565b50600a6118d8565b6118ee565b612ef0565b6118f1565b90565b61223e9161222d611529565b5061223661262c565b919091612715565b600190565b90612256916122506116fe565b5061190d565b90565b61226b906122656116fe565b506121f2565b90565b6122766116fe565b5061227f611728565b90565b7fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf90565b6122db6122e2946122d16060949897956122c7608086019a5f870190610636565b602085019061091c565b604083019061052b565b019061052b565b565b60200190565b5190565b9395949092919542612308612302896104a2565b916104a2565b1161238157916123739161237a9361236a61237f9899612352612329612282565b6123438b938b612337610302565b958694602086016122a6565b6020820181038252038261166f565b61236461235e826122ea565b916122e4565b20612f65565b92909192612f82565b9182612fcc565b612d3f565b565b61239c875f918291632341d78760e11b835260048301610538565b0390fd5b7f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c990565b919461240c6124169298979561240260a0966123f861241d9a6123ee60c08a019e5f8b0190610636565b602089019061091c565b604087019061091c565b606085019061052b565b608083019061052b565b019061052b565b565b91602061244092949361243960408201965f83019061091c565b019061091c565b565b96959193929490944261245d612457836104a2565b916104a2565b1161251757906124c66124cf9493926124ae6124776123a0565b61249f8c80948c916124898d9161301e565b9192612493610302565b978896602088016123c4565b6020820181038252038261166f565b6124c06124ba826122ea565b916122e4565b20612f65565b92909192612f82565b806124e26124dc87610473565b91610473565b036124f757506124f59293919091612639565b565b84906125135f9283926325c0072360e11b84526004840161241f565b0390fd5b612532905f91829163313c898160e11b835260048301610538565b0390fd5b906125519161254c612547826117b7565b6127b2565b612553565b565b9061255d91612971565b50565b9061256a91612536565b565b90612576906118cc565b5f5260205260405f2090565b6125a79161259d6125a2926125956116fe565b50600161256c565b611c6b565b61171b565b90565b6125b46040611aea565b90565b5f90565b5f90565b6125c76125aa565b90602080836125d46125b7565b8152016125df6125bb565b81525050565b6125ed6125bf565b90565b90612603916125fd6125e5565b50613051565b90565b61260e611529565b506126286126226301ffc9a760e01b610310565b91610310565b1490565b612634611bda565b503390565b916126479291600192613079565b565b604090612672612679949695939661266860608401985f85019061091c565b602083019061052b565b019061052b565b565b9061268691036104a2565b90565b929192612697818390612582565b90816126ac6126a65f196104a2565b916104a2565b106126b9575b5050509050565b816126cc6126c6876104a2565b916104a2565b106126f2576126e993946126e191939261267b565b905f92613079565b805f80806126b2565b50612711849291925f938493637dc7a0d960e11b855260048501612649565b0390fd5b918261273161272b6127265f61197f565b610473565b91610473565b1461278b578161275161274b6127465f61197f565b610473565b91610473565b146127645761276292919091613188565b565b6127876127705f61197f565b5f91829163ec442f0560e01b835260048301610929565b0390fd5b6127ae6127975f61197f565b5f918291634b637e8f60e11b835260048301610929565b0390fd5b6127c4906127be61262c565b90613255565b565b906127d260ff91610ff6565b9181191691161790565b6127e59061035d565b90565b90565b906128006127fb612807926127dc565b6127e8565b82546127c6565b9055565b612813611529565b5061282861282282849061219a565b1561035d565b5f146128b157612850600161284b5f6128436005869061177d565b01859061215d565b6127eb565b9061285961262c565b9061289661289061288a7f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d95611771565b926118cc565b926118cc565b9261289f610302565b806128a9816106ba565b0390a4600190565b50505f90565b6128bf61176d565b506128c93061202e565b6128fb6128f57f0000000000000000000000000000000000000000000000000000000000000000610473565b91610473565b1480612937575b5f1461292c577f000000000000000000000000000000000000000000000000000000000000000090565b612934613301565b90565b504661296b6129657f00000000000000000000000000000000000000000000000000000000000000006104a2565b916104a2565b14612902565b612979611529565b5061298581839061219a565b5f14612a0d576129ac5f6129a75f61299f6005869061177d565b01859061215d565b6127eb565b906129b561262c565b906129f26129ec6129e67ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b95611771565b926118cc565b926118cc565b926129fb610302565b80612a05816106ba565b0390a4600190565b50505f90565b612a27612a22612a2c92610eb0565b610890565b6104a2565b90565b916020612a50929493612a4960408201965f83019061052b565b0190610ebb565b565b612a5a6121c4565b50612a636121c8565b81612a76612a7083612a13565b916104a2565b1015612a895750612a869061340a565b90565b90612aa45f928392637669fc0f60e11b845260048401612a2f565b0390fd5b5490565b90565b612ac3612abe612ac892612aac565b610890565b6104a2565b90565b90565b65ffffffffffff1690565b612ae5612aea91611702565b612ace565b90565b612af79054612ad9565b90565b90565b612b11612b0c612b1692612afa565b610890565b6104a2565b90565b60301c90565b60018060d01b031690565b612b36612b3b91612b19565b612b1f565b90565b612b489054612b2a565b90565b612b5f612b5a612b6492610ff3565b610890565b611409565b90565b90612bbb90612b74611494565b50612b805f8401612aa8565b612b895f61198b565b908080612b9f612b996005612aaf565b916104a2565b11612c1c575b5090612bb65f860193919293612acb565b613a5d565b80612bce612bc85f61198b565b916104a2565b145f14612be4575050612be05f612b4b565b5b90565b612c115f91612c0c612c0684612c17960192612c006001612afd565b90611aa0565b91612acb565b613a53565b01612b3e565b612be1565b80612c2a612c3092916136e8565b90611aa0565b9083612c62612c5c612c575f612c51818c01612c4c8991612acb565b613a53565b01612aed565b610eb0565b91610eb0565b105f14612c735750905b905f612ba5565b9150612c8990612c836001612afd565b906119a7565b612c6c565b80612ca9612ca3612c9e5f61197f565b610473565b91610473565b14612cc557612cc391612cbb5f61197f565b919091613188565b565b612ce8612cd15f61197f565b5f91829163ec442f0560e01b835260048301610929565b0390fd5b612cf46121c4565b50612cfe4361340a565b90565b90612d1260018060a01b0391610ff6565b9181191691161790565b90565b90612d34612d2f612d3b926118cc565b612d1c565b8254612d01565b9055565b90612dc891612dc2612d5082611c20565b612d6584612d6060098690611bde565b612d1f565b82818590612da5612d9f612d997f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f956118cc565b926118cc565b926118cc565b92612dae610302565b80612db8816106ba565b0390a49291613aec565b91613b04565b565b612df1612dec612de7612df693612ddf611c52565b50600a6118d8565b6118ee565b613cb2565b613d31565b90565b9081612e15612e0f612e0a5f61197f565b610473565b91610473565b14612e3157612e2f9190612e285f61197f565b9091613188565b565b612e54612e3d5f61197f565b5f918291634b637e8f60e11b835260048301610929565b0390fd5b612e6a90612e646116fe565b50613d82565b90565b90565b612e7861156d565b50612ead7f0000000000000000000000000000000000000000000000000000000000000000612ea76006612e6d565b90613e9d565b90565b612eb861156d565b50612eed7f0000000000000000000000000000000000000000000000000000000000000000612ee76007612e6d565b90613e9d565b90565b612ef8611494565b50612f045f8201612aa8565b80612f17612f115f61198b565b916104a2565b145f14612f2d575050612f295f612b4b565b5b90565b612f5a5f91612f55612f4f84612f60960192612f496001612afd565b90611aa0565b91612acb565b613a53565b01612b3e565b612f2a565b612f7f90612f7161176d565b50612f7a6128b7565b613eeb565b90565b92612f9d92612fa694612f93611bda565b5092909192613fb1565b909291926140dc565b90565b916020612fca929493612fc360408201965f83019061091c565b019061052b565b565b612fd58161301e565b91612fe8612fe2846104a2565b916104a2565b03612ff1575050565b61300b5f9283926301d4b62360e61b845260048401612fa9565b0390fd5b600161301b91016104a2565b90565b6130329061302a6116fe565b506008611c6b565b61304e61303e8261171b565b916130488361300f565b90611ef2565b90565b9061307161306c613076936130646125e5565b50600a6118d8565b6118ee565b614252565b90565b90928161309661309061308b5f61197f565b610473565b91610473565b1461316157836130b66130b06130ab5f61197f565b610473565b91610473565b1461313a576130da836130d56130ce6001869061256c565b8790611c6b565b611ef2565b6130e4575b505050565b91909161312f61311d6131177f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925936118cc565b936118cc565b93613126610302565b91829182610538565b0390a35f80806130df565b61315d6131465f61197f565b5f918291634a1406b160e11b835260048301610929565b0390fd5b61318461316d5f61197f565b5f91829163e602df0560e01b835260048301610929565b0390fd5b91826131a461319e6131995f61197f565b610473565b91610473565b14158061320f575b6131bf575b6131bd92919091614273565b565b6131c7611e4f565b806131ee575b156131b1575f6336e278fd60e21b8152806131ea600482016106ba565b0390fd5b5061320a6132046131fd610d06565b339061219a565b1561035d565b6131cd565b508161322b6132256132205f61197f565b610473565b91610473565b14156131ac565b91602061325392949361324c60408201965f83019061091c565b0190610636565b565b9061326a61326483839061219a565b1561035d565b613272575050565b61328c5f92839263e2517d3f60e01b845260048401613232565b0390fd5b7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f90565b909594926132ff946132ee6132f8926132e46080966132da60a088019c5f890190610636565b6020870190610636565b6040850190610636565b606083019061052b565b019061091c565b565b61330961176d565b50613312613290565b6133897f00000000000000000000000000000000000000000000000000000000000000009161337a7f0000000000000000000000000000000000000000000000000000000000000000466133653061202e565b9161336e610302565b968795602087016132b4565b6020820181038252038261166f565b61339b613395826122ea565b916122e4565b2090565b90565b6133b66133b16133bb9261339f565b610890565b6106f3565b90565b6133c7906133a2565b9052565b9160206133ec9294936133e560408201965f8301906133be565b019061052b565b565b6134026133fd613407926104a2565b610890565b610eb0565b90565b6134126121c4565b508061342c61342665ffffffffffff612a13565b916104a2565b1161343d5761343a906133ee565b90565b60306134595f9283926306dfcc6560e41b8452600484016133cb565b0390fd5b90565b61347461346f6134799261345d565b610890565b6104a2565b90565b90565b61349361348e6134989261347c565b610890565b6106f3565b90565b6134ba906134b46134ae6134bf946106f3565b916104a2565b90611099565b6104a2565b90565b90565b6134d96134d46134de926134c2565b610890565b6106f3565b90565b1b90565b613504906134fe6134f8613509946106f3565b916104a2565b906134e1565b6104a2565b90565b90565b61352361351e6135289261350c565b610890565b6104a2565b90565b90565b61354261353d6135479261352b565b610890565b6106f3565b90565b90565b61356161355c6135669261354a565b610890565b6104a2565b90565b90565b61358061357b61358592613569565b610890565b6106f3565b90565b90565b61359f61359a6135a492613588565b610890565b6104a2565b90565b90565b6135be6135b96135c3926135a7565b610890565b6106f3565b90565b90565b6135dd6135d86135e2926135c6565b610890565b6104a2565b90565b90565b6135fc6135f7613601926135e5565b610890565b6106f3565b90565b61361861361361361d92613569565b610890565b6104a2565b90565b90565b61363761363261363c92613620565b610890565b6106f3565b90565b61365361364e613658926135e5565b610890565b6104a2565b90565b61366f61366a61367492612afa565b610890565b6106f3565b90565b90565b61368e61368961369392613677565b610890565b6104a2565b90565b906136a191026104a2565b90565b634e487b7160e01b5f52601260045260245ffd5b6136c46136ca916104a2565b916104a2565b9081156136d5570490565b6136a4565b906136e591016104a2565b90565b6136f06116fe565b50806137056136ff6001612afd565b916104a2565b1115613a50578061391a6138f76138e76138d76138c76138b76138a76138976138876138776138678b61386161385a6139209f61383a61382a61384a9261374c6001612afd565b908061376461375e600160801b613460565b916104a2565b1015613a22575b806137876137816801000000000000000061350f565b916104a2565b10156139f4575b806137a66137a064010000000061354d565b916104a2565b10156139c6575b806137c36137bd6201000061358b565b916104a2565b1015613998575b806137df6137d96101006135c9565b916104a2565b101561396a575b806137fa6137f46010613604565b916104a2565b101561393c575b61381461380e600461363f565b916104a2565b1015613923575b613825600361367a565b613696565b613834600161365b565b9061349b565b61384481866136b8565b906136da565b613854600161365b565b9061349b565b80926136b8565b906136da565b613871600161365b565b9061349b565b613881818c6136b8565b906136da565b613891600161365b565b9061349b565b6138a1818a6136b8565b906136da565b6138b1600161365b565b9061349b565b6138c181886136b8565b906136da565b6138d1600161365b565b9061349b565b6138e181866136b8565b906136da565b6138f1600161365b565b9061349b565b9161391461390e6139098580946136b8565b6104a2565b916104a2565b11614303565b9061267b565b90565b61393790613931600161365b565b906134e5565b61381b565b6139536139649161394d60046135e8565b9061349b565b9161395e6002613623565b906134e5565b90613801565b6139816139929161397b60086135aa565b9061349b565b9161398c60046135e8565b906134e5565b906137e6565b6139af6139c0916139a9601061356c565b9061349b565b916139ba60086135aa565b906134e5565b906137ca565b6139dd6139ee916139d7602061352e565b9061349b565b916139e8601061356c565b906134e5565b906137ad565b613a0b613a1c91613a0560406134c5565b9061349b565b91613a16602061352e565b906134e5565b9061378e565b613a39613a4a91613a33608061347f565b9061349b565b91613a4460406134c5565b906134e5565b9061376b565b90565b5f5260205f200190565b93919092613a696116fe565b505b81613a7e613a78836104a2565b916104a2565b1015613ae457613a8f82829061434f565b90613aa55f613a9f888590613a53565b01612aed565b613ab7613ab187610eb0565b91610eb0565b115f14613ac75750915b91613a6b565b929150613ade90613ad86001612afd565b906119a7565b90613ac1565b925050915090565b613afe90613af86116fe565b50611c81565b90565b90565b91909180613b1a613b1485610473565b91610473565b141580613c98575b613b2c575b505050565b80613b47613b41613b3c5f61197f565b610473565b91610473565b03613c08575b5081613b69613b63613b5e5f61197f565b610473565b91610473565b03613b75575b80613b27565b613bbc613baf613bb692613b8b600a86906118d8565b90613ba9613ba3613b9d6001936143e8565b936118ee565b91613b01565b9061443b565b92906118f1565b916118f1565b919091613be97fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926118cc565b92613bfe613bf5610302565b92839283611f12565b0390a25f80613b6f565b613c47613c4d613c40613c1d600a85906118d8565b6002613c3a613c34613c2e896143e8565b936118ee565b91613b01565b9061443b565b92906118f1565b916118f1565b919091613c7a7fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926118cc565b92613c8f613c86610302565b92839283611f12565b0390a25f613b4d565b5081613cac613ca65f61198b565b916104a2565b11613b22565b5f613cc691613cbf6116fe565b5001612aa8565b90565b613cdd613cd8613ce2926109a6565b610890565b6104a2565b90565b613cee9061352e565b9052565b916020613d13929493613d0c60408201965f830190613ce5565b019061052b565b565b613d29613d24613d2e926104a2565b610890565b6109a6565b90565b613d39611c52565b5080613d51613d4b63ffffffff613cc9565b916104a2565b11613d6257613d5f90613d15565b90565b6020613d7e5f9283926306dfcc6560e41b845260048401613cf2565b0390fd5b613d99613d9e91613d916116fe565b506008611c6b565b61171b565b90565b90565b613db8613db3613dbd92613da1565b610ff6565b6105f2565b90565b613dca60ff613da4565b90565b5f5260205f2090565b905f9291805490613df0613de983611586565b80946115b0565b916001811690815f14613e475750600114613e0b575b505050565b613e189192939450613dcd565b915f925b818410613e2f57505001905f8080613e06565b60018160209295939554848601520191019290613e1c565b92949550505060ff19168252151560200201905f8080613e06565b90613e6c91613dd6565b90565b90613e8f613e8892613e7f610302565b93848092613e62565b038361166f565b565b613e9a90613e6f565b90565b90613ea661156d565b50613eb082611771565b613ec9613ec3613ebe613dc0565b6105f2565b916105f2565b14155f14613ede5750613edb906144c5565b90565b613ee89150613e91565b90565b604291613ef661176d565b50604051917f19010000000000000000000000000000000000000000000000000000000000008352600283015260228201522090565b5f90565b613f3c613f4191611702565b611ed3565b90565b90565b613f5b613f56613f6092613f44565b610890565b6104a2565b90565b613f98613f9f94613f8e606094989795613f84608086019a5f870190610636565b60208501906106f9565b6040830190610636565b0190610636565b565b613fa9610302565b3d5f823e3d90fd5b939293613fbc611bda565b50613fc5613f2c565b50613fce61176d565b50613fd885613f30565b61400a6140047f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0613f47565b916104a2565b11614097579061402d602094955f94939293614024610302565b94859485613f63565b838052039060015afa15614092576140455f51610ff6565b8061406061405a6140555f61197f565b610473565b91610473565b14614076575f916140705f610ffb565b91929190565b506140805f61197f565b60019161408c5f610ffb565b91929190565b613fa1565b5050506140a35f61197f565b9060039291929190565b634e487b7160e01b5f52602160045260245ffd5b600411156140cb57565b6140ad565b906140da826140c1565b565b806140ef6140e95f6140d0565b916140d0565b145f146140fa575050565b8061410e61410860016140d0565b916140d0565b145f14614131575f63f645eedf60e01b81528061412d600482016106ba565b0390fd5b8061414561413f60026140d0565b916140d0565b145f146141735761416f61415883613f30565b5f91829163fce698f760e01b835260048301610538565b0390fd5b61418661418060036140d0565b916140d0565b1461418e5750565b6141a9905f9182916335e2f38360e21b835260048301610643565b0390fd5b634e487b7160e01b5f52603260045260245ffd5b5f5260205f2090565b6141d381612aa8565b8210156141ed576141e56001916141c1565b910201905f90565b6141ad565b906141fc90610eb0565b9052565b9061420a90611409565b9052565b9061424461423b5f61421e6125aa565b9461423561422d838301612aed565b8388016141f2565b01612b3e565b60208401614200565b565b61424f9061420e565b90565b614270915f61426a926142636125e5565b50016141ca565b50614246565b90565b9291614281848383916144f5565b8361429c6142966142915f61197f565b610473565b91610473565b146142b1575b6142af929391909161467f565b565b6142b9611728565b936142c2614664565b94806142d66142d0886104a2565b916104a2565b116142e3575093506142a2565b85906142ff5f928392630e58ae9360e11b845260048401611f12565b0390fd5b61430b6116fe565b50151590565b61432561432061432a92613620565b610890565b6104a2565b90565b61433961433f916104a2565b916104a2565b90811561434a570490565b6136a4565b61437461437a9261435e6116fe565b50828116921861436e6002614311565b9061432d565b906119a7565b90565b90565b61439461438f6143999261437d565b610890565b6106f3565b90565b6143a590614380565b9052565b9160206143ca9294936143c360408201965f83019061439c565b019061052b565b565b6143e06143db6143e5926104a2565b610890565b611409565b90565b6143f0611494565b508061440a61440460018060d01b036118f1565b916104a2565b1161441b57614418906143cc565b90565b60d06144375f9283926306dfcc6560e41b8452600484016143a9565b0390fd5b90614471614477939261444c611494565b50614455611494565b50809361446a6144636121c8565b9492612ef0565b9091614afa565b9161473e565b91909190565b61449161448c6144969261352b565b610890565b6104a2565b90565b369037565b906144c36144ab83611b22565b926020806144b98693611aff565b9201910390614499565b565b6144cd61156d565b506144d7816147a8565b906144ea6144e5602061447d565b61449e565b918252602082015290565b9190918061451361450d6145085f61197f565b610473565b91610473565b145f146145f4576145376145308361452b600261171b565b6119a7565b6002611ef2565b5b8261455361454d6145485f61197f565b610473565b91610473565b145f146145c8576145776145708361456b600261171b565b61267b565b6002611ef2565b5b9190916145c36145b16145ab7fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef936118cc565b936118cc565b936145ba610302565b91829182610538565b0390a3565b6145ef826145e96145da5f8790611c6b565b916145e48361171b565b6136da565b90611ef2565b614578565b6146076146025f8390611c6b565b61171b565b8061461a614614856104a2565b916104a2565b106146425761462d61463d91849061267b565b6146385f8490611c6b565b611ef2565b614538565b906146609091925f93849363391434e360e21b855260048501612649565b0390fd5b61466c6116fe565b5061467c60018060d01b036118f1565b90565b916146d76146d16146de94806146a561469f61469a5f61197f565b610473565b91610473565b1461470f575b846146c66146c06146bb5f61197f565b610473565b91610473565b146146e0575b611c20565b92611c20565b9091613b04565b565b614708600b60026147026146fc6146f6896143e8565b936118ee565b91613b01565b9061443b565b50506146cc565b614737600b600161473161472b614725896143e8565b936118ee565b91613b01565b9061443b565b50506146ab565b916147625f6147679461474f611494565b50614758611494565b5001929192612acb565b6149ac565b91909190565b61478161477c61478692613da1565b610890565b6104a2565b90565b90565b6147a061479b6147a592614789565b610890565b6104a2565b90565b6147bd6147c2916147b76116fe565b50611771565b613f30565b6147cc60ff61476d565b16806147e16147db601f61478c565b916104a2565b116147e95790565b5f632cd44ac360e21b815280614801600482016106ba565b0390fd5b5490565b6148136040611aea565b90565b5f5260205f2090565b61482881614805565b8210156148425761483a600191614816565b910201905f90565b6141ad565b634e487b7160e01b5f525f60045260245ffd5b6148649051610eb0565b90565b9061487865ffffffffffff91610ff6565b9181191691161790565b61489661489161489b92610eb0565b610890565b610eb0565b90565b90565b906148b66148b16148bd92614882565b61489e565b8254614867565b9055565b6148cb9051611409565b90565b60301b90565b906148e665ffffffffffff19916148ce565b9181191691161790565b6149046148ff61490992611409565b610890565b611409565b90565b90565b9061492461491f61492b926148f0565b61490c565b82546148d4565b9055565b9061495960205f61495f9461495182820161494b84880161485a565b906148a1565b0192016148c1565b9061490f565b565b9190614972576149709161492f565b565b614847565b90815491680100000000000000008310156149a7578261499f9160016149a59501815561481f565b90614961565b565b61165b565b909291926149b8611494565b506149c1611494565b506149cb82614805565b806149de6149d85f61198b565b916104a2565b115f14614aae57614a04906149fe84916149f86001612afd565b90611aa0565b90613a53565b90614a105f8301612aed565b92614a1c5f8401612b3e565b9380614a30614a2a85610eb0565b91610eb0565b11614a9257614a47614a4184610eb0565b91610eb0565b145f14614a62575050614a5d905f85910161490f565b5b9190565b614a8d9250614a8886614a7f614a76614809565b945f86016141f2565b60208401614200565b614977565b614a5e565b5f632520601d60e01b815280614aaa600482016106ba565b0390fd5b50614ad991614ad485614acb614ac2614809565b945f86016141f2565b60208401614200565b614977565b614ae25f612b4b565b9190565b634e487b7160e01b5f52605160045260245ffd5b91909180600114614b1957600203614ae657614b1591611513565b905b565b50614b23916114d4565b90614b1756
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\x14\x90V[a\0\x1D_5a\x02\xFCV[\x80c\x01\xFF\xC9\xA7\x14a\x02\xF7W\x80c\x06\xFD\xDE\x03\x14a\x02\xF2W\x80c\t^\xA7\xB3\x14a\x02\xEDW\x80c\x18\x16\r\xDD\x14a\x02\xE8W\x80c#\xB8r\xDD\x14a\x02\xE3W\x80c$\x8A\x9C\xA3\x14a\x02\xDEW\x80c//\xF1]\x14a\x02\xD9W\x80c1<\xE5g\x14a\x02\xD4W\x80c6D\xE5\x15\x14a\x02\xCFW\x80c6V\x8A\xBE\x14a\x02\xCAW\x80c:F\xB1\xA8\x14a\x02\xC5W\x80c@\xC1\x0F\x19\x14a\x02\xC0W\x80cK\xDD6\xCE\x14a\x02\xBBW\x80cK\xF5\xD7\xE9\x14a\x02\xB6W\x80cO\x1B\xFC\x9E\x14a\x02\xB1W\x80cX|\xDE\x1E\x14a\x02\xACW\x80c\\\x19\xA9\\\x14a\x02\xA7W\x80co\xCF\xFFE\x14a\x02\xA2W\x80cp\xA0\x821\x14a\x02\x9DW\x80cy\xCCg\x90\x14a\x02\x98W\x80cz\x8C\xD1V\x14a\x02\x93W\x80c~\xCE\xBE\0\x14a\x02\x8EW\x80c\x83\xF1!\x1B\x14a\x02\x89W\x80c\x84&\xAD\xF2\x14a\x02\x84W\x80c\x84L\x90&\x14a\x02\x7FW\x80c\x84\xB0\x19n\x14a\x02zW\x80c\x8AT%!\x14a\x02uW\x80c\x8D3C\xD6\x14a\x02pW\x80c\x8ES\x9E\x8C\x14a\x02kW\x80c\x90-U\xA5\x14a\x02fW\x80c\x91\xD1HT\x14a\x02aW\x80c\x91\xDD\xAD\xF4\x14a\x02\\W\x80c\x95\xD8\x9BA\x14a\x02WW\x80c\x9A\xB2N\xB0\x14a\x02RW\x80c\x9B~\xF6K\x14a\x02MW\x80c\xA2\x17\xFD\xDF\x14a\x02HW\x80c\xA9\x05\x9C\xBB\x14a\x02CW\x80c\xAA\x08*\x9D\x14a\x02>W\x80c\xB0\xCA%>\x14a\x029W\x80c\xBBMD6\x14a\x024W\x80c\xC0*\xE7T\x14a\x02/W\x80c\xC3\xCD\xA5 \x14a\x02*W\x80c\xD5\x05\xAC\xCF\x14a\x02%W\x80c\xD5Gt\x1F\x14a\x02 W\x80c\xDDb\xED>\x14a\x02\x1BWc\xF1\x12~\xD8\x03a\0\x0EWa\x14ZV[a\x13vV[a\x13\x15V[a\x12\xDBV[a\x121V[a\x11uV[a\x11@V[a\x11\nV[a\x10\xD5V[a\x10cV[a\x10.V[a\x0F\xBEV[a\x0FGV[a\x0F\x12V[a\x0E\xDDV[a\x0EzV[a\x0EEV[a\r\xCEV[a\r\x99V[a\r5V[a\x0C\xCAV[a\x0B\x85V[a\x0B2V[a\n\xD9V[a\n\xA4V[a\noV[a\n;V[a\n\x06V[a\t\xD1V[a\tsV[a\t>V[a\x08\xC9V[a\x08XV[a\x08#V[a\x07\xEFV[a\x07\xB9V[a\x07\x85V[a\x07PV[a\x07\x1BV[a\x06\xBFV[a\x06XV[a\x05\xBCV[a\x05MV[a\x04\xF5V[a\x043V[a\x03\x84V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[c\xFF\xFF\xFF\xFF`\xE0\x1B\x16\x90V[a\x03%\x81a\x03\x10V[\x03a\x03,WV[_\x80\xFD[\x90P5\x90a\x03=\x82a\x03\x1CV[V[\x90` \x82\x82\x03\x12a\x03XWa\x03U\x91_\x01a\x030V[\x90V[a\x03\x0CV[\x15\x15\x90V[a\x03k\x90a\x03]V[\x90RV[\x91\x90a\x03\x82\x90_` \x85\x01\x94\x01\x90a\x03bV[V[4a\x03\xB4Wa\x03\xB0a\x03\x9Fa\x03\x9A6`\x04a\x03?V[a\x15-V[a\x03\xA7a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[_\x91\x03\x12a\x03\xC3WV[a\x03\x0CV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x04\ta\x04\x12` \x93a\x04\x17\x93a\x04\0\x81a\x03\xC8V[\x93\x84\x80\x93a\x03\xCCV[\x95\x86\x91\x01a\x03\xD5V[a\x03\xE0V[\x01\x90V[a\x040\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03\xEAV[\x90V[4a\x04cWa\x04C6`\x04a\x03\xB9V[a\x04_a\x04Na\x16\xC6V[a\x04Va\x03\x02V[\x91\x82\x91\x82a\x04\x1BV[\x03\x90\xF3[a\x03\x08V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04|\x90a\x04hV[\x90V[a\x04\x88\x81a\x04sV[\x03a\x04\x8FWV[_\x80\xFD[\x90P5\x90a\x04\xA0\x82a\x04\x7FV[V[\x90V[a\x04\xAE\x81a\x04\xA2V[\x03a\x04\xB5WV[_\x80\xFD[\x90P5\x90a\x04\xC6\x82a\x04\xA5V[V[\x91\x90`@\x83\x82\x03\x12a\x04\xF0W\x80a\x04\xE4a\x04\xED\x92_\x86\x01a\x04\x93V[\x93` \x01a\x04\xB9V[\x90V[a\x03\x0CV[4a\x05&Wa\x05\"a\x05\x11a\x05\x0B6`\x04a\x04\xC8V[\x90a\x16\xDCV[a\x05\x19a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[a\x054\x90a\x04\xA2V[\x90RV[\x91\x90a\x05K\x90_` \x85\x01\x94\x01\x90a\x05+V[V[4a\x05}Wa\x05]6`\x04a\x03\xB9V[a\x05ya\x05ha\x17(V[a\x05pa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90\x91``\x82\x84\x03\x12a\x05\xB7Wa\x05\xB4a\x05\x9D\x84_\x85\x01a\x04\x93V[\x93a\x05\xAB\x81` \x86\x01a\x04\x93V[\x93`@\x01a\x04\xB9V[\x90V[a\x03\x0CV[4a\x05\xEDWa\x05\xE9a\x05\xD8a\x05\xD26`\x04a\x05\x82V[\x91a\x17>V[a\x05\xE0a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[\x90V[a\x05\xFE\x81a\x05\xF2V[\x03a\x06\x05WV[_\x80\xFD[\x90P5\x90a\x06\x16\x82a\x05\xF5V[V[\x90` \x82\x82\x03\x12a\x061Wa\x06.\x91_\x01a\x06\tV[\x90V[a\x03\x0CV[a\x06?\x90a\x05\xF2V[\x90RV[\x91\x90a\x06V\x90_` \x85\x01\x94\x01\x90a\x066V[V[4a\x06\x88Wa\x06\x84a\x06sa\x06n6`\x04a\x06\x18V[a\x17\xB7V[a\x06{a\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[\x91\x90`@\x83\x82\x03\x12a\x06\xB5W\x80a\x06\xA9a\x06\xB2\x92_\x86\x01a\x06\tV[\x93` \x01a\x04\x93V[\x90V[a\x03\x0CV[_\x01\x90V[4a\x06\xEEWa\x06\xD8a\x06\xD26`\x04a\x06\x8DV[\x90a\x18\x03V[a\x06\xE0a\x03\x02V[\x80a\x06\xEA\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[`\xFF\x16\x90V[a\x07\x02\x90a\x06\xF3V[\x90RV[\x91\x90a\x07\x19\x90_` \x85\x01\x94\x01\x90a\x06\xF9V[V[4a\x07KWa\x07+6`\x04a\x03\xB9V[a\x07Ga\x076a\x182V[a\x07>a\x03\x02V[\x91\x82\x91\x82a\x07\x06V[\x03\x90\xF3[a\x03\x08V[4a\x07\x80Wa\x07`6`\x04a\x03\xB9V[a\x07|a\x07ka\x18HV[a\x07sa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[4a\x07\xB4Wa\x07\x9Ea\x07\x986`\x04a\x06\x8DV[\x90a\x18\\V[a\x07\xA6a\x03\x02V[\x80a\x07\xB0\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\x07\xEAWa\x07\xE6a\x07\xD5a\x07\xCF6`\x04a\x04\xC8V[\x90a\x19\rV[a\x07\xDDa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x08\x1EWa\x08\x08a\x08\x026`\x04a\x04\xC8V[\x90a\x1A\x94V[a\x08\x10a\x03\x02V[\x80a\x08\x1A\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\x08SWa\x0836`\x04a\x03\xB9V[a\x08Oa\x08>a\x1A\xC5V[a\x08Fa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x08\x88Wa\x08h6`\x04a\x03\xB9V[a\x08\x84a\x08sa\x1B\x84V[a\x08{a\x03\x02V[\x91\x82\x91\x82a\x04\x1BV[\x03\x90\xF3[a\x03\x08V[\x90V[\x90V[a\x08\xA7a\x08\xA2a\x08\xAC\x92a\x08\x8DV[a\x08\x90V[a\x04\xA2V[\x90V[a\x08\xBBbv\xA7\0a\x08\x93V[\x90V[a\x08\xC6a\x08\xAFV[\x90V[4a\x08\xF9Wa\x08\xD96`\x04a\x03\xB9V[a\x08\xF5a\x08\xE4a\x08\xBEV[a\x08\xECa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90` \x82\x82\x03\x12a\t\x17Wa\t\x14\x91_\x01a\x04\x93V[\x90V[a\x03\x0CV[a\t%\x90a\x04sV[\x90RV[\x91\x90a\t<\x90_` \x85\x01\x94\x01\x90a\t\x1CV[V[4a\tnWa\tja\tYa\tT6`\x04a\x08\xFEV[a\x1C V[a\taa\x03\x02V[\x91\x82\x91\x82a\t)V[\x03\x90\xF3[a\x03\x08V[4a\t\xA1Wa\t\x8Ba\t\x866`\x04a\x08\xFEV[a\x1C?V[a\t\x93a\x03\x02V[\x80a\t\x9D\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[c\xFF\xFF\xFF\xFF\x16\x90V[a\t\xB8\x90a\t\xA6V[\x90RV[\x91\x90a\t\xCF\x90_` \x85\x01\x94\x01\x90a\t\xAFV[V[4a\n\x01Wa\t\xFDa\t\xECa\t\xE76`\x04a\x08\xFEV[a\x1CVV[a\t\xF4a\x03\x02V[\x91\x82\x91\x82a\t\xBCV[\x03\x90\xF3[a\x03\x08V[4a\n6Wa\n2a\n!a\n\x1C6`\x04a\x08\xFEV[a\x1C\x81V[a\n)a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\njWa\nTa\nN6`\x04a\x04\xC8V[\x90a\x1D\xB6V[a\n\\a\x03\x02V[\x80a\nf\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\n\x9FWa\n\x7F6`\x04a\x03\xB9V[a\n\x9Ba\n\x8Aa\x1D\xC2V[a\n\x92a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\n\xD4Wa\n\xD0a\n\xBFa\n\xBA6`\x04a\x08\xFEV[a\x1E:V[a\n\xC7a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x0B\tWa\n\xE96`\x04a\x03\xB9V[a\x0B\x05a\n\xF4a\x1EOV[a\n\xFCa\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0BbWa\x0BB6`\x04a\x03\xB9V[a\x0B^a\x0BMa\x0B\x0EV[a\x0BUa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90` \x82\x82\x03\x12a\x0B\x80Wa\x0B}\x91_\x01a\x04\xB9V[\x90V[a\x03\x0CV[4a\x0B\xB3Wa\x0B\x9Da\x0B\x986`\x04a\x0BgV[a \x1AV[a\x0B\xA5a\x03\x02V[\x80a\x0B\xAF\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[`\xFF`\xF8\x1B\x16\x90V[a\x0B\xCA\x90a\x0B\xB8V[\x90RV[Q\x90V[` \x91\x81R\x01\x90V[` \x01\x90V[a\x0B\xEA\x90a\x04\xA2V[\x90RV[\x90a\x0B\xFB\x81` \x93a\x0B\xE1V[\x01\x90V[` \x01\x90V[\x90a\x0C\"a\x0C\x1Ca\x0C\x15\x84a\x0B\xCEV[\x80\x93a\x0B\xD2V[\x92a\x0B\xDBV[\x90_[\x81\x81\x10a\x0C2WPPP\x90V[\x90\x91\x92a\x0CKa\x0CE`\x01\x92\x86Qa\x0B\xEEV[\x94a\x0B\xFFV[\x91\x01\x91\x90\x91a\x0C%V[\x93\x95\x91\x94a\x0C\xA6a\x0C\x9Ba\x0C\xBA\x95a\x0C\x8Da\x0C\xB0\x95a\x0C\xC7\x9C\x9Aa\x0C\x80`\xE0\x8C\x01\x92_\x8D\x01\x90a\x0B\xC1V[\x8A\x82\x03` \x8C\x01Ra\x03\xEAV[\x90\x88\x82\x03`@\x8A\x01Ra\x03\xEAV[\x97``\x87\x01\x90a\x05+V[`\x80\x85\x01\x90a\t\x1CV[`\xA0\x83\x01\x90a\x066V[`\xC0\x81\x84\x03\x91\x01Ra\x0C\x05V[\x90V[4a\r\x01Wa\x0C\xDA6`\x04a\x03\xB9V[a\x0C\xFDa\x0C\xE5a \xA2V[\x93a\x0C\xF4\x97\x95\x97\x93\x91\x93a\x03\x02V[\x97\x88\x97\x88a\x0CUV[\x03\x90\xF3[a\x03\x08V[\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t\x90V[a\r2a\r\x06V[\x90V[4a\reWa\rE6`\x04a\x03\xB9V[a\raa\rPa\r*V[a\rXa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[\x7F\x9B\x12\xE0\xC5p~II\x15\xE5\x8B\x05d\xF1\x8A\xAA\xD9\xB7J\xC6\x9B\xFC\x81Z\x1E\xDA\xDC\x8EK\xD02\xEB\x90V[a\r\x96a\rjV[\x90V[4a\r\xC9Wa\r\xA96`\x04a\x03\xB9V[a\r\xC5a\r\xB4a\r\x8EV[a\r\xBCa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[4a\r\xFEWa\r\xFAa\r\xE9a\r\xE46`\x04a\x0BgV[a!,V[a\r\xF1a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90V[a\x0E\x1Aa\x0E\x15a\x0E\x1F\x92a\x0E\x03V[a\x08\x90V[a\x04\xA2V[\x90V[a\x0E7k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x0E\x06V[\x90V[a\x0EBa\x0E\"V[\x90V[4a\x0EuWa\x0EU6`\x04a\x03\xB9V[a\x0Eqa\x0E`a\x0E:V[a\x0Eha\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x0E\xABWa\x0E\xA7a\x0E\x96a\x0E\x906`\x04a\x06\x8DV[\x90a!\x9AV[a\x0E\x9Ea\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x0E\xC4\x90a\x0E\xB0V[\x90RV[\x91\x90a\x0E\xDB\x90_` \x85\x01\x94\x01\x90a\x0E\xBBV[V[4a\x0F\rWa\x0E\xED6`\x04a\x03\xB9V[a\x0F\ta\x0E\xF8a!\xC8V[a\x0F\0a\x03\x02V[\x91\x82\x91\x82a\x0E\xC8V[\x03\x90\xF3[a\x03\x08V[4a\x0FBWa\x0F\"6`\x04a\x03\xB9V[a\x0F>a\x0F-a!\xDCV[a\x0F5a\x03\x02V[\x91\x82\x91\x82a\x04\x1BV[\x03\x90\xF3[a\x03\x08V[4a\x0FwWa\x0Fsa\x0Fba\x0F]6`\x04a\x08\xFEV[a!\xF2V[a\x0Fja\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90V[a\x0F\x93a\x0F\x8Ea\x0F\x98\x92a\x0F|V[a\x08\x90V[a\x04\xA2V[\x90V[a\x0F\xB0k\x02\xE8vi\xC3\x08sj\x04\0\0\0a\x0F\x7FV[\x90V[a\x0F\xBBa\x0F\x9BV[\x90V[4a\x0F\xEEWa\x0F\xCE6`\x04a\x03\xB9V[a\x0F\xEAa\x0F\xD9a\x0F\xB3V[a\x0F\xE1a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[\x90V[_\x1B\x90V[a\x10\x0Fa\x10\na\x10\x14\x92a\x0F\xF3V[a\x0F\xF6V[a\x05\xF2V[\x90V[a\x10 _a\x0F\xFBV[\x90V[a\x10+a\x10\x17V[\x90V[4a\x10^Wa\x10>6`\x04a\x03\xB9V[a\x10Za\x10Ia\x10#V[a\x10Qa\x03\x02V[\x91\x82\x91\x82a\x06CV[\x03\x90\xF3[a\x03\x08V[4a\x10\x94Wa\x10\x90a\x10\x7Fa\x10y6`\x04a\x04\xC8V[\x90a\"!V[a\x10\x87a\x03\x02V[\x91\x82\x91\x82a\x03oV[\x03\x90\xF3[a\x03\x08V[\x1C\x90V[\x90V[a\x10\xB0\x90`\x08a\x10\xB5\x93\x02a\x10\x99V[a\x10\x9DV[\x90V[\x90a\x10\xC3\x91Ta\x10\xA0V[\x90V[a\x10\xD2`\x0C_\x90a\x10\xB8V[\x90V[4a\x11\x05Wa\x10\xE56`\x04a\x03\xB9V[a\x11\x01a\x10\xF0a\x10\xC6V[a\x10\xF8a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x11;Wa\x117a\x11&a\x11 6`\x04a\x04\xC8V[\x90a\"CV[a\x11.a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x11pWa\x11la\x11[a\x11V6`\x04a\x08\xFEV[a\"YV[a\x11ca\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[4a\x11\xA5Wa\x11\x856`\x04a\x03\xB9V[a\x11\xA1a\x11\x90a\"nV[a\x11\x98a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[a\x11\xB3\x81a\x06\xF3V[\x03a\x11\xBAWV[_\x80\xFD[\x90P5\x90a\x11\xCB\x82a\x11\xAAV[V[\x90\x91`\xC0\x82\x84\x03\x12a\x12,Wa\x11\xE5\x83_\x84\x01a\x04\x93V[\x92a\x11\xF3\x81` \x85\x01a\x04\xB9V[\x92a\x12\x01\x82`@\x83\x01a\x04\xB9V[\x92a\x12)a\x12\x12\x84``\x85\x01a\x11\xBEV[\x93a\x12 \x81`\x80\x86\x01a\x06\tV[\x93`\xA0\x01a\x06\tV[\x90V[a\x03\x0CV[4a\x12fWa\x12Pa\x12D6`\x04a\x11\xCDV[\x94\x93\x90\x93\x92\x91\x92a\"\xEEV[a\x12Xa\x03\x02V[\x80a\x12b\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[`\xE0\x81\x83\x03\x12a\x12\xD6Wa\x12\x81\x82_\x83\x01a\x04\x93V[\x92a\x12\x8F\x83` \x84\x01a\x04\x93V[\x92a\x12\x9D\x81`@\x85\x01a\x04\xB9V[\x92a\x12\xAB\x82``\x83\x01a\x04\xB9V[\x92a\x12\xD3a\x12\xBC\x84`\x80\x85\x01a\x11\xBEV[\x93a\x12\xCA\x81`\xA0\x86\x01a\x06\tV[\x93`\xC0\x01a\x06\tV[\x90V[a\x03\x0CV[4a\x13\x10Wa\x12\xFAa\x12\xEE6`\x04a\x12kV[\x95\x94\x90\x94\x93\x91\x93a$BV[a\x13\x02a\x03\x02V[\x80a\x13\x0C\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[4a\x13DWa\x13.a\x13(6`\x04a\x06\x8DV[\x90a%`V[a\x136a\x03\x02V[\x80a\x13@\x81a\x06\xBAV[\x03\x90\xF3[a\x03\x08V[\x91\x90`@\x83\x82\x03\x12a\x13qW\x80a\x13ea\x13n\x92_\x86\x01a\x04\x93V[\x93` \x01a\x04\x93V[\x90V[a\x03\x0CV[4a\x13\xA7Wa\x13\xA3a\x13\x92a\x13\x8C6`\x04a\x13IV[\x90a%\x82V[a\x13\x9Aa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xF3[a\x03\x08V[a\x13\xB5\x81a\t\xA6V[\x03a\x13\xBCWV[_\x80\xFD[\x90P5\x90a\x13\xCD\x82a\x13\xACV[V[\x91\x90`@\x83\x82\x03\x12a\x13\xF7W\x80a\x13\xEBa\x13\xF4\x92_\x86\x01a\x04\x93V[\x93` \x01a\x13\xC0V[\x90V[a\x03\x0CV[a\x14\x05\x90a\x0E\xB0V[\x90RV[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x14\x1D\x90a\x14\tV[\x90RV[\x90` \x80a\x14C\x93a\x149_\x82\x01Q_\x86\x01\x90a\x13\xFCV[\x01Q\x91\x01\x90a\x14\x14V[V[\x91\x90a\x14X\x90_`@\x85\x01\x94\x01\x90a\x14!V[V[4a\x14\x8BWa\x14\x87a\x14va\x14p6`\x04a\x13\xCFV[\x90a%\xF0V[a\x14~a\x03\x02V[\x91\x82\x91\x82a\x14EV[\x03\x90\xF3[a\x03\x08V[_\x80\xFD[_\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x14\xB8a\x14\xBE\x91a\x14\tV[\x91a\x14\tV[\x01\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x14\xCFWV[a\x14\x98V[\x90a\x14\xE7\x91a\x14\xE1a\x14\x94V[Pa\x14\xACV[\x90V[a\x14\xF6a\x14\xFC\x91a\x14\tV[\x91a\x14\tV[\x90\x03\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x15\x0EWV[a\x14\x98V[\x90a\x15&\x91a\x15 a\x14\x94V[Pa\x14\xEAV[\x90V[_\x90V[a\x155a\x15)V[P\x80a\x15Pa\x15Jcye\xDB\x0B`\xE0\x1Ba\x03\x10V[\x91a\x03\x10V[\x14\x90\x81\x15a\x15]W[P\x90V[a\x15g\x91Pa&\x06V[_a\x15YV[``\x90V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x01`\x02\x83\x04\x92\x16\x80\x15a\x15\xA6W[` \x83\x10\x14a\x15\xA1WV[a\x15rV[\x91`\x7F\x16\x91a\x15\x96V[` \x91\x81R\x01\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a\x15\xDCa\x15\xD5\x83a\x15\x86V[\x80\x94a\x15\xB0V[\x91`\x01\x81\x16\x90\x81_\x14a\x163WP`\x01\x14a\x15\xF7W[PPPV[a\x16\x04\x91\x92\x93\x94Pa\x15\xB9V[\x91_\x92[\x81\x84\x10a\x16\x1BWPP\x01\x90_\x80\x80a\x15\xF2V[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a\x16\x08V[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a\x15\xF2V[\x90a\x16X\x91a\x15\xC2V[\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x16y\x90a\x03\xE0V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x16\x93W`@RV[a\x16[V[\x90a\x16\xB8a\x16\xB1\x92a\x16\xA8a\x03\x02V[\x93\x84\x80\x92a\x16NV[\x03\x83a\x16oV[V[a\x16\xC3\x90a\x16\x98V[\x90V[a\x16\xCEa\x15mV[Pa\x16\xD9`\x03a\x16\xBAV[\x90V[a\x16\xF9\x91a\x16\xE8a\x15)V[Pa\x16\xF1a&,V[\x91\x90\x91a&9V[`\x01\x90V[_\x90V[_\x1C\x90V[a\x17\x13a\x17\x18\x91a\x17\x02V[a\x10\x9DV[\x90V[a\x17%\x90Ta\x17\x07V[\x90V[a\x170a\x16\xFEV[Pa\x17;`\x02a\x17\x1BV[\x90V[\x91a\x17h\x92a\x17Ka\x15)V[Pa\x17`a\x17Wa&,V[\x82\x90\x84\x91a&\x89V[\x91\x90\x91a'\x15V[`\x01\x90V[_\x90V[a\x17z\x90a\x05\xF2V[\x90V[\x90a\x17\x87\x90a\x17qV[_R` R`@_ \x90V[\x90V[a\x17\xA2a\x17\xA7\x91a\x17\x02V[a\x17\x93V[\x90V[a\x17\xB4\x90Ta\x17\x96V[\x90V[`\x01a\x17\xD0a\x17\xD6\x92a\x17\xC8a\x17mV[P`\x05a\x17}V[\x01a\x17\xAAV[\x90V[\x90a\x17\xF4\x91a\x17\xEFa\x17\xEA\x82a\x17\xB7V[a'\xB2V[a\x17\xF6V[V[\x90a\x18\0\x91a(\x0BV[PV[\x90a\x18\r\x91a\x17\xD9V[V[_\x90V[\x90V[a\x18*a\x18%a\x18/\x92a\x18\x13V[a\x08\x90V[a\x06\xF3V[\x90V[a\x18:a\x18\x0FV[Pa\x18E`\x12a\x18\x16V[\x90V[a\x18Pa\x17mV[Pa\x18Ya(\xB7V[\x90V[\x90\x80a\x18wa\x18qa\x18la&,V[a\x04sV[\x91a\x04sV[\x03a\x18\x88Wa\x18\x85\x91a)qV[PV[_c3K\xD9\x19`\xE1\x1B\x81R\x80a\x18\xA0`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[a\x18\xB8a\x18\xB3a\x18\xBD\x92a\x04hV[a\x08\x90V[a\x04hV[\x90V[a\x18\xC9\x90a\x18\xA4V[\x90V[a\x18\xD5\x90a\x18\xC0V[\x90V[\x90a\x18\xE2\x90a\x18\xCCV[_R` R`@_ \x90V[\x90V[a\x19\x05a\x19\0a\x19\n\x92a\x14\tV[a\x08\x90V[a\x04\xA2V[\x90V[a\x19D\x91a\x199a\x193a\x19.a\x19?\x94a\x19&a\x16\xFEV[P`\na\x18\xD8V[a\x18\xEEV[\x91a*RV[\x90a+gV[a\x18\xF1V[\x90V[\x90a\x19a\x91a\x19\\a\x19Wa\rjV[a'\xB2V[a\x19\xCCV[V[a\x19wa\x19ra\x19|\x92a\x0F\xF3V[a\x08\x90V[a\x04hV[\x90V[a\x19\x88\x90a\x19cV[\x90V[a\x19\x9Fa\x19\x9Aa\x19\xA4\x92a\x0F\xF3V[a\x08\x90V[a\x04\xA2V[\x90V[a\x19\xB6a\x19\xBC\x91\x93\x92\x93a\x04\xA2V[\x92a\x04\xA2V[\x82\x01\x80\x92\x11a\x19\xC7WV[a\x14\x98V[\x90\x81a\x19\xE8a\x19\xE2a\x19\xDD_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14a\x1AxW\x80a\x1A\0a\x19\xFA_a\x19\x8BV[\x91a\x04\xA2V[\x14a\x1A\\Wa\x1A\x17a\x1A\x10a\x17(V[\x82\x90a\x19\xA7V[a\x1A0a\x1A*a\x1A%a\x0E\"V[a\x04\xA2V[\x91a\x04\xA2V[\x11a\x1A@Wa\x1A>\x91a,\x8EV[V[_c\x17~?\xC3`\xE0\x1B\x81R\x80a\x1AX`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1At`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x1A\x90`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x90a\x1A\x9E\x91a\x19GV[V[a\x1A\xAFa\x1A\xB5\x91\x93\x92\x93a\x04\xA2V[\x92a\x04\xA2V[\x82\x03\x91\x82\x11a\x1A\xC0WV[a\x14\x98V[a\x1A\xCDa\x16\xFEV[Pa\x1A\xE7a\x1A\xD9a\x0E\"V[a\x1A\xE1a\x17(V[\x90a\x1A\xA0V[\x90V[\x90a\x1A\xFDa\x1A\xF6a\x03\x02V[\x92\x83a\x16oV[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1B\x1DWa\x1B\x19` \x91a\x03\xE0V[\x01\x90V[a\x16[V[\x90a\x1B4a\x1B/\x83a\x1A\xFFV[a\x1A\xEAV[\x91\x82RV[_\x7Fmode=blocknumber&from=default\0\0\0\x91\x01RV[a\x1Bj`\x1Da\x1B\"V[\x90a\x1Bw` \x83\x01a\x1B9V[V[a\x1B\x81a\x1B`V[\x90V[a\x1B\x8Ca\x15mV[Pa\x1B\x95a!\xC8V[a\x1B\xAEa\x1B\xA8a\x1B\xA3a,\xECV[a\x0E\xB0V[\x91a\x0E\xB0V[\x03a\x1B\xBEWa\x1B\xBBa\x1ByV[\x90V[_c\x01\xBF\xC1\xC5`\xE6\x1B\x81R\x80a\x1B\xD6`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_\x90V[\x90a\x1B\xE8\x90a\x18\xCCV[_R` R`@_ \x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x1C\x0Ba\x1C\x10\x91a\x17\x02V[a\x1B\xF4V[\x90V[a\x1C\x1D\x90Ta\x1B\xFFV[\x90V[a\x1C7a\x1C<\x91a\x1C/a\x1B\xDAV[P`\ta\x1B\xDEV[a\x1C\x13V[\x90V[a\x1CP\x90a\x1CKa&,V[a-?V[V[_\x90V[a\x1Ch\x90a\x1Cba\x1CRV[Pa-\xCAV[\x90V[\x90a\x1Cu\x90a\x18\xCCV[_R` R`@_ \x90V[a\x1C\x97a\x1C\x9C\x91a\x1C\x90a\x16\xFEV[P_a\x1CkV[a\x17\x1BV[\x90V[\x90a\x1C\xB9\x91a\x1C\xB4a\x1C\xAFa\r\x06V[a'\xB2V[a\x1C\xBBV[V[\x80a\x1C\xD6a\x1C\xD0a\x1C\xCB_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14a\x1D\x9AW\x81a\x1C\xEEa\x1C\xE8_a\x19\x8BV[\x91a\x04\xA2V[\x14a\x1D~Wa\x1D\x04a\x1C\xFEa\x1EOV[\x15a\x03]V[a\x1DbWa\x1D\x13\x81\x83\x90a-\xF9V[3\x90a\x1D]a\x1DKa\x1DE\x7F\xBE\xF4\xF8\x1C\x18\x14\xC6A\xED\xE8^\xBA\xAC\xF1\x9D\x04\x8B,[U\x98\n\xDF\xA6\xEF\x0F\x95le\x135\xA2\x93a\x18\xCCV[\x93a\x18\xCCV[\x93a\x1DTa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xA3V[_c\xB8\xB5\xCA-`\xE0\x1B\x81R\x80a\x1Dz`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1D\x96`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x1D\xB2`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x90a\x1D\xC0\x91a\x1C\x9FV[V[a\x1D\xCAa\x16\xFEV[Pa\x1D\xD5`\x0Ca\x17\x1BV[a\x1D\xE7a\x1D\xE1_a\x19\x8BV[\x91a\x04\xA2V[\x14\x80\x15a\x1E\x16W[a\x1E\nWa\x1E\x07a\x1E\0`\x0Ca\x17\x1BV[B\x90a\x1A\xA0V[\x90V[a\x1E\x13_a\x19\x8BV[\x90V[PBa\x1E3a\x1E-a\x1E(`\x0Ca\x17\x1BV[a\x04\xA2V[\x91a\x04\xA2V[\x10\x15a\x1D\xEFV[a\x1EL\x90a\x1EFa\x16\xFEV[Pa.XV[\x90V[a\x1EWa\x15)V[Pa\x1Eb`\x0Ca\x17\x1BV[a\x1Eta\x1En_a\x19\x8BV[\x91a\x04\xA2V[\x14\x15\x80a\x1E\x7FW[\x90V[PBa\x1E\x9Ca\x1E\x96a\x1E\x91`\x0Ca\x17\x1BV[a\x04\xA2V[\x91a\x04\xA2V[\x10a\x1E|V[a\x1E\xBB\x90a\x1E\xB6a\x1E\xB1a\x10\x17V[a'\xB2V[a\x1F5V[V[\x90a\x1E\xC9_\x19\x91a\x0F\xF6V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x1E\xE7a\x1E\xE2a\x1E\xEC\x92a\x04\xA2V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[\x90a\x1F\x07a\x1F\x02a\x1F\x0E\x92a\x1E\xD3V[a\x1E\xEFV[\x82Ta\x1E\xBDV[\x90UV[\x91` a\x1F3\x92\x94\x93a\x1F,`@\x82\x01\x96_\x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[\x80a\x1FHa\x1FBBa\x04\xA2V[\x91a\x04\xA2V[\x11\x15a\x1F\xFEW\x80a\x1F\x81a\x1F{\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xA2V[\x91a\x04\xA2V[\x11a\x1F\xE2Wa\x1F\x90`\x0Ca\x17\x1BV[a\x1F\x9B\x82`\x0Ca\x1E\xF2V[\x903\x90a\x1F\xC8\x7F\xDDh\x96\xDC\xF1\xD4\xB3\x11\xCC\xA8}\xD1\x9B\xBB\xA2\xEA\x9C\xE2\xF8g\xC1V\x88x\xA0C\x8Af\xA1\xAF\xEE\xEC\x92a\x18\xCCV[\x92a\x1F\xDDa\x1F\xD4a\x03\x02V[\x92\x83\x92\x83a\x1F\x12V[\x03\x90\xA2V[_c\xEFi\xAFe`\xE0\x1B\x81R\x80a\x1F\xFA`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[_c\xA5e\x83S`\xE0\x1B\x81R\x80a \x16`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[a #\x90a\x1E\xA2V[V[_\x90V[``\x90V[a 7\x90a\x18\xC0V[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a RW` \x80\x91\x02\x01\x90V[a\x16[V[\x90a ia d\x83a :V[a\x1A\xEAV[\x91\x82RV[6\x907V[\x90a \x98a \x80\x83a WV[\x92` \x80a \x8E\x86\x93a :V[\x92\x01\x91\x03\x90a nV[V[`\x0F`\xF8\x1B\x90V[a \xAAa %V[Pa \xB3a\x15mV[Pa \xBCa\x15mV[Pa \xC5a\x16\xFEV[Pa \xCEa\x1B\xDAV[Pa \xD7a\x17mV[Pa \xE0a )V[Pa \xE9a.pV[\x90a \xF2a.\xB0V[\x90F\x90a \xFE0a .V[\x90a!\x08_a\x0F\xFBV[\x90a!\x1Aa!\x15_a\x19\x8BV[a sV[\x90a!#a \x9AV[\x96\x95\x94\x93\x92\x91\x90V[a!Ua!Z\x91a!;a\x16\xFEV[Pa!Oa!I`\x0Ba\x18\xEEV[\x91a*RV[\x90a+gV[a\x18\xF1V[\x90V[\x90a!g\x90a\x18\xCCV[_R` R`@_ \x90V[`\xFF\x16\x90V[a!\x85a!\x8A\x91a\x17\x02V[a!sV[\x90V[a!\x97\x90Ta!yV[\x90V[a!\xC1\x91_a!\xB6a!\xBC\x93a!\xAEa\x15)V[P`\x05a\x17}V[\x01a!]V[a!\x8DV[\x90V[_\x90V[a!\xD0a!\xC4V[Pa!\xD9a,\xECV[\x90V[a!\xE4a\x15mV[Pa!\xEF`\x04a\x16\xBAV[\x90V[a\"\x19a\"\x14a\"\x0Fa\"\x1E\x93a\"\x07a\x16\xFEV[P`\na\x18\xD8V[a\x18\xEEV[a.\xF0V[a\x18\xF1V[\x90V[a\">\x91a\"-a\x15)V[Pa\"6a&,V[\x91\x90\x91a'\x15V[`\x01\x90V[\x90a\"V\x91a\"Pa\x16\xFEV[Pa\x19\rV[\x90V[a\"k\x90a\"ea\x16\xFEV[Pa!\xF2V[\x90V[a\"va\x16\xFEV[Pa\"\x7Fa\x17(V[\x90V[\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF\x90V[a\"\xDBa\"\xE2\x94a\"\xD1``\x94\x98\x97\x95a\"\xC7`\x80\x86\x01\x9A_\x87\x01\x90a\x066V[` \x85\x01\x90a\t\x1CV[`@\x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[` \x01\x90V[Q\x90V[\x93\x95\x94\x90\x92\x91\x95Ba#\x08a#\x02\x89a\x04\xA2V[\x91a\x04\xA2V[\x11a#\x81W\x91a#s\x91a#z\x93a#ja#\x7F\x98\x99a#Ra#)a\"\x82V[a#C\x8B\x93\x8Ba#7a\x03\x02V[\x95\x86\x94` \x86\x01a\"\xA6V[` \x82\x01\x81\x03\x82R\x03\x82a\x16oV[a#da#^\x82a\"\xEAV[\x91a\"\xE4V[ a/eV[\x92\x90\x91\x92a/\x82V[\x91\x82a/\xCCV[a-?V[V[a#\x9C\x87_\x91\x82\x91c#A\xD7\x87`\xE1\x1B\x83R`\x04\x83\x01a\x058V[\x03\x90\xFD[\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x90V[\x91\x94a$\x0Ca$\x16\x92\x98\x97\x95a$\x02`\xA0\x96a#\xF8a$\x1D\x9Aa#\xEE`\xC0\x8A\x01\x9E_\x8B\x01\x90a\x066V[` \x89\x01\x90a\t\x1CV[`@\x87\x01\x90a\t\x1CV[``\x85\x01\x90a\x05+V[`\x80\x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[\x91` a$@\x92\x94\x93a$9`@\x82\x01\x96_\x83\x01\x90a\t\x1CV[\x01\x90a\t\x1CV[V[\x96\x95\x91\x93\x92\x94\x90\x94Ba$]a$W\x83a\x04\xA2V[\x91a\x04\xA2V[\x11a%\x17W\x90a$\xC6a$\xCF\x94\x93\x92a$\xAEa$wa#\xA0V[a$\x9F\x8C\x80\x94\x8C\x91a$\x89\x8D\x91a0\x1EV[\x91\x92a$\x93a\x03\x02V[\x97\x88\x96` \x88\x01a#\xC4V[` \x82\x01\x81\x03\x82R\x03\x82a\x16oV[a$\xC0a$\xBA\x82a\"\xEAV[\x91a\"\xE4V[ a/eV[\x92\x90\x91\x92a/\x82V[\x80a$\xE2a$\xDC\x87a\x04sV[\x91a\x04sV[\x03a$\xF7WPa$\xF5\x92\x93\x91\x90\x91a&9V[V[\x84\x90a%\x13_\x92\x83\x92c%\xC0\x07#`\xE1\x1B\x84R`\x04\x84\x01a$\x1FV[\x03\x90\xFD[a%2\x90_\x91\x82\x91c1<\x89\x81`\xE1\x1B\x83R`\x04\x83\x01a\x058V[\x03\x90\xFD[\x90a%Q\x91a%La%G\x82a\x17\xB7V[a'\xB2V[a%SV[V[\x90a%]\x91a)qV[PV[\x90a%j\x91a%6V[V[\x90a%v\x90a\x18\xCCV[_R` R`@_ \x90V[a%\xA7\x91a%\x9Da%\xA2\x92a%\x95a\x16\xFEV[P`\x01a%lV[a\x1CkV[a\x17\x1BV[\x90V[a%\xB4`@a\x1A\xEAV[\x90V[_\x90V[_\x90V[a%\xC7a%\xAAV[\x90` \x80\x83a%\xD4a%\xB7V[\x81R\x01a%\xDFa%\xBBV[\x81RPPV[a%\xEDa%\xBFV[\x90V[\x90a&\x03\x91a%\xFDa%\xE5V[Pa0QV[\x90V[a&\x0Ea\x15)V[Pa&(a&\"c\x01\xFF\xC9\xA7`\xE0\x1Ba\x03\x10V[\x91a\x03\x10V[\x14\x90V[a&4a\x1B\xDAV[P3\x90V[\x91a&G\x92\x91`\x01\x92a0yV[V[`@\x90a&ra&y\x94\x96\x95\x93\x96a&h``\x84\x01\x98_\x85\x01\x90a\t\x1CV[` \x83\x01\x90a\x05+V[\x01\x90a\x05+V[V[\x90a&\x86\x91\x03a\x04\xA2V[\x90V[\x92\x91\x92a&\x97\x81\x83\x90a%\x82V[\x90\x81a&\xACa&\xA6_\x19a\x04\xA2V[\x91a\x04\xA2V[\x10a&\xB9W[PPP\x90PV[\x81a&\xCCa&\xC6\x87a\x04\xA2V[\x91a\x04\xA2V[\x10a&\xF2Wa&\xE9\x93\x94a&\xE1\x91\x93\x92a&{V[\x90_\x92a0yV[\x80_\x80\x80a&\xB2V[Pa'\x11\x84\x92\x91\x92_\x93\x84\x93c}\xC7\xA0\xD9`\xE1\x1B\x85R`\x04\x85\x01a&IV[\x03\x90\xFD[\x91\x82a'1a'+a'&_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14a'\x8BW\x81a'Qa'Ka'F_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14a'dWa'b\x92\x91\x90\x91a1\x88V[V[a'\x87a'p_a\x19\x7FV[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\t)V[\x03\x90\xFD[a'\xAEa'\x97_a\x19\x7FV[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\t)V[\x03\x90\xFD[a'\xC4\x90a'\xBEa&,V[\x90a2UV[V[\x90a'\xD2`\xFF\x91a\x0F\xF6V[\x91\x81\x19\x16\x91\x16\x17\x90V[a'\xE5\x90a\x03]V[\x90V[\x90V[\x90a(\0a'\xFBa(\x07\x92a'\xDCV[a'\xE8V[\x82Ta'\xC6V[\x90UV[a(\x13a\x15)V[Pa((a(\"\x82\x84\x90a!\x9AV[\x15a\x03]V[_\x14a(\xB1Wa(P`\x01a(K_a(C`\x05\x86\x90a\x17}V[\x01\x85\x90a!]V[a'\xEBV[\x90a(Ya&,V[\x90a(\x96a(\x90a(\x8A\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x95a\x17qV[\x92a\x18\xCCV[\x92a\x18\xCCV[\x92a(\x9Fa\x03\x02V[\x80a(\xA9\x81a\x06\xBAV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a(\xBFa\x17mV[Pa(\xC90a .V[a(\xFBa(\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04sV[\x91a\x04sV[\x14\x80a)7W[_\x14a),W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a)4a3\x01V[\x90V[PFa)ka)e\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xA2V[\x91a\x04\xA2V[\x14a)\x02V[a)ya\x15)V[Pa)\x85\x81\x83\x90a!\x9AV[_\x14a*\rWa)\xAC_a)\xA7_a)\x9F`\x05\x86\x90a\x17}V[\x01\x85\x90a!]V[a'\xEBV[\x90a)\xB5a&,V[\x90a)\xF2a)\xECa)\xE6\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x95a\x17qV[\x92a\x18\xCCV[\x92a\x18\xCCV[\x92a)\xFBa\x03\x02V[\x80a*\x05\x81a\x06\xBAV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a*'a*\"a*,\x92a\x0E\xB0V[a\x08\x90V[a\x04\xA2V[\x90V[\x91` a*P\x92\x94\x93a*I`@\x82\x01\x96_\x83\x01\x90a\x05+V[\x01\x90a\x0E\xBBV[V[a*Za!\xC4V[Pa*ca!\xC8V[\x81a*va*p\x83a*\x13V[\x91a\x04\xA2V[\x10\x15a*\x89WPa*\x86\x90a4\nV[\x90V[\x90a*\xA4_\x92\x83\x92cvi\xFC\x0F`\xE1\x1B\x84R`\x04\x84\x01a*/V[\x03\x90\xFD[T\x90V[\x90V[a*\xC3a*\xBEa*\xC8\x92a*\xACV[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a*\xE5a*\xEA\x91a\x17\x02V[a*\xCEV[\x90V[a*\xF7\x90Ta*\xD9V[\x90V[\x90V[a+\x11a+\x0Ca+\x16\x92a*\xFAV[a\x08\x90V[a\x04\xA2V[\x90V[`0\x1C\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a+6a+;\x91a+\x19V[a+\x1FV[\x90V[a+H\x90Ta+*V[\x90V[a+_a+Za+d\x92a\x0F\xF3V[a\x08\x90V[a\x14\tV[\x90V[\x90a+\xBB\x90a+ta\x14\x94V[Pa+\x80_\x84\x01a*\xA8V[a+\x89_a\x19\x8BV[\x90\x80\x80a+\x9Fa+\x99`\x05a*\xAFV[\x91a\x04\xA2V[\x11a,\x1CW[P\x90a+\xB6_\x86\x01\x93\x91\x92\x93a*\xCBV[a:]V[\x80a+\xCEa+\xC8_a\x19\x8BV[\x91a\x04\xA2V[\x14_\x14a+\xE4WPPa+\xE0_a+KV[[\x90V[a,\x11_\x91a,\x0Ca,\x06\x84a,\x17\x96\x01\x92a,\0`\x01a*\xFDV[\x90a\x1A\xA0V[\x91a*\xCBV[a:SV[\x01a+>V[a+\xE1V[\x80a,*a,0\x92\x91a6\xE8V[\x90a\x1A\xA0V[\x90\x83a,ba,\\a,W_a,Q\x81\x8C\x01a,L\x89\x91a*\xCBV[a:SV[\x01a*\xEDV[a\x0E\xB0V[\x91a\x0E\xB0V[\x10_\x14a,sWP\x90[\x90_a+\xA5V[\x91Pa,\x89\x90a,\x83`\x01a*\xFDV[\x90a\x19\xA7V[a,lV[\x80a,\xA9a,\xA3a,\x9E_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14a,\xC5Wa,\xC3\x91a,\xBB_a\x19\x7FV[\x91\x90\x91a1\x88V[V[a,\xE8a,\xD1_a\x19\x7FV[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\t)V[\x03\x90\xFD[a,\xF4a!\xC4V[Pa,\xFECa4\nV[\x90V[\x90a-\x12`\x01\x80`\xA0\x1B\x03\x91a\x0F\xF6V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a-4a-/a-;\x92a\x18\xCCV[a-\x1CV[\x82Ta-\x01V[\x90UV[\x90a-\xC8\x91a-\xC2a-P\x82a\x1C V[a-e\x84a-``\t\x86\x90a\x1B\xDEV[a-\x1FV[\x82\x81\x85\x90a-\xA5a-\x9Fa-\x99\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x95a\x18\xCCV[\x92a\x18\xCCV[\x92a\x18\xCCV[\x92a-\xAEa\x03\x02V[\x80a-\xB8\x81a\x06\xBAV[\x03\x90\xA4\x92\x91a:\xECV[\x91a;\x04V[V[a-\xF1a-\xECa-\xE7a-\xF6\x93a-\xDFa\x1CRV[P`\na\x18\xD8V[a\x18\xEEV[a<\xB2V[a=1V[\x90V[\x90\x81a.\x15a.\x0Fa.\n_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14a.1Wa./\x91\x90a.(_a\x19\x7FV[\x90\x91a1\x88V[V[a.Ta.=_a\x19\x7FV[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\t)V[\x03\x90\xFD[a.j\x90a.da\x16\xFEV[Pa=\x82V[\x90V[\x90V[a.xa\x15mV[Pa.\xAD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a.\xA7`\x06a.mV[\x90a>\x9DV[\x90V[a.\xB8a\x15mV[Pa.\xED\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a.\xE7`\x07a.mV[\x90a>\x9DV[\x90V[a.\xF8a\x14\x94V[Pa/\x04_\x82\x01a*\xA8V[\x80a/\x17a/\x11_a\x19\x8BV[\x91a\x04\xA2V[\x14_\x14a/-WPPa/)_a+KV[[\x90V[a/Z_\x91a/Ua/O\x84a/`\x96\x01\x92a/I`\x01a*\xFDV[\x90a\x1A\xA0V[\x91a*\xCBV[a:SV[\x01a+>V[a/*V[a/\x7F\x90a/qa\x17mV[Pa/za(\xB7V[a>\xEBV[\x90V[\x92a/\x9D\x92a/\xA6\x94a/\x93a\x1B\xDAV[P\x92\x90\x91\x92a?\xB1V[\x90\x92\x91\x92a@\xDCV[\x90V[\x91` a/\xCA\x92\x94\x93a/\xC3`@\x82\x01\x96_\x83\x01\x90a\t\x1CV[\x01\x90a\x05+V[V[a/\xD5\x81a0\x1EV[\x91a/\xE8a/\xE2\x84a\x04\xA2V[\x91a\x04\xA2V[\x03a/\xF1WPPV[a0\x0B_\x92\x83\x92c\x01\xD4\xB6#`\xE6\x1B\x84R`\x04\x84\x01a/\xA9V[\x03\x90\xFD[`\x01a0\x1B\x91\x01a\x04\xA2V[\x90V[a02\x90a0*a\x16\xFEV[P`\x08a\x1CkV[a0Na0>\x82a\x17\x1BV[\x91a0H\x83a0\x0FV[\x90a\x1E\xF2V[\x90V[\x90a0qa0la0v\x93a0da%\xE5V[P`\na\x18\xD8V[a\x18\xEEV[aBRV[\x90V[\x90\x92\x81a0\x96a0\x90a0\x8B_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14a1aW\x83a0\xB6a0\xB0a0\xAB_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14a1:Wa0\xDA\x83a0\xD5a0\xCE`\x01\x86\x90a%lV[\x87\x90a\x1CkV[a\x1E\xF2V[a0\xE4W[PPPV[\x91\x90\x91a1/a1\x1Da1\x17\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x93a\x18\xCCV[\x93a\x18\xCCV[\x93a1&a\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xA3_\x80\x80a0\xDFV[a1]a1F_a\x19\x7FV[_\x91\x82\x91cJ\x14\x06\xB1`\xE1\x1B\x83R`\x04\x83\x01a\t)V[\x03\x90\xFD[a1\x84a1m_a\x19\x7FV[_\x91\x82\x91c\xE6\x02\xDF\x05`\xE0\x1B\x83R`\x04\x83\x01a\t)V[\x03\x90\xFD[\x91\x82a1\xA4a1\x9Ea1\x99_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14\x15\x80a2\x0FW[a1\xBFW[a1\xBD\x92\x91\x90\x91aBsV[V[a1\xC7a\x1EOV[\x80a1\xEEW[\x15a1\xB1W_c6\xE2x\xFD`\xE2\x1B\x81R\x80a1\xEA`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[Pa2\na2\x04a1\xFDa\r\x06V[3\x90a!\x9AV[\x15a\x03]V[a1\xCDV[P\x81a2+a2%a2 _a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14\x15a1\xACV[\x91` a2S\x92\x94\x93a2L`@\x82\x01\x96_\x83\x01\x90a\t\x1CV[\x01\x90a\x066V[V[\x90a2ja2d\x83\x83\x90a!\x9AV[\x15a\x03]V[a2rWPPV[a2\x8C_\x92\x83\x92c\xE2Q}?`\xE0\x1B\x84R`\x04\x84\x01a22V[\x03\x90\xFD[\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x90V[\x90\x95\x94\x92a2\xFF\x94a2\xEEa2\xF8\x92a2\xE4`\x80\x96a2\xDA`\xA0\x88\x01\x9C_\x89\x01\x90a\x066V[` \x87\x01\x90a\x066V[`@\x85\x01\x90a\x066V[``\x83\x01\x90a\x05+V[\x01\x90a\t\x1CV[V[a3\ta\x17mV[Pa3\x12a2\x90V[a3\x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a3z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Fa3e0a .V[\x91a3na\x03\x02V[\x96\x87\x95` \x87\x01a2\xB4V[` \x82\x01\x81\x03\x82R\x03\x82a\x16oV[a3\x9Ba3\x95\x82a\"\xEAV[\x91a\"\xE4V[ \x90V[\x90V[a3\xB6a3\xB1a3\xBB\x92a3\x9FV[a\x08\x90V[a\x06\xF3V[\x90V[a3\xC7\x90a3\xA2V[\x90RV[\x91` a3\xEC\x92\x94\x93a3\xE5`@\x82\x01\x96_\x83\x01\x90a3\xBEV[\x01\x90a\x05+V[V[a4\x02a3\xFDa4\x07\x92a\x04\xA2V[a\x08\x90V[a\x0E\xB0V[\x90V[a4\x12a!\xC4V[P\x80a4,a4&e\xFF\xFF\xFF\xFF\xFF\xFFa*\x13V[\x91a\x04\xA2V[\x11a4=Wa4:\x90a3\xEEV[\x90V[`0a4Y_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a3\xCBV[\x03\x90\xFD[\x90V[a4ta4oa4y\x92a4]V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a4\x93a4\x8Ea4\x98\x92a4|V[a\x08\x90V[a\x06\xF3V[\x90V[a4\xBA\x90a4\xB4a4\xAEa4\xBF\x94a\x06\xF3V[\x91a\x04\xA2V[\x90a\x10\x99V[a\x04\xA2V[\x90V[\x90V[a4\xD9a4\xD4a4\xDE\x92a4\xC2V[a\x08\x90V[a\x06\xF3V[\x90V[\x1B\x90V[a5\x04\x90a4\xFEa4\xF8a5\t\x94a\x06\xF3V[\x91a\x04\xA2V[\x90a4\xE1V[a\x04\xA2V[\x90V[\x90V[a5#a5\x1Ea5(\x92a5\x0CV[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a5Ba5=a5G\x92a5+V[a\x08\x90V[a\x06\xF3V[\x90V[\x90V[a5aa5\\a5f\x92a5JV[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a5\x80a5{a5\x85\x92a5iV[a\x08\x90V[a\x06\xF3V[\x90V[\x90V[a5\x9Fa5\x9Aa5\xA4\x92a5\x88V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a5\xBEa5\xB9a5\xC3\x92a5\xA7V[a\x08\x90V[a\x06\xF3V[\x90V[\x90V[a5\xDDa5\xD8a5\xE2\x92a5\xC6V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a5\xFCa5\xF7a6\x01\x92a5\xE5V[a\x08\x90V[a\x06\xF3V[\x90V[a6\x18a6\x13a6\x1D\x92a5iV[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[a67a62a6<\x92a6 V[a\x08\x90V[a\x06\xF3V[\x90V[a6Sa6Na6X\x92a5\xE5V[a\x08\x90V[a\x04\xA2V[\x90V[a6oa6ja6t\x92a*\xFAV[a\x08\x90V[a\x06\xF3V[\x90V[\x90V[a6\x8Ea6\x89a6\x93\x92a6wV[a\x08\x90V[a\x04\xA2V[\x90V[\x90a6\xA1\x91\x02a\x04\xA2V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a6\xC4a6\xCA\x91a\x04\xA2V[\x91a\x04\xA2V[\x90\x81\x15a6\xD5W\x04\x90V[a6\xA4V[\x90a6\xE5\x91\x01a\x04\xA2V[\x90V[a6\xF0a\x16\xFEV[P\x80a7\x05a6\xFF`\x01a*\xFDV[\x91a\x04\xA2V[\x11\x15a:PW\x80a9\x1Aa8\xF7a8\xE7a8\xD7a8\xC7a8\xB7a8\xA7a8\x97a8\x87a8wa8g\x8Ba8aa8Za9 \x9Fa8:a8*a8J\x92a7L`\x01a*\xFDV[\x90\x80a7da7^`\x01`\x80\x1Ba4`V[\x91a\x04\xA2V[\x10\x15a:\"W[\x80a7\x87a7\x81h\x01\0\0\0\0\0\0\0\0a5\x0FV[\x91a\x04\xA2V[\x10\x15a9\xF4W[\x80a7\xA6a7\xA0d\x01\0\0\0\0a5MV[\x91a\x04\xA2V[\x10\x15a9\xC6W[\x80a7\xC3a7\xBDb\x01\0\0a5\x8BV[\x91a\x04\xA2V[\x10\x15a9\x98W[\x80a7\xDFa7\xD9a\x01\0a5\xC9V[\x91a\x04\xA2V[\x10\x15a9jW[\x80a7\xFAa7\xF4`\x10a6\x04V[\x91a\x04\xA2V[\x10\x15a9<W[a8\x14a8\x0E`\x04a6?V[\x91a\x04\xA2V[\x10\x15a9#W[a8%`\x03a6zV[a6\x96V[a84`\x01a6[V[\x90a4\x9BV[a8D\x81\x86a6\xB8V[\x90a6\xDAV[a8T`\x01a6[V[\x90a4\x9BV[\x80\x92a6\xB8V[\x90a6\xDAV[a8q`\x01a6[V[\x90a4\x9BV[a8\x81\x81\x8Ca6\xB8V[\x90a6\xDAV[a8\x91`\x01a6[V[\x90a4\x9BV[a8\xA1\x81\x8Aa6\xB8V[\x90a6\xDAV[a8\xB1`\x01a6[V[\x90a4\x9BV[a8\xC1\x81\x88a6\xB8V[\x90a6\xDAV[a8\xD1`\x01a6[V[\x90a4\x9BV[a8\xE1\x81\x86a6\xB8V[\x90a6\xDAV[a8\xF1`\x01a6[V[\x90a4\x9BV[\x91a9\x14a9\x0Ea9\t\x85\x80\x94a6\xB8V[a\x04\xA2V[\x91a\x04\xA2V[\x11aC\x03V[\x90a&{V[\x90V[a97\x90a91`\x01a6[V[\x90a4\xE5V[a8\x1BV[a9Sa9d\x91a9M`\x04a5\xE8V[\x90a4\x9BV[\x91a9^`\x02a6#V[\x90a4\xE5V[\x90a8\x01V[a9\x81a9\x92\x91a9{`\x08a5\xAAV[\x90a4\x9BV[\x91a9\x8C`\x04a5\xE8V[\x90a4\xE5V[\x90a7\xE6V[a9\xAFa9\xC0\x91a9\xA9`\x10a5lV[\x90a4\x9BV[\x91a9\xBA`\x08a5\xAAV[\x90a4\xE5V[\x90a7\xCAV[a9\xDDa9\xEE\x91a9\xD7` a5.V[\x90a4\x9BV[\x91a9\xE8`\x10a5lV[\x90a4\xE5V[\x90a7\xADV[a:\x0Ba:\x1C\x91a:\x05`@a4\xC5V[\x90a4\x9BV[\x91a:\x16` a5.V[\x90a4\xE5V[\x90a7\x8EV[a:9a:J\x91a:3`\x80a4\x7FV[\x90a4\x9BV[\x91a:D`@a4\xC5V[\x90a4\xE5V[\x90a7kV[\x90V[_R` _ \x01\x90V[\x93\x91\x90\x92a:ia\x16\xFEV[P[\x81a:~a:x\x83a\x04\xA2V[\x91a\x04\xA2V[\x10\x15a:\xE4Wa:\x8F\x82\x82\x90aCOV[\x90a:\xA5_a:\x9F\x88\x85\x90a:SV[\x01a*\xEDV[a:\xB7a:\xB1\x87a\x0E\xB0V[\x91a\x0E\xB0V[\x11_\x14a:\xC7WP\x91[\x91a:kV[\x92\x91Pa:\xDE\x90a:\xD8`\x01a*\xFDV[\x90a\x19\xA7V[\x90a:\xC1V[\x92PP\x91P\x90V[a:\xFE\x90a:\xF8a\x16\xFEV[Pa\x1C\x81V[\x90V[\x90V[\x91\x90\x91\x80a;\x1Aa;\x14\x85a\x04sV[\x91a\x04sV[\x14\x15\x80a<\x98W[a;,W[PPPV[\x80a;Ga;Aa;<_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x03a<\x08W[P\x81a;ia;ca;^_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x03a;uW[\x80a;'V[a;\xBCa;\xAFa;\xB6\x92a;\x8B`\n\x86\x90a\x18\xD8V[\x90a;\xA9a;\xA3a;\x9D`\x01\x93aC\xE8V[\x93a\x18\xEEV[\x91a;\x01V[\x90aD;V[\x92\x90a\x18\xF1V[\x91a\x18\xF1V[\x91\x90\x91a;\xE9\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x18\xCCV[\x92a;\xFEa;\xF5a\x03\x02V[\x92\x83\x92\x83a\x1F\x12V[\x03\x90\xA2_\x80a;oV[a<Ga<Ma<@a<\x1D`\n\x85\x90a\x18\xD8V[`\x02a<:a<4a<.\x89aC\xE8V[\x93a\x18\xEEV[\x91a;\x01V[\x90aD;V[\x92\x90a\x18\xF1V[\x91a\x18\xF1V[\x91\x90\x91a<z\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x18\xCCV[\x92a<\x8Fa<\x86a\x03\x02V[\x92\x83\x92\x83a\x1F\x12V[\x03\x90\xA2_a;MV[P\x81a<\xACa<\xA6_a\x19\x8BV[\x91a\x04\xA2V[\x11a;\"V[_a<\xC6\x91a<\xBFa\x16\xFEV[P\x01a*\xA8V[\x90V[a<\xDDa<\xD8a<\xE2\x92a\t\xA6V[a\x08\x90V[a\x04\xA2V[\x90V[a<\xEE\x90a5.V[\x90RV[\x91` a=\x13\x92\x94\x93a=\x0C`@\x82\x01\x96_\x83\x01\x90a<\xE5V[\x01\x90a\x05+V[V[a=)a=$a=.\x92a\x04\xA2V[a\x08\x90V[a\t\xA6V[\x90V[a=9a\x1CRV[P\x80a=Qa=Kc\xFF\xFF\xFF\xFFa<\xC9V[\x91a\x04\xA2V[\x11a=bWa=_\x90a=\x15V[\x90V[` a=~_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a<\xF2V[\x03\x90\xFD[a=\x99a=\x9E\x91a=\x91a\x16\xFEV[P`\x08a\x1CkV[a\x17\x1BV[\x90V[\x90V[a=\xB8a=\xB3a=\xBD\x92a=\xA1V[a\x0F\xF6V[a\x05\xF2V[\x90V[a=\xCA`\xFFa=\xA4V[\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a=\xF0a=\xE9\x83a\x15\x86V[\x80\x94a\x15\xB0V[\x91`\x01\x81\x16\x90\x81_\x14a>GWP`\x01\x14a>\x0BW[PPPV[a>\x18\x91\x92\x93\x94Pa=\xCDV[\x91_\x92[\x81\x84\x10a>/WPP\x01\x90_\x80\x80a>\x06V[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a>\x1CV[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a>\x06V[\x90a>l\x91a=\xD6V[\x90V[\x90a>\x8Fa>\x88\x92a>\x7Fa\x03\x02V[\x93\x84\x80\x92a>bV[\x03\x83a\x16oV[V[a>\x9A\x90a>oV[\x90V[\x90a>\xA6a\x15mV[Pa>\xB0\x82a\x17qV[a>\xC9a>\xC3a>\xBEa=\xC0V[a\x05\xF2V[\x91a\x05\xF2V[\x14\x15_\x14a>\xDEWPa>\xDB\x90aD\xC5V[\x90V[a>\xE8\x91Pa>\x91V[\x90V[`B\x91a>\xF6a\x17mV[P`@Q\x91\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[_\x90V[a?<a?A\x91a\x17\x02V[a\x1E\xD3V[\x90V[\x90V[a?[a?Va?`\x92a?DV[a\x08\x90V[a\x04\xA2V[\x90V[a?\x98a?\x9F\x94a?\x8E``\x94\x98\x97\x95a?\x84`\x80\x86\x01\x9A_\x87\x01\x90a\x066V[` \x85\x01\x90a\x06\xF9V[`@\x83\x01\x90a\x066V[\x01\x90a\x066V[V[a?\xA9a\x03\x02V[=_\x82>=\x90\xFD[\x93\x92\x93a?\xBCa\x1B\xDAV[Pa?\xC5a?,V[Pa?\xCEa\x17mV[Pa?\xD8\x85a?0V[a@\na@\x04\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0a?GV[\x91a\x04\xA2V[\x11a@\x97W\x90a@-` \x94\x95_\x94\x93\x92\x93a@$a\x03\x02V[\x94\x85\x94\x85a?cV[\x83\x80R\x03\x90`\x01Z\xFA\x15a@\x92Wa@E_Qa\x0F\xF6V[\x80a@`a@Za@U_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14a@vW_\x91a@p_a\x0F\xFBV[\x91\x92\x91\x90V[Pa@\x80_a\x19\x7FV[`\x01\x91a@\x8C_a\x0F\xFBV[\x91\x92\x91\x90V[a?\xA1V[PPPa@\xA3_a\x19\x7FV[\x90`\x03\x92\x91\x92\x91\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x04\x11\x15a@\xCBWV[a@\xADV[\x90a@\xDA\x82a@\xC1V[V[\x80a@\xEFa@\xE9_a@\xD0V[\x91a@\xD0V[\x14_\x14a@\xFAWPPV[\x80aA\x0EaA\x08`\x01a@\xD0V[\x91a@\xD0V[\x14_\x14aA1W_c\xF6E\xEE\xDF`\xE0\x1B\x81R\x80aA-`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[\x80aAEaA?`\x02a@\xD0V[\x91a@\xD0V[\x14_\x14aAsWaAoaAX\x83a?0V[_\x91\x82\x91c\xFC\xE6\x98\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x058V[\x03\x90\xFD[aA\x86aA\x80`\x03a@\xD0V[\x91a@\xD0V[\x14aA\x8EWPV[aA\xA9\x90_\x91\x82\x91c5\xE2\xF3\x83`\xE2\x1B\x83R`\x04\x83\x01a\x06CV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_R` _ \x90V[aA\xD3\x81a*\xA8V[\x82\x10\x15aA\xEDWaA\xE5`\x01\x91aA\xC1V[\x91\x02\x01\x90_\x90V[aA\xADV[\x90aA\xFC\x90a\x0E\xB0V[\x90RV[\x90aB\n\x90a\x14\tV[\x90RV[\x90aBDaB;_aB\x1Ea%\xAAV[\x94aB5aB-\x83\x83\x01a*\xEDV[\x83\x88\x01aA\xF2V[\x01a+>V[` \x84\x01aB\0V[V[aBO\x90aB\x0EV[\x90V[aBp\x91_aBj\x92aBca%\xE5V[P\x01aA\xCAV[PaBFV[\x90V[\x92\x91aB\x81\x84\x83\x83\x91aD\xF5V[\x83aB\x9CaB\x96aB\x91_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14aB\xB1W[aB\xAF\x92\x93\x91\x90\x91aF\x7FV[V[aB\xB9a\x17(V[\x93aB\xC2aFdV[\x94\x80aB\xD6aB\xD0\x88a\x04\xA2V[\x91a\x04\xA2V[\x11aB\xE3WP\x93PaB\xA2V[\x85\x90aB\xFF_\x92\x83\x92c\x0EX\xAE\x93`\xE1\x1B\x84R`\x04\x84\x01a\x1F\x12V[\x03\x90\xFD[aC\x0Ba\x16\xFEV[P\x15\x15\x90V[aC%aC aC*\x92a6 V[a\x08\x90V[a\x04\xA2V[\x90V[aC9aC?\x91a\x04\xA2V[\x91a\x04\xA2V[\x90\x81\x15aCJW\x04\x90V[a6\xA4V[aCtaCz\x92aC^a\x16\xFEV[P\x82\x81\x16\x92\x18aCn`\x02aC\x11V[\x90aC-V[\x90a\x19\xA7V[\x90V[\x90V[aC\x94aC\x8FaC\x99\x92aC}V[a\x08\x90V[a\x06\xF3V[\x90V[aC\xA5\x90aC\x80V[\x90RV[\x91` aC\xCA\x92\x94\x93aC\xC3`@\x82\x01\x96_\x83\x01\x90aC\x9CV[\x01\x90a\x05+V[V[aC\xE0aC\xDBaC\xE5\x92a\x04\xA2V[a\x08\x90V[a\x14\tV[\x90V[aC\xF0a\x14\x94V[P\x80aD\naD\x04`\x01\x80`\xD0\x1B\x03a\x18\xF1V[\x91a\x04\xA2V[\x11aD\x1BWaD\x18\x90aC\xCCV[\x90V[`\xD0aD7_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01aC\xA9V[\x03\x90\xFD[\x90aDqaDw\x93\x92aDLa\x14\x94V[PaDUa\x14\x94V[P\x80\x93aDjaDca!\xC8V[\x94\x92a.\xF0V[\x90\x91aJ\xFAV[\x91aG>V[\x91\x90\x91\x90V[aD\x91aD\x8CaD\x96\x92a5+V[a\x08\x90V[a\x04\xA2V[\x90V[6\x907V[\x90aD\xC3aD\xAB\x83a\x1B\"V[\x92` \x80aD\xB9\x86\x93a\x1A\xFFV[\x92\x01\x91\x03\x90aD\x99V[V[aD\xCDa\x15mV[PaD\xD7\x81aG\xA8V[\x90aD\xEAaD\xE5` aD}V[aD\x9EV[\x91\x82R` \x82\x01R\x90V[\x91\x90\x91\x80aE\x13aE\raE\x08_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14_\x14aE\xF4WaE7aE0\x83aE+`\x02a\x17\x1BV[a\x19\xA7V[`\x02a\x1E\xF2V[[\x82aESaEMaEH_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14_\x14aE\xC8WaEwaEp\x83aEk`\x02a\x17\x1BV[a&{V[`\x02a\x1E\xF2V[[\x91\x90\x91aE\xC3aE\xB1aE\xAB\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93a\x18\xCCV[\x93a\x18\xCCV[\x93aE\xBAa\x03\x02V[\x91\x82\x91\x82a\x058V[\x03\x90\xA3V[aE\xEF\x82aE\xE9aE\xDA_\x87\x90a\x1CkV[\x91aE\xE4\x83a\x17\x1BV[a6\xDAV[\x90a\x1E\xF2V[aExV[aF\x07aF\x02_\x83\x90a\x1CkV[a\x17\x1BV[\x80aF\x1AaF\x14\x85a\x04\xA2V[\x91a\x04\xA2V[\x10aFBWaF-aF=\x91\x84\x90a&{V[aF8_\x84\x90a\x1CkV[a\x1E\xF2V[aE8V[\x90aF`\x90\x91\x92_\x93\x84\x93c9\x144\xE3`\xE2\x1B\x85R`\x04\x85\x01a&IV[\x03\x90\xFD[aFla\x16\xFEV[PaF|`\x01\x80`\xD0\x1B\x03a\x18\xF1V[\x90V[\x91aF\xD7aF\xD1aF\xDE\x94\x80aF\xA5aF\x9FaF\x9A_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14aG\x0FW[\x84aF\xC6aF\xC0aF\xBB_a\x19\x7FV[a\x04sV[\x91a\x04sV[\x14aF\xE0W[a\x1C V[\x92a\x1C V[\x90\x91a;\x04V[V[aG\x08`\x0B`\x02aG\x02aF\xFCaF\xF6\x89aC\xE8V[\x93a\x18\xEEV[\x91a;\x01V[\x90aD;V[PPaF\xCCV[aG7`\x0B`\x01aG1aG+aG%\x89aC\xE8V[\x93a\x18\xEEV[\x91a;\x01V[\x90aD;V[PPaF\xABV[\x91aGb_aGg\x94aGOa\x14\x94V[PaGXa\x14\x94V[P\x01\x92\x91\x92a*\xCBV[aI\xACV[\x91\x90\x91\x90V[aG\x81aG|aG\x86\x92a=\xA1V[a\x08\x90V[a\x04\xA2V[\x90V[\x90V[aG\xA0aG\x9BaG\xA5\x92aG\x89V[a\x08\x90V[a\x04\xA2V[\x90V[aG\xBDaG\xC2\x91aG\xB7a\x16\xFEV[Pa\x17qV[a?0V[aG\xCC`\xFFaGmV[\x16\x80aG\xE1aG\xDB`\x1FaG\x8CV[\x91a\x04\xA2V[\x11aG\xE9W\x90V[_c,\xD4J\xC3`\xE2\x1B\x81R\x80aH\x01`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[T\x90V[aH\x13`@a\x1A\xEAV[\x90V[_R` _ \x90V[aH(\x81aH\x05V[\x82\x10\x15aHBWaH:`\x01\x91aH\x16V[\x91\x02\x01\x90_\x90V[aA\xADV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[aHd\x90Qa\x0E\xB0V[\x90V[\x90aHxe\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x0F\xF6V[\x91\x81\x19\x16\x91\x16\x17\x90V[aH\x96aH\x91aH\x9B\x92a\x0E\xB0V[a\x08\x90V[a\x0E\xB0V[\x90V[\x90V[\x90aH\xB6aH\xB1aH\xBD\x92aH\x82V[aH\x9EV[\x82TaHgV[\x90UV[aH\xCB\x90Qa\x14\tV[\x90V[`0\x1B\x90V[\x90aH\xE6e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91aH\xCEV[\x91\x81\x19\x16\x91\x16\x17\x90V[aI\x04aH\xFFaI\t\x92a\x14\tV[a\x08\x90V[a\x14\tV[\x90V[\x90V[\x90aI$aI\x1FaI+\x92aH\xF0V[aI\x0CV[\x82TaH\xD4V[\x90UV[\x90aIY` _aI_\x94aIQ\x82\x82\x01aIK\x84\x88\x01aHZV[\x90aH\xA1V[\x01\x92\x01aH\xC1V[\x90aI\x0FV[V[\x91\x90aIrWaIp\x91aI/V[V[aHGV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15aI\xA7W\x82aI\x9F\x91`\x01aI\xA5\x95\x01\x81UaH\x1FV[\x90aIaV[V[a\x16[V[\x90\x92\x91\x92aI\xB8a\x14\x94V[PaI\xC1a\x14\x94V[PaI\xCB\x82aH\x05V[\x80aI\xDEaI\xD8_a\x19\x8BV[\x91a\x04\xA2V[\x11_\x14aJ\xAEWaJ\x04\x90aI\xFE\x84\x91aI\xF8`\x01a*\xFDV[\x90a\x1A\xA0V[\x90a:SV[\x90aJ\x10_\x83\x01a*\xEDV[\x92aJ\x1C_\x84\x01a+>V[\x93\x80aJ0aJ*\x85a\x0E\xB0V[\x91a\x0E\xB0V[\x11aJ\x92WaJGaJA\x84a\x0E\xB0V[\x91a\x0E\xB0V[\x14_\x14aJbWPPaJ]\x90_\x85\x91\x01aI\x0FV[[\x91\x90V[aJ\x8D\x92PaJ\x88\x86aJ\x7FaJvaH\tV[\x94_\x86\x01aA\xF2V[` \x84\x01aB\0V[aIwV[aJ^V[_c% `\x1D`\xE0\x1B\x81R\x80aJ\xAA`\x04\x82\x01a\x06\xBAV[\x03\x90\xFD[PaJ\xD9\x91aJ\xD4\x85aJ\xCBaJ\xC2aH\tV[\x94_\x86\x01aA\xF2V[` \x84\x01aB\0V[aIwV[aJ\xE2_a+KV[\x91\x90V[cNH{q`\xE0\x1B_R`Q`\x04R`$_\xFD[\x91\x90\x91\x80`\x01\x14aK\x19W`\x02\x03aJ\xE6WaK\x15\x91a\x15\x13V[\x90[V[PaK#\x91a\x14\xD4V[\x90aK\x17V",
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
