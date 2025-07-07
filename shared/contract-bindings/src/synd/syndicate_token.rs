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
    ///0x610180604052346100845761001b610015610158565b906103bd565b610023610089565b614bac611c8382396080518161298d015260a051816129c4015260c05181612954015260e0518161339a015261010051816133bf01526101205181612f0101526101405181612f41015261016051818181610b710152611fda0152614bac90f35b61008f565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b906100bb90610093565b810190811060018060401b038211176100d357604052565b61009d565b906100eb6100e4610089565b92836100b1565b565b5f80fd5b60018060a01b031690565b610105906100f1565b90565b610111816100fc565b0361011857565b5f80fd5b9050519061012982610108565b565b91906040838203126101535780610147610150925f860161011c565b9360200161011c565b90565b6100ed565b61017661682f8038038061016b816100d8565b92833981019061012b565b9091565b60018060401b03811161019657610192602091610093565b0190565b61009d565b906101ad6101a88361017a565b6100d8565b918252565b5f7f53796e6469636174650000000000000000000000000000000000000000000000910152565b6101e3600961019b565b906101f0602083016101b2565b565b6101fa6101d9565b90565b5f7f53594e4400000000000000000000000000000000000000000000000000000000910152565b61022e600461019b565b9061023b602083016101fd565b565b610245610224565b90565b90565b90565b61026261025d61026792610248565b61024b565b6100f1565b90565b6102739061024e565b90565b5f0190565b90565b90565b61029561029061029a9261027b565b61024b565b61027e565b90565b6102a96276a700610281565b90565b634e487b7160e01b5f52601160045260245ffd5b6102cf6102d59193929361027e565b9261027e565b82018092116102e057565b6102ac565b6102f96102f46102fe92610248565b61024b565b61027e565b90565b5f1b90565b906103125f1991610301565b9181191691161790565b61033061032b6103359261027e565b61024b565b61027e565b90565b90565b9061035061034b6103579261031c565b610338565b8254610306565b9055565b90565b61037261036d61037792610248565b610301565b61035b565b90565b6103835f61035e565b90565b90565b61039d6103986103a292610386565b61024b565b61027e565b90565b6103ba6b02e87669c308736a04000000610389565b90565b906103df6103c96101f2565b6103d16101f2565b6103d961023d565b916104a5565b816103fa6103f46103ef5f61026a565b6100fc565b916100fc565b14610489578061041a61041461040f5f61026a565b6100fc565b916100fc565b1461046d5761045c61046b926104384261043261029d565b906102c0565b6101605261044f6104485f6102e5565b600c61033b565b61045761037a565b610941565b506104656103a5565b90610a0f565b565b5f63d92e233d60e01b81528061048560048201610276565b0390fd5b5f63d92e233d60e01b8152806104a160048201610276565b0390fd5b906104b092916104b2565b565b906104bd92916104bf565b565b906104ca92916104cc565b565b906104d792916104d9565b565b906104e49291610531565b565b5f7f3100000000000000000000000000000000000000000000000000000000000000910152565b610517600161019b565b90610524602083016104e6565b565b61052e61050d565b90565b90610545929161053f610526565b90610547565b565b90610553939291610599565b565b90565b90565b60200190565b5190565b61057961057461057e926100f1565b61024b565b6100f1565b90565b61058a90610565565b90565b61059690610581565b90565b6105aa6105fa946105df939461062e565b6105be816105b86006610555565b90610abc565b610120526105d6836105d06007610555565b90610abc565b61014052610558565b6105f16105eb82610561565b9161055b565b2060e052610558565b61060c61060682610561565b9161055b565b20610100524660a05261061d610bc1565b6080526106293061058d565b60c052565b906106389161063a565b565b9061064491610646565b565b9061065091610897565b565b634e487b7160e01b5f525f60045260245ffd5b5190565b634e487b7160e01b5f52602260045260245ffd5b906001600283049216801561069d575b602083101461069857565b610669565b91607f169161068d565b5f5260205f2090565b601f602091010490565b1b90565b919060086106d99102916106d35f19846106ba565b926106ba565b9181191691161790565b91906106f96106f46107019361031c565b610338565b9083546106be565b9055565b5f90565b61071b91610715610705565b916106e3565b565b5b818110610729575050565b806107365f600193610709565b0161071e565b9190601f811161074c575b505050565b61075861077d936106a7565b906020610764846106b0565b83019310610785575b610776906106b0565b019061071d565b5f8080610747565b91506107768192905061076d565b1c90565b906107a7905f1990600802610793565b191690565b816107b691610797565b906002021790565b906107c881610665565b9060018060401b038211610886576107ea826107e4855461067d565b8561073c565b602090601f831160011461081e5791809161080d935f92610812575b50506107ac565b90555b565b90915001515f80610806565b601f1983169161082d856106a7565b925f5b81811061086e57509160029391856001969410610854575b50505002019055610810565b610864910151601f841690610797565b90555f8080610848565b91936020600181928787015181550195019201610830565b61009d565b90610895916107be565b565b906108a66108ad92600361088b565b600461088b565b565b5f90565b151590565b6108c19061035b565b90565b906108ce906108b8565b5f5260205260405f2090565b6108e390610581565b90565b906108f0906108da565b5f5260205260405f2090565b9061090860ff91610301565b9181191691161790565b61091b906108b3565b90565b90565b9061093661093161093d92610912565b61091e565b82546108fc565b9055565b6109496108af565b5061095e610958828490610c5e565b156108b3565b5f146109e75761098660016109815f610979600586906108c4565b0185906108e6565b610921565b9061098f610c8c565b906109cc6109c66109c07f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d956108b8565b926108da565b926108da565b926109d5610089565b806109df81610276565b0390a4600190565b50505f90565b6109f6906100fc565b9052565b9190610a0d905f602085019401906109ed565b565b80610a2a610a24610a1f5f61026a565b6100fc565b916100fc565b14610a4657610a4491610a3c5f61026a565b919091610cbd565b565b610a69610a525f61026a565b5f91829163ec442f0560e01b8352600483016109fa565b0390fd5b5f90565b90565b610a88610a83610a8d92610a71565b61024b565b61027e565b90565b90565b610aa7610aa2610aac92610a90565b610301565b61035b565b90565b610ab960ff610a93565b90565b90610ac5610a6d565b50610ad7610ad283610558565b610561565b610aea610ae46020610a74565b9161027e565b105f14610afe5750610afb90610e57565b90565b5f610b0c610b129392610d67565b0161088b565b610b22610b1d610aaf565b6108b8565b90565b5f90565b7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f90565b610b57905161035b565b90565b610b639061035b565b9052565b610b709061027e565b9052565b90959492610bbf94610bae610bb892610ba4608096610b9a60a088019c5f890190610b5a565b6020870190610b5a565b6040850190610b5a565b6060830190610b67565b01906109ed565b565b610bc9610b25565b50610bd2610b29565b610c1c610bdf60e0610b4d565b91610c0d610bee610100610b4d565b46610bf83061058d565b91610c01610089565b96879560208701610b74565b602082018103825203826100b1565b610c2e610c2882610561565b9161055b565b2090565b5f1c90565b60ff1690565b610c49610c4e91610c32565b610c37565b90565b610c5b9054610c3d565b90565b610c85915f610c7a610c8093610c726108af565b5060056108c4565b016108e6565b610c51565b90565b5f90565b610c94610c88565b503390565b7f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a67490565b9182610cd9610cd3610cce5f61026a565b6100fc565b916100fc565b141580610d44575b610cf4575b610cf292919091610f7b565b565b610cfc610f05565b80610d23575b15610ce6575f6336e278fd60e21b815280610d1f60048201610276565b0390fd5b50610d3f610d39610d32610c99565b3390610c5e565b156108b3565b610d02565b5081610d60610d5a610d555f61026a565b6100fc565b916100fc565b1415610ce1565b90565b90565b610d81610d7c610d8692610d6a565b61024b565b61027e565b90565b60209181520190565b90825f9392825e0152565b610dbc610dc5602093610dca93610db381610665565b93848093610d89565b95869101610d92565b610093565b0190565b610de39160208201915f818403910152610d9d565b90565b610e00610dfb610df583610561565b9261055b565b610b4d565b9060208110610e0e575b5090565b610e20905f19906020036008026106ba565b165f610e0a565b610e33610e3891610c32565b61031c565b90565b610e4f610e4a610e549261027e565b610301565b61035b565b90565b610e5f610a6d565b50610e6981610558565b90610e7382610561565b610e86610e80601f610d6d565b9161027e565b11610ebb5750610eb381610ead610ea7610ea2610eb895610de6565b610e27565b91610561565b17610e3b565b6108b8565b90565b610edd90610ec7610089565b91829163305a27a960e01b835260048301610dce565b0390fd5b90565b610ef0610ef591610c32565b610ee1565b90565b610f029054610ee4565b90565b610f0d6108af565b50610f18600c610ef8565b610f2a610f245f6102e5565b9161027e565b141580610f35575b90565b5042610f52610f4c610f47600c610ef8565b61027e565b9161027e565b10610f32565b916020610f79929493610f7260408201965f830190610b67565b0190610b67565b565b9291610f8984838391611084565b83610fa4610f9e610f995f61026a565b6100fc565b916100fc565b14610fb9575b610fb79293919091611251565b565b610fc16111f3565b93610fca611230565b9480610fde610fd88861027e565b9161027e565b11610feb57509350610faa565b85906110075f928392630e58ae9360e11b845260048401610f58565b0390fd5b90611015906108da565b5f5260205260405f2090565b60409061104a611051949695939661104060608401985f8501906109ed565b6020830190610b67565b0190610b67565b565b9061105e910361027e565b90565b9061106c910161027e565b90565b9190611082905f60208501940190610b67565b565b919091806110a261109c6110975f61026a565b6100fc565b916100fc565b145f14611183576110c66110bf836110ba6002610ef8565b6102c0565b600261033b565b5b826110e26110dc6110d75f61026a565b6100fc565b916100fc565b145f14611157576111066110ff836110fa6002610ef8565b611053565b600261033b565b5b91909161115261114061113a7fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef936108da565b936108da565b93611149610089565b9182918261106f565b0390a3565b61117e826111786111695f879061100b565b9161117383610ef8565b611061565b9061033b565b611107565b6111966111915f839061100b565b610ef8565b806111a96111a38561027e565b9161027e565b106111d1576111bc6111cc918490611053565b6111c75f849061100b565b61033b565b6110c7565b906111ef9091925f93849363391434e360e21b855260048501611021565b0390fd5b6111fb610705565b506112066002610ef8565b90565b60018060d01b031690565b61122861122361122d92611209565b61024b565b61027e565b90565b611238610705565b5061124860018060d01b03611214565b90565b90565b90565b916112a96112a36112b0948061127761127161126c5f61026a565b6100fc565b916100fc565b146112e1575b8461129861129261128d5f61026a565b6100fc565b916100fc565b146112b2575b6114d9565b926114d9565b909161150e565b565b6112da600b60026112d46112ce6112c8896113c3565b9361124b565b9161124e565b90611416565b505061129e565b611309600b60016113036112fd6112f7896113c3565b9361124b565b9161124e565b90611416565b505061127d565b5f90565b61132061132691611209565b91611209565b019060018060d01b03821161133757565b6102ac565b9061134f91611349611310565b50611314565b90565b90565b60ff1690565b61136f61136a61137492611352565b61024b565b611355565b90565b6113809061135b565b9052565b9160206113a592949361139e60408201965f830190611377565b0190610b67565b565b6113bb6113b66113c09261027e565b61024b565b611209565b90565b6113cb611310565b50806113e56113df60018060d01b03611214565b9161027e565b116113f6576113f3906113a7565b90565b60d06114125f9283926306dfcc6560e41b845260048401611384565b0390fd5b9061144c6114529392611427611310565b50611430611310565b50809361144561143e6116c0565b949261176d565b9091611c53565b916117e2565b91909190565b61146461146a91611209565b91611209565b90039060018060d01b03821161147c57565b6102ac565b906114949161148e611310565b50611458565b90565b906114a1906108da565b5f5260205260405f2090565b60018060a01b031690565b6114c46114c991610c32565b6114ad565b90565b6114d690546114b8565b90565b6114f06114f5916114e8610c88565b506009611497565b6114cc565b90565b90611502906108da565b5f5260205260405f2090565b9190918061152461151e856100fc565b916100fc565b1415806116a2575b611536575b505050565b8061155161154b6115465f61026a565b6100fc565b916100fc565b03611612575b508161157361156d6115685f61026a565b6100fc565b916100fc565b0361157f575b80611531565b6115c66115b96115c092611595600a86906114f8565b906115b36115ad6115a76001936113c3565b9361124b565b9161124e565b90611416565b9290611214565b91611214565b9190916115f37fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926108da565b926116086115ff610089565b92839283610f58565b0390a25f80611579565b61165161165761164a611627600a85906114f8565b600261164461163e611638896113c3565b9361124b565b9161124e565b90611416565b9290611214565b91611214565b9190916116847fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926108da565b92611699611690610089565b92839283610f58565b0390a25f611557565b50816116b66116b05f6102e5565b9161027e565b1161152c565b5f90565b6116c86116bc565b506116d1611811565b90565b5490565b90565b6116ef6116ea6116f4926116d8565b61024b565b61027e565b90565b61170661170c9193929361027e565b9261027e565b820391821161171757565b6102ac565b90565b60301c90565b60018060d01b031690565b61173c6117419161171f565b611725565b90565b61174e9054611730565b90565b61176561176061176a92610248565b61024b565b611209565b90565b611775611310565b506117815f82016116d4565b8061179461178e5f6102e5565b9161027e565b145f146117aa5750506117a65f611751565b5b90565b6117d75f916117d26117cc846117dd9601926117c660016116db565b906116f7565b9161171c565b611826565b01611744565b6117a7565b916118065f61180b946117f3611310565b506117fc611310565b500192919261171c565b611a2b565b91909190565b6118196116bc565b5061182343611bec565b90565b5f5260205f200190565b5490565b61183e60406100d8565b90565b65ffffffffffff1690565b9061185690611841565b9052565b9061186490611209565b9052565b5f5260205f2090565b634e487b7160e01b5f52603260045260245ffd5b61188e81611830565b8210156118a8576118a0600191611868565b910201905f90565b611871565b6118b79051611841565b90565b906118cb65ffffffffffff91610301565b9181191691161790565b6118e96118e46118ee92611841565b61024b565b611841565b90565b90565b90611909611904611910926118d5565b6118f1565b82546118ba565b9055565b61191e9051611209565b90565b60301b90565b9061193965ffffffffffff1991611921565b9181191691161790565b61195761195261195c92611209565b61024b565b611209565b90565b90565b9061197761197261197e92611943565b61195f565b8254611927565b9055565b906119ac60205f6119b2946119a482820161199e8488016118ad565b906118f4565b019201611914565b90611962565b565b91906119c5576119c391611982565b565b610652565b90815491680100000000000000008310156119fa57826119f29160016119f895018155611885565b906119b4565b565b61009d565b65ffffffffffff1690565b611a16611a1b91610c32565b6119ff565b90565b611a289054611a0a565b90565b90929192611a37611310565b50611a40611310565b50611a4a82611830565b80611a5d611a575f6102e5565b9161027e565b115f14611b2d57611a8390611a7d8491611a7760016116db565b906116f7565b90611826565b90611a8f5f8301611a1e565b92611a9b5f8401611744565b9380611aaf611aa985611841565b91611841565b11611b1157611ac6611ac084611841565b91611841565b145f14611ae1575050611adc905f859101611962565b5b9190565b611b0c9250611b0786611afe611af5611834565b945f860161184c565b6020840161185a565b6119ca565b611add565b5f632520601d60e01b815280611b2960048201610276565b0390fd5b50611b5891611b5385611b4a611b41611834565b945f860161184c565b6020840161185a565b6119ca565b611b615f611751565b9190565b611b79611b74611b7e92611841565b61024b565b61027e565b90565b90565b611b98611b93611b9d92611b81565b61024b565b611355565b90565b611ba990611b84565b9052565b916020611bce929493611bc760408201965f830190611ba0565b0190610b67565b565b611be4611bdf611be99261027e565b61024b565b611841565b90565b611bf46116bc565b5080611c0e611c0865ffffffffffff611b65565b9161027e565b11611c1f57611c1c90611bd0565b90565b6030611c3b5f9283926306dfcc6560e41b845260048401611bad565b0390fd5b634e487b7160e01b5f52605160045260245ffd5b91909180600114611c7257600203611c3f57611c6e91611481565b905b565b50611c7c9161133c565b90611c7056fe60806040526004361015610013575b6114d3565b61001d5f3561030c565b806301ffc9a71461030757806306fdde0314610302578063095ea7b3146102fd57806318160ddd146102f857806323b872dd146102f3578063248a9ca3146102ee5780632f2ff15d146102e9578063313ce567146102e45780633644e515146102df57806336568abe146102da5780633a46b1a8146102d557806340c10f19146102d057806342966c68146102cb5780634bdd36ce146102c65780634bf5d7e9146102c15780634f1bfc9e146102bc578063587cde1e146102b75780635c19a95c146102b25780636fcfff45146102ad57806370a08231146102a857806379cc6790146102a35780637a8cd1561461029e5780637ecebe001461029957806383f1211b146102945780638426adf21461028f578063844c90261461028a57806384b0196e146102855780638a542521146102805780638d3343d61461027b5780638e539e8c14610276578063902d55a51461027157806391d148541461026c57806391ddadf41461026757806395d89b41146102625780639ab24eb01461025d5780639b7ef64b14610258578063a217fddf14610253578063a9059cbb1461024e578063aa082a9d14610249578063b0ca253e14610244578063bb4d44361461023f578063c02ae7541461023a578063c3cda52014610235578063d505accf14610230578063d547741f1461022b578063dd62ed3e146102265763f1127ed80361000e5761149d565b6113b9565b611358565b61131e565b611274565b6111b8565b611183565b61114d565b611118565b6110a6565b611071565b611001565b610f8a565b610f55565b610f20565b610ebd565b610e88565b610e11565b610ddc565b610d78565b610d0d565b610bc8565b610b93565b610b3a565b610b05565b610ad0565b610a9c565b610a67565b610a32565b6109d4565b61099f565b61092a565b6108b9565b610884565b610851565b6107ff565b6107c9565b610795565b610760565b61072b565b6106cf565b610668565b6105cc565b61055d565b610505565b610443565b610394565b60e01c90565b60405190565b5f80fd5b5f80fd5b63ffffffff60e01b1690565b61033581610320565b0361033c57565b5f80fd5b9050359061034d8261032c565b565b9060208282031261036857610365915f01610340565b90565b61031c565b151590565b61037b9061036d565b9052565b9190610392905f60208501940190610372565b565b346103c4576103c06103af6103aa36600461034f565b611570565b6103b7610312565b9182918261037f565b0390f35b610318565b5f9103126103d357565b61031c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b61041961042260209361042793610410816103d8565b938480936103dc565b958691016103e5565b6103f0565b0190565b6104409160208201915f8184039101526103fa565b90565b34610473576104533660046103c9565b61046f61045e611709565b610466610312565b9182918261042b565b0390f35b610318565b60018060a01b031690565b61048c90610478565b90565b61049881610483565b0361049f57565b5f80fd5b905035906104b08261048f565b565b90565b6104be816104b2565b036104c557565b5f80fd5b905035906104d6826104b5565b565b919060408382031261050057806104f46104fd925f86016104a3565b936020016104c9565b90565b61031c565b346105365761053261052161051b3660046104d8565b9061171f565b610529610312565b9182918261037f565b0390f35b610318565b610544906104b2565b9052565b919061055b905f6020850194019061053b565b565b3461058d5761056d3660046103c9565b61058961057861176b565b610580610312565b91829182610548565b0390f35b610318565b90916060828403126105c7576105c46105ad845f85016104a3565b936105bb81602086016104a3565b936040016104c9565b90565b61031c565b346105fd576105f96105e86105e2366004610592565b91611781565b6105f0610312565b9182918261037f565b0390f35b610318565b90565b61060e81610602565b0361061557565b5f80fd5b9050359061062682610605565b565b906020828203126106415761063e915f01610619565b90565b61031c565b61064f90610602565b9052565b9190610666905f60208501940190610646565b565b346106985761069461068361067e366004610628565b6117fa565b61068b610312565b91829182610653565b0390f35b610318565b91906040838203126106c557806106b96106c2925f8601610619565b936020016104a3565b90565b61031c565b5f0190565b346106fe576106e86106e236600461069d565b90611846565b6106f0610312565b806106fa816106ca565b0390f35b610318565b60ff1690565b61071290610703565b9052565b9190610729905f60208501940190610709565b565b3461075b5761073b3660046103c9565b610757610746611875565b61074e610312565b91829182610716565b0390f35b610318565b34610790576107703660046103c9565b61078c61077b61188b565b610783610312565b91829182610653565b0390f35b610318565b346107c4576107ae6107a836600461069d565b9061189f565b6107b6610312565b806107c0816106ca565b0390f35b610318565b346107fa576107f66107e56107df3660046104d8565b90611950565b6107ed610312565b91829182610548565b0390f35b610318565b3461082e576108186108123660046104d8565b90611ad7565b610820610312565b8061082a816106ca565b0390f35b610318565b9060208282031261084c57610849915f016104c9565b90565b61031c565b3461087f57610869610864366004610833565b611ae3565b610871610312565b8061087b816106ca565b0390f35b610318565b346108b4576108943660046103c9565b6108b061089f611b48565b6108a7610312565b91829182610548565b0390f35b610318565b346108e9576108c93660046103c9565b6108e56108d4611c07565b6108dc610312565b9182918261042b565b0390f35b610318565b90565b90565b61090861090361090d926108ee565b6108f1565b6104b2565b90565b61091c6276a7006108f4565b90565b610927610910565b90565b3461095a5761093a3660046103c9565b61095661094561091f565b61094d610312565b91829182610548565b0390f35b610318565b9060208282031261097857610975915f016104a3565b90565b61031c565b61098690610483565b9052565b919061099d905f6020850194019061097d565b565b346109cf576109cb6109ba6109b536600461095f565b611ca3565b6109c2610312565b9182918261098a565b0390f35b610318565b34610a02576109ec6109e736600461095f565b611cc2565b6109f4610312565b806109fe816106ca565b0390f35b610318565b63ffffffff1690565b610a1990610a07565b9052565b9190610a30905f60208501940190610a10565b565b34610a6257610a5e610a4d610a4836600461095f565b611cd9565b610a55610312565b91829182610a1d565b0390f35b610318565b34610a9757610a93610a82610a7d36600461095f565b611d04565b610a8a610312565b91829182610548565b0390f35b610318565b34610acb57610ab5610aaf3660046104d8565b90611e39565b610abd610312565b80610ac7816106ca565b0390f35b610318565b34610b0057610ae03660046103c9565b610afc610aeb611e45565b610af3610312565b91829182610548565b0390f35b610318565b34610b3557610b31610b20610b1b36600461095f565b611ebd565b610b28610312565b91829182610548565b0390f35b610318565b34610b6a57610b4a3660046103c9565b610b66610b55611ed2565b610b5d610312565b9182918261037f565b0390f35b610318565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610bc357610ba33660046103c9565b610bbf610bae610b6f565b610bb6610312565b91829182610548565b0390f35b610318565b34610bf657610be0610bdb366004610833565b61209d565b610be8610312565b80610bf2816106ca565b0390f35b610318565b60ff60f81b1690565b610c0d90610bfb565b9052565b5190565b60209181520190565b60200190565b610c2d906104b2565b9052565b90610c3e81602093610c24565b0190565b60200190565b90610c65610c5f610c5884610c11565b8093610c15565b92610c1e565b905f5b818110610c755750505090565b909192610c8e610c886001928651610c31565b94610c42565b9101919091610c68565b93959194610ce9610cde610cfd95610cd0610cf395610d0a9c9a610cc360e08c01925f8d0190610c04565b8a820360208c01526103fa565b9088820360408a01526103fa565b97606087019061053b565b608085019061097d565b60a0830190610646565b60c0818403910152610c48565b90565b34610d4457610d1d3660046103c9565b610d40610d28612125565b93610d37979597939193610312565b97889788610c98565b0390f35b610318565b7f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a67490565b610d75610d49565b90565b34610da857610d883660046103c9565b610da4610d93610d6d565b610d9b610312565b91829182610653565b0390f35b610318565b7f9b12e0c5707e494915e58b0564f18aaad9b74ac69bfc815a1edadc8e4bd032eb90565b610dd9610dad565b90565b34610e0c57610dec3660046103c9565b610e08610df7610dd1565b610dff610312565b91829182610653565b0390f35b610318565b34610e4157610e3d610e2c610e27366004610833565b6121af565b610e34610312565b91829182610548565b0390f35b610318565b90565b610e5d610e58610e6292610e46565b6108f1565b6104b2565b90565b610e7a6b033b2e3c9fd0803ce8000000610e49565b90565b610e85610e65565b90565b34610eb857610e983660046103c9565b610eb4610ea3610e7d565b610eab610312565b91829182610548565b0390f35b610318565b34610eee57610eea610ed9610ed336600461069d565b9061221d565b610ee1610312565b9182918261037f565b0390f35b610318565b65ffffffffffff1690565b610f0790610ef3565b9052565b9190610f1e905f60208501940190610efe565b565b34610f5057610f303660046103c9565b610f4c610f3b61224b565b610f43610312565b91829182610f0b565b0390f35b610318565b34610f8557610f653660046103c9565b610f81610f7061225f565b610f78610312565b9182918261042b565b0390f35b610318565b34610fba57610fb6610fa5610fa036600461095f565b612275565b610fad610312565b91829182610548565b0390f35b610318565b90565b610fd6610fd1610fdb92610fbf565b6108f1565b6104b2565b90565b610ff36b02e87669c308736a04000000610fc2565b90565b610ffe610fde565b90565b34611031576110113660046103c9565b61102d61101c610ff6565b611024610312565b91829182610548565b0390f35b610318565b90565b5f1b90565b61105261104d61105792611036565b611039565b610602565b90565b6110635f61103e565b90565b61106e61105a565b90565b346110a1576110813660046103c9565b61109d61108c611066565b611094610312565b91829182610653565b0390f35b610318565b346110d7576110d36110c26110bc3660046104d8565b906122a4565b6110ca610312565b9182918261037f565b0390f35b610318565b1c90565b90565b6110f39060086110f893026110dc565b6110e0565b90565b9061110691546110e3565b90565b611115600c5f906110fb565b90565b34611148576111283660046103c9565b611144611133611109565b61113b610312565b91829182610548565b0390f35b610318565b3461117e5761117a6111696111633660046104d8565b906122c6565b611171610312565b91829182610548565b0390f35b610318565b346111b3576111af61119e61119936600461095f565b6122dc565b6111a6610312565b91829182610548565b0390f35b610318565b346111e8576111c83660046103c9565b6111e46111d36122f1565b6111db610312565b91829182610548565b0390f35b610318565b6111f681610703565b036111fd57565b5f80fd5b9050359061120e826111ed565b565b909160c08284031261126f57611228835f84016104a3565b9261123681602085016104c9565b9261124482604083016104c9565b9261126c6112558460608501611201565b936112638160808601610619565b9360a001610619565b90565b61031c565b346112a957611293611287366004611210565b94939093929192612371565b61129b610312565b806112a5816106ca565b0390f35b610318565b60e081830312611319576112c4825f83016104a3565b926112d283602084016104a3565b926112e081604085016104c9565b926112ee82606083016104c9565b926113166112ff8460808501611201565b9361130d8160a08601610619565b9360c001610619565b90565b61031c565b346113535761133d6113313660046112ae565b959490949391936124c5565b611345610312565b8061134f816106ca565b0390f35b610318565b346113875761137161136b36600461069d565b906125e3565b611379610312565b80611383816106ca565b0390f35b610318565b91906040838203126113b457806113a86113b1925f86016104a3565b936020016104a3565b90565b61031c565b346113ea576113e66113d56113cf36600461138c565b90612605565b6113dd610312565b91829182610548565b0390f35b610318565b6113f881610a07565b036113ff57565b5f80fd5b90503590611410826113ef565b565b919060408382031261143a578061142e611437925f86016104a3565b93602001611403565b90565b61031c565b61144890610ef3565b9052565b60018060d01b031690565b6114609061144c565b9052565b906020806114869361147c5f8201515f86019061143f565b0151910190611457565b565b919061149b905f60408501940190611464565b565b346114ce576114ca6114b96114b3366004611412565b90612673565b6114c1610312565b91829182611488565b0390f35b610318565b5f80fd5b5f90565b634e487b7160e01b5f52601160045260245ffd5b6114fb6115019161144c565b9161144c565b019060018060d01b03821161151257565b6114db565b9061152a916115246114d7565b506114ef565b90565b61153961153f9161144c565b9161144c565b90039060018060d01b03821161155157565b6114db565b90611569916115636114d7565b5061152d565b90565b5f90565b61157861156c565b508061159361158d637965db0b60e01b610320565b91610320565b149081156115a0575b5090565b6115aa9150612689565b5f61159c565b606090565b634e487b7160e01b5f52602260045260245ffd5b90600160028304921680156115e9575b60208310146115e457565b6115b5565b91607f16916115d9565b60209181520190565b5f5260205f2090565b905f929180549061161f611618836115c9565b80946115f3565b916001811690815f14611676575060011461163a575b505050565b61164791929394506115fc565b915f925b81841061165e57505001905f8080611635565b6001816020929593955484860152019101929061164b565b92949550505060ff19168252151560200201905f8080611635565b9061169b91611605565b90565b634e487b7160e01b5f52604160045260245ffd5b906116bc906103f0565b810190811067ffffffffffffffff8211176116d657604052565b61169e565b906116fb6116f4926116eb610312565b93848092611691565b03836116b2565b565b611706906116db565b90565b6117116115b0565b5061171c60036116fd565b90565b61173c9161172b61156c565b506117346126af565b9190916126bc565b600190565b5f90565b5f1c90565b61175661175b91611745565b6110e0565b90565b611768905461174a565b90565b611773611741565b5061177e600261175e565b90565b916117ab9261178e61156c565b506117a361179a6126af565b8290849161270c565b919091612798565b600190565b5f90565b6117bd90610602565b90565b906117ca906117b4565b5f5260205260405f2090565b90565b6117e56117ea91611745565b6117d6565b90565b6117f790546117d9565b90565b60016118136118199261180b6117b0565b5060056117c0565b016117ed565b90565b906118379161183261182d826117fa565b612835565b611839565b565b906118439161288e565b50565b906118509161181c565b565b5f90565b90565b61186d61186861187292611856565b6108f1565b610703565b90565b61187d611852565b506118886012611859565b90565b6118936117b0565b5061189c61293a565b90565b90806118ba6118b46118af6126af565b610483565b91610483565b036118cb576118c8916129f4565b50565b5f63334bd91960e11b8152806118e3600482016106ca565b0390fd5b6118fb6118f661190092610478565b6108f1565b610478565b90565b61190c906118e7565b90565b61191890611903565b90565b906119259061190f565b5f5260205260405f2090565b90565b61194861194361194d9261144c565b6108f1565b6104b2565b90565b6119879161197c61197661197161198294611969611741565b50600a61191b565b611931565b91612ad5565b90612bea565b611934565b90565b906119a49161199f61199a610dad565b612835565b611a0f565b565b6119ba6119b56119bf92611036565b6108f1565b610478565b90565b6119cb906119a6565b90565b6119e26119dd6119e792611036565b6108f1565b6104b2565b90565b6119f96119ff919392936104b2565b926104b2565b8201809211611a0a57565b6114db565b9081611a2b611a25611a205f6119c2565b610483565b91610483565b14611abb5780611a43611a3d5f6119ce565b916104b2565b14611a9f57611a5a611a5361176b565b82906119ea565b611a73611a6d611a68610e65565b6104b2565b916104b2565b11611a8357611a8191612d11565b565b5f63177e3fc360e01b815280611a9b600482016106ca565b0390fd5b5f631f2a200560e01b815280611ab7600482016106ca565b0390fd5b5f63d92e233d60e01b815280611ad3600482016106ca565b0390fd5b90611ae19161198a565b565b80611af6611af05f6119ce565b916104b2565b14611b0757611b059033612d6f565b565b5f631f2a200560e01b815280611b1f600482016106ca565b0390fd5b611b32611b38919392936104b2565b926104b2565b8203918211611b4357565b6114db565b611b50611741565b50611b6a611b5c610e65565b611b6461176b565b90611b23565b90565b90611b80611b79610312565b92836116b2565b565b67ffffffffffffffff8111611ba057611b9c6020916103f0565b0190565b61169e565b90611bb7611bb283611b82565b611b6d565b918252565b5f7f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c74000000910152565b611bed601d611ba5565b90611bfa60208301611bbc565b565b611c04611be3565b90565b611c0f6115b0565b50611c1861224b565b611c31611c2b611c26612dce565b610ef3565b91610ef3565b03611c4157611c3e611bfc565b90565b5f6301bfc1c560e61b815280611c59600482016106ca565b0390fd5b5f90565b90611c6b9061190f565b5f5260205260405f2090565b60018060a01b031690565b611c8e611c9391611745565b611c77565b90565b611ca09054611c82565b90565b611cba611cbf91611cb2611c5d565b506009611c61565b611c96565b90565b611cd390611cce6126af565b612e21565b565b5f90565b611ceb90611ce5611cd5565b50612eac565b90565b90611cf89061190f565b5f5260205260405f2090565b611d1a611d1f91611d13611741565b505f611cee565b61175e565b90565b90611d3c91611d37611d32610d49565b612835565b611d3e565b565b80611d59611d53611d4e5f6119c2565b610483565b91610483565b14611e1d5781611d71611d6b5f6119ce565b916104b2565b14611e0157611d87611d81611ed2565b1561036d565b611de557611d96818390612d6f565b3390611de0611dce611dc87fbef4f81c1814c641ede85ebaacf19d048b2c5b55980adfa6ef0f956c651335a29361190f565b9361190f565b93611dd7610312565b91829182610548565b0390a3565b5f63b8b5ca2d60e01b815280611dfd600482016106ca565b0390fd5b5f631f2a200560e01b815280611e19600482016106ca565b0390fd5b5f63d92e233d60e01b815280611e35600482016106ca565b0390fd5b90611e4391611d22565b565b611e4d611741565b50611e58600c61175e565b611e6a611e645f6119ce565b916104b2565b148015611e99575b611e8d57611e8a611e83600c61175e565b4290611b23565b90565b611e965f6119ce565b90565b5042611eb6611eb0611eab600c61175e565b6104b2565b916104b2565b1015611e72565b611ecf90611ec9611741565b50612edb565b90565b611eda61156c565b50611ee5600c61175e565b611ef7611ef15f6119ce565b916104b2565b141580611f02575b90565b5042611f1f611f19611f14600c61175e565b6104b2565b916104b2565b10611eff565b611f3e90611f39611f3461105a565b612835565b611fb8565b565b90611f4c5f1991611039565b9181191691161790565b611f6a611f65611f6f926104b2565b6108f1565b6104b2565b90565b90565b90611f8a611f85611f9192611f56565b611f72565b8254611f40565b9055565b916020611fb6929493611faf60408201965f83019061053b565b019061053b565b565b80611fcb611fc5426104b2565b916104b2565b11156120815780612004611ffe7f00000000000000000000000000000000000000000000000000000000000000006104b2565b916104b2565b1161206557612013600c61175e565b61201e82600c611f75565b90339061204b7fdd6896dcf1d4b311cca87dd19bbba2ea9ce2f867c1568878a0438a66a1afeeec9261190f565b92612060612057610312565b92839283611f95565b0390a2565b5f63ef69af6560e01b81528061207d600482016106ca565b0390fd5b5f63a565835360e01b815280612099600482016106ca565b0390fd5b6120a690611f25565b565b5f90565b606090565b6120ba90611903565b90565b67ffffffffffffffff81116120d55760208091020190565b61169e565b906120ec6120e7836120bd565b611b6d565b918252565b369037565b9061211b612103836120da565b9260208061211186936120bd565b92019103906120f1565b565b600f60f81b90565b61212d6120a8565b506121366115b0565b5061213f6115b0565b50612148611741565b50612151611c5d565b5061215a6117b0565b506121636120ac565b5061216c612ef3565b90612175612f33565b904690612181306120b1565b9061218b5f61103e565b9061219d6121985f6119ce565b6120f6565b906121a661211d565b96959493929190565b6121d86121dd916121be611741565b506121d26121cc600b611931565b91612ad5565b90612bea565b611934565b90565b906121ea9061190f565b5f5260205260405f2090565b60ff1690565b61220861220d91611745565b6121f6565b90565b61221a90546121fc565b90565b612244915f61223961223f9361223161156c565b5060056117c0565b016121e0565b612210565b90565b5f90565b612253612247565b5061225c612dce565b90565b6122676115b0565b5061227260046116fd565b90565b61229c6122976122926122a19361228a611741565b50600a61191b565b611931565b612f73565b611934565b90565b6122c1916122b061156c565b506122b96126af565b919091612798565b600190565b906122d9916122d3611741565b50611950565b90565b6122ee906122e8611741565b50612275565b90565b6122f9611741565b5061230261176b565b90565b7fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf90565b61235e6123659461235460609498979561234a608086019a5f870190610646565b602085019061097d565b604083019061053b565b019061053b565b565b60200190565b5190565b939594909291954261238b612385896104b2565b916104b2565b1161240457916123f6916123fd936123ed61240298996123d56123ac612305565b6123c68b938b6123ba610312565b95869460208601612329565b602082018103825203826116b2565b6123e76123e18261236d565b91612367565b20612fe8565b92909192613005565b918261304f565b612e21565b565b61241f875f918291632341d78760e11b835260048301610548565b0390fd5b7f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c990565b919461248f6124999298979561248560a09661247b6124a09a61247160c08a019e5f8b0190610646565b602089019061097d565b604087019061097d565b606085019061053b565b608083019061053b565b019061053b565b565b9160206124c39294936124bc60408201965f83019061097d565b019061097d565b565b9695919392949094426124e06124da836104b2565b916104b2565b1161259a57906125496125529493926125316124fa612423565b6125228c80948c9161250c8d916130a1565b9192612516610312565b97889660208801612447565b602082018103825203826116b2565b61254361253d8261236d565b91612367565b20612fe8565b92909192613005565b8061256561255f87610483565b91610483565b0361257a575061257892939190916126bc565b565b84906125965f9283926325c0072360e11b8452600484016124a2565b0390fd5b6125b5905f91829163313c898160e11b835260048301610548565b0390fd5b906125d4916125cf6125ca826117fa565b612835565b6125d6565b565b906125e0916129f4565b50565b906125ed916125b9565b565b906125f99061190f565b5f5260205260405f2090565b61262a9161262061262592612618611741565b5060016125ef565b611cee565b61175e565b90565b6126376040611b6d565b90565b5f90565b5f90565b61264a61262d565b906020808361265761263a565b81520161266261263e565b81525050565b612670612642565b90565b9061268691612680612668565b506130d4565b90565b61269161156c565b506126ab6126a56301ffc9a760e01b610320565b91610320565b1490565b6126b7611c5d565b503390565b916126ca92916001926130fc565b565b6040906126f56126fc94969593966126eb60608401985f85019061097d565b602083019061053b565b019061053b565b565b9061270991036104b2565b90565b92919261271a818390612605565b908161272f6127295f196104b2565b916104b2565b1061273c575b5050509050565b8161274f612749876104b2565b916104b2565b106127755761276c93946127649193926126fe565b905f926130fc565b805f8080612735565b50612794849291925f938493637dc7a0d960e11b8552600485016126cc565b0390fd5b91826127b46127ae6127a95f6119c2565b610483565b91610483565b1461280e57816127d46127ce6127c95f6119c2565b610483565b91610483565b146127e7576127e59291909161320b565b565b61280a6127f35f6119c2565b5f91829163ec442f0560e01b83526004830161098a565b0390fd5b61283161281a5f6119c2565b5f918291634b637e8f60e11b83526004830161098a565b0390fd5b612847906128416126af565b906132d8565b565b9061285560ff91611039565b9181191691161790565b6128689061036d565b90565b90565b9061288361287e61288a9261285f565b61286b565b8254612849565b9055565b61289661156c565b506128ab6128a582849061221d565b1561036d565b5f14612934576128d360016128ce5f6128c6600586906117c0565b0185906121e0565b61286e565b906128dc6126af565b9061291961291361290d7f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d956117b4565b9261190f565b9261190f565b92612922610312565b8061292c816106ca565b0390a4600190565b50505f90565b6129426117b0565b5061294c306120b1565b61297e6129787f0000000000000000000000000000000000000000000000000000000000000000610483565b91610483565b14806129ba575b5f146129af577f000000000000000000000000000000000000000000000000000000000000000090565b6129b7613384565b90565b50466129ee6129e87f00000000000000000000000000000000000000000000000000000000000000006104b2565b916104b2565b14612985565b6129fc61156c565b50612a0881839061221d565b5f14612a9057612a2f5f612a2a5f612a22600586906117c0565b0185906121e0565b61286e565b90612a386126af565b90612a75612a6f612a697ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b956117b4565b9261190f565b9261190f565b92612a7e610312565b80612a88816106ca565b0390a4600190565b50505f90565b612aaa612aa5612aaf92610ef3565b6108f1565b6104b2565b90565b916020612ad3929493612acc60408201965f83019061053b565b0190610efe565b565b612add612247565b50612ae661224b565b81612af9612af383612a96565b916104b2565b1015612b0c5750612b099061348d565b90565b90612b275f928392637669fc0f60e11b845260048401612ab2565b0390fd5b5490565b90565b612b46612b41612b4b92612b2f565b6108f1565b6104b2565b90565b90565b65ffffffffffff1690565b612b68612b6d91611745565b612b51565b90565b612b7a9054612b5c565b90565b90565b612b94612b8f612b9992612b7d565b6108f1565b6104b2565b90565b60301c90565b60018060d01b031690565b612bb9612bbe91612b9c565b612ba2565b90565b612bcb9054612bad565b90565b612be2612bdd612be792611036565b6108f1565b61144c565b90565b90612c3e90612bf76114d7565b50612c035f8401612b2b565b612c0c5f6119ce565b908080612c22612c1c6005612b32565b916104b2565b11612c9f575b5090612c395f860193919293612b4e565b613ae0565b80612c51612c4b5f6119ce565b916104b2565b145f14612c67575050612c635f612bce565b5b90565b612c945f91612c8f612c8984612c9a960192612c836001612b80565b90611b23565b91612b4e565b613ad6565b01612bc1565b612c64565b80612cad612cb3929161376b565b90611b23565b9083612ce5612cdf612cda5f612cd4818c01612ccf8991612b4e565b613ad6565b01612b70565b610ef3565b91610ef3565b105f14612cf65750905b905f612c28565b9150612d0c90612d066001612b80565b906119ea565b612cef565b80612d2c612d26612d215f6119c2565b610483565b91610483565b14612d4857612d4691612d3e5f6119c2565b91909161320b565b565b612d6b612d545f6119c2565b5f91829163ec442f0560e01b83526004830161098a565b0390fd5b9081612d8b612d85612d805f6119c2565b610483565b91610483565b14612da757612da59190612d9e5f6119c2565b909161320b565b565b612dca612db35f6119c2565b5f918291634b637e8f60e11b83526004830161098a565b0390fd5b612dd6612247565b50612de04361348d565b90565b90612df460018060a01b0391611039565b9181191691161790565b90565b90612e16612e11612e1d9261190f565b612dfe565b8254612de3565b9055565b90612eaa91612ea4612e3282611ca3565b612e4784612e4260098690611c61565b612e01565b82818590612e87612e81612e7b7f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9561190f565b9261190f565b9261190f565b92612e90610312565b80612e9a816106ca565b0390a49291613b6f565b91613b87565b565b612ed3612ece612ec9612ed893612ec1611cd5565b50600a61191b565b611931565b613d35565b613db4565b90565b612eed90612ee7611741565b50613e05565b90565b90565b612efb6115b0565b50612f307f0000000000000000000000000000000000000000000000000000000000000000612f2a6006612ef0565b90613f20565b90565b612f3b6115b0565b50612f707f0000000000000000000000000000000000000000000000000000000000000000612f6a6007612ef0565b90613f20565b90565b612f7b6114d7565b50612f875f8201612b2b565b80612f9a612f945f6119ce565b916104b2565b145f14612fb0575050612fac5f612bce565b5b90565b612fdd5f91612fd8612fd284612fe3960192612fcc6001612b80565b90611b23565b91612b4e565b613ad6565b01612bc1565b612fad565b61300290612ff46117b0565b50612ffd61293a565b613f6e565b90565b926130209261302994613016611c5d565b5092909192614034565b9092919261415f565b90565b91602061304d92949361304660408201965f83019061097d565b019061053b565b565b613058816130a1565b9161306b613065846104b2565b916104b2565b03613074575050565b61308e5f9283926301d4b62360e61b84526004840161302c565b0390fd5b600161309e91016104b2565b90565b6130b5906130ad611741565b506008611cee565b6130d16130c18261175e565b916130cb83613092565b90611f75565b90565b906130f46130ef6130f9936130e7612668565b50600a61191b565b611931565b6142d5565b90565b90928161311961311361310e5f6119c2565b610483565b91610483565b146131e4578361313961313361312e5f6119c2565b610483565b91610483565b146131bd5761315d83613158613151600186906125ef565b8790611cee565b611f75565b613167575b505050565b9190916131b26131a061319a7f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b9259361190f565b9361190f565b936131a9610312565b91829182610548565b0390a35f8080613162565b6131e06131c95f6119c2565b5f918291634a1406b160e11b83526004830161098a565b0390fd5b6132076131f05f6119c2565b5f91829163e602df0560e01b83526004830161098a565b0390fd5b918261322761322161321c5f6119c2565b610483565b91610483565b141580613292575b613242575b613240929190916142f6565b565b61324a611ed2565b80613271575b15613234575f6336e278fd60e21b81528061326d600482016106ca565b0390fd5b5061328d613287613280610d49565b339061221d565b1561036d565b613250565b50816132ae6132a86132a35f6119c2565b610483565b91610483565b141561322f565b9160206132d69294936132cf60408201965f83019061097d565b0190610646565b565b906132ed6132e783839061221d565b1561036d565b6132f5575050565b61330f5f92839263e2517d3f60e01b8452600484016132b5565b0390fd5b7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f90565b909594926133829461337161337b9261336760809661335d60a088019c5f890190610646565b6020870190610646565b6040850190610646565b606083019061053b565b019061097d565b565b61338c6117b0565b50613395613313565b61340c7f0000000000000000000000000000000000000000000000000000000000000000916133fd7f0000000000000000000000000000000000000000000000000000000000000000466133e8306120b1565b916133f1610312565b96879560208701613337565b602082018103825203826116b2565b61341e6134188261236d565b91612367565b2090565b90565b61343961343461343e92613422565b6108f1565b610703565b90565b61344a90613425565b9052565b91602061346f92949361346860408201965f830190613441565b019061053b565b565b61348561348061348a926104b2565b6108f1565b610ef3565b90565b613495612247565b50806134af6134a965ffffffffffff612a96565b916104b2565b116134c0576134bd90613471565b90565b60306134dc5f9283926306dfcc6560e41b84526004840161344e565b0390fd5b90565b6134f76134f26134fc926134e0565b6108f1565b6104b2565b90565b90565b61351661351161351b926134ff565b6108f1565b610703565b90565b61353d9061353761353161354294610703565b916104b2565b906110dc565b6104b2565b90565b90565b61355c61355761356192613545565b6108f1565b610703565b90565b1b90565b6135879061358161357b61358c94610703565b916104b2565b90613564565b6104b2565b90565b90565b6135a66135a16135ab9261358f565b6108f1565b6104b2565b90565b90565b6135c56135c06135ca926135ae565b6108f1565b610703565b90565b90565b6135e46135df6135e9926135cd565b6108f1565b6104b2565b90565b90565b6136036135fe613608926135ec565b6108f1565b610703565b90565b90565b61362261361d6136279261360b565b6108f1565b6104b2565b90565b90565b61364161363c6136469261362a565b6108f1565b610703565b90565b90565b61366061365b61366592613649565b6108f1565b6104b2565b90565b90565b61367f61367a61368492613668565b6108f1565b610703565b90565b61369b6136966136a0926135ec565b6108f1565b6104b2565b90565b90565b6136ba6136b56136bf926136a3565b6108f1565b610703565b90565b6136d66136d16136db92613668565b6108f1565b6104b2565b90565b6136f26136ed6136f792612b7d565b6108f1565b610703565b90565b90565b61371161370c613716926136fa565b6108f1565b6104b2565b90565b9061372491026104b2565b90565b634e487b7160e01b5f52601260045260245ffd5b61374761374d916104b2565b916104b2565b908115613758570490565b613727565b9061376891016104b2565b90565b613773611741565b50806137886137826001612b80565b916104b2565b1115613ad3578061399d61397a61396a61395a61394a61393a61392a61391a61390a6138fa6138ea8b6138e46138dd6139a39f6138bd6138ad6138cd926137cf6001612b80565b90806137e76137e1600160801b6134e3565b916104b2565b1015613aa5575b8061380a61380468010000000000000000613592565b916104b2565b1015613a77575b806138296138236401000000006135d0565b916104b2565b1015613a49575b806138466138406201000061360e565b916104b2565b1015613a1b575b8061386261385c61010061364c565b916104b2565b10156139ed575b8061387d6138776010613687565b916104b2565b10156139bf575b61389761389160046136c2565b916104b2565b10156139a6575b6138a860036136fd565b613719565b6138b760016136de565b9061351e565b6138c7818661373b565b9061375d565b6138d760016136de565b9061351e565b809261373b565b9061375d565b6138f460016136de565b9061351e565b613904818c61373b565b9061375d565b61391460016136de565b9061351e565b613924818a61373b565b9061375d565b61393460016136de565b9061351e565b613944818861373b565b9061375d565b61395460016136de565b9061351e565b613964818661373b565b9061375d565b61397460016136de565b9061351e565b9161399761399161398c85809461373b565b6104b2565b916104b2565b11614386565b906126fe565b90565b6139ba906139b460016136de565b90613568565b61389e565b6139d66139e7916139d0600461366b565b9061351e565b916139e160026136a6565b90613568565b90613884565b613a04613a15916139fe600861362d565b9061351e565b91613a0f600461366b565b90613568565b90613869565b613a32613a4391613a2c60106135ef565b9061351e565b91613a3d600861362d565b90613568565b9061384d565b613a60613a7191613a5a60206135b1565b9061351e565b91613a6b60106135ef565b90613568565b90613830565b613a8e613a9f91613a886040613548565b9061351e565b91613a9960206135b1565b90613568565b90613811565b613abc613acd91613ab66080613502565b9061351e565b91613ac76040613548565b90613568565b906137ee565b90565b5f5260205f200190565b93919092613aec611741565b505b81613b01613afb836104b2565b916104b2565b1015613b6757613b128282906143d2565b90613b285f613b22888590613ad6565b01612b70565b613b3a613b3487610ef3565b91610ef3565b115f14613b4a5750915b91613aee565b929150613b6190613b5b6001612b80565b906119ea565b90613b44565b925050915090565b613b8190613b7b611741565b50611d04565b90565b90565b91909180613b9d613b9785610483565b91610483565b141580613d1b575b613baf575b505050565b80613bca613bc4613bbf5f6119c2565b610483565b91610483565b03613c8b575b5081613bec613be6613be15f6119c2565b610483565b91610483565b03613bf8575b80613baa565b613c3f613c32613c3992613c0e600a869061191b565b90613c2c613c26613c2060019361446b565b93611931565b91613b84565b906144be565b9290611934565b91611934565b919091613c6c7fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a7249261190f565b92613c81613c78610312565b92839283611f95565b0390a25f80613bf2565b613cca613cd0613cc3613ca0600a859061191b565b6002613cbd613cb7613cb18961446b565b93611931565b91613b84565b906144be565b9290611934565b91611934565b919091613cfd7fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a7249261190f565b92613d12613d09610312565b92839283611f95565b0390a25f613bd0565b5081613d2f613d295f6119ce565b916104b2565b11613ba5565b5f613d4991613d42611741565b5001612b2b565b90565b613d60613d5b613d6592610a07565b6108f1565b6104b2565b90565b613d71906135b1565b9052565b916020613d96929493613d8f60408201965f830190613d68565b019061053b565b565b613dac613da7613db1926104b2565b6108f1565b610a07565b90565b613dbc611cd5565b5080613dd4613dce63ffffffff613d4c565b916104b2565b11613de557613de290613d98565b90565b6020613e015f9283926306dfcc6560e41b845260048401613d75565b0390fd5b613e1c613e2191613e14611741565b506008611cee565b61175e565b90565b90565b613e3b613e36613e4092613e24565b611039565b610602565b90565b613e4d60ff613e27565b90565b5f5260205f2090565b905f9291805490613e73613e6c836115c9565b80946115f3565b916001811690815f14613eca5750600114613e8e575b505050565b613e9b9192939450613e50565b915f925b818410613eb257505001905f8080613e89565b60018160209295939554848601520191019290613e9f565b92949550505060ff19168252151560200201905f8080613e89565b90613eef91613e59565b90565b90613f12613f0b92613f02610312565b93848092613ee5565b03836116b2565b565b613f1d90613ef2565b90565b90613f296115b0565b50613f33826117b4565b613f4c613f46613f41613e43565b610602565b91610602565b14155f14613f615750613f5e90614548565b90565b613f6b9150613f14565b90565b604291613f796117b0565b50604051917f19010000000000000000000000000000000000000000000000000000000000008352600283015260228201522090565b5f90565b613fbf613fc491611745565b611f56565b90565b90565b613fde613fd9613fe392613fc7565b6108f1565b6104b2565b90565b61401b61402294614011606094989795614007608086019a5f870190610646565b6020850190610709565b6040830190610646565b0190610646565b565b61402c610312565b3d5f823e3d90fd5b93929361403f611c5d565b50614048613faf565b506140516117b0565b5061405b85613fb3565b61408d6140877f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0613fca565b916104b2565b1161411a57906140b0602094955f949392936140a7610312565b94859485613fe6565b838052039060015afa15614115576140c85f51611039565b806140e36140dd6140d85f6119c2565b610483565b91610483565b146140f9575f916140f35f61103e565b91929190565b506141035f6119c2565b60019161410f5f61103e565b91929190565b614024565b5050506141265f6119c2565b9060039291929190565b634e487b7160e01b5f52602160045260245ffd5b6004111561414e57565b614130565b9061415d82614144565b565b8061417261416c5f614153565b91614153565b145f1461417d575050565b8061419161418b6001614153565b91614153565b145f146141b4575f63f645eedf60e01b8152806141b0600482016106ca565b0390fd5b806141c86141c26002614153565b91614153565b145f146141f6576141f26141db83613fb3565b5f91829163fce698f760e01b835260048301610548565b0390fd5b6142096142036003614153565b91614153565b146142115750565b61422c905f9182916335e2f38360e21b835260048301610653565b0390fd5b634e487b7160e01b5f52603260045260245ffd5b5f5260205f2090565b61425681612b2b565b82101561427057614268600191614244565b910201905f90565b614230565b9061427f90610ef3565b9052565b9061428d9061144c565b9052565b906142c76142be5f6142a161262d565b946142b86142b0838301612b70565b838801614275565b01612bc1565b60208401614283565b565b6142d290614291565b90565b6142f3915f6142ed926142e6612668565b500161424d565b506142c9565b90565b929161430484838391614578565b8361431f6143196143145f6119c2565b610483565b91610483565b14614334575b6143329293919091614702565b565b61433c61176b565b936143456146e7565b9480614359614353886104b2565b916104b2565b1161436657509350614325565b85906143825f928392630e58ae9360e11b845260048401611f95565b0390fd5b61438e611741565b50151590565b6143a86143a36143ad926136a3565b6108f1565b6104b2565b90565b6143bc6143c2916104b2565b916104b2565b9081156143cd570490565b613727565b6143f76143fd926143e1611741565b5082811692186143f16002614394565b906143b0565b906119ea565b90565b90565b61441761441261441c92614400565b6108f1565b610703565b90565b61442890614403565b9052565b91602061444d92949361444660408201965f83019061441f565b019061053b565b565b61446361445e614468926104b2565b6108f1565b61144c565b90565b6144736114d7565b508061448d61448760018060d01b03611934565b916104b2565b1161449e5761449b9061444f565b90565b60d06144ba5f9283926306dfcc6560e41b84526004840161442c565b0390fd5b906144f46144fa93926144cf6114d7565b506144d86114d7565b5080936144ed6144e661224b565b9492612f73565b9091614b7d565b916147c1565b91909190565b61451461450f614519926135ae565b6108f1565b6104b2565b90565b369037565b9061454661452e83611ba5565b9260208061453c8693611b82565b920191039061451c565b565b6145506115b0565b5061455a8161482b565b9061456d6145686020614500565b614521565b918252602082015290565b9190918061459661459061458b5f6119c2565b610483565b91610483565b145f14614677576145ba6145b3836145ae600261175e565b6119ea565b6002611f75565b5b826145d66145d06145cb5f6119c2565b610483565b91610483565b145f1461464b576145fa6145f3836145ee600261175e565b6126fe565b6002611f75565b5b91909161464661463461462e7fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef9361190f565b9361190f565b9361463d610312565b91829182610548565b0390a3565b6146728261466c61465d5f8790611cee565b916146678361175e565b61375d565b90611f75565b6145fb565b61468a6146855f8390611cee565b61175e565b8061469d614697856104b2565b916104b2565b106146c5576146b06146c09184906126fe565b6146bb5f8490611cee565b611f75565b6145bb565b906146e39091925f93849363391434e360e21b8552600485016126cc565b0390fd5b6146ef611741565b506146ff60018060d01b03611934565b90565b9161475a614754614761948061472861472261471d5f6119c2565b610483565b91610483565b14614792575b8461474961474361473e5f6119c2565b610483565b91610483565b14614763575b611ca3565b92611ca3565b9091613b87565b565b61478b600b600261478561477f6147798961446b565b93611931565b91613b84565b906144be565b505061474f565b6147ba600b60016147b46147ae6147a88961446b565b93611931565b91613b84565b906144be565b505061472e565b916147e55f6147ea946147d26114d7565b506147db6114d7565b5001929192612b4e565b614a2f565b91909190565b6148046147ff61480992613e24565b6108f1565b6104b2565b90565b90565b61482361481e6148289261480c565b6108f1565b6104b2565b90565b6148406148459161483a611741565b506117b4565b613fb3565b61484f60ff6147f0565b168061486461485e601f61480f565b916104b2565b1161486c5790565b5f632cd44ac360e21b815280614884600482016106ca565b0390fd5b5490565b6148966040611b6d565b90565b5f5260205f2090565b6148ab81614888565b8210156148c5576148bd600191614899565b910201905f90565b614230565b634e487b7160e01b5f525f60045260245ffd5b6148e79051610ef3565b90565b906148fb65ffffffffffff91611039565b9181191691161790565b61491961491461491e92610ef3565b6108f1565b610ef3565b90565b90565b9061493961493461494092614905565b614921565b82546148ea565b9055565b61494e905161144c565b90565b60301b90565b9061496965ffffffffffff1991614951565b9181191691161790565b61498761498261498c9261144c565b6108f1565b61144c565b90565b90565b906149a76149a26149ae92614973565b61498f565b8254614957565b9055565b906149dc60205f6149e2946149d48282016149ce8488016148dd565b90614924565b019201614944565b90614992565b565b91906149f5576149f3916149b2565b565b6148ca565b9081549168010000000000000000831015614a2a5782614a22916001614a28950181556148a2565b906149e4565b565b61169e565b90929192614a3b6114d7565b50614a446114d7565b50614a4e82614888565b80614a61614a5b5f6119ce565b916104b2565b115f14614b3157614a8790614a818491614a7b6001612b80565b90611b23565b90613ad6565b90614a935f8301612b70565b92614a9f5f8401612bc1565b9380614ab3614aad85610ef3565b91610ef3565b11614b1557614aca614ac484610ef3565b91610ef3565b145f14614ae5575050614ae0905f859101614992565b5b9190565b614b109250614b0b86614b02614af961488c565b945f8601614275565b60208401614283565b6149fa565b614ae1565b5f632520601d60e01b815280614b2d600482016106ca565b0390fd5b50614b5c91614b5785614b4e614b4561488c565b945f8601614275565b60208401614283565b6149fa565b614b655f612bce565b9190565b634e487b7160e01b5f52605160045260245ffd5b91909180600114614b9c57600203614b6957614b9891611556565b905b565b50614ba691611517565b90614b9a56
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\x80`@R4a\0\x84Wa\0\x1Ba\0\x15a\x01XV[\x90a\x03\xBDV[a\0#a\0\x89V[aK\xACa\x1C\x83\x829`\x80Q\x81a)\x8D\x01R`\xA0Q\x81a)\xC4\x01R`\xC0Q\x81a)T\x01R`\xE0Q\x81a3\x9A\x01Ra\x01\0Q\x81a3\xBF\x01Ra\x01 Q\x81a/\x01\x01Ra\x01@Q\x81a/A\x01Ra\x01`Q\x81\x81\x81a\x0Bq\x01Ra\x1F\xDA\x01RaK\xAC\x90\xF3[a\0\x8FV[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0\xBB\x90a\0\x93V[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\xD3W`@RV[a\0\x9DV[\x90a\0\xEBa\0\xE4a\0\x89V[\x92\x83a\0\xB1V[V[_\x80\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x01\x05\x90a\0\xF1V[\x90V[a\x01\x11\x81a\0\xFCV[\x03a\x01\x18WV[_\x80\xFD[\x90PQ\x90a\x01)\x82a\x01\x08V[V[\x91\x90`@\x83\x82\x03\x12a\x01SW\x80a\x01Ga\x01P\x92_\x86\x01a\x01\x1CV[\x93` \x01a\x01\x1CV[\x90V[a\0\xEDV[a\x01vah/\x808\x03\x80a\x01k\x81a\0\xD8V[\x92\x839\x81\x01\x90a\x01+V[\x90\x91V[`\x01\x80`@\x1B\x03\x81\x11a\x01\x96Wa\x01\x92` \x91a\0\x93V[\x01\x90V[a\0\x9DV[\x90a\x01\xADa\x01\xA8\x83a\x01zV[a\0\xD8V[\x91\x82RV[_\x7FSyndicate\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x01\xE3`\ta\x01\x9BV[\x90a\x01\xF0` \x83\x01a\x01\xB2V[V[a\x01\xFAa\x01\xD9V[\x90V[_\x7FSYND\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x02.`\x04a\x01\x9BV[\x90a\x02;` \x83\x01a\x01\xFDV[V[a\x02Ea\x02$V[\x90V[\x90V[\x90V[a\x02ba\x02]a\x02g\x92a\x02HV[a\x02KV[a\0\xF1V[\x90V[a\x02s\x90a\x02NV[\x90V[_\x01\x90V[\x90V[\x90V[a\x02\x95a\x02\x90a\x02\x9A\x92a\x02{V[a\x02KV[a\x02~V[\x90V[a\x02\xA9bv\xA7\0a\x02\x81V[\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x02\xCFa\x02\xD5\x91\x93\x92\x93a\x02~V[\x92a\x02~V[\x82\x01\x80\x92\x11a\x02\xE0WV[a\x02\xACV[a\x02\xF9a\x02\xF4a\x02\xFE\x92a\x02HV[a\x02KV[a\x02~V[\x90V[_\x1B\x90V[\x90a\x03\x12_\x19\x91a\x03\x01V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x030a\x03+a\x035\x92a\x02~V[a\x02KV[a\x02~V[\x90V[\x90V[\x90a\x03Pa\x03Ka\x03W\x92a\x03\x1CV[a\x038V[\x82Ta\x03\x06V[\x90UV[\x90V[a\x03ra\x03ma\x03w\x92a\x02HV[a\x03\x01V[a\x03[V[\x90V[a\x03\x83_a\x03^V[\x90V[\x90V[a\x03\x9Da\x03\x98a\x03\xA2\x92a\x03\x86V[a\x02KV[a\x02~V[\x90V[a\x03\xBAk\x02\xE8vi\xC3\x08sj\x04\0\0\0a\x03\x89V[\x90V[\x90a\x03\xDFa\x03\xC9a\x01\xF2V[a\x03\xD1a\x01\xF2V[a\x03\xD9a\x02=V[\x91a\x04\xA5V[\x81a\x03\xFAa\x03\xF4a\x03\xEF_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x04\x89W\x80a\x04\x1Aa\x04\x14a\x04\x0F_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x04mWa\x04\\a\x04k\x92a\x048Ba\x042a\x02\x9DV[\x90a\x02\xC0V[a\x01`Ra\x04Oa\x04H_a\x02\xE5V[`\x0Ca\x03;V[a\x04Wa\x03zV[a\tAV[Pa\x04ea\x03\xA5V[\x90a\n\x0FV[V[_c\xD9.#=`\xE0\x1B\x81R\x80a\x04\x85`\x04\x82\x01a\x02vV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x04\xA1`\x04\x82\x01a\x02vV[\x03\x90\xFD[\x90a\x04\xB0\x92\x91a\x04\xB2V[V[\x90a\x04\xBD\x92\x91a\x04\xBFV[V[\x90a\x04\xCA\x92\x91a\x04\xCCV[V[\x90a\x04\xD7\x92\x91a\x04\xD9V[V[\x90a\x04\xE4\x92\x91a\x051V[V[_\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x05\x17`\x01a\x01\x9BV[\x90a\x05$` \x83\x01a\x04\xE6V[V[a\x05.a\x05\rV[\x90V[\x90a\x05E\x92\x91a\x05?a\x05&V[\x90a\x05GV[V[\x90a\x05S\x93\x92\x91a\x05\x99V[V[\x90V[\x90V[` \x01\x90V[Q\x90V[a\x05ya\x05ta\x05~\x92a\0\xF1V[a\x02KV[a\0\xF1V[\x90V[a\x05\x8A\x90a\x05eV[\x90V[a\x05\x96\x90a\x05\x81V[\x90V[a\x05\xAAa\x05\xFA\x94a\x05\xDF\x93\x94a\x06.V[a\x05\xBE\x81a\x05\xB8`\x06a\x05UV[\x90a\n\xBCV[a\x01 Ra\x05\xD6\x83a\x05\xD0`\x07a\x05UV[\x90a\n\xBCV[a\x01@Ra\x05XV[a\x05\xF1a\x05\xEB\x82a\x05aV[\x91a\x05[V[ `\xE0Ra\x05XV[a\x06\x0Ca\x06\x06\x82a\x05aV[\x91a\x05[V[ a\x01\0RF`\xA0Ra\x06\x1Da\x0B\xC1V[`\x80Ra\x06)0a\x05\x8DV[`\xC0RV[\x90a\x068\x91a\x06:V[V[\x90a\x06D\x91a\x06FV[V[\x90a\x06P\x91a\x08\x97V[V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[Q\x90V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x01`\x02\x83\x04\x92\x16\x80\x15a\x06\x9DW[` \x83\x10\x14a\x06\x98WV[a\x06iV[\x91`\x7F\x16\x91a\x06\x8DV[_R` _ \x90V[`\x1F` \x91\x01\x04\x90V[\x1B\x90V[\x91\x90`\x08a\x06\xD9\x91\x02\x91a\x06\xD3_\x19\x84a\x06\xBAV[\x92a\x06\xBAV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x91\x90a\x06\xF9a\x06\xF4a\x07\x01\x93a\x03\x1CV[a\x038V[\x90\x83Ta\x06\xBEV[\x90UV[_\x90V[a\x07\x1B\x91a\x07\x15a\x07\x05V[\x91a\x06\xE3V[V[[\x81\x81\x10a\x07)WPPV[\x80a\x076_`\x01\x93a\x07\tV[\x01a\x07\x1EV[\x91\x90`\x1F\x81\x11a\x07LW[PPPV[a\x07Xa\x07}\x93a\x06\xA7V[\x90` a\x07d\x84a\x06\xB0V[\x83\x01\x93\x10a\x07\x85W[a\x07v\x90a\x06\xB0V[\x01\x90a\x07\x1DV[_\x80\x80a\x07GV[\x91Pa\x07v\x81\x92\x90Pa\x07mV[\x1C\x90V[\x90a\x07\xA7\x90_\x19\x90`\x08\x02a\x07\x93V[\x19\x16\x90V[\x81a\x07\xB6\x91a\x07\x97V[\x90`\x02\x02\x17\x90V[\x90a\x07\xC8\x81a\x06eV[\x90`\x01\x80`@\x1B\x03\x82\x11a\x08\x86Wa\x07\xEA\x82a\x07\xE4\x85Ta\x06}V[\x85a\x07<V[` \x90`\x1F\x83\x11`\x01\x14a\x08\x1EW\x91\x80\x91a\x08\r\x93_\x92a\x08\x12W[PPa\x07\xACV[\x90U[V[\x90\x91P\x01Q_\x80a\x08\x06V[`\x1F\x19\x83\x16\x91a\x08-\x85a\x06\xA7V[\x92_[\x81\x81\x10a\x08nWP\x91`\x02\x93\x91\x85`\x01\x96\x94\x10a\x08TW[PPP\x02\x01\x90Ua\x08\x10V[a\x08d\x91\x01Q`\x1F\x84\x16\x90a\x07\x97V[\x90U_\x80\x80a\x08HV[\x91\x93` `\x01\x81\x92\x87\x87\x01Q\x81U\x01\x95\x01\x92\x01a\x080V[a\0\x9DV[\x90a\x08\x95\x91a\x07\xBEV[V[\x90a\x08\xA6a\x08\xAD\x92`\x03a\x08\x8BV[`\x04a\x08\x8BV[V[_\x90V[\x15\x15\x90V[a\x08\xC1\x90a\x03[V[\x90V[\x90a\x08\xCE\x90a\x08\xB8V[_R` R`@_ \x90V[a\x08\xE3\x90a\x05\x81V[\x90V[\x90a\x08\xF0\x90a\x08\xDAV[_R` R`@_ \x90V[\x90a\t\x08`\xFF\x91a\x03\x01V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\t\x1B\x90a\x08\xB3V[\x90V[\x90V[\x90a\t6a\t1a\t=\x92a\t\x12V[a\t\x1EV[\x82Ta\x08\xFCV[\x90UV[a\tIa\x08\xAFV[Pa\t^a\tX\x82\x84\x90a\x0C^V[\x15a\x08\xB3V[_\x14a\t\xE7Wa\t\x86`\x01a\t\x81_a\ty`\x05\x86\x90a\x08\xC4V[\x01\x85\x90a\x08\xE6V[a\t!V[\x90a\t\x8Fa\x0C\x8CV[\x90a\t\xCCa\t\xC6a\t\xC0\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x95a\x08\xB8V[\x92a\x08\xDAV[\x92a\x08\xDAV[\x92a\t\xD5a\0\x89V[\x80a\t\xDF\x81a\x02vV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a\t\xF6\x90a\0\xFCV[\x90RV[\x91\x90a\n\r\x90_` \x85\x01\x94\x01\x90a\t\xEDV[V[\x80a\n*a\n$a\n\x1F_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\nFWa\nD\x91a\n<_a\x02jV[\x91\x90\x91a\x0C\xBDV[V[a\nia\nR_a\x02jV[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\t\xFAV[\x03\x90\xFD[_\x90V[\x90V[a\n\x88a\n\x83a\n\x8D\x92a\nqV[a\x02KV[a\x02~V[\x90V[\x90V[a\n\xA7a\n\xA2a\n\xAC\x92a\n\x90V[a\x03\x01V[a\x03[V[\x90V[a\n\xB9`\xFFa\n\x93V[\x90V[\x90a\n\xC5a\nmV[Pa\n\xD7a\n\xD2\x83a\x05XV[a\x05aV[a\n\xEAa\n\xE4` a\ntV[\x91a\x02~V[\x10_\x14a\n\xFEWPa\n\xFB\x90a\x0EWV[\x90V[_a\x0B\x0Ca\x0B\x12\x93\x92a\rgV[\x01a\x08\x8BV[a\x0B\"a\x0B\x1Da\n\xAFV[a\x08\xB8V[\x90V[_\x90V[\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x90V[a\x0BW\x90Qa\x03[V[\x90V[a\x0Bc\x90a\x03[V[\x90RV[a\x0Bp\x90a\x02~V[\x90RV[\x90\x95\x94\x92a\x0B\xBF\x94a\x0B\xAEa\x0B\xB8\x92a\x0B\xA4`\x80\x96a\x0B\x9A`\xA0\x88\x01\x9C_\x89\x01\x90a\x0BZV[` \x87\x01\x90a\x0BZV[`@\x85\x01\x90a\x0BZV[``\x83\x01\x90a\x0BgV[\x01\x90a\t\xEDV[V[a\x0B\xC9a\x0B%V[Pa\x0B\xD2a\x0B)V[a\x0C\x1Ca\x0B\xDF`\xE0a\x0BMV[\x91a\x0C\ra\x0B\xEEa\x01\0a\x0BMV[Fa\x0B\xF80a\x05\x8DV[\x91a\x0C\x01a\0\x89V[\x96\x87\x95` \x87\x01a\x0BtV[` \x82\x01\x81\x03\x82R\x03\x82a\0\xB1V[a\x0C.a\x0C(\x82a\x05aV[\x91a\x05[V[ \x90V[_\x1C\x90V[`\xFF\x16\x90V[a\x0CIa\x0CN\x91a\x0C2V[a\x0C7V[\x90V[a\x0C[\x90Ta\x0C=V[\x90V[a\x0C\x85\x91_a\x0Cza\x0C\x80\x93a\x0Cra\x08\xAFV[P`\x05a\x08\xC4V[\x01a\x08\xE6V[a\x0CQV[\x90V[_\x90V[a\x0C\x94a\x0C\x88V[P3\x90V[\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t\x90V[\x91\x82a\x0C\xD9a\x0C\xD3a\x0C\xCE_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14\x15\x80a\rDW[a\x0C\xF4W[a\x0C\xF2\x92\x91\x90\x91a\x0F{V[V[a\x0C\xFCa\x0F\x05V[\x80a\r#W[\x15a\x0C\xE6W_c6\xE2x\xFD`\xE2\x1B\x81R\x80a\r\x1F`\x04\x82\x01a\x02vV[\x03\x90\xFD[Pa\r?a\r9a\r2a\x0C\x99V[3\x90a\x0C^V[\x15a\x08\xB3V[a\r\x02V[P\x81a\r`a\rZa\rU_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14\x15a\x0C\xE1V[\x90V[\x90V[a\r\x81a\r|a\r\x86\x92a\rjV[a\x02KV[a\x02~V[\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[a\r\xBCa\r\xC5` \x93a\r\xCA\x93a\r\xB3\x81a\x06eV[\x93\x84\x80\x93a\r\x89V[\x95\x86\x91\x01a\r\x92V[a\0\x93V[\x01\x90V[a\r\xE3\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\r\x9DV[\x90V[a\x0E\0a\r\xFBa\r\xF5\x83a\x05aV[\x92a\x05[V[a\x0BMV[\x90` \x81\x10a\x0E\x0EW[P\x90V[a\x0E \x90_\x19\x90` \x03`\x08\x02a\x06\xBAV[\x16_a\x0E\nV[a\x0E3a\x0E8\x91a\x0C2V[a\x03\x1CV[\x90V[a\x0EOa\x0EJa\x0ET\x92a\x02~V[a\x03\x01V[a\x03[V[\x90V[a\x0E_a\nmV[Pa\x0Ei\x81a\x05XV[\x90a\x0Es\x82a\x05aV[a\x0E\x86a\x0E\x80`\x1Fa\rmV[\x91a\x02~V[\x11a\x0E\xBBWPa\x0E\xB3\x81a\x0E\xADa\x0E\xA7a\x0E\xA2a\x0E\xB8\x95a\r\xE6V[a\x0E'V[\x91a\x05aV[\x17a\x0E;V[a\x08\xB8V[\x90V[a\x0E\xDD\x90a\x0E\xC7a\0\x89V[\x91\x82\x91c0Z'\xA9`\xE0\x1B\x83R`\x04\x83\x01a\r\xCEV[\x03\x90\xFD[\x90V[a\x0E\xF0a\x0E\xF5\x91a\x0C2V[a\x0E\xE1V[\x90V[a\x0F\x02\x90Ta\x0E\xE4V[\x90V[a\x0F\ra\x08\xAFV[Pa\x0F\x18`\x0Ca\x0E\xF8V[a\x0F*a\x0F$_a\x02\xE5V[\x91a\x02~V[\x14\x15\x80a\x0F5W[\x90V[PBa\x0FRa\x0FLa\x0FG`\x0Ca\x0E\xF8V[a\x02~V[\x91a\x02~V[\x10a\x0F2V[\x91` a\x0Fy\x92\x94\x93a\x0Fr`@\x82\x01\x96_\x83\x01\x90a\x0BgV[\x01\x90a\x0BgV[V[\x92\x91a\x0F\x89\x84\x83\x83\x91a\x10\x84V[\x83a\x0F\xA4a\x0F\x9Ea\x0F\x99_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x0F\xB9W[a\x0F\xB7\x92\x93\x91\x90\x91a\x12QV[V[a\x0F\xC1a\x11\xF3V[\x93a\x0F\xCAa\x120V[\x94\x80a\x0F\xDEa\x0F\xD8\x88a\x02~V[\x91a\x02~V[\x11a\x0F\xEBWP\x93Pa\x0F\xAAV[\x85\x90a\x10\x07_\x92\x83\x92c\x0EX\xAE\x93`\xE1\x1B\x84R`\x04\x84\x01a\x0FXV[\x03\x90\xFD[\x90a\x10\x15\x90a\x08\xDAV[_R` R`@_ \x90V[`@\x90a\x10Ja\x10Q\x94\x96\x95\x93\x96a\x10@``\x84\x01\x98_\x85\x01\x90a\t\xEDV[` \x83\x01\x90a\x0BgV[\x01\x90a\x0BgV[V[\x90a\x10^\x91\x03a\x02~V[\x90V[\x90a\x10l\x91\x01a\x02~V[\x90V[\x91\x90a\x10\x82\x90_` \x85\x01\x94\x01\x90a\x0BgV[V[\x91\x90\x91\x80a\x10\xA2a\x10\x9Ca\x10\x97_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14_\x14a\x11\x83Wa\x10\xC6a\x10\xBF\x83a\x10\xBA`\x02a\x0E\xF8V[a\x02\xC0V[`\x02a\x03;V[[\x82a\x10\xE2a\x10\xDCa\x10\xD7_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14_\x14a\x11WWa\x11\x06a\x10\xFF\x83a\x10\xFA`\x02a\x0E\xF8V[a\x10SV[`\x02a\x03;V[[\x91\x90\x91a\x11Ra\x11@a\x11:\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93a\x08\xDAV[\x93a\x08\xDAV[\x93a\x11Ia\0\x89V[\x91\x82\x91\x82a\x10oV[\x03\x90\xA3V[a\x11~\x82a\x11xa\x11i_\x87\x90a\x10\x0BV[\x91a\x11s\x83a\x0E\xF8V[a\x10aV[\x90a\x03;V[a\x11\x07V[a\x11\x96a\x11\x91_\x83\x90a\x10\x0BV[a\x0E\xF8V[\x80a\x11\xA9a\x11\xA3\x85a\x02~V[\x91a\x02~V[\x10a\x11\xD1Wa\x11\xBCa\x11\xCC\x91\x84\x90a\x10SV[a\x11\xC7_\x84\x90a\x10\x0BV[a\x03;V[a\x10\xC7V[\x90a\x11\xEF\x90\x91\x92_\x93\x84\x93c9\x144\xE3`\xE2\x1B\x85R`\x04\x85\x01a\x10!V[\x03\x90\xFD[a\x11\xFBa\x07\x05V[Pa\x12\x06`\x02a\x0E\xF8V[\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x12(a\x12#a\x12-\x92a\x12\tV[a\x02KV[a\x02~V[\x90V[a\x128a\x07\x05V[Pa\x12H`\x01\x80`\xD0\x1B\x03a\x12\x14V[\x90V[\x90V[\x90V[\x91a\x12\xA9a\x12\xA3a\x12\xB0\x94\x80a\x12wa\x12qa\x12l_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x12\xE1W[\x84a\x12\x98a\x12\x92a\x12\x8D_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x14a\x12\xB2W[a\x14\xD9V[\x92a\x14\xD9V[\x90\x91a\x15\x0EV[V[a\x12\xDA`\x0B`\x02a\x12\xD4a\x12\xCEa\x12\xC8\x89a\x13\xC3V[\x93a\x12KV[\x91a\x12NV[\x90a\x14\x16V[PPa\x12\x9EV[a\x13\t`\x0B`\x01a\x13\x03a\x12\xFDa\x12\xF7\x89a\x13\xC3V[\x93a\x12KV[\x91a\x12NV[\x90a\x14\x16V[PPa\x12}V[_\x90V[a\x13 a\x13&\x91a\x12\tV[\x91a\x12\tV[\x01\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x137WV[a\x02\xACV[\x90a\x13O\x91a\x13Ia\x13\x10V[Pa\x13\x14V[\x90V[\x90V[`\xFF\x16\x90V[a\x13oa\x13ja\x13t\x92a\x13RV[a\x02KV[a\x13UV[\x90V[a\x13\x80\x90a\x13[V[\x90RV[\x91` a\x13\xA5\x92\x94\x93a\x13\x9E`@\x82\x01\x96_\x83\x01\x90a\x13wV[\x01\x90a\x0BgV[V[a\x13\xBBa\x13\xB6a\x13\xC0\x92a\x02~V[a\x02KV[a\x12\tV[\x90V[a\x13\xCBa\x13\x10V[P\x80a\x13\xE5a\x13\xDF`\x01\x80`\xD0\x1B\x03a\x12\x14V[\x91a\x02~V[\x11a\x13\xF6Wa\x13\xF3\x90a\x13\xA7V[\x90V[`\xD0a\x14\x12_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a\x13\x84V[\x03\x90\xFD[\x90a\x14La\x14R\x93\x92a\x14'a\x13\x10V[Pa\x140a\x13\x10V[P\x80\x93a\x14Ea\x14>a\x16\xC0V[\x94\x92a\x17mV[\x90\x91a\x1CSV[\x91a\x17\xE2V[\x91\x90\x91\x90V[a\x14da\x14j\x91a\x12\tV[\x91a\x12\tV[\x90\x03\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x14|WV[a\x02\xACV[\x90a\x14\x94\x91a\x14\x8Ea\x13\x10V[Pa\x14XV[\x90V[\x90a\x14\xA1\x90a\x08\xDAV[_R` R`@_ \x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x14\xC4a\x14\xC9\x91a\x0C2V[a\x14\xADV[\x90V[a\x14\xD6\x90Ta\x14\xB8V[\x90V[a\x14\xF0a\x14\xF5\x91a\x14\xE8a\x0C\x88V[P`\ta\x14\x97V[a\x14\xCCV[\x90V[\x90a\x15\x02\x90a\x08\xDAV[_R` R`@_ \x90V[\x91\x90\x91\x80a\x15$a\x15\x1E\x85a\0\xFCV[\x91a\0\xFCV[\x14\x15\x80a\x16\xA2W[a\x156W[PPPV[\x80a\x15Qa\x15Ka\x15F_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x03a\x16\x12W[P\x81a\x15sa\x15ma\x15h_a\x02jV[a\0\xFCV[\x91a\0\xFCV[\x03a\x15\x7FW[\x80a\x151V[a\x15\xC6a\x15\xB9a\x15\xC0\x92a\x15\x95`\n\x86\x90a\x14\xF8V[\x90a\x15\xB3a\x15\xADa\x15\xA7`\x01\x93a\x13\xC3V[\x93a\x12KV[\x91a\x12NV[\x90a\x14\x16V[\x92\x90a\x12\x14V[\x91a\x12\x14V[\x91\x90\x91a\x15\xF3\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x08\xDAV[\x92a\x16\x08a\x15\xFFa\0\x89V[\x92\x83\x92\x83a\x0FXV[\x03\x90\xA2_\x80a\x15yV[a\x16Qa\x16Wa\x16Ja\x16'`\n\x85\x90a\x14\xF8V[`\x02a\x16Da\x16>a\x168\x89a\x13\xC3V[\x93a\x12KV[\x91a\x12NV[\x90a\x14\x16V[\x92\x90a\x12\x14V[\x91a\x12\x14V[\x91\x90\x91a\x16\x84\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x08\xDAV[\x92a\x16\x99a\x16\x90a\0\x89V[\x92\x83\x92\x83a\x0FXV[\x03\x90\xA2_a\x15WV[P\x81a\x16\xB6a\x16\xB0_a\x02\xE5V[\x91a\x02~V[\x11a\x15,V[_\x90V[a\x16\xC8a\x16\xBCV[Pa\x16\xD1a\x18\x11V[\x90V[T\x90V[\x90V[a\x16\xEFa\x16\xEAa\x16\xF4\x92a\x16\xD8V[a\x02KV[a\x02~V[\x90V[a\x17\x06a\x17\x0C\x91\x93\x92\x93a\x02~V[\x92a\x02~V[\x82\x03\x91\x82\x11a\x17\x17WV[a\x02\xACV[\x90V[`0\x1C\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x17<a\x17A\x91a\x17\x1FV[a\x17%V[\x90V[a\x17N\x90Ta\x170V[\x90V[a\x17ea\x17`a\x17j\x92a\x02HV[a\x02KV[a\x12\tV[\x90V[a\x17ua\x13\x10V[Pa\x17\x81_\x82\x01a\x16\xD4V[\x80a\x17\x94a\x17\x8E_a\x02\xE5V[\x91a\x02~V[\x14_\x14a\x17\xAAWPPa\x17\xA6_a\x17QV[[\x90V[a\x17\xD7_\x91a\x17\xD2a\x17\xCC\x84a\x17\xDD\x96\x01\x92a\x17\xC6`\x01a\x16\xDBV[\x90a\x16\xF7V[\x91a\x17\x1CV[a\x18&V[\x01a\x17DV[a\x17\xA7V[\x91a\x18\x06_a\x18\x0B\x94a\x17\xF3a\x13\x10V[Pa\x17\xFCa\x13\x10V[P\x01\x92\x91\x92a\x17\x1CV[a\x1A+V[\x91\x90\x91\x90V[a\x18\x19a\x16\xBCV[Pa\x18#Ca\x1B\xECV[\x90V[_R` _ \x01\x90V[T\x90V[a\x18>`@a\0\xD8V[\x90V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90a\x18V\x90a\x18AV[\x90RV[\x90a\x18d\x90a\x12\tV[\x90RV[_R` _ \x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[a\x18\x8E\x81a\x180V[\x82\x10\x15a\x18\xA8Wa\x18\xA0`\x01\x91a\x18hV[\x91\x02\x01\x90_\x90V[a\x18qV[a\x18\xB7\x90Qa\x18AV[\x90V[\x90a\x18\xCBe\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x03\x01V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x18\xE9a\x18\xE4a\x18\xEE\x92a\x18AV[a\x02KV[a\x18AV[\x90V[\x90V[\x90a\x19\ta\x19\x04a\x19\x10\x92a\x18\xD5V[a\x18\xF1V[\x82Ta\x18\xBAV[\x90UV[a\x19\x1E\x90Qa\x12\tV[\x90V[`0\x1B\x90V[\x90a\x199e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91a\x19!V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x19Wa\x19Ra\x19\\\x92a\x12\tV[a\x02KV[a\x12\tV[\x90V[\x90V[\x90a\x19wa\x19ra\x19~\x92a\x19CV[a\x19_V[\x82Ta\x19'V[\x90UV[\x90a\x19\xAC` _a\x19\xB2\x94a\x19\xA4\x82\x82\x01a\x19\x9E\x84\x88\x01a\x18\xADV[\x90a\x18\xF4V[\x01\x92\x01a\x19\x14V[\x90a\x19bV[V[\x91\x90a\x19\xC5Wa\x19\xC3\x91a\x19\x82V[V[a\x06RV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x19\xFAW\x82a\x19\xF2\x91`\x01a\x19\xF8\x95\x01\x81Ua\x18\x85V[\x90a\x19\xB4V[V[a\0\x9DV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x1A\x16a\x1A\x1B\x91a\x0C2V[a\x19\xFFV[\x90V[a\x1A(\x90Ta\x1A\nV[\x90V[\x90\x92\x91\x92a\x1A7a\x13\x10V[Pa\x1A@a\x13\x10V[Pa\x1AJ\x82a\x180V[\x80a\x1A]a\x1AW_a\x02\xE5V[\x91a\x02~V[\x11_\x14a\x1B-Wa\x1A\x83\x90a\x1A}\x84\x91a\x1Aw`\x01a\x16\xDBV[\x90a\x16\xF7V[\x90a\x18&V[\x90a\x1A\x8F_\x83\x01a\x1A\x1EV[\x92a\x1A\x9B_\x84\x01a\x17DV[\x93\x80a\x1A\xAFa\x1A\xA9\x85a\x18AV[\x91a\x18AV[\x11a\x1B\x11Wa\x1A\xC6a\x1A\xC0\x84a\x18AV[\x91a\x18AV[\x14_\x14a\x1A\xE1WPPa\x1A\xDC\x90_\x85\x91\x01a\x19bV[[\x91\x90V[a\x1B\x0C\x92Pa\x1B\x07\x86a\x1A\xFEa\x1A\xF5a\x184V[\x94_\x86\x01a\x18LV[` \x84\x01a\x18ZV[a\x19\xCAV[a\x1A\xDDV[_c% `\x1D`\xE0\x1B\x81R\x80a\x1B)`\x04\x82\x01a\x02vV[\x03\x90\xFD[Pa\x1BX\x91a\x1BS\x85a\x1BJa\x1BAa\x184V[\x94_\x86\x01a\x18LV[` \x84\x01a\x18ZV[a\x19\xCAV[a\x1Ba_a\x17QV[\x91\x90V[a\x1Bya\x1Bta\x1B~\x92a\x18AV[a\x02KV[a\x02~V[\x90V[\x90V[a\x1B\x98a\x1B\x93a\x1B\x9D\x92a\x1B\x81V[a\x02KV[a\x13UV[\x90V[a\x1B\xA9\x90a\x1B\x84V[\x90RV[\x91` a\x1B\xCE\x92\x94\x93a\x1B\xC7`@\x82\x01\x96_\x83\x01\x90a\x1B\xA0V[\x01\x90a\x0BgV[V[a\x1B\xE4a\x1B\xDFa\x1B\xE9\x92a\x02~V[a\x02KV[a\x18AV[\x90V[a\x1B\xF4a\x16\xBCV[P\x80a\x1C\x0Ea\x1C\x08e\xFF\xFF\xFF\xFF\xFF\xFFa\x1BeV[\x91a\x02~V[\x11a\x1C\x1FWa\x1C\x1C\x90a\x1B\xD0V[\x90V[`0a\x1C;_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a\x1B\xADV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`Q`\x04R`$_\xFD[\x91\x90\x91\x80`\x01\x14a\x1CrW`\x02\x03a\x1C?Wa\x1Cn\x91a\x14\x81V[\x90[V[Pa\x1C|\x91a\x13<V[\x90a\x1CpV\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x14\xD3V[a\0\x1D_5a\x03\x0CV[\x80c\x01\xFF\xC9\xA7\x14a\x03\x07W\x80c\x06\xFD\xDE\x03\x14a\x03\x02W\x80c\t^\xA7\xB3\x14a\x02\xFDW\x80c\x18\x16\r\xDD\x14a\x02\xF8W\x80c#\xB8r\xDD\x14a\x02\xF3W\x80c$\x8A\x9C\xA3\x14a\x02\xEEW\x80c//\xF1]\x14a\x02\xE9W\x80c1<\xE5g\x14a\x02\xE4W\x80c6D\xE5\x15\x14a\x02\xDFW\x80c6V\x8A\xBE\x14a\x02\xDAW\x80c:F\xB1\xA8\x14a\x02\xD5W\x80c@\xC1\x0F\x19\x14a\x02\xD0W\x80cB\x96lh\x14a\x02\xCBW\x80cK\xDD6\xCE\x14a\x02\xC6W\x80cK\xF5\xD7\xE9\x14a\x02\xC1W\x80cO\x1B\xFC\x9E\x14a\x02\xBCW\x80cX|\xDE\x1E\x14a\x02\xB7W\x80c\\\x19\xA9\\\x14a\x02\xB2W\x80co\xCF\xFFE\x14a\x02\xADW\x80cp\xA0\x821\x14a\x02\xA8W\x80cy\xCCg\x90\x14a\x02\xA3W\x80cz\x8C\xD1V\x14a\x02\x9EW\x80c~\xCE\xBE\0\x14a\x02\x99W\x80c\x83\xF1!\x1B\x14a\x02\x94W\x80c\x84&\xAD\xF2\x14a\x02\x8FW\x80c\x84L\x90&\x14a\x02\x8AW\x80c\x84\xB0\x19n\x14a\x02\x85W\x80c\x8AT%!\x14a\x02\x80W\x80c\x8D3C\xD6\x14a\x02{W\x80c\x8ES\x9E\x8C\x14a\x02vW\x80c\x90-U\xA5\x14a\x02qW\x80c\x91\xD1HT\x14a\x02lW\x80c\x91\xDD\xAD\xF4\x14a\x02gW\x80c\x95\xD8\x9BA\x14a\x02bW\x80c\x9A\xB2N\xB0\x14a\x02]W\x80c\x9B~\xF6K\x14a\x02XW\x80c\xA2\x17\xFD\xDF\x14a\x02SW\x80c\xA9\x05\x9C\xBB\x14a\x02NW\x80c\xAA\x08*\x9D\x14a\x02IW\x80c\xB0\xCA%>\x14a\x02DW\x80c\xBBMD6\x14a\x02?W\x80c\xC0*\xE7T\x14a\x02:W\x80c\xC3\xCD\xA5 \x14a\x025W\x80c\xD5\x05\xAC\xCF\x14a\x020W\x80c\xD5Gt\x1F\x14a\x02+W\x80c\xDDb\xED>\x14a\x02&Wc\xF1\x12~\xD8\x03a\0\x0EWa\x14\x9DV[a\x13\xB9V[a\x13XV[a\x13\x1EV[a\x12tV[a\x11\xB8V[a\x11\x83V[a\x11MV[a\x11\x18V[a\x10\xA6V[a\x10qV[a\x10\x01V[a\x0F\x8AV[a\x0FUV[a\x0F V[a\x0E\xBDV[a\x0E\x88V[a\x0E\x11V[a\r\xDCV[a\rxV[a\r\rV[a\x0B\xC8V[a\x0B\x93V[a\x0B:V[a\x0B\x05V[a\n\xD0V[a\n\x9CV[a\ngV[a\n2V[a\t\xD4V[a\t\x9FV[a\t*V[a\x08\xB9V[a\x08\x84V[a\x08QV[a\x07\xFFV[a\x07\xC9V[a\x07\x95V[a\x07`V[a\x07+V[a\x06\xCFV[a\x06hV[a\x05\xCCV[a\x05]V[a\x05\x05V[a\x04CV[a\x03\x94V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[c\xFF\xFF\xFF\xFF`\xE0\x1B\x16\x90V[a\x035\x81a\x03 V[\x03a\x03<WV[_\x80\xFD[\x90P5\x90a\x03M\x82a\x03,V[V[\x90` \x82\x82\x03\x12a\x03hWa\x03e\x91_\x01a\x03@V[\x90V[a\x03\x1CV[\x15\x15\x90V[a\x03{\x90a\x03mV[\x90RV[\x91\x90a\x03\x92\x90_` \x85\x01\x94\x01\x90a\x03rV[V[4a\x03\xC4Wa\x03\xC0a\x03\xAFa\x03\xAA6`\x04a\x03OV[a\x15pV[a\x03\xB7a\x03\x12V[\x91\x82\x91\x82a\x03\x7FV[\x03\x90\xF3[a\x03\x18V[_\x91\x03\x12a\x03\xD3WV[a\x03\x1CV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x04\x19a\x04\"` \x93a\x04'\x93a\x04\x10\x81a\x03\xD8V[\x93\x84\x80\x93a\x03\xDCV[\x95\x86\x91\x01a\x03\xE5V[a\x03\xF0V[\x01\x90V[a\x04@\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03\xFAV[\x90V[4a\x04sWa\x04S6`\x04a\x03\xC9V[a\x04oa\x04^a\x17\tV[a\x04fa\x03\x12V[\x91\x82\x91\x82a\x04+V[\x03\x90\xF3[a\x03\x18V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\x8C\x90a\x04xV[\x90V[a\x04\x98\x81a\x04\x83V[\x03a\x04\x9FWV[_\x80\xFD[\x90P5\x90a\x04\xB0\x82a\x04\x8FV[V[\x90V[a\x04\xBE\x81a\x04\xB2V[\x03a\x04\xC5WV[_\x80\xFD[\x90P5\x90a\x04\xD6\x82a\x04\xB5V[V[\x91\x90`@\x83\x82\x03\x12a\x05\0W\x80a\x04\xF4a\x04\xFD\x92_\x86\x01a\x04\xA3V[\x93` \x01a\x04\xC9V[\x90V[a\x03\x1CV[4a\x056Wa\x052a\x05!a\x05\x1B6`\x04a\x04\xD8V[\x90a\x17\x1FV[a\x05)a\x03\x12V[\x91\x82\x91\x82a\x03\x7FV[\x03\x90\xF3[a\x03\x18V[a\x05D\x90a\x04\xB2V[\x90RV[\x91\x90a\x05[\x90_` \x85\x01\x94\x01\x90a\x05;V[V[4a\x05\x8DWa\x05m6`\x04a\x03\xC9V[a\x05\x89a\x05xa\x17kV[a\x05\x80a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[\x90\x91``\x82\x84\x03\x12a\x05\xC7Wa\x05\xC4a\x05\xAD\x84_\x85\x01a\x04\xA3V[\x93a\x05\xBB\x81` \x86\x01a\x04\xA3V[\x93`@\x01a\x04\xC9V[\x90V[a\x03\x1CV[4a\x05\xFDWa\x05\xF9a\x05\xE8a\x05\xE26`\x04a\x05\x92V[\x91a\x17\x81V[a\x05\xF0a\x03\x12V[\x91\x82\x91\x82a\x03\x7FV[\x03\x90\xF3[a\x03\x18V[\x90V[a\x06\x0E\x81a\x06\x02V[\x03a\x06\x15WV[_\x80\xFD[\x90P5\x90a\x06&\x82a\x06\x05V[V[\x90` \x82\x82\x03\x12a\x06AWa\x06>\x91_\x01a\x06\x19V[\x90V[a\x03\x1CV[a\x06O\x90a\x06\x02V[\x90RV[\x91\x90a\x06f\x90_` \x85\x01\x94\x01\x90a\x06FV[V[4a\x06\x98Wa\x06\x94a\x06\x83a\x06~6`\x04a\x06(V[a\x17\xFAV[a\x06\x8Ba\x03\x12V[\x91\x82\x91\x82a\x06SV[\x03\x90\xF3[a\x03\x18V[\x91\x90`@\x83\x82\x03\x12a\x06\xC5W\x80a\x06\xB9a\x06\xC2\x92_\x86\x01a\x06\x19V[\x93` \x01a\x04\xA3V[\x90V[a\x03\x1CV[_\x01\x90V[4a\x06\xFEWa\x06\xE8a\x06\xE26`\x04a\x06\x9DV[\x90a\x18FV[a\x06\xF0a\x03\x12V[\x80a\x06\xFA\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[`\xFF\x16\x90V[a\x07\x12\x90a\x07\x03V[\x90RV[\x91\x90a\x07)\x90_` \x85\x01\x94\x01\x90a\x07\tV[V[4a\x07[Wa\x07;6`\x04a\x03\xC9V[a\x07Wa\x07Fa\x18uV[a\x07Na\x03\x12V[\x91\x82\x91\x82a\x07\x16V[\x03\x90\xF3[a\x03\x18V[4a\x07\x90Wa\x07p6`\x04a\x03\xC9V[a\x07\x8Ca\x07{a\x18\x8BV[a\x07\x83a\x03\x12V[\x91\x82\x91\x82a\x06SV[\x03\x90\xF3[a\x03\x18V[4a\x07\xC4Wa\x07\xAEa\x07\xA86`\x04a\x06\x9DV[\x90a\x18\x9FV[a\x07\xB6a\x03\x12V[\x80a\x07\xC0\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[4a\x07\xFAWa\x07\xF6a\x07\xE5a\x07\xDF6`\x04a\x04\xD8V[\x90a\x19PV[a\x07\xEDa\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\x08.Wa\x08\x18a\x08\x126`\x04a\x04\xD8V[\x90a\x1A\xD7V[a\x08 a\x03\x12V[\x80a\x08*\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[\x90` \x82\x82\x03\x12a\x08LWa\x08I\x91_\x01a\x04\xC9V[\x90V[a\x03\x1CV[4a\x08\x7FWa\x08ia\x08d6`\x04a\x083V[a\x1A\xE3V[a\x08qa\x03\x12V[\x80a\x08{\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[4a\x08\xB4Wa\x08\x946`\x04a\x03\xC9V[a\x08\xB0a\x08\x9Fa\x1BHV[a\x08\xA7a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\x08\xE9Wa\x08\xC96`\x04a\x03\xC9V[a\x08\xE5a\x08\xD4a\x1C\x07V[a\x08\xDCa\x03\x12V[\x91\x82\x91\x82a\x04+V[\x03\x90\xF3[a\x03\x18V[\x90V[\x90V[a\t\x08a\t\x03a\t\r\x92a\x08\xEEV[a\x08\xF1V[a\x04\xB2V[\x90V[a\t\x1Cbv\xA7\0a\x08\xF4V[\x90V[a\t'a\t\x10V[\x90V[4a\tZWa\t:6`\x04a\x03\xC9V[a\tVa\tEa\t\x1FV[a\tMa\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[\x90` \x82\x82\x03\x12a\txWa\tu\x91_\x01a\x04\xA3V[\x90V[a\x03\x1CV[a\t\x86\x90a\x04\x83V[\x90RV[\x91\x90a\t\x9D\x90_` \x85\x01\x94\x01\x90a\t}V[V[4a\t\xCFWa\t\xCBa\t\xBAa\t\xB56`\x04a\t_V[a\x1C\xA3V[a\t\xC2a\x03\x12V[\x91\x82\x91\x82a\t\x8AV[\x03\x90\xF3[a\x03\x18V[4a\n\x02Wa\t\xECa\t\xE76`\x04a\t_V[a\x1C\xC2V[a\t\xF4a\x03\x12V[\x80a\t\xFE\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[c\xFF\xFF\xFF\xFF\x16\x90V[a\n\x19\x90a\n\x07V[\x90RV[\x91\x90a\n0\x90_` \x85\x01\x94\x01\x90a\n\x10V[V[4a\nbWa\n^a\nMa\nH6`\x04a\t_V[a\x1C\xD9V[a\nUa\x03\x12V[\x91\x82\x91\x82a\n\x1DV[\x03\x90\xF3[a\x03\x18V[4a\n\x97Wa\n\x93a\n\x82a\n}6`\x04a\t_V[a\x1D\x04V[a\n\x8Aa\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\n\xCBWa\n\xB5a\n\xAF6`\x04a\x04\xD8V[\x90a\x1E9V[a\n\xBDa\x03\x12V[\x80a\n\xC7\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[4a\x0B\0Wa\n\xE06`\x04a\x03\xC9V[a\n\xFCa\n\xEBa\x1EEV[a\n\xF3a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\x0B5Wa\x0B1a\x0B a\x0B\x1B6`\x04a\t_V[a\x1E\xBDV[a\x0B(a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\x0BjWa\x0BJ6`\x04a\x03\xC9V[a\x0Bfa\x0BUa\x1E\xD2V[a\x0B]a\x03\x12V[\x91\x82\x91\x82a\x03\x7FV[\x03\x90\xF3[a\x03\x18V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0B\xC3Wa\x0B\xA36`\x04a\x03\xC9V[a\x0B\xBFa\x0B\xAEa\x0BoV[a\x0B\xB6a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\x0B\xF6Wa\x0B\xE0a\x0B\xDB6`\x04a\x083V[a \x9DV[a\x0B\xE8a\x03\x12V[\x80a\x0B\xF2\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[`\xFF`\xF8\x1B\x16\x90V[a\x0C\r\x90a\x0B\xFBV[\x90RV[Q\x90V[` \x91\x81R\x01\x90V[` \x01\x90V[a\x0C-\x90a\x04\xB2V[\x90RV[\x90a\x0C>\x81` \x93a\x0C$V[\x01\x90V[` \x01\x90V[\x90a\x0Cea\x0C_a\x0CX\x84a\x0C\x11V[\x80\x93a\x0C\x15V[\x92a\x0C\x1EV[\x90_[\x81\x81\x10a\x0CuWPPP\x90V[\x90\x91\x92a\x0C\x8Ea\x0C\x88`\x01\x92\x86Qa\x0C1V[\x94a\x0CBV[\x91\x01\x91\x90\x91a\x0ChV[\x93\x95\x91\x94a\x0C\xE9a\x0C\xDEa\x0C\xFD\x95a\x0C\xD0a\x0C\xF3\x95a\r\n\x9C\x9Aa\x0C\xC3`\xE0\x8C\x01\x92_\x8D\x01\x90a\x0C\x04V[\x8A\x82\x03` \x8C\x01Ra\x03\xFAV[\x90\x88\x82\x03`@\x8A\x01Ra\x03\xFAV[\x97``\x87\x01\x90a\x05;V[`\x80\x85\x01\x90a\t}V[`\xA0\x83\x01\x90a\x06FV[`\xC0\x81\x84\x03\x91\x01Ra\x0CHV[\x90V[4a\rDWa\r\x1D6`\x04a\x03\xC9V[a\r@a\r(a!%V[\x93a\r7\x97\x95\x97\x93\x91\x93a\x03\x12V[\x97\x88\x97\x88a\x0C\x98V[\x03\x90\xF3[a\x03\x18V[\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t\x90V[a\rua\rIV[\x90V[4a\r\xA8Wa\r\x886`\x04a\x03\xC9V[a\r\xA4a\r\x93a\rmV[a\r\x9Ba\x03\x12V[\x91\x82\x91\x82a\x06SV[\x03\x90\xF3[a\x03\x18V[\x7F\x9B\x12\xE0\xC5p~II\x15\xE5\x8B\x05d\xF1\x8A\xAA\xD9\xB7J\xC6\x9B\xFC\x81Z\x1E\xDA\xDC\x8EK\xD02\xEB\x90V[a\r\xD9a\r\xADV[\x90V[4a\x0E\x0CWa\r\xEC6`\x04a\x03\xC9V[a\x0E\x08a\r\xF7a\r\xD1V[a\r\xFFa\x03\x12V[\x91\x82\x91\x82a\x06SV[\x03\x90\xF3[a\x03\x18V[4a\x0EAWa\x0E=a\x0E,a\x0E'6`\x04a\x083V[a!\xAFV[a\x0E4a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[\x90V[a\x0E]a\x0EXa\x0Eb\x92a\x0EFV[a\x08\xF1V[a\x04\xB2V[\x90V[a\x0Ezk\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x0EIV[\x90V[a\x0E\x85a\x0EeV[\x90V[4a\x0E\xB8Wa\x0E\x986`\x04a\x03\xC9V[a\x0E\xB4a\x0E\xA3a\x0E}V[a\x0E\xABa\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\x0E\xEEWa\x0E\xEAa\x0E\xD9a\x0E\xD36`\x04a\x06\x9DV[\x90a\"\x1DV[a\x0E\xE1a\x03\x12V[\x91\x82\x91\x82a\x03\x7FV[\x03\x90\xF3[a\x03\x18V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x0F\x07\x90a\x0E\xF3V[\x90RV[\x91\x90a\x0F\x1E\x90_` \x85\x01\x94\x01\x90a\x0E\xFEV[V[4a\x0FPWa\x0F06`\x04a\x03\xC9V[a\x0FLa\x0F;a\"KV[a\x0FCa\x03\x12V[\x91\x82\x91\x82a\x0F\x0BV[\x03\x90\xF3[a\x03\x18V[4a\x0F\x85Wa\x0Fe6`\x04a\x03\xC9V[a\x0F\x81a\x0Fpa\"_V[a\x0Fxa\x03\x12V[\x91\x82\x91\x82a\x04+V[\x03\x90\xF3[a\x03\x18V[4a\x0F\xBAWa\x0F\xB6a\x0F\xA5a\x0F\xA06`\x04a\t_V[a\"uV[a\x0F\xADa\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[\x90V[a\x0F\xD6a\x0F\xD1a\x0F\xDB\x92a\x0F\xBFV[a\x08\xF1V[a\x04\xB2V[\x90V[a\x0F\xF3k\x02\xE8vi\xC3\x08sj\x04\0\0\0a\x0F\xC2V[\x90V[a\x0F\xFEa\x0F\xDEV[\x90V[4a\x101Wa\x10\x116`\x04a\x03\xC9V[a\x10-a\x10\x1Ca\x0F\xF6V[a\x10$a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[\x90V[_\x1B\x90V[a\x10Ra\x10Ma\x10W\x92a\x106V[a\x109V[a\x06\x02V[\x90V[a\x10c_a\x10>V[\x90V[a\x10na\x10ZV[\x90V[4a\x10\xA1Wa\x10\x816`\x04a\x03\xC9V[a\x10\x9Da\x10\x8Ca\x10fV[a\x10\x94a\x03\x12V[\x91\x82\x91\x82a\x06SV[\x03\x90\xF3[a\x03\x18V[4a\x10\xD7Wa\x10\xD3a\x10\xC2a\x10\xBC6`\x04a\x04\xD8V[\x90a\"\xA4V[a\x10\xCAa\x03\x12V[\x91\x82\x91\x82a\x03\x7FV[\x03\x90\xF3[a\x03\x18V[\x1C\x90V[\x90V[a\x10\xF3\x90`\x08a\x10\xF8\x93\x02a\x10\xDCV[a\x10\xE0V[\x90V[\x90a\x11\x06\x91Ta\x10\xE3V[\x90V[a\x11\x15`\x0C_\x90a\x10\xFBV[\x90V[4a\x11HWa\x11(6`\x04a\x03\xC9V[a\x11Da\x113a\x11\tV[a\x11;a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\x11~Wa\x11za\x11ia\x11c6`\x04a\x04\xD8V[\x90a\"\xC6V[a\x11qa\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\x11\xB3Wa\x11\xAFa\x11\x9Ea\x11\x996`\x04a\t_V[a\"\xDCV[a\x11\xA6a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\x11\xE8Wa\x11\xC86`\x04a\x03\xC9V[a\x11\xE4a\x11\xD3a\"\xF1V[a\x11\xDBa\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[a\x11\xF6\x81a\x07\x03V[\x03a\x11\xFDWV[_\x80\xFD[\x90P5\x90a\x12\x0E\x82a\x11\xEDV[V[\x90\x91`\xC0\x82\x84\x03\x12a\x12oWa\x12(\x83_\x84\x01a\x04\xA3V[\x92a\x126\x81` \x85\x01a\x04\xC9V[\x92a\x12D\x82`@\x83\x01a\x04\xC9V[\x92a\x12la\x12U\x84``\x85\x01a\x12\x01V[\x93a\x12c\x81`\x80\x86\x01a\x06\x19V[\x93`\xA0\x01a\x06\x19V[\x90V[a\x03\x1CV[4a\x12\xA9Wa\x12\x93a\x12\x876`\x04a\x12\x10V[\x94\x93\x90\x93\x92\x91\x92a#qV[a\x12\x9Ba\x03\x12V[\x80a\x12\xA5\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[`\xE0\x81\x83\x03\x12a\x13\x19Wa\x12\xC4\x82_\x83\x01a\x04\xA3V[\x92a\x12\xD2\x83` \x84\x01a\x04\xA3V[\x92a\x12\xE0\x81`@\x85\x01a\x04\xC9V[\x92a\x12\xEE\x82``\x83\x01a\x04\xC9V[\x92a\x13\x16a\x12\xFF\x84`\x80\x85\x01a\x12\x01V[\x93a\x13\r\x81`\xA0\x86\x01a\x06\x19V[\x93`\xC0\x01a\x06\x19V[\x90V[a\x03\x1CV[4a\x13SWa\x13=a\x1316`\x04a\x12\xAEV[\x95\x94\x90\x94\x93\x91\x93a$\xC5V[a\x13Ea\x03\x12V[\x80a\x13O\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[4a\x13\x87Wa\x13qa\x13k6`\x04a\x06\x9DV[\x90a%\xE3V[a\x13ya\x03\x12V[\x80a\x13\x83\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[\x91\x90`@\x83\x82\x03\x12a\x13\xB4W\x80a\x13\xA8a\x13\xB1\x92_\x86\x01a\x04\xA3V[\x93` \x01a\x04\xA3V[\x90V[a\x03\x1CV[4a\x13\xEAWa\x13\xE6a\x13\xD5a\x13\xCF6`\x04a\x13\x8CV[\x90a&\x05V[a\x13\xDDa\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[a\x13\xF8\x81a\n\x07V[\x03a\x13\xFFWV[_\x80\xFD[\x90P5\x90a\x14\x10\x82a\x13\xEFV[V[\x91\x90`@\x83\x82\x03\x12a\x14:W\x80a\x14.a\x147\x92_\x86\x01a\x04\xA3V[\x93` \x01a\x14\x03V[\x90V[a\x03\x1CV[a\x14H\x90a\x0E\xF3V[\x90RV[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x14`\x90a\x14LV[\x90RV[\x90` \x80a\x14\x86\x93a\x14|_\x82\x01Q_\x86\x01\x90a\x14?V[\x01Q\x91\x01\x90a\x14WV[V[\x91\x90a\x14\x9B\x90_`@\x85\x01\x94\x01\x90a\x14dV[V[4a\x14\xCEWa\x14\xCAa\x14\xB9a\x14\xB36`\x04a\x14\x12V[\x90a&sV[a\x14\xC1a\x03\x12V[\x91\x82\x91\x82a\x14\x88V[\x03\x90\xF3[a\x03\x18V[_\x80\xFD[_\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x14\xFBa\x15\x01\x91a\x14LV[\x91a\x14LV[\x01\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x15\x12WV[a\x14\xDBV[\x90a\x15*\x91a\x15$a\x14\xD7V[Pa\x14\xEFV[\x90V[a\x159a\x15?\x91a\x14LV[\x91a\x14LV[\x90\x03\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x15QWV[a\x14\xDBV[\x90a\x15i\x91a\x15ca\x14\xD7V[Pa\x15-V[\x90V[_\x90V[a\x15xa\x15lV[P\x80a\x15\x93a\x15\x8Dcye\xDB\x0B`\xE0\x1Ba\x03 V[\x91a\x03 V[\x14\x90\x81\x15a\x15\xA0W[P\x90V[a\x15\xAA\x91Pa&\x89V[_a\x15\x9CV[``\x90V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x01`\x02\x83\x04\x92\x16\x80\x15a\x15\xE9W[` \x83\x10\x14a\x15\xE4WV[a\x15\xB5V[\x91`\x7F\x16\x91a\x15\xD9V[` \x91\x81R\x01\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a\x16\x1Fa\x16\x18\x83a\x15\xC9V[\x80\x94a\x15\xF3V[\x91`\x01\x81\x16\x90\x81_\x14a\x16vWP`\x01\x14a\x16:W[PPPV[a\x16G\x91\x92\x93\x94Pa\x15\xFCV[\x91_\x92[\x81\x84\x10a\x16^WPP\x01\x90_\x80\x80a\x165V[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a\x16KV[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a\x165V[\x90a\x16\x9B\x91a\x16\x05V[\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x16\xBC\x90a\x03\xF0V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x16\xD6W`@RV[a\x16\x9EV[\x90a\x16\xFBa\x16\xF4\x92a\x16\xEBa\x03\x12V[\x93\x84\x80\x92a\x16\x91V[\x03\x83a\x16\xB2V[V[a\x17\x06\x90a\x16\xDBV[\x90V[a\x17\x11a\x15\xB0V[Pa\x17\x1C`\x03a\x16\xFDV[\x90V[a\x17<\x91a\x17+a\x15lV[Pa\x174a&\xAFV[\x91\x90\x91a&\xBCV[`\x01\x90V[_\x90V[_\x1C\x90V[a\x17Va\x17[\x91a\x17EV[a\x10\xE0V[\x90V[a\x17h\x90Ta\x17JV[\x90V[a\x17sa\x17AV[Pa\x17~`\x02a\x17^V[\x90V[\x91a\x17\xAB\x92a\x17\x8Ea\x15lV[Pa\x17\xA3a\x17\x9Aa&\xAFV[\x82\x90\x84\x91a'\x0CV[\x91\x90\x91a'\x98V[`\x01\x90V[_\x90V[a\x17\xBD\x90a\x06\x02V[\x90V[\x90a\x17\xCA\x90a\x17\xB4V[_R` R`@_ \x90V[\x90V[a\x17\xE5a\x17\xEA\x91a\x17EV[a\x17\xD6V[\x90V[a\x17\xF7\x90Ta\x17\xD9V[\x90V[`\x01a\x18\x13a\x18\x19\x92a\x18\x0Ba\x17\xB0V[P`\x05a\x17\xC0V[\x01a\x17\xEDV[\x90V[\x90a\x187\x91a\x182a\x18-\x82a\x17\xFAV[a(5V[a\x189V[V[\x90a\x18C\x91a(\x8EV[PV[\x90a\x18P\x91a\x18\x1CV[V[_\x90V[\x90V[a\x18ma\x18ha\x18r\x92a\x18VV[a\x08\xF1V[a\x07\x03V[\x90V[a\x18}a\x18RV[Pa\x18\x88`\x12a\x18YV[\x90V[a\x18\x93a\x17\xB0V[Pa\x18\x9Ca):V[\x90V[\x90\x80a\x18\xBAa\x18\xB4a\x18\xAFa&\xAFV[a\x04\x83V[\x91a\x04\x83V[\x03a\x18\xCBWa\x18\xC8\x91a)\xF4V[PV[_c3K\xD9\x19`\xE1\x1B\x81R\x80a\x18\xE3`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[a\x18\xFBa\x18\xF6a\x19\0\x92a\x04xV[a\x08\xF1V[a\x04xV[\x90V[a\x19\x0C\x90a\x18\xE7V[\x90V[a\x19\x18\x90a\x19\x03V[\x90V[\x90a\x19%\x90a\x19\x0FV[_R` R`@_ \x90V[\x90V[a\x19Ha\x19Ca\x19M\x92a\x14LV[a\x08\xF1V[a\x04\xB2V[\x90V[a\x19\x87\x91a\x19|a\x19va\x19qa\x19\x82\x94a\x19ia\x17AV[P`\na\x19\x1BV[a\x191V[\x91a*\xD5V[\x90a+\xEAV[a\x194V[\x90V[\x90a\x19\xA4\x91a\x19\x9Fa\x19\x9Aa\r\xADV[a(5V[a\x1A\x0FV[V[a\x19\xBAa\x19\xB5a\x19\xBF\x92a\x106V[a\x08\xF1V[a\x04xV[\x90V[a\x19\xCB\x90a\x19\xA6V[\x90V[a\x19\xE2a\x19\xDDa\x19\xE7\x92a\x106V[a\x08\xF1V[a\x04\xB2V[\x90V[a\x19\xF9a\x19\xFF\x91\x93\x92\x93a\x04\xB2V[\x92a\x04\xB2V[\x82\x01\x80\x92\x11a\x1A\nWV[a\x14\xDBV[\x90\x81a\x1A+a\x1A%a\x1A _a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14a\x1A\xBBW\x80a\x1ACa\x1A=_a\x19\xCEV[\x91a\x04\xB2V[\x14a\x1A\x9FWa\x1AZa\x1ASa\x17kV[\x82\x90a\x19\xEAV[a\x1Asa\x1Ama\x1Aha\x0EeV[a\x04\xB2V[\x91a\x04\xB2V[\x11a\x1A\x83Wa\x1A\x81\x91a-\x11V[V[_c\x17~?\xC3`\xE0\x1B\x81R\x80a\x1A\x9B`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1A\xB7`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x1A\xD3`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[\x90a\x1A\xE1\x91a\x19\x8AV[V[\x80a\x1A\xF6a\x1A\xF0_a\x19\xCEV[\x91a\x04\xB2V[\x14a\x1B\x07Wa\x1B\x05\x903a-oV[V[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1B\x1F`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[a\x1B2a\x1B8\x91\x93\x92\x93a\x04\xB2V[\x92a\x04\xB2V[\x82\x03\x91\x82\x11a\x1BCWV[a\x14\xDBV[a\x1BPa\x17AV[Pa\x1Bja\x1B\\a\x0EeV[a\x1Bda\x17kV[\x90a\x1B#V[\x90V[\x90a\x1B\x80a\x1Bya\x03\x12V[\x92\x83a\x16\xB2V[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1B\xA0Wa\x1B\x9C` \x91a\x03\xF0V[\x01\x90V[a\x16\x9EV[\x90a\x1B\xB7a\x1B\xB2\x83a\x1B\x82V[a\x1BmV[\x91\x82RV[_\x7Fmode=blocknumber&from=default\0\0\0\x91\x01RV[a\x1B\xED`\x1Da\x1B\xA5V[\x90a\x1B\xFA` \x83\x01a\x1B\xBCV[V[a\x1C\x04a\x1B\xE3V[\x90V[a\x1C\x0Fa\x15\xB0V[Pa\x1C\x18a\"KV[a\x1C1a\x1C+a\x1C&a-\xCEV[a\x0E\xF3V[\x91a\x0E\xF3V[\x03a\x1CAWa\x1C>a\x1B\xFCV[\x90V[_c\x01\xBF\xC1\xC5`\xE6\x1B\x81R\x80a\x1CY`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[_\x90V[\x90a\x1Ck\x90a\x19\x0FV[_R` R`@_ \x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x1C\x8Ea\x1C\x93\x91a\x17EV[a\x1CwV[\x90V[a\x1C\xA0\x90Ta\x1C\x82V[\x90V[a\x1C\xBAa\x1C\xBF\x91a\x1C\xB2a\x1C]V[P`\ta\x1CaV[a\x1C\x96V[\x90V[a\x1C\xD3\x90a\x1C\xCEa&\xAFV[a.!V[V[_\x90V[a\x1C\xEB\x90a\x1C\xE5a\x1C\xD5V[Pa.\xACV[\x90V[\x90a\x1C\xF8\x90a\x19\x0FV[_R` R`@_ \x90V[a\x1D\x1Aa\x1D\x1F\x91a\x1D\x13a\x17AV[P_a\x1C\xEEV[a\x17^V[\x90V[\x90a\x1D<\x91a\x1D7a\x1D2a\rIV[a(5V[a\x1D>V[V[\x80a\x1DYa\x1DSa\x1DN_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14a\x1E\x1DW\x81a\x1Dqa\x1Dk_a\x19\xCEV[\x91a\x04\xB2V[\x14a\x1E\x01Wa\x1D\x87a\x1D\x81a\x1E\xD2V[\x15a\x03mV[a\x1D\xE5Wa\x1D\x96\x81\x83\x90a-oV[3\x90a\x1D\xE0a\x1D\xCEa\x1D\xC8\x7F\xBE\xF4\xF8\x1C\x18\x14\xC6A\xED\xE8^\xBA\xAC\xF1\x9D\x04\x8B,[U\x98\n\xDF\xA6\xEF\x0F\x95le\x135\xA2\x93a\x19\x0FV[\x93a\x19\x0FV[\x93a\x1D\xD7a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xA3V[_c\xB8\xB5\xCA-`\xE0\x1B\x81R\x80a\x1D\xFD`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1E\x19`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x1E5`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[\x90a\x1EC\x91a\x1D\"V[V[a\x1EMa\x17AV[Pa\x1EX`\x0Ca\x17^V[a\x1Eja\x1Ed_a\x19\xCEV[\x91a\x04\xB2V[\x14\x80\x15a\x1E\x99W[a\x1E\x8DWa\x1E\x8Aa\x1E\x83`\x0Ca\x17^V[B\x90a\x1B#V[\x90V[a\x1E\x96_a\x19\xCEV[\x90V[PBa\x1E\xB6a\x1E\xB0a\x1E\xAB`\x0Ca\x17^V[a\x04\xB2V[\x91a\x04\xB2V[\x10\x15a\x1ErV[a\x1E\xCF\x90a\x1E\xC9a\x17AV[Pa.\xDBV[\x90V[a\x1E\xDAa\x15lV[Pa\x1E\xE5`\x0Ca\x17^V[a\x1E\xF7a\x1E\xF1_a\x19\xCEV[\x91a\x04\xB2V[\x14\x15\x80a\x1F\x02W[\x90V[PBa\x1F\x1Fa\x1F\x19a\x1F\x14`\x0Ca\x17^V[a\x04\xB2V[\x91a\x04\xB2V[\x10a\x1E\xFFV[a\x1F>\x90a\x1F9a\x1F4a\x10ZV[a(5V[a\x1F\xB8V[V[\x90a\x1FL_\x19\x91a\x109V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x1Fja\x1Fea\x1Fo\x92a\x04\xB2V[a\x08\xF1V[a\x04\xB2V[\x90V[\x90V[\x90a\x1F\x8Aa\x1F\x85a\x1F\x91\x92a\x1FVV[a\x1FrV[\x82Ta\x1F@V[\x90UV[\x91` a\x1F\xB6\x92\x94\x93a\x1F\xAF`@\x82\x01\x96_\x83\x01\x90a\x05;V[\x01\x90a\x05;V[V[\x80a\x1F\xCBa\x1F\xC5Ba\x04\xB2V[\x91a\x04\xB2V[\x11\x15a \x81W\x80a \x04a\x1F\xFE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xB2V[\x91a\x04\xB2V[\x11a eWa \x13`\x0Ca\x17^V[a \x1E\x82`\x0Ca\x1FuV[\x903\x90a K\x7F\xDDh\x96\xDC\xF1\xD4\xB3\x11\xCC\xA8}\xD1\x9B\xBB\xA2\xEA\x9C\xE2\xF8g\xC1V\x88x\xA0C\x8Af\xA1\xAF\xEE\xEC\x92a\x19\x0FV[\x92a `a Wa\x03\x12V[\x92\x83\x92\x83a\x1F\x95V[\x03\x90\xA2V[_c\xEFi\xAFe`\xE0\x1B\x81R\x80a }`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[_c\xA5e\x83S`\xE0\x1B\x81R\x80a \x99`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[a \xA6\x90a\x1F%V[V[_\x90V[``\x90V[a \xBA\x90a\x19\x03V[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a \xD5W` \x80\x91\x02\x01\x90V[a\x16\x9EV[\x90a \xECa \xE7\x83a \xBDV[a\x1BmV[\x91\x82RV[6\x907V[\x90a!\x1Ba!\x03\x83a \xDAV[\x92` \x80a!\x11\x86\x93a \xBDV[\x92\x01\x91\x03\x90a \xF1V[V[`\x0F`\xF8\x1B\x90V[a!-a \xA8V[Pa!6a\x15\xB0V[Pa!?a\x15\xB0V[Pa!Ha\x17AV[Pa!Qa\x1C]V[Pa!Za\x17\xB0V[Pa!ca \xACV[Pa!la.\xF3V[\x90a!ua/3V[\x90F\x90a!\x810a \xB1V[\x90a!\x8B_a\x10>V[\x90a!\x9Da!\x98_a\x19\xCEV[a \xF6V[\x90a!\xA6a!\x1DV[\x96\x95\x94\x93\x92\x91\x90V[a!\xD8a!\xDD\x91a!\xBEa\x17AV[Pa!\xD2a!\xCC`\x0Ba\x191V[\x91a*\xD5V[\x90a+\xEAV[a\x194V[\x90V[\x90a!\xEA\x90a\x19\x0FV[_R` R`@_ \x90V[`\xFF\x16\x90V[a\"\x08a\"\r\x91a\x17EV[a!\xF6V[\x90V[a\"\x1A\x90Ta!\xFCV[\x90V[a\"D\x91_a\"9a\"?\x93a\"1a\x15lV[P`\x05a\x17\xC0V[\x01a!\xE0V[a\"\x10V[\x90V[_\x90V[a\"Sa\"GV[Pa\"\\a-\xCEV[\x90V[a\"ga\x15\xB0V[Pa\"r`\x04a\x16\xFDV[\x90V[a\"\x9Ca\"\x97a\"\x92a\"\xA1\x93a\"\x8Aa\x17AV[P`\na\x19\x1BV[a\x191V[a/sV[a\x194V[\x90V[a\"\xC1\x91a\"\xB0a\x15lV[Pa\"\xB9a&\xAFV[\x91\x90\x91a'\x98V[`\x01\x90V[\x90a\"\xD9\x91a\"\xD3a\x17AV[Pa\x19PV[\x90V[a\"\xEE\x90a\"\xE8a\x17AV[Pa\"uV[\x90V[a\"\xF9a\x17AV[Pa#\x02a\x17kV[\x90V[\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF\x90V[a#^a#e\x94a#T``\x94\x98\x97\x95a#J`\x80\x86\x01\x9A_\x87\x01\x90a\x06FV[` \x85\x01\x90a\t}V[`@\x83\x01\x90a\x05;V[\x01\x90a\x05;V[V[` \x01\x90V[Q\x90V[\x93\x95\x94\x90\x92\x91\x95Ba#\x8Ba#\x85\x89a\x04\xB2V[\x91a\x04\xB2V[\x11a$\x04W\x91a#\xF6\x91a#\xFD\x93a#\xEDa$\x02\x98\x99a#\xD5a#\xACa#\x05V[a#\xC6\x8B\x93\x8Ba#\xBAa\x03\x12V[\x95\x86\x94` \x86\x01a#)V[` \x82\x01\x81\x03\x82R\x03\x82a\x16\xB2V[a#\xE7a#\xE1\x82a#mV[\x91a#gV[ a/\xE8V[\x92\x90\x91\x92a0\x05V[\x91\x82a0OV[a.!V[V[a$\x1F\x87_\x91\x82\x91c#A\xD7\x87`\xE1\x1B\x83R`\x04\x83\x01a\x05HV[\x03\x90\xFD[\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x90V[\x91\x94a$\x8Fa$\x99\x92\x98\x97\x95a$\x85`\xA0\x96a${a$\xA0\x9Aa$q`\xC0\x8A\x01\x9E_\x8B\x01\x90a\x06FV[` \x89\x01\x90a\t}V[`@\x87\x01\x90a\t}V[``\x85\x01\x90a\x05;V[`\x80\x83\x01\x90a\x05;V[\x01\x90a\x05;V[V[\x91` a$\xC3\x92\x94\x93a$\xBC`@\x82\x01\x96_\x83\x01\x90a\t}V[\x01\x90a\t}V[V[\x96\x95\x91\x93\x92\x94\x90\x94Ba$\xE0a$\xDA\x83a\x04\xB2V[\x91a\x04\xB2V[\x11a%\x9AW\x90a%Ia%R\x94\x93\x92a%1a$\xFAa$#V[a%\"\x8C\x80\x94\x8C\x91a%\x0C\x8D\x91a0\xA1V[\x91\x92a%\x16a\x03\x12V[\x97\x88\x96` \x88\x01a$GV[` \x82\x01\x81\x03\x82R\x03\x82a\x16\xB2V[a%Ca%=\x82a#mV[\x91a#gV[ a/\xE8V[\x92\x90\x91\x92a0\x05V[\x80a%ea%_\x87a\x04\x83V[\x91a\x04\x83V[\x03a%zWPa%x\x92\x93\x91\x90\x91a&\xBCV[V[\x84\x90a%\x96_\x92\x83\x92c%\xC0\x07#`\xE1\x1B\x84R`\x04\x84\x01a$\xA2V[\x03\x90\xFD[a%\xB5\x90_\x91\x82\x91c1<\x89\x81`\xE1\x1B\x83R`\x04\x83\x01a\x05HV[\x03\x90\xFD[\x90a%\xD4\x91a%\xCFa%\xCA\x82a\x17\xFAV[a(5V[a%\xD6V[V[\x90a%\xE0\x91a)\xF4V[PV[\x90a%\xED\x91a%\xB9V[V[\x90a%\xF9\x90a\x19\x0FV[_R` R`@_ \x90V[a&*\x91a& a&%\x92a&\x18a\x17AV[P`\x01a%\xEFV[a\x1C\xEEV[a\x17^V[\x90V[a&7`@a\x1BmV[\x90V[_\x90V[_\x90V[a&Ja&-V[\x90` \x80\x83a&Wa&:V[\x81R\x01a&ba&>V[\x81RPPV[a&pa&BV[\x90V[\x90a&\x86\x91a&\x80a&hV[Pa0\xD4V[\x90V[a&\x91a\x15lV[Pa&\xABa&\xA5c\x01\xFF\xC9\xA7`\xE0\x1Ba\x03 V[\x91a\x03 V[\x14\x90V[a&\xB7a\x1C]V[P3\x90V[\x91a&\xCA\x92\x91`\x01\x92a0\xFCV[V[`@\x90a&\xF5a&\xFC\x94\x96\x95\x93\x96a&\xEB``\x84\x01\x98_\x85\x01\x90a\t}V[` \x83\x01\x90a\x05;V[\x01\x90a\x05;V[V[\x90a'\t\x91\x03a\x04\xB2V[\x90V[\x92\x91\x92a'\x1A\x81\x83\x90a&\x05V[\x90\x81a'/a')_\x19a\x04\xB2V[\x91a\x04\xB2V[\x10a'<W[PPP\x90PV[\x81a'Oa'I\x87a\x04\xB2V[\x91a\x04\xB2V[\x10a'uWa'l\x93\x94a'd\x91\x93\x92a&\xFEV[\x90_\x92a0\xFCV[\x80_\x80\x80a'5V[Pa'\x94\x84\x92\x91\x92_\x93\x84\x93c}\xC7\xA0\xD9`\xE1\x1B\x85R`\x04\x85\x01a&\xCCV[\x03\x90\xFD[\x91\x82a'\xB4a'\xAEa'\xA9_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14a(\x0EW\x81a'\xD4a'\xCEa'\xC9_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14a'\xE7Wa'\xE5\x92\x91\x90\x91a2\x0BV[V[a(\na'\xF3_a\x19\xC2V[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\t\x8AV[\x03\x90\xFD[a(1a(\x1A_a\x19\xC2V[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\t\x8AV[\x03\x90\xFD[a(G\x90a(Aa&\xAFV[\x90a2\xD8V[V[\x90a(U`\xFF\x91a\x109V[\x91\x81\x19\x16\x91\x16\x17\x90V[a(h\x90a\x03mV[\x90V[\x90V[\x90a(\x83a(~a(\x8A\x92a(_V[a(kV[\x82Ta(IV[\x90UV[a(\x96a\x15lV[Pa(\xABa(\xA5\x82\x84\x90a\"\x1DV[\x15a\x03mV[_\x14a)4Wa(\xD3`\x01a(\xCE_a(\xC6`\x05\x86\x90a\x17\xC0V[\x01\x85\x90a!\xE0V[a(nV[\x90a(\xDCa&\xAFV[\x90a)\x19a)\x13a)\r\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x95a\x17\xB4V[\x92a\x19\x0FV[\x92a\x19\x0FV[\x92a)\"a\x03\x12V[\x80a),\x81a\x06\xCAV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a)Ba\x17\xB0V[Pa)L0a \xB1V[a)~a)x\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\x83V[\x91a\x04\x83V[\x14\x80a)\xBAW[_\x14a)\xAFW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a)\xB7a3\x84V[\x90V[PFa)\xEEa)\xE8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xB2V[\x91a\x04\xB2V[\x14a)\x85V[a)\xFCa\x15lV[Pa*\x08\x81\x83\x90a\"\x1DV[_\x14a*\x90Wa*/_a**_a*\"`\x05\x86\x90a\x17\xC0V[\x01\x85\x90a!\xE0V[a(nV[\x90a*8a&\xAFV[\x90a*ua*oa*i\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x95a\x17\xB4V[\x92a\x19\x0FV[\x92a\x19\x0FV[\x92a*~a\x03\x12V[\x80a*\x88\x81a\x06\xCAV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a*\xAAa*\xA5a*\xAF\x92a\x0E\xF3V[a\x08\xF1V[a\x04\xB2V[\x90V[\x91` a*\xD3\x92\x94\x93a*\xCC`@\x82\x01\x96_\x83\x01\x90a\x05;V[\x01\x90a\x0E\xFEV[V[a*\xDDa\"GV[Pa*\xE6a\"KV[\x81a*\xF9a*\xF3\x83a*\x96V[\x91a\x04\xB2V[\x10\x15a+\x0CWPa+\t\x90a4\x8DV[\x90V[\x90a+'_\x92\x83\x92cvi\xFC\x0F`\xE1\x1B\x84R`\x04\x84\x01a*\xB2V[\x03\x90\xFD[T\x90V[\x90V[a+Fa+Aa+K\x92a+/V[a\x08\xF1V[a\x04\xB2V[\x90V[\x90V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a+ha+m\x91a\x17EV[a+QV[\x90V[a+z\x90Ta+\\V[\x90V[\x90V[a+\x94a+\x8Fa+\x99\x92a+}V[a\x08\xF1V[a\x04\xB2V[\x90V[`0\x1C\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a+\xB9a+\xBE\x91a+\x9CV[a+\xA2V[\x90V[a+\xCB\x90Ta+\xADV[\x90V[a+\xE2a+\xDDa+\xE7\x92a\x106V[a\x08\xF1V[a\x14LV[\x90V[\x90a,>\x90a+\xF7a\x14\xD7V[Pa,\x03_\x84\x01a++V[a,\x0C_a\x19\xCEV[\x90\x80\x80a,\"a,\x1C`\x05a+2V[\x91a\x04\xB2V[\x11a,\x9FW[P\x90a,9_\x86\x01\x93\x91\x92\x93a+NV[a:\xE0V[\x80a,Qa,K_a\x19\xCEV[\x91a\x04\xB2V[\x14_\x14a,gWPPa,c_a+\xCEV[[\x90V[a,\x94_\x91a,\x8Fa,\x89\x84a,\x9A\x96\x01\x92a,\x83`\x01a+\x80V[\x90a\x1B#V[\x91a+NV[a:\xD6V[\x01a+\xC1V[a,dV[\x80a,\xADa,\xB3\x92\x91a7kV[\x90a\x1B#V[\x90\x83a,\xE5a,\xDFa,\xDA_a,\xD4\x81\x8C\x01a,\xCF\x89\x91a+NV[a:\xD6V[\x01a+pV[a\x0E\xF3V[\x91a\x0E\xF3V[\x10_\x14a,\xF6WP\x90[\x90_a,(V[\x91Pa-\x0C\x90a-\x06`\x01a+\x80V[\x90a\x19\xEAV[a,\xEFV[\x80a-,a-&a-!_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14a-HWa-F\x91a->_a\x19\xC2V[\x91\x90\x91a2\x0BV[V[a-ka-T_a\x19\xC2V[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\t\x8AV[\x03\x90\xFD[\x90\x81a-\x8Ba-\x85a-\x80_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14a-\xA7Wa-\xA5\x91\x90a-\x9E_a\x19\xC2V[\x90\x91a2\x0BV[V[a-\xCAa-\xB3_a\x19\xC2V[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\t\x8AV[\x03\x90\xFD[a-\xD6a\"GV[Pa-\xE0Ca4\x8DV[\x90V[\x90a-\xF4`\x01\x80`\xA0\x1B\x03\x91a\x109V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a.\x16a.\x11a.\x1D\x92a\x19\x0FV[a-\xFEV[\x82Ta-\xE3V[\x90UV[\x90a.\xAA\x91a.\xA4a.2\x82a\x1C\xA3V[a.G\x84a.B`\t\x86\x90a\x1CaV[a.\x01V[\x82\x81\x85\x90a.\x87a.\x81a.{\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x95a\x19\x0FV[\x92a\x19\x0FV[\x92a\x19\x0FV[\x92a.\x90a\x03\x12V[\x80a.\x9A\x81a\x06\xCAV[\x03\x90\xA4\x92\x91a;oV[\x91a;\x87V[V[a.\xD3a.\xCEa.\xC9a.\xD8\x93a.\xC1a\x1C\xD5V[P`\na\x19\x1BV[a\x191V[a=5V[a=\xB4V[\x90V[a.\xED\x90a.\xE7a\x17AV[Pa>\x05V[\x90V[\x90V[a.\xFBa\x15\xB0V[Pa/0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a/*`\x06a.\xF0V[\x90a? V[\x90V[a/;a\x15\xB0V[Pa/p\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a/j`\x07a.\xF0V[\x90a? V[\x90V[a/{a\x14\xD7V[Pa/\x87_\x82\x01a++V[\x80a/\x9Aa/\x94_a\x19\xCEV[\x91a\x04\xB2V[\x14_\x14a/\xB0WPPa/\xAC_a+\xCEV[[\x90V[a/\xDD_\x91a/\xD8a/\xD2\x84a/\xE3\x96\x01\x92a/\xCC`\x01a+\x80V[\x90a\x1B#V[\x91a+NV[a:\xD6V[\x01a+\xC1V[a/\xADV[a0\x02\x90a/\xF4a\x17\xB0V[Pa/\xFDa):V[a?nV[\x90V[\x92a0 \x92a0)\x94a0\x16a\x1C]V[P\x92\x90\x91\x92a@4V[\x90\x92\x91\x92aA_V[\x90V[\x91` a0M\x92\x94\x93a0F`@\x82\x01\x96_\x83\x01\x90a\t}V[\x01\x90a\x05;V[V[a0X\x81a0\xA1V[\x91a0ka0e\x84a\x04\xB2V[\x91a\x04\xB2V[\x03a0tWPPV[a0\x8E_\x92\x83\x92c\x01\xD4\xB6#`\xE6\x1B\x84R`\x04\x84\x01a0,V[\x03\x90\xFD[`\x01a0\x9E\x91\x01a\x04\xB2V[\x90V[a0\xB5\x90a0\xADa\x17AV[P`\x08a\x1C\xEEV[a0\xD1a0\xC1\x82a\x17^V[\x91a0\xCB\x83a0\x92V[\x90a\x1FuV[\x90V[\x90a0\xF4a0\xEFa0\xF9\x93a0\xE7a&hV[P`\na\x19\x1BV[a\x191V[aB\xD5V[\x90V[\x90\x92\x81a1\x19a1\x13a1\x0E_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14a1\xE4W\x83a19a13a1._a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14a1\xBDWa1]\x83a1Xa1Q`\x01\x86\x90a%\xEFV[\x87\x90a\x1C\xEEV[a\x1FuV[a1gW[PPPV[\x91\x90\x91a1\xB2a1\xA0a1\x9A\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x93a\x19\x0FV[\x93a\x19\x0FV[\x93a1\xA9a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xA3_\x80\x80a1bV[a1\xE0a1\xC9_a\x19\xC2V[_\x91\x82\x91cJ\x14\x06\xB1`\xE1\x1B\x83R`\x04\x83\x01a\t\x8AV[\x03\x90\xFD[a2\x07a1\xF0_a\x19\xC2V[_\x91\x82\x91c\xE6\x02\xDF\x05`\xE0\x1B\x83R`\x04\x83\x01a\t\x8AV[\x03\x90\xFD[\x91\x82a2'a2!a2\x1C_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14\x15\x80a2\x92W[a2BW[a2@\x92\x91\x90\x91aB\xF6V[V[a2Ja\x1E\xD2V[\x80a2qW[\x15a24W_c6\xE2x\xFD`\xE2\x1B\x81R\x80a2m`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[Pa2\x8Da2\x87a2\x80a\rIV[3\x90a\"\x1DV[\x15a\x03mV[a2PV[P\x81a2\xAEa2\xA8a2\xA3_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14\x15a2/V[\x91` a2\xD6\x92\x94\x93a2\xCF`@\x82\x01\x96_\x83\x01\x90a\t}V[\x01\x90a\x06FV[V[\x90a2\xEDa2\xE7\x83\x83\x90a\"\x1DV[\x15a\x03mV[a2\xF5WPPV[a3\x0F_\x92\x83\x92c\xE2Q}?`\xE0\x1B\x84R`\x04\x84\x01a2\xB5V[\x03\x90\xFD[\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x90V[\x90\x95\x94\x92a3\x82\x94a3qa3{\x92a3g`\x80\x96a3]`\xA0\x88\x01\x9C_\x89\x01\x90a\x06FV[` \x87\x01\x90a\x06FV[`@\x85\x01\x90a\x06FV[``\x83\x01\x90a\x05;V[\x01\x90a\t}V[V[a3\x8Ca\x17\xB0V[Pa3\x95a3\x13V[a4\x0C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a3\xFD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Fa3\xE80a \xB1V[\x91a3\xF1a\x03\x12V[\x96\x87\x95` \x87\x01a37V[` \x82\x01\x81\x03\x82R\x03\x82a\x16\xB2V[a4\x1Ea4\x18\x82a#mV[\x91a#gV[ \x90V[\x90V[a49a44a4>\x92a4\"V[a\x08\xF1V[a\x07\x03V[\x90V[a4J\x90a4%V[\x90RV[\x91` a4o\x92\x94\x93a4h`@\x82\x01\x96_\x83\x01\x90a4AV[\x01\x90a\x05;V[V[a4\x85a4\x80a4\x8A\x92a\x04\xB2V[a\x08\xF1V[a\x0E\xF3V[\x90V[a4\x95a\"GV[P\x80a4\xAFa4\xA9e\xFF\xFF\xFF\xFF\xFF\xFFa*\x96V[\x91a\x04\xB2V[\x11a4\xC0Wa4\xBD\x90a4qV[\x90V[`0a4\xDC_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a4NV[\x03\x90\xFD[\x90V[a4\xF7a4\xF2a4\xFC\x92a4\xE0V[a\x08\xF1V[a\x04\xB2V[\x90V[\x90V[a5\x16a5\x11a5\x1B\x92a4\xFFV[a\x08\xF1V[a\x07\x03V[\x90V[a5=\x90a57a51a5B\x94a\x07\x03V[\x91a\x04\xB2V[\x90a\x10\xDCV[a\x04\xB2V[\x90V[\x90V[a5\\a5Wa5a\x92a5EV[a\x08\xF1V[a\x07\x03V[\x90V[\x1B\x90V[a5\x87\x90a5\x81a5{a5\x8C\x94a\x07\x03V[\x91a\x04\xB2V[\x90a5dV[a\x04\xB2V[\x90V[\x90V[a5\xA6a5\xA1a5\xAB\x92a5\x8FV[a\x08\xF1V[a\x04\xB2V[\x90V[\x90V[a5\xC5a5\xC0a5\xCA\x92a5\xAEV[a\x08\xF1V[a\x07\x03V[\x90V[\x90V[a5\xE4a5\xDFa5\xE9\x92a5\xCDV[a\x08\xF1V[a\x04\xB2V[\x90V[\x90V[a6\x03a5\xFEa6\x08\x92a5\xECV[a\x08\xF1V[a\x07\x03V[\x90V[\x90V[a6\"a6\x1Da6'\x92a6\x0BV[a\x08\xF1V[a\x04\xB2V[\x90V[\x90V[a6Aa6<a6F\x92a6*V[a\x08\xF1V[a\x07\x03V[\x90V[\x90V[a6`a6[a6e\x92a6IV[a\x08\xF1V[a\x04\xB2V[\x90V[\x90V[a6\x7Fa6za6\x84\x92a6hV[a\x08\xF1V[a\x07\x03V[\x90V[a6\x9Ba6\x96a6\xA0\x92a5\xECV[a\x08\xF1V[a\x04\xB2V[\x90V[\x90V[a6\xBAa6\xB5a6\xBF\x92a6\xA3V[a\x08\xF1V[a\x07\x03V[\x90V[a6\xD6a6\xD1a6\xDB\x92a6hV[a\x08\xF1V[a\x04\xB2V[\x90V[a6\xF2a6\xEDa6\xF7\x92a+}V[a\x08\xF1V[a\x07\x03V[\x90V[\x90V[a7\x11a7\x0Ca7\x16\x92a6\xFAV[a\x08\xF1V[a\x04\xB2V[\x90V[\x90a7$\x91\x02a\x04\xB2V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a7Ga7M\x91a\x04\xB2V[\x91a\x04\xB2V[\x90\x81\x15a7XW\x04\x90V[a7'V[\x90a7h\x91\x01a\x04\xB2V[\x90V[a7sa\x17AV[P\x80a7\x88a7\x82`\x01a+\x80V[\x91a\x04\xB2V[\x11\x15a:\xD3W\x80a9\x9Da9za9ja9Za9Ja9:a9*a9\x1Aa9\na8\xFAa8\xEA\x8Ba8\xE4a8\xDDa9\xA3\x9Fa8\xBDa8\xADa8\xCD\x92a7\xCF`\x01a+\x80V[\x90\x80a7\xE7a7\xE1`\x01`\x80\x1Ba4\xE3V[\x91a\x04\xB2V[\x10\x15a:\xA5W[\x80a8\na8\x04h\x01\0\0\0\0\0\0\0\0a5\x92V[\x91a\x04\xB2V[\x10\x15a:wW[\x80a8)a8#d\x01\0\0\0\0a5\xD0V[\x91a\x04\xB2V[\x10\x15a:IW[\x80a8Fa8@b\x01\0\0a6\x0EV[\x91a\x04\xB2V[\x10\x15a:\x1BW[\x80a8ba8\\a\x01\0a6LV[\x91a\x04\xB2V[\x10\x15a9\xEDW[\x80a8}a8w`\x10a6\x87V[\x91a\x04\xB2V[\x10\x15a9\xBFW[a8\x97a8\x91`\x04a6\xC2V[\x91a\x04\xB2V[\x10\x15a9\xA6W[a8\xA8`\x03a6\xFDV[a7\x19V[a8\xB7`\x01a6\xDEV[\x90a5\x1EV[a8\xC7\x81\x86a7;V[\x90a7]V[a8\xD7`\x01a6\xDEV[\x90a5\x1EV[\x80\x92a7;V[\x90a7]V[a8\xF4`\x01a6\xDEV[\x90a5\x1EV[a9\x04\x81\x8Ca7;V[\x90a7]V[a9\x14`\x01a6\xDEV[\x90a5\x1EV[a9$\x81\x8Aa7;V[\x90a7]V[a94`\x01a6\xDEV[\x90a5\x1EV[a9D\x81\x88a7;V[\x90a7]V[a9T`\x01a6\xDEV[\x90a5\x1EV[a9d\x81\x86a7;V[\x90a7]V[a9t`\x01a6\xDEV[\x90a5\x1EV[\x91a9\x97a9\x91a9\x8C\x85\x80\x94a7;V[a\x04\xB2V[\x91a\x04\xB2V[\x11aC\x86V[\x90a&\xFEV[\x90V[a9\xBA\x90a9\xB4`\x01a6\xDEV[\x90a5hV[a8\x9EV[a9\xD6a9\xE7\x91a9\xD0`\x04a6kV[\x90a5\x1EV[\x91a9\xE1`\x02a6\xA6V[\x90a5hV[\x90a8\x84V[a:\x04a:\x15\x91a9\xFE`\x08a6-V[\x90a5\x1EV[\x91a:\x0F`\x04a6kV[\x90a5hV[\x90a8iV[a:2a:C\x91a:,`\x10a5\xEFV[\x90a5\x1EV[\x91a:=`\x08a6-V[\x90a5hV[\x90a8MV[a:`a:q\x91a:Z` a5\xB1V[\x90a5\x1EV[\x91a:k`\x10a5\xEFV[\x90a5hV[\x90a80V[a:\x8Ea:\x9F\x91a:\x88`@a5HV[\x90a5\x1EV[\x91a:\x99` a5\xB1V[\x90a5hV[\x90a8\x11V[a:\xBCa:\xCD\x91a:\xB6`\x80a5\x02V[\x90a5\x1EV[\x91a:\xC7`@a5HV[\x90a5hV[\x90a7\xEEV[\x90V[_R` _ \x01\x90V[\x93\x91\x90\x92a:\xECa\x17AV[P[\x81a;\x01a:\xFB\x83a\x04\xB2V[\x91a\x04\xB2V[\x10\x15a;gWa;\x12\x82\x82\x90aC\xD2V[\x90a;(_a;\"\x88\x85\x90a:\xD6V[\x01a+pV[a;:a;4\x87a\x0E\xF3V[\x91a\x0E\xF3V[\x11_\x14a;JWP\x91[\x91a:\xEEV[\x92\x91Pa;a\x90a;[`\x01a+\x80V[\x90a\x19\xEAV[\x90a;DV[\x92PP\x91P\x90V[a;\x81\x90a;{a\x17AV[Pa\x1D\x04V[\x90V[\x90V[\x91\x90\x91\x80a;\x9Da;\x97\x85a\x04\x83V[\x91a\x04\x83V[\x14\x15\x80a=\x1BW[a;\xAFW[PPPV[\x80a;\xCAa;\xC4a;\xBF_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x03a<\x8BW[P\x81a;\xECa;\xE6a;\xE1_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x03a;\xF8W[\x80a;\xAAV[a<?a<2a<9\x92a<\x0E`\n\x86\x90a\x19\x1BV[\x90a<,a<&a< `\x01\x93aDkV[\x93a\x191V[\x91a;\x84V[\x90aD\xBEV[\x92\x90a\x194V[\x91a\x194V[\x91\x90\x91a<l\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x19\x0FV[\x92a<\x81a<xa\x03\x12V[\x92\x83\x92\x83a\x1F\x95V[\x03\x90\xA2_\x80a;\xF2V[a<\xCAa<\xD0a<\xC3a<\xA0`\n\x85\x90a\x19\x1BV[`\x02a<\xBDa<\xB7a<\xB1\x89aDkV[\x93a\x191V[\x91a;\x84V[\x90aD\xBEV[\x92\x90a\x194V[\x91a\x194V[\x91\x90\x91a<\xFD\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x19\x0FV[\x92a=\x12a=\ta\x03\x12V[\x92\x83\x92\x83a\x1F\x95V[\x03\x90\xA2_a;\xD0V[P\x81a=/a=)_a\x19\xCEV[\x91a\x04\xB2V[\x11a;\xA5V[_a=I\x91a=Ba\x17AV[P\x01a++V[\x90V[a=`a=[a=e\x92a\n\x07V[a\x08\xF1V[a\x04\xB2V[\x90V[a=q\x90a5\xB1V[\x90RV[\x91` a=\x96\x92\x94\x93a=\x8F`@\x82\x01\x96_\x83\x01\x90a=hV[\x01\x90a\x05;V[V[a=\xACa=\xA7a=\xB1\x92a\x04\xB2V[a\x08\xF1V[a\n\x07V[\x90V[a=\xBCa\x1C\xD5V[P\x80a=\xD4a=\xCEc\xFF\xFF\xFF\xFFa=LV[\x91a\x04\xB2V[\x11a=\xE5Wa=\xE2\x90a=\x98V[\x90V[` a>\x01_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a=uV[\x03\x90\xFD[a>\x1Ca>!\x91a>\x14a\x17AV[P`\x08a\x1C\xEEV[a\x17^V[\x90V[\x90V[a>;a>6a>@\x92a>$V[a\x109V[a\x06\x02V[\x90V[a>M`\xFFa>'V[\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a>sa>l\x83a\x15\xC9V[\x80\x94a\x15\xF3V[\x91`\x01\x81\x16\x90\x81_\x14a>\xCAWP`\x01\x14a>\x8EW[PPPV[a>\x9B\x91\x92\x93\x94Pa>PV[\x91_\x92[\x81\x84\x10a>\xB2WPP\x01\x90_\x80\x80a>\x89V[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a>\x9FV[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a>\x89V[\x90a>\xEF\x91a>YV[\x90V[\x90a?\x12a?\x0B\x92a?\x02a\x03\x12V[\x93\x84\x80\x92a>\xE5V[\x03\x83a\x16\xB2V[V[a?\x1D\x90a>\xF2V[\x90V[\x90a?)a\x15\xB0V[Pa?3\x82a\x17\xB4V[a?La?Fa?Aa>CV[a\x06\x02V[\x91a\x06\x02V[\x14\x15_\x14a?aWPa?^\x90aEHV[\x90V[a?k\x91Pa?\x14V[\x90V[`B\x91a?ya\x17\xB0V[P`@Q\x91\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[_\x90V[a?\xBFa?\xC4\x91a\x17EV[a\x1FVV[\x90V[\x90V[a?\xDEa?\xD9a?\xE3\x92a?\xC7V[a\x08\xF1V[a\x04\xB2V[\x90V[a@\x1Ba@\"\x94a@\x11``\x94\x98\x97\x95a@\x07`\x80\x86\x01\x9A_\x87\x01\x90a\x06FV[` \x85\x01\x90a\x07\tV[`@\x83\x01\x90a\x06FV[\x01\x90a\x06FV[V[a@,a\x03\x12V[=_\x82>=\x90\xFD[\x93\x92\x93a@?a\x1C]V[Pa@Ha?\xAFV[Pa@Qa\x17\xB0V[Pa@[\x85a?\xB3V[a@\x8Da@\x87\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0a?\xCAV[\x91a\x04\xB2V[\x11aA\x1AW\x90a@\xB0` \x94\x95_\x94\x93\x92\x93a@\xA7a\x03\x12V[\x94\x85\x94\x85a?\xE6V[\x83\x80R\x03\x90`\x01Z\xFA\x15aA\x15Wa@\xC8_Qa\x109V[\x80a@\xE3a@\xDDa@\xD8_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14a@\xF9W_\x91a@\xF3_a\x10>V[\x91\x92\x91\x90V[PaA\x03_a\x19\xC2V[`\x01\x91aA\x0F_a\x10>V[\x91\x92\x91\x90V[a@$V[PPPaA&_a\x19\xC2V[\x90`\x03\x92\x91\x92\x91\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x04\x11\x15aANWV[aA0V[\x90aA]\x82aADV[V[\x80aAraAl_aASV[\x91aASV[\x14_\x14aA}WPPV[\x80aA\x91aA\x8B`\x01aASV[\x91aASV[\x14_\x14aA\xB4W_c\xF6E\xEE\xDF`\xE0\x1B\x81R\x80aA\xB0`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[\x80aA\xC8aA\xC2`\x02aASV[\x91aASV[\x14_\x14aA\xF6WaA\xF2aA\xDB\x83a?\xB3V[_\x91\x82\x91c\xFC\xE6\x98\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x05HV[\x03\x90\xFD[aB\taB\x03`\x03aASV[\x91aASV[\x14aB\x11WPV[aB,\x90_\x91\x82\x91c5\xE2\xF3\x83`\xE2\x1B\x83R`\x04\x83\x01a\x06SV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_R` _ \x90V[aBV\x81a++V[\x82\x10\x15aBpWaBh`\x01\x91aBDV[\x91\x02\x01\x90_\x90V[aB0V[\x90aB\x7F\x90a\x0E\xF3V[\x90RV[\x90aB\x8D\x90a\x14LV[\x90RV[\x90aB\xC7aB\xBE_aB\xA1a&-V[\x94aB\xB8aB\xB0\x83\x83\x01a+pV[\x83\x88\x01aBuV[\x01a+\xC1V[` \x84\x01aB\x83V[V[aB\xD2\x90aB\x91V[\x90V[aB\xF3\x91_aB\xED\x92aB\xE6a&hV[P\x01aBMV[PaB\xC9V[\x90V[\x92\x91aC\x04\x84\x83\x83\x91aExV[\x83aC\x1FaC\x19aC\x14_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14aC4W[aC2\x92\x93\x91\x90\x91aG\x02V[V[aC<a\x17kV[\x93aCEaF\xE7V[\x94\x80aCYaCS\x88a\x04\xB2V[\x91a\x04\xB2V[\x11aCfWP\x93PaC%V[\x85\x90aC\x82_\x92\x83\x92c\x0EX\xAE\x93`\xE1\x1B\x84R`\x04\x84\x01a\x1F\x95V[\x03\x90\xFD[aC\x8Ea\x17AV[P\x15\x15\x90V[aC\xA8aC\xA3aC\xAD\x92a6\xA3V[a\x08\xF1V[a\x04\xB2V[\x90V[aC\xBCaC\xC2\x91a\x04\xB2V[\x91a\x04\xB2V[\x90\x81\x15aC\xCDW\x04\x90V[a7'V[aC\xF7aC\xFD\x92aC\xE1a\x17AV[P\x82\x81\x16\x92\x18aC\xF1`\x02aC\x94V[\x90aC\xB0V[\x90a\x19\xEAV[\x90V[\x90V[aD\x17aD\x12aD\x1C\x92aD\0V[a\x08\xF1V[a\x07\x03V[\x90V[aD(\x90aD\x03V[\x90RV[\x91` aDM\x92\x94\x93aDF`@\x82\x01\x96_\x83\x01\x90aD\x1FV[\x01\x90a\x05;V[V[aDcaD^aDh\x92a\x04\xB2V[a\x08\xF1V[a\x14LV[\x90V[aDsa\x14\xD7V[P\x80aD\x8DaD\x87`\x01\x80`\xD0\x1B\x03a\x194V[\x91a\x04\xB2V[\x11aD\x9EWaD\x9B\x90aDOV[\x90V[`\xD0aD\xBA_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01aD,V[\x03\x90\xFD[\x90aD\xF4aD\xFA\x93\x92aD\xCFa\x14\xD7V[PaD\xD8a\x14\xD7V[P\x80\x93aD\xEDaD\xE6a\"KV[\x94\x92a/sV[\x90\x91aK}V[\x91aG\xC1V[\x91\x90\x91\x90V[aE\x14aE\x0FaE\x19\x92a5\xAEV[a\x08\xF1V[a\x04\xB2V[\x90V[6\x907V[\x90aEFaE.\x83a\x1B\xA5V[\x92` \x80aE<\x86\x93a\x1B\x82V[\x92\x01\x91\x03\x90aE\x1CV[V[aEPa\x15\xB0V[PaEZ\x81aH+V[\x90aEmaEh` aE\0V[aE!V[\x91\x82R` \x82\x01R\x90V[\x91\x90\x91\x80aE\x96aE\x90aE\x8B_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14_\x14aFwWaE\xBAaE\xB3\x83aE\xAE`\x02a\x17^V[a\x19\xEAV[`\x02a\x1FuV[[\x82aE\xD6aE\xD0aE\xCB_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14_\x14aFKWaE\xFAaE\xF3\x83aE\xEE`\x02a\x17^V[a&\xFEV[`\x02a\x1FuV[[\x91\x90\x91aFFaF4aF.\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93a\x19\x0FV[\x93a\x19\x0FV[\x93aF=a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xA3V[aFr\x82aFlaF]_\x87\x90a\x1C\xEEV[\x91aFg\x83a\x17^V[a7]V[\x90a\x1FuV[aE\xFBV[aF\x8AaF\x85_\x83\x90a\x1C\xEEV[a\x17^V[\x80aF\x9DaF\x97\x85a\x04\xB2V[\x91a\x04\xB2V[\x10aF\xC5WaF\xB0aF\xC0\x91\x84\x90a&\xFEV[aF\xBB_\x84\x90a\x1C\xEEV[a\x1FuV[aE\xBBV[\x90aF\xE3\x90\x91\x92_\x93\x84\x93c9\x144\xE3`\xE2\x1B\x85R`\x04\x85\x01a&\xCCV[\x03\x90\xFD[aF\xEFa\x17AV[PaF\xFF`\x01\x80`\xD0\x1B\x03a\x194V[\x90V[\x91aGZaGTaGa\x94\x80aG(aG\"aG\x1D_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14aG\x92W[\x84aGIaGCaG>_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14aGcW[a\x1C\xA3V[\x92a\x1C\xA3V[\x90\x91a;\x87V[V[aG\x8B`\x0B`\x02aG\x85aG\x7FaGy\x89aDkV[\x93a\x191V[\x91a;\x84V[\x90aD\xBEV[PPaGOV[aG\xBA`\x0B`\x01aG\xB4aG\xAEaG\xA8\x89aDkV[\x93a\x191V[\x91a;\x84V[\x90aD\xBEV[PPaG.V[\x91aG\xE5_aG\xEA\x94aG\xD2a\x14\xD7V[PaG\xDBa\x14\xD7V[P\x01\x92\x91\x92a+NV[aJ/V[\x91\x90\x91\x90V[aH\x04aG\xFFaH\t\x92a>$V[a\x08\xF1V[a\x04\xB2V[\x90V[\x90V[aH#aH\x1EaH(\x92aH\x0CV[a\x08\xF1V[a\x04\xB2V[\x90V[aH@aHE\x91aH:a\x17AV[Pa\x17\xB4V[a?\xB3V[aHO`\xFFaG\xF0V[\x16\x80aHdaH^`\x1FaH\x0FV[\x91a\x04\xB2V[\x11aHlW\x90V[_c,\xD4J\xC3`\xE2\x1B\x81R\x80aH\x84`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[T\x90V[aH\x96`@a\x1BmV[\x90V[_R` _ \x90V[aH\xAB\x81aH\x88V[\x82\x10\x15aH\xC5WaH\xBD`\x01\x91aH\x99V[\x91\x02\x01\x90_\x90V[aB0V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[aH\xE7\x90Qa\x0E\xF3V[\x90V[\x90aH\xFBe\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x109V[\x91\x81\x19\x16\x91\x16\x17\x90V[aI\x19aI\x14aI\x1E\x92a\x0E\xF3V[a\x08\xF1V[a\x0E\xF3V[\x90V[\x90V[\x90aI9aI4aI@\x92aI\x05V[aI!V[\x82TaH\xEAV[\x90UV[aIN\x90Qa\x14LV[\x90V[`0\x1B\x90V[\x90aIie\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91aIQV[\x91\x81\x19\x16\x91\x16\x17\x90V[aI\x87aI\x82aI\x8C\x92a\x14LV[a\x08\xF1V[a\x14LV[\x90V[\x90V[\x90aI\xA7aI\xA2aI\xAE\x92aIsV[aI\x8FV[\x82TaIWV[\x90UV[\x90aI\xDC` _aI\xE2\x94aI\xD4\x82\x82\x01aI\xCE\x84\x88\x01aH\xDDV[\x90aI$V[\x01\x92\x01aIDV[\x90aI\x92V[V[\x91\x90aI\xF5WaI\xF3\x91aI\xB2V[V[aH\xCAV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15aJ*W\x82aJ\"\x91`\x01aJ(\x95\x01\x81UaH\xA2V[\x90aI\xE4V[V[a\x16\x9EV[\x90\x92\x91\x92aJ;a\x14\xD7V[PaJDa\x14\xD7V[PaJN\x82aH\x88V[\x80aJaaJ[_a\x19\xCEV[\x91a\x04\xB2V[\x11_\x14aK1WaJ\x87\x90aJ\x81\x84\x91aJ{`\x01a+\x80V[\x90a\x1B#V[\x90a:\xD6V[\x90aJ\x93_\x83\x01a+pV[\x92aJ\x9F_\x84\x01a+\xC1V[\x93\x80aJ\xB3aJ\xAD\x85a\x0E\xF3V[\x91a\x0E\xF3V[\x11aK\x15WaJ\xCAaJ\xC4\x84a\x0E\xF3V[\x91a\x0E\xF3V[\x14_\x14aJ\xE5WPPaJ\xE0\x90_\x85\x91\x01aI\x92V[[\x91\x90V[aK\x10\x92PaK\x0B\x86aK\x02aJ\xF9aH\x8CV[\x94_\x86\x01aBuV[` \x84\x01aB\x83V[aI\xFAV[aJ\xE1V[_c% `\x1D`\xE0\x1B\x81R\x80aK-`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[PaK\\\x91aKW\x85aKNaKEaH\x8CV[\x94_\x86\x01aBuV[` \x84\x01aB\x83V[aI\xFAV[aKe_a+\xCEV[\x91\x90V[cNH{q`\xE0\x1B_R`Q`\x04R`$_\xFD[\x91\x90\x91\x80`\x01\x14aK\x9CW`\x02\x03aKiWaK\x98\x91a\x15VV[\x90[V[PaK\xA6\x91a\x15\x17V[\x90aK\x9AV",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b6114d3565b61001d5f3561030c565b806301ffc9a71461030757806306fdde0314610302578063095ea7b3146102fd57806318160ddd146102f857806323b872dd146102f3578063248a9ca3146102ee5780632f2ff15d146102e9578063313ce567146102e45780633644e515146102df57806336568abe146102da5780633a46b1a8146102d557806340c10f19146102d057806342966c68146102cb5780634bdd36ce146102c65780634bf5d7e9146102c15780634f1bfc9e146102bc578063587cde1e146102b75780635c19a95c146102b25780636fcfff45146102ad57806370a08231146102a857806379cc6790146102a35780637a8cd1561461029e5780637ecebe001461029957806383f1211b146102945780638426adf21461028f578063844c90261461028a57806384b0196e146102855780638a542521146102805780638d3343d61461027b5780638e539e8c14610276578063902d55a51461027157806391d148541461026c57806391ddadf41461026757806395d89b41146102625780639ab24eb01461025d5780639b7ef64b14610258578063a217fddf14610253578063a9059cbb1461024e578063aa082a9d14610249578063b0ca253e14610244578063bb4d44361461023f578063c02ae7541461023a578063c3cda52014610235578063d505accf14610230578063d547741f1461022b578063dd62ed3e146102265763f1127ed80361000e5761149d565b6113b9565b611358565b61131e565b611274565b6111b8565b611183565b61114d565b611118565b6110a6565b611071565b611001565b610f8a565b610f55565b610f20565b610ebd565b610e88565b610e11565b610ddc565b610d78565b610d0d565b610bc8565b610b93565b610b3a565b610b05565b610ad0565b610a9c565b610a67565b610a32565b6109d4565b61099f565b61092a565b6108b9565b610884565b610851565b6107ff565b6107c9565b610795565b610760565b61072b565b6106cf565b610668565b6105cc565b61055d565b610505565b610443565b610394565b60e01c90565b60405190565b5f80fd5b5f80fd5b63ffffffff60e01b1690565b61033581610320565b0361033c57565b5f80fd5b9050359061034d8261032c565b565b9060208282031261036857610365915f01610340565b90565b61031c565b151590565b61037b9061036d565b9052565b9190610392905f60208501940190610372565b565b346103c4576103c06103af6103aa36600461034f565b611570565b6103b7610312565b9182918261037f565b0390f35b610318565b5f9103126103d357565b61031c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b61041961042260209361042793610410816103d8565b938480936103dc565b958691016103e5565b6103f0565b0190565b6104409160208201915f8184039101526103fa565b90565b34610473576104533660046103c9565b61046f61045e611709565b610466610312565b9182918261042b565b0390f35b610318565b60018060a01b031690565b61048c90610478565b90565b61049881610483565b0361049f57565b5f80fd5b905035906104b08261048f565b565b90565b6104be816104b2565b036104c557565b5f80fd5b905035906104d6826104b5565b565b919060408382031261050057806104f46104fd925f86016104a3565b936020016104c9565b90565b61031c565b346105365761053261052161051b3660046104d8565b9061171f565b610529610312565b9182918261037f565b0390f35b610318565b610544906104b2565b9052565b919061055b905f6020850194019061053b565b565b3461058d5761056d3660046103c9565b61058961057861176b565b610580610312565b91829182610548565b0390f35b610318565b90916060828403126105c7576105c46105ad845f85016104a3565b936105bb81602086016104a3565b936040016104c9565b90565b61031c565b346105fd576105f96105e86105e2366004610592565b91611781565b6105f0610312565b9182918261037f565b0390f35b610318565b90565b61060e81610602565b0361061557565b5f80fd5b9050359061062682610605565b565b906020828203126106415761063e915f01610619565b90565b61031c565b61064f90610602565b9052565b9190610666905f60208501940190610646565b565b346106985761069461068361067e366004610628565b6117fa565b61068b610312565b91829182610653565b0390f35b610318565b91906040838203126106c557806106b96106c2925f8601610619565b936020016104a3565b90565b61031c565b5f0190565b346106fe576106e86106e236600461069d565b90611846565b6106f0610312565b806106fa816106ca565b0390f35b610318565b60ff1690565b61071290610703565b9052565b9190610729905f60208501940190610709565b565b3461075b5761073b3660046103c9565b610757610746611875565b61074e610312565b91829182610716565b0390f35b610318565b34610790576107703660046103c9565b61078c61077b61188b565b610783610312565b91829182610653565b0390f35b610318565b346107c4576107ae6107a836600461069d565b9061189f565b6107b6610312565b806107c0816106ca565b0390f35b610318565b346107fa576107f66107e56107df3660046104d8565b90611950565b6107ed610312565b91829182610548565b0390f35b610318565b3461082e576108186108123660046104d8565b90611ad7565b610820610312565b8061082a816106ca565b0390f35b610318565b9060208282031261084c57610849915f016104c9565b90565b61031c565b3461087f57610869610864366004610833565b611ae3565b610871610312565b8061087b816106ca565b0390f35b610318565b346108b4576108943660046103c9565b6108b061089f611b48565b6108a7610312565b91829182610548565b0390f35b610318565b346108e9576108c93660046103c9565b6108e56108d4611c07565b6108dc610312565b9182918261042b565b0390f35b610318565b90565b90565b61090861090361090d926108ee565b6108f1565b6104b2565b90565b61091c6276a7006108f4565b90565b610927610910565b90565b3461095a5761093a3660046103c9565b61095661094561091f565b61094d610312565b91829182610548565b0390f35b610318565b9060208282031261097857610975915f016104a3565b90565b61031c565b61098690610483565b9052565b919061099d905f6020850194019061097d565b565b346109cf576109cb6109ba6109b536600461095f565b611ca3565b6109c2610312565b9182918261098a565b0390f35b610318565b34610a02576109ec6109e736600461095f565b611cc2565b6109f4610312565b806109fe816106ca565b0390f35b610318565b63ffffffff1690565b610a1990610a07565b9052565b9190610a30905f60208501940190610a10565b565b34610a6257610a5e610a4d610a4836600461095f565b611cd9565b610a55610312565b91829182610a1d565b0390f35b610318565b34610a9757610a93610a82610a7d36600461095f565b611d04565b610a8a610312565b91829182610548565b0390f35b610318565b34610acb57610ab5610aaf3660046104d8565b90611e39565b610abd610312565b80610ac7816106ca565b0390f35b610318565b34610b0057610ae03660046103c9565b610afc610aeb611e45565b610af3610312565b91829182610548565b0390f35b610318565b34610b3557610b31610b20610b1b36600461095f565b611ebd565b610b28610312565b91829182610548565b0390f35b610318565b34610b6a57610b4a3660046103c9565b610b66610b55611ed2565b610b5d610312565b9182918261037f565b0390f35b610318565b7f000000000000000000000000000000000000000000000000000000000000000090565b34610bc357610ba33660046103c9565b610bbf610bae610b6f565b610bb6610312565b91829182610548565b0390f35b610318565b34610bf657610be0610bdb366004610833565b61209d565b610be8610312565b80610bf2816106ca565b0390f35b610318565b60ff60f81b1690565b610c0d90610bfb565b9052565b5190565b60209181520190565b60200190565b610c2d906104b2565b9052565b90610c3e81602093610c24565b0190565b60200190565b90610c65610c5f610c5884610c11565b8093610c15565b92610c1e565b905f5b818110610c755750505090565b909192610c8e610c886001928651610c31565b94610c42565b9101919091610c68565b93959194610ce9610cde610cfd95610cd0610cf395610d0a9c9a610cc360e08c01925f8d0190610c04565b8a820360208c01526103fa565b9088820360408a01526103fa565b97606087019061053b565b608085019061097d565b60a0830190610646565b60c0818403910152610c48565b90565b34610d4457610d1d3660046103c9565b610d40610d28612125565b93610d37979597939193610312565b97889788610c98565b0390f35b610318565b7f84fe74c71a28b69aa960486ca0e8c1418c86e9ea2cd6b5849b95e2c8f407a67490565b610d75610d49565b90565b34610da857610d883660046103c9565b610da4610d93610d6d565b610d9b610312565b91829182610653565b0390f35b610318565b7f9b12e0c5707e494915e58b0564f18aaad9b74ac69bfc815a1edadc8e4bd032eb90565b610dd9610dad565b90565b34610e0c57610dec3660046103c9565b610e08610df7610dd1565b610dff610312565b91829182610653565b0390f35b610318565b34610e4157610e3d610e2c610e27366004610833565b6121af565b610e34610312565b91829182610548565b0390f35b610318565b90565b610e5d610e58610e6292610e46565b6108f1565b6104b2565b90565b610e7a6b033b2e3c9fd0803ce8000000610e49565b90565b610e85610e65565b90565b34610eb857610e983660046103c9565b610eb4610ea3610e7d565b610eab610312565b91829182610548565b0390f35b610318565b34610eee57610eea610ed9610ed336600461069d565b9061221d565b610ee1610312565b9182918261037f565b0390f35b610318565b65ffffffffffff1690565b610f0790610ef3565b9052565b9190610f1e905f60208501940190610efe565b565b34610f5057610f303660046103c9565b610f4c610f3b61224b565b610f43610312565b91829182610f0b565b0390f35b610318565b34610f8557610f653660046103c9565b610f81610f7061225f565b610f78610312565b9182918261042b565b0390f35b610318565b34610fba57610fb6610fa5610fa036600461095f565b612275565b610fad610312565b91829182610548565b0390f35b610318565b90565b610fd6610fd1610fdb92610fbf565b6108f1565b6104b2565b90565b610ff36b02e87669c308736a04000000610fc2565b90565b610ffe610fde565b90565b34611031576110113660046103c9565b61102d61101c610ff6565b611024610312565b91829182610548565b0390f35b610318565b90565b5f1b90565b61105261104d61105792611036565b611039565b610602565b90565b6110635f61103e565b90565b61106e61105a565b90565b346110a1576110813660046103c9565b61109d61108c611066565b611094610312565b91829182610653565b0390f35b610318565b346110d7576110d36110c26110bc3660046104d8565b906122a4565b6110ca610312565b9182918261037f565b0390f35b610318565b1c90565b90565b6110f39060086110f893026110dc565b6110e0565b90565b9061110691546110e3565b90565b611115600c5f906110fb565b90565b34611148576111283660046103c9565b611144611133611109565b61113b610312565b91829182610548565b0390f35b610318565b3461117e5761117a6111696111633660046104d8565b906122c6565b611171610312565b91829182610548565b0390f35b610318565b346111b3576111af61119e61119936600461095f565b6122dc565b6111a6610312565b91829182610548565b0390f35b610318565b346111e8576111c83660046103c9565b6111e46111d36122f1565b6111db610312565b91829182610548565b0390f35b610318565b6111f681610703565b036111fd57565b5f80fd5b9050359061120e826111ed565b565b909160c08284031261126f57611228835f84016104a3565b9261123681602085016104c9565b9261124482604083016104c9565b9261126c6112558460608501611201565b936112638160808601610619565b9360a001610619565b90565b61031c565b346112a957611293611287366004611210565b94939093929192612371565b61129b610312565b806112a5816106ca565b0390f35b610318565b60e081830312611319576112c4825f83016104a3565b926112d283602084016104a3565b926112e081604085016104c9565b926112ee82606083016104c9565b926113166112ff8460808501611201565b9361130d8160a08601610619565b9360c001610619565b90565b61031c565b346113535761133d6113313660046112ae565b959490949391936124c5565b611345610312565b8061134f816106ca565b0390f35b610318565b346113875761137161136b36600461069d565b906125e3565b611379610312565b80611383816106ca565b0390f35b610318565b91906040838203126113b457806113a86113b1925f86016104a3565b936020016104a3565b90565b61031c565b346113ea576113e66113d56113cf36600461138c565b90612605565b6113dd610312565b91829182610548565b0390f35b610318565b6113f881610a07565b036113ff57565b5f80fd5b90503590611410826113ef565b565b919060408382031261143a578061142e611437925f86016104a3565b93602001611403565b90565b61031c565b61144890610ef3565b9052565b60018060d01b031690565b6114609061144c565b9052565b906020806114869361147c5f8201515f86019061143f565b0151910190611457565b565b919061149b905f60408501940190611464565b565b346114ce576114ca6114b96114b3366004611412565b90612673565b6114c1610312565b91829182611488565b0390f35b610318565b5f80fd5b5f90565b634e487b7160e01b5f52601160045260245ffd5b6114fb6115019161144c565b9161144c565b019060018060d01b03821161151257565b6114db565b9061152a916115246114d7565b506114ef565b90565b61153961153f9161144c565b9161144c565b90039060018060d01b03821161155157565b6114db565b90611569916115636114d7565b5061152d565b90565b5f90565b61157861156c565b508061159361158d637965db0b60e01b610320565b91610320565b149081156115a0575b5090565b6115aa9150612689565b5f61159c565b606090565b634e487b7160e01b5f52602260045260245ffd5b90600160028304921680156115e9575b60208310146115e457565b6115b5565b91607f16916115d9565b60209181520190565b5f5260205f2090565b905f929180549061161f611618836115c9565b80946115f3565b916001811690815f14611676575060011461163a575b505050565b61164791929394506115fc565b915f925b81841061165e57505001905f8080611635565b6001816020929593955484860152019101929061164b565b92949550505060ff19168252151560200201905f8080611635565b9061169b91611605565b90565b634e487b7160e01b5f52604160045260245ffd5b906116bc906103f0565b810190811067ffffffffffffffff8211176116d657604052565b61169e565b906116fb6116f4926116eb610312565b93848092611691565b03836116b2565b565b611706906116db565b90565b6117116115b0565b5061171c60036116fd565b90565b61173c9161172b61156c565b506117346126af565b9190916126bc565b600190565b5f90565b5f1c90565b61175661175b91611745565b6110e0565b90565b611768905461174a565b90565b611773611741565b5061177e600261175e565b90565b916117ab9261178e61156c565b506117a361179a6126af565b8290849161270c565b919091612798565b600190565b5f90565b6117bd90610602565b90565b906117ca906117b4565b5f5260205260405f2090565b90565b6117e56117ea91611745565b6117d6565b90565b6117f790546117d9565b90565b60016118136118199261180b6117b0565b5060056117c0565b016117ed565b90565b906118379161183261182d826117fa565b612835565b611839565b565b906118439161288e565b50565b906118509161181c565b565b5f90565b90565b61186d61186861187292611856565b6108f1565b610703565b90565b61187d611852565b506118886012611859565b90565b6118936117b0565b5061189c61293a565b90565b90806118ba6118b46118af6126af565b610483565b91610483565b036118cb576118c8916129f4565b50565b5f63334bd91960e11b8152806118e3600482016106ca565b0390fd5b6118fb6118f661190092610478565b6108f1565b610478565b90565b61190c906118e7565b90565b61191890611903565b90565b906119259061190f565b5f5260205260405f2090565b90565b61194861194361194d9261144c565b6108f1565b6104b2565b90565b6119879161197c61197661197161198294611969611741565b50600a61191b565b611931565b91612ad5565b90612bea565b611934565b90565b906119a49161199f61199a610dad565b612835565b611a0f565b565b6119ba6119b56119bf92611036565b6108f1565b610478565b90565b6119cb906119a6565b90565b6119e26119dd6119e792611036565b6108f1565b6104b2565b90565b6119f96119ff919392936104b2565b926104b2565b8201809211611a0a57565b6114db565b9081611a2b611a25611a205f6119c2565b610483565b91610483565b14611abb5780611a43611a3d5f6119ce565b916104b2565b14611a9f57611a5a611a5361176b565b82906119ea565b611a73611a6d611a68610e65565b6104b2565b916104b2565b11611a8357611a8191612d11565b565b5f63177e3fc360e01b815280611a9b600482016106ca565b0390fd5b5f631f2a200560e01b815280611ab7600482016106ca565b0390fd5b5f63d92e233d60e01b815280611ad3600482016106ca565b0390fd5b90611ae19161198a565b565b80611af6611af05f6119ce565b916104b2565b14611b0757611b059033612d6f565b565b5f631f2a200560e01b815280611b1f600482016106ca565b0390fd5b611b32611b38919392936104b2565b926104b2565b8203918211611b4357565b6114db565b611b50611741565b50611b6a611b5c610e65565b611b6461176b565b90611b23565b90565b90611b80611b79610312565b92836116b2565b565b67ffffffffffffffff8111611ba057611b9c6020916103f0565b0190565b61169e565b90611bb7611bb283611b82565b611b6d565b918252565b5f7f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c74000000910152565b611bed601d611ba5565b90611bfa60208301611bbc565b565b611c04611be3565b90565b611c0f6115b0565b50611c1861224b565b611c31611c2b611c26612dce565b610ef3565b91610ef3565b03611c4157611c3e611bfc565b90565b5f6301bfc1c560e61b815280611c59600482016106ca565b0390fd5b5f90565b90611c6b9061190f565b5f5260205260405f2090565b60018060a01b031690565b611c8e611c9391611745565b611c77565b90565b611ca09054611c82565b90565b611cba611cbf91611cb2611c5d565b506009611c61565b611c96565b90565b611cd390611cce6126af565b612e21565b565b5f90565b611ceb90611ce5611cd5565b50612eac565b90565b90611cf89061190f565b5f5260205260405f2090565b611d1a611d1f91611d13611741565b505f611cee565b61175e565b90565b90611d3c91611d37611d32610d49565b612835565b611d3e565b565b80611d59611d53611d4e5f6119c2565b610483565b91610483565b14611e1d5781611d71611d6b5f6119ce565b916104b2565b14611e0157611d87611d81611ed2565b1561036d565b611de557611d96818390612d6f565b3390611de0611dce611dc87fbef4f81c1814c641ede85ebaacf19d048b2c5b55980adfa6ef0f956c651335a29361190f565b9361190f565b93611dd7610312565b91829182610548565b0390a3565b5f63b8b5ca2d60e01b815280611dfd600482016106ca565b0390fd5b5f631f2a200560e01b815280611e19600482016106ca565b0390fd5b5f63d92e233d60e01b815280611e35600482016106ca565b0390fd5b90611e4391611d22565b565b611e4d611741565b50611e58600c61175e565b611e6a611e645f6119ce565b916104b2565b148015611e99575b611e8d57611e8a611e83600c61175e565b4290611b23565b90565b611e965f6119ce565b90565b5042611eb6611eb0611eab600c61175e565b6104b2565b916104b2565b1015611e72565b611ecf90611ec9611741565b50612edb565b90565b611eda61156c565b50611ee5600c61175e565b611ef7611ef15f6119ce565b916104b2565b141580611f02575b90565b5042611f1f611f19611f14600c61175e565b6104b2565b916104b2565b10611eff565b611f3e90611f39611f3461105a565b612835565b611fb8565b565b90611f4c5f1991611039565b9181191691161790565b611f6a611f65611f6f926104b2565b6108f1565b6104b2565b90565b90565b90611f8a611f85611f9192611f56565b611f72565b8254611f40565b9055565b916020611fb6929493611faf60408201965f83019061053b565b019061053b565b565b80611fcb611fc5426104b2565b916104b2565b11156120815780612004611ffe7f00000000000000000000000000000000000000000000000000000000000000006104b2565b916104b2565b1161206557612013600c61175e565b61201e82600c611f75565b90339061204b7fdd6896dcf1d4b311cca87dd19bbba2ea9ce2f867c1568878a0438a66a1afeeec9261190f565b92612060612057610312565b92839283611f95565b0390a2565b5f63ef69af6560e01b81528061207d600482016106ca565b0390fd5b5f63a565835360e01b815280612099600482016106ca565b0390fd5b6120a690611f25565b565b5f90565b606090565b6120ba90611903565b90565b67ffffffffffffffff81116120d55760208091020190565b61169e565b906120ec6120e7836120bd565b611b6d565b918252565b369037565b9061211b612103836120da565b9260208061211186936120bd565b92019103906120f1565b565b600f60f81b90565b61212d6120a8565b506121366115b0565b5061213f6115b0565b50612148611741565b50612151611c5d565b5061215a6117b0565b506121636120ac565b5061216c612ef3565b90612175612f33565b904690612181306120b1565b9061218b5f61103e565b9061219d6121985f6119ce565b6120f6565b906121a661211d565b96959493929190565b6121d86121dd916121be611741565b506121d26121cc600b611931565b91612ad5565b90612bea565b611934565b90565b906121ea9061190f565b5f5260205260405f2090565b60ff1690565b61220861220d91611745565b6121f6565b90565b61221a90546121fc565b90565b612244915f61223961223f9361223161156c565b5060056117c0565b016121e0565b612210565b90565b5f90565b612253612247565b5061225c612dce565b90565b6122676115b0565b5061227260046116fd565b90565b61229c6122976122926122a19361228a611741565b50600a61191b565b611931565b612f73565b611934565b90565b6122c1916122b061156c565b506122b96126af565b919091612798565b600190565b906122d9916122d3611741565b50611950565b90565b6122ee906122e8611741565b50612275565b90565b6122f9611741565b5061230261176b565b90565b7fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf90565b61235e6123659461235460609498979561234a608086019a5f870190610646565b602085019061097d565b604083019061053b565b019061053b565b565b60200190565b5190565b939594909291954261238b612385896104b2565b916104b2565b1161240457916123f6916123fd936123ed61240298996123d56123ac612305565b6123c68b938b6123ba610312565b95869460208601612329565b602082018103825203826116b2565b6123e76123e18261236d565b91612367565b20612fe8565b92909192613005565b918261304f565b612e21565b565b61241f875f918291632341d78760e11b835260048301610548565b0390fd5b7f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c990565b919461248f6124999298979561248560a09661247b6124a09a61247160c08a019e5f8b0190610646565b602089019061097d565b604087019061097d565b606085019061053b565b608083019061053b565b019061053b565b565b9160206124c39294936124bc60408201965f83019061097d565b019061097d565b565b9695919392949094426124e06124da836104b2565b916104b2565b1161259a57906125496125529493926125316124fa612423565b6125228c80948c9161250c8d916130a1565b9192612516610312565b97889660208801612447565b602082018103825203826116b2565b61254361253d8261236d565b91612367565b20612fe8565b92909192613005565b8061256561255f87610483565b91610483565b0361257a575061257892939190916126bc565b565b84906125965f9283926325c0072360e11b8452600484016124a2565b0390fd5b6125b5905f91829163313c898160e11b835260048301610548565b0390fd5b906125d4916125cf6125ca826117fa565b612835565b6125d6565b565b906125e0916129f4565b50565b906125ed916125b9565b565b906125f99061190f565b5f5260205260405f2090565b61262a9161262061262592612618611741565b5060016125ef565b611cee565b61175e565b90565b6126376040611b6d565b90565b5f90565b5f90565b61264a61262d565b906020808361265761263a565b81520161266261263e565b81525050565b612670612642565b90565b9061268691612680612668565b506130d4565b90565b61269161156c565b506126ab6126a56301ffc9a760e01b610320565b91610320565b1490565b6126b7611c5d565b503390565b916126ca92916001926130fc565b565b6040906126f56126fc94969593966126eb60608401985f85019061097d565b602083019061053b565b019061053b565b565b9061270991036104b2565b90565b92919261271a818390612605565b908161272f6127295f196104b2565b916104b2565b1061273c575b5050509050565b8161274f612749876104b2565b916104b2565b106127755761276c93946127649193926126fe565b905f926130fc565b805f8080612735565b50612794849291925f938493637dc7a0d960e11b8552600485016126cc565b0390fd5b91826127b46127ae6127a95f6119c2565b610483565b91610483565b1461280e57816127d46127ce6127c95f6119c2565b610483565b91610483565b146127e7576127e59291909161320b565b565b61280a6127f35f6119c2565b5f91829163ec442f0560e01b83526004830161098a565b0390fd5b61283161281a5f6119c2565b5f918291634b637e8f60e11b83526004830161098a565b0390fd5b612847906128416126af565b906132d8565b565b9061285560ff91611039565b9181191691161790565b6128689061036d565b90565b90565b9061288361287e61288a9261285f565b61286b565b8254612849565b9055565b61289661156c565b506128ab6128a582849061221d565b1561036d565b5f14612934576128d360016128ce5f6128c6600586906117c0565b0185906121e0565b61286e565b906128dc6126af565b9061291961291361290d7f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d956117b4565b9261190f565b9261190f565b92612922610312565b8061292c816106ca565b0390a4600190565b50505f90565b6129426117b0565b5061294c306120b1565b61297e6129787f0000000000000000000000000000000000000000000000000000000000000000610483565b91610483565b14806129ba575b5f146129af577f000000000000000000000000000000000000000000000000000000000000000090565b6129b7613384565b90565b50466129ee6129e87f00000000000000000000000000000000000000000000000000000000000000006104b2565b916104b2565b14612985565b6129fc61156c565b50612a0881839061221d565b5f14612a9057612a2f5f612a2a5f612a22600586906117c0565b0185906121e0565b61286e565b90612a386126af565b90612a75612a6f612a697ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b956117b4565b9261190f565b9261190f565b92612a7e610312565b80612a88816106ca565b0390a4600190565b50505f90565b612aaa612aa5612aaf92610ef3565b6108f1565b6104b2565b90565b916020612ad3929493612acc60408201965f83019061053b565b0190610efe565b565b612add612247565b50612ae661224b565b81612af9612af383612a96565b916104b2565b1015612b0c5750612b099061348d565b90565b90612b275f928392637669fc0f60e11b845260048401612ab2565b0390fd5b5490565b90565b612b46612b41612b4b92612b2f565b6108f1565b6104b2565b90565b90565b65ffffffffffff1690565b612b68612b6d91611745565b612b51565b90565b612b7a9054612b5c565b90565b90565b612b94612b8f612b9992612b7d565b6108f1565b6104b2565b90565b60301c90565b60018060d01b031690565b612bb9612bbe91612b9c565b612ba2565b90565b612bcb9054612bad565b90565b612be2612bdd612be792611036565b6108f1565b61144c565b90565b90612c3e90612bf76114d7565b50612c035f8401612b2b565b612c0c5f6119ce565b908080612c22612c1c6005612b32565b916104b2565b11612c9f575b5090612c395f860193919293612b4e565b613ae0565b80612c51612c4b5f6119ce565b916104b2565b145f14612c67575050612c635f612bce565b5b90565b612c945f91612c8f612c8984612c9a960192612c836001612b80565b90611b23565b91612b4e565b613ad6565b01612bc1565b612c64565b80612cad612cb3929161376b565b90611b23565b9083612ce5612cdf612cda5f612cd4818c01612ccf8991612b4e565b613ad6565b01612b70565b610ef3565b91610ef3565b105f14612cf65750905b905f612c28565b9150612d0c90612d066001612b80565b906119ea565b612cef565b80612d2c612d26612d215f6119c2565b610483565b91610483565b14612d4857612d4691612d3e5f6119c2565b91909161320b565b565b612d6b612d545f6119c2565b5f91829163ec442f0560e01b83526004830161098a565b0390fd5b9081612d8b612d85612d805f6119c2565b610483565b91610483565b14612da757612da59190612d9e5f6119c2565b909161320b565b565b612dca612db35f6119c2565b5f918291634b637e8f60e11b83526004830161098a565b0390fd5b612dd6612247565b50612de04361348d565b90565b90612df460018060a01b0391611039565b9181191691161790565b90565b90612e16612e11612e1d9261190f565b612dfe565b8254612de3565b9055565b90612eaa91612ea4612e3282611ca3565b612e4784612e4260098690611c61565b612e01565b82818590612e87612e81612e7b7f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9561190f565b9261190f565b9261190f565b92612e90610312565b80612e9a816106ca565b0390a49291613b6f565b91613b87565b565b612ed3612ece612ec9612ed893612ec1611cd5565b50600a61191b565b611931565b613d35565b613db4565b90565b612eed90612ee7611741565b50613e05565b90565b90565b612efb6115b0565b50612f307f0000000000000000000000000000000000000000000000000000000000000000612f2a6006612ef0565b90613f20565b90565b612f3b6115b0565b50612f707f0000000000000000000000000000000000000000000000000000000000000000612f6a6007612ef0565b90613f20565b90565b612f7b6114d7565b50612f875f8201612b2b565b80612f9a612f945f6119ce565b916104b2565b145f14612fb0575050612fac5f612bce565b5b90565b612fdd5f91612fd8612fd284612fe3960192612fcc6001612b80565b90611b23565b91612b4e565b613ad6565b01612bc1565b612fad565b61300290612ff46117b0565b50612ffd61293a565b613f6e565b90565b926130209261302994613016611c5d565b5092909192614034565b9092919261415f565b90565b91602061304d92949361304660408201965f83019061097d565b019061053b565b565b613058816130a1565b9161306b613065846104b2565b916104b2565b03613074575050565b61308e5f9283926301d4b62360e61b84526004840161302c565b0390fd5b600161309e91016104b2565b90565b6130b5906130ad611741565b506008611cee565b6130d16130c18261175e565b916130cb83613092565b90611f75565b90565b906130f46130ef6130f9936130e7612668565b50600a61191b565b611931565b6142d5565b90565b90928161311961311361310e5f6119c2565b610483565b91610483565b146131e4578361313961313361312e5f6119c2565b610483565b91610483565b146131bd5761315d83613158613151600186906125ef565b8790611cee565b611f75565b613167575b505050565b9190916131b26131a061319a7f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b9259361190f565b9361190f565b936131a9610312565b91829182610548565b0390a35f8080613162565b6131e06131c95f6119c2565b5f918291634a1406b160e11b83526004830161098a565b0390fd5b6132076131f05f6119c2565b5f91829163e602df0560e01b83526004830161098a565b0390fd5b918261322761322161321c5f6119c2565b610483565b91610483565b141580613292575b613242575b613240929190916142f6565b565b61324a611ed2565b80613271575b15613234575f6336e278fd60e21b81528061326d600482016106ca565b0390fd5b5061328d613287613280610d49565b339061221d565b1561036d565b613250565b50816132ae6132a86132a35f6119c2565b610483565b91610483565b141561322f565b9160206132d69294936132cf60408201965f83019061097d565b0190610646565b565b906132ed6132e783839061221d565b1561036d565b6132f5575050565b61330f5f92839263e2517d3f60e01b8452600484016132b5565b0390fd5b7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f90565b909594926133829461337161337b9261336760809661335d60a088019c5f890190610646565b6020870190610646565b6040850190610646565b606083019061053b565b019061097d565b565b61338c6117b0565b50613395613313565b61340c7f0000000000000000000000000000000000000000000000000000000000000000916133fd7f0000000000000000000000000000000000000000000000000000000000000000466133e8306120b1565b916133f1610312565b96879560208701613337565b602082018103825203826116b2565b61341e6134188261236d565b91612367565b2090565b90565b61343961343461343e92613422565b6108f1565b610703565b90565b61344a90613425565b9052565b91602061346f92949361346860408201965f830190613441565b019061053b565b565b61348561348061348a926104b2565b6108f1565b610ef3565b90565b613495612247565b50806134af6134a965ffffffffffff612a96565b916104b2565b116134c0576134bd90613471565b90565b60306134dc5f9283926306dfcc6560e41b84526004840161344e565b0390fd5b90565b6134f76134f26134fc926134e0565b6108f1565b6104b2565b90565b90565b61351661351161351b926134ff565b6108f1565b610703565b90565b61353d9061353761353161354294610703565b916104b2565b906110dc565b6104b2565b90565b90565b61355c61355761356192613545565b6108f1565b610703565b90565b1b90565b6135879061358161357b61358c94610703565b916104b2565b90613564565b6104b2565b90565b90565b6135a66135a16135ab9261358f565b6108f1565b6104b2565b90565b90565b6135c56135c06135ca926135ae565b6108f1565b610703565b90565b90565b6135e46135df6135e9926135cd565b6108f1565b6104b2565b90565b90565b6136036135fe613608926135ec565b6108f1565b610703565b90565b90565b61362261361d6136279261360b565b6108f1565b6104b2565b90565b90565b61364161363c6136469261362a565b6108f1565b610703565b90565b90565b61366061365b61366592613649565b6108f1565b6104b2565b90565b90565b61367f61367a61368492613668565b6108f1565b610703565b90565b61369b6136966136a0926135ec565b6108f1565b6104b2565b90565b90565b6136ba6136b56136bf926136a3565b6108f1565b610703565b90565b6136d66136d16136db92613668565b6108f1565b6104b2565b90565b6136f26136ed6136f792612b7d565b6108f1565b610703565b90565b90565b61371161370c613716926136fa565b6108f1565b6104b2565b90565b9061372491026104b2565b90565b634e487b7160e01b5f52601260045260245ffd5b61374761374d916104b2565b916104b2565b908115613758570490565b613727565b9061376891016104b2565b90565b613773611741565b50806137886137826001612b80565b916104b2565b1115613ad3578061399d61397a61396a61395a61394a61393a61392a61391a61390a6138fa6138ea8b6138e46138dd6139a39f6138bd6138ad6138cd926137cf6001612b80565b90806137e76137e1600160801b6134e3565b916104b2565b1015613aa5575b8061380a61380468010000000000000000613592565b916104b2565b1015613a77575b806138296138236401000000006135d0565b916104b2565b1015613a49575b806138466138406201000061360e565b916104b2565b1015613a1b575b8061386261385c61010061364c565b916104b2565b10156139ed575b8061387d6138776010613687565b916104b2565b10156139bf575b61389761389160046136c2565b916104b2565b10156139a6575b6138a860036136fd565b613719565b6138b760016136de565b9061351e565b6138c7818661373b565b9061375d565b6138d760016136de565b9061351e565b809261373b565b9061375d565b6138f460016136de565b9061351e565b613904818c61373b565b9061375d565b61391460016136de565b9061351e565b613924818a61373b565b9061375d565b61393460016136de565b9061351e565b613944818861373b565b9061375d565b61395460016136de565b9061351e565b613964818661373b565b9061375d565b61397460016136de565b9061351e565b9161399761399161398c85809461373b565b6104b2565b916104b2565b11614386565b906126fe565b90565b6139ba906139b460016136de565b90613568565b61389e565b6139d66139e7916139d0600461366b565b9061351e565b916139e160026136a6565b90613568565b90613884565b613a04613a15916139fe600861362d565b9061351e565b91613a0f600461366b565b90613568565b90613869565b613a32613a4391613a2c60106135ef565b9061351e565b91613a3d600861362d565b90613568565b9061384d565b613a60613a7191613a5a60206135b1565b9061351e565b91613a6b60106135ef565b90613568565b90613830565b613a8e613a9f91613a886040613548565b9061351e565b91613a9960206135b1565b90613568565b90613811565b613abc613acd91613ab66080613502565b9061351e565b91613ac76040613548565b90613568565b906137ee565b90565b5f5260205f200190565b93919092613aec611741565b505b81613b01613afb836104b2565b916104b2565b1015613b6757613b128282906143d2565b90613b285f613b22888590613ad6565b01612b70565b613b3a613b3487610ef3565b91610ef3565b115f14613b4a5750915b91613aee565b929150613b6190613b5b6001612b80565b906119ea565b90613b44565b925050915090565b613b8190613b7b611741565b50611d04565b90565b90565b91909180613b9d613b9785610483565b91610483565b141580613d1b575b613baf575b505050565b80613bca613bc4613bbf5f6119c2565b610483565b91610483565b03613c8b575b5081613bec613be6613be15f6119c2565b610483565b91610483565b03613bf8575b80613baa565b613c3f613c32613c3992613c0e600a869061191b565b90613c2c613c26613c2060019361446b565b93611931565b91613b84565b906144be565b9290611934565b91611934565b919091613c6c7fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a7249261190f565b92613c81613c78610312565b92839283611f95565b0390a25f80613bf2565b613cca613cd0613cc3613ca0600a859061191b565b6002613cbd613cb7613cb18961446b565b93611931565b91613b84565b906144be565b9290611934565b91611934565b919091613cfd7fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a7249261190f565b92613d12613d09610312565b92839283611f95565b0390a25f613bd0565b5081613d2f613d295f6119ce565b916104b2565b11613ba5565b5f613d4991613d42611741565b5001612b2b565b90565b613d60613d5b613d6592610a07565b6108f1565b6104b2565b90565b613d71906135b1565b9052565b916020613d96929493613d8f60408201965f830190613d68565b019061053b565b565b613dac613da7613db1926104b2565b6108f1565b610a07565b90565b613dbc611cd5565b5080613dd4613dce63ffffffff613d4c565b916104b2565b11613de557613de290613d98565b90565b6020613e015f9283926306dfcc6560e41b845260048401613d75565b0390fd5b613e1c613e2191613e14611741565b506008611cee565b61175e565b90565b90565b613e3b613e36613e4092613e24565b611039565b610602565b90565b613e4d60ff613e27565b90565b5f5260205f2090565b905f9291805490613e73613e6c836115c9565b80946115f3565b916001811690815f14613eca5750600114613e8e575b505050565b613e9b9192939450613e50565b915f925b818410613eb257505001905f8080613e89565b60018160209295939554848601520191019290613e9f565b92949550505060ff19168252151560200201905f8080613e89565b90613eef91613e59565b90565b90613f12613f0b92613f02610312565b93848092613ee5565b03836116b2565b565b613f1d90613ef2565b90565b90613f296115b0565b50613f33826117b4565b613f4c613f46613f41613e43565b610602565b91610602565b14155f14613f615750613f5e90614548565b90565b613f6b9150613f14565b90565b604291613f796117b0565b50604051917f19010000000000000000000000000000000000000000000000000000000000008352600283015260228201522090565b5f90565b613fbf613fc491611745565b611f56565b90565b90565b613fde613fd9613fe392613fc7565b6108f1565b6104b2565b90565b61401b61402294614011606094989795614007608086019a5f870190610646565b6020850190610709565b6040830190610646565b0190610646565b565b61402c610312565b3d5f823e3d90fd5b93929361403f611c5d565b50614048613faf565b506140516117b0565b5061405b85613fb3565b61408d6140877f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0613fca565b916104b2565b1161411a57906140b0602094955f949392936140a7610312565b94859485613fe6565b838052039060015afa15614115576140c85f51611039565b806140e36140dd6140d85f6119c2565b610483565b91610483565b146140f9575f916140f35f61103e565b91929190565b506141035f6119c2565b60019161410f5f61103e565b91929190565b614024565b5050506141265f6119c2565b9060039291929190565b634e487b7160e01b5f52602160045260245ffd5b6004111561414e57565b614130565b9061415d82614144565b565b8061417261416c5f614153565b91614153565b145f1461417d575050565b8061419161418b6001614153565b91614153565b145f146141b4575f63f645eedf60e01b8152806141b0600482016106ca565b0390fd5b806141c86141c26002614153565b91614153565b145f146141f6576141f26141db83613fb3565b5f91829163fce698f760e01b835260048301610548565b0390fd5b6142096142036003614153565b91614153565b146142115750565b61422c905f9182916335e2f38360e21b835260048301610653565b0390fd5b634e487b7160e01b5f52603260045260245ffd5b5f5260205f2090565b61425681612b2b565b82101561427057614268600191614244565b910201905f90565b614230565b9061427f90610ef3565b9052565b9061428d9061144c565b9052565b906142c76142be5f6142a161262d565b946142b86142b0838301612b70565b838801614275565b01612bc1565b60208401614283565b565b6142d290614291565b90565b6142f3915f6142ed926142e6612668565b500161424d565b506142c9565b90565b929161430484838391614578565b8361431f6143196143145f6119c2565b610483565b91610483565b14614334575b6143329293919091614702565b565b61433c61176b565b936143456146e7565b9480614359614353886104b2565b916104b2565b1161436657509350614325565b85906143825f928392630e58ae9360e11b845260048401611f95565b0390fd5b61438e611741565b50151590565b6143a86143a36143ad926136a3565b6108f1565b6104b2565b90565b6143bc6143c2916104b2565b916104b2565b9081156143cd570490565b613727565b6143f76143fd926143e1611741565b5082811692186143f16002614394565b906143b0565b906119ea565b90565b90565b61441761441261441c92614400565b6108f1565b610703565b90565b61442890614403565b9052565b91602061444d92949361444660408201965f83019061441f565b019061053b565b565b61446361445e614468926104b2565b6108f1565b61144c565b90565b6144736114d7565b508061448d61448760018060d01b03611934565b916104b2565b1161449e5761449b9061444f565b90565b60d06144ba5f9283926306dfcc6560e41b84526004840161442c565b0390fd5b906144f46144fa93926144cf6114d7565b506144d86114d7565b5080936144ed6144e661224b565b9492612f73565b9091614b7d565b916147c1565b91909190565b61451461450f614519926135ae565b6108f1565b6104b2565b90565b369037565b9061454661452e83611ba5565b9260208061453c8693611b82565b920191039061451c565b565b6145506115b0565b5061455a8161482b565b9061456d6145686020614500565b614521565b918252602082015290565b9190918061459661459061458b5f6119c2565b610483565b91610483565b145f14614677576145ba6145b3836145ae600261175e565b6119ea565b6002611f75565b5b826145d66145d06145cb5f6119c2565b610483565b91610483565b145f1461464b576145fa6145f3836145ee600261175e565b6126fe565b6002611f75565b5b91909161464661463461462e7fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef9361190f565b9361190f565b9361463d610312565b91829182610548565b0390a3565b6146728261466c61465d5f8790611cee565b916146678361175e565b61375d565b90611f75565b6145fb565b61468a6146855f8390611cee565b61175e565b8061469d614697856104b2565b916104b2565b106146c5576146b06146c09184906126fe565b6146bb5f8490611cee565b611f75565b6145bb565b906146e39091925f93849363391434e360e21b8552600485016126cc565b0390fd5b6146ef611741565b506146ff60018060d01b03611934565b90565b9161475a614754614761948061472861472261471d5f6119c2565b610483565b91610483565b14614792575b8461474961474361473e5f6119c2565b610483565b91610483565b14614763575b611ca3565b92611ca3565b9091613b87565b565b61478b600b600261478561477f6147798961446b565b93611931565b91613b84565b906144be565b505061474f565b6147ba600b60016147b46147ae6147a88961446b565b93611931565b91613b84565b906144be565b505061472e565b916147e55f6147ea946147d26114d7565b506147db6114d7565b5001929192612b4e565b614a2f565b91909190565b6148046147ff61480992613e24565b6108f1565b6104b2565b90565b90565b61482361481e6148289261480c565b6108f1565b6104b2565b90565b6148406148459161483a611741565b506117b4565b613fb3565b61484f60ff6147f0565b168061486461485e601f61480f565b916104b2565b1161486c5790565b5f632cd44ac360e21b815280614884600482016106ca565b0390fd5b5490565b6148966040611b6d565b90565b5f5260205f2090565b6148ab81614888565b8210156148c5576148bd600191614899565b910201905f90565b614230565b634e487b7160e01b5f525f60045260245ffd5b6148e79051610ef3565b90565b906148fb65ffffffffffff91611039565b9181191691161790565b61491961491461491e92610ef3565b6108f1565b610ef3565b90565b90565b9061493961493461494092614905565b614921565b82546148ea565b9055565b61494e905161144c565b90565b60301b90565b9061496965ffffffffffff1991614951565b9181191691161790565b61498761498261498c9261144c565b6108f1565b61144c565b90565b90565b906149a76149a26149ae92614973565b61498f565b8254614957565b9055565b906149dc60205f6149e2946149d48282016149ce8488016148dd565b90614924565b019201614944565b90614992565b565b91906149f5576149f3916149b2565b565b6148ca565b9081549168010000000000000000831015614a2a5782614a22916001614a28950181556148a2565b906149e4565b565b61169e565b90929192614a3b6114d7565b50614a446114d7565b50614a4e82614888565b80614a61614a5b5f6119ce565b916104b2565b115f14614b3157614a8790614a818491614a7b6001612b80565b90611b23565b90613ad6565b90614a935f8301612b70565b92614a9f5f8401612bc1565b9380614ab3614aad85610ef3565b91610ef3565b11614b1557614aca614ac484610ef3565b91610ef3565b145f14614ae5575050614ae0905f859101614992565b5b9190565b614b109250614b0b86614b02614af961488c565b945f8601614275565b60208401614283565b6149fa565b614ae1565b5f632520601d60e01b815280614b2d600482016106ca565b0390fd5b50614b5c91614b5785614b4e614b4561488c565b945f8601614275565b60208401614283565b6149fa565b614b655f612bce565b9190565b634e487b7160e01b5f52605160045260245ffd5b91909180600114614b9c57600203614b6957614b9891611556565b905b565b50614ba691611517565b90614b9a56
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\x14\xD3V[a\0\x1D_5a\x03\x0CV[\x80c\x01\xFF\xC9\xA7\x14a\x03\x07W\x80c\x06\xFD\xDE\x03\x14a\x03\x02W\x80c\t^\xA7\xB3\x14a\x02\xFDW\x80c\x18\x16\r\xDD\x14a\x02\xF8W\x80c#\xB8r\xDD\x14a\x02\xF3W\x80c$\x8A\x9C\xA3\x14a\x02\xEEW\x80c//\xF1]\x14a\x02\xE9W\x80c1<\xE5g\x14a\x02\xE4W\x80c6D\xE5\x15\x14a\x02\xDFW\x80c6V\x8A\xBE\x14a\x02\xDAW\x80c:F\xB1\xA8\x14a\x02\xD5W\x80c@\xC1\x0F\x19\x14a\x02\xD0W\x80cB\x96lh\x14a\x02\xCBW\x80cK\xDD6\xCE\x14a\x02\xC6W\x80cK\xF5\xD7\xE9\x14a\x02\xC1W\x80cO\x1B\xFC\x9E\x14a\x02\xBCW\x80cX|\xDE\x1E\x14a\x02\xB7W\x80c\\\x19\xA9\\\x14a\x02\xB2W\x80co\xCF\xFFE\x14a\x02\xADW\x80cp\xA0\x821\x14a\x02\xA8W\x80cy\xCCg\x90\x14a\x02\xA3W\x80cz\x8C\xD1V\x14a\x02\x9EW\x80c~\xCE\xBE\0\x14a\x02\x99W\x80c\x83\xF1!\x1B\x14a\x02\x94W\x80c\x84&\xAD\xF2\x14a\x02\x8FW\x80c\x84L\x90&\x14a\x02\x8AW\x80c\x84\xB0\x19n\x14a\x02\x85W\x80c\x8AT%!\x14a\x02\x80W\x80c\x8D3C\xD6\x14a\x02{W\x80c\x8ES\x9E\x8C\x14a\x02vW\x80c\x90-U\xA5\x14a\x02qW\x80c\x91\xD1HT\x14a\x02lW\x80c\x91\xDD\xAD\xF4\x14a\x02gW\x80c\x95\xD8\x9BA\x14a\x02bW\x80c\x9A\xB2N\xB0\x14a\x02]W\x80c\x9B~\xF6K\x14a\x02XW\x80c\xA2\x17\xFD\xDF\x14a\x02SW\x80c\xA9\x05\x9C\xBB\x14a\x02NW\x80c\xAA\x08*\x9D\x14a\x02IW\x80c\xB0\xCA%>\x14a\x02DW\x80c\xBBMD6\x14a\x02?W\x80c\xC0*\xE7T\x14a\x02:W\x80c\xC3\xCD\xA5 \x14a\x025W\x80c\xD5\x05\xAC\xCF\x14a\x020W\x80c\xD5Gt\x1F\x14a\x02+W\x80c\xDDb\xED>\x14a\x02&Wc\xF1\x12~\xD8\x03a\0\x0EWa\x14\x9DV[a\x13\xB9V[a\x13XV[a\x13\x1EV[a\x12tV[a\x11\xB8V[a\x11\x83V[a\x11MV[a\x11\x18V[a\x10\xA6V[a\x10qV[a\x10\x01V[a\x0F\x8AV[a\x0FUV[a\x0F V[a\x0E\xBDV[a\x0E\x88V[a\x0E\x11V[a\r\xDCV[a\rxV[a\r\rV[a\x0B\xC8V[a\x0B\x93V[a\x0B:V[a\x0B\x05V[a\n\xD0V[a\n\x9CV[a\ngV[a\n2V[a\t\xD4V[a\t\x9FV[a\t*V[a\x08\xB9V[a\x08\x84V[a\x08QV[a\x07\xFFV[a\x07\xC9V[a\x07\x95V[a\x07`V[a\x07+V[a\x06\xCFV[a\x06hV[a\x05\xCCV[a\x05]V[a\x05\x05V[a\x04CV[a\x03\x94V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[c\xFF\xFF\xFF\xFF`\xE0\x1B\x16\x90V[a\x035\x81a\x03 V[\x03a\x03<WV[_\x80\xFD[\x90P5\x90a\x03M\x82a\x03,V[V[\x90` \x82\x82\x03\x12a\x03hWa\x03e\x91_\x01a\x03@V[\x90V[a\x03\x1CV[\x15\x15\x90V[a\x03{\x90a\x03mV[\x90RV[\x91\x90a\x03\x92\x90_` \x85\x01\x94\x01\x90a\x03rV[V[4a\x03\xC4Wa\x03\xC0a\x03\xAFa\x03\xAA6`\x04a\x03OV[a\x15pV[a\x03\xB7a\x03\x12V[\x91\x82\x91\x82a\x03\x7FV[\x03\x90\xF3[a\x03\x18V[_\x91\x03\x12a\x03\xD3WV[a\x03\x1CV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x04\x19a\x04\"` \x93a\x04'\x93a\x04\x10\x81a\x03\xD8V[\x93\x84\x80\x93a\x03\xDCV[\x95\x86\x91\x01a\x03\xE5V[a\x03\xF0V[\x01\x90V[a\x04@\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03\xFAV[\x90V[4a\x04sWa\x04S6`\x04a\x03\xC9V[a\x04oa\x04^a\x17\tV[a\x04fa\x03\x12V[\x91\x82\x91\x82a\x04+V[\x03\x90\xF3[a\x03\x18V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\x8C\x90a\x04xV[\x90V[a\x04\x98\x81a\x04\x83V[\x03a\x04\x9FWV[_\x80\xFD[\x90P5\x90a\x04\xB0\x82a\x04\x8FV[V[\x90V[a\x04\xBE\x81a\x04\xB2V[\x03a\x04\xC5WV[_\x80\xFD[\x90P5\x90a\x04\xD6\x82a\x04\xB5V[V[\x91\x90`@\x83\x82\x03\x12a\x05\0W\x80a\x04\xF4a\x04\xFD\x92_\x86\x01a\x04\xA3V[\x93` \x01a\x04\xC9V[\x90V[a\x03\x1CV[4a\x056Wa\x052a\x05!a\x05\x1B6`\x04a\x04\xD8V[\x90a\x17\x1FV[a\x05)a\x03\x12V[\x91\x82\x91\x82a\x03\x7FV[\x03\x90\xF3[a\x03\x18V[a\x05D\x90a\x04\xB2V[\x90RV[\x91\x90a\x05[\x90_` \x85\x01\x94\x01\x90a\x05;V[V[4a\x05\x8DWa\x05m6`\x04a\x03\xC9V[a\x05\x89a\x05xa\x17kV[a\x05\x80a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[\x90\x91``\x82\x84\x03\x12a\x05\xC7Wa\x05\xC4a\x05\xAD\x84_\x85\x01a\x04\xA3V[\x93a\x05\xBB\x81` \x86\x01a\x04\xA3V[\x93`@\x01a\x04\xC9V[\x90V[a\x03\x1CV[4a\x05\xFDWa\x05\xF9a\x05\xE8a\x05\xE26`\x04a\x05\x92V[\x91a\x17\x81V[a\x05\xF0a\x03\x12V[\x91\x82\x91\x82a\x03\x7FV[\x03\x90\xF3[a\x03\x18V[\x90V[a\x06\x0E\x81a\x06\x02V[\x03a\x06\x15WV[_\x80\xFD[\x90P5\x90a\x06&\x82a\x06\x05V[V[\x90` \x82\x82\x03\x12a\x06AWa\x06>\x91_\x01a\x06\x19V[\x90V[a\x03\x1CV[a\x06O\x90a\x06\x02V[\x90RV[\x91\x90a\x06f\x90_` \x85\x01\x94\x01\x90a\x06FV[V[4a\x06\x98Wa\x06\x94a\x06\x83a\x06~6`\x04a\x06(V[a\x17\xFAV[a\x06\x8Ba\x03\x12V[\x91\x82\x91\x82a\x06SV[\x03\x90\xF3[a\x03\x18V[\x91\x90`@\x83\x82\x03\x12a\x06\xC5W\x80a\x06\xB9a\x06\xC2\x92_\x86\x01a\x06\x19V[\x93` \x01a\x04\xA3V[\x90V[a\x03\x1CV[_\x01\x90V[4a\x06\xFEWa\x06\xE8a\x06\xE26`\x04a\x06\x9DV[\x90a\x18FV[a\x06\xF0a\x03\x12V[\x80a\x06\xFA\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[`\xFF\x16\x90V[a\x07\x12\x90a\x07\x03V[\x90RV[\x91\x90a\x07)\x90_` \x85\x01\x94\x01\x90a\x07\tV[V[4a\x07[Wa\x07;6`\x04a\x03\xC9V[a\x07Wa\x07Fa\x18uV[a\x07Na\x03\x12V[\x91\x82\x91\x82a\x07\x16V[\x03\x90\xF3[a\x03\x18V[4a\x07\x90Wa\x07p6`\x04a\x03\xC9V[a\x07\x8Ca\x07{a\x18\x8BV[a\x07\x83a\x03\x12V[\x91\x82\x91\x82a\x06SV[\x03\x90\xF3[a\x03\x18V[4a\x07\xC4Wa\x07\xAEa\x07\xA86`\x04a\x06\x9DV[\x90a\x18\x9FV[a\x07\xB6a\x03\x12V[\x80a\x07\xC0\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[4a\x07\xFAWa\x07\xF6a\x07\xE5a\x07\xDF6`\x04a\x04\xD8V[\x90a\x19PV[a\x07\xEDa\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\x08.Wa\x08\x18a\x08\x126`\x04a\x04\xD8V[\x90a\x1A\xD7V[a\x08 a\x03\x12V[\x80a\x08*\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[\x90` \x82\x82\x03\x12a\x08LWa\x08I\x91_\x01a\x04\xC9V[\x90V[a\x03\x1CV[4a\x08\x7FWa\x08ia\x08d6`\x04a\x083V[a\x1A\xE3V[a\x08qa\x03\x12V[\x80a\x08{\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[4a\x08\xB4Wa\x08\x946`\x04a\x03\xC9V[a\x08\xB0a\x08\x9Fa\x1BHV[a\x08\xA7a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\x08\xE9Wa\x08\xC96`\x04a\x03\xC9V[a\x08\xE5a\x08\xD4a\x1C\x07V[a\x08\xDCa\x03\x12V[\x91\x82\x91\x82a\x04+V[\x03\x90\xF3[a\x03\x18V[\x90V[\x90V[a\t\x08a\t\x03a\t\r\x92a\x08\xEEV[a\x08\xF1V[a\x04\xB2V[\x90V[a\t\x1Cbv\xA7\0a\x08\xF4V[\x90V[a\t'a\t\x10V[\x90V[4a\tZWa\t:6`\x04a\x03\xC9V[a\tVa\tEa\t\x1FV[a\tMa\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[\x90` \x82\x82\x03\x12a\txWa\tu\x91_\x01a\x04\xA3V[\x90V[a\x03\x1CV[a\t\x86\x90a\x04\x83V[\x90RV[\x91\x90a\t\x9D\x90_` \x85\x01\x94\x01\x90a\t}V[V[4a\t\xCFWa\t\xCBa\t\xBAa\t\xB56`\x04a\t_V[a\x1C\xA3V[a\t\xC2a\x03\x12V[\x91\x82\x91\x82a\t\x8AV[\x03\x90\xF3[a\x03\x18V[4a\n\x02Wa\t\xECa\t\xE76`\x04a\t_V[a\x1C\xC2V[a\t\xF4a\x03\x12V[\x80a\t\xFE\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[c\xFF\xFF\xFF\xFF\x16\x90V[a\n\x19\x90a\n\x07V[\x90RV[\x91\x90a\n0\x90_` \x85\x01\x94\x01\x90a\n\x10V[V[4a\nbWa\n^a\nMa\nH6`\x04a\t_V[a\x1C\xD9V[a\nUa\x03\x12V[\x91\x82\x91\x82a\n\x1DV[\x03\x90\xF3[a\x03\x18V[4a\n\x97Wa\n\x93a\n\x82a\n}6`\x04a\t_V[a\x1D\x04V[a\n\x8Aa\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\n\xCBWa\n\xB5a\n\xAF6`\x04a\x04\xD8V[\x90a\x1E9V[a\n\xBDa\x03\x12V[\x80a\n\xC7\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[4a\x0B\0Wa\n\xE06`\x04a\x03\xC9V[a\n\xFCa\n\xEBa\x1EEV[a\n\xF3a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\x0B5Wa\x0B1a\x0B a\x0B\x1B6`\x04a\t_V[a\x1E\xBDV[a\x0B(a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\x0BjWa\x0BJ6`\x04a\x03\xC9V[a\x0Bfa\x0BUa\x1E\xD2V[a\x0B]a\x03\x12V[\x91\x82\x91\x82a\x03\x7FV[\x03\x90\xF3[a\x03\x18V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[4a\x0B\xC3Wa\x0B\xA36`\x04a\x03\xC9V[a\x0B\xBFa\x0B\xAEa\x0BoV[a\x0B\xB6a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\x0B\xF6Wa\x0B\xE0a\x0B\xDB6`\x04a\x083V[a \x9DV[a\x0B\xE8a\x03\x12V[\x80a\x0B\xF2\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[`\xFF`\xF8\x1B\x16\x90V[a\x0C\r\x90a\x0B\xFBV[\x90RV[Q\x90V[` \x91\x81R\x01\x90V[` \x01\x90V[a\x0C-\x90a\x04\xB2V[\x90RV[\x90a\x0C>\x81` \x93a\x0C$V[\x01\x90V[` \x01\x90V[\x90a\x0Cea\x0C_a\x0CX\x84a\x0C\x11V[\x80\x93a\x0C\x15V[\x92a\x0C\x1EV[\x90_[\x81\x81\x10a\x0CuWPPP\x90V[\x90\x91\x92a\x0C\x8Ea\x0C\x88`\x01\x92\x86Qa\x0C1V[\x94a\x0CBV[\x91\x01\x91\x90\x91a\x0ChV[\x93\x95\x91\x94a\x0C\xE9a\x0C\xDEa\x0C\xFD\x95a\x0C\xD0a\x0C\xF3\x95a\r\n\x9C\x9Aa\x0C\xC3`\xE0\x8C\x01\x92_\x8D\x01\x90a\x0C\x04V[\x8A\x82\x03` \x8C\x01Ra\x03\xFAV[\x90\x88\x82\x03`@\x8A\x01Ra\x03\xFAV[\x97``\x87\x01\x90a\x05;V[`\x80\x85\x01\x90a\t}V[`\xA0\x83\x01\x90a\x06FV[`\xC0\x81\x84\x03\x91\x01Ra\x0CHV[\x90V[4a\rDWa\r\x1D6`\x04a\x03\xC9V[a\r@a\r(a!%V[\x93a\r7\x97\x95\x97\x93\x91\x93a\x03\x12V[\x97\x88\x97\x88a\x0C\x98V[\x03\x90\xF3[a\x03\x18V[\x7F\x84\xFEt\xC7\x1A(\xB6\x9A\xA9`Hl\xA0\xE8\xC1A\x8C\x86\xE9\xEA,\xD6\xB5\x84\x9B\x95\xE2\xC8\xF4\x07\xA6t\x90V[a\rua\rIV[\x90V[4a\r\xA8Wa\r\x886`\x04a\x03\xC9V[a\r\xA4a\r\x93a\rmV[a\r\x9Ba\x03\x12V[\x91\x82\x91\x82a\x06SV[\x03\x90\xF3[a\x03\x18V[\x7F\x9B\x12\xE0\xC5p~II\x15\xE5\x8B\x05d\xF1\x8A\xAA\xD9\xB7J\xC6\x9B\xFC\x81Z\x1E\xDA\xDC\x8EK\xD02\xEB\x90V[a\r\xD9a\r\xADV[\x90V[4a\x0E\x0CWa\r\xEC6`\x04a\x03\xC9V[a\x0E\x08a\r\xF7a\r\xD1V[a\r\xFFa\x03\x12V[\x91\x82\x91\x82a\x06SV[\x03\x90\xF3[a\x03\x18V[4a\x0EAWa\x0E=a\x0E,a\x0E'6`\x04a\x083V[a!\xAFV[a\x0E4a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[\x90V[a\x0E]a\x0EXa\x0Eb\x92a\x0EFV[a\x08\xF1V[a\x04\xB2V[\x90V[a\x0Ezk\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x0EIV[\x90V[a\x0E\x85a\x0EeV[\x90V[4a\x0E\xB8Wa\x0E\x986`\x04a\x03\xC9V[a\x0E\xB4a\x0E\xA3a\x0E}V[a\x0E\xABa\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\x0E\xEEWa\x0E\xEAa\x0E\xD9a\x0E\xD36`\x04a\x06\x9DV[\x90a\"\x1DV[a\x0E\xE1a\x03\x12V[\x91\x82\x91\x82a\x03\x7FV[\x03\x90\xF3[a\x03\x18V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x0F\x07\x90a\x0E\xF3V[\x90RV[\x91\x90a\x0F\x1E\x90_` \x85\x01\x94\x01\x90a\x0E\xFEV[V[4a\x0FPWa\x0F06`\x04a\x03\xC9V[a\x0FLa\x0F;a\"KV[a\x0FCa\x03\x12V[\x91\x82\x91\x82a\x0F\x0BV[\x03\x90\xF3[a\x03\x18V[4a\x0F\x85Wa\x0Fe6`\x04a\x03\xC9V[a\x0F\x81a\x0Fpa\"_V[a\x0Fxa\x03\x12V[\x91\x82\x91\x82a\x04+V[\x03\x90\xF3[a\x03\x18V[4a\x0F\xBAWa\x0F\xB6a\x0F\xA5a\x0F\xA06`\x04a\t_V[a\"uV[a\x0F\xADa\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[\x90V[a\x0F\xD6a\x0F\xD1a\x0F\xDB\x92a\x0F\xBFV[a\x08\xF1V[a\x04\xB2V[\x90V[a\x0F\xF3k\x02\xE8vi\xC3\x08sj\x04\0\0\0a\x0F\xC2V[\x90V[a\x0F\xFEa\x0F\xDEV[\x90V[4a\x101Wa\x10\x116`\x04a\x03\xC9V[a\x10-a\x10\x1Ca\x0F\xF6V[a\x10$a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[\x90V[_\x1B\x90V[a\x10Ra\x10Ma\x10W\x92a\x106V[a\x109V[a\x06\x02V[\x90V[a\x10c_a\x10>V[\x90V[a\x10na\x10ZV[\x90V[4a\x10\xA1Wa\x10\x816`\x04a\x03\xC9V[a\x10\x9Da\x10\x8Ca\x10fV[a\x10\x94a\x03\x12V[\x91\x82\x91\x82a\x06SV[\x03\x90\xF3[a\x03\x18V[4a\x10\xD7Wa\x10\xD3a\x10\xC2a\x10\xBC6`\x04a\x04\xD8V[\x90a\"\xA4V[a\x10\xCAa\x03\x12V[\x91\x82\x91\x82a\x03\x7FV[\x03\x90\xF3[a\x03\x18V[\x1C\x90V[\x90V[a\x10\xF3\x90`\x08a\x10\xF8\x93\x02a\x10\xDCV[a\x10\xE0V[\x90V[\x90a\x11\x06\x91Ta\x10\xE3V[\x90V[a\x11\x15`\x0C_\x90a\x10\xFBV[\x90V[4a\x11HWa\x11(6`\x04a\x03\xC9V[a\x11Da\x113a\x11\tV[a\x11;a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\x11~Wa\x11za\x11ia\x11c6`\x04a\x04\xD8V[\x90a\"\xC6V[a\x11qa\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\x11\xB3Wa\x11\xAFa\x11\x9Ea\x11\x996`\x04a\t_V[a\"\xDCV[a\x11\xA6a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[4a\x11\xE8Wa\x11\xC86`\x04a\x03\xC9V[a\x11\xE4a\x11\xD3a\"\xF1V[a\x11\xDBa\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[a\x11\xF6\x81a\x07\x03V[\x03a\x11\xFDWV[_\x80\xFD[\x90P5\x90a\x12\x0E\x82a\x11\xEDV[V[\x90\x91`\xC0\x82\x84\x03\x12a\x12oWa\x12(\x83_\x84\x01a\x04\xA3V[\x92a\x126\x81` \x85\x01a\x04\xC9V[\x92a\x12D\x82`@\x83\x01a\x04\xC9V[\x92a\x12la\x12U\x84``\x85\x01a\x12\x01V[\x93a\x12c\x81`\x80\x86\x01a\x06\x19V[\x93`\xA0\x01a\x06\x19V[\x90V[a\x03\x1CV[4a\x12\xA9Wa\x12\x93a\x12\x876`\x04a\x12\x10V[\x94\x93\x90\x93\x92\x91\x92a#qV[a\x12\x9Ba\x03\x12V[\x80a\x12\xA5\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[`\xE0\x81\x83\x03\x12a\x13\x19Wa\x12\xC4\x82_\x83\x01a\x04\xA3V[\x92a\x12\xD2\x83` \x84\x01a\x04\xA3V[\x92a\x12\xE0\x81`@\x85\x01a\x04\xC9V[\x92a\x12\xEE\x82``\x83\x01a\x04\xC9V[\x92a\x13\x16a\x12\xFF\x84`\x80\x85\x01a\x12\x01V[\x93a\x13\r\x81`\xA0\x86\x01a\x06\x19V[\x93`\xC0\x01a\x06\x19V[\x90V[a\x03\x1CV[4a\x13SWa\x13=a\x1316`\x04a\x12\xAEV[\x95\x94\x90\x94\x93\x91\x93a$\xC5V[a\x13Ea\x03\x12V[\x80a\x13O\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[4a\x13\x87Wa\x13qa\x13k6`\x04a\x06\x9DV[\x90a%\xE3V[a\x13ya\x03\x12V[\x80a\x13\x83\x81a\x06\xCAV[\x03\x90\xF3[a\x03\x18V[\x91\x90`@\x83\x82\x03\x12a\x13\xB4W\x80a\x13\xA8a\x13\xB1\x92_\x86\x01a\x04\xA3V[\x93` \x01a\x04\xA3V[\x90V[a\x03\x1CV[4a\x13\xEAWa\x13\xE6a\x13\xD5a\x13\xCF6`\x04a\x13\x8CV[\x90a&\x05V[a\x13\xDDa\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xF3[a\x03\x18V[a\x13\xF8\x81a\n\x07V[\x03a\x13\xFFWV[_\x80\xFD[\x90P5\x90a\x14\x10\x82a\x13\xEFV[V[\x91\x90`@\x83\x82\x03\x12a\x14:W\x80a\x14.a\x147\x92_\x86\x01a\x04\xA3V[\x93` \x01a\x14\x03V[\x90V[a\x03\x1CV[a\x14H\x90a\x0E\xF3V[\x90RV[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x14`\x90a\x14LV[\x90RV[\x90` \x80a\x14\x86\x93a\x14|_\x82\x01Q_\x86\x01\x90a\x14?V[\x01Q\x91\x01\x90a\x14WV[V[\x91\x90a\x14\x9B\x90_`@\x85\x01\x94\x01\x90a\x14dV[V[4a\x14\xCEWa\x14\xCAa\x14\xB9a\x14\xB36`\x04a\x14\x12V[\x90a&sV[a\x14\xC1a\x03\x12V[\x91\x82\x91\x82a\x14\x88V[\x03\x90\xF3[a\x03\x18V[_\x80\xFD[_\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x14\xFBa\x15\x01\x91a\x14LV[\x91a\x14LV[\x01\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x15\x12WV[a\x14\xDBV[\x90a\x15*\x91a\x15$a\x14\xD7V[Pa\x14\xEFV[\x90V[a\x159a\x15?\x91a\x14LV[\x91a\x14LV[\x90\x03\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x15QWV[a\x14\xDBV[\x90a\x15i\x91a\x15ca\x14\xD7V[Pa\x15-V[\x90V[_\x90V[a\x15xa\x15lV[P\x80a\x15\x93a\x15\x8Dcye\xDB\x0B`\xE0\x1Ba\x03 V[\x91a\x03 V[\x14\x90\x81\x15a\x15\xA0W[P\x90V[a\x15\xAA\x91Pa&\x89V[_a\x15\x9CV[``\x90V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x01`\x02\x83\x04\x92\x16\x80\x15a\x15\xE9W[` \x83\x10\x14a\x15\xE4WV[a\x15\xB5V[\x91`\x7F\x16\x91a\x15\xD9V[` \x91\x81R\x01\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a\x16\x1Fa\x16\x18\x83a\x15\xC9V[\x80\x94a\x15\xF3V[\x91`\x01\x81\x16\x90\x81_\x14a\x16vWP`\x01\x14a\x16:W[PPPV[a\x16G\x91\x92\x93\x94Pa\x15\xFCV[\x91_\x92[\x81\x84\x10a\x16^WPP\x01\x90_\x80\x80a\x165V[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a\x16KV[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a\x165V[\x90a\x16\x9B\x91a\x16\x05V[\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x16\xBC\x90a\x03\xF0V[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x16\xD6W`@RV[a\x16\x9EV[\x90a\x16\xFBa\x16\xF4\x92a\x16\xEBa\x03\x12V[\x93\x84\x80\x92a\x16\x91V[\x03\x83a\x16\xB2V[V[a\x17\x06\x90a\x16\xDBV[\x90V[a\x17\x11a\x15\xB0V[Pa\x17\x1C`\x03a\x16\xFDV[\x90V[a\x17<\x91a\x17+a\x15lV[Pa\x174a&\xAFV[\x91\x90\x91a&\xBCV[`\x01\x90V[_\x90V[_\x1C\x90V[a\x17Va\x17[\x91a\x17EV[a\x10\xE0V[\x90V[a\x17h\x90Ta\x17JV[\x90V[a\x17sa\x17AV[Pa\x17~`\x02a\x17^V[\x90V[\x91a\x17\xAB\x92a\x17\x8Ea\x15lV[Pa\x17\xA3a\x17\x9Aa&\xAFV[\x82\x90\x84\x91a'\x0CV[\x91\x90\x91a'\x98V[`\x01\x90V[_\x90V[a\x17\xBD\x90a\x06\x02V[\x90V[\x90a\x17\xCA\x90a\x17\xB4V[_R` R`@_ \x90V[\x90V[a\x17\xE5a\x17\xEA\x91a\x17EV[a\x17\xD6V[\x90V[a\x17\xF7\x90Ta\x17\xD9V[\x90V[`\x01a\x18\x13a\x18\x19\x92a\x18\x0Ba\x17\xB0V[P`\x05a\x17\xC0V[\x01a\x17\xEDV[\x90V[\x90a\x187\x91a\x182a\x18-\x82a\x17\xFAV[a(5V[a\x189V[V[\x90a\x18C\x91a(\x8EV[PV[\x90a\x18P\x91a\x18\x1CV[V[_\x90V[\x90V[a\x18ma\x18ha\x18r\x92a\x18VV[a\x08\xF1V[a\x07\x03V[\x90V[a\x18}a\x18RV[Pa\x18\x88`\x12a\x18YV[\x90V[a\x18\x93a\x17\xB0V[Pa\x18\x9Ca):V[\x90V[\x90\x80a\x18\xBAa\x18\xB4a\x18\xAFa&\xAFV[a\x04\x83V[\x91a\x04\x83V[\x03a\x18\xCBWa\x18\xC8\x91a)\xF4V[PV[_c3K\xD9\x19`\xE1\x1B\x81R\x80a\x18\xE3`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[a\x18\xFBa\x18\xF6a\x19\0\x92a\x04xV[a\x08\xF1V[a\x04xV[\x90V[a\x19\x0C\x90a\x18\xE7V[\x90V[a\x19\x18\x90a\x19\x03V[\x90V[\x90a\x19%\x90a\x19\x0FV[_R` R`@_ \x90V[\x90V[a\x19Ha\x19Ca\x19M\x92a\x14LV[a\x08\xF1V[a\x04\xB2V[\x90V[a\x19\x87\x91a\x19|a\x19va\x19qa\x19\x82\x94a\x19ia\x17AV[P`\na\x19\x1BV[a\x191V[\x91a*\xD5V[\x90a+\xEAV[a\x194V[\x90V[\x90a\x19\xA4\x91a\x19\x9Fa\x19\x9Aa\r\xADV[a(5V[a\x1A\x0FV[V[a\x19\xBAa\x19\xB5a\x19\xBF\x92a\x106V[a\x08\xF1V[a\x04xV[\x90V[a\x19\xCB\x90a\x19\xA6V[\x90V[a\x19\xE2a\x19\xDDa\x19\xE7\x92a\x106V[a\x08\xF1V[a\x04\xB2V[\x90V[a\x19\xF9a\x19\xFF\x91\x93\x92\x93a\x04\xB2V[\x92a\x04\xB2V[\x82\x01\x80\x92\x11a\x1A\nWV[a\x14\xDBV[\x90\x81a\x1A+a\x1A%a\x1A _a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14a\x1A\xBBW\x80a\x1ACa\x1A=_a\x19\xCEV[\x91a\x04\xB2V[\x14a\x1A\x9FWa\x1AZa\x1ASa\x17kV[\x82\x90a\x19\xEAV[a\x1Asa\x1Ama\x1Aha\x0EeV[a\x04\xB2V[\x91a\x04\xB2V[\x11a\x1A\x83Wa\x1A\x81\x91a-\x11V[V[_c\x17~?\xC3`\xE0\x1B\x81R\x80a\x1A\x9B`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1A\xB7`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x1A\xD3`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[\x90a\x1A\xE1\x91a\x19\x8AV[V[\x80a\x1A\xF6a\x1A\xF0_a\x19\xCEV[\x91a\x04\xB2V[\x14a\x1B\x07Wa\x1B\x05\x903a-oV[V[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1B\x1F`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[a\x1B2a\x1B8\x91\x93\x92\x93a\x04\xB2V[\x92a\x04\xB2V[\x82\x03\x91\x82\x11a\x1BCWV[a\x14\xDBV[a\x1BPa\x17AV[Pa\x1Bja\x1B\\a\x0EeV[a\x1Bda\x17kV[\x90a\x1B#V[\x90V[\x90a\x1B\x80a\x1Bya\x03\x12V[\x92\x83a\x16\xB2V[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1B\xA0Wa\x1B\x9C` \x91a\x03\xF0V[\x01\x90V[a\x16\x9EV[\x90a\x1B\xB7a\x1B\xB2\x83a\x1B\x82V[a\x1BmV[\x91\x82RV[_\x7Fmode=blocknumber&from=default\0\0\0\x91\x01RV[a\x1B\xED`\x1Da\x1B\xA5V[\x90a\x1B\xFA` \x83\x01a\x1B\xBCV[V[a\x1C\x04a\x1B\xE3V[\x90V[a\x1C\x0Fa\x15\xB0V[Pa\x1C\x18a\"KV[a\x1C1a\x1C+a\x1C&a-\xCEV[a\x0E\xF3V[\x91a\x0E\xF3V[\x03a\x1CAWa\x1C>a\x1B\xFCV[\x90V[_c\x01\xBF\xC1\xC5`\xE6\x1B\x81R\x80a\x1CY`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[_\x90V[\x90a\x1Ck\x90a\x19\x0FV[_R` R`@_ \x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x1C\x8Ea\x1C\x93\x91a\x17EV[a\x1CwV[\x90V[a\x1C\xA0\x90Ta\x1C\x82V[\x90V[a\x1C\xBAa\x1C\xBF\x91a\x1C\xB2a\x1C]V[P`\ta\x1CaV[a\x1C\x96V[\x90V[a\x1C\xD3\x90a\x1C\xCEa&\xAFV[a.!V[V[_\x90V[a\x1C\xEB\x90a\x1C\xE5a\x1C\xD5V[Pa.\xACV[\x90V[\x90a\x1C\xF8\x90a\x19\x0FV[_R` R`@_ \x90V[a\x1D\x1Aa\x1D\x1F\x91a\x1D\x13a\x17AV[P_a\x1C\xEEV[a\x17^V[\x90V[\x90a\x1D<\x91a\x1D7a\x1D2a\rIV[a(5V[a\x1D>V[V[\x80a\x1DYa\x1DSa\x1DN_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14a\x1E\x1DW\x81a\x1Dqa\x1Dk_a\x19\xCEV[\x91a\x04\xB2V[\x14a\x1E\x01Wa\x1D\x87a\x1D\x81a\x1E\xD2V[\x15a\x03mV[a\x1D\xE5Wa\x1D\x96\x81\x83\x90a-oV[3\x90a\x1D\xE0a\x1D\xCEa\x1D\xC8\x7F\xBE\xF4\xF8\x1C\x18\x14\xC6A\xED\xE8^\xBA\xAC\xF1\x9D\x04\x8B,[U\x98\n\xDF\xA6\xEF\x0F\x95le\x135\xA2\x93a\x19\x0FV[\x93a\x19\x0FV[\x93a\x1D\xD7a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xA3V[_c\xB8\xB5\xCA-`\xE0\x1B\x81R\x80a\x1D\xFD`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x1E\x19`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x1E5`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[\x90a\x1EC\x91a\x1D\"V[V[a\x1EMa\x17AV[Pa\x1EX`\x0Ca\x17^V[a\x1Eja\x1Ed_a\x19\xCEV[\x91a\x04\xB2V[\x14\x80\x15a\x1E\x99W[a\x1E\x8DWa\x1E\x8Aa\x1E\x83`\x0Ca\x17^V[B\x90a\x1B#V[\x90V[a\x1E\x96_a\x19\xCEV[\x90V[PBa\x1E\xB6a\x1E\xB0a\x1E\xAB`\x0Ca\x17^V[a\x04\xB2V[\x91a\x04\xB2V[\x10\x15a\x1ErV[a\x1E\xCF\x90a\x1E\xC9a\x17AV[Pa.\xDBV[\x90V[a\x1E\xDAa\x15lV[Pa\x1E\xE5`\x0Ca\x17^V[a\x1E\xF7a\x1E\xF1_a\x19\xCEV[\x91a\x04\xB2V[\x14\x15\x80a\x1F\x02W[\x90V[PBa\x1F\x1Fa\x1F\x19a\x1F\x14`\x0Ca\x17^V[a\x04\xB2V[\x91a\x04\xB2V[\x10a\x1E\xFFV[a\x1F>\x90a\x1F9a\x1F4a\x10ZV[a(5V[a\x1F\xB8V[V[\x90a\x1FL_\x19\x91a\x109V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x1Fja\x1Fea\x1Fo\x92a\x04\xB2V[a\x08\xF1V[a\x04\xB2V[\x90V[\x90V[\x90a\x1F\x8Aa\x1F\x85a\x1F\x91\x92a\x1FVV[a\x1FrV[\x82Ta\x1F@V[\x90UV[\x91` a\x1F\xB6\x92\x94\x93a\x1F\xAF`@\x82\x01\x96_\x83\x01\x90a\x05;V[\x01\x90a\x05;V[V[\x80a\x1F\xCBa\x1F\xC5Ba\x04\xB2V[\x91a\x04\xB2V[\x11\x15a \x81W\x80a \x04a\x1F\xFE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xB2V[\x91a\x04\xB2V[\x11a eWa \x13`\x0Ca\x17^V[a \x1E\x82`\x0Ca\x1FuV[\x903\x90a K\x7F\xDDh\x96\xDC\xF1\xD4\xB3\x11\xCC\xA8}\xD1\x9B\xBB\xA2\xEA\x9C\xE2\xF8g\xC1V\x88x\xA0C\x8Af\xA1\xAF\xEE\xEC\x92a\x19\x0FV[\x92a `a Wa\x03\x12V[\x92\x83\x92\x83a\x1F\x95V[\x03\x90\xA2V[_c\xEFi\xAFe`\xE0\x1B\x81R\x80a }`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[_c\xA5e\x83S`\xE0\x1B\x81R\x80a \x99`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[a \xA6\x90a\x1F%V[V[_\x90V[``\x90V[a \xBA\x90a\x19\x03V[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a \xD5W` \x80\x91\x02\x01\x90V[a\x16\x9EV[\x90a \xECa \xE7\x83a \xBDV[a\x1BmV[\x91\x82RV[6\x907V[\x90a!\x1Ba!\x03\x83a \xDAV[\x92` \x80a!\x11\x86\x93a \xBDV[\x92\x01\x91\x03\x90a \xF1V[V[`\x0F`\xF8\x1B\x90V[a!-a \xA8V[Pa!6a\x15\xB0V[Pa!?a\x15\xB0V[Pa!Ha\x17AV[Pa!Qa\x1C]V[Pa!Za\x17\xB0V[Pa!ca \xACV[Pa!la.\xF3V[\x90a!ua/3V[\x90F\x90a!\x810a \xB1V[\x90a!\x8B_a\x10>V[\x90a!\x9Da!\x98_a\x19\xCEV[a \xF6V[\x90a!\xA6a!\x1DV[\x96\x95\x94\x93\x92\x91\x90V[a!\xD8a!\xDD\x91a!\xBEa\x17AV[Pa!\xD2a!\xCC`\x0Ba\x191V[\x91a*\xD5V[\x90a+\xEAV[a\x194V[\x90V[\x90a!\xEA\x90a\x19\x0FV[_R` R`@_ \x90V[`\xFF\x16\x90V[a\"\x08a\"\r\x91a\x17EV[a!\xF6V[\x90V[a\"\x1A\x90Ta!\xFCV[\x90V[a\"D\x91_a\"9a\"?\x93a\"1a\x15lV[P`\x05a\x17\xC0V[\x01a!\xE0V[a\"\x10V[\x90V[_\x90V[a\"Sa\"GV[Pa\"\\a-\xCEV[\x90V[a\"ga\x15\xB0V[Pa\"r`\x04a\x16\xFDV[\x90V[a\"\x9Ca\"\x97a\"\x92a\"\xA1\x93a\"\x8Aa\x17AV[P`\na\x19\x1BV[a\x191V[a/sV[a\x194V[\x90V[a\"\xC1\x91a\"\xB0a\x15lV[Pa\"\xB9a&\xAFV[\x91\x90\x91a'\x98V[`\x01\x90V[\x90a\"\xD9\x91a\"\xD3a\x17AV[Pa\x19PV[\x90V[a\"\xEE\x90a\"\xE8a\x17AV[Pa\"uV[\x90V[a\"\xF9a\x17AV[Pa#\x02a\x17kV[\x90V[\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF\x90V[a#^a#e\x94a#T``\x94\x98\x97\x95a#J`\x80\x86\x01\x9A_\x87\x01\x90a\x06FV[` \x85\x01\x90a\t}V[`@\x83\x01\x90a\x05;V[\x01\x90a\x05;V[V[` \x01\x90V[Q\x90V[\x93\x95\x94\x90\x92\x91\x95Ba#\x8Ba#\x85\x89a\x04\xB2V[\x91a\x04\xB2V[\x11a$\x04W\x91a#\xF6\x91a#\xFD\x93a#\xEDa$\x02\x98\x99a#\xD5a#\xACa#\x05V[a#\xC6\x8B\x93\x8Ba#\xBAa\x03\x12V[\x95\x86\x94` \x86\x01a#)V[` \x82\x01\x81\x03\x82R\x03\x82a\x16\xB2V[a#\xE7a#\xE1\x82a#mV[\x91a#gV[ a/\xE8V[\x92\x90\x91\x92a0\x05V[\x91\x82a0OV[a.!V[V[a$\x1F\x87_\x91\x82\x91c#A\xD7\x87`\xE1\x1B\x83R`\x04\x83\x01a\x05HV[\x03\x90\xFD[\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x90V[\x91\x94a$\x8Fa$\x99\x92\x98\x97\x95a$\x85`\xA0\x96a${a$\xA0\x9Aa$q`\xC0\x8A\x01\x9E_\x8B\x01\x90a\x06FV[` \x89\x01\x90a\t}V[`@\x87\x01\x90a\t}V[``\x85\x01\x90a\x05;V[`\x80\x83\x01\x90a\x05;V[\x01\x90a\x05;V[V[\x91` a$\xC3\x92\x94\x93a$\xBC`@\x82\x01\x96_\x83\x01\x90a\t}V[\x01\x90a\t}V[V[\x96\x95\x91\x93\x92\x94\x90\x94Ba$\xE0a$\xDA\x83a\x04\xB2V[\x91a\x04\xB2V[\x11a%\x9AW\x90a%Ia%R\x94\x93\x92a%1a$\xFAa$#V[a%\"\x8C\x80\x94\x8C\x91a%\x0C\x8D\x91a0\xA1V[\x91\x92a%\x16a\x03\x12V[\x97\x88\x96` \x88\x01a$GV[` \x82\x01\x81\x03\x82R\x03\x82a\x16\xB2V[a%Ca%=\x82a#mV[\x91a#gV[ a/\xE8V[\x92\x90\x91\x92a0\x05V[\x80a%ea%_\x87a\x04\x83V[\x91a\x04\x83V[\x03a%zWPa%x\x92\x93\x91\x90\x91a&\xBCV[V[\x84\x90a%\x96_\x92\x83\x92c%\xC0\x07#`\xE1\x1B\x84R`\x04\x84\x01a$\xA2V[\x03\x90\xFD[a%\xB5\x90_\x91\x82\x91c1<\x89\x81`\xE1\x1B\x83R`\x04\x83\x01a\x05HV[\x03\x90\xFD[\x90a%\xD4\x91a%\xCFa%\xCA\x82a\x17\xFAV[a(5V[a%\xD6V[V[\x90a%\xE0\x91a)\xF4V[PV[\x90a%\xED\x91a%\xB9V[V[\x90a%\xF9\x90a\x19\x0FV[_R` R`@_ \x90V[a&*\x91a& a&%\x92a&\x18a\x17AV[P`\x01a%\xEFV[a\x1C\xEEV[a\x17^V[\x90V[a&7`@a\x1BmV[\x90V[_\x90V[_\x90V[a&Ja&-V[\x90` \x80\x83a&Wa&:V[\x81R\x01a&ba&>V[\x81RPPV[a&pa&BV[\x90V[\x90a&\x86\x91a&\x80a&hV[Pa0\xD4V[\x90V[a&\x91a\x15lV[Pa&\xABa&\xA5c\x01\xFF\xC9\xA7`\xE0\x1Ba\x03 V[\x91a\x03 V[\x14\x90V[a&\xB7a\x1C]V[P3\x90V[\x91a&\xCA\x92\x91`\x01\x92a0\xFCV[V[`@\x90a&\xF5a&\xFC\x94\x96\x95\x93\x96a&\xEB``\x84\x01\x98_\x85\x01\x90a\t}V[` \x83\x01\x90a\x05;V[\x01\x90a\x05;V[V[\x90a'\t\x91\x03a\x04\xB2V[\x90V[\x92\x91\x92a'\x1A\x81\x83\x90a&\x05V[\x90\x81a'/a')_\x19a\x04\xB2V[\x91a\x04\xB2V[\x10a'<W[PPP\x90PV[\x81a'Oa'I\x87a\x04\xB2V[\x91a\x04\xB2V[\x10a'uWa'l\x93\x94a'd\x91\x93\x92a&\xFEV[\x90_\x92a0\xFCV[\x80_\x80\x80a'5V[Pa'\x94\x84\x92\x91\x92_\x93\x84\x93c}\xC7\xA0\xD9`\xE1\x1B\x85R`\x04\x85\x01a&\xCCV[\x03\x90\xFD[\x91\x82a'\xB4a'\xAEa'\xA9_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14a(\x0EW\x81a'\xD4a'\xCEa'\xC9_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14a'\xE7Wa'\xE5\x92\x91\x90\x91a2\x0BV[V[a(\na'\xF3_a\x19\xC2V[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\t\x8AV[\x03\x90\xFD[a(1a(\x1A_a\x19\xC2V[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\t\x8AV[\x03\x90\xFD[a(G\x90a(Aa&\xAFV[\x90a2\xD8V[V[\x90a(U`\xFF\x91a\x109V[\x91\x81\x19\x16\x91\x16\x17\x90V[a(h\x90a\x03mV[\x90V[\x90V[\x90a(\x83a(~a(\x8A\x92a(_V[a(kV[\x82Ta(IV[\x90UV[a(\x96a\x15lV[Pa(\xABa(\xA5\x82\x84\x90a\"\x1DV[\x15a\x03mV[_\x14a)4Wa(\xD3`\x01a(\xCE_a(\xC6`\x05\x86\x90a\x17\xC0V[\x01\x85\x90a!\xE0V[a(nV[\x90a(\xDCa&\xAFV[\x90a)\x19a)\x13a)\r\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x95a\x17\xB4V[\x92a\x19\x0FV[\x92a\x19\x0FV[\x92a)\"a\x03\x12V[\x80a),\x81a\x06\xCAV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a)Ba\x17\xB0V[Pa)L0a \xB1V[a)~a)x\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\x83V[\x91a\x04\x83V[\x14\x80a)\xBAW[_\x14a)\xAFW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a)\xB7a3\x84V[\x90V[PFa)\xEEa)\xE8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\xB2V[\x91a\x04\xB2V[\x14a)\x85V[a)\xFCa\x15lV[Pa*\x08\x81\x83\x90a\"\x1DV[_\x14a*\x90Wa*/_a**_a*\"`\x05\x86\x90a\x17\xC0V[\x01\x85\x90a!\xE0V[a(nV[\x90a*8a&\xAFV[\x90a*ua*oa*i\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x95a\x17\xB4V[\x92a\x19\x0FV[\x92a\x19\x0FV[\x92a*~a\x03\x12V[\x80a*\x88\x81a\x06\xCAV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a*\xAAa*\xA5a*\xAF\x92a\x0E\xF3V[a\x08\xF1V[a\x04\xB2V[\x90V[\x91` a*\xD3\x92\x94\x93a*\xCC`@\x82\x01\x96_\x83\x01\x90a\x05;V[\x01\x90a\x0E\xFEV[V[a*\xDDa\"GV[Pa*\xE6a\"KV[\x81a*\xF9a*\xF3\x83a*\x96V[\x91a\x04\xB2V[\x10\x15a+\x0CWPa+\t\x90a4\x8DV[\x90V[\x90a+'_\x92\x83\x92cvi\xFC\x0F`\xE1\x1B\x84R`\x04\x84\x01a*\xB2V[\x03\x90\xFD[T\x90V[\x90V[a+Fa+Aa+K\x92a+/V[a\x08\xF1V[a\x04\xB2V[\x90V[\x90V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a+ha+m\x91a\x17EV[a+QV[\x90V[a+z\x90Ta+\\V[\x90V[\x90V[a+\x94a+\x8Fa+\x99\x92a+}V[a\x08\xF1V[a\x04\xB2V[\x90V[`0\x1C\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a+\xB9a+\xBE\x91a+\x9CV[a+\xA2V[\x90V[a+\xCB\x90Ta+\xADV[\x90V[a+\xE2a+\xDDa+\xE7\x92a\x106V[a\x08\xF1V[a\x14LV[\x90V[\x90a,>\x90a+\xF7a\x14\xD7V[Pa,\x03_\x84\x01a++V[a,\x0C_a\x19\xCEV[\x90\x80\x80a,\"a,\x1C`\x05a+2V[\x91a\x04\xB2V[\x11a,\x9FW[P\x90a,9_\x86\x01\x93\x91\x92\x93a+NV[a:\xE0V[\x80a,Qa,K_a\x19\xCEV[\x91a\x04\xB2V[\x14_\x14a,gWPPa,c_a+\xCEV[[\x90V[a,\x94_\x91a,\x8Fa,\x89\x84a,\x9A\x96\x01\x92a,\x83`\x01a+\x80V[\x90a\x1B#V[\x91a+NV[a:\xD6V[\x01a+\xC1V[a,dV[\x80a,\xADa,\xB3\x92\x91a7kV[\x90a\x1B#V[\x90\x83a,\xE5a,\xDFa,\xDA_a,\xD4\x81\x8C\x01a,\xCF\x89\x91a+NV[a:\xD6V[\x01a+pV[a\x0E\xF3V[\x91a\x0E\xF3V[\x10_\x14a,\xF6WP\x90[\x90_a,(V[\x91Pa-\x0C\x90a-\x06`\x01a+\x80V[\x90a\x19\xEAV[a,\xEFV[\x80a-,a-&a-!_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14a-HWa-F\x91a->_a\x19\xC2V[\x91\x90\x91a2\x0BV[V[a-ka-T_a\x19\xC2V[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\t\x8AV[\x03\x90\xFD[\x90\x81a-\x8Ba-\x85a-\x80_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14a-\xA7Wa-\xA5\x91\x90a-\x9E_a\x19\xC2V[\x90\x91a2\x0BV[V[a-\xCAa-\xB3_a\x19\xC2V[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\t\x8AV[\x03\x90\xFD[a-\xD6a\"GV[Pa-\xE0Ca4\x8DV[\x90V[\x90a-\xF4`\x01\x80`\xA0\x1B\x03\x91a\x109V[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a.\x16a.\x11a.\x1D\x92a\x19\x0FV[a-\xFEV[\x82Ta-\xE3V[\x90UV[\x90a.\xAA\x91a.\xA4a.2\x82a\x1C\xA3V[a.G\x84a.B`\t\x86\x90a\x1CaV[a.\x01V[\x82\x81\x85\x90a.\x87a.\x81a.{\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x95a\x19\x0FV[\x92a\x19\x0FV[\x92a\x19\x0FV[\x92a.\x90a\x03\x12V[\x80a.\x9A\x81a\x06\xCAV[\x03\x90\xA4\x92\x91a;oV[\x91a;\x87V[V[a.\xD3a.\xCEa.\xC9a.\xD8\x93a.\xC1a\x1C\xD5V[P`\na\x19\x1BV[a\x191V[a=5V[a=\xB4V[\x90V[a.\xED\x90a.\xE7a\x17AV[Pa>\x05V[\x90V[\x90V[a.\xFBa\x15\xB0V[Pa/0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a/*`\x06a.\xF0V[\x90a? V[\x90V[a/;a\x15\xB0V[Pa/p\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a/j`\x07a.\xF0V[\x90a? V[\x90V[a/{a\x14\xD7V[Pa/\x87_\x82\x01a++V[\x80a/\x9Aa/\x94_a\x19\xCEV[\x91a\x04\xB2V[\x14_\x14a/\xB0WPPa/\xAC_a+\xCEV[[\x90V[a/\xDD_\x91a/\xD8a/\xD2\x84a/\xE3\x96\x01\x92a/\xCC`\x01a+\x80V[\x90a\x1B#V[\x91a+NV[a:\xD6V[\x01a+\xC1V[a/\xADV[a0\x02\x90a/\xF4a\x17\xB0V[Pa/\xFDa):V[a?nV[\x90V[\x92a0 \x92a0)\x94a0\x16a\x1C]V[P\x92\x90\x91\x92a@4V[\x90\x92\x91\x92aA_V[\x90V[\x91` a0M\x92\x94\x93a0F`@\x82\x01\x96_\x83\x01\x90a\t}V[\x01\x90a\x05;V[V[a0X\x81a0\xA1V[\x91a0ka0e\x84a\x04\xB2V[\x91a\x04\xB2V[\x03a0tWPPV[a0\x8E_\x92\x83\x92c\x01\xD4\xB6#`\xE6\x1B\x84R`\x04\x84\x01a0,V[\x03\x90\xFD[`\x01a0\x9E\x91\x01a\x04\xB2V[\x90V[a0\xB5\x90a0\xADa\x17AV[P`\x08a\x1C\xEEV[a0\xD1a0\xC1\x82a\x17^V[\x91a0\xCB\x83a0\x92V[\x90a\x1FuV[\x90V[\x90a0\xF4a0\xEFa0\xF9\x93a0\xE7a&hV[P`\na\x19\x1BV[a\x191V[aB\xD5V[\x90V[\x90\x92\x81a1\x19a1\x13a1\x0E_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14a1\xE4W\x83a19a13a1._a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14a1\xBDWa1]\x83a1Xa1Q`\x01\x86\x90a%\xEFV[\x87\x90a\x1C\xEEV[a\x1FuV[a1gW[PPPV[\x91\x90\x91a1\xB2a1\xA0a1\x9A\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x93a\x19\x0FV[\x93a\x19\x0FV[\x93a1\xA9a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xA3_\x80\x80a1bV[a1\xE0a1\xC9_a\x19\xC2V[_\x91\x82\x91cJ\x14\x06\xB1`\xE1\x1B\x83R`\x04\x83\x01a\t\x8AV[\x03\x90\xFD[a2\x07a1\xF0_a\x19\xC2V[_\x91\x82\x91c\xE6\x02\xDF\x05`\xE0\x1B\x83R`\x04\x83\x01a\t\x8AV[\x03\x90\xFD[\x91\x82a2'a2!a2\x1C_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14\x15\x80a2\x92W[a2BW[a2@\x92\x91\x90\x91aB\xF6V[V[a2Ja\x1E\xD2V[\x80a2qW[\x15a24W_c6\xE2x\xFD`\xE2\x1B\x81R\x80a2m`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[Pa2\x8Da2\x87a2\x80a\rIV[3\x90a\"\x1DV[\x15a\x03mV[a2PV[P\x81a2\xAEa2\xA8a2\xA3_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14\x15a2/V[\x91` a2\xD6\x92\x94\x93a2\xCF`@\x82\x01\x96_\x83\x01\x90a\t}V[\x01\x90a\x06FV[V[\x90a2\xEDa2\xE7\x83\x83\x90a\"\x1DV[\x15a\x03mV[a2\xF5WPPV[a3\x0F_\x92\x83\x92c\xE2Q}?`\xE0\x1B\x84R`\x04\x84\x01a2\xB5V[\x03\x90\xFD[\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x90V[\x90\x95\x94\x92a3\x82\x94a3qa3{\x92a3g`\x80\x96a3]`\xA0\x88\x01\x9C_\x89\x01\x90a\x06FV[` \x87\x01\x90a\x06FV[`@\x85\x01\x90a\x06FV[``\x83\x01\x90a\x05;V[\x01\x90a\t}V[V[a3\x8Ca\x17\xB0V[Pa3\x95a3\x13V[a4\x0C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a3\xFD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Fa3\xE80a \xB1V[\x91a3\xF1a\x03\x12V[\x96\x87\x95` \x87\x01a37V[` \x82\x01\x81\x03\x82R\x03\x82a\x16\xB2V[a4\x1Ea4\x18\x82a#mV[\x91a#gV[ \x90V[\x90V[a49a44a4>\x92a4\"V[a\x08\xF1V[a\x07\x03V[\x90V[a4J\x90a4%V[\x90RV[\x91` a4o\x92\x94\x93a4h`@\x82\x01\x96_\x83\x01\x90a4AV[\x01\x90a\x05;V[V[a4\x85a4\x80a4\x8A\x92a\x04\xB2V[a\x08\xF1V[a\x0E\xF3V[\x90V[a4\x95a\"GV[P\x80a4\xAFa4\xA9e\xFF\xFF\xFF\xFF\xFF\xFFa*\x96V[\x91a\x04\xB2V[\x11a4\xC0Wa4\xBD\x90a4qV[\x90V[`0a4\xDC_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a4NV[\x03\x90\xFD[\x90V[a4\xF7a4\xF2a4\xFC\x92a4\xE0V[a\x08\xF1V[a\x04\xB2V[\x90V[\x90V[a5\x16a5\x11a5\x1B\x92a4\xFFV[a\x08\xF1V[a\x07\x03V[\x90V[a5=\x90a57a51a5B\x94a\x07\x03V[\x91a\x04\xB2V[\x90a\x10\xDCV[a\x04\xB2V[\x90V[\x90V[a5\\a5Wa5a\x92a5EV[a\x08\xF1V[a\x07\x03V[\x90V[\x1B\x90V[a5\x87\x90a5\x81a5{a5\x8C\x94a\x07\x03V[\x91a\x04\xB2V[\x90a5dV[a\x04\xB2V[\x90V[\x90V[a5\xA6a5\xA1a5\xAB\x92a5\x8FV[a\x08\xF1V[a\x04\xB2V[\x90V[\x90V[a5\xC5a5\xC0a5\xCA\x92a5\xAEV[a\x08\xF1V[a\x07\x03V[\x90V[\x90V[a5\xE4a5\xDFa5\xE9\x92a5\xCDV[a\x08\xF1V[a\x04\xB2V[\x90V[\x90V[a6\x03a5\xFEa6\x08\x92a5\xECV[a\x08\xF1V[a\x07\x03V[\x90V[\x90V[a6\"a6\x1Da6'\x92a6\x0BV[a\x08\xF1V[a\x04\xB2V[\x90V[\x90V[a6Aa6<a6F\x92a6*V[a\x08\xF1V[a\x07\x03V[\x90V[\x90V[a6`a6[a6e\x92a6IV[a\x08\xF1V[a\x04\xB2V[\x90V[\x90V[a6\x7Fa6za6\x84\x92a6hV[a\x08\xF1V[a\x07\x03V[\x90V[a6\x9Ba6\x96a6\xA0\x92a5\xECV[a\x08\xF1V[a\x04\xB2V[\x90V[\x90V[a6\xBAa6\xB5a6\xBF\x92a6\xA3V[a\x08\xF1V[a\x07\x03V[\x90V[a6\xD6a6\xD1a6\xDB\x92a6hV[a\x08\xF1V[a\x04\xB2V[\x90V[a6\xF2a6\xEDa6\xF7\x92a+}V[a\x08\xF1V[a\x07\x03V[\x90V[\x90V[a7\x11a7\x0Ca7\x16\x92a6\xFAV[a\x08\xF1V[a\x04\xB2V[\x90V[\x90a7$\x91\x02a\x04\xB2V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a7Ga7M\x91a\x04\xB2V[\x91a\x04\xB2V[\x90\x81\x15a7XW\x04\x90V[a7'V[\x90a7h\x91\x01a\x04\xB2V[\x90V[a7sa\x17AV[P\x80a7\x88a7\x82`\x01a+\x80V[\x91a\x04\xB2V[\x11\x15a:\xD3W\x80a9\x9Da9za9ja9Za9Ja9:a9*a9\x1Aa9\na8\xFAa8\xEA\x8Ba8\xE4a8\xDDa9\xA3\x9Fa8\xBDa8\xADa8\xCD\x92a7\xCF`\x01a+\x80V[\x90\x80a7\xE7a7\xE1`\x01`\x80\x1Ba4\xE3V[\x91a\x04\xB2V[\x10\x15a:\xA5W[\x80a8\na8\x04h\x01\0\0\0\0\0\0\0\0a5\x92V[\x91a\x04\xB2V[\x10\x15a:wW[\x80a8)a8#d\x01\0\0\0\0a5\xD0V[\x91a\x04\xB2V[\x10\x15a:IW[\x80a8Fa8@b\x01\0\0a6\x0EV[\x91a\x04\xB2V[\x10\x15a:\x1BW[\x80a8ba8\\a\x01\0a6LV[\x91a\x04\xB2V[\x10\x15a9\xEDW[\x80a8}a8w`\x10a6\x87V[\x91a\x04\xB2V[\x10\x15a9\xBFW[a8\x97a8\x91`\x04a6\xC2V[\x91a\x04\xB2V[\x10\x15a9\xA6W[a8\xA8`\x03a6\xFDV[a7\x19V[a8\xB7`\x01a6\xDEV[\x90a5\x1EV[a8\xC7\x81\x86a7;V[\x90a7]V[a8\xD7`\x01a6\xDEV[\x90a5\x1EV[\x80\x92a7;V[\x90a7]V[a8\xF4`\x01a6\xDEV[\x90a5\x1EV[a9\x04\x81\x8Ca7;V[\x90a7]V[a9\x14`\x01a6\xDEV[\x90a5\x1EV[a9$\x81\x8Aa7;V[\x90a7]V[a94`\x01a6\xDEV[\x90a5\x1EV[a9D\x81\x88a7;V[\x90a7]V[a9T`\x01a6\xDEV[\x90a5\x1EV[a9d\x81\x86a7;V[\x90a7]V[a9t`\x01a6\xDEV[\x90a5\x1EV[\x91a9\x97a9\x91a9\x8C\x85\x80\x94a7;V[a\x04\xB2V[\x91a\x04\xB2V[\x11aC\x86V[\x90a&\xFEV[\x90V[a9\xBA\x90a9\xB4`\x01a6\xDEV[\x90a5hV[a8\x9EV[a9\xD6a9\xE7\x91a9\xD0`\x04a6kV[\x90a5\x1EV[\x91a9\xE1`\x02a6\xA6V[\x90a5hV[\x90a8\x84V[a:\x04a:\x15\x91a9\xFE`\x08a6-V[\x90a5\x1EV[\x91a:\x0F`\x04a6kV[\x90a5hV[\x90a8iV[a:2a:C\x91a:,`\x10a5\xEFV[\x90a5\x1EV[\x91a:=`\x08a6-V[\x90a5hV[\x90a8MV[a:`a:q\x91a:Z` a5\xB1V[\x90a5\x1EV[\x91a:k`\x10a5\xEFV[\x90a5hV[\x90a80V[a:\x8Ea:\x9F\x91a:\x88`@a5HV[\x90a5\x1EV[\x91a:\x99` a5\xB1V[\x90a5hV[\x90a8\x11V[a:\xBCa:\xCD\x91a:\xB6`\x80a5\x02V[\x90a5\x1EV[\x91a:\xC7`@a5HV[\x90a5hV[\x90a7\xEEV[\x90V[_R` _ \x01\x90V[\x93\x91\x90\x92a:\xECa\x17AV[P[\x81a;\x01a:\xFB\x83a\x04\xB2V[\x91a\x04\xB2V[\x10\x15a;gWa;\x12\x82\x82\x90aC\xD2V[\x90a;(_a;\"\x88\x85\x90a:\xD6V[\x01a+pV[a;:a;4\x87a\x0E\xF3V[\x91a\x0E\xF3V[\x11_\x14a;JWP\x91[\x91a:\xEEV[\x92\x91Pa;a\x90a;[`\x01a+\x80V[\x90a\x19\xEAV[\x90a;DV[\x92PP\x91P\x90V[a;\x81\x90a;{a\x17AV[Pa\x1D\x04V[\x90V[\x90V[\x91\x90\x91\x80a;\x9Da;\x97\x85a\x04\x83V[\x91a\x04\x83V[\x14\x15\x80a=\x1BW[a;\xAFW[PPPV[\x80a;\xCAa;\xC4a;\xBF_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x03a<\x8BW[P\x81a;\xECa;\xE6a;\xE1_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x03a;\xF8W[\x80a;\xAAV[a<?a<2a<9\x92a<\x0E`\n\x86\x90a\x19\x1BV[\x90a<,a<&a< `\x01\x93aDkV[\x93a\x191V[\x91a;\x84V[\x90aD\xBEV[\x92\x90a\x194V[\x91a\x194V[\x91\x90\x91a<l\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x19\x0FV[\x92a<\x81a<xa\x03\x12V[\x92\x83\x92\x83a\x1F\x95V[\x03\x90\xA2_\x80a;\xF2V[a<\xCAa<\xD0a<\xC3a<\xA0`\n\x85\x90a\x19\x1BV[`\x02a<\xBDa<\xB7a<\xB1\x89aDkV[\x93a\x191V[\x91a;\x84V[\x90aD\xBEV[\x92\x90a\x194V[\x91a\x194V[\x91\x90\x91a<\xFD\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x19\x0FV[\x92a=\x12a=\ta\x03\x12V[\x92\x83\x92\x83a\x1F\x95V[\x03\x90\xA2_a;\xD0V[P\x81a=/a=)_a\x19\xCEV[\x91a\x04\xB2V[\x11a;\xA5V[_a=I\x91a=Ba\x17AV[P\x01a++V[\x90V[a=`a=[a=e\x92a\n\x07V[a\x08\xF1V[a\x04\xB2V[\x90V[a=q\x90a5\xB1V[\x90RV[\x91` a=\x96\x92\x94\x93a=\x8F`@\x82\x01\x96_\x83\x01\x90a=hV[\x01\x90a\x05;V[V[a=\xACa=\xA7a=\xB1\x92a\x04\xB2V[a\x08\xF1V[a\n\x07V[\x90V[a=\xBCa\x1C\xD5V[P\x80a=\xD4a=\xCEc\xFF\xFF\xFF\xFFa=LV[\x91a\x04\xB2V[\x11a=\xE5Wa=\xE2\x90a=\x98V[\x90V[` a>\x01_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a=uV[\x03\x90\xFD[a>\x1Ca>!\x91a>\x14a\x17AV[P`\x08a\x1C\xEEV[a\x17^V[\x90V[\x90V[a>;a>6a>@\x92a>$V[a\x109V[a\x06\x02V[\x90V[a>M`\xFFa>'V[\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a>sa>l\x83a\x15\xC9V[\x80\x94a\x15\xF3V[\x91`\x01\x81\x16\x90\x81_\x14a>\xCAWP`\x01\x14a>\x8EW[PPPV[a>\x9B\x91\x92\x93\x94Pa>PV[\x91_\x92[\x81\x84\x10a>\xB2WPP\x01\x90_\x80\x80a>\x89V[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a>\x9FV[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a>\x89V[\x90a>\xEF\x91a>YV[\x90V[\x90a?\x12a?\x0B\x92a?\x02a\x03\x12V[\x93\x84\x80\x92a>\xE5V[\x03\x83a\x16\xB2V[V[a?\x1D\x90a>\xF2V[\x90V[\x90a?)a\x15\xB0V[Pa?3\x82a\x17\xB4V[a?La?Fa?Aa>CV[a\x06\x02V[\x91a\x06\x02V[\x14\x15_\x14a?aWPa?^\x90aEHV[\x90V[a?k\x91Pa?\x14V[\x90V[`B\x91a?ya\x17\xB0V[P`@Q\x91\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[_\x90V[a?\xBFa?\xC4\x91a\x17EV[a\x1FVV[\x90V[\x90V[a?\xDEa?\xD9a?\xE3\x92a?\xC7V[a\x08\xF1V[a\x04\xB2V[\x90V[a@\x1Ba@\"\x94a@\x11``\x94\x98\x97\x95a@\x07`\x80\x86\x01\x9A_\x87\x01\x90a\x06FV[` \x85\x01\x90a\x07\tV[`@\x83\x01\x90a\x06FV[\x01\x90a\x06FV[V[a@,a\x03\x12V[=_\x82>=\x90\xFD[\x93\x92\x93a@?a\x1C]V[Pa@Ha?\xAFV[Pa@Qa\x17\xB0V[Pa@[\x85a?\xB3V[a@\x8Da@\x87\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0a?\xCAV[\x91a\x04\xB2V[\x11aA\x1AW\x90a@\xB0` \x94\x95_\x94\x93\x92\x93a@\xA7a\x03\x12V[\x94\x85\x94\x85a?\xE6V[\x83\x80R\x03\x90`\x01Z\xFA\x15aA\x15Wa@\xC8_Qa\x109V[\x80a@\xE3a@\xDDa@\xD8_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14a@\xF9W_\x91a@\xF3_a\x10>V[\x91\x92\x91\x90V[PaA\x03_a\x19\xC2V[`\x01\x91aA\x0F_a\x10>V[\x91\x92\x91\x90V[a@$V[PPPaA&_a\x19\xC2V[\x90`\x03\x92\x91\x92\x91\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x04\x11\x15aANWV[aA0V[\x90aA]\x82aADV[V[\x80aAraAl_aASV[\x91aASV[\x14_\x14aA}WPPV[\x80aA\x91aA\x8B`\x01aASV[\x91aASV[\x14_\x14aA\xB4W_c\xF6E\xEE\xDF`\xE0\x1B\x81R\x80aA\xB0`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[\x80aA\xC8aA\xC2`\x02aASV[\x91aASV[\x14_\x14aA\xF6WaA\xF2aA\xDB\x83a?\xB3V[_\x91\x82\x91c\xFC\xE6\x98\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x05HV[\x03\x90\xFD[aB\taB\x03`\x03aASV[\x91aASV[\x14aB\x11WPV[aB,\x90_\x91\x82\x91c5\xE2\xF3\x83`\xE2\x1B\x83R`\x04\x83\x01a\x06SV[\x03\x90\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_R` _ \x90V[aBV\x81a++V[\x82\x10\x15aBpWaBh`\x01\x91aBDV[\x91\x02\x01\x90_\x90V[aB0V[\x90aB\x7F\x90a\x0E\xF3V[\x90RV[\x90aB\x8D\x90a\x14LV[\x90RV[\x90aB\xC7aB\xBE_aB\xA1a&-V[\x94aB\xB8aB\xB0\x83\x83\x01a+pV[\x83\x88\x01aBuV[\x01a+\xC1V[` \x84\x01aB\x83V[V[aB\xD2\x90aB\x91V[\x90V[aB\xF3\x91_aB\xED\x92aB\xE6a&hV[P\x01aBMV[PaB\xC9V[\x90V[\x92\x91aC\x04\x84\x83\x83\x91aExV[\x83aC\x1FaC\x19aC\x14_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14aC4W[aC2\x92\x93\x91\x90\x91aG\x02V[V[aC<a\x17kV[\x93aCEaF\xE7V[\x94\x80aCYaCS\x88a\x04\xB2V[\x91a\x04\xB2V[\x11aCfWP\x93PaC%V[\x85\x90aC\x82_\x92\x83\x92c\x0EX\xAE\x93`\xE1\x1B\x84R`\x04\x84\x01a\x1F\x95V[\x03\x90\xFD[aC\x8Ea\x17AV[P\x15\x15\x90V[aC\xA8aC\xA3aC\xAD\x92a6\xA3V[a\x08\xF1V[a\x04\xB2V[\x90V[aC\xBCaC\xC2\x91a\x04\xB2V[\x91a\x04\xB2V[\x90\x81\x15aC\xCDW\x04\x90V[a7'V[aC\xF7aC\xFD\x92aC\xE1a\x17AV[P\x82\x81\x16\x92\x18aC\xF1`\x02aC\x94V[\x90aC\xB0V[\x90a\x19\xEAV[\x90V[\x90V[aD\x17aD\x12aD\x1C\x92aD\0V[a\x08\xF1V[a\x07\x03V[\x90V[aD(\x90aD\x03V[\x90RV[\x91` aDM\x92\x94\x93aDF`@\x82\x01\x96_\x83\x01\x90aD\x1FV[\x01\x90a\x05;V[V[aDcaD^aDh\x92a\x04\xB2V[a\x08\xF1V[a\x14LV[\x90V[aDsa\x14\xD7V[P\x80aD\x8DaD\x87`\x01\x80`\xD0\x1B\x03a\x194V[\x91a\x04\xB2V[\x11aD\x9EWaD\x9B\x90aDOV[\x90V[`\xD0aD\xBA_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01aD,V[\x03\x90\xFD[\x90aD\xF4aD\xFA\x93\x92aD\xCFa\x14\xD7V[PaD\xD8a\x14\xD7V[P\x80\x93aD\xEDaD\xE6a\"KV[\x94\x92a/sV[\x90\x91aK}V[\x91aG\xC1V[\x91\x90\x91\x90V[aE\x14aE\x0FaE\x19\x92a5\xAEV[a\x08\xF1V[a\x04\xB2V[\x90V[6\x907V[\x90aEFaE.\x83a\x1B\xA5V[\x92` \x80aE<\x86\x93a\x1B\x82V[\x92\x01\x91\x03\x90aE\x1CV[V[aEPa\x15\xB0V[PaEZ\x81aH+V[\x90aEmaEh` aE\0V[aE!V[\x91\x82R` \x82\x01R\x90V[\x91\x90\x91\x80aE\x96aE\x90aE\x8B_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14_\x14aFwWaE\xBAaE\xB3\x83aE\xAE`\x02a\x17^V[a\x19\xEAV[`\x02a\x1FuV[[\x82aE\xD6aE\xD0aE\xCB_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14_\x14aFKWaE\xFAaE\xF3\x83aE\xEE`\x02a\x17^V[a&\xFEV[`\x02a\x1FuV[[\x91\x90\x91aFFaF4aF.\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93a\x19\x0FV[\x93a\x19\x0FV[\x93aF=a\x03\x12V[\x91\x82\x91\x82a\x05HV[\x03\x90\xA3V[aFr\x82aFlaF]_\x87\x90a\x1C\xEEV[\x91aFg\x83a\x17^V[a7]V[\x90a\x1FuV[aE\xFBV[aF\x8AaF\x85_\x83\x90a\x1C\xEEV[a\x17^V[\x80aF\x9DaF\x97\x85a\x04\xB2V[\x91a\x04\xB2V[\x10aF\xC5WaF\xB0aF\xC0\x91\x84\x90a&\xFEV[aF\xBB_\x84\x90a\x1C\xEEV[a\x1FuV[aE\xBBV[\x90aF\xE3\x90\x91\x92_\x93\x84\x93c9\x144\xE3`\xE2\x1B\x85R`\x04\x85\x01a&\xCCV[\x03\x90\xFD[aF\xEFa\x17AV[PaF\xFF`\x01\x80`\xD0\x1B\x03a\x194V[\x90V[\x91aGZaGTaGa\x94\x80aG(aG\"aG\x1D_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14aG\x92W[\x84aGIaGCaG>_a\x19\xC2V[a\x04\x83V[\x91a\x04\x83V[\x14aGcW[a\x1C\xA3V[\x92a\x1C\xA3V[\x90\x91a;\x87V[V[aG\x8B`\x0B`\x02aG\x85aG\x7FaGy\x89aDkV[\x93a\x191V[\x91a;\x84V[\x90aD\xBEV[PPaGOV[aG\xBA`\x0B`\x01aG\xB4aG\xAEaG\xA8\x89aDkV[\x93a\x191V[\x91a;\x84V[\x90aD\xBEV[PPaG.V[\x91aG\xE5_aG\xEA\x94aG\xD2a\x14\xD7V[PaG\xDBa\x14\xD7V[P\x01\x92\x91\x92a+NV[aJ/V[\x91\x90\x91\x90V[aH\x04aG\xFFaH\t\x92a>$V[a\x08\xF1V[a\x04\xB2V[\x90V[\x90V[aH#aH\x1EaH(\x92aH\x0CV[a\x08\xF1V[a\x04\xB2V[\x90V[aH@aHE\x91aH:a\x17AV[Pa\x17\xB4V[a?\xB3V[aHO`\xFFaG\xF0V[\x16\x80aHdaH^`\x1FaH\x0FV[\x91a\x04\xB2V[\x11aHlW\x90V[_c,\xD4J\xC3`\xE2\x1B\x81R\x80aH\x84`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[T\x90V[aH\x96`@a\x1BmV[\x90V[_R` _ \x90V[aH\xAB\x81aH\x88V[\x82\x10\x15aH\xC5WaH\xBD`\x01\x91aH\x99V[\x91\x02\x01\x90_\x90V[aB0V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[aH\xE7\x90Qa\x0E\xF3V[\x90V[\x90aH\xFBe\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x109V[\x91\x81\x19\x16\x91\x16\x17\x90V[aI\x19aI\x14aI\x1E\x92a\x0E\xF3V[a\x08\xF1V[a\x0E\xF3V[\x90V[\x90V[\x90aI9aI4aI@\x92aI\x05V[aI!V[\x82TaH\xEAV[\x90UV[aIN\x90Qa\x14LV[\x90V[`0\x1B\x90V[\x90aIie\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91aIQV[\x91\x81\x19\x16\x91\x16\x17\x90V[aI\x87aI\x82aI\x8C\x92a\x14LV[a\x08\xF1V[a\x14LV[\x90V[\x90V[\x90aI\xA7aI\xA2aI\xAE\x92aIsV[aI\x8FV[\x82TaIWV[\x90UV[\x90aI\xDC` _aI\xE2\x94aI\xD4\x82\x82\x01aI\xCE\x84\x88\x01aH\xDDV[\x90aI$V[\x01\x92\x01aIDV[\x90aI\x92V[V[\x91\x90aI\xF5WaI\xF3\x91aI\xB2V[V[aH\xCAV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15aJ*W\x82aJ\"\x91`\x01aJ(\x95\x01\x81UaH\xA2V[\x90aI\xE4V[V[a\x16\x9EV[\x90\x92\x91\x92aJ;a\x14\xD7V[PaJDa\x14\xD7V[PaJN\x82aH\x88V[\x80aJaaJ[_a\x19\xCEV[\x91a\x04\xB2V[\x11_\x14aK1WaJ\x87\x90aJ\x81\x84\x91aJ{`\x01a+\x80V[\x90a\x1B#V[\x90a:\xD6V[\x90aJ\x93_\x83\x01a+pV[\x92aJ\x9F_\x84\x01a+\xC1V[\x93\x80aJ\xB3aJ\xAD\x85a\x0E\xF3V[\x91a\x0E\xF3V[\x11aK\x15WaJ\xCAaJ\xC4\x84a\x0E\xF3V[\x91a\x0E\xF3V[\x14_\x14aJ\xE5WPPaJ\xE0\x90_\x85\x91\x01aI\x92V[[\x91\x90V[aK\x10\x92PaK\x0B\x86aK\x02aJ\xF9aH\x8CV[\x94_\x86\x01aBuV[` \x84\x01aB\x83V[aI\xFAV[aJ\xE1V[_c% `\x1D`\xE0\x1B\x81R\x80aK-`\x04\x82\x01a\x06\xCAV[\x03\x90\xFD[PaK\\\x91aKW\x85aKNaKEaH\x8CV[\x94_\x86\x01aBuV[` \x84\x01aB\x83V[aI\xFAV[aKe_a+\xCEV[\x91\x90V[cNH{q`\xE0\x1B_R`Q`\x04R`$_\xFD[\x91\x90\x91\x80`\x01\x14aK\x9CW`\x02\x03aKiWaK\x98\x91a\x15VV[\x90[V[PaK\xA6\x91a\x15\x17V[\x90aK\x9AV",
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
    /**Function with signature `getRemainingEmissions()` and selector `0x4bdd36ce`.
```solidity
function getRemainingEmissions() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRemainingEmissionsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
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
            type Return = alloy::sol_types::private::primitives::aliases::U256;
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
                        let r: getRemainingEmissionsReturn = r.into();
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
                        let r: getRemainingEmissionsReturn = r.into();
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
            [66u8, 150u8, 108u8, 104u8],
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
        const COUNT: usize = 47usize;
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
                    fn getRemainingEmissions(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getRemainingEmissionsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(SyndicateTokenCalls::getRemainingEmissions)
                    }
                    getRemainingEmissions
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
                    fn getRemainingEmissions(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <getRemainingEmissionsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(SyndicateTokenCalls::getRemainingEmissions)
                    }
                    getRemainingEmissions
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
        ///Creates a new call builder for the [`getRemainingEmissions`] function.
        pub fn getRemainingEmissions(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getRemainingEmissionsCall, N> {
            self.call_builder(&getRemainingEmissionsCall)
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
