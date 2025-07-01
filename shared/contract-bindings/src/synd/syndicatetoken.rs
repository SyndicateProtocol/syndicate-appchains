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
    event Transfer(address indexed from, address indexed to, uint256 value);

    constructor(address defaultAdmin, address syndTreasuryAddress);

    function CLOCK_MODE() external view returns (string memory);
    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
    function DOMAIN_SEPARATOR() external view returns (bytes32);
    function EMISSION_MINTER_ROLE() external view returns (bytes32);
    function INITIAL_MINT_SUPPLY() external view returns (uint256);
    function TOTAL_SUPPLY() external view returns (uint256);
    function allowance(address owner, address spender) external view returns (uint256);
    function approve(address spender, uint256 value) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
    function burn(uint256 amount) external;
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
    function getRoleAdmin(bytes32 role) external view returns (bytes32);
    function getVotes(address account) external view returns (uint256);
    function getVotingPower(address account) external view returns (uint256);
    function grantRole(bytes32 role, address account) external;
    function hasRole(bytes32 role, address account) external view returns (bool);
    function mint(address to, uint256 amount) external;
    function name() external view returns (string memory);
    function nonces(address owner) external view returns (uint256);
    function numCheckpoints(address account) external view returns (uint32);
    function permit(address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) external;
    function renounceRole(bytes32 role, address callerConfirmation) external;
    function revokeRole(bytes32 role, address account) external;
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function symbol() external view returns (string memory);
    function totalSupply() external view returns (uint256);
    function transfer(address to, uint256 value) external returns (bool);
    function transferFrom(address from, address to, uint256 value) external returns (bool);
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
    ///0x610160604052346100735761001b610015610147565b906102d4565b610023610078565b614532611b09823960805181612332015260a05181612369015260c051816122f9015260e05181612cf901526101005181612d1e015261012051816128a6015261014051816128e6015261453290f35b61007e565b60405190565b5f80fd5b601f801991011690565b634e487b7160e01b5f52604160045260245ffd5b906100aa90610082565b810190811060018060401b038211176100c257604052565b61008c565b906100da6100d3610078565b92836100a0565b565b5f80fd5b60018060a01b031690565b6100f4906100e0565b90565b610100816100eb565b0361010757565b5f80fd5b90505190610118826100f7565b565b9190604083820312610142578061013661013f925f860161010b565b9360200161010b565b90565b6100dc565b61016561603b8038038061015a816100c7565b92833981019061011a565b9091565b60018060401b03811161018557610181602091610082565b0190565b61008c565b9061019c61019783610169565b6100c7565b918252565b5f7f53796e6469636174650000000000000000000000000000000000000000000000910152565b6101d2600961018a565b906101df602083016101a1565b565b6101e96101c8565b90565b5f7f53594e4400000000000000000000000000000000000000000000000000000000910152565b61021d600461018a565b9061022a602083016101ec565b565b610234610213565b90565b90565b90565b61025161024c61025692610237565b61023a565b6100e0565b90565b6102629061023d565b90565b5f0190565b90565b5f1b90565b61028661028161028b92610237565b61026d565b61026a565b90565b6102975f610272565b90565b90565b90565b6102b46102af6102b99261029a565b61023a565b61029d565b90565b6102d16b02e87669c308736a040000006102a0565b90565b906102f66102e06101e1565b6102e86101e1565b6102f061022c565b91610393565b8161031161030b6103065f610259565b6100eb565b916100eb565b14610377578061033161032b6103265f610259565b6100eb565b916100eb565b1461035b5761034a6103599261034561028e565b61084e565b506103536102bc565b9061091c565b565b5f63d92e233d60e01b81528061037360048201610265565b0390fd5b5f63d92e233d60e01b81528061038f60048201610265565b0390fd5b9061039e92916103a0565b565b906103ab92916103ad565b565b906103b892916103ba565b565b906103c592916103c7565b565b906103d2929161041f565b565b5f7f3100000000000000000000000000000000000000000000000000000000000000910152565b610405600161018a565b90610412602083016103d4565b565b61041c6103fb565b90565b90610433929161042d610414565b90610435565b565b90610441939291610487565b565b90565b90565b60200190565b5190565b61046761046261046c926100e0565b61023a565b6100e0565b90565b61047890610453565b90565b6104849061046f565b90565b6104986104e8946104cd939461051c565b6104ac816104a66006610443565b906109c9565b610120526104c4836104be6007610443565b906109c9565b61014052610446565b6104df6104d98261044f565b91610449565b2060e052610446565b6104fa6104f48261044f565b91610449565b20610100524660a05261050b610ace565b6080526105173061047b565b60c052565b9061052691610528565b565b9061053291610534565b565b9061053e916107a4565b565b634e487b7160e01b5f525f60045260245ffd5b5190565b634e487b7160e01b5f52602260045260245ffd5b906001600283049216801561058b575b602083101461058657565b610557565b91607f169161057b565b5f5260205f2090565b601f602091010490565b1b90565b919060086105c79102916105c15f19846105a8565b926105a8565b9181191691161790565b6105e56105e06105ea9261029d565b61023a565b61029d565b90565b90565b919061060661060161060e936105d1565b6105ed565b9083546105ac565b9055565b5f90565b61062891610622610612565b916105f0565b565b5b818110610636575050565b806106435f600193610616565b0161062b565b9190601f8111610659575b505050565b61066561068a93610595565b9060206106718461059e565b83019310610692575b6106839061059e565b019061062a565b5f8080610654565b91506106838192905061067a565b1c90565b906106b4905f19906008026106a0565b191690565b816106c3916106a4565b906002021790565b906106d581610553565b9060018060401b038211610793576106f7826106f1855461056b565b85610649565b602090601f831160011461072b5791809161071a935f9261071f575b50506106b9565b90555b565b90915001515f80610713565b601f1983169161073a85610595565b925f5b81811061077b57509160029391856001969410610761575b5050500201905561071d565b610771910151601f8416906106a4565b90555f8080610755565b9193602060018192878701518155019501920161073d565b61008c565b906107a2916106cb565b565b906107b36107ba926003610798565b6004610798565b565b5f90565b151590565b6107ce9061026a565b90565b906107db906107c5565b5f5260205260405f2090565b6107f09061046f565b90565b906107fd906107e7565b5f5260205260405f2090565b9061081560ff9161026d565b9181191691161790565b610828906107c0565b90565b90565b9061084361083e61084a9261081f565b61082b565b8254610809565b9055565b6108566107bc565b5061086b610865828490610b6b565b156107c0565b5f146108f457610893600161088e5f610886600586906107d1565b0185906107f3565b61082e565b9061089c610b99565b906108d96108d36108cd7f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d956107c5565b926107e7565b926107e7565b926108e2610078565b806108ec81610265565b0390a4600190565b50505f90565b610903906100eb565b9052565b919061091a905f602085019401906108fa565b565b8061093761093161092c5f610259565b6100eb565b916100eb565b1461095357610951916109495f610259565b919091610ba6565b565b61097661095f5f610259565b5f91829163ec442f0560e01b835260048301610907565b0390fd5b5f90565b90565b61099561099061099a9261097e565b61023a565b61029d565b90565b90565b6109b46109af6109b99261099d565b61026d565b61026a565b90565b6109c660ff6109a0565b90565b906109d261097a565b506109e46109df83610446565b61044f565b6109f76109f16020610981565b9161029d565b105f14610a0b5750610a0890610ca5565b90565b5f610a19610a1f9392610bb5565b01610798565b610a2f610a2a6109bc565b6107c5565b90565b5f90565b7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f90565b610a64905161026a565b90565b610a709061026a565b9052565b610a7d9061029d565b9052565b90959492610acc94610abb610ac592610ab1608096610aa760a088019c5f890190610a67565b6020870190610a67565b6040850190610a67565b6060830190610a74565b01906108fa565b565b610ad6610a32565b50610adf610a36565b610b29610aec60e0610a5a565b91610b1a610afb610100610a5a565b46610b053061047b565b91610b0e610078565b96879560208701610a81565b602082018103825203826100a0565b610b3b610b358261044f565b91610449565b2090565b5f1c90565b60ff1690565b610b56610b5b91610b3f565b610b44565b90565b610b689054610b4a565b90565b610b92915f610b87610b8d93610b7f6107bc565b5060056107d1565b016107f3565b610b5e565b90565b5f90565b610ba1610b95565b503390565b91610bb392919091610d52565b565b90565b90565b610bcf610bca610bd492610bb8565b61023a565b61029d565b90565b60209181520190565b90825f9392825e0152565b610c0a610c13602093610c1893610c0181610553565b93848093610bd7565b95869101610be0565b610082565b0190565b610c319160208201915f818403910152610beb565b90565b610c4e610c49610c438361044f565b92610449565b610a5a565b9060208110610c5c575b5090565b610c6e905f19906020036008026105a8565b165f610c58565b610c81610c8691610b3f565b6105d1565b90565b610c9d610c98610ca29261029d565b61026d565b61026a565b90565b610cad61097a565b50610cb781610446565b90610cc18261044f565b610cd4610cce601f610bbb565b9161029d565b11610d095750610d0181610cfb610cf5610cf0610d0695610c34565b610c75565b9161044f565b17610c89565b6107c5565b90565b610d2b90610d15610078565b91829163305a27a960e01b835260048301610c1c565b0390fd5b916020610d50929493610d4960408201965f830190610a74565b0190610a74565b565b9291610d6084838391610eee565b83610d7b610d75610d705f610259565b6100eb565b916100eb565b14610d90575b610d8e92939190916110bb565b565b610d9861105d565b93610da161109a565b9480610db5610daf8861029d565b9161029d565b11610dc257509350610d81565b8590610dde5f928392630e58ae9360e11b845260048401610d2f565b0390fd5b90610dec906107e7565b5f5260205260405f2090565b90565b610e07610e0c91610b3f565b610df8565b90565b610e199054610dfb565b90565b604090610e45610e4c9496959396610e3b60608401985f8501906108fa565b6020830190610a74565b0190610a74565b565b90610e59910361029d565b90565b90610e685f199161026d565b9181191691161790565b90610e87610e82610e8e926105d1565b6105ed565b8254610e5c565b9055565b634e487b7160e01b5f52601160045260245ffd5b610eb5610ebb9193929361029d565b9261029d565b8201809211610ec657565b610e92565b90610ed6910161029d565b90565b9190610eec905f60208501940190610a74565b565b91909180610f0c610f06610f015f610259565b6100eb565b916100eb565b145f14610fed57610f30610f2983610f246002610e0f565b610ea6565b6002610e72565b5b82610f4c610f46610f415f610259565b6100eb565b916100eb565b145f14610fc157610f70610f6983610f646002610e0f565b610e4e565b6002610e72565b5b919091610fbc610faa610fa47fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef936107e7565b936107e7565b93610fb3610078565b91829182610ed9565b0390a3565b610fe882610fe2610fd35f8790610de2565b91610fdd83610e0f565b610ecb565b90610e72565b610f71565b611000610ffb5f8390610de2565b610e0f565b8061101361100d8561029d565b9161029d565b1061103b57611026611036918490610e4e565b6110315f8490610de2565b610e72565b610f31565b906110599091925f93849363391434e360e21b855260048501610e1c565b0390fd5b611065610612565b506110706002610e0f565b90565b60018060d01b031690565b61109261108d61109792611073565b61023a565b61029d565b90565b6110a2610612565b506110b260018060d01b0361107e565b90565b90565b90565b9161111361110d61111a94806110e16110db6110d65f610259565b6100eb565b916100eb565b1461114b575b846111026110fc6110f75f610259565b6100eb565b916100eb565b1461111c575b611343565b92611343565b9091611394565b565b611144600b600261113e6111386111328961122d565b936110b5565b916110b8565b90611280565b5050611108565b611173600b600161116d6111676111618961122d565b936110b5565b916110b8565b90611280565b50506110e7565b5f90565b61118a61119091611073565b91611073565b019060018060d01b0382116111a157565b610e92565b906111b9916111b361117a565b5061117e565b90565b90565b60ff1690565b6111d96111d46111de926111bc565b61023a565b6111bf565b90565b6111ea906111c5565b9052565b91602061120f92949361120860408201965f8301906111e1565b0190610a74565b565b61122561122061122a9261029d565b61023a565b611073565b90565b61123561117a565b508061124f61124960018060d01b0361107e565b9161029d565b116112605761125d90611211565b90565b60d061127c5f9283926306dfcc6560e41b8452600484016111ee565b0390fd5b906112b66112bc939261129161117a565b5061129a61117a565b5080936112af6112a8611546565b94926115f3565b9091611ad9565b91611668565b91909190565b6112ce6112d491611073565b91611073565b90039060018060d01b0382116112e657565b610e92565b906112fe916112f861117a565b506112c2565b90565b9061130b906107e7565b5f5260205260405f2090565b60018060a01b031690565b61132e61133391610b3f565b611317565b90565b6113409054611322565b90565b61135a61135f91611352610b95565b506009611301565b611336565b90565b61137661137161137b92610237565b61023a565b61029d565b90565b90611388906107e7565b5f5260205260405f2090565b919091806113aa6113a4856100eb565b916100eb565b141580611528575b6113bc575b505050565b806113d76113d16113cc5f610259565b6100eb565b916100eb565b03611498575b50816113f96113f36113ee5f610259565b6100eb565b916100eb565b03611405575b806113b7565b61144c61143f6114469261141b600a869061137e565b9061143961143361142d60019361122d565b936110b5565b916110b8565b90611280565b929061107e565b9161107e565b9190916114797fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926107e7565b9261148e611485610078565b92839283610d2f565b0390a25f806113ff565b6114d76114dd6114d06114ad600a859061137e565b60026114ca6114c46114be8961122d565b936110b5565b916110b8565b90611280565b929061107e565b9161107e565b91909161150a7fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724926107e7565b9261151f611516610078565b92839283610d2f565b0390a25f6113dd565b508161153c6115365f611362565b9161029d565b116113b2565b5f90565b61154e611542565b50611557611697565b90565b5490565b90565b61157561157061157a9261155e565b61023a565b61029d565b90565b61158c6115929193929361029d565b9261029d565b820391821161159d57565b610e92565b90565b60301c90565b60018060d01b031690565b6115c26115c7916115a5565b6115ab565b90565b6115d490546115b6565b90565b6115eb6115e66115f092610237565b61023a565b611073565b90565b6115fb61117a565b506116075f820161155a565b8061161a6116145f611362565b9161029d565b145f1461163057505061162c5f6115d7565b5b90565b61165d5f916116586116528461166396019261164c6001611561565b9061157d565b916115a2565b6116ac565b016115ca565b61162d565b9161168c5f6116919461167961117a565b5061168261117a565b50019291926115a2565b6118b1565b91909190565b61169f611542565b506116a943611a72565b90565b5f5260205f200190565b5490565b6116c460406100c7565b90565b65ffffffffffff1690565b906116dc906116c7565b9052565b906116ea90611073565b9052565b5f5260205f2090565b634e487b7160e01b5f52603260045260245ffd5b611714816116b6565b82101561172e576117266001916116ee565b910201905f90565b6116f7565b61173d90516116c7565b90565b9061175165ffffffffffff9161026d565b9181191691161790565b61176f61176a611774926116c7565b61023a565b6116c7565b90565b90565b9061178f61178a6117969261175b565b611777565b8254611740565b9055565b6117a49051611073565b90565b60301b90565b906117bf65ffffffffffff19916117a7565b9181191691161790565b6117dd6117d86117e292611073565b61023a565b611073565b90565b90565b906117fd6117f8611804926117c9565b6117e5565b82546117ad565b9055565b9061183260205f6118389461182a828201611824848801611733565b9061177a565b01920161179a565b906117e8565b565b919061184b5761184991611808565b565b610540565b9081549168010000000000000000831015611880578261187891600161187e9501815561170b565b9061183a565b565b61008c565b65ffffffffffff1690565b61189c6118a191610b3f565b611885565b90565b6118ae9054611890565b90565b909291926118bd61117a565b506118c661117a565b506118d0826116b6565b806118e36118dd5f611362565b9161029d565b115f146119b3576119099061190384916118fd6001611561565b9061157d565b906116ac565b906119155f83016118a4565b926119215f84016115ca565b938061193561192f856116c7565b916116c7565b116119975761194c611946846116c7565b916116c7565b145f14611967575050611962905f8591016117e8565b5b9190565b611992925061198d8661198461197b6116ba565b945f86016116d2565b602084016116e0565b611850565b611963565b5f632520601d60e01b8152806119af60048201610265565b0390fd5b506119de916119d9856119d06119c76116ba565b945f86016116d2565b602084016116e0565b611850565b6119e75f6115d7565b9190565b6119ff6119fa611a04926116c7565b61023a565b61029d565b90565b90565b611a1e611a19611a2392611a07565b61023a565b6111bf565b90565b611a2f90611a0a565b9052565b916020611a54929493611a4d60408201965f830190611a26565b0190610a74565b565b611a6a611a65611a6f9261029d565b61023a565b6116c7565b90565b611a7a611542565b5080611a94611a8e65ffffffffffff6119eb565b9161029d565b11611aa557611aa290611a56565b90565b6030611ac15f9283926306dfcc6560e41b845260048401611a33565b0390fd5b634e487b7160e01b5f52605160045260245ffd5b91909180600114611af857600203611ac557611af4916112eb565b905b565b50611b02916111a6565b90611af656fe60806040526004361015610013575b6111e6565b61001d5f3561028c565b806301ffc9a71461028757806306fdde0314610282578063095ea7b31461027d57806318160ddd1461027857806323b872dd14610273578063248a9ca31461026e5780632f2ff15d14610269578063313ce567146102645780633644e5151461025f57806336568abe1461025a5780633a46b1a81461025557806340c10f191461025057806342966c681461024b5780634bdd36ce146102465780634bf5d7e914610241578063587cde1e1461023c5780635c19a95c146102375780636fcfff451461023257806370a082311461022d5780637ecebe001461022857806384b0196e146102235780638d3343d61461021e5780638e539e8c14610219578063902d55a51461021457806391d148541461020f57806391ddadf41461020a57806395d89b41146102055780639ab24eb0146102005780639b7ef64b146101fb578063a217fddf146101f6578063a9059cbb146101f1578063b0ca253e146101ec578063bb4d4436146101e7578063c02ae754146101e2578063c3cda520146101dd578063d505accf146101d8578063d547741f146101d3578063dd62ed3e146101ce5763f1127ed80361000e576111b0565b6110cc565b61106b565b611031565b610f87565b610ecb565b610e96565b610e60565b610e2a565b610df5565b610d85565b610d0e565b610cd9565b610ca4565b610c41565b610c0c565b610b92565b610b5d565b610af2565b6109ab565b610976565b610941565b6108e3565b6108ae565b610839565b610804565b6107d1565b61077f565b610749565b610715565b6106e0565b6106ab565b61064f565b6105e8565b61054c565b6104dd565b610485565b6103c3565b610314565b60e01c90565b60405190565b5f80fd5b5f80fd5b63ffffffff60e01b1690565b6102b5816102a0565b036102bc57565b5f80fd5b905035906102cd826102ac565b565b906020828203126102e8576102e5915f016102c0565b90565b61029c565b151590565b6102fb906102ed565b9052565b9190610312905f602085019401906102f2565b565b346103445761034061032f61032a3660046102cf565b611283565b610337610292565b918291826102ff565b0390f35b610298565b5f91031261035357565b61029c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b6103996103a26020936103a79361039081610358565b9384809361035c565b95869101610365565b610370565b0190565b6103c09160208201915f81840391015261037a565b90565b346103f3576103d3366004610349565b6103ef6103de61141c565b6103e6610292565b918291826103ab565b0390f35b610298565b60018060a01b031690565b61040c906103f8565b90565b61041881610403565b0361041f57565b5f80fd5b905035906104308261040f565b565b90565b61043e81610432565b0361044557565b5f80fd5b9050359061045682610435565b565b9190604083820312610480578061047461047d925f8601610423565b93602001610449565b90565b61029c565b346104b6576104b26104a161049b366004610458565b90611432565b6104a9610292565b918291826102ff565b0390f35b610298565b6104c490610432565b9052565b91906104db905f602085019401906104bb565b565b3461050d576104ed366004610349565b6105096104f8611481565b610500610292565b918291826104c8565b0390f35b610298565b90916060828403126105475761054461052d845f8501610423565b9361053b8160208601610423565b93604001610449565b90565b61029c565b3461057d57610579610568610562366004610512565b91611497565b610570610292565b918291826102ff565b0390f35b610298565b90565b61058e81610582565b0361059557565b5f80fd5b905035906105a682610585565b565b906020828203126105c1576105be915f01610599565b90565b61029c565b6105cf90610582565b9052565b91906105e6905f602085019401906105c6565b565b34610618576106146106036105fe3660046105a8565b611510565b61060b610292565b918291826105d3565b0390f35b610298565b91906040838203126106455780610639610642925f8601610599565b93602001610423565b90565b61029c565b5f0190565b3461067e5761066861066236600461061d565b9061155c565b610670610292565b8061067a8161064a565b0390f35b610298565b60ff1690565b61069290610683565b9052565b91906106a9905f60208501940190610689565b565b346106db576106bb366004610349565b6106d76106c661158b565b6106ce610292565b91829182610696565b0390f35b610298565b34610710576106f0366004610349565b61070c6106fb6115a1565b610703610292565b918291826105d3565b0390f35b610298565b346107445761072e61072836600461061d565b906115b5565b610736610292565b806107408161064a565b0390f35b610298565b3461077a5761077661076561075f366004610458565b90611666565b61076d610292565b918291826104c8565b0390f35b610298565b346107ae57610798610792366004610458565b906117ed565b6107a0610292565b806107aa8161064a565b0390f35b610298565b906020828203126107cc576107c9915f01610449565b90565b61029c565b346107ff576107e96107e43660046107b3565b6117f9565b6107f1610292565b806107fb8161064a565b0390f35b610298565b3461083457610814366004610349565b61083061081f61185e565b610827610292565b918291826104c8565b0390f35b610298565b3461086957610849366004610349565b61086561085461191d565b61085c610292565b918291826103ab565b0390f35b610298565b9060208282031261088757610884915f01610423565b90565b61029c565b61089590610403565b9052565b91906108ac905f6020850194019061088c565b565b346108de576108da6108c96108c436600461086e565b6119b9565b6108d1610292565b91829182610899565b0390f35b610298565b34610911576108fb6108f636600461086e565b6119d8565b610903610292565b8061090d8161064a565b0390f35b610298565b63ffffffff1690565b61092890610916565b9052565b919061093f905f6020850194019061091f565b565b346109715761096d61095c61095736600461086e565b6119ef565b610964610292565b9182918261092c565b0390f35b610298565b346109a6576109a261099161098c36600461086e565b611a1a565b610999610292565b918291826104c8565b0390f35b610298565b346109db576109d76109c66109c136600461086e565b611a38565b6109ce610292565b918291826104c8565b0390f35b610298565b60ff60f81b1690565b6109f2906109e0565b9052565b5190565b60209181520190565b60200190565b610a1290610432565b9052565b90610a2381602093610a09565b0190565b60200190565b90610a4a610a44610a3d846109f6565b80936109fa565b92610a03565b905f5b818110610a5a5750505090565b909192610a73610a6d6001928651610a16565b94610a27565b9101919091610a4d565b93959194610ace610ac3610ae295610ab5610ad895610aef9c9a610aa860e08c01925f8d01906109e9565b8a820360208c015261037a565b9088820360408a015261037a565b9760608701906104bb565b608085019061088c565b60a08301906105c6565b60c0818403910152610a2d565b90565b34610b2957610b02366004610349565b610b25610b0d611aca565b93610b1c979597939193610292565b97889788610a7d565b0390f35b610298565b7f9b12e0c5707e494915e58b0564f18aaad9b74ac69bfc815a1edadc8e4bd032eb90565b610b5a610b2e565b90565b34610b8d57610b6d366004610349565b610b89610b78610b52565b610b80610292565b918291826105d3565b0390f35b610298565b34610bc257610bbe610bad610ba83660046107b3565b611b54565b610bb5610292565b918291826104c8565b0390f35b610298565b90565b90565b610be1610bdc610be692610bc7565b610bca565b610432565b90565b610bfe6b033b2e3c9fd0803ce8000000610bcd565b90565b610c09610be9565b90565b34610c3c57610c1c366004610349565b610c38610c27610c01565b610c2f610292565b918291826104c8565b0390f35b610298565b34610c7257610c6e610c5d610c5736600461061d565b90611bc2565b610c65610292565b918291826102ff565b0390f35b610298565b65ffffffffffff1690565b610c8b90610c77565b9052565b9190610ca2905f60208501940190610c82565b565b34610cd457610cb4366004610349565b610cd0610cbf611bf0565b610cc7610292565b91829182610c8f565b0390f35b610298565b34610d0957610ce9366004610349565b610d05610cf4611c04565b610cfc610292565b918291826103ab565b0390f35b610298565b34610d3e57610d3a610d29610d2436600461086e565b611c1a565b610d31610292565b918291826104c8565b0390f35b610298565b90565b610d5a610d55610d5f92610d43565b610bca565b610432565b90565b610d776b02e87669c308736a04000000610d46565b90565b610d82610d62565b90565b34610db557610d95366004610349565b610db1610da0610d7a565b610da8610292565b918291826104c8565b0390f35b610298565b90565b5f1b90565b610dd6610dd1610ddb92610dba565b610dbd565b610582565b90565b610de75f610dc2565b90565b610df2610dde565b90565b34610e2557610e05366004610349565b610e21610e10610dea565b610e18610292565b918291826105d3565b0390f35b610298565b34610e5b57610e57610e46610e40366004610458565b90611c49565b610e4e610292565b918291826102ff565b0390f35b610298565b34610e9157610e8d610e7c610e76366004610458565b90611c6b565b610e84610292565b918291826104c8565b0390f35b610298565b34610ec657610ec2610eb1610eac36600461086e565b611c81565b610eb9610292565b918291826104c8565b0390f35b610298565b34610efb57610edb366004610349565b610ef7610ee6611c96565b610eee610292565b918291826104c8565b0390f35b610298565b610f0981610683565b03610f1057565b5f80fd5b90503590610f2182610f00565b565b909160c082840312610f8257610f3b835f8401610423565b92610f498160208501610449565b92610f578260408301610449565b92610f7f610f688460608501610f14565b93610f768160808601610599565b9360a001610599565b90565b61029c565b34610fbc57610fa6610f9a366004610f23565b94939093929192611d16565b610fae610292565b80610fb88161064a565b0390f35b610298565b60e08183031261102c57610fd7825f8301610423565b92610fe58360208401610423565b92610ff38160408501610449565b926110018260608301610449565b926110296110128460808501610f14565b936110208160a08601610599565b9360c001610599565b90565b61029c565b3461106657611050611044366004610fc1565b95949094939193611e6a565b611058610292565b806110628161064a565b0390f35b610298565b3461109a5761108461107e36600461061d565b90611f88565b61108c610292565b806110968161064a565b0390f35b610298565b91906040838203126110c757806110bb6110c4925f8601610423565b93602001610423565b90565b61029c565b346110fd576110f96110e86110e236600461109f565b90611faa565b6110f0610292565b918291826104c8565b0390f35b610298565b61110b81610916565b0361111257565b5f80fd5b9050359061112382611102565b565b919060408382031261114d578061114161114a925f8601610423565b93602001611116565b90565b61029c565b61115b90610c77565b9052565b60018060d01b031690565b6111739061115f565b9052565b906020806111999361118f5f8201515f860190611152565b015191019061116a565b565b91906111ae905f60408501940190611177565b565b346111e1576111dd6111cc6111c6366004611125565b90612018565b6111d4610292565b9182918261119b565b0390f35b610298565b5f80fd5b5f90565b634e487b7160e01b5f52601160045260245ffd5b61120e6112149161115f565b9161115f565b019060018060d01b03821161122557565b6111ee565b9061123d916112376111ea565b50611202565b90565b61124c6112529161115f565b9161115f565b90039060018060d01b03821161126457565b6111ee565b9061127c916112766111ea565b50611240565b90565b5f90565b61128b61127f565b50806112a66112a0637965db0b60e01b6102a0565b916102a0565b149081156112b3575b5090565b6112bd915061202e565b5f6112af565b606090565b634e487b7160e01b5f52602260045260245ffd5b90600160028304921680156112fc575b60208310146112f757565b6112c8565b91607f16916112ec565b60209181520190565b5f5260205f2090565b905f929180549061133261132b836112dc565b8094611306565b916001811690815f14611389575060011461134d575b505050565b61135a919293945061130f565b915f925b81841061137157505001905f8080611348565b6001816020929593955484860152019101929061135e565b92949550505060ff19168252151560200201905f8080611348565b906113ae91611318565b90565b634e487b7160e01b5f52604160045260245ffd5b906113cf90610370565b810190811067ffffffffffffffff8211176113e957604052565b6113b1565b9061140e611407926113fe610292565b938480926113a4565b03836113c5565b565b611419906113ee565b90565b6114246112c3565b5061142f6003611410565b90565b61144f9161143e61127f565b50611447612054565b919091612061565b600190565b5f90565b5f1c90565b90565b61146c61147191611458565b61145d565b90565b61147e9054611460565b90565b611489611454565b506114946002611474565b90565b916114c1926114a461127f565b506114b96114b0612054565b829084916120b1565b91909161213d565b600190565b5f90565b6114d390610582565b90565b906114e0906114ca565b5f5260205260405f2090565b90565b6114fb61150091611458565b6114ec565b90565b61150d90546114ef565b90565b600161152961152f926115216114c6565b5060056114d6565b01611503565b90565b9061154d9161154861154382611510565b6121da565b61154f565b565b9061155991612233565b50565b9061156691611532565b565b5f90565b90565b61158361157e6115889261156c565b610bca565b610683565b90565b611593611568565b5061159e601261156f565b90565b6115a96114c6565b506115b26122df565b90565b90806115d06115ca6115c5612054565b610403565b91610403565b036115e1576115de91612399565b50565b5f63334bd91960e11b8152806115f96004820161064a565b0390fd5b61161161160c611616926103f8565b610bca565b6103f8565b90565b611622906115fd565b90565b61162e90611619565b90565b9061163b90611625565b5f5260205260405f2090565b90565b61165e6116596116639261115f565b610bca565b610432565b90565b61169d9161169261168c6116876116989461167f611454565b50600a611631565b611647565b9161247a565b9061258f565b61164a565b90565b906116ba916116b56116b0610b2e565b6121da565b611725565b565b6116d06116cb6116d592610dba565b610bca565b6103f8565b90565b6116e1906116bc565b90565b6116f86116f36116fd92610dba565b610bca565b610432565b90565b61170f61171591939293610432565b92610432565b820180921161172057565b6111ee565b908161174161173b6117365f6116d8565b610403565b91610403565b146117d157806117596117535f6116e4565b91610432565b146117b557611770611769611481565b8290611700565b61178961178361177e610be9565b610432565b91610432565b1161179957611797916126b6565b565b5f63177e3fc360e01b8152806117b16004820161064a565b0390fd5b5f631f2a200560e01b8152806117cd6004820161064a565b0390fd5b5f63d92e233d60e01b8152806117e96004820161064a565b0390fd5b906117f7916116a0565b565b8061180c6118065f6116e4565b91610432565b1461181d5761181b9033612714565b565b5f631f2a200560e01b8152806118356004820161064a565b0390fd5b61184861184e91939293610432565b92610432565b820391821161185957565b6111ee565b611866611454565b50611880611872610be9565b61187a611481565b90611839565b90565b9061189661188f610292565b92836113c5565b565b67ffffffffffffffff81116118b6576118b2602091610370565b0190565b6113b1565b906118cd6118c883611898565b611883565b918252565b5f7f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c74000000910152565b611903601d6118bb565b90611910602083016118d2565b565b61191a6118f9565b90565b6119256112c3565b5061192e611bf0565b61194761194161193c612773565b610c77565b91610c77565b0361195757611954611912565b90565b5f6301bfc1c560e61b81528061196f6004820161064a565b0390fd5b5f90565b9061198190611625565b5f5260205260405f2090565b60018060a01b031690565b6119a46119a991611458565b61198d565b90565b6119b69054611998565b90565b6119d06119d5916119c8611973565b506009611977565b6119ac565b90565b6119e9906119e4612054565b6127c6565b565b5f90565b611a01906119fb6119eb565b50612851565b90565b90611a0e90611625565b5f5260205260405f2090565b611a30611a3591611a29611454565b505f611a04565b611474565b90565b611a4a90611a44611454565b50612880565b90565b5f90565b606090565b611a5f90611619565b90565b67ffffffffffffffff8111611a7a5760208091020190565b6113b1565b90611a91611a8c83611a62565b611883565b918252565b369037565b90611ac0611aa883611a7f565b92602080611ab68693611a62565b9201910390611a96565b565b600f60f81b90565b611ad2611a4d565b50611adb6112c3565b50611ae46112c3565b50611aed611454565b50611af6611973565b50611aff6114c6565b50611b08611a51565b50611b11612898565b90611b1a6128d8565b904690611b2630611a56565b90611b305f610dc2565b90611b42611b3d5f6116e4565b611a9b565b90611b4b611ac2565b96959493929190565b611b7d611b8291611b63611454565b50611b77611b71600b611647565b9161247a565b9061258f565b61164a565b90565b90611b8f90611625565b5f5260205260405f2090565b60ff1690565b611bad611bb291611458565b611b9b565b90565b611bbf9054611ba1565b90565b611be9915f611bde611be493611bd661127f565b5060056114d6565b01611b85565b611bb5565b90565b5f90565b611bf8611bec565b50611c01612773565b90565b611c0c6112c3565b50611c176004611410565b90565b611c41611c3c611c37611c4693611c2f611454565b50600a611631565b611647565b612918565b61164a565b90565b611c6691611c5561127f565b50611c5e612054565b91909161213d565b600190565b90611c7e91611c78611454565b50611666565b90565b611c9390611c8d611454565b50611c1a565b90565b611c9e611454565b50611ca7611481565b90565b7fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf90565b611d03611d0a94611cf9606094989795611cef608086019a5f8701906105c6565b602085019061088c565b60408301906104bb565b01906104bb565b565b60200190565b5190565b9395949092919542611d30611d2a89610432565b91610432565b11611da95791611d9b91611da293611d92611da79899611d7a611d51611caa565b611d6b8b938b611d5f610292565b95869460208601611cce565b602082018103825203826113c5565b611d8c611d8682611d12565b91611d0c565b2061298d565b929091926129aa565b91826129f4565b6127c6565b565b611dc4875f918291632341d78760e11b8352600483016104c8565b0390fd5b7f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c990565b9194611e34611e3e92989795611e2a60a096611e20611e459a611e1660c08a019e5f8b01906105c6565b602089019061088c565b604087019061088c565b60608501906104bb565b60808301906104bb565b01906104bb565b565b916020611e68929493611e6160408201965f83019061088c565b019061088c565b565b969591939294909442611e85611e7f83610432565b91610432565b11611f3f5790611eee611ef7949392611ed6611e9f611dc8565b611ec78c80948c91611eb18d91612a9b565b9192611ebb610292565b97889660208801611dec565b602082018103825203826113c5565b611ee8611ee282611d12565b91611d0c565b2061298d565b929091926129aa565b80611f0a611f0487610403565b91610403565b03611f1f5750611f1d9293919091612061565b565b8490611f3b5f9283926325c0072360e11b845260048401611e47565b0390fd5b611f5a905f91829163313c898160e11b8352600483016104c8565b0390fd5b90611f7991611f74611f6f82611510565b6121da565b611f7b565b565b90611f8591612399565b50565b90611f9291611f5e565b565b90611f9e90611625565b5f5260205260405f2090565b611fcf91611fc5611fca92611fbd611454565b506001611f94565b611a04565b611474565b90565b611fdc6040611883565b90565b5f90565b5f90565b611fef611fd2565b9060208083611ffc611fdf565b815201612007611fe3565b81525050565b612015611fe7565b90565b9061202b9161202561200d565b50612ace565b90565b61203661127f565b5061205061204a6301ffc9a760e01b6102a0565b916102a0565b1490565b61205c611973565b503390565b9161206f9291600192612af6565b565b60409061209a6120a1949695939661209060608401985f85019061088c565b60208301906104bb565b01906104bb565b565b906120ae9103610432565b90565b9291926120bf818390611faa565b90816120d46120ce5f19610432565b91610432565b106120e1575b5050509050565b816120f46120ee87610432565b91610432565b1061211a5761211193946121099193926120a3565b905f92612af6565b805f80806120da565b50612139849291925f938493637dc7a0d960e11b855260048501612071565b0390fd5b918261215961215361214e5f6116d8565b610403565b91610403565b146121b3578161217961217361216e5f6116d8565b610403565b91610403565b1461218c5761218a92919091612c05565b565b6121af6121985f6116d8565b5f91829163ec442f0560e01b835260048301610899565b0390fd5b6121d66121bf5f6116d8565b5f918291634b637e8f60e11b835260048301610899565b0390fd5b6121ec906121e6612054565b90612c37565b565b906121fa60ff91610dbd565b9181191691161790565b61220d906102ed565b90565b90565b9061222861222361222f92612204565b612210565b82546121ee565b9055565b61223b61127f565b5061225061224a828490611bc2565b156102ed565b5f146122d95761227860016122735f61226b600586906114d6565b018590611b85565b612213565b90612281612054565b906122be6122b86122b27f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d956114ca565b92611625565b92611625565b926122c7610292565b806122d18161064a565b0390a4600190565b50505f90565b6122e76114c6565b506122f130611a56565b61232361231d7f0000000000000000000000000000000000000000000000000000000000000000610403565b91610403565b148061235f575b5f14612354577f000000000000000000000000000000000000000000000000000000000000000090565b61235c612ce3565b90565b504661239361238d7f0000000000000000000000000000000000000000000000000000000000000000610432565b91610432565b1461232a565b6123a161127f565b506123ad818390611bc2565b5f14612435576123d45f6123cf5f6123c7600586906114d6565b018590611b85565b612213565b906123dd612054565b9061241a61241461240e7ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b956114ca565b92611625565b92611625565b92612423610292565b8061242d8161064a565b0390a4600190565b50505f90565b61244f61244a61245492610c77565b610bca565b610432565b90565b91602061247892949361247160408201965f8301906104bb565b0190610c82565b565b612482611bec565b5061248b611bf0565b8161249e6124988361243b565b91610432565b10156124b157506124ae90612dec565b90565b906124cc5f928392637669fc0f60e11b845260048401612457565b0390fd5b5490565b90565b6124eb6124e66124f0926124d4565b610bca565b610432565b90565b90565b65ffffffffffff1690565b61250d61251291611458565b6124f6565b90565b61251f9054612501565b90565b90565b61253961253461253e92612522565b610bca565b610432565b90565b60301c90565b60018060d01b031690565b61255e61256391612541565b612547565b90565b6125709054612552565b90565b61258761258261258c92610dba565b610bca565b61115f565b90565b906125e39061259c6111ea565b506125a85f84016124d0565b6125b15f6116e4565b9080806125c76125c160056124d7565b91610432565b11612644575b50906125de5f8601939192936124f3565b613443565b806125f66125f05f6116e4565b91610432565b145f1461260c5750506126085f612573565b5b90565b6126395f9161263461262e8461263f9601926126286001612525565b90611839565b916124f3565b613439565b01612566565b612609565b8061265261265892916130ce565b90611839565b908361268a61268461267f5f612679818c0161267489916124f3565b613439565b01612515565b610c77565b91610c77565b105f1461269b5750905b905f6125cd565b91506126b1906126ab6001612525565b90611700565b612694565b806126d16126cb6126c65f6116d8565b610403565b91610403565b146126ed576126eb916126e35f6116d8565b919091612c05565b565b6127106126f95f6116d8565b5f91829163ec442f0560e01b835260048301610899565b0390fd5b908161273061272a6127255f6116d8565b610403565b91610403565b1461274c5761274a91906127435f6116d8565b9091612c05565b565b61276f6127585f6116d8565b5f918291634b637e8f60e11b835260048301610899565b0390fd5b61277b611bec565b5061278543612dec565b90565b9061279960018060a01b0391610dbd565b9181191691161790565b90565b906127bb6127b66127c292611625565b6127a3565b8254612788565b9055565b9061284f916128496127d7826119b9565b6127ec846127e760098690611977565b6127a6565b8281859061282c6128266128207f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f95611625565b92611625565b92611625565b92612835610292565b8061283f8161064a565b0390a492916134d2565b9161350d565b565b61287861287361286e61287d936128666119eb565b50600a611631565b611647565b6136bb565b61373a565b90565b6128929061288c611454565b5061378b565b90565b90565b6128a06112c3565b506128d57f00000000000000000000000000000000000000000000000000000000000000006128cf6006612895565b906138a6565b90565b6128e06112c3565b506129157f000000000000000000000000000000000000000000000000000000000000000061290f6007612895565b906138a6565b90565b6129206111ea565b5061292c5f82016124d0565b8061293f6129395f6116e4565b91610432565b145f146129555750506129515f612573565b5b90565b6129825f9161297d612977846129889601926129716001612525565b90611839565b916124f3565b613439565b01612566565b612952565b6129a7906129996114c6565b506129a26122df565b6138f4565b90565b926129c5926129ce946129bb611973565b50929091926139ba565b90929192613ae5565b90565b9160206129f29294936129eb60408201965f83019061088c565b01906104bb565b565b6129fd81612a9b565b91612a10612a0a84610432565b91610432565b03612a19575050565b612a335f9283926301d4b62360e61b8452600484016129d1565b0390fd5b6001612a439101610432565b90565b90612a525f1991610dbd565b9181191691161790565b612a70612a6b612a7592610432565b610bca565b610432565b90565b90565b90612a90612a8b612a9792612a5c565b612a78565b8254612a46565b9055565b612aaf90612aa7611454565b506008611a04565b612acb612abb82611474565b91612ac583612a37565b90612a7b565b90565b90612aee612ae9612af393612ae161200d565b50600a611631565b611647565b613c5b565b90565b909281612b13612b0d612b085f6116d8565b610403565b91610403565b14612bde5783612b33612b2d612b285f6116d8565b610403565b91610403565b14612bb757612b5783612b52612b4b60018690611f94565b8790611a04565b612a7b565b612b61575b505050565b919091612bac612b9a612b947f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92593611625565b93611625565b93612ba3610292565b918291826104c8565b0390a35f8080612b5c565b612bda612bc35f6116d8565b5f918291634a1406b160e11b835260048301610899565b0390fd5b612c01612bea5f6116d8565b5f91829163e602df0560e01b835260048301610899565b0390fd5b91612c1292919091613c7c565b565b916020612c35929493612c2e60408201965f83019061088c565b01906105c6565b565b90612c4c612c46838390611bc2565b156102ed565b612c54575050565b612c6e5f92839263e2517d3f60e01b845260048401612c14565b0390fd5b7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f90565b90959492612ce194612cd0612cda92612cc6608096612cbc60a088019c5f8901906105c6565b60208701906105c6565b60408501906105c6565b60608301906104bb565b019061088c565b565b612ceb6114c6565b50612cf4612c72565b612d6b7f000000000000000000000000000000000000000000000000000000000000000091612d5c7f000000000000000000000000000000000000000000000000000000000000000046612d4730611a56565b91612d50610292565b96879560208701612c96565b602082018103825203826113c5565b612d7d612d7782611d12565b91611d0c565b2090565b90565b612d98612d93612d9d92612d81565b610bca565b610683565b90565b612da990612d84565b9052565b916020612dce929493612dc760408201965f830190612da0565b01906104bb565b565b612de4612ddf612de992610432565b610bca565b610c77565b90565b612df4611bec565b5080612e0e612e0865ffffffffffff61243b565b91610432565b11612e1f57612e1c90612dd0565b90565b6030612e3b5f9283926306dfcc6560e41b845260048401612dad565b0390fd5b90565b612e56612e51612e5b92612e3f565b610bca565b610432565b90565b90565b612e75612e70612e7a92612e5e565b610bca565b610683565b90565b1c90565b612ea090612e9a612e94612ea594610683565b91610432565b90612e7d565b610432565b90565b90565b612ebf612eba612ec492612ea8565b610bca565b610683565b90565b1b90565b612eea90612ee4612ede612eef94610683565b91610432565b90612ec7565b610432565b90565b90565b612f09612f04612f0e92612ef2565b610bca565b610432565b90565b90565b612f28612f23612f2d92612f11565b610bca565b610683565b90565b90565b612f47612f42612f4c92612f30565b610bca565b610432565b90565b90565b612f66612f61612f6b92612f4f565b610bca565b610683565b90565b90565b612f85612f80612f8a92612f6e565b610bca565b610432565b90565b90565b612fa4612f9f612fa992612f8d565b610bca565b610683565b90565b90565b612fc3612fbe612fc892612fac565b610bca565b610432565b90565b90565b612fe2612fdd612fe792612fcb565b610bca565b610683565b90565b612ffe612ff961300392612f4f565b610bca565b610432565b90565b90565b61301d61301861302292613006565b610bca565b610683565b90565b61303961303461303e92612fcb565b610bca565b610432565b90565b61305561305061305a92612522565b610bca565b610683565b90565b90565b61307461306f6130799261305d565b610bca565b610432565b90565b906130879102610432565b90565b634e487b7160e01b5f52601260045260245ffd5b6130aa6130b091610432565b91610432565b9081156130bb570490565b61308a565b906130cb9101610432565b90565b6130d6611454565b50806130eb6130e56001612525565b91610432565b111561343657806133006132dd6132cd6132bd6132ad61329d61328d61327d61326d61325d61324d8b6132476132406133069f613220613210613230926131326001612525565b908061314a613144600160801b612e42565b91610432565b1015613408575b8061316d61316768010000000000000000612ef5565b91610432565b10156133da575b8061318c613186640100000000612f33565b91610432565b10156133ac575b806131a96131a362010000612f71565b91610432565b101561337e575b806131c56131bf610100612faf565b91610432565b1015613350575b806131e06131da6010612fea565b91610432565b1015613322575b6131fa6131f46004613025565b91610432565b1015613309575b61320b6003613060565b61307c565b61321a6001613041565b90612e81565b61322a818661309e565b906130c0565b61323a6001613041565b90612e81565b809261309e565b906130c0565b6132576001613041565b90612e81565b613267818c61309e565b906130c0565b6132776001613041565b90612e81565b613287818a61309e565b906130c0565b6132976001613041565b90612e81565b6132a7818861309e565b906130c0565b6132b76001613041565b90612e81565b6132c7818661309e565b906130c0565b6132d76001613041565b90612e81565b916132fa6132f46132ef85809461309e565b610432565b91610432565b11613d0c565b906120a3565b90565b61331d906133176001613041565b90612ecb565b613201565b61333961334a916133336004612fce565b90612e81565b916133446002613009565b90612ecb565b906131e7565b613367613378916133616008612f90565b90612e81565b916133726004612fce565b90612ecb565b906131cc565b6133956133a69161338f6010612f52565b90612e81565b916133a06008612f90565b90612ecb565b906131b0565b6133c36133d4916133bd6020612f14565b90612e81565b916133ce6010612f52565b90612ecb565b90613193565b6133f1613402916133eb6040612eab565b90612e81565b916133fc6020612f14565b90612ecb565b90613174565b61341f613430916134196080612e61565b90612e81565b9161342a6040612eab565b90612ecb565b90613151565b90565b5f5260205f200190565b9391909261344f611454565b505b8161346461345e83610432565b91610432565b10156134ca57613475828290613d58565b9061348b5f613485888590613439565b01612515565b61349d61349787610c77565b91610c77565b115f146134ad5750915b91613451565b9291506134c4906134be6001612525565b90611700565b906134a7565b925050915090565b6134e4906134de611454565b50611a1a565b90565b90565b91602061350b92949361350460408201965f8301906104bb565b01906104bb565b565b9190918061352361351d85610403565b91610403565b1415806136a1575b613535575b505050565b8061355061354a6135455f6116d8565b610403565b91610403565b03613611575b508161357261356c6135675f6116d8565b610403565b91610403565b0361357e575b80613530565b6135c56135b86135bf92613594600a8690611631565b906135b26135ac6135a6600193613df1565b93611647565b916134e7565b90613e44565b929061164a565b9161164a565b9190916135f27fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72492611625565b926136076135fe610292565b928392836134ea565b0390a25f80613578565b613650613656613649613626600a8590611631565b600261364361363d61363789613df1565b93611647565b916134e7565b90613e44565b929061164a565b9161164a565b9190916136837fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72492611625565b9261369861368f610292565b928392836134ea565b0390a25f613556565b50816136b56136af5f6116e4565b91610432565b1161352b565b5f6136cf916136c8611454565b50016124d0565b90565b6136e66136e16136eb92610916565b610bca565b610432565b90565b6136f790612f14565b9052565b91602061371c92949361371560408201965f8301906136ee565b01906104bb565b565b61373261372d61373792610432565b610bca565b610916565b90565b6137426119eb565b508061375a61375463ffffffff6136d2565b91610432565b1161376b576137689061371e565b90565b60206137875f9283926306dfcc6560e41b8452600484016136fb565b0390fd5b6137a26137a79161379a611454565b506008611a04565b611474565b90565b90565b6137c16137bc6137c6926137aa565b610dbd565b610582565b90565b6137d360ff6137ad565b90565b5f5260205f2090565b905f92918054906137f96137f2836112dc565b8094611306565b916001811690815f146138505750600114613814575b505050565b61382191929394506137d6565b915f925b81841061383857505001905f808061380f565b60018160209295939554848601520191019290613825565b92949550505060ff19168252151560200201905f808061380f565b90613875916137df565b90565b9061389861389192613888610292565b9384809261386b565b03836113c5565b565b6138a390613878565b90565b906138af6112c3565b506138b9826114ca565b6138d26138cc6138c76137c9565b610582565b91610582565b14155f146138e757506138e490613ece565b90565b6138f1915061389a565b90565b6042916138ff6114c6565b50604051917f19010000000000000000000000000000000000000000000000000000000000008352600283015260228201522090565b5f90565b61394561394a91611458565b612a5c565b90565b90565b61396461395f6139699261394d565b610bca565b610432565b90565b6139a16139a89461399760609498979561398d608086019a5f8701906105c6565b6020850190610689565b60408301906105c6565b01906105c6565b565b6139b2610292565b3d5f823e3d90fd5b9392936139c5611973565b506139ce613935565b506139d76114c6565b506139e185613939565b613a13613a0d7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0613950565b91610432565b11613aa05790613a36602094955f94939293613a2d610292565b9485948561396c565b838052039060015afa15613a9b57613a4e5f51610dbd565b80613a69613a63613a5e5f6116d8565b610403565b91610403565b14613a7f575f91613a795f610dc2565b91929190565b50613a895f6116d8565b600191613a955f610dc2565b91929190565b6139aa565b505050613aac5f6116d8565b9060039291929190565b634e487b7160e01b5f52602160045260245ffd5b60041115613ad457565b613ab6565b90613ae382613aca565b565b80613af8613af25f613ad9565b91613ad9565b145f14613b03575050565b80613b17613b116001613ad9565b91613ad9565b145f14613b3a575f63f645eedf60e01b815280613b366004820161064a565b0390fd5b80613b4e613b486002613ad9565b91613ad9565b145f14613b7c57613b78613b6183613939565b5f91829163fce698f760e01b8352600483016104c8565b0390fd5b613b8f613b896003613ad9565b91613ad9565b14613b975750565b613bb2905f9182916335e2f38360e21b8352600483016105d3565b0390fd5b634e487b7160e01b5f52603260045260245ffd5b5f5260205f2090565b613bdc816124d0565b821015613bf657613bee600191613bca565b910201905f90565b613bb6565b90613c0590610c77565b9052565b90613c139061115f565b9052565b90613c4d613c445f613c27611fd2565b94613c3e613c36838301612515565b838801613bfb565b01612566565b60208401613c09565b565b613c5890613c17565b90565b613c79915f613c7392613c6c61200d565b5001613bd3565b50613c4f565b90565b9291613c8a84838391613efe565b83613ca5613c9f613c9a5f6116d8565b610403565b91610403565b14613cba575b613cb89293919091614088565b565b613cc2611481565b93613ccb61406d565b9480613cdf613cd988610432565b91610432565b11613cec57509350613cab565b8590613d085f928392630e58ae9360e11b8452600484016134ea565b0390fd5b613d14611454565b50151590565b613d2e613d29613d3392613006565b610bca565b610432565b90565b613d42613d4891610432565b91610432565b908115613d53570490565b61308a565b613d7d613d8392613d67611454565b508281169218613d776002613d1a565b90613d36565b90611700565b90565b90565b613d9d613d98613da292613d86565b610bca565b610683565b90565b613dae90613d89565b9052565b916020613dd3929493613dcc60408201965f830190613da5565b01906104bb565b565b613de9613de4613dee92610432565b610bca565b61115f565b90565b613df96111ea565b5080613e13613e0d60018060d01b0361164a565b91610432565b11613e2457613e2190613dd5565b90565b60d0613e405f9283926306dfcc6560e41b845260048401613db2565b0390fd5b90613e7a613e809392613e556111ea565b50613e5e6111ea565b508093613e73613e6c611bf0565b9492612918565b9091614503565b91614147565b91909190565b613e9a613e95613e9f92612f11565b610bca565b610432565b90565b369037565b90613ecc613eb4836118bb565b92602080613ec28693611898565b9201910390613ea2565b565b613ed66112c3565b50613ee0816141b1565b90613ef3613eee6020613e86565b613ea7565b918252602082015290565b91909180613f1c613f16613f115f6116d8565b610403565b91610403565b145f14613ffd57613f40613f3983613f346002611474565b611700565b6002612a7b565b5b82613f5c613f56613f515f6116d8565b610403565b91610403565b145f14613fd157613f80613f7983613f746002611474565b6120a3565b6002612a7b565b5b919091613fcc613fba613fb47fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef93611625565b93611625565b93613fc3610292565b918291826104c8565b0390a3565b613ff882613ff2613fe35f8790611a04565b91613fed83611474565b6130c0565b90612a7b565b613f81565b61401061400b5f8390611a04565b611474565b8061402361401d85610432565b91610432565b1061404b576140366140469184906120a3565b6140415f8490611a04565b612a7b565b613f41565b906140699091925f93849363391434e360e21b855260048501612071565b0390fd5b614075611454565b5061408560018060d01b0361164a565b90565b916140e06140da6140e794806140ae6140a86140a35f6116d8565b610403565b91610403565b14614118575b846140cf6140c96140c45f6116d8565b610403565b91610403565b146140e9575b6119b9565b926119b9565b909161350d565b565b614111600b600261410b6141056140ff89613df1565b93611647565b916134e7565b90613e44565b50506140d5565b614140600b600161413a61413461412e89613df1565b93611647565b916134e7565b90613e44565b50506140b4565b9161416b5f614170946141586111ea565b506141616111ea565b50019291926124f3565b6143b5565b91909190565b61418a61418561418f926137aa565b610bca565b610432565b90565b90565b6141a96141a46141ae92614192565b610bca565b610432565b90565b6141c66141cb916141c0611454565b506114ca565b613939565b6141d560ff614176565b16806141ea6141e4601f614195565b91610432565b116141f25790565b5f632cd44ac360e21b81528061420a6004820161064a565b0390fd5b5490565b61421c6040611883565b90565b5f5260205f2090565b6142318161420e565b82101561424b5761424360019161421f565b910201905f90565b613bb6565b634e487b7160e01b5f525f60045260245ffd5b61426d9051610c77565b90565b9061428165ffffffffffff91610dbd565b9181191691161790565b61429f61429a6142a492610c77565b610bca565b610c77565b90565b90565b906142bf6142ba6142c69261428b565b6142a7565b8254614270565b9055565b6142d4905161115f565b90565b60301b90565b906142ef65ffffffffffff19916142d7565b9181191691161790565b61430d6143086143129261115f565b610bca565b61115f565b90565b90565b9061432d614328614334926142f9565b614315565b82546142dd565b9055565b9061436260205f6143689461435a828201614354848801614263565b906142aa565b0192016142ca565b90614318565b565b919061437b5761437991614338565b565b614250565b90815491680100000000000000008310156143b057826143a89160016143ae95018155614228565b9061436a565b565b6113b1565b909291926143c16111ea565b506143ca6111ea565b506143d48261420e565b806143e76143e15f6116e4565b91610432565b115f146144b75761440d9061440784916144016001612525565b90611839565b90613439565b906144195f8301612515565b926144255f8401612566565b938061443961443385610c77565b91610c77565b1161449b5761445061444a84610c77565b91610c77565b145f1461446b575050614466905f859101614318565b5b9190565b61449692506144918661448861447f614212565b945f8601613bfb565b60208401613c09565b614380565b614467565b5f632520601d60e01b8152806144b36004820161064a565b0390fd5b506144e2916144dd856144d46144cb614212565b945f8601613bfb565b60208401613c09565b614380565b6144eb5f612573565b9190565b634e487b7160e01b5f52605160045260245ffd5b91909180600114614522576002036144ef5761451e91611269565b905b565b5061452c9161122a565b9061452056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01``@R4a\0sWa\0\x1Ba\0\x15a\x01GV[\x90a\x02\xD4V[a\0#a\0xV[aE2a\x1B\t\x829`\x80Q\x81a#2\x01R`\xA0Q\x81a#i\x01R`\xC0Q\x81a\"\xF9\x01R`\xE0Q\x81a,\xF9\x01Ra\x01\0Q\x81a-\x1E\x01Ra\x01 Q\x81a(\xA6\x01Ra\x01@Q\x81a(\xE6\x01RaE2\x90\xF3[a\0~V[`@Q\x90V[_\x80\xFD[`\x1F\x80\x19\x91\x01\x16\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\0\xAA\x90a\0\x82V[\x81\x01\x90\x81\x10`\x01\x80`@\x1B\x03\x82\x11\x17a\0\xC2W`@RV[a\0\x8CV[\x90a\0\xDAa\0\xD3a\0xV[\x92\x83a\0\xA0V[V[_\x80\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\0\xF4\x90a\0\xE0V[\x90V[a\x01\0\x81a\0\xEBV[\x03a\x01\x07WV[_\x80\xFD[\x90PQ\x90a\x01\x18\x82a\0\xF7V[V[\x91\x90`@\x83\x82\x03\x12a\x01BW\x80a\x016a\x01?\x92_\x86\x01a\x01\x0BV[\x93` \x01a\x01\x0BV[\x90V[a\0\xDCV[a\x01ea`;\x808\x03\x80a\x01Z\x81a\0\xC7V[\x92\x839\x81\x01\x90a\x01\x1AV[\x90\x91V[`\x01\x80`@\x1B\x03\x81\x11a\x01\x85Wa\x01\x81` \x91a\0\x82V[\x01\x90V[a\0\x8CV[\x90a\x01\x9Ca\x01\x97\x83a\x01iV[a\0\xC7V[\x91\x82RV[_\x7FSyndicate\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x01\xD2`\ta\x01\x8AV[\x90a\x01\xDF` \x83\x01a\x01\xA1V[V[a\x01\xE9a\x01\xC8V[\x90V[_\x7FSYND\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x02\x1D`\x04a\x01\x8AV[\x90a\x02*` \x83\x01a\x01\xECV[V[a\x024a\x02\x13V[\x90V[\x90V[\x90V[a\x02Qa\x02La\x02V\x92a\x027V[a\x02:V[a\0\xE0V[\x90V[a\x02b\x90a\x02=V[\x90V[_\x01\x90V[\x90V[_\x1B\x90V[a\x02\x86a\x02\x81a\x02\x8B\x92a\x027V[a\x02mV[a\x02jV[\x90V[a\x02\x97_a\x02rV[\x90V[\x90V[\x90V[a\x02\xB4a\x02\xAFa\x02\xB9\x92a\x02\x9AV[a\x02:V[a\x02\x9DV[\x90V[a\x02\xD1k\x02\xE8vi\xC3\x08sj\x04\0\0\0a\x02\xA0V[\x90V[\x90a\x02\xF6a\x02\xE0a\x01\xE1V[a\x02\xE8a\x01\xE1V[a\x02\xF0a\x02,V[\x91a\x03\x93V[\x81a\x03\x11a\x03\x0Ba\x03\x06_a\x02YV[a\0\xEBV[\x91a\0\xEBV[\x14a\x03wW\x80a\x031a\x03+a\x03&_a\x02YV[a\0\xEBV[\x91a\0\xEBV[\x14a\x03[Wa\x03Ja\x03Y\x92a\x03Ea\x02\x8EV[a\x08NV[Pa\x03Sa\x02\xBCV[\x90a\t\x1CV[V[_c\xD9.#=`\xE0\x1B\x81R\x80a\x03s`\x04\x82\x01a\x02eV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x03\x8F`\x04\x82\x01a\x02eV[\x03\x90\xFD[\x90a\x03\x9E\x92\x91a\x03\xA0V[V[\x90a\x03\xAB\x92\x91a\x03\xADV[V[\x90a\x03\xB8\x92\x91a\x03\xBAV[V[\x90a\x03\xC5\x92\x91a\x03\xC7V[V[\x90a\x03\xD2\x92\x91a\x04\x1FV[V[_\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x01RV[a\x04\x05`\x01a\x01\x8AV[\x90a\x04\x12` \x83\x01a\x03\xD4V[V[a\x04\x1Ca\x03\xFBV[\x90V[\x90a\x043\x92\x91a\x04-a\x04\x14V[\x90a\x045V[V[\x90a\x04A\x93\x92\x91a\x04\x87V[V[\x90V[\x90V[` \x01\x90V[Q\x90V[a\x04ga\x04ba\x04l\x92a\0\xE0V[a\x02:V[a\0\xE0V[\x90V[a\x04x\x90a\x04SV[\x90V[a\x04\x84\x90a\x04oV[\x90V[a\x04\x98a\x04\xE8\x94a\x04\xCD\x93\x94a\x05\x1CV[a\x04\xAC\x81a\x04\xA6`\x06a\x04CV[\x90a\t\xC9V[a\x01 Ra\x04\xC4\x83a\x04\xBE`\x07a\x04CV[\x90a\t\xC9V[a\x01@Ra\x04FV[a\x04\xDFa\x04\xD9\x82a\x04OV[\x91a\x04IV[ `\xE0Ra\x04FV[a\x04\xFAa\x04\xF4\x82a\x04OV[\x91a\x04IV[ a\x01\0RF`\xA0Ra\x05\x0Ba\n\xCEV[`\x80Ra\x05\x170a\x04{V[`\xC0RV[\x90a\x05&\x91a\x05(V[V[\x90a\x052\x91a\x054V[V[\x90a\x05>\x91a\x07\xA4V[V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[Q\x90V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x01`\x02\x83\x04\x92\x16\x80\x15a\x05\x8BW[` \x83\x10\x14a\x05\x86WV[a\x05WV[\x91`\x7F\x16\x91a\x05{V[_R` _ \x90V[`\x1F` \x91\x01\x04\x90V[\x1B\x90V[\x91\x90`\x08a\x05\xC7\x91\x02\x91a\x05\xC1_\x19\x84a\x05\xA8V[\x92a\x05\xA8V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x05\xE5a\x05\xE0a\x05\xEA\x92a\x02\x9DV[a\x02:V[a\x02\x9DV[\x90V[\x90V[\x91\x90a\x06\x06a\x06\x01a\x06\x0E\x93a\x05\xD1V[a\x05\xEDV[\x90\x83Ta\x05\xACV[\x90UV[_\x90V[a\x06(\x91a\x06\"a\x06\x12V[\x91a\x05\xF0V[V[[\x81\x81\x10a\x066WPPV[\x80a\x06C_`\x01\x93a\x06\x16V[\x01a\x06+V[\x91\x90`\x1F\x81\x11a\x06YW[PPPV[a\x06ea\x06\x8A\x93a\x05\x95V[\x90` a\x06q\x84a\x05\x9EV[\x83\x01\x93\x10a\x06\x92W[a\x06\x83\x90a\x05\x9EV[\x01\x90a\x06*V[_\x80\x80a\x06TV[\x91Pa\x06\x83\x81\x92\x90Pa\x06zV[\x1C\x90V[\x90a\x06\xB4\x90_\x19\x90`\x08\x02a\x06\xA0V[\x19\x16\x90V[\x81a\x06\xC3\x91a\x06\xA4V[\x90`\x02\x02\x17\x90V[\x90a\x06\xD5\x81a\x05SV[\x90`\x01\x80`@\x1B\x03\x82\x11a\x07\x93Wa\x06\xF7\x82a\x06\xF1\x85Ta\x05kV[\x85a\x06IV[` \x90`\x1F\x83\x11`\x01\x14a\x07+W\x91\x80\x91a\x07\x1A\x93_\x92a\x07\x1FW[PPa\x06\xB9V[\x90U[V[\x90\x91P\x01Q_\x80a\x07\x13V[`\x1F\x19\x83\x16\x91a\x07:\x85a\x05\x95V[\x92_[\x81\x81\x10a\x07{WP\x91`\x02\x93\x91\x85`\x01\x96\x94\x10a\x07aW[PPP\x02\x01\x90Ua\x07\x1DV[a\x07q\x91\x01Q`\x1F\x84\x16\x90a\x06\xA4V[\x90U_\x80\x80a\x07UV[\x91\x93` `\x01\x81\x92\x87\x87\x01Q\x81U\x01\x95\x01\x92\x01a\x07=V[a\0\x8CV[\x90a\x07\xA2\x91a\x06\xCBV[V[\x90a\x07\xB3a\x07\xBA\x92`\x03a\x07\x98V[`\x04a\x07\x98V[V[_\x90V[\x15\x15\x90V[a\x07\xCE\x90a\x02jV[\x90V[\x90a\x07\xDB\x90a\x07\xC5V[_R` R`@_ \x90V[a\x07\xF0\x90a\x04oV[\x90V[\x90a\x07\xFD\x90a\x07\xE7V[_R` R`@_ \x90V[\x90a\x08\x15`\xFF\x91a\x02mV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x08(\x90a\x07\xC0V[\x90V[\x90V[\x90a\x08Ca\x08>a\x08J\x92a\x08\x1FV[a\x08+V[\x82Ta\x08\tV[\x90UV[a\x08Va\x07\xBCV[Pa\x08ka\x08e\x82\x84\x90a\x0BkV[\x15a\x07\xC0V[_\x14a\x08\xF4Wa\x08\x93`\x01a\x08\x8E_a\x08\x86`\x05\x86\x90a\x07\xD1V[\x01\x85\x90a\x07\xF3V[a\x08.V[\x90a\x08\x9Ca\x0B\x99V[\x90a\x08\xD9a\x08\xD3a\x08\xCD\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x95a\x07\xC5V[\x92a\x07\xE7V[\x92a\x07\xE7V[\x92a\x08\xE2a\0xV[\x80a\x08\xEC\x81a\x02eV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a\t\x03\x90a\0\xEBV[\x90RV[\x91\x90a\t\x1A\x90_` \x85\x01\x94\x01\x90a\x08\xFAV[V[\x80a\t7a\t1a\t,_a\x02YV[a\0\xEBV[\x91a\0\xEBV[\x14a\tSWa\tQ\x91a\tI_a\x02YV[\x91\x90\x91a\x0B\xA6V[V[a\tva\t__a\x02YV[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\t\x07V[\x03\x90\xFD[_\x90V[\x90V[a\t\x95a\t\x90a\t\x9A\x92a\t~V[a\x02:V[a\x02\x9DV[\x90V[\x90V[a\t\xB4a\t\xAFa\t\xB9\x92a\t\x9DV[a\x02mV[a\x02jV[\x90V[a\t\xC6`\xFFa\t\xA0V[\x90V[\x90a\t\xD2a\tzV[Pa\t\xE4a\t\xDF\x83a\x04FV[a\x04OV[a\t\xF7a\t\xF1` a\t\x81V[\x91a\x02\x9DV[\x10_\x14a\n\x0BWPa\n\x08\x90a\x0C\xA5V[\x90V[_a\n\x19a\n\x1F\x93\x92a\x0B\xB5V[\x01a\x07\x98V[a\n/a\n*a\t\xBCV[a\x07\xC5V[\x90V[_\x90V[\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x90V[a\nd\x90Qa\x02jV[\x90V[a\np\x90a\x02jV[\x90RV[a\n}\x90a\x02\x9DV[\x90RV[\x90\x95\x94\x92a\n\xCC\x94a\n\xBBa\n\xC5\x92a\n\xB1`\x80\x96a\n\xA7`\xA0\x88\x01\x9C_\x89\x01\x90a\ngV[` \x87\x01\x90a\ngV[`@\x85\x01\x90a\ngV[``\x83\x01\x90a\ntV[\x01\x90a\x08\xFAV[V[a\n\xD6a\n2V[Pa\n\xDFa\n6V[a\x0B)a\n\xEC`\xE0a\nZV[\x91a\x0B\x1Aa\n\xFBa\x01\0a\nZV[Fa\x0B\x050a\x04{V[\x91a\x0B\x0Ea\0xV[\x96\x87\x95` \x87\x01a\n\x81V[` \x82\x01\x81\x03\x82R\x03\x82a\0\xA0V[a\x0B;a\x0B5\x82a\x04OV[\x91a\x04IV[ \x90V[_\x1C\x90V[`\xFF\x16\x90V[a\x0BVa\x0B[\x91a\x0B?V[a\x0BDV[\x90V[a\x0Bh\x90Ta\x0BJV[\x90V[a\x0B\x92\x91_a\x0B\x87a\x0B\x8D\x93a\x0B\x7Fa\x07\xBCV[P`\x05a\x07\xD1V[\x01a\x07\xF3V[a\x0B^V[\x90V[_\x90V[a\x0B\xA1a\x0B\x95V[P3\x90V[\x91a\x0B\xB3\x92\x91\x90\x91a\rRV[V[\x90V[\x90V[a\x0B\xCFa\x0B\xCAa\x0B\xD4\x92a\x0B\xB8V[a\x02:V[a\x02\x9DV[\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[a\x0C\na\x0C\x13` \x93a\x0C\x18\x93a\x0C\x01\x81a\x05SV[\x93\x84\x80\x93a\x0B\xD7V[\x95\x86\x91\x01a\x0B\xE0V[a\0\x82V[\x01\x90V[a\x0C1\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x0B\xEBV[\x90V[a\x0CNa\x0CIa\x0CC\x83a\x04OV[\x92a\x04IV[a\nZV[\x90` \x81\x10a\x0C\\W[P\x90V[a\x0Cn\x90_\x19\x90` \x03`\x08\x02a\x05\xA8V[\x16_a\x0CXV[a\x0C\x81a\x0C\x86\x91a\x0B?V[a\x05\xD1V[\x90V[a\x0C\x9Da\x0C\x98a\x0C\xA2\x92a\x02\x9DV[a\x02mV[a\x02jV[\x90V[a\x0C\xADa\tzV[Pa\x0C\xB7\x81a\x04FV[\x90a\x0C\xC1\x82a\x04OV[a\x0C\xD4a\x0C\xCE`\x1Fa\x0B\xBBV[\x91a\x02\x9DV[\x11a\r\tWPa\r\x01\x81a\x0C\xFBa\x0C\xF5a\x0C\xF0a\r\x06\x95a\x0C4V[a\x0CuV[\x91a\x04OV[\x17a\x0C\x89V[a\x07\xC5V[\x90V[a\r+\x90a\r\x15a\0xV[\x91\x82\x91c0Z'\xA9`\xE0\x1B\x83R`\x04\x83\x01a\x0C\x1CV[\x03\x90\xFD[\x91` a\rP\x92\x94\x93a\rI`@\x82\x01\x96_\x83\x01\x90a\ntV[\x01\x90a\ntV[V[\x92\x91a\r`\x84\x83\x83\x91a\x0E\xEEV[\x83a\r{a\rua\rp_a\x02YV[a\0\xEBV[\x91a\0\xEBV[\x14a\r\x90W[a\r\x8E\x92\x93\x91\x90\x91a\x10\xBBV[V[a\r\x98a\x10]V[\x93a\r\xA1a\x10\x9AV[\x94\x80a\r\xB5a\r\xAF\x88a\x02\x9DV[\x91a\x02\x9DV[\x11a\r\xC2WP\x93Pa\r\x81V[\x85\x90a\r\xDE_\x92\x83\x92c\x0EX\xAE\x93`\xE1\x1B\x84R`\x04\x84\x01a\r/V[\x03\x90\xFD[\x90a\r\xEC\x90a\x07\xE7V[_R` R`@_ \x90V[\x90V[a\x0E\x07a\x0E\x0C\x91a\x0B?V[a\r\xF8V[\x90V[a\x0E\x19\x90Ta\r\xFBV[\x90V[`@\x90a\x0EEa\x0EL\x94\x96\x95\x93\x96a\x0E;``\x84\x01\x98_\x85\x01\x90a\x08\xFAV[` \x83\x01\x90a\ntV[\x01\x90a\ntV[V[\x90a\x0EY\x91\x03a\x02\x9DV[\x90V[\x90a\x0Eh_\x19\x91a\x02mV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90a\x0E\x87a\x0E\x82a\x0E\x8E\x92a\x05\xD1V[a\x05\xEDV[\x82Ta\x0E\\V[\x90UV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x0E\xB5a\x0E\xBB\x91\x93\x92\x93a\x02\x9DV[\x92a\x02\x9DV[\x82\x01\x80\x92\x11a\x0E\xC6WV[a\x0E\x92V[\x90a\x0E\xD6\x91\x01a\x02\x9DV[\x90V[\x91\x90a\x0E\xEC\x90_` \x85\x01\x94\x01\x90a\ntV[V[\x91\x90\x91\x80a\x0F\x0Ca\x0F\x06a\x0F\x01_a\x02YV[a\0\xEBV[\x91a\0\xEBV[\x14_\x14a\x0F\xEDWa\x0F0a\x0F)\x83a\x0F$`\x02a\x0E\x0FV[a\x0E\xA6V[`\x02a\x0ErV[[\x82a\x0FLa\x0FFa\x0FA_a\x02YV[a\0\xEBV[\x91a\0\xEBV[\x14_\x14a\x0F\xC1Wa\x0Fpa\x0Fi\x83a\x0Fd`\x02a\x0E\x0FV[a\x0ENV[`\x02a\x0ErV[[\x91\x90\x91a\x0F\xBCa\x0F\xAAa\x0F\xA4\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93a\x07\xE7V[\x93a\x07\xE7V[\x93a\x0F\xB3a\0xV[\x91\x82\x91\x82a\x0E\xD9V[\x03\x90\xA3V[a\x0F\xE8\x82a\x0F\xE2a\x0F\xD3_\x87\x90a\r\xE2V[\x91a\x0F\xDD\x83a\x0E\x0FV[a\x0E\xCBV[\x90a\x0ErV[a\x0FqV[a\x10\0a\x0F\xFB_\x83\x90a\r\xE2V[a\x0E\x0FV[\x80a\x10\x13a\x10\r\x85a\x02\x9DV[\x91a\x02\x9DV[\x10a\x10;Wa\x10&a\x106\x91\x84\x90a\x0ENV[a\x101_\x84\x90a\r\xE2V[a\x0ErV[a\x0F1V[\x90a\x10Y\x90\x91\x92_\x93\x84\x93c9\x144\xE3`\xE2\x1B\x85R`\x04\x85\x01a\x0E\x1CV[\x03\x90\xFD[a\x10ea\x06\x12V[Pa\x10p`\x02a\x0E\x0FV[\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x10\x92a\x10\x8Da\x10\x97\x92a\x10sV[a\x02:V[a\x02\x9DV[\x90V[a\x10\xA2a\x06\x12V[Pa\x10\xB2`\x01\x80`\xD0\x1B\x03a\x10~V[\x90V[\x90V[\x90V[\x91a\x11\x13a\x11\ra\x11\x1A\x94\x80a\x10\xE1a\x10\xDBa\x10\xD6_a\x02YV[a\0\xEBV[\x91a\0\xEBV[\x14a\x11KW[\x84a\x11\x02a\x10\xFCa\x10\xF7_a\x02YV[a\0\xEBV[\x91a\0\xEBV[\x14a\x11\x1CW[a\x13CV[\x92a\x13CV[\x90\x91a\x13\x94V[V[a\x11D`\x0B`\x02a\x11>a\x118a\x112\x89a\x12-V[\x93a\x10\xB5V[\x91a\x10\xB8V[\x90a\x12\x80V[PPa\x11\x08V[a\x11s`\x0B`\x01a\x11ma\x11ga\x11a\x89a\x12-V[\x93a\x10\xB5V[\x91a\x10\xB8V[\x90a\x12\x80V[PPa\x10\xE7V[_\x90V[a\x11\x8Aa\x11\x90\x91a\x10sV[\x91a\x10sV[\x01\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x11\xA1WV[a\x0E\x92V[\x90a\x11\xB9\x91a\x11\xB3a\x11zV[Pa\x11~V[\x90V[\x90V[`\xFF\x16\x90V[a\x11\xD9a\x11\xD4a\x11\xDE\x92a\x11\xBCV[a\x02:V[a\x11\xBFV[\x90V[a\x11\xEA\x90a\x11\xC5V[\x90RV[\x91` a\x12\x0F\x92\x94\x93a\x12\x08`@\x82\x01\x96_\x83\x01\x90a\x11\xE1V[\x01\x90a\ntV[V[a\x12%a\x12 a\x12*\x92a\x02\x9DV[a\x02:V[a\x10sV[\x90V[a\x125a\x11zV[P\x80a\x12Oa\x12I`\x01\x80`\xD0\x1B\x03a\x10~V[\x91a\x02\x9DV[\x11a\x12`Wa\x12]\x90a\x12\x11V[\x90V[`\xD0a\x12|_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a\x11\xEEV[\x03\x90\xFD[\x90a\x12\xB6a\x12\xBC\x93\x92a\x12\x91a\x11zV[Pa\x12\x9Aa\x11zV[P\x80\x93a\x12\xAFa\x12\xA8a\x15FV[\x94\x92a\x15\xF3V[\x90\x91a\x1A\xD9V[\x91a\x16hV[\x91\x90\x91\x90V[a\x12\xCEa\x12\xD4\x91a\x10sV[\x91a\x10sV[\x90\x03\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x12\xE6WV[a\x0E\x92V[\x90a\x12\xFE\x91a\x12\xF8a\x11zV[Pa\x12\xC2V[\x90V[\x90a\x13\x0B\x90a\x07\xE7V[_R` R`@_ \x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x13.a\x133\x91a\x0B?V[a\x13\x17V[\x90V[a\x13@\x90Ta\x13\"V[\x90V[a\x13Za\x13_\x91a\x13Ra\x0B\x95V[P`\ta\x13\x01V[a\x136V[\x90V[a\x13va\x13qa\x13{\x92a\x027V[a\x02:V[a\x02\x9DV[\x90V[\x90a\x13\x88\x90a\x07\xE7V[_R` R`@_ \x90V[\x91\x90\x91\x80a\x13\xAAa\x13\xA4\x85a\0\xEBV[\x91a\0\xEBV[\x14\x15\x80a\x15(W[a\x13\xBCW[PPPV[\x80a\x13\xD7a\x13\xD1a\x13\xCC_a\x02YV[a\0\xEBV[\x91a\0\xEBV[\x03a\x14\x98W[P\x81a\x13\xF9a\x13\xF3a\x13\xEE_a\x02YV[a\0\xEBV[\x91a\0\xEBV[\x03a\x14\x05W[\x80a\x13\xB7V[a\x14La\x14?a\x14F\x92a\x14\x1B`\n\x86\x90a\x13~V[\x90a\x149a\x143a\x14-`\x01\x93a\x12-V[\x93a\x10\xB5V[\x91a\x10\xB8V[\x90a\x12\x80V[\x92\x90a\x10~V[\x91a\x10~V[\x91\x90\x91a\x14y\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x07\xE7V[\x92a\x14\x8Ea\x14\x85a\0xV[\x92\x83\x92\x83a\r/V[\x03\x90\xA2_\x80a\x13\xFFV[a\x14\xD7a\x14\xDDa\x14\xD0a\x14\xAD`\n\x85\x90a\x13~V[`\x02a\x14\xCAa\x14\xC4a\x14\xBE\x89a\x12-V[\x93a\x10\xB5V[\x91a\x10\xB8V[\x90a\x12\x80V[\x92\x90a\x10~V[\x91a\x10~V[\x91\x90\x91a\x15\n\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x07\xE7V[\x92a\x15\x1Fa\x15\x16a\0xV[\x92\x83\x92\x83a\r/V[\x03\x90\xA2_a\x13\xDDV[P\x81a\x15<a\x156_a\x13bV[\x91a\x02\x9DV[\x11a\x13\xB2V[_\x90V[a\x15Na\x15BV[Pa\x15Wa\x16\x97V[\x90V[T\x90V[\x90V[a\x15ua\x15pa\x15z\x92a\x15^V[a\x02:V[a\x02\x9DV[\x90V[a\x15\x8Ca\x15\x92\x91\x93\x92\x93a\x02\x9DV[\x92a\x02\x9DV[\x82\x03\x91\x82\x11a\x15\x9DWV[a\x0E\x92V[\x90V[`0\x1C\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x15\xC2a\x15\xC7\x91a\x15\xA5V[a\x15\xABV[\x90V[a\x15\xD4\x90Ta\x15\xB6V[\x90V[a\x15\xEBa\x15\xE6a\x15\xF0\x92a\x027V[a\x02:V[a\x10sV[\x90V[a\x15\xFBa\x11zV[Pa\x16\x07_\x82\x01a\x15ZV[\x80a\x16\x1Aa\x16\x14_a\x13bV[\x91a\x02\x9DV[\x14_\x14a\x160WPPa\x16,_a\x15\xD7V[[\x90V[a\x16]_\x91a\x16Xa\x16R\x84a\x16c\x96\x01\x92a\x16L`\x01a\x15aV[\x90a\x15}V[\x91a\x15\xA2V[a\x16\xACV[\x01a\x15\xCAV[a\x16-V[\x91a\x16\x8C_a\x16\x91\x94a\x16ya\x11zV[Pa\x16\x82a\x11zV[P\x01\x92\x91\x92a\x15\xA2V[a\x18\xB1V[\x91\x90\x91\x90V[a\x16\x9Fa\x15BV[Pa\x16\xA9Ca\x1ArV[\x90V[_R` _ \x01\x90V[T\x90V[a\x16\xC4`@a\0\xC7V[\x90V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x90a\x16\xDC\x90a\x16\xC7V[\x90RV[\x90a\x16\xEA\x90a\x10sV[\x90RV[_R` _ \x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[a\x17\x14\x81a\x16\xB6V[\x82\x10\x15a\x17.Wa\x17&`\x01\x91a\x16\xEEV[\x91\x02\x01\x90_\x90V[a\x16\xF7V[a\x17=\x90Qa\x16\xC7V[\x90V[\x90a\x17Qe\xFF\xFF\xFF\xFF\xFF\xFF\x91a\x02mV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x17oa\x17ja\x17t\x92a\x16\xC7V[a\x02:V[a\x16\xC7V[\x90V[\x90V[\x90a\x17\x8Fa\x17\x8Aa\x17\x96\x92a\x17[V[a\x17wV[\x82Ta\x17@V[\x90UV[a\x17\xA4\x90Qa\x10sV[\x90V[`0\x1B\x90V[\x90a\x17\xBFe\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91a\x17\xA7V[\x91\x81\x19\x16\x91\x16\x17\x90V[a\x17\xDDa\x17\xD8a\x17\xE2\x92a\x10sV[a\x02:V[a\x10sV[\x90V[\x90V[\x90a\x17\xFDa\x17\xF8a\x18\x04\x92a\x17\xC9V[a\x17\xE5V[\x82Ta\x17\xADV[\x90UV[\x90a\x182` _a\x188\x94a\x18*\x82\x82\x01a\x18$\x84\x88\x01a\x173V[\x90a\x17zV[\x01\x92\x01a\x17\x9AV[\x90a\x17\xE8V[V[\x91\x90a\x18KWa\x18I\x91a\x18\x08V[V[a\x05@V[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x18\x80W\x82a\x18x\x91`\x01a\x18~\x95\x01\x81Ua\x17\x0BV[\x90a\x18:V[V[a\0\x8CV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x18\x9Ca\x18\xA1\x91a\x0B?V[a\x18\x85V[\x90V[a\x18\xAE\x90Ta\x18\x90V[\x90V[\x90\x92\x91\x92a\x18\xBDa\x11zV[Pa\x18\xC6a\x11zV[Pa\x18\xD0\x82a\x16\xB6V[\x80a\x18\xE3a\x18\xDD_a\x13bV[\x91a\x02\x9DV[\x11_\x14a\x19\xB3Wa\x19\t\x90a\x19\x03\x84\x91a\x18\xFD`\x01a\x15aV[\x90a\x15}V[\x90a\x16\xACV[\x90a\x19\x15_\x83\x01a\x18\xA4V[\x92a\x19!_\x84\x01a\x15\xCAV[\x93\x80a\x195a\x19/\x85a\x16\xC7V[\x91a\x16\xC7V[\x11a\x19\x97Wa\x19La\x19F\x84a\x16\xC7V[\x91a\x16\xC7V[\x14_\x14a\x19gWPPa\x19b\x90_\x85\x91\x01a\x17\xE8V[[\x91\x90V[a\x19\x92\x92Pa\x19\x8D\x86a\x19\x84a\x19{a\x16\xBAV[\x94_\x86\x01a\x16\xD2V[` \x84\x01a\x16\xE0V[a\x18PV[a\x19cV[_c% `\x1D`\xE0\x1B\x81R\x80a\x19\xAF`\x04\x82\x01a\x02eV[\x03\x90\xFD[Pa\x19\xDE\x91a\x19\xD9\x85a\x19\xD0a\x19\xC7a\x16\xBAV[\x94_\x86\x01a\x16\xD2V[` \x84\x01a\x16\xE0V[a\x18PV[a\x19\xE7_a\x15\xD7V[\x91\x90V[a\x19\xFFa\x19\xFAa\x1A\x04\x92a\x16\xC7V[a\x02:V[a\x02\x9DV[\x90V[\x90V[a\x1A\x1Ea\x1A\x19a\x1A#\x92a\x1A\x07V[a\x02:V[a\x11\xBFV[\x90V[a\x1A/\x90a\x1A\nV[\x90RV[\x91` a\x1AT\x92\x94\x93a\x1AM`@\x82\x01\x96_\x83\x01\x90a\x1A&V[\x01\x90a\ntV[V[a\x1Aja\x1Aea\x1Ao\x92a\x02\x9DV[a\x02:V[a\x16\xC7V[\x90V[a\x1Aza\x15BV[P\x80a\x1A\x94a\x1A\x8Ee\xFF\xFF\xFF\xFF\xFF\xFFa\x19\xEBV[\x91a\x02\x9DV[\x11a\x1A\xA5Wa\x1A\xA2\x90a\x1AVV[\x90V[`0a\x1A\xC1_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a\x1A3V[\x03\x90\xFD[cNH{q`\xE0\x1B_R`Q`\x04R`$_\xFD[\x91\x90\x91\x80`\x01\x14a\x1A\xF8W`\x02\x03a\x1A\xC5Wa\x1A\xF4\x91a\x12\xEBV[\x90[V[Pa\x1B\x02\x91a\x11\xA6V[\x90a\x1A\xF6V\xFE`\x80`@R`\x046\x10\x15a\0\x13W[a\x11\xE6V[a\0\x1D_5a\x02\x8CV[\x80c\x01\xFF\xC9\xA7\x14a\x02\x87W\x80c\x06\xFD\xDE\x03\x14a\x02\x82W\x80c\t^\xA7\xB3\x14a\x02}W\x80c\x18\x16\r\xDD\x14a\x02xW\x80c#\xB8r\xDD\x14a\x02sW\x80c$\x8A\x9C\xA3\x14a\x02nW\x80c//\xF1]\x14a\x02iW\x80c1<\xE5g\x14a\x02dW\x80c6D\xE5\x15\x14a\x02_W\x80c6V\x8A\xBE\x14a\x02ZW\x80c:F\xB1\xA8\x14a\x02UW\x80c@\xC1\x0F\x19\x14a\x02PW\x80cB\x96lh\x14a\x02KW\x80cK\xDD6\xCE\x14a\x02FW\x80cK\xF5\xD7\xE9\x14a\x02AW\x80cX|\xDE\x1E\x14a\x02<W\x80c\\\x19\xA9\\\x14a\x027W\x80co\xCF\xFFE\x14a\x022W\x80cp\xA0\x821\x14a\x02-W\x80c~\xCE\xBE\0\x14a\x02(W\x80c\x84\xB0\x19n\x14a\x02#W\x80c\x8D3C\xD6\x14a\x02\x1EW\x80c\x8ES\x9E\x8C\x14a\x02\x19W\x80c\x90-U\xA5\x14a\x02\x14W\x80c\x91\xD1HT\x14a\x02\x0FW\x80c\x91\xDD\xAD\xF4\x14a\x02\nW\x80c\x95\xD8\x9BA\x14a\x02\x05W\x80c\x9A\xB2N\xB0\x14a\x02\0W\x80c\x9B~\xF6K\x14a\x01\xFBW\x80c\xA2\x17\xFD\xDF\x14a\x01\xF6W\x80c\xA9\x05\x9C\xBB\x14a\x01\xF1W\x80c\xB0\xCA%>\x14a\x01\xECW\x80c\xBBMD6\x14a\x01\xE7W\x80c\xC0*\xE7T\x14a\x01\xE2W\x80c\xC3\xCD\xA5 \x14a\x01\xDDW\x80c\xD5\x05\xAC\xCF\x14a\x01\xD8W\x80c\xD5Gt\x1F\x14a\x01\xD3W\x80c\xDDb\xED>\x14a\x01\xCEWc\xF1\x12~\xD8\x03a\0\x0EWa\x11\xB0V[a\x10\xCCV[a\x10kV[a\x101V[a\x0F\x87V[a\x0E\xCBV[a\x0E\x96V[a\x0E`V[a\x0E*V[a\r\xF5V[a\r\x85V[a\r\x0EV[a\x0C\xD9V[a\x0C\xA4V[a\x0CAV[a\x0C\x0CV[a\x0B\x92V[a\x0B]V[a\n\xF2V[a\t\xABV[a\tvV[a\tAV[a\x08\xE3V[a\x08\xAEV[a\x089V[a\x08\x04V[a\x07\xD1V[a\x07\x7FV[a\x07IV[a\x07\x15V[a\x06\xE0V[a\x06\xABV[a\x06OV[a\x05\xE8V[a\x05LV[a\x04\xDDV[a\x04\x85V[a\x03\xC3V[a\x03\x14V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[c\xFF\xFF\xFF\xFF`\xE0\x1B\x16\x90V[a\x02\xB5\x81a\x02\xA0V[\x03a\x02\xBCWV[_\x80\xFD[\x90P5\x90a\x02\xCD\x82a\x02\xACV[V[\x90` \x82\x82\x03\x12a\x02\xE8Wa\x02\xE5\x91_\x01a\x02\xC0V[\x90V[a\x02\x9CV[\x15\x15\x90V[a\x02\xFB\x90a\x02\xEDV[\x90RV[\x91\x90a\x03\x12\x90_` \x85\x01\x94\x01\x90a\x02\xF2V[V[4a\x03DWa\x03@a\x03/a\x03*6`\x04a\x02\xCFV[a\x12\x83V[a\x037a\x02\x92V[\x91\x82\x91\x82a\x02\xFFV[\x03\x90\xF3[a\x02\x98V[_\x91\x03\x12a\x03SWV[a\x02\x9CV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x03\x99a\x03\xA2` \x93a\x03\xA7\x93a\x03\x90\x81a\x03XV[\x93\x84\x80\x93a\x03\\V[\x95\x86\x91\x01a\x03eV[a\x03pV[\x01\x90V[a\x03\xC0\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03zV[\x90V[4a\x03\xF3Wa\x03\xD36`\x04a\x03IV[a\x03\xEFa\x03\xDEa\x14\x1CV[a\x03\xE6a\x02\x92V[\x91\x82\x91\x82a\x03\xABV[\x03\x90\xF3[a\x02\x98V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\x0C\x90a\x03\xF8V[\x90V[a\x04\x18\x81a\x04\x03V[\x03a\x04\x1FWV[_\x80\xFD[\x90P5\x90a\x040\x82a\x04\x0FV[V[\x90V[a\x04>\x81a\x042V[\x03a\x04EWV[_\x80\xFD[\x90P5\x90a\x04V\x82a\x045V[V[\x91\x90`@\x83\x82\x03\x12a\x04\x80W\x80a\x04ta\x04}\x92_\x86\x01a\x04#V[\x93` \x01a\x04IV[\x90V[a\x02\x9CV[4a\x04\xB6Wa\x04\xB2a\x04\xA1a\x04\x9B6`\x04a\x04XV[\x90a\x142V[a\x04\xA9a\x02\x92V[\x91\x82\x91\x82a\x02\xFFV[\x03\x90\xF3[a\x02\x98V[a\x04\xC4\x90a\x042V[\x90RV[\x91\x90a\x04\xDB\x90_` \x85\x01\x94\x01\x90a\x04\xBBV[V[4a\x05\rWa\x04\xED6`\x04a\x03IV[a\x05\ta\x04\xF8a\x14\x81V[a\x05\0a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[\x90\x91``\x82\x84\x03\x12a\x05GWa\x05Da\x05-\x84_\x85\x01a\x04#V[\x93a\x05;\x81` \x86\x01a\x04#V[\x93`@\x01a\x04IV[\x90V[a\x02\x9CV[4a\x05}Wa\x05ya\x05ha\x05b6`\x04a\x05\x12V[\x91a\x14\x97V[a\x05pa\x02\x92V[\x91\x82\x91\x82a\x02\xFFV[\x03\x90\xF3[a\x02\x98V[\x90V[a\x05\x8E\x81a\x05\x82V[\x03a\x05\x95WV[_\x80\xFD[\x90P5\x90a\x05\xA6\x82a\x05\x85V[V[\x90` \x82\x82\x03\x12a\x05\xC1Wa\x05\xBE\x91_\x01a\x05\x99V[\x90V[a\x02\x9CV[a\x05\xCF\x90a\x05\x82V[\x90RV[\x91\x90a\x05\xE6\x90_` \x85\x01\x94\x01\x90a\x05\xC6V[V[4a\x06\x18Wa\x06\x14a\x06\x03a\x05\xFE6`\x04a\x05\xA8V[a\x15\x10V[a\x06\x0Ba\x02\x92V[\x91\x82\x91\x82a\x05\xD3V[\x03\x90\xF3[a\x02\x98V[\x91\x90`@\x83\x82\x03\x12a\x06EW\x80a\x069a\x06B\x92_\x86\x01a\x05\x99V[\x93` \x01a\x04#V[\x90V[a\x02\x9CV[_\x01\x90V[4a\x06~Wa\x06ha\x06b6`\x04a\x06\x1DV[\x90a\x15\\V[a\x06pa\x02\x92V[\x80a\x06z\x81a\x06JV[\x03\x90\xF3[a\x02\x98V[`\xFF\x16\x90V[a\x06\x92\x90a\x06\x83V[\x90RV[\x91\x90a\x06\xA9\x90_` \x85\x01\x94\x01\x90a\x06\x89V[V[4a\x06\xDBWa\x06\xBB6`\x04a\x03IV[a\x06\xD7a\x06\xC6a\x15\x8BV[a\x06\xCEa\x02\x92V[\x91\x82\x91\x82a\x06\x96V[\x03\x90\xF3[a\x02\x98V[4a\x07\x10Wa\x06\xF06`\x04a\x03IV[a\x07\x0Ca\x06\xFBa\x15\xA1V[a\x07\x03a\x02\x92V[\x91\x82\x91\x82a\x05\xD3V[\x03\x90\xF3[a\x02\x98V[4a\x07DWa\x07.a\x07(6`\x04a\x06\x1DV[\x90a\x15\xB5V[a\x076a\x02\x92V[\x80a\x07@\x81a\x06JV[\x03\x90\xF3[a\x02\x98V[4a\x07zWa\x07va\x07ea\x07_6`\x04a\x04XV[\x90a\x16fV[a\x07ma\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[4a\x07\xAEWa\x07\x98a\x07\x926`\x04a\x04XV[\x90a\x17\xEDV[a\x07\xA0a\x02\x92V[\x80a\x07\xAA\x81a\x06JV[\x03\x90\xF3[a\x02\x98V[\x90` \x82\x82\x03\x12a\x07\xCCWa\x07\xC9\x91_\x01a\x04IV[\x90V[a\x02\x9CV[4a\x07\xFFWa\x07\xE9a\x07\xE46`\x04a\x07\xB3V[a\x17\xF9V[a\x07\xF1a\x02\x92V[\x80a\x07\xFB\x81a\x06JV[\x03\x90\xF3[a\x02\x98V[4a\x084Wa\x08\x146`\x04a\x03IV[a\x080a\x08\x1Fa\x18^V[a\x08'a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[4a\x08iWa\x08I6`\x04a\x03IV[a\x08ea\x08Ta\x19\x1DV[a\x08\\a\x02\x92V[\x91\x82\x91\x82a\x03\xABV[\x03\x90\xF3[a\x02\x98V[\x90` \x82\x82\x03\x12a\x08\x87Wa\x08\x84\x91_\x01a\x04#V[\x90V[a\x02\x9CV[a\x08\x95\x90a\x04\x03V[\x90RV[\x91\x90a\x08\xAC\x90_` \x85\x01\x94\x01\x90a\x08\x8CV[V[4a\x08\xDEWa\x08\xDAa\x08\xC9a\x08\xC46`\x04a\x08nV[a\x19\xB9V[a\x08\xD1a\x02\x92V[\x91\x82\x91\x82a\x08\x99V[\x03\x90\xF3[a\x02\x98V[4a\t\x11Wa\x08\xFBa\x08\xF66`\x04a\x08nV[a\x19\xD8V[a\t\x03a\x02\x92V[\x80a\t\r\x81a\x06JV[\x03\x90\xF3[a\x02\x98V[c\xFF\xFF\xFF\xFF\x16\x90V[a\t(\x90a\t\x16V[\x90RV[\x91\x90a\t?\x90_` \x85\x01\x94\x01\x90a\t\x1FV[V[4a\tqWa\tma\t\\a\tW6`\x04a\x08nV[a\x19\xEFV[a\tda\x02\x92V[\x91\x82\x91\x82a\t,V[\x03\x90\xF3[a\x02\x98V[4a\t\xA6Wa\t\xA2a\t\x91a\t\x8C6`\x04a\x08nV[a\x1A\x1AV[a\t\x99a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[4a\t\xDBWa\t\xD7a\t\xC6a\t\xC16`\x04a\x08nV[a\x1A8V[a\t\xCEa\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[`\xFF`\xF8\x1B\x16\x90V[a\t\xF2\x90a\t\xE0V[\x90RV[Q\x90V[` \x91\x81R\x01\x90V[` \x01\x90V[a\n\x12\x90a\x042V[\x90RV[\x90a\n#\x81` \x93a\n\tV[\x01\x90V[` \x01\x90V[\x90a\nJa\nDa\n=\x84a\t\xF6V[\x80\x93a\t\xFAV[\x92a\n\x03V[\x90_[\x81\x81\x10a\nZWPPP\x90V[\x90\x91\x92a\nsa\nm`\x01\x92\x86Qa\n\x16V[\x94a\n'V[\x91\x01\x91\x90\x91a\nMV[\x93\x95\x91\x94a\n\xCEa\n\xC3a\n\xE2\x95a\n\xB5a\n\xD8\x95a\n\xEF\x9C\x9Aa\n\xA8`\xE0\x8C\x01\x92_\x8D\x01\x90a\t\xE9V[\x8A\x82\x03` \x8C\x01Ra\x03zV[\x90\x88\x82\x03`@\x8A\x01Ra\x03zV[\x97``\x87\x01\x90a\x04\xBBV[`\x80\x85\x01\x90a\x08\x8CV[`\xA0\x83\x01\x90a\x05\xC6V[`\xC0\x81\x84\x03\x91\x01Ra\n-V[\x90V[4a\x0B)Wa\x0B\x026`\x04a\x03IV[a\x0B%a\x0B\ra\x1A\xCAV[\x93a\x0B\x1C\x97\x95\x97\x93\x91\x93a\x02\x92V[\x97\x88\x97\x88a\n}V[\x03\x90\xF3[a\x02\x98V[\x7F\x9B\x12\xE0\xC5p~II\x15\xE5\x8B\x05d\xF1\x8A\xAA\xD9\xB7J\xC6\x9B\xFC\x81Z\x1E\xDA\xDC\x8EK\xD02\xEB\x90V[a\x0BZa\x0B.V[\x90V[4a\x0B\x8DWa\x0Bm6`\x04a\x03IV[a\x0B\x89a\x0Bxa\x0BRV[a\x0B\x80a\x02\x92V[\x91\x82\x91\x82a\x05\xD3V[\x03\x90\xF3[a\x02\x98V[4a\x0B\xC2Wa\x0B\xBEa\x0B\xADa\x0B\xA86`\x04a\x07\xB3V[a\x1BTV[a\x0B\xB5a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[\x90V[\x90V[a\x0B\xE1a\x0B\xDCa\x0B\xE6\x92a\x0B\xC7V[a\x0B\xCAV[a\x042V[\x90V[a\x0B\xFEk\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x0B\xCDV[\x90V[a\x0C\ta\x0B\xE9V[\x90V[4a\x0C<Wa\x0C\x1C6`\x04a\x03IV[a\x0C8a\x0C'a\x0C\x01V[a\x0C/a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[4a\x0CrWa\x0Cna\x0C]a\x0CW6`\x04a\x06\x1DV[\x90a\x1B\xC2V[a\x0Cea\x02\x92V[\x91\x82\x91\x82a\x02\xFFV[\x03\x90\xF3[a\x02\x98V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x0C\x8B\x90a\x0CwV[\x90RV[\x91\x90a\x0C\xA2\x90_` \x85\x01\x94\x01\x90a\x0C\x82V[V[4a\x0C\xD4Wa\x0C\xB46`\x04a\x03IV[a\x0C\xD0a\x0C\xBFa\x1B\xF0V[a\x0C\xC7a\x02\x92V[\x91\x82\x91\x82a\x0C\x8FV[\x03\x90\xF3[a\x02\x98V[4a\r\tWa\x0C\xE96`\x04a\x03IV[a\r\x05a\x0C\xF4a\x1C\x04V[a\x0C\xFCa\x02\x92V[\x91\x82\x91\x82a\x03\xABV[\x03\x90\xF3[a\x02\x98V[4a\r>Wa\r:a\r)a\r$6`\x04a\x08nV[a\x1C\x1AV[a\r1a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[\x90V[a\rZa\rUa\r_\x92a\rCV[a\x0B\xCAV[a\x042V[\x90V[a\rwk\x02\xE8vi\xC3\x08sj\x04\0\0\0a\rFV[\x90V[a\r\x82a\rbV[\x90V[4a\r\xB5Wa\r\x956`\x04a\x03IV[a\r\xB1a\r\xA0a\rzV[a\r\xA8a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[\x90V[_\x1B\x90V[a\r\xD6a\r\xD1a\r\xDB\x92a\r\xBAV[a\r\xBDV[a\x05\x82V[\x90V[a\r\xE7_a\r\xC2V[\x90V[a\r\xF2a\r\xDEV[\x90V[4a\x0E%Wa\x0E\x056`\x04a\x03IV[a\x0E!a\x0E\x10a\r\xEAV[a\x0E\x18a\x02\x92V[\x91\x82\x91\x82a\x05\xD3V[\x03\x90\xF3[a\x02\x98V[4a\x0E[Wa\x0EWa\x0EFa\x0E@6`\x04a\x04XV[\x90a\x1CIV[a\x0ENa\x02\x92V[\x91\x82\x91\x82a\x02\xFFV[\x03\x90\xF3[a\x02\x98V[4a\x0E\x91Wa\x0E\x8Da\x0E|a\x0Ev6`\x04a\x04XV[\x90a\x1CkV[a\x0E\x84a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[4a\x0E\xC6Wa\x0E\xC2a\x0E\xB1a\x0E\xAC6`\x04a\x08nV[a\x1C\x81V[a\x0E\xB9a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[4a\x0E\xFBWa\x0E\xDB6`\x04a\x03IV[a\x0E\xF7a\x0E\xE6a\x1C\x96V[a\x0E\xEEa\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[a\x0F\t\x81a\x06\x83V[\x03a\x0F\x10WV[_\x80\xFD[\x90P5\x90a\x0F!\x82a\x0F\0V[V[\x90\x91`\xC0\x82\x84\x03\x12a\x0F\x82Wa\x0F;\x83_\x84\x01a\x04#V[\x92a\x0FI\x81` \x85\x01a\x04IV[\x92a\x0FW\x82`@\x83\x01a\x04IV[\x92a\x0F\x7Fa\x0Fh\x84``\x85\x01a\x0F\x14V[\x93a\x0Fv\x81`\x80\x86\x01a\x05\x99V[\x93`\xA0\x01a\x05\x99V[\x90V[a\x02\x9CV[4a\x0F\xBCWa\x0F\xA6a\x0F\x9A6`\x04a\x0F#V[\x94\x93\x90\x93\x92\x91\x92a\x1D\x16V[a\x0F\xAEa\x02\x92V[\x80a\x0F\xB8\x81a\x06JV[\x03\x90\xF3[a\x02\x98V[`\xE0\x81\x83\x03\x12a\x10,Wa\x0F\xD7\x82_\x83\x01a\x04#V[\x92a\x0F\xE5\x83` \x84\x01a\x04#V[\x92a\x0F\xF3\x81`@\x85\x01a\x04IV[\x92a\x10\x01\x82``\x83\x01a\x04IV[\x92a\x10)a\x10\x12\x84`\x80\x85\x01a\x0F\x14V[\x93a\x10 \x81`\xA0\x86\x01a\x05\x99V[\x93`\xC0\x01a\x05\x99V[\x90V[a\x02\x9CV[4a\x10fWa\x10Pa\x10D6`\x04a\x0F\xC1V[\x95\x94\x90\x94\x93\x91\x93a\x1EjV[a\x10Xa\x02\x92V[\x80a\x10b\x81a\x06JV[\x03\x90\xF3[a\x02\x98V[4a\x10\x9AWa\x10\x84a\x10~6`\x04a\x06\x1DV[\x90a\x1F\x88V[a\x10\x8Ca\x02\x92V[\x80a\x10\x96\x81a\x06JV[\x03\x90\xF3[a\x02\x98V[\x91\x90`@\x83\x82\x03\x12a\x10\xC7W\x80a\x10\xBBa\x10\xC4\x92_\x86\x01a\x04#V[\x93` \x01a\x04#V[\x90V[a\x02\x9CV[4a\x10\xFDWa\x10\xF9a\x10\xE8a\x10\xE26`\x04a\x10\x9FV[\x90a\x1F\xAAV[a\x10\xF0a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[a\x11\x0B\x81a\t\x16V[\x03a\x11\x12WV[_\x80\xFD[\x90P5\x90a\x11#\x82a\x11\x02V[V[\x91\x90`@\x83\x82\x03\x12a\x11MW\x80a\x11Aa\x11J\x92_\x86\x01a\x04#V[\x93` \x01a\x11\x16V[\x90V[a\x02\x9CV[a\x11[\x90a\x0CwV[\x90RV[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x11s\x90a\x11_V[\x90RV[\x90` \x80a\x11\x99\x93a\x11\x8F_\x82\x01Q_\x86\x01\x90a\x11RV[\x01Q\x91\x01\x90a\x11jV[V[\x91\x90a\x11\xAE\x90_`@\x85\x01\x94\x01\x90a\x11wV[V[4a\x11\xE1Wa\x11\xDDa\x11\xCCa\x11\xC66`\x04a\x11%V[\x90a \x18V[a\x11\xD4a\x02\x92V[\x91\x82\x91\x82a\x11\x9BV[\x03\x90\xF3[a\x02\x98V[_\x80\xFD[_\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x12\x0Ea\x12\x14\x91a\x11_V[\x91a\x11_V[\x01\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x12%WV[a\x11\xEEV[\x90a\x12=\x91a\x127a\x11\xEAV[Pa\x12\x02V[\x90V[a\x12La\x12R\x91a\x11_V[\x91a\x11_V[\x90\x03\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x12dWV[a\x11\xEEV[\x90a\x12|\x91a\x12va\x11\xEAV[Pa\x12@V[\x90V[_\x90V[a\x12\x8Ba\x12\x7FV[P\x80a\x12\xA6a\x12\xA0cye\xDB\x0B`\xE0\x1Ba\x02\xA0V[\x91a\x02\xA0V[\x14\x90\x81\x15a\x12\xB3W[P\x90V[a\x12\xBD\x91Pa .V[_a\x12\xAFV[``\x90V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x01`\x02\x83\x04\x92\x16\x80\x15a\x12\xFCW[` \x83\x10\x14a\x12\xF7WV[a\x12\xC8V[\x91`\x7F\x16\x91a\x12\xECV[` \x91\x81R\x01\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a\x132a\x13+\x83a\x12\xDCV[\x80\x94a\x13\x06V[\x91`\x01\x81\x16\x90\x81_\x14a\x13\x89WP`\x01\x14a\x13MW[PPPV[a\x13Z\x91\x92\x93\x94Pa\x13\x0FV[\x91_\x92[\x81\x84\x10a\x13qWPP\x01\x90_\x80\x80a\x13HV[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a\x13^V[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a\x13HV[\x90a\x13\xAE\x91a\x13\x18V[\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x13\xCF\x90a\x03pV[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x13\xE9W`@RV[a\x13\xB1V[\x90a\x14\x0Ea\x14\x07\x92a\x13\xFEa\x02\x92V[\x93\x84\x80\x92a\x13\xA4V[\x03\x83a\x13\xC5V[V[a\x14\x19\x90a\x13\xEEV[\x90V[a\x14$a\x12\xC3V[Pa\x14/`\x03a\x14\x10V[\x90V[a\x14O\x91a\x14>a\x12\x7FV[Pa\x14Ga TV[\x91\x90\x91a aV[`\x01\x90V[_\x90V[_\x1C\x90V[\x90V[a\x14la\x14q\x91a\x14XV[a\x14]V[\x90V[a\x14~\x90Ta\x14`V[\x90V[a\x14\x89a\x14TV[Pa\x14\x94`\x02a\x14tV[\x90V[\x91a\x14\xC1\x92a\x14\xA4a\x12\x7FV[Pa\x14\xB9a\x14\xB0a TV[\x82\x90\x84\x91a \xB1V[\x91\x90\x91a!=V[`\x01\x90V[_\x90V[a\x14\xD3\x90a\x05\x82V[\x90V[\x90a\x14\xE0\x90a\x14\xCAV[_R` R`@_ \x90V[\x90V[a\x14\xFBa\x15\0\x91a\x14XV[a\x14\xECV[\x90V[a\x15\r\x90Ta\x14\xEFV[\x90V[`\x01a\x15)a\x15/\x92a\x15!a\x14\xC6V[P`\x05a\x14\xD6V[\x01a\x15\x03V[\x90V[\x90a\x15M\x91a\x15Ha\x15C\x82a\x15\x10V[a!\xDAV[a\x15OV[V[\x90a\x15Y\x91a\"3V[PV[\x90a\x15f\x91a\x152V[V[_\x90V[\x90V[a\x15\x83a\x15~a\x15\x88\x92a\x15lV[a\x0B\xCAV[a\x06\x83V[\x90V[a\x15\x93a\x15hV[Pa\x15\x9E`\x12a\x15oV[\x90V[a\x15\xA9a\x14\xC6V[Pa\x15\xB2a\"\xDFV[\x90V[\x90\x80a\x15\xD0a\x15\xCAa\x15\xC5a TV[a\x04\x03V[\x91a\x04\x03V[\x03a\x15\xE1Wa\x15\xDE\x91a#\x99V[PV[_c3K\xD9\x19`\xE1\x1B\x81R\x80a\x15\xF9`\x04\x82\x01a\x06JV[\x03\x90\xFD[a\x16\x11a\x16\x0Ca\x16\x16\x92a\x03\xF8V[a\x0B\xCAV[a\x03\xF8V[\x90V[a\x16\"\x90a\x15\xFDV[\x90V[a\x16.\x90a\x16\x19V[\x90V[\x90a\x16;\x90a\x16%V[_R` R`@_ \x90V[\x90V[a\x16^a\x16Ya\x16c\x92a\x11_V[a\x0B\xCAV[a\x042V[\x90V[a\x16\x9D\x91a\x16\x92a\x16\x8Ca\x16\x87a\x16\x98\x94a\x16\x7Fa\x14TV[P`\na\x161V[a\x16GV[\x91a$zV[\x90a%\x8FV[a\x16JV[\x90V[\x90a\x16\xBA\x91a\x16\xB5a\x16\xB0a\x0B.V[a!\xDAV[a\x17%V[V[a\x16\xD0a\x16\xCBa\x16\xD5\x92a\r\xBAV[a\x0B\xCAV[a\x03\xF8V[\x90V[a\x16\xE1\x90a\x16\xBCV[\x90V[a\x16\xF8a\x16\xF3a\x16\xFD\x92a\r\xBAV[a\x0B\xCAV[a\x042V[\x90V[a\x17\x0Fa\x17\x15\x91\x93\x92\x93a\x042V[\x92a\x042V[\x82\x01\x80\x92\x11a\x17 WV[a\x11\xEEV[\x90\x81a\x17Aa\x17;a\x176_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a\x17\xD1W\x80a\x17Ya\x17S_a\x16\xE4V[\x91a\x042V[\x14a\x17\xB5Wa\x17pa\x17ia\x14\x81V[\x82\x90a\x17\0V[a\x17\x89a\x17\x83a\x17~a\x0B\xE9V[a\x042V[\x91a\x042V[\x11a\x17\x99Wa\x17\x97\x91a&\xB6V[V[_c\x17~?\xC3`\xE0\x1B\x81R\x80a\x17\xB1`\x04\x82\x01a\x06JV[\x03\x90\xFD[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x17\xCD`\x04\x82\x01a\x06JV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x17\xE9`\x04\x82\x01a\x06JV[\x03\x90\xFD[\x90a\x17\xF7\x91a\x16\xA0V[V[\x80a\x18\x0Ca\x18\x06_a\x16\xE4V[\x91a\x042V[\x14a\x18\x1DWa\x18\x1B\x903a'\x14V[V[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x185`\x04\x82\x01a\x06JV[\x03\x90\xFD[a\x18Ha\x18N\x91\x93\x92\x93a\x042V[\x92a\x042V[\x82\x03\x91\x82\x11a\x18YWV[a\x11\xEEV[a\x18fa\x14TV[Pa\x18\x80a\x18ra\x0B\xE9V[a\x18za\x14\x81V[\x90a\x189V[\x90V[\x90a\x18\x96a\x18\x8Fa\x02\x92V[\x92\x83a\x13\xC5V[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x18\xB6Wa\x18\xB2` \x91a\x03pV[\x01\x90V[a\x13\xB1V[\x90a\x18\xCDa\x18\xC8\x83a\x18\x98V[a\x18\x83V[\x91\x82RV[_\x7Fmode=blocknumber&from=default\0\0\0\x91\x01RV[a\x19\x03`\x1Da\x18\xBBV[\x90a\x19\x10` \x83\x01a\x18\xD2V[V[a\x19\x1Aa\x18\xF9V[\x90V[a\x19%a\x12\xC3V[Pa\x19.a\x1B\xF0V[a\x19Ga\x19Aa\x19<a'sV[a\x0CwV[\x91a\x0CwV[\x03a\x19WWa\x19Ta\x19\x12V[\x90V[_c\x01\xBF\xC1\xC5`\xE6\x1B\x81R\x80a\x19o`\x04\x82\x01a\x06JV[\x03\x90\xFD[_\x90V[\x90a\x19\x81\x90a\x16%V[_R` R`@_ \x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x19\xA4a\x19\xA9\x91a\x14XV[a\x19\x8DV[\x90V[a\x19\xB6\x90Ta\x19\x98V[\x90V[a\x19\xD0a\x19\xD5\x91a\x19\xC8a\x19sV[P`\ta\x19wV[a\x19\xACV[\x90V[a\x19\xE9\x90a\x19\xE4a TV[a'\xC6V[V[_\x90V[a\x1A\x01\x90a\x19\xFBa\x19\xEBV[Pa(QV[\x90V[\x90a\x1A\x0E\x90a\x16%V[_R` R`@_ \x90V[a\x1A0a\x1A5\x91a\x1A)a\x14TV[P_a\x1A\x04V[a\x14tV[\x90V[a\x1AJ\x90a\x1ADa\x14TV[Pa(\x80V[\x90V[_\x90V[``\x90V[a\x1A_\x90a\x16\x19V[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1AzW` \x80\x91\x02\x01\x90V[a\x13\xB1V[\x90a\x1A\x91a\x1A\x8C\x83a\x1AbV[a\x18\x83V[\x91\x82RV[6\x907V[\x90a\x1A\xC0a\x1A\xA8\x83a\x1A\x7FV[\x92` \x80a\x1A\xB6\x86\x93a\x1AbV[\x92\x01\x91\x03\x90a\x1A\x96V[V[`\x0F`\xF8\x1B\x90V[a\x1A\xD2a\x1AMV[Pa\x1A\xDBa\x12\xC3V[Pa\x1A\xE4a\x12\xC3V[Pa\x1A\xEDa\x14TV[Pa\x1A\xF6a\x19sV[Pa\x1A\xFFa\x14\xC6V[Pa\x1B\x08a\x1AQV[Pa\x1B\x11a(\x98V[\x90a\x1B\x1Aa(\xD8V[\x90F\x90a\x1B&0a\x1AVV[\x90a\x1B0_a\r\xC2V[\x90a\x1BBa\x1B=_a\x16\xE4V[a\x1A\x9BV[\x90a\x1BKa\x1A\xC2V[\x96\x95\x94\x93\x92\x91\x90V[a\x1B}a\x1B\x82\x91a\x1Bca\x14TV[Pa\x1Bwa\x1Bq`\x0Ba\x16GV[\x91a$zV[\x90a%\x8FV[a\x16JV[\x90V[\x90a\x1B\x8F\x90a\x16%V[_R` R`@_ \x90V[`\xFF\x16\x90V[a\x1B\xADa\x1B\xB2\x91a\x14XV[a\x1B\x9BV[\x90V[a\x1B\xBF\x90Ta\x1B\xA1V[\x90V[a\x1B\xE9\x91_a\x1B\xDEa\x1B\xE4\x93a\x1B\xD6a\x12\x7FV[P`\x05a\x14\xD6V[\x01a\x1B\x85V[a\x1B\xB5V[\x90V[_\x90V[a\x1B\xF8a\x1B\xECV[Pa\x1C\x01a'sV[\x90V[a\x1C\x0Ca\x12\xC3V[Pa\x1C\x17`\x04a\x14\x10V[\x90V[a\x1CAa\x1C<a\x1C7a\x1CF\x93a\x1C/a\x14TV[P`\na\x161V[a\x16GV[a)\x18V[a\x16JV[\x90V[a\x1Cf\x91a\x1CUa\x12\x7FV[Pa\x1C^a TV[\x91\x90\x91a!=V[`\x01\x90V[\x90a\x1C~\x91a\x1Cxa\x14TV[Pa\x16fV[\x90V[a\x1C\x93\x90a\x1C\x8Da\x14TV[Pa\x1C\x1AV[\x90V[a\x1C\x9Ea\x14TV[Pa\x1C\xA7a\x14\x81V[\x90V[\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF\x90V[a\x1D\x03a\x1D\n\x94a\x1C\xF9``\x94\x98\x97\x95a\x1C\xEF`\x80\x86\x01\x9A_\x87\x01\x90a\x05\xC6V[` \x85\x01\x90a\x08\x8CV[`@\x83\x01\x90a\x04\xBBV[\x01\x90a\x04\xBBV[V[` \x01\x90V[Q\x90V[\x93\x95\x94\x90\x92\x91\x95Ba\x1D0a\x1D*\x89a\x042V[\x91a\x042V[\x11a\x1D\xA9W\x91a\x1D\x9B\x91a\x1D\xA2\x93a\x1D\x92a\x1D\xA7\x98\x99a\x1Dza\x1DQa\x1C\xAAV[a\x1Dk\x8B\x93\x8Ba\x1D_a\x02\x92V[\x95\x86\x94` \x86\x01a\x1C\xCEV[` \x82\x01\x81\x03\x82R\x03\x82a\x13\xC5V[a\x1D\x8Ca\x1D\x86\x82a\x1D\x12V[\x91a\x1D\x0CV[ a)\x8DV[\x92\x90\x91\x92a)\xAAV[\x91\x82a)\xF4V[a'\xC6V[V[a\x1D\xC4\x87_\x91\x82\x91c#A\xD7\x87`\xE1\x1B\x83R`\x04\x83\x01a\x04\xC8V[\x03\x90\xFD[\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x90V[\x91\x94a\x1E4a\x1E>\x92\x98\x97\x95a\x1E*`\xA0\x96a\x1E a\x1EE\x9Aa\x1E\x16`\xC0\x8A\x01\x9E_\x8B\x01\x90a\x05\xC6V[` \x89\x01\x90a\x08\x8CV[`@\x87\x01\x90a\x08\x8CV[``\x85\x01\x90a\x04\xBBV[`\x80\x83\x01\x90a\x04\xBBV[\x01\x90a\x04\xBBV[V[\x91` a\x1Eh\x92\x94\x93a\x1Ea`@\x82\x01\x96_\x83\x01\x90a\x08\x8CV[\x01\x90a\x08\x8CV[V[\x96\x95\x91\x93\x92\x94\x90\x94Ba\x1E\x85a\x1E\x7F\x83a\x042V[\x91a\x042V[\x11a\x1F?W\x90a\x1E\xEEa\x1E\xF7\x94\x93\x92a\x1E\xD6a\x1E\x9Fa\x1D\xC8V[a\x1E\xC7\x8C\x80\x94\x8C\x91a\x1E\xB1\x8D\x91a*\x9BV[\x91\x92a\x1E\xBBa\x02\x92V[\x97\x88\x96` \x88\x01a\x1D\xECV[` \x82\x01\x81\x03\x82R\x03\x82a\x13\xC5V[a\x1E\xE8a\x1E\xE2\x82a\x1D\x12V[\x91a\x1D\x0CV[ a)\x8DV[\x92\x90\x91\x92a)\xAAV[\x80a\x1F\na\x1F\x04\x87a\x04\x03V[\x91a\x04\x03V[\x03a\x1F\x1FWPa\x1F\x1D\x92\x93\x91\x90\x91a aV[V[\x84\x90a\x1F;_\x92\x83\x92c%\xC0\x07#`\xE1\x1B\x84R`\x04\x84\x01a\x1EGV[\x03\x90\xFD[a\x1FZ\x90_\x91\x82\x91c1<\x89\x81`\xE1\x1B\x83R`\x04\x83\x01a\x04\xC8V[\x03\x90\xFD[\x90a\x1Fy\x91a\x1Fta\x1Fo\x82a\x15\x10V[a!\xDAV[a\x1F{V[V[\x90a\x1F\x85\x91a#\x99V[PV[\x90a\x1F\x92\x91a\x1F^V[V[\x90a\x1F\x9E\x90a\x16%V[_R` R`@_ \x90V[a\x1F\xCF\x91a\x1F\xC5a\x1F\xCA\x92a\x1F\xBDa\x14TV[P`\x01a\x1F\x94V[a\x1A\x04V[a\x14tV[\x90V[a\x1F\xDC`@a\x18\x83V[\x90V[_\x90V[_\x90V[a\x1F\xEFa\x1F\xD2V[\x90` \x80\x83a\x1F\xFCa\x1F\xDFV[\x81R\x01a \x07a\x1F\xE3V[\x81RPPV[a \x15a\x1F\xE7V[\x90V[\x90a +\x91a %a \rV[Pa*\xCEV[\x90V[a 6a\x12\x7FV[Pa Pa Jc\x01\xFF\xC9\xA7`\xE0\x1Ba\x02\xA0V[\x91a\x02\xA0V[\x14\x90V[a \\a\x19sV[P3\x90V[\x91a o\x92\x91`\x01\x92a*\xF6V[V[`@\x90a \x9Aa \xA1\x94\x96\x95\x93\x96a \x90``\x84\x01\x98_\x85\x01\x90a\x08\x8CV[` \x83\x01\x90a\x04\xBBV[\x01\x90a\x04\xBBV[V[\x90a \xAE\x91\x03a\x042V[\x90V[\x92\x91\x92a \xBF\x81\x83\x90a\x1F\xAAV[\x90\x81a \xD4a \xCE_\x19a\x042V[\x91a\x042V[\x10a \xE1W[PPP\x90PV[\x81a \xF4a \xEE\x87a\x042V[\x91a\x042V[\x10a!\x1AWa!\x11\x93\x94a!\t\x91\x93\x92a \xA3V[\x90_\x92a*\xF6V[\x80_\x80\x80a \xDAV[Pa!9\x84\x92\x91\x92_\x93\x84\x93c}\xC7\xA0\xD9`\xE1\x1B\x85R`\x04\x85\x01a qV[\x03\x90\xFD[\x91\x82a!Ya!Sa!N_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a!\xB3W\x81a!ya!sa!n_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a!\x8CWa!\x8A\x92\x91\x90\x91a,\x05V[V[a!\xAFa!\x98_a\x16\xD8V[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\x08\x99V[\x03\x90\xFD[a!\xD6a!\xBF_a\x16\xD8V[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\x08\x99V[\x03\x90\xFD[a!\xEC\x90a!\xE6a TV[\x90a,7V[V[\x90a!\xFA`\xFF\x91a\r\xBDV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\"\r\x90a\x02\xEDV[\x90V[\x90V[\x90a\"(a\"#a\"/\x92a\"\x04V[a\"\x10V[\x82Ta!\xEEV[\x90UV[a\";a\x12\x7FV[Pa\"Pa\"J\x82\x84\x90a\x1B\xC2V[\x15a\x02\xEDV[_\x14a\"\xD9Wa\"x`\x01a\"s_a\"k`\x05\x86\x90a\x14\xD6V[\x01\x85\x90a\x1B\x85V[a\"\x13V[\x90a\"\x81a TV[\x90a\"\xBEa\"\xB8a\"\xB2\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x95a\x14\xCAV[\x92a\x16%V[\x92a\x16%V[\x92a\"\xC7a\x02\x92V[\x80a\"\xD1\x81a\x06JV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a\"\xE7a\x14\xC6V[Pa\"\xF10a\x1AVV[a##a#\x1D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\x03V[\x91a\x04\x03V[\x14\x80a#_W[_\x14a#TW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a#\\a,\xE3V[\x90V[PFa#\x93a#\x8D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x042V[\x91a\x042V[\x14a#*V[a#\xA1a\x12\x7FV[Pa#\xAD\x81\x83\x90a\x1B\xC2V[_\x14a$5Wa#\xD4_a#\xCF_a#\xC7`\x05\x86\x90a\x14\xD6V[\x01\x85\x90a\x1B\x85V[a\"\x13V[\x90a#\xDDa TV[\x90a$\x1Aa$\x14a$\x0E\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x95a\x14\xCAV[\x92a\x16%V[\x92a\x16%V[\x92a$#a\x02\x92V[\x80a$-\x81a\x06JV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a$Oa$Ja$T\x92a\x0CwV[a\x0B\xCAV[a\x042V[\x90V[\x91` a$x\x92\x94\x93a$q`@\x82\x01\x96_\x83\x01\x90a\x04\xBBV[\x01\x90a\x0C\x82V[V[a$\x82a\x1B\xECV[Pa$\x8Ba\x1B\xF0V[\x81a$\x9Ea$\x98\x83a$;V[\x91a\x042V[\x10\x15a$\xB1WPa$\xAE\x90a-\xECV[\x90V[\x90a$\xCC_\x92\x83\x92cvi\xFC\x0F`\xE1\x1B\x84R`\x04\x84\x01a$WV[\x03\x90\xFD[T\x90V[\x90V[a$\xEBa$\xE6a$\xF0\x92a$\xD4V[a\x0B\xCAV[a\x042V[\x90V[\x90V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a%\ra%\x12\x91a\x14XV[a$\xF6V[\x90V[a%\x1F\x90Ta%\x01V[\x90V[\x90V[a%9a%4a%>\x92a%\"V[a\x0B\xCAV[a\x042V[\x90V[`0\x1C\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a%^a%c\x91a%AV[a%GV[\x90V[a%p\x90Ta%RV[\x90V[a%\x87a%\x82a%\x8C\x92a\r\xBAV[a\x0B\xCAV[a\x11_V[\x90V[\x90a%\xE3\x90a%\x9Ca\x11\xEAV[Pa%\xA8_\x84\x01a$\xD0V[a%\xB1_a\x16\xE4V[\x90\x80\x80a%\xC7a%\xC1`\x05a$\xD7V[\x91a\x042V[\x11a&DW[P\x90a%\xDE_\x86\x01\x93\x91\x92\x93a$\xF3V[a4CV[\x80a%\xF6a%\xF0_a\x16\xE4V[\x91a\x042V[\x14_\x14a&\x0CWPPa&\x08_a%sV[[\x90V[a&9_\x91a&4a&.\x84a&?\x96\x01\x92a&(`\x01a%%V[\x90a\x189V[\x91a$\xF3V[a49V[\x01a%fV[a&\tV[\x80a&Ra&X\x92\x91a0\xCEV[\x90a\x189V[\x90\x83a&\x8Aa&\x84a&\x7F_a&y\x81\x8C\x01a&t\x89\x91a$\xF3V[a49V[\x01a%\x15V[a\x0CwV[\x91a\x0CwV[\x10_\x14a&\x9BWP\x90[\x90_a%\xCDV[\x91Pa&\xB1\x90a&\xAB`\x01a%%V[\x90a\x17\0V[a&\x94V[\x80a&\xD1a&\xCBa&\xC6_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a&\xEDWa&\xEB\x91a&\xE3_a\x16\xD8V[\x91\x90\x91a,\x05V[V[a'\x10a&\xF9_a\x16\xD8V[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\x08\x99V[\x03\x90\xFD[\x90\x81a'0a'*a'%_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a'LWa'J\x91\x90a'C_a\x16\xD8V[\x90\x91a,\x05V[V[a'oa'X_a\x16\xD8V[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\x08\x99V[\x03\x90\xFD[a'{a\x1B\xECV[Pa'\x85Ca-\xECV[\x90V[\x90a'\x99`\x01\x80`\xA0\x1B\x03\x91a\r\xBDV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a'\xBBa'\xB6a'\xC2\x92a\x16%V[a'\xA3V[\x82Ta'\x88V[\x90UV[\x90a(O\x91a(Ia'\xD7\x82a\x19\xB9V[a'\xEC\x84a'\xE7`\t\x86\x90a\x19wV[a'\xA6V[\x82\x81\x85\x90a(,a(&a( \x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x95a\x16%V[\x92a\x16%V[\x92a\x16%V[\x92a(5a\x02\x92V[\x80a(?\x81a\x06JV[\x03\x90\xA4\x92\x91a4\xD2V[\x91a5\rV[V[a(xa(sa(na(}\x93a(fa\x19\xEBV[P`\na\x161V[a\x16GV[a6\xBBV[a7:V[\x90V[a(\x92\x90a(\x8Ca\x14TV[Pa7\x8BV[\x90V[\x90V[a(\xA0a\x12\xC3V[Pa(\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a(\xCF`\x06a(\x95V[\x90a8\xA6V[\x90V[a(\xE0a\x12\xC3V[Pa)\x15\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a)\x0F`\x07a(\x95V[\x90a8\xA6V[\x90V[a) a\x11\xEAV[Pa),_\x82\x01a$\xD0V[\x80a)?a)9_a\x16\xE4V[\x91a\x042V[\x14_\x14a)UWPPa)Q_a%sV[[\x90V[a)\x82_\x91a)}a)w\x84a)\x88\x96\x01\x92a)q`\x01a%%V[\x90a\x189V[\x91a$\xF3V[a49V[\x01a%fV[a)RV[a)\xA7\x90a)\x99a\x14\xC6V[Pa)\xA2a\"\xDFV[a8\xF4V[\x90V[\x92a)\xC5\x92a)\xCE\x94a)\xBBa\x19sV[P\x92\x90\x91\x92a9\xBAV[\x90\x92\x91\x92a:\xE5V[\x90V[\x91` a)\xF2\x92\x94\x93a)\xEB`@\x82\x01\x96_\x83\x01\x90a\x08\x8CV[\x01\x90a\x04\xBBV[V[a)\xFD\x81a*\x9BV[\x91a*\x10a*\n\x84a\x042V[\x91a\x042V[\x03a*\x19WPPV[a*3_\x92\x83\x92c\x01\xD4\xB6#`\xE6\x1B\x84R`\x04\x84\x01a)\xD1V[\x03\x90\xFD[`\x01a*C\x91\x01a\x042V[\x90V[\x90a*R_\x19\x91a\r\xBDV[\x91\x81\x19\x16\x91\x16\x17\x90V[a*pa*ka*u\x92a\x042V[a\x0B\xCAV[a\x042V[\x90V[\x90V[\x90a*\x90a*\x8Ba*\x97\x92a*\\V[a*xV[\x82Ta*FV[\x90UV[a*\xAF\x90a*\xA7a\x14TV[P`\x08a\x1A\x04V[a*\xCBa*\xBB\x82a\x14tV[\x91a*\xC5\x83a*7V[\x90a*{V[\x90V[\x90a*\xEEa*\xE9a*\xF3\x93a*\xE1a \rV[P`\na\x161V[a\x16GV[a<[V[\x90V[\x90\x92\x81a+\x13a+\ra+\x08_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a+\xDEW\x83a+3a+-a+(_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a+\xB7Wa+W\x83a+Ra+K`\x01\x86\x90a\x1F\x94V[\x87\x90a\x1A\x04V[a*{V[a+aW[PPPV[\x91\x90\x91a+\xACa+\x9Aa+\x94\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x93a\x16%V[\x93a\x16%V[\x93a+\xA3a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xA3_\x80\x80a+\\V[a+\xDAa+\xC3_a\x16\xD8V[_\x91\x82\x91cJ\x14\x06\xB1`\xE1\x1B\x83R`\x04\x83\x01a\x08\x99V[\x03\x90\xFD[a,\x01a+\xEA_a\x16\xD8V[_\x91\x82\x91c\xE6\x02\xDF\x05`\xE0\x1B\x83R`\x04\x83\x01a\x08\x99V[\x03\x90\xFD[\x91a,\x12\x92\x91\x90\x91a<|V[V[\x91` a,5\x92\x94\x93a,.`@\x82\x01\x96_\x83\x01\x90a\x08\x8CV[\x01\x90a\x05\xC6V[V[\x90a,La,F\x83\x83\x90a\x1B\xC2V[\x15a\x02\xEDV[a,TWPPV[a,n_\x92\x83\x92c\xE2Q}?`\xE0\x1B\x84R`\x04\x84\x01a,\x14V[\x03\x90\xFD[\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x90V[\x90\x95\x94\x92a,\xE1\x94a,\xD0a,\xDA\x92a,\xC6`\x80\x96a,\xBC`\xA0\x88\x01\x9C_\x89\x01\x90a\x05\xC6V[` \x87\x01\x90a\x05\xC6V[`@\x85\x01\x90a\x05\xC6V[``\x83\x01\x90a\x04\xBBV[\x01\x90a\x08\x8CV[V[a,\xEBa\x14\xC6V[Pa,\xF4a,rV[a-k\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a-\\\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Fa-G0a\x1AVV[\x91a-Pa\x02\x92V[\x96\x87\x95` \x87\x01a,\x96V[` \x82\x01\x81\x03\x82R\x03\x82a\x13\xC5V[a-}a-w\x82a\x1D\x12V[\x91a\x1D\x0CV[ \x90V[\x90V[a-\x98a-\x93a-\x9D\x92a-\x81V[a\x0B\xCAV[a\x06\x83V[\x90V[a-\xA9\x90a-\x84V[\x90RV[\x91` a-\xCE\x92\x94\x93a-\xC7`@\x82\x01\x96_\x83\x01\x90a-\xA0V[\x01\x90a\x04\xBBV[V[a-\xE4a-\xDFa-\xE9\x92a\x042V[a\x0B\xCAV[a\x0CwV[\x90V[a-\xF4a\x1B\xECV[P\x80a.\x0Ea.\x08e\xFF\xFF\xFF\xFF\xFF\xFFa$;V[\x91a\x042V[\x11a.\x1FWa.\x1C\x90a-\xD0V[\x90V[`0a.;_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a-\xADV[\x03\x90\xFD[\x90V[a.Va.Qa.[\x92a.?V[a\x0B\xCAV[a\x042V[\x90V[\x90V[a.ua.pa.z\x92a.^V[a\x0B\xCAV[a\x06\x83V[\x90V[\x1C\x90V[a.\xA0\x90a.\x9Aa.\x94a.\xA5\x94a\x06\x83V[\x91a\x042V[\x90a.}V[a\x042V[\x90V[\x90V[a.\xBFa.\xBAa.\xC4\x92a.\xA8V[a\x0B\xCAV[a\x06\x83V[\x90V[\x1B\x90V[a.\xEA\x90a.\xE4a.\xDEa.\xEF\x94a\x06\x83V[\x91a\x042V[\x90a.\xC7V[a\x042V[\x90V[\x90V[a/\ta/\x04a/\x0E\x92a.\xF2V[a\x0B\xCAV[a\x042V[\x90V[\x90V[a/(a/#a/-\x92a/\x11V[a\x0B\xCAV[a\x06\x83V[\x90V[\x90V[a/Ga/Ba/L\x92a/0V[a\x0B\xCAV[a\x042V[\x90V[\x90V[a/fa/aa/k\x92a/OV[a\x0B\xCAV[a\x06\x83V[\x90V[\x90V[a/\x85a/\x80a/\x8A\x92a/nV[a\x0B\xCAV[a\x042V[\x90V[\x90V[a/\xA4a/\x9Fa/\xA9\x92a/\x8DV[a\x0B\xCAV[a\x06\x83V[\x90V[\x90V[a/\xC3a/\xBEa/\xC8\x92a/\xACV[a\x0B\xCAV[a\x042V[\x90V[\x90V[a/\xE2a/\xDDa/\xE7\x92a/\xCBV[a\x0B\xCAV[a\x06\x83V[\x90V[a/\xFEa/\xF9a0\x03\x92a/OV[a\x0B\xCAV[a\x042V[\x90V[\x90V[a0\x1Da0\x18a0\"\x92a0\x06V[a\x0B\xCAV[a\x06\x83V[\x90V[a09a04a0>\x92a/\xCBV[a\x0B\xCAV[a\x042V[\x90V[a0Ua0Pa0Z\x92a%\"V[a\x0B\xCAV[a\x06\x83V[\x90V[\x90V[a0ta0oa0y\x92a0]V[a\x0B\xCAV[a\x042V[\x90V[\x90a0\x87\x91\x02a\x042V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a0\xAAa0\xB0\x91a\x042V[\x91a\x042V[\x90\x81\x15a0\xBBW\x04\x90V[a0\x8AV[\x90a0\xCB\x91\x01a\x042V[\x90V[a0\xD6a\x14TV[P\x80a0\xEBa0\xE5`\x01a%%V[\x91a\x042V[\x11\x15a46W\x80a3\0a2\xDDa2\xCDa2\xBDa2\xADa2\x9Da2\x8Da2}a2ma2]a2M\x8Ba2Ga2@a3\x06\x9Fa2 a2\x10a20\x92a12`\x01a%%V[\x90\x80a1Ja1D`\x01`\x80\x1Ba.BV[\x91a\x042V[\x10\x15a4\x08W[\x80a1ma1gh\x01\0\0\0\0\0\0\0\0a.\xF5V[\x91a\x042V[\x10\x15a3\xDAW[\x80a1\x8Ca1\x86d\x01\0\0\0\0a/3V[\x91a\x042V[\x10\x15a3\xACW[\x80a1\xA9a1\xA3b\x01\0\0a/qV[\x91a\x042V[\x10\x15a3~W[\x80a1\xC5a1\xBFa\x01\0a/\xAFV[\x91a\x042V[\x10\x15a3PW[\x80a1\xE0a1\xDA`\x10a/\xEAV[\x91a\x042V[\x10\x15a3\"W[a1\xFAa1\xF4`\x04a0%V[\x91a\x042V[\x10\x15a3\tW[a2\x0B`\x03a0`V[a0|V[a2\x1A`\x01a0AV[\x90a.\x81V[a2*\x81\x86a0\x9EV[\x90a0\xC0V[a2:`\x01a0AV[\x90a.\x81V[\x80\x92a0\x9EV[\x90a0\xC0V[a2W`\x01a0AV[\x90a.\x81V[a2g\x81\x8Ca0\x9EV[\x90a0\xC0V[a2w`\x01a0AV[\x90a.\x81V[a2\x87\x81\x8Aa0\x9EV[\x90a0\xC0V[a2\x97`\x01a0AV[\x90a.\x81V[a2\xA7\x81\x88a0\x9EV[\x90a0\xC0V[a2\xB7`\x01a0AV[\x90a.\x81V[a2\xC7\x81\x86a0\x9EV[\x90a0\xC0V[a2\xD7`\x01a0AV[\x90a.\x81V[\x91a2\xFAa2\xF4a2\xEF\x85\x80\x94a0\x9EV[a\x042V[\x91a\x042V[\x11a=\x0CV[\x90a \xA3V[\x90V[a3\x1D\x90a3\x17`\x01a0AV[\x90a.\xCBV[a2\x01V[a39a3J\x91a33`\x04a/\xCEV[\x90a.\x81V[\x91a3D`\x02a0\tV[\x90a.\xCBV[\x90a1\xE7V[a3ga3x\x91a3a`\x08a/\x90V[\x90a.\x81V[\x91a3r`\x04a/\xCEV[\x90a.\xCBV[\x90a1\xCCV[a3\x95a3\xA6\x91a3\x8F`\x10a/RV[\x90a.\x81V[\x91a3\xA0`\x08a/\x90V[\x90a.\xCBV[\x90a1\xB0V[a3\xC3a3\xD4\x91a3\xBD` a/\x14V[\x90a.\x81V[\x91a3\xCE`\x10a/RV[\x90a.\xCBV[\x90a1\x93V[a3\xF1a4\x02\x91a3\xEB`@a.\xABV[\x90a.\x81V[\x91a3\xFC` a/\x14V[\x90a.\xCBV[\x90a1tV[a4\x1Fa40\x91a4\x19`\x80a.aV[\x90a.\x81V[\x91a4*`@a.\xABV[\x90a.\xCBV[\x90a1QV[\x90V[_R` _ \x01\x90V[\x93\x91\x90\x92a4Oa\x14TV[P[\x81a4da4^\x83a\x042V[\x91a\x042V[\x10\x15a4\xCAWa4u\x82\x82\x90a=XV[\x90a4\x8B_a4\x85\x88\x85\x90a49V[\x01a%\x15V[a4\x9Da4\x97\x87a\x0CwV[\x91a\x0CwV[\x11_\x14a4\xADWP\x91[\x91a4QV[\x92\x91Pa4\xC4\x90a4\xBE`\x01a%%V[\x90a\x17\0V[\x90a4\xA7V[\x92PP\x91P\x90V[a4\xE4\x90a4\xDEa\x14TV[Pa\x1A\x1AV[\x90V[\x90V[\x91` a5\x0B\x92\x94\x93a5\x04`@\x82\x01\x96_\x83\x01\x90a\x04\xBBV[\x01\x90a\x04\xBBV[V[\x91\x90\x91\x80a5#a5\x1D\x85a\x04\x03V[\x91a\x04\x03V[\x14\x15\x80a6\xA1W[a55W[PPPV[\x80a5Pa5Ja5E_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x03a6\x11W[P\x81a5ra5la5g_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x03a5~W[\x80a50V[a5\xC5a5\xB8a5\xBF\x92a5\x94`\n\x86\x90a\x161V[\x90a5\xB2a5\xACa5\xA6`\x01\x93a=\xF1V[\x93a\x16GV[\x91a4\xE7V[\x90a>DV[\x92\x90a\x16JV[\x91a\x16JV[\x91\x90\x91a5\xF2\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x16%V[\x92a6\x07a5\xFEa\x02\x92V[\x92\x83\x92\x83a4\xEAV[\x03\x90\xA2_\x80a5xV[a6Pa6Va6Ia6&`\n\x85\x90a\x161V[`\x02a6Ca6=a67\x89a=\xF1V[\x93a\x16GV[\x91a4\xE7V[\x90a>DV[\x92\x90a\x16JV[\x91a\x16JV[\x91\x90\x91a6\x83\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x16%V[\x92a6\x98a6\x8Fa\x02\x92V[\x92\x83\x92\x83a4\xEAV[\x03\x90\xA2_a5VV[P\x81a6\xB5a6\xAF_a\x16\xE4V[\x91a\x042V[\x11a5+V[_a6\xCF\x91a6\xC8a\x14TV[P\x01a$\xD0V[\x90V[a6\xE6a6\xE1a6\xEB\x92a\t\x16V[a\x0B\xCAV[a\x042V[\x90V[a6\xF7\x90a/\x14V[\x90RV[\x91` a7\x1C\x92\x94\x93a7\x15`@\x82\x01\x96_\x83\x01\x90a6\xEEV[\x01\x90a\x04\xBBV[V[a72a7-a77\x92a\x042V[a\x0B\xCAV[a\t\x16V[\x90V[a7Ba\x19\xEBV[P\x80a7Za7Tc\xFF\xFF\xFF\xFFa6\xD2V[\x91a\x042V[\x11a7kWa7h\x90a7\x1EV[\x90V[` a7\x87_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a6\xFBV[\x03\x90\xFD[a7\xA2a7\xA7\x91a7\x9Aa\x14TV[P`\x08a\x1A\x04V[a\x14tV[\x90V[\x90V[a7\xC1a7\xBCa7\xC6\x92a7\xAAV[a\r\xBDV[a\x05\x82V[\x90V[a7\xD3`\xFFa7\xADV[\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a7\xF9a7\xF2\x83a\x12\xDCV[\x80\x94a\x13\x06V[\x91`\x01\x81\x16\x90\x81_\x14a8PWP`\x01\x14a8\x14W[PPPV[a8!\x91\x92\x93\x94Pa7\xD6V[\x91_\x92[\x81\x84\x10a88WPP\x01\x90_\x80\x80a8\x0FV[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a8%V[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a8\x0FV[\x90a8u\x91a7\xDFV[\x90V[\x90a8\x98a8\x91\x92a8\x88a\x02\x92V[\x93\x84\x80\x92a8kV[\x03\x83a\x13\xC5V[V[a8\xA3\x90a8xV[\x90V[\x90a8\xAFa\x12\xC3V[Pa8\xB9\x82a\x14\xCAV[a8\xD2a8\xCCa8\xC7a7\xC9V[a\x05\x82V[\x91a\x05\x82V[\x14\x15_\x14a8\xE7WPa8\xE4\x90a>\xCEV[\x90V[a8\xF1\x91Pa8\x9AV[\x90V[`B\x91a8\xFFa\x14\xC6V[P`@Q\x91\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[_\x90V[a9Ea9J\x91a\x14XV[a*\\V[\x90V[\x90V[a9da9_a9i\x92a9MV[a\x0B\xCAV[a\x042V[\x90V[a9\xA1a9\xA8\x94a9\x97``\x94\x98\x97\x95a9\x8D`\x80\x86\x01\x9A_\x87\x01\x90a\x05\xC6V[` \x85\x01\x90a\x06\x89V[`@\x83\x01\x90a\x05\xC6V[\x01\x90a\x05\xC6V[V[a9\xB2a\x02\x92V[=_\x82>=\x90\xFD[\x93\x92\x93a9\xC5a\x19sV[Pa9\xCEa95V[Pa9\xD7a\x14\xC6V[Pa9\xE1\x85a99V[a:\x13a:\r\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0a9PV[\x91a\x042V[\x11a:\xA0W\x90a:6` \x94\x95_\x94\x93\x92\x93a:-a\x02\x92V[\x94\x85\x94\x85a9lV[\x83\x80R\x03\x90`\x01Z\xFA\x15a:\x9BWa:N_Qa\r\xBDV[\x80a:ia:ca:^_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a:\x7FW_\x91a:y_a\r\xC2V[\x91\x92\x91\x90V[Pa:\x89_a\x16\xD8V[`\x01\x91a:\x95_a\r\xC2V[\x91\x92\x91\x90V[a9\xAAV[PPPa:\xAC_a\x16\xD8V[\x90`\x03\x92\x91\x92\x91\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x04\x11\x15a:\xD4WV[a:\xB6V[\x90a:\xE3\x82a:\xCAV[V[\x80a:\xF8a:\xF2_a:\xD9V[\x91a:\xD9V[\x14_\x14a;\x03WPPV[\x80a;\x17a;\x11`\x01a:\xD9V[\x91a:\xD9V[\x14_\x14a;:W_c\xF6E\xEE\xDF`\xE0\x1B\x81R\x80a;6`\x04\x82\x01a\x06JV[\x03\x90\xFD[\x80a;Na;H`\x02a:\xD9V[\x91a:\xD9V[\x14_\x14a;|Wa;xa;a\x83a99V[_\x91\x82\x91c\xFC\xE6\x98\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x04\xC8V[\x03\x90\xFD[a;\x8Fa;\x89`\x03a:\xD9V[\x91a:\xD9V[\x14a;\x97WPV[a;\xB2\x90_\x91\x82\x91c5\xE2\xF3\x83`\xE2\x1B\x83R`\x04\x83\x01a\x05\xD3V[\x03\x90\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_R` _ \x90V[a;\xDC\x81a$\xD0V[\x82\x10\x15a;\xF6Wa;\xEE`\x01\x91a;\xCAV[\x91\x02\x01\x90_\x90V[a;\xB6V[\x90a<\x05\x90a\x0CwV[\x90RV[\x90a<\x13\x90a\x11_V[\x90RV[\x90a<Ma<D_a<'a\x1F\xD2V[\x94a<>a<6\x83\x83\x01a%\x15V[\x83\x88\x01a;\xFBV[\x01a%fV[` \x84\x01a<\tV[V[a<X\x90a<\x17V[\x90V[a<y\x91_a<s\x92a<la \rV[P\x01a;\xD3V[Pa<OV[\x90V[\x92\x91a<\x8A\x84\x83\x83\x91a>\xFEV[\x83a<\xA5a<\x9Fa<\x9A_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a<\xBAW[a<\xB8\x92\x93\x91\x90\x91a@\x88V[V[a<\xC2a\x14\x81V[\x93a<\xCBa@mV[\x94\x80a<\xDFa<\xD9\x88a\x042V[\x91a\x042V[\x11a<\xECWP\x93Pa<\xABV[\x85\x90a=\x08_\x92\x83\x92c\x0EX\xAE\x93`\xE1\x1B\x84R`\x04\x84\x01a4\xEAV[\x03\x90\xFD[a=\x14a\x14TV[P\x15\x15\x90V[a=.a=)a=3\x92a0\x06V[a\x0B\xCAV[a\x042V[\x90V[a=Ba=H\x91a\x042V[\x91a\x042V[\x90\x81\x15a=SW\x04\x90V[a0\x8AV[a=}a=\x83\x92a=ga\x14TV[P\x82\x81\x16\x92\x18a=w`\x02a=\x1AV[\x90a=6V[\x90a\x17\0V[\x90V[\x90V[a=\x9Da=\x98a=\xA2\x92a=\x86V[a\x0B\xCAV[a\x06\x83V[\x90V[a=\xAE\x90a=\x89V[\x90RV[\x91` a=\xD3\x92\x94\x93a=\xCC`@\x82\x01\x96_\x83\x01\x90a=\xA5V[\x01\x90a\x04\xBBV[V[a=\xE9a=\xE4a=\xEE\x92a\x042V[a\x0B\xCAV[a\x11_V[\x90V[a=\xF9a\x11\xEAV[P\x80a>\x13a>\r`\x01\x80`\xD0\x1B\x03a\x16JV[\x91a\x042V[\x11a>$Wa>!\x90a=\xD5V[\x90V[`\xD0a>@_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a=\xB2V[\x03\x90\xFD[\x90a>za>\x80\x93\x92a>Ua\x11\xEAV[Pa>^a\x11\xEAV[P\x80\x93a>sa>la\x1B\xF0V[\x94\x92a)\x18V[\x90\x91aE\x03V[\x91aAGV[\x91\x90\x91\x90V[a>\x9Aa>\x95a>\x9F\x92a/\x11V[a\x0B\xCAV[a\x042V[\x90V[6\x907V[\x90a>\xCCa>\xB4\x83a\x18\xBBV[\x92` \x80a>\xC2\x86\x93a\x18\x98V[\x92\x01\x91\x03\x90a>\xA2V[V[a>\xD6a\x12\xC3V[Pa>\xE0\x81aA\xB1V[\x90a>\xF3a>\xEE` a>\x86V[a>\xA7V[\x91\x82R` \x82\x01R\x90V[\x91\x90\x91\x80a?\x1Ca?\x16a?\x11_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14_\x14a?\xFDWa?@a?9\x83a?4`\x02a\x14tV[a\x17\0V[`\x02a*{V[[\x82a?\\a?Va?Q_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14_\x14a?\xD1Wa?\x80a?y\x83a?t`\x02a\x14tV[a \xA3V[`\x02a*{V[[\x91\x90\x91a?\xCCa?\xBAa?\xB4\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93a\x16%V[\x93a\x16%V[\x93a?\xC3a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xA3V[a?\xF8\x82a?\xF2a?\xE3_\x87\x90a\x1A\x04V[\x91a?\xED\x83a\x14tV[a0\xC0V[\x90a*{V[a?\x81V[a@\x10a@\x0B_\x83\x90a\x1A\x04V[a\x14tV[\x80a@#a@\x1D\x85a\x042V[\x91a\x042V[\x10a@KWa@6a@F\x91\x84\x90a \xA3V[a@A_\x84\x90a\x1A\x04V[a*{V[a?AV[\x90a@i\x90\x91\x92_\x93\x84\x93c9\x144\xE3`\xE2\x1B\x85R`\x04\x85\x01a qV[\x03\x90\xFD[a@ua\x14TV[Pa@\x85`\x01\x80`\xD0\x1B\x03a\x16JV[\x90V[\x91a@\xE0a@\xDAa@\xE7\x94\x80a@\xAEa@\xA8a@\xA3_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14aA\x18W[\x84a@\xCFa@\xC9a@\xC4_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a@\xE9W[a\x19\xB9V[\x92a\x19\xB9V[\x90\x91a5\rV[V[aA\x11`\x0B`\x02aA\x0BaA\x05a@\xFF\x89a=\xF1V[\x93a\x16GV[\x91a4\xE7V[\x90a>DV[PPa@\xD5V[aA@`\x0B`\x01aA:aA4aA.\x89a=\xF1V[\x93a\x16GV[\x91a4\xE7V[\x90a>DV[PPa@\xB4V[\x91aAk_aAp\x94aAXa\x11\xEAV[PaAaa\x11\xEAV[P\x01\x92\x91\x92a$\xF3V[aC\xB5V[\x91\x90\x91\x90V[aA\x8AaA\x85aA\x8F\x92a7\xAAV[a\x0B\xCAV[a\x042V[\x90V[\x90V[aA\xA9aA\xA4aA\xAE\x92aA\x92V[a\x0B\xCAV[a\x042V[\x90V[aA\xC6aA\xCB\x91aA\xC0a\x14TV[Pa\x14\xCAV[a99V[aA\xD5`\xFFaAvV[\x16\x80aA\xEAaA\xE4`\x1FaA\x95V[\x91a\x042V[\x11aA\xF2W\x90V[_c,\xD4J\xC3`\xE2\x1B\x81R\x80aB\n`\x04\x82\x01a\x06JV[\x03\x90\xFD[T\x90V[aB\x1C`@a\x18\x83V[\x90V[_R` _ \x90V[aB1\x81aB\x0EV[\x82\x10\x15aBKWaBC`\x01\x91aB\x1FV[\x91\x02\x01\x90_\x90V[a;\xB6V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[aBm\x90Qa\x0CwV[\x90V[\x90aB\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x91a\r\xBDV[\x91\x81\x19\x16\x91\x16\x17\x90V[aB\x9FaB\x9AaB\xA4\x92a\x0CwV[a\x0B\xCAV[a\x0CwV[\x90V[\x90V[\x90aB\xBFaB\xBAaB\xC6\x92aB\x8BV[aB\xA7V[\x82TaBpV[\x90UV[aB\xD4\x90Qa\x11_V[\x90V[`0\x1B\x90V[\x90aB\xEFe\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91aB\xD7V[\x91\x81\x19\x16\x91\x16\x17\x90V[aC\raC\x08aC\x12\x92a\x11_V[a\x0B\xCAV[a\x11_V[\x90V[\x90V[\x90aC-aC(aC4\x92aB\xF9V[aC\x15V[\x82TaB\xDDV[\x90UV[\x90aCb` _aCh\x94aCZ\x82\x82\x01aCT\x84\x88\x01aBcV[\x90aB\xAAV[\x01\x92\x01aB\xCAV[\x90aC\x18V[V[\x91\x90aC{WaCy\x91aC8V[V[aBPV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15aC\xB0W\x82aC\xA8\x91`\x01aC\xAE\x95\x01\x81UaB(V[\x90aCjV[V[a\x13\xB1V[\x90\x92\x91\x92aC\xC1a\x11\xEAV[PaC\xCAa\x11\xEAV[PaC\xD4\x82aB\x0EV[\x80aC\xE7aC\xE1_a\x16\xE4V[\x91a\x042V[\x11_\x14aD\xB7WaD\r\x90aD\x07\x84\x91aD\x01`\x01a%%V[\x90a\x189V[\x90a49V[\x90aD\x19_\x83\x01a%\x15V[\x92aD%_\x84\x01a%fV[\x93\x80aD9aD3\x85a\x0CwV[\x91a\x0CwV[\x11aD\x9BWaDPaDJ\x84a\x0CwV[\x91a\x0CwV[\x14_\x14aDkWPPaDf\x90_\x85\x91\x01aC\x18V[[\x91\x90V[aD\x96\x92PaD\x91\x86aD\x88aD\x7FaB\x12V[\x94_\x86\x01a;\xFBV[` \x84\x01a<\tV[aC\x80V[aDgV[_c% `\x1D`\xE0\x1B\x81R\x80aD\xB3`\x04\x82\x01a\x06JV[\x03\x90\xFD[PaD\xE2\x91aD\xDD\x85aD\xD4aD\xCBaB\x12V[\x94_\x86\x01a;\xFBV[` \x84\x01a<\tV[aC\x80V[aD\xEB_a%sV[\x91\x90V[cNH{q`\xE0\x1B_R`Q`\x04R`$_\xFD[\x91\x90\x91\x80`\x01\x14aE\"W`\x02\x03aD\xEFWaE\x1E\x91a\x12iV[\x90[V[PaE,\x91a\x12*V[\x90aE V",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610013575b6111e6565b61001d5f3561028c565b806301ffc9a71461028757806306fdde0314610282578063095ea7b31461027d57806318160ddd1461027857806323b872dd14610273578063248a9ca31461026e5780632f2ff15d14610269578063313ce567146102645780633644e5151461025f57806336568abe1461025a5780633a46b1a81461025557806340c10f191461025057806342966c681461024b5780634bdd36ce146102465780634bf5d7e914610241578063587cde1e1461023c5780635c19a95c146102375780636fcfff451461023257806370a082311461022d5780637ecebe001461022857806384b0196e146102235780638d3343d61461021e5780638e539e8c14610219578063902d55a51461021457806391d148541461020f57806391ddadf41461020a57806395d89b41146102055780639ab24eb0146102005780639b7ef64b146101fb578063a217fddf146101f6578063a9059cbb146101f1578063b0ca253e146101ec578063bb4d4436146101e7578063c02ae754146101e2578063c3cda520146101dd578063d505accf146101d8578063d547741f146101d3578063dd62ed3e146101ce5763f1127ed80361000e576111b0565b6110cc565b61106b565b611031565b610f87565b610ecb565b610e96565b610e60565b610e2a565b610df5565b610d85565b610d0e565b610cd9565b610ca4565b610c41565b610c0c565b610b92565b610b5d565b610af2565b6109ab565b610976565b610941565b6108e3565b6108ae565b610839565b610804565b6107d1565b61077f565b610749565b610715565b6106e0565b6106ab565b61064f565b6105e8565b61054c565b6104dd565b610485565b6103c3565b610314565b60e01c90565b60405190565b5f80fd5b5f80fd5b63ffffffff60e01b1690565b6102b5816102a0565b036102bc57565b5f80fd5b905035906102cd826102ac565b565b906020828203126102e8576102e5915f016102c0565b90565b61029c565b151590565b6102fb906102ed565b9052565b9190610312905f602085019401906102f2565b565b346103445761034061032f61032a3660046102cf565b611283565b610337610292565b918291826102ff565b0390f35b610298565b5f91031261035357565b61029c565b5190565b60209181520190565b90825f9392825e0152565b601f801991011690565b6103996103a26020936103a79361039081610358565b9384809361035c565b95869101610365565b610370565b0190565b6103c09160208201915f81840391015261037a565b90565b346103f3576103d3366004610349565b6103ef6103de61141c565b6103e6610292565b918291826103ab565b0390f35b610298565b60018060a01b031690565b61040c906103f8565b90565b61041881610403565b0361041f57565b5f80fd5b905035906104308261040f565b565b90565b61043e81610432565b0361044557565b5f80fd5b9050359061045682610435565b565b9190604083820312610480578061047461047d925f8601610423565b93602001610449565b90565b61029c565b346104b6576104b26104a161049b366004610458565b90611432565b6104a9610292565b918291826102ff565b0390f35b610298565b6104c490610432565b9052565b91906104db905f602085019401906104bb565b565b3461050d576104ed366004610349565b6105096104f8611481565b610500610292565b918291826104c8565b0390f35b610298565b90916060828403126105475761054461052d845f8501610423565b9361053b8160208601610423565b93604001610449565b90565b61029c565b3461057d57610579610568610562366004610512565b91611497565b610570610292565b918291826102ff565b0390f35b610298565b90565b61058e81610582565b0361059557565b5f80fd5b905035906105a682610585565b565b906020828203126105c1576105be915f01610599565b90565b61029c565b6105cf90610582565b9052565b91906105e6905f602085019401906105c6565b565b34610618576106146106036105fe3660046105a8565b611510565b61060b610292565b918291826105d3565b0390f35b610298565b91906040838203126106455780610639610642925f8601610599565b93602001610423565b90565b61029c565b5f0190565b3461067e5761066861066236600461061d565b9061155c565b610670610292565b8061067a8161064a565b0390f35b610298565b60ff1690565b61069290610683565b9052565b91906106a9905f60208501940190610689565b565b346106db576106bb366004610349565b6106d76106c661158b565b6106ce610292565b91829182610696565b0390f35b610298565b34610710576106f0366004610349565b61070c6106fb6115a1565b610703610292565b918291826105d3565b0390f35b610298565b346107445761072e61072836600461061d565b906115b5565b610736610292565b806107408161064a565b0390f35b610298565b3461077a5761077661076561075f366004610458565b90611666565b61076d610292565b918291826104c8565b0390f35b610298565b346107ae57610798610792366004610458565b906117ed565b6107a0610292565b806107aa8161064a565b0390f35b610298565b906020828203126107cc576107c9915f01610449565b90565b61029c565b346107ff576107e96107e43660046107b3565b6117f9565b6107f1610292565b806107fb8161064a565b0390f35b610298565b3461083457610814366004610349565b61083061081f61185e565b610827610292565b918291826104c8565b0390f35b610298565b3461086957610849366004610349565b61086561085461191d565b61085c610292565b918291826103ab565b0390f35b610298565b9060208282031261088757610884915f01610423565b90565b61029c565b61089590610403565b9052565b91906108ac905f6020850194019061088c565b565b346108de576108da6108c96108c436600461086e565b6119b9565b6108d1610292565b91829182610899565b0390f35b610298565b34610911576108fb6108f636600461086e565b6119d8565b610903610292565b8061090d8161064a565b0390f35b610298565b63ffffffff1690565b61092890610916565b9052565b919061093f905f6020850194019061091f565b565b346109715761096d61095c61095736600461086e565b6119ef565b610964610292565b9182918261092c565b0390f35b610298565b346109a6576109a261099161098c36600461086e565b611a1a565b610999610292565b918291826104c8565b0390f35b610298565b346109db576109d76109c66109c136600461086e565b611a38565b6109ce610292565b918291826104c8565b0390f35b610298565b60ff60f81b1690565b6109f2906109e0565b9052565b5190565b60209181520190565b60200190565b610a1290610432565b9052565b90610a2381602093610a09565b0190565b60200190565b90610a4a610a44610a3d846109f6565b80936109fa565b92610a03565b905f5b818110610a5a5750505090565b909192610a73610a6d6001928651610a16565b94610a27565b9101919091610a4d565b93959194610ace610ac3610ae295610ab5610ad895610aef9c9a610aa860e08c01925f8d01906109e9565b8a820360208c015261037a565b9088820360408a015261037a565b9760608701906104bb565b608085019061088c565b60a08301906105c6565b60c0818403910152610a2d565b90565b34610b2957610b02366004610349565b610b25610b0d611aca565b93610b1c979597939193610292565b97889788610a7d565b0390f35b610298565b7f9b12e0c5707e494915e58b0564f18aaad9b74ac69bfc815a1edadc8e4bd032eb90565b610b5a610b2e565b90565b34610b8d57610b6d366004610349565b610b89610b78610b52565b610b80610292565b918291826105d3565b0390f35b610298565b34610bc257610bbe610bad610ba83660046107b3565b611b54565b610bb5610292565b918291826104c8565b0390f35b610298565b90565b90565b610be1610bdc610be692610bc7565b610bca565b610432565b90565b610bfe6b033b2e3c9fd0803ce8000000610bcd565b90565b610c09610be9565b90565b34610c3c57610c1c366004610349565b610c38610c27610c01565b610c2f610292565b918291826104c8565b0390f35b610298565b34610c7257610c6e610c5d610c5736600461061d565b90611bc2565b610c65610292565b918291826102ff565b0390f35b610298565b65ffffffffffff1690565b610c8b90610c77565b9052565b9190610ca2905f60208501940190610c82565b565b34610cd457610cb4366004610349565b610cd0610cbf611bf0565b610cc7610292565b91829182610c8f565b0390f35b610298565b34610d0957610ce9366004610349565b610d05610cf4611c04565b610cfc610292565b918291826103ab565b0390f35b610298565b34610d3e57610d3a610d29610d2436600461086e565b611c1a565b610d31610292565b918291826104c8565b0390f35b610298565b90565b610d5a610d55610d5f92610d43565b610bca565b610432565b90565b610d776b02e87669c308736a04000000610d46565b90565b610d82610d62565b90565b34610db557610d95366004610349565b610db1610da0610d7a565b610da8610292565b918291826104c8565b0390f35b610298565b90565b5f1b90565b610dd6610dd1610ddb92610dba565b610dbd565b610582565b90565b610de75f610dc2565b90565b610df2610dde565b90565b34610e2557610e05366004610349565b610e21610e10610dea565b610e18610292565b918291826105d3565b0390f35b610298565b34610e5b57610e57610e46610e40366004610458565b90611c49565b610e4e610292565b918291826102ff565b0390f35b610298565b34610e9157610e8d610e7c610e76366004610458565b90611c6b565b610e84610292565b918291826104c8565b0390f35b610298565b34610ec657610ec2610eb1610eac36600461086e565b611c81565b610eb9610292565b918291826104c8565b0390f35b610298565b34610efb57610edb366004610349565b610ef7610ee6611c96565b610eee610292565b918291826104c8565b0390f35b610298565b610f0981610683565b03610f1057565b5f80fd5b90503590610f2182610f00565b565b909160c082840312610f8257610f3b835f8401610423565b92610f498160208501610449565b92610f578260408301610449565b92610f7f610f688460608501610f14565b93610f768160808601610599565b9360a001610599565b90565b61029c565b34610fbc57610fa6610f9a366004610f23565b94939093929192611d16565b610fae610292565b80610fb88161064a565b0390f35b610298565b60e08183031261102c57610fd7825f8301610423565b92610fe58360208401610423565b92610ff38160408501610449565b926110018260608301610449565b926110296110128460808501610f14565b936110208160a08601610599565b9360c001610599565b90565b61029c565b3461106657611050611044366004610fc1565b95949094939193611e6a565b611058610292565b806110628161064a565b0390f35b610298565b3461109a5761108461107e36600461061d565b90611f88565b61108c610292565b806110968161064a565b0390f35b610298565b91906040838203126110c757806110bb6110c4925f8601610423565b93602001610423565b90565b61029c565b346110fd576110f96110e86110e236600461109f565b90611faa565b6110f0610292565b918291826104c8565b0390f35b610298565b61110b81610916565b0361111257565b5f80fd5b9050359061112382611102565b565b919060408382031261114d578061114161114a925f8601610423565b93602001611116565b90565b61029c565b61115b90610c77565b9052565b60018060d01b031690565b6111739061115f565b9052565b906020806111999361118f5f8201515f860190611152565b015191019061116a565b565b91906111ae905f60408501940190611177565b565b346111e1576111dd6111cc6111c6366004611125565b90612018565b6111d4610292565b9182918261119b565b0390f35b610298565b5f80fd5b5f90565b634e487b7160e01b5f52601160045260245ffd5b61120e6112149161115f565b9161115f565b019060018060d01b03821161122557565b6111ee565b9061123d916112376111ea565b50611202565b90565b61124c6112529161115f565b9161115f565b90039060018060d01b03821161126457565b6111ee565b9061127c916112766111ea565b50611240565b90565b5f90565b61128b61127f565b50806112a66112a0637965db0b60e01b6102a0565b916102a0565b149081156112b3575b5090565b6112bd915061202e565b5f6112af565b606090565b634e487b7160e01b5f52602260045260245ffd5b90600160028304921680156112fc575b60208310146112f757565b6112c8565b91607f16916112ec565b60209181520190565b5f5260205f2090565b905f929180549061133261132b836112dc565b8094611306565b916001811690815f14611389575060011461134d575b505050565b61135a919293945061130f565b915f925b81841061137157505001905f8080611348565b6001816020929593955484860152019101929061135e565b92949550505060ff19168252151560200201905f8080611348565b906113ae91611318565b90565b634e487b7160e01b5f52604160045260245ffd5b906113cf90610370565b810190811067ffffffffffffffff8211176113e957604052565b6113b1565b9061140e611407926113fe610292565b938480926113a4565b03836113c5565b565b611419906113ee565b90565b6114246112c3565b5061142f6003611410565b90565b61144f9161143e61127f565b50611447612054565b919091612061565b600190565b5f90565b5f1c90565b90565b61146c61147191611458565b61145d565b90565b61147e9054611460565b90565b611489611454565b506114946002611474565b90565b916114c1926114a461127f565b506114b96114b0612054565b829084916120b1565b91909161213d565b600190565b5f90565b6114d390610582565b90565b906114e0906114ca565b5f5260205260405f2090565b90565b6114fb61150091611458565b6114ec565b90565b61150d90546114ef565b90565b600161152961152f926115216114c6565b5060056114d6565b01611503565b90565b9061154d9161154861154382611510565b6121da565b61154f565b565b9061155991612233565b50565b9061156691611532565b565b5f90565b90565b61158361157e6115889261156c565b610bca565b610683565b90565b611593611568565b5061159e601261156f565b90565b6115a96114c6565b506115b26122df565b90565b90806115d06115ca6115c5612054565b610403565b91610403565b036115e1576115de91612399565b50565b5f63334bd91960e11b8152806115f96004820161064a565b0390fd5b61161161160c611616926103f8565b610bca565b6103f8565b90565b611622906115fd565b90565b61162e90611619565b90565b9061163b90611625565b5f5260205260405f2090565b90565b61165e6116596116639261115f565b610bca565b610432565b90565b61169d9161169261168c6116876116989461167f611454565b50600a611631565b611647565b9161247a565b9061258f565b61164a565b90565b906116ba916116b56116b0610b2e565b6121da565b611725565b565b6116d06116cb6116d592610dba565b610bca565b6103f8565b90565b6116e1906116bc565b90565b6116f86116f36116fd92610dba565b610bca565b610432565b90565b61170f61171591939293610432565b92610432565b820180921161172057565b6111ee565b908161174161173b6117365f6116d8565b610403565b91610403565b146117d157806117596117535f6116e4565b91610432565b146117b557611770611769611481565b8290611700565b61178961178361177e610be9565b610432565b91610432565b1161179957611797916126b6565b565b5f63177e3fc360e01b8152806117b16004820161064a565b0390fd5b5f631f2a200560e01b8152806117cd6004820161064a565b0390fd5b5f63d92e233d60e01b8152806117e96004820161064a565b0390fd5b906117f7916116a0565b565b8061180c6118065f6116e4565b91610432565b1461181d5761181b9033612714565b565b5f631f2a200560e01b8152806118356004820161064a565b0390fd5b61184861184e91939293610432565b92610432565b820391821161185957565b6111ee565b611866611454565b50611880611872610be9565b61187a611481565b90611839565b90565b9061189661188f610292565b92836113c5565b565b67ffffffffffffffff81116118b6576118b2602091610370565b0190565b6113b1565b906118cd6118c883611898565b611883565b918252565b5f7f6d6f64653d626c6f636b6e756d6265722666726f6d3d64656661756c74000000910152565b611903601d6118bb565b90611910602083016118d2565b565b61191a6118f9565b90565b6119256112c3565b5061192e611bf0565b61194761194161193c612773565b610c77565b91610c77565b0361195757611954611912565b90565b5f6301bfc1c560e61b81528061196f6004820161064a565b0390fd5b5f90565b9061198190611625565b5f5260205260405f2090565b60018060a01b031690565b6119a46119a991611458565b61198d565b90565b6119b69054611998565b90565b6119d06119d5916119c8611973565b506009611977565b6119ac565b90565b6119e9906119e4612054565b6127c6565b565b5f90565b611a01906119fb6119eb565b50612851565b90565b90611a0e90611625565b5f5260205260405f2090565b611a30611a3591611a29611454565b505f611a04565b611474565b90565b611a4a90611a44611454565b50612880565b90565b5f90565b606090565b611a5f90611619565b90565b67ffffffffffffffff8111611a7a5760208091020190565b6113b1565b90611a91611a8c83611a62565b611883565b918252565b369037565b90611ac0611aa883611a7f565b92602080611ab68693611a62565b9201910390611a96565b565b600f60f81b90565b611ad2611a4d565b50611adb6112c3565b50611ae46112c3565b50611aed611454565b50611af6611973565b50611aff6114c6565b50611b08611a51565b50611b11612898565b90611b1a6128d8565b904690611b2630611a56565b90611b305f610dc2565b90611b42611b3d5f6116e4565b611a9b565b90611b4b611ac2565b96959493929190565b611b7d611b8291611b63611454565b50611b77611b71600b611647565b9161247a565b9061258f565b61164a565b90565b90611b8f90611625565b5f5260205260405f2090565b60ff1690565b611bad611bb291611458565b611b9b565b90565b611bbf9054611ba1565b90565b611be9915f611bde611be493611bd661127f565b5060056114d6565b01611b85565b611bb5565b90565b5f90565b611bf8611bec565b50611c01612773565b90565b611c0c6112c3565b50611c176004611410565b90565b611c41611c3c611c37611c4693611c2f611454565b50600a611631565b611647565b612918565b61164a565b90565b611c6691611c5561127f565b50611c5e612054565b91909161213d565b600190565b90611c7e91611c78611454565b50611666565b90565b611c9390611c8d611454565b50611c1a565b90565b611c9e611454565b50611ca7611481565b90565b7fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf90565b611d03611d0a94611cf9606094989795611cef608086019a5f8701906105c6565b602085019061088c565b60408301906104bb565b01906104bb565b565b60200190565b5190565b9395949092919542611d30611d2a89610432565b91610432565b11611da95791611d9b91611da293611d92611da79899611d7a611d51611caa565b611d6b8b938b611d5f610292565b95869460208601611cce565b602082018103825203826113c5565b611d8c611d8682611d12565b91611d0c565b2061298d565b929091926129aa565b91826129f4565b6127c6565b565b611dc4875f918291632341d78760e11b8352600483016104c8565b0390fd5b7f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c990565b9194611e34611e3e92989795611e2a60a096611e20611e459a611e1660c08a019e5f8b01906105c6565b602089019061088c565b604087019061088c565b60608501906104bb565b60808301906104bb565b01906104bb565b565b916020611e68929493611e6160408201965f83019061088c565b019061088c565b565b969591939294909442611e85611e7f83610432565b91610432565b11611f3f5790611eee611ef7949392611ed6611e9f611dc8565b611ec78c80948c91611eb18d91612a9b565b9192611ebb610292565b97889660208801611dec565b602082018103825203826113c5565b611ee8611ee282611d12565b91611d0c565b2061298d565b929091926129aa565b80611f0a611f0487610403565b91610403565b03611f1f5750611f1d9293919091612061565b565b8490611f3b5f9283926325c0072360e11b845260048401611e47565b0390fd5b611f5a905f91829163313c898160e11b8352600483016104c8565b0390fd5b90611f7991611f74611f6f82611510565b6121da565b611f7b565b565b90611f8591612399565b50565b90611f9291611f5e565b565b90611f9e90611625565b5f5260205260405f2090565b611fcf91611fc5611fca92611fbd611454565b506001611f94565b611a04565b611474565b90565b611fdc6040611883565b90565b5f90565b5f90565b611fef611fd2565b9060208083611ffc611fdf565b815201612007611fe3565b81525050565b612015611fe7565b90565b9061202b9161202561200d565b50612ace565b90565b61203661127f565b5061205061204a6301ffc9a760e01b6102a0565b916102a0565b1490565b61205c611973565b503390565b9161206f9291600192612af6565b565b60409061209a6120a1949695939661209060608401985f85019061088c565b60208301906104bb565b01906104bb565b565b906120ae9103610432565b90565b9291926120bf818390611faa565b90816120d46120ce5f19610432565b91610432565b106120e1575b5050509050565b816120f46120ee87610432565b91610432565b1061211a5761211193946121099193926120a3565b905f92612af6565b805f80806120da565b50612139849291925f938493637dc7a0d960e11b855260048501612071565b0390fd5b918261215961215361214e5f6116d8565b610403565b91610403565b146121b3578161217961217361216e5f6116d8565b610403565b91610403565b1461218c5761218a92919091612c05565b565b6121af6121985f6116d8565b5f91829163ec442f0560e01b835260048301610899565b0390fd5b6121d66121bf5f6116d8565b5f918291634b637e8f60e11b835260048301610899565b0390fd5b6121ec906121e6612054565b90612c37565b565b906121fa60ff91610dbd565b9181191691161790565b61220d906102ed565b90565b90565b9061222861222361222f92612204565b612210565b82546121ee565b9055565b61223b61127f565b5061225061224a828490611bc2565b156102ed565b5f146122d95761227860016122735f61226b600586906114d6565b018590611b85565b612213565b90612281612054565b906122be6122b86122b27f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d956114ca565b92611625565b92611625565b926122c7610292565b806122d18161064a565b0390a4600190565b50505f90565b6122e76114c6565b506122f130611a56565b61232361231d7f0000000000000000000000000000000000000000000000000000000000000000610403565b91610403565b148061235f575b5f14612354577f000000000000000000000000000000000000000000000000000000000000000090565b61235c612ce3565b90565b504661239361238d7f0000000000000000000000000000000000000000000000000000000000000000610432565b91610432565b1461232a565b6123a161127f565b506123ad818390611bc2565b5f14612435576123d45f6123cf5f6123c7600586906114d6565b018590611b85565b612213565b906123dd612054565b9061241a61241461240e7ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b956114ca565b92611625565b92611625565b92612423610292565b8061242d8161064a565b0390a4600190565b50505f90565b61244f61244a61245492610c77565b610bca565b610432565b90565b91602061247892949361247160408201965f8301906104bb565b0190610c82565b565b612482611bec565b5061248b611bf0565b8161249e6124988361243b565b91610432565b10156124b157506124ae90612dec565b90565b906124cc5f928392637669fc0f60e11b845260048401612457565b0390fd5b5490565b90565b6124eb6124e66124f0926124d4565b610bca565b610432565b90565b90565b65ffffffffffff1690565b61250d61251291611458565b6124f6565b90565b61251f9054612501565b90565b90565b61253961253461253e92612522565b610bca565b610432565b90565b60301c90565b60018060d01b031690565b61255e61256391612541565b612547565b90565b6125709054612552565b90565b61258761258261258c92610dba565b610bca565b61115f565b90565b906125e39061259c6111ea565b506125a85f84016124d0565b6125b15f6116e4565b9080806125c76125c160056124d7565b91610432565b11612644575b50906125de5f8601939192936124f3565b613443565b806125f66125f05f6116e4565b91610432565b145f1461260c5750506126085f612573565b5b90565b6126395f9161263461262e8461263f9601926126286001612525565b90611839565b916124f3565b613439565b01612566565b612609565b8061265261265892916130ce565b90611839565b908361268a61268461267f5f612679818c0161267489916124f3565b613439565b01612515565b610c77565b91610c77565b105f1461269b5750905b905f6125cd565b91506126b1906126ab6001612525565b90611700565b612694565b806126d16126cb6126c65f6116d8565b610403565b91610403565b146126ed576126eb916126e35f6116d8565b919091612c05565b565b6127106126f95f6116d8565b5f91829163ec442f0560e01b835260048301610899565b0390fd5b908161273061272a6127255f6116d8565b610403565b91610403565b1461274c5761274a91906127435f6116d8565b9091612c05565b565b61276f6127585f6116d8565b5f918291634b637e8f60e11b835260048301610899565b0390fd5b61277b611bec565b5061278543612dec565b90565b9061279960018060a01b0391610dbd565b9181191691161790565b90565b906127bb6127b66127c292611625565b6127a3565b8254612788565b9055565b9061284f916128496127d7826119b9565b6127ec846127e760098690611977565b6127a6565b8281859061282c6128266128207f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f95611625565b92611625565b92611625565b92612835610292565b8061283f8161064a565b0390a492916134d2565b9161350d565b565b61287861287361286e61287d936128666119eb565b50600a611631565b611647565b6136bb565b61373a565b90565b6128929061288c611454565b5061378b565b90565b90565b6128a06112c3565b506128d57f00000000000000000000000000000000000000000000000000000000000000006128cf6006612895565b906138a6565b90565b6128e06112c3565b506129157f000000000000000000000000000000000000000000000000000000000000000061290f6007612895565b906138a6565b90565b6129206111ea565b5061292c5f82016124d0565b8061293f6129395f6116e4565b91610432565b145f146129555750506129515f612573565b5b90565b6129825f9161297d612977846129889601926129716001612525565b90611839565b916124f3565b613439565b01612566565b612952565b6129a7906129996114c6565b506129a26122df565b6138f4565b90565b926129c5926129ce946129bb611973565b50929091926139ba565b90929192613ae5565b90565b9160206129f29294936129eb60408201965f83019061088c565b01906104bb565b565b6129fd81612a9b565b91612a10612a0a84610432565b91610432565b03612a19575050565b612a335f9283926301d4b62360e61b8452600484016129d1565b0390fd5b6001612a439101610432565b90565b90612a525f1991610dbd565b9181191691161790565b612a70612a6b612a7592610432565b610bca565b610432565b90565b90565b90612a90612a8b612a9792612a5c565b612a78565b8254612a46565b9055565b612aaf90612aa7611454565b506008611a04565b612acb612abb82611474565b91612ac583612a37565b90612a7b565b90565b90612aee612ae9612af393612ae161200d565b50600a611631565b611647565b613c5b565b90565b909281612b13612b0d612b085f6116d8565b610403565b91610403565b14612bde5783612b33612b2d612b285f6116d8565b610403565b91610403565b14612bb757612b5783612b52612b4b60018690611f94565b8790611a04565b612a7b565b612b61575b505050565b919091612bac612b9a612b947f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92593611625565b93611625565b93612ba3610292565b918291826104c8565b0390a35f8080612b5c565b612bda612bc35f6116d8565b5f918291634a1406b160e11b835260048301610899565b0390fd5b612c01612bea5f6116d8565b5f91829163e602df0560e01b835260048301610899565b0390fd5b91612c1292919091613c7c565b565b916020612c35929493612c2e60408201965f83019061088c565b01906105c6565b565b90612c4c612c46838390611bc2565b156102ed565b612c54575050565b612c6e5f92839263e2517d3f60e01b845260048401612c14565b0390fd5b7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f90565b90959492612ce194612cd0612cda92612cc6608096612cbc60a088019c5f8901906105c6565b60208701906105c6565b60408501906105c6565b60608301906104bb565b019061088c565b565b612ceb6114c6565b50612cf4612c72565b612d6b7f000000000000000000000000000000000000000000000000000000000000000091612d5c7f000000000000000000000000000000000000000000000000000000000000000046612d4730611a56565b91612d50610292565b96879560208701612c96565b602082018103825203826113c5565b612d7d612d7782611d12565b91611d0c565b2090565b90565b612d98612d93612d9d92612d81565b610bca565b610683565b90565b612da990612d84565b9052565b916020612dce929493612dc760408201965f830190612da0565b01906104bb565b565b612de4612ddf612de992610432565b610bca565b610c77565b90565b612df4611bec565b5080612e0e612e0865ffffffffffff61243b565b91610432565b11612e1f57612e1c90612dd0565b90565b6030612e3b5f9283926306dfcc6560e41b845260048401612dad565b0390fd5b90565b612e56612e51612e5b92612e3f565b610bca565b610432565b90565b90565b612e75612e70612e7a92612e5e565b610bca565b610683565b90565b1c90565b612ea090612e9a612e94612ea594610683565b91610432565b90612e7d565b610432565b90565b90565b612ebf612eba612ec492612ea8565b610bca565b610683565b90565b1b90565b612eea90612ee4612ede612eef94610683565b91610432565b90612ec7565b610432565b90565b90565b612f09612f04612f0e92612ef2565b610bca565b610432565b90565b90565b612f28612f23612f2d92612f11565b610bca565b610683565b90565b90565b612f47612f42612f4c92612f30565b610bca565b610432565b90565b90565b612f66612f61612f6b92612f4f565b610bca565b610683565b90565b90565b612f85612f80612f8a92612f6e565b610bca565b610432565b90565b90565b612fa4612f9f612fa992612f8d565b610bca565b610683565b90565b90565b612fc3612fbe612fc892612fac565b610bca565b610432565b90565b90565b612fe2612fdd612fe792612fcb565b610bca565b610683565b90565b612ffe612ff961300392612f4f565b610bca565b610432565b90565b90565b61301d61301861302292613006565b610bca565b610683565b90565b61303961303461303e92612fcb565b610bca565b610432565b90565b61305561305061305a92612522565b610bca565b610683565b90565b90565b61307461306f6130799261305d565b610bca565b610432565b90565b906130879102610432565b90565b634e487b7160e01b5f52601260045260245ffd5b6130aa6130b091610432565b91610432565b9081156130bb570490565b61308a565b906130cb9101610432565b90565b6130d6611454565b50806130eb6130e56001612525565b91610432565b111561343657806133006132dd6132cd6132bd6132ad61329d61328d61327d61326d61325d61324d8b6132476132406133069f613220613210613230926131326001612525565b908061314a613144600160801b612e42565b91610432565b1015613408575b8061316d61316768010000000000000000612ef5565b91610432565b10156133da575b8061318c613186640100000000612f33565b91610432565b10156133ac575b806131a96131a362010000612f71565b91610432565b101561337e575b806131c56131bf610100612faf565b91610432565b1015613350575b806131e06131da6010612fea565b91610432565b1015613322575b6131fa6131f46004613025565b91610432565b1015613309575b61320b6003613060565b61307c565b61321a6001613041565b90612e81565b61322a818661309e565b906130c0565b61323a6001613041565b90612e81565b809261309e565b906130c0565b6132576001613041565b90612e81565b613267818c61309e565b906130c0565b6132776001613041565b90612e81565b613287818a61309e565b906130c0565b6132976001613041565b90612e81565b6132a7818861309e565b906130c0565b6132b76001613041565b90612e81565b6132c7818661309e565b906130c0565b6132d76001613041565b90612e81565b916132fa6132f46132ef85809461309e565b610432565b91610432565b11613d0c565b906120a3565b90565b61331d906133176001613041565b90612ecb565b613201565b61333961334a916133336004612fce565b90612e81565b916133446002613009565b90612ecb565b906131e7565b613367613378916133616008612f90565b90612e81565b916133726004612fce565b90612ecb565b906131cc565b6133956133a69161338f6010612f52565b90612e81565b916133a06008612f90565b90612ecb565b906131b0565b6133c36133d4916133bd6020612f14565b90612e81565b916133ce6010612f52565b90612ecb565b90613193565b6133f1613402916133eb6040612eab565b90612e81565b916133fc6020612f14565b90612ecb565b90613174565b61341f613430916134196080612e61565b90612e81565b9161342a6040612eab565b90612ecb565b90613151565b90565b5f5260205f200190565b9391909261344f611454565b505b8161346461345e83610432565b91610432565b10156134ca57613475828290613d58565b9061348b5f613485888590613439565b01612515565b61349d61349787610c77565b91610c77565b115f146134ad5750915b91613451565b9291506134c4906134be6001612525565b90611700565b906134a7565b925050915090565b6134e4906134de611454565b50611a1a565b90565b90565b91602061350b92949361350460408201965f8301906104bb565b01906104bb565b565b9190918061352361351d85610403565b91610403565b1415806136a1575b613535575b505050565b8061355061354a6135455f6116d8565b610403565b91610403565b03613611575b508161357261356c6135675f6116d8565b610403565b91610403565b0361357e575b80613530565b6135c56135b86135bf92613594600a8690611631565b906135b26135ac6135a6600193613df1565b93611647565b916134e7565b90613e44565b929061164a565b9161164a565b9190916135f27fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72492611625565b926136076135fe610292565b928392836134ea565b0390a25f80613578565b613650613656613649613626600a8590611631565b600261364361363d61363789613df1565b93611647565b916134e7565b90613e44565b929061164a565b9161164a565b9190916136837fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72492611625565b9261369861368f610292565b928392836134ea565b0390a25f613556565b50816136b56136af5f6116e4565b91610432565b1161352b565b5f6136cf916136c8611454565b50016124d0565b90565b6136e66136e16136eb92610916565b610bca565b610432565b90565b6136f790612f14565b9052565b91602061371c92949361371560408201965f8301906136ee565b01906104bb565b565b61373261372d61373792610432565b610bca565b610916565b90565b6137426119eb565b508061375a61375463ffffffff6136d2565b91610432565b1161376b576137689061371e565b90565b60206137875f9283926306dfcc6560e41b8452600484016136fb565b0390fd5b6137a26137a79161379a611454565b506008611a04565b611474565b90565b90565b6137c16137bc6137c6926137aa565b610dbd565b610582565b90565b6137d360ff6137ad565b90565b5f5260205f2090565b905f92918054906137f96137f2836112dc565b8094611306565b916001811690815f146138505750600114613814575b505050565b61382191929394506137d6565b915f925b81841061383857505001905f808061380f565b60018160209295939554848601520191019290613825565b92949550505060ff19168252151560200201905f808061380f565b90613875916137df565b90565b9061389861389192613888610292565b9384809261386b565b03836113c5565b565b6138a390613878565b90565b906138af6112c3565b506138b9826114ca565b6138d26138cc6138c76137c9565b610582565b91610582565b14155f146138e757506138e490613ece565b90565b6138f1915061389a565b90565b6042916138ff6114c6565b50604051917f19010000000000000000000000000000000000000000000000000000000000008352600283015260228201522090565b5f90565b61394561394a91611458565b612a5c565b90565b90565b61396461395f6139699261394d565b610bca565b610432565b90565b6139a16139a89461399760609498979561398d608086019a5f8701906105c6565b6020850190610689565b60408301906105c6565b01906105c6565b565b6139b2610292565b3d5f823e3d90fd5b9392936139c5611973565b506139ce613935565b506139d76114c6565b506139e185613939565b613a13613a0d7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0613950565b91610432565b11613aa05790613a36602094955f94939293613a2d610292565b9485948561396c565b838052039060015afa15613a9b57613a4e5f51610dbd565b80613a69613a63613a5e5f6116d8565b610403565b91610403565b14613a7f575f91613a795f610dc2565b91929190565b50613a895f6116d8565b600191613a955f610dc2565b91929190565b6139aa565b505050613aac5f6116d8565b9060039291929190565b634e487b7160e01b5f52602160045260245ffd5b60041115613ad457565b613ab6565b90613ae382613aca565b565b80613af8613af25f613ad9565b91613ad9565b145f14613b03575050565b80613b17613b116001613ad9565b91613ad9565b145f14613b3a575f63f645eedf60e01b815280613b366004820161064a565b0390fd5b80613b4e613b486002613ad9565b91613ad9565b145f14613b7c57613b78613b6183613939565b5f91829163fce698f760e01b8352600483016104c8565b0390fd5b613b8f613b896003613ad9565b91613ad9565b14613b975750565b613bb2905f9182916335e2f38360e21b8352600483016105d3565b0390fd5b634e487b7160e01b5f52603260045260245ffd5b5f5260205f2090565b613bdc816124d0565b821015613bf657613bee600191613bca565b910201905f90565b613bb6565b90613c0590610c77565b9052565b90613c139061115f565b9052565b90613c4d613c445f613c27611fd2565b94613c3e613c36838301612515565b838801613bfb565b01612566565b60208401613c09565b565b613c5890613c17565b90565b613c79915f613c7392613c6c61200d565b5001613bd3565b50613c4f565b90565b9291613c8a84838391613efe565b83613ca5613c9f613c9a5f6116d8565b610403565b91610403565b14613cba575b613cb89293919091614088565b565b613cc2611481565b93613ccb61406d565b9480613cdf613cd988610432565b91610432565b11613cec57509350613cab565b8590613d085f928392630e58ae9360e11b8452600484016134ea565b0390fd5b613d14611454565b50151590565b613d2e613d29613d3392613006565b610bca565b610432565b90565b613d42613d4891610432565b91610432565b908115613d53570490565b61308a565b613d7d613d8392613d67611454565b508281169218613d776002613d1a565b90613d36565b90611700565b90565b90565b613d9d613d98613da292613d86565b610bca565b610683565b90565b613dae90613d89565b9052565b916020613dd3929493613dcc60408201965f830190613da5565b01906104bb565b565b613de9613de4613dee92610432565b610bca565b61115f565b90565b613df96111ea565b5080613e13613e0d60018060d01b0361164a565b91610432565b11613e2457613e2190613dd5565b90565b60d0613e405f9283926306dfcc6560e41b845260048401613db2565b0390fd5b90613e7a613e809392613e556111ea565b50613e5e6111ea565b508093613e73613e6c611bf0565b9492612918565b9091614503565b91614147565b91909190565b613e9a613e95613e9f92612f11565b610bca565b610432565b90565b369037565b90613ecc613eb4836118bb565b92602080613ec28693611898565b9201910390613ea2565b565b613ed66112c3565b50613ee0816141b1565b90613ef3613eee6020613e86565b613ea7565b918252602082015290565b91909180613f1c613f16613f115f6116d8565b610403565b91610403565b145f14613ffd57613f40613f3983613f346002611474565b611700565b6002612a7b565b5b82613f5c613f56613f515f6116d8565b610403565b91610403565b145f14613fd157613f80613f7983613f746002611474565b6120a3565b6002612a7b565b5b919091613fcc613fba613fb47fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef93611625565b93611625565b93613fc3610292565b918291826104c8565b0390a3565b613ff882613ff2613fe35f8790611a04565b91613fed83611474565b6130c0565b90612a7b565b613f81565b61401061400b5f8390611a04565b611474565b8061402361401d85610432565b91610432565b1061404b576140366140469184906120a3565b6140415f8490611a04565b612a7b565b613f41565b906140699091925f93849363391434e360e21b855260048501612071565b0390fd5b614075611454565b5061408560018060d01b0361164a565b90565b916140e06140da6140e794806140ae6140a86140a35f6116d8565b610403565b91610403565b14614118575b846140cf6140c96140c45f6116d8565b610403565b91610403565b146140e9575b6119b9565b926119b9565b909161350d565b565b614111600b600261410b6141056140ff89613df1565b93611647565b916134e7565b90613e44565b50506140d5565b614140600b600161413a61413461412e89613df1565b93611647565b916134e7565b90613e44565b50506140b4565b9161416b5f614170946141586111ea565b506141616111ea565b50019291926124f3565b6143b5565b91909190565b61418a61418561418f926137aa565b610bca565b610432565b90565b90565b6141a96141a46141ae92614192565b610bca565b610432565b90565b6141c66141cb916141c0611454565b506114ca565b613939565b6141d560ff614176565b16806141ea6141e4601f614195565b91610432565b116141f25790565b5f632cd44ac360e21b81528061420a6004820161064a565b0390fd5b5490565b61421c6040611883565b90565b5f5260205f2090565b6142318161420e565b82101561424b5761424360019161421f565b910201905f90565b613bb6565b634e487b7160e01b5f525f60045260245ffd5b61426d9051610c77565b90565b9061428165ffffffffffff91610dbd565b9181191691161790565b61429f61429a6142a492610c77565b610bca565b610c77565b90565b90565b906142bf6142ba6142c69261428b565b6142a7565b8254614270565b9055565b6142d4905161115f565b90565b60301b90565b906142ef65ffffffffffff19916142d7565b9181191691161790565b61430d6143086143129261115f565b610bca565b61115f565b90565b90565b9061432d614328614334926142f9565b614315565b82546142dd565b9055565b9061436260205f6143689461435a828201614354848801614263565b906142aa565b0192016142ca565b90614318565b565b919061437b5761437991614338565b565b614250565b90815491680100000000000000008310156143b057826143a89160016143ae95018155614228565b9061436a565b565b6113b1565b909291926143c16111ea565b506143ca6111ea565b506143d48261420e565b806143e76143e15f6116e4565b91610432565b115f146144b75761440d9061440784916144016001612525565b90611839565b90613439565b906144195f8301612515565b926144255f8401612566565b938061443961443385610c77565b91610c77565b1161449b5761445061444a84610c77565b91610c77565b145f1461446b575050614466905f859101614318565b5b9190565b61449692506144918661448861447f614212565b945f8601613bfb565b60208401613c09565b614380565b614467565b5f632520601d60e01b8152806144b36004820161064a565b0390fd5b506144e2916144dd856144d46144cb614212565b945f8601613bfb565b60208401613c09565b614380565b6144eb5f612573565b9190565b634e487b7160e01b5f52605160045260245ffd5b91909180600114614522576002036144ef5761451e91611269565b905b565b5061452c9161122a565b9061452056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x13W[a\x11\xE6V[a\0\x1D_5a\x02\x8CV[\x80c\x01\xFF\xC9\xA7\x14a\x02\x87W\x80c\x06\xFD\xDE\x03\x14a\x02\x82W\x80c\t^\xA7\xB3\x14a\x02}W\x80c\x18\x16\r\xDD\x14a\x02xW\x80c#\xB8r\xDD\x14a\x02sW\x80c$\x8A\x9C\xA3\x14a\x02nW\x80c//\xF1]\x14a\x02iW\x80c1<\xE5g\x14a\x02dW\x80c6D\xE5\x15\x14a\x02_W\x80c6V\x8A\xBE\x14a\x02ZW\x80c:F\xB1\xA8\x14a\x02UW\x80c@\xC1\x0F\x19\x14a\x02PW\x80cB\x96lh\x14a\x02KW\x80cK\xDD6\xCE\x14a\x02FW\x80cK\xF5\xD7\xE9\x14a\x02AW\x80cX|\xDE\x1E\x14a\x02<W\x80c\\\x19\xA9\\\x14a\x027W\x80co\xCF\xFFE\x14a\x022W\x80cp\xA0\x821\x14a\x02-W\x80c~\xCE\xBE\0\x14a\x02(W\x80c\x84\xB0\x19n\x14a\x02#W\x80c\x8D3C\xD6\x14a\x02\x1EW\x80c\x8ES\x9E\x8C\x14a\x02\x19W\x80c\x90-U\xA5\x14a\x02\x14W\x80c\x91\xD1HT\x14a\x02\x0FW\x80c\x91\xDD\xAD\xF4\x14a\x02\nW\x80c\x95\xD8\x9BA\x14a\x02\x05W\x80c\x9A\xB2N\xB0\x14a\x02\0W\x80c\x9B~\xF6K\x14a\x01\xFBW\x80c\xA2\x17\xFD\xDF\x14a\x01\xF6W\x80c\xA9\x05\x9C\xBB\x14a\x01\xF1W\x80c\xB0\xCA%>\x14a\x01\xECW\x80c\xBBMD6\x14a\x01\xE7W\x80c\xC0*\xE7T\x14a\x01\xE2W\x80c\xC3\xCD\xA5 \x14a\x01\xDDW\x80c\xD5\x05\xAC\xCF\x14a\x01\xD8W\x80c\xD5Gt\x1F\x14a\x01\xD3W\x80c\xDDb\xED>\x14a\x01\xCEWc\xF1\x12~\xD8\x03a\0\x0EWa\x11\xB0V[a\x10\xCCV[a\x10kV[a\x101V[a\x0F\x87V[a\x0E\xCBV[a\x0E\x96V[a\x0E`V[a\x0E*V[a\r\xF5V[a\r\x85V[a\r\x0EV[a\x0C\xD9V[a\x0C\xA4V[a\x0CAV[a\x0C\x0CV[a\x0B\x92V[a\x0B]V[a\n\xF2V[a\t\xABV[a\tvV[a\tAV[a\x08\xE3V[a\x08\xAEV[a\x089V[a\x08\x04V[a\x07\xD1V[a\x07\x7FV[a\x07IV[a\x07\x15V[a\x06\xE0V[a\x06\xABV[a\x06OV[a\x05\xE8V[a\x05LV[a\x04\xDDV[a\x04\x85V[a\x03\xC3V[a\x03\x14V[`\xE0\x1C\x90V[`@Q\x90V[_\x80\xFD[_\x80\xFD[c\xFF\xFF\xFF\xFF`\xE0\x1B\x16\x90V[a\x02\xB5\x81a\x02\xA0V[\x03a\x02\xBCWV[_\x80\xFD[\x90P5\x90a\x02\xCD\x82a\x02\xACV[V[\x90` \x82\x82\x03\x12a\x02\xE8Wa\x02\xE5\x91_\x01a\x02\xC0V[\x90V[a\x02\x9CV[\x15\x15\x90V[a\x02\xFB\x90a\x02\xEDV[\x90RV[\x91\x90a\x03\x12\x90_` \x85\x01\x94\x01\x90a\x02\xF2V[V[4a\x03DWa\x03@a\x03/a\x03*6`\x04a\x02\xCFV[a\x12\x83V[a\x037a\x02\x92V[\x91\x82\x91\x82a\x02\xFFV[\x03\x90\xF3[a\x02\x98V[_\x91\x03\x12a\x03SWV[a\x02\x9CV[Q\x90V[` \x91\x81R\x01\x90V[\x90\x82_\x93\x92\x82^\x01RV[`\x1F\x80\x19\x91\x01\x16\x90V[a\x03\x99a\x03\xA2` \x93a\x03\xA7\x93a\x03\x90\x81a\x03XV[\x93\x84\x80\x93a\x03\\V[\x95\x86\x91\x01a\x03eV[a\x03pV[\x01\x90V[a\x03\xC0\x91` \x82\x01\x91_\x81\x84\x03\x91\x01Ra\x03zV[\x90V[4a\x03\xF3Wa\x03\xD36`\x04a\x03IV[a\x03\xEFa\x03\xDEa\x14\x1CV[a\x03\xE6a\x02\x92V[\x91\x82\x91\x82a\x03\xABV[\x03\x90\xF3[a\x02\x98V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x04\x0C\x90a\x03\xF8V[\x90V[a\x04\x18\x81a\x04\x03V[\x03a\x04\x1FWV[_\x80\xFD[\x90P5\x90a\x040\x82a\x04\x0FV[V[\x90V[a\x04>\x81a\x042V[\x03a\x04EWV[_\x80\xFD[\x90P5\x90a\x04V\x82a\x045V[V[\x91\x90`@\x83\x82\x03\x12a\x04\x80W\x80a\x04ta\x04}\x92_\x86\x01a\x04#V[\x93` \x01a\x04IV[\x90V[a\x02\x9CV[4a\x04\xB6Wa\x04\xB2a\x04\xA1a\x04\x9B6`\x04a\x04XV[\x90a\x142V[a\x04\xA9a\x02\x92V[\x91\x82\x91\x82a\x02\xFFV[\x03\x90\xF3[a\x02\x98V[a\x04\xC4\x90a\x042V[\x90RV[\x91\x90a\x04\xDB\x90_` \x85\x01\x94\x01\x90a\x04\xBBV[V[4a\x05\rWa\x04\xED6`\x04a\x03IV[a\x05\ta\x04\xF8a\x14\x81V[a\x05\0a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[\x90\x91``\x82\x84\x03\x12a\x05GWa\x05Da\x05-\x84_\x85\x01a\x04#V[\x93a\x05;\x81` \x86\x01a\x04#V[\x93`@\x01a\x04IV[\x90V[a\x02\x9CV[4a\x05}Wa\x05ya\x05ha\x05b6`\x04a\x05\x12V[\x91a\x14\x97V[a\x05pa\x02\x92V[\x91\x82\x91\x82a\x02\xFFV[\x03\x90\xF3[a\x02\x98V[\x90V[a\x05\x8E\x81a\x05\x82V[\x03a\x05\x95WV[_\x80\xFD[\x90P5\x90a\x05\xA6\x82a\x05\x85V[V[\x90` \x82\x82\x03\x12a\x05\xC1Wa\x05\xBE\x91_\x01a\x05\x99V[\x90V[a\x02\x9CV[a\x05\xCF\x90a\x05\x82V[\x90RV[\x91\x90a\x05\xE6\x90_` \x85\x01\x94\x01\x90a\x05\xC6V[V[4a\x06\x18Wa\x06\x14a\x06\x03a\x05\xFE6`\x04a\x05\xA8V[a\x15\x10V[a\x06\x0Ba\x02\x92V[\x91\x82\x91\x82a\x05\xD3V[\x03\x90\xF3[a\x02\x98V[\x91\x90`@\x83\x82\x03\x12a\x06EW\x80a\x069a\x06B\x92_\x86\x01a\x05\x99V[\x93` \x01a\x04#V[\x90V[a\x02\x9CV[_\x01\x90V[4a\x06~Wa\x06ha\x06b6`\x04a\x06\x1DV[\x90a\x15\\V[a\x06pa\x02\x92V[\x80a\x06z\x81a\x06JV[\x03\x90\xF3[a\x02\x98V[`\xFF\x16\x90V[a\x06\x92\x90a\x06\x83V[\x90RV[\x91\x90a\x06\xA9\x90_` \x85\x01\x94\x01\x90a\x06\x89V[V[4a\x06\xDBWa\x06\xBB6`\x04a\x03IV[a\x06\xD7a\x06\xC6a\x15\x8BV[a\x06\xCEa\x02\x92V[\x91\x82\x91\x82a\x06\x96V[\x03\x90\xF3[a\x02\x98V[4a\x07\x10Wa\x06\xF06`\x04a\x03IV[a\x07\x0Ca\x06\xFBa\x15\xA1V[a\x07\x03a\x02\x92V[\x91\x82\x91\x82a\x05\xD3V[\x03\x90\xF3[a\x02\x98V[4a\x07DWa\x07.a\x07(6`\x04a\x06\x1DV[\x90a\x15\xB5V[a\x076a\x02\x92V[\x80a\x07@\x81a\x06JV[\x03\x90\xF3[a\x02\x98V[4a\x07zWa\x07va\x07ea\x07_6`\x04a\x04XV[\x90a\x16fV[a\x07ma\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[4a\x07\xAEWa\x07\x98a\x07\x926`\x04a\x04XV[\x90a\x17\xEDV[a\x07\xA0a\x02\x92V[\x80a\x07\xAA\x81a\x06JV[\x03\x90\xF3[a\x02\x98V[\x90` \x82\x82\x03\x12a\x07\xCCWa\x07\xC9\x91_\x01a\x04IV[\x90V[a\x02\x9CV[4a\x07\xFFWa\x07\xE9a\x07\xE46`\x04a\x07\xB3V[a\x17\xF9V[a\x07\xF1a\x02\x92V[\x80a\x07\xFB\x81a\x06JV[\x03\x90\xF3[a\x02\x98V[4a\x084Wa\x08\x146`\x04a\x03IV[a\x080a\x08\x1Fa\x18^V[a\x08'a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[4a\x08iWa\x08I6`\x04a\x03IV[a\x08ea\x08Ta\x19\x1DV[a\x08\\a\x02\x92V[\x91\x82\x91\x82a\x03\xABV[\x03\x90\xF3[a\x02\x98V[\x90` \x82\x82\x03\x12a\x08\x87Wa\x08\x84\x91_\x01a\x04#V[\x90V[a\x02\x9CV[a\x08\x95\x90a\x04\x03V[\x90RV[\x91\x90a\x08\xAC\x90_` \x85\x01\x94\x01\x90a\x08\x8CV[V[4a\x08\xDEWa\x08\xDAa\x08\xC9a\x08\xC46`\x04a\x08nV[a\x19\xB9V[a\x08\xD1a\x02\x92V[\x91\x82\x91\x82a\x08\x99V[\x03\x90\xF3[a\x02\x98V[4a\t\x11Wa\x08\xFBa\x08\xF66`\x04a\x08nV[a\x19\xD8V[a\t\x03a\x02\x92V[\x80a\t\r\x81a\x06JV[\x03\x90\xF3[a\x02\x98V[c\xFF\xFF\xFF\xFF\x16\x90V[a\t(\x90a\t\x16V[\x90RV[\x91\x90a\t?\x90_` \x85\x01\x94\x01\x90a\t\x1FV[V[4a\tqWa\tma\t\\a\tW6`\x04a\x08nV[a\x19\xEFV[a\tda\x02\x92V[\x91\x82\x91\x82a\t,V[\x03\x90\xF3[a\x02\x98V[4a\t\xA6Wa\t\xA2a\t\x91a\t\x8C6`\x04a\x08nV[a\x1A\x1AV[a\t\x99a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[4a\t\xDBWa\t\xD7a\t\xC6a\t\xC16`\x04a\x08nV[a\x1A8V[a\t\xCEa\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[`\xFF`\xF8\x1B\x16\x90V[a\t\xF2\x90a\t\xE0V[\x90RV[Q\x90V[` \x91\x81R\x01\x90V[` \x01\x90V[a\n\x12\x90a\x042V[\x90RV[\x90a\n#\x81` \x93a\n\tV[\x01\x90V[` \x01\x90V[\x90a\nJa\nDa\n=\x84a\t\xF6V[\x80\x93a\t\xFAV[\x92a\n\x03V[\x90_[\x81\x81\x10a\nZWPPP\x90V[\x90\x91\x92a\nsa\nm`\x01\x92\x86Qa\n\x16V[\x94a\n'V[\x91\x01\x91\x90\x91a\nMV[\x93\x95\x91\x94a\n\xCEa\n\xC3a\n\xE2\x95a\n\xB5a\n\xD8\x95a\n\xEF\x9C\x9Aa\n\xA8`\xE0\x8C\x01\x92_\x8D\x01\x90a\t\xE9V[\x8A\x82\x03` \x8C\x01Ra\x03zV[\x90\x88\x82\x03`@\x8A\x01Ra\x03zV[\x97``\x87\x01\x90a\x04\xBBV[`\x80\x85\x01\x90a\x08\x8CV[`\xA0\x83\x01\x90a\x05\xC6V[`\xC0\x81\x84\x03\x91\x01Ra\n-V[\x90V[4a\x0B)Wa\x0B\x026`\x04a\x03IV[a\x0B%a\x0B\ra\x1A\xCAV[\x93a\x0B\x1C\x97\x95\x97\x93\x91\x93a\x02\x92V[\x97\x88\x97\x88a\n}V[\x03\x90\xF3[a\x02\x98V[\x7F\x9B\x12\xE0\xC5p~II\x15\xE5\x8B\x05d\xF1\x8A\xAA\xD9\xB7J\xC6\x9B\xFC\x81Z\x1E\xDA\xDC\x8EK\xD02\xEB\x90V[a\x0BZa\x0B.V[\x90V[4a\x0B\x8DWa\x0Bm6`\x04a\x03IV[a\x0B\x89a\x0Bxa\x0BRV[a\x0B\x80a\x02\x92V[\x91\x82\x91\x82a\x05\xD3V[\x03\x90\xF3[a\x02\x98V[4a\x0B\xC2Wa\x0B\xBEa\x0B\xADa\x0B\xA86`\x04a\x07\xB3V[a\x1BTV[a\x0B\xB5a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[\x90V[\x90V[a\x0B\xE1a\x0B\xDCa\x0B\xE6\x92a\x0B\xC7V[a\x0B\xCAV[a\x042V[\x90V[a\x0B\xFEk\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x0B\xCDV[\x90V[a\x0C\ta\x0B\xE9V[\x90V[4a\x0C<Wa\x0C\x1C6`\x04a\x03IV[a\x0C8a\x0C'a\x0C\x01V[a\x0C/a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[4a\x0CrWa\x0Cna\x0C]a\x0CW6`\x04a\x06\x1DV[\x90a\x1B\xC2V[a\x0Cea\x02\x92V[\x91\x82\x91\x82a\x02\xFFV[\x03\x90\xF3[a\x02\x98V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a\x0C\x8B\x90a\x0CwV[\x90RV[\x91\x90a\x0C\xA2\x90_` \x85\x01\x94\x01\x90a\x0C\x82V[V[4a\x0C\xD4Wa\x0C\xB46`\x04a\x03IV[a\x0C\xD0a\x0C\xBFa\x1B\xF0V[a\x0C\xC7a\x02\x92V[\x91\x82\x91\x82a\x0C\x8FV[\x03\x90\xF3[a\x02\x98V[4a\r\tWa\x0C\xE96`\x04a\x03IV[a\r\x05a\x0C\xF4a\x1C\x04V[a\x0C\xFCa\x02\x92V[\x91\x82\x91\x82a\x03\xABV[\x03\x90\xF3[a\x02\x98V[4a\r>Wa\r:a\r)a\r$6`\x04a\x08nV[a\x1C\x1AV[a\r1a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[\x90V[a\rZa\rUa\r_\x92a\rCV[a\x0B\xCAV[a\x042V[\x90V[a\rwk\x02\xE8vi\xC3\x08sj\x04\0\0\0a\rFV[\x90V[a\r\x82a\rbV[\x90V[4a\r\xB5Wa\r\x956`\x04a\x03IV[a\r\xB1a\r\xA0a\rzV[a\r\xA8a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[\x90V[_\x1B\x90V[a\r\xD6a\r\xD1a\r\xDB\x92a\r\xBAV[a\r\xBDV[a\x05\x82V[\x90V[a\r\xE7_a\r\xC2V[\x90V[a\r\xF2a\r\xDEV[\x90V[4a\x0E%Wa\x0E\x056`\x04a\x03IV[a\x0E!a\x0E\x10a\r\xEAV[a\x0E\x18a\x02\x92V[\x91\x82\x91\x82a\x05\xD3V[\x03\x90\xF3[a\x02\x98V[4a\x0E[Wa\x0EWa\x0EFa\x0E@6`\x04a\x04XV[\x90a\x1CIV[a\x0ENa\x02\x92V[\x91\x82\x91\x82a\x02\xFFV[\x03\x90\xF3[a\x02\x98V[4a\x0E\x91Wa\x0E\x8Da\x0E|a\x0Ev6`\x04a\x04XV[\x90a\x1CkV[a\x0E\x84a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[4a\x0E\xC6Wa\x0E\xC2a\x0E\xB1a\x0E\xAC6`\x04a\x08nV[a\x1C\x81V[a\x0E\xB9a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[4a\x0E\xFBWa\x0E\xDB6`\x04a\x03IV[a\x0E\xF7a\x0E\xE6a\x1C\x96V[a\x0E\xEEa\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[a\x0F\t\x81a\x06\x83V[\x03a\x0F\x10WV[_\x80\xFD[\x90P5\x90a\x0F!\x82a\x0F\0V[V[\x90\x91`\xC0\x82\x84\x03\x12a\x0F\x82Wa\x0F;\x83_\x84\x01a\x04#V[\x92a\x0FI\x81` \x85\x01a\x04IV[\x92a\x0FW\x82`@\x83\x01a\x04IV[\x92a\x0F\x7Fa\x0Fh\x84``\x85\x01a\x0F\x14V[\x93a\x0Fv\x81`\x80\x86\x01a\x05\x99V[\x93`\xA0\x01a\x05\x99V[\x90V[a\x02\x9CV[4a\x0F\xBCWa\x0F\xA6a\x0F\x9A6`\x04a\x0F#V[\x94\x93\x90\x93\x92\x91\x92a\x1D\x16V[a\x0F\xAEa\x02\x92V[\x80a\x0F\xB8\x81a\x06JV[\x03\x90\xF3[a\x02\x98V[`\xE0\x81\x83\x03\x12a\x10,Wa\x0F\xD7\x82_\x83\x01a\x04#V[\x92a\x0F\xE5\x83` \x84\x01a\x04#V[\x92a\x0F\xF3\x81`@\x85\x01a\x04IV[\x92a\x10\x01\x82``\x83\x01a\x04IV[\x92a\x10)a\x10\x12\x84`\x80\x85\x01a\x0F\x14V[\x93a\x10 \x81`\xA0\x86\x01a\x05\x99V[\x93`\xC0\x01a\x05\x99V[\x90V[a\x02\x9CV[4a\x10fWa\x10Pa\x10D6`\x04a\x0F\xC1V[\x95\x94\x90\x94\x93\x91\x93a\x1EjV[a\x10Xa\x02\x92V[\x80a\x10b\x81a\x06JV[\x03\x90\xF3[a\x02\x98V[4a\x10\x9AWa\x10\x84a\x10~6`\x04a\x06\x1DV[\x90a\x1F\x88V[a\x10\x8Ca\x02\x92V[\x80a\x10\x96\x81a\x06JV[\x03\x90\xF3[a\x02\x98V[\x91\x90`@\x83\x82\x03\x12a\x10\xC7W\x80a\x10\xBBa\x10\xC4\x92_\x86\x01a\x04#V[\x93` \x01a\x04#V[\x90V[a\x02\x9CV[4a\x10\xFDWa\x10\xF9a\x10\xE8a\x10\xE26`\x04a\x10\x9FV[\x90a\x1F\xAAV[a\x10\xF0a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xF3[a\x02\x98V[a\x11\x0B\x81a\t\x16V[\x03a\x11\x12WV[_\x80\xFD[\x90P5\x90a\x11#\x82a\x11\x02V[V[\x91\x90`@\x83\x82\x03\x12a\x11MW\x80a\x11Aa\x11J\x92_\x86\x01a\x04#V[\x93` \x01a\x11\x16V[\x90V[a\x02\x9CV[a\x11[\x90a\x0CwV[\x90RV[`\x01\x80`\xD0\x1B\x03\x16\x90V[a\x11s\x90a\x11_V[\x90RV[\x90` \x80a\x11\x99\x93a\x11\x8F_\x82\x01Q_\x86\x01\x90a\x11RV[\x01Q\x91\x01\x90a\x11jV[V[\x91\x90a\x11\xAE\x90_`@\x85\x01\x94\x01\x90a\x11wV[V[4a\x11\xE1Wa\x11\xDDa\x11\xCCa\x11\xC66`\x04a\x11%V[\x90a \x18V[a\x11\xD4a\x02\x92V[\x91\x82\x91\x82a\x11\x9BV[\x03\x90\xF3[a\x02\x98V[_\x80\xFD[_\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[a\x12\x0Ea\x12\x14\x91a\x11_V[\x91a\x11_V[\x01\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x12%WV[a\x11\xEEV[\x90a\x12=\x91a\x127a\x11\xEAV[Pa\x12\x02V[\x90V[a\x12La\x12R\x91a\x11_V[\x91a\x11_V[\x90\x03\x90`\x01\x80`\xD0\x1B\x03\x82\x11a\x12dWV[a\x11\xEEV[\x90a\x12|\x91a\x12va\x11\xEAV[Pa\x12@V[\x90V[_\x90V[a\x12\x8Ba\x12\x7FV[P\x80a\x12\xA6a\x12\xA0cye\xDB\x0B`\xE0\x1Ba\x02\xA0V[\x91a\x02\xA0V[\x14\x90\x81\x15a\x12\xB3W[P\x90V[a\x12\xBD\x91Pa .V[_a\x12\xAFV[``\x90V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x01`\x02\x83\x04\x92\x16\x80\x15a\x12\xFCW[` \x83\x10\x14a\x12\xF7WV[a\x12\xC8V[\x91`\x7F\x16\x91a\x12\xECV[` \x91\x81R\x01\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a\x132a\x13+\x83a\x12\xDCV[\x80\x94a\x13\x06V[\x91`\x01\x81\x16\x90\x81_\x14a\x13\x89WP`\x01\x14a\x13MW[PPPV[a\x13Z\x91\x92\x93\x94Pa\x13\x0FV[\x91_\x92[\x81\x84\x10a\x13qWPP\x01\x90_\x80\x80a\x13HV[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a\x13^V[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a\x13HV[\x90a\x13\xAE\x91a\x13\x18V[\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90a\x13\xCF\x90a\x03pV[\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x13\xE9W`@RV[a\x13\xB1V[\x90a\x14\x0Ea\x14\x07\x92a\x13\xFEa\x02\x92V[\x93\x84\x80\x92a\x13\xA4V[\x03\x83a\x13\xC5V[V[a\x14\x19\x90a\x13\xEEV[\x90V[a\x14$a\x12\xC3V[Pa\x14/`\x03a\x14\x10V[\x90V[a\x14O\x91a\x14>a\x12\x7FV[Pa\x14Ga TV[\x91\x90\x91a aV[`\x01\x90V[_\x90V[_\x1C\x90V[\x90V[a\x14la\x14q\x91a\x14XV[a\x14]V[\x90V[a\x14~\x90Ta\x14`V[\x90V[a\x14\x89a\x14TV[Pa\x14\x94`\x02a\x14tV[\x90V[\x91a\x14\xC1\x92a\x14\xA4a\x12\x7FV[Pa\x14\xB9a\x14\xB0a TV[\x82\x90\x84\x91a \xB1V[\x91\x90\x91a!=V[`\x01\x90V[_\x90V[a\x14\xD3\x90a\x05\x82V[\x90V[\x90a\x14\xE0\x90a\x14\xCAV[_R` R`@_ \x90V[\x90V[a\x14\xFBa\x15\0\x91a\x14XV[a\x14\xECV[\x90V[a\x15\r\x90Ta\x14\xEFV[\x90V[`\x01a\x15)a\x15/\x92a\x15!a\x14\xC6V[P`\x05a\x14\xD6V[\x01a\x15\x03V[\x90V[\x90a\x15M\x91a\x15Ha\x15C\x82a\x15\x10V[a!\xDAV[a\x15OV[V[\x90a\x15Y\x91a\"3V[PV[\x90a\x15f\x91a\x152V[V[_\x90V[\x90V[a\x15\x83a\x15~a\x15\x88\x92a\x15lV[a\x0B\xCAV[a\x06\x83V[\x90V[a\x15\x93a\x15hV[Pa\x15\x9E`\x12a\x15oV[\x90V[a\x15\xA9a\x14\xC6V[Pa\x15\xB2a\"\xDFV[\x90V[\x90\x80a\x15\xD0a\x15\xCAa\x15\xC5a TV[a\x04\x03V[\x91a\x04\x03V[\x03a\x15\xE1Wa\x15\xDE\x91a#\x99V[PV[_c3K\xD9\x19`\xE1\x1B\x81R\x80a\x15\xF9`\x04\x82\x01a\x06JV[\x03\x90\xFD[a\x16\x11a\x16\x0Ca\x16\x16\x92a\x03\xF8V[a\x0B\xCAV[a\x03\xF8V[\x90V[a\x16\"\x90a\x15\xFDV[\x90V[a\x16.\x90a\x16\x19V[\x90V[\x90a\x16;\x90a\x16%V[_R` R`@_ \x90V[\x90V[a\x16^a\x16Ya\x16c\x92a\x11_V[a\x0B\xCAV[a\x042V[\x90V[a\x16\x9D\x91a\x16\x92a\x16\x8Ca\x16\x87a\x16\x98\x94a\x16\x7Fa\x14TV[P`\na\x161V[a\x16GV[\x91a$zV[\x90a%\x8FV[a\x16JV[\x90V[\x90a\x16\xBA\x91a\x16\xB5a\x16\xB0a\x0B.V[a!\xDAV[a\x17%V[V[a\x16\xD0a\x16\xCBa\x16\xD5\x92a\r\xBAV[a\x0B\xCAV[a\x03\xF8V[\x90V[a\x16\xE1\x90a\x16\xBCV[\x90V[a\x16\xF8a\x16\xF3a\x16\xFD\x92a\r\xBAV[a\x0B\xCAV[a\x042V[\x90V[a\x17\x0Fa\x17\x15\x91\x93\x92\x93a\x042V[\x92a\x042V[\x82\x01\x80\x92\x11a\x17 WV[a\x11\xEEV[\x90\x81a\x17Aa\x17;a\x176_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a\x17\xD1W\x80a\x17Ya\x17S_a\x16\xE4V[\x91a\x042V[\x14a\x17\xB5Wa\x17pa\x17ia\x14\x81V[\x82\x90a\x17\0V[a\x17\x89a\x17\x83a\x17~a\x0B\xE9V[a\x042V[\x91a\x042V[\x11a\x17\x99Wa\x17\x97\x91a&\xB6V[V[_c\x17~?\xC3`\xE0\x1B\x81R\x80a\x17\xB1`\x04\x82\x01a\x06JV[\x03\x90\xFD[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x17\xCD`\x04\x82\x01a\x06JV[\x03\x90\xFD[_c\xD9.#=`\xE0\x1B\x81R\x80a\x17\xE9`\x04\x82\x01a\x06JV[\x03\x90\xFD[\x90a\x17\xF7\x91a\x16\xA0V[V[\x80a\x18\x0Ca\x18\x06_a\x16\xE4V[\x91a\x042V[\x14a\x18\x1DWa\x18\x1B\x903a'\x14V[V[_c\x1F* \x05`\xE0\x1B\x81R\x80a\x185`\x04\x82\x01a\x06JV[\x03\x90\xFD[a\x18Ha\x18N\x91\x93\x92\x93a\x042V[\x92a\x042V[\x82\x03\x91\x82\x11a\x18YWV[a\x11\xEEV[a\x18fa\x14TV[Pa\x18\x80a\x18ra\x0B\xE9V[a\x18za\x14\x81V[\x90a\x189V[\x90V[\x90a\x18\x96a\x18\x8Fa\x02\x92V[\x92\x83a\x13\xC5V[V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x18\xB6Wa\x18\xB2` \x91a\x03pV[\x01\x90V[a\x13\xB1V[\x90a\x18\xCDa\x18\xC8\x83a\x18\x98V[a\x18\x83V[\x91\x82RV[_\x7Fmode=blocknumber&from=default\0\0\0\x91\x01RV[a\x19\x03`\x1Da\x18\xBBV[\x90a\x19\x10` \x83\x01a\x18\xD2V[V[a\x19\x1Aa\x18\xF9V[\x90V[a\x19%a\x12\xC3V[Pa\x19.a\x1B\xF0V[a\x19Ga\x19Aa\x19<a'sV[a\x0CwV[\x91a\x0CwV[\x03a\x19WWa\x19Ta\x19\x12V[\x90V[_c\x01\xBF\xC1\xC5`\xE6\x1B\x81R\x80a\x19o`\x04\x82\x01a\x06JV[\x03\x90\xFD[_\x90V[\x90a\x19\x81\x90a\x16%V[_R` R`@_ \x90V[`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x19\xA4a\x19\xA9\x91a\x14XV[a\x19\x8DV[\x90V[a\x19\xB6\x90Ta\x19\x98V[\x90V[a\x19\xD0a\x19\xD5\x91a\x19\xC8a\x19sV[P`\ta\x19wV[a\x19\xACV[\x90V[a\x19\xE9\x90a\x19\xE4a TV[a'\xC6V[V[_\x90V[a\x1A\x01\x90a\x19\xFBa\x19\xEBV[Pa(QV[\x90V[\x90a\x1A\x0E\x90a\x16%V[_R` R`@_ \x90V[a\x1A0a\x1A5\x91a\x1A)a\x14TV[P_a\x1A\x04V[a\x14tV[\x90V[a\x1AJ\x90a\x1ADa\x14TV[Pa(\x80V[\x90V[_\x90V[``\x90V[a\x1A_\x90a\x16\x19V[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x1AzW` \x80\x91\x02\x01\x90V[a\x13\xB1V[\x90a\x1A\x91a\x1A\x8C\x83a\x1AbV[a\x18\x83V[\x91\x82RV[6\x907V[\x90a\x1A\xC0a\x1A\xA8\x83a\x1A\x7FV[\x92` \x80a\x1A\xB6\x86\x93a\x1AbV[\x92\x01\x91\x03\x90a\x1A\x96V[V[`\x0F`\xF8\x1B\x90V[a\x1A\xD2a\x1AMV[Pa\x1A\xDBa\x12\xC3V[Pa\x1A\xE4a\x12\xC3V[Pa\x1A\xEDa\x14TV[Pa\x1A\xF6a\x19sV[Pa\x1A\xFFa\x14\xC6V[Pa\x1B\x08a\x1AQV[Pa\x1B\x11a(\x98V[\x90a\x1B\x1Aa(\xD8V[\x90F\x90a\x1B&0a\x1AVV[\x90a\x1B0_a\r\xC2V[\x90a\x1BBa\x1B=_a\x16\xE4V[a\x1A\x9BV[\x90a\x1BKa\x1A\xC2V[\x96\x95\x94\x93\x92\x91\x90V[a\x1B}a\x1B\x82\x91a\x1Bca\x14TV[Pa\x1Bwa\x1Bq`\x0Ba\x16GV[\x91a$zV[\x90a%\x8FV[a\x16JV[\x90V[\x90a\x1B\x8F\x90a\x16%V[_R` R`@_ \x90V[`\xFF\x16\x90V[a\x1B\xADa\x1B\xB2\x91a\x14XV[a\x1B\x9BV[\x90V[a\x1B\xBF\x90Ta\x1B\xA1V[\x90V[a\x1B\xE9\x91_a\x1B\xDEa\x1B\xE4\x93a\x1B\xD6a\x12\x7FV[P`\x05a\x14\xD6V[\x01a\x1B\x85V[a\x1B\xB5V[\x90V[_\x90V[a\x1B\xF8a\x1B\xECV[Pa\x1C\x01a'sV[\x90V[a\x1C\x0Ca\x12\xC3V[Pa\x1C\x17`\x04a\x14\x10V[\x90V[a\x1CAa\x1C<a\x1C7a\x1CF\x93a\x1C/a\x14TV[P`\na\x161V[a\x16GV[a)\x18V[a\x16JV[\x90V[a\x1Cf\x91a\x1CUa\x12\x7FV[Pa\x1C^a TV[\x91\x90\x91a!=V[`\x01\x90V[\x90a\x1C~\x91a\x1Cxa\x14TV[Pa\x16fV[\x90V[a\x1C\x93\x90a\x1C\x8Da\x14TV[Pa\x1C\x1AV[\x90V[a\x1C\x9Ea\x14TV[Pa\x1C\xA7a\x14\x81V[\x90V[\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF\x90V[a\x1D\x03a\x1D\n\x94a\x1C\xF9``\x94\x98\x97\x95a\x1C\xEF`\x80\x86\x01\x9A_\x87\x01\x90a\x05\xC6V[` \x85\x01\x90a\x08\x8CV[`@\x83\x01\x90a\x04\xBBV[\x01\x90a\x04\xBBV[V[` \x01\x90V[Q\x90V[\x93\x95\x94\x90\x92\x91\x95Ba\x1D0a\x1D*\x89a\x042V[\x91a\x042V[\x11a\x1D\xA9W\x91a\x1D\x9B\x91a\x1D\xA2\x93a\x1D\x92a\x1D\xA7\x98\x99a\x1Dza\x1DQa\x1C\xAAV[a\x1Dk\x8B\x93\x8Ba\x1D_a\x02\x92V[\x95\x86\x94` \x86\x01a\x1C\xCEV[` \x82\x01\x81\x03\x82R\x03\x82a\x13\xC5V[a\x1D\x8Ca\x1D\x86\x82a\x1D\x12V[\x91a\x1D\x0CV[ a)\x8DV[\x92\x90\x91\x92a)\xAAV[\x91\x82a)\xF4V[a'\xC6V[V[a\x1D\xC4\x87_\x91\x82\x91c#A\xD7\x87`\xE1\x1B\x83R`\x04\x83\x01a\x04\xC8V[\x03\x90\xFD[\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x90V[\x91\x94a\x1E4a\x1E>\x92\x98\x97\x95a\x1E*`\xA0\x96a\x1E a\x1EE\x9Aa\x1E\x16`\xC0\x8A\x01\x9E_\x8B\x01\x90a\x05\xC6V[` \x89\x01\x90a\x08\x8CV[`@\x87\x01\x90a\x08\x8CV[``\x85\x01\x90a\x04\xBBV[`\x80\x83\x01\x90a\x04\xBBV[\x01\x90a\x04\xBBV[V[\x91` a\x1Eh\x92\x94\x93a\x1Ea`@\x82\x01\x96_\x83\x01\x90a\x08\x8CV[\x01\x90a\x08\x8CV[V[\x96\x95\x91\x93\x92\x94\x90\x94Ba\x1E\x85a\x1E\x7F\x83a\x042V[\x91a\x042V[\x11a\x1F?W\x90a\x1E\xEEa\x1E\xF7\x94\x93\x92a\x1E\xD6a\x1E\x9Fa\x1D\xC8V[a\x1E\xC7\x8C\x80\x94\x8C\x91a\x1E\xB1\x8D\x91a*\x9BV[\x91\x92a\x1E\xBBa\x02\x92V[\x97\x88\x96` \x88\x01a\x1D\xECV[` \x82\x01\x81\x03\x82R\x03\x82a\x13\xC5V[a\x1E\xE8a\x1E\xE2\x82a\x1D\x12V[\x91a\x1D\x0CV[ a)\x8DV[\x92\x90\x91\x92a)\xAAV[\x80a\x1F\na\x1F\x04\x87a\x04\x03V[\x91a\x04\x03V[\x03a\x1F\x1FWPa\x1F\x1D\x92\x93\x91\x90\x91a aV[V[\x84\x90a\x1F;_\x92\x83\x92c%\xC0\x07#`\xE1\x1B\x84R`\x04\x84\x01a\x1EGV[\x03\x90\xFD[a\x1FZ\x90_\x91\x82\x91c1<\x89\x81`\xE1\x1B\x83R`\x04\x83\x01a\x04\xC8V[\x03\x90\xFD[\x90a\x1Fy\x91a\x1Fta\x1Fo\x82a\x15\x10V[a!\xDAV[a\x1F{V[V[\x90a\x1F\x85\x91a#\x99V[PV[\x90a\x1F\x92\x91a\x1F^V[V[\x90a\x1F\x9E\x90a\x16%V[_R` R`@_ \x90V[a\x1F\xCF\x91a\x1F\xC5a\x1F\xCA\x92a\x1F\xBDa\x14TV[P`\x01a\x1F\x94V[a\x1A\x04V[a\x14tV[\x90V[a\x1F\xDC`@a\x18\x83V[\x90V[_\x90V[_\x90V[a\x1F\xEFa\x1F\xD2V[\x90` \x80\x83a\x1F\xFCa\x1F\xDFV[\x81R\x01a \x07a\x1F\xE3V[\x81RPPV[a \x15a\x1F\xE7V[\x90V[\x90a +\x91a %a \rV[Pa*\xCEV[\x90V[a 6a\x12\x7FV[Pa Pa Jc\x01\xFF\xC9\xA7`\xE0\x1Ba\x02\xA0V[\x91a\x02\xA0V[\x14\x90V[a \\a\x19sV[P3\x90V[\x91a o\x92\x91`\x01\x92a*\xF6V[V[`@\x90a \x9Aa \xA1\x94\x96\x95\x93\x96a \x90``\x84\x01\x98_\x85\x01\x90a\x08\x8CV[` \x83\x01\x90a\x04\xBBV[\x01\x90a\x04\xBBV[V[\x90a \xAE\x91\x03a\x042V[\x90V[\x92\x91\x92a \xBF\x81\x83\x90a\x1F\xAAV[\x90\x81a \xD4a \xCE_\x19a\x042V[\x91a\x042V[\x10a \xE1W[PPP\x90PV[\x81a \xF4a \xEE\x87a\x042V[\x91a\x042V[\x10a!\x1AWa!\x11\x93\x94a!\t\x91\x93\x92a \xA3V[\x90_\x92a*\xF6V[\x80_\x80\x80a \xDAV[Pa!9\x84\x92\x91\x92_\x93\x84\x93c}\xC7\xA0\xD9`\xE1\x1B\x85R`\x04\x85\x01a qV[\x03\x90\xFD[\x91\x82a!Ya!Sa!N_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a!\xB3W\x81a!ya!sa!n_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a!\x8CWa!\x8A\x92\x91\x90\x91a,\x05V[V[a!\xAFa!\x98_a\x16\xD8V[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\x08\x99V[\x03\x90\xFD[a!\xD6a!\xBF_a\x16\xD8V[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\x08\x99V[\x03\x90\xFD[a!\xEC\x90a!\xE6a TV[\x90a,7V[V[\x90a!\xFA`\xFF\x91a\r\xBDV[\x91\x81\x19\x16\x91\x16\x17\x90V[a\"\r\x90a\x02\xEDV[\x90V[\x90V[\x90a\"(a\"#a\"/\x92a\"\x04V[a\"\x10V[\x82Ta!\xEEV[\x90UV[a\";a\x12\x7FV[Pa\"Pa\"J\x82\x84\x90a\x1B\xC2V[\x15a\x02\xEDV[_\x14a\"\xD9Wa\"x`\x01a\"s_a\"k`\x05\x86\x90a\x14\xD6V[\x01\x85\x90a\x1B\x85V[a\"\x13V[\x90a\"\x81a TV[\x90a\"\xBEa\"\xB8a\"\xB2\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x95a\x14\xCAV[\x92a\x16%V[\x92a\x16%V[\x92a\"\xC7a\x02\x92V[\x80a\"\xD1\x81a\x06JV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a\"\xE7a\x14\xC6V[Pa\"\xF10a\x1AVV[a##a#\x1D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04\x03V[\x91a\x04\x03V[\x14\x80a#_W[_\x14a#TW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a#\\a,\xE3V[\x90V[PFa#\x93a#\x8D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x042V[\x91a\x042V[\x14a#*V[a#\xA1a\x12\x7FV[Pa#\xAD\x81\x83\x90a\x1B\xC2V[_\x14a$5Wa#\xD4_a#\xCF_a#\xC7`\x05\x86\x90a\x14\xD6V[\x01\x85\x90a\x1B\x85V[a\"\x13V[\x90a#\xDDa TV[\x90a$\x1Aa$\x14a$\x0E\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x95a\x14\xCAV[\x92a\x16%V[\x92a\x16%V[\x92a$#a\x02\x92V[\x80a$-\x81a\x06JV[\x03\x90\xA4`\x01\x90V[PP_\x90V[a$Oa$Ja$T\x92a\x0CwV[a\x0B\xCAV[a\x042V[\x90V[\x91` a$x\x92\x94\x93a$q`@\x82\x01\x96_\x83\x01\x90a\x04\xBBV[\x01\x90a\x0C\x82V[V[a$\x82a\x1B\xECV[Pa$\x8Ba\x1B\xF0V[\x81a$\x9Ea$\x98\x83a$;V[\x91a\x042V[\x10\x15a$\xB1WPa$\xAE\x90a-\xECV[\x90V[\x90a$\xCC_\x92\x83\x92cvi\xFC\x0F`\xE1\x1B\x84R`\x04\x84\x01a$WV[\x03\x90\xFD[T\x90V[\x90V[a$\xEBa$\xE6a$\xF0\x92a$\xD4V[a\x0B\xCAV[a\x042V[\x90V[\x90V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[a%\ra%\x12\x91a\x14XV[a$\xF6V[\x90V[a%\x1F\x90Ta%\x01V[\x90V[\x90V[a%9a%4a%>\x92a%\"V[a\x0B\xCAV[a\x042V[\x90V[`0\x1C\x90V[`\x01\x80`\xD0\x1B\x03\x16\x90V[a%^a%c\x91a%AV[a%GV[\x90V[a%p\x90Ta%RV[\x90V[a%\x87a%\x82a%\x8C\x92a\r\xBAV[a\x0B\xCAV[a\x11_V[\x90V[\x90a%\xE3\x90a%\x9Ca\x11\xEAV[Pa%\xA8_\x84\x01a$\xD0V[a%\xB1_a\x16\xE4V[\x90\x80\x80a%\xC7a%\xC1`\x05a$\xD7V[\x91a\x042V[\x11a&DW[P\x90a%\xDE_\x86\x01\x93\x91\x92\x93a$\xF3V[a4CV[\x80a%\xF6a%\xF0_a\x16\xE4V[\x91a\x042V[\x14_\x14a&\x0CWPPa&\x08_a%sV[[\x90V[a&9_\x91a&4a&.\x84a&?\x96\x01\x92a&(`\x01a%%V[\x90a\x189V[\x91a$\xF3V[a49V[\x01a%fV[a&\tV[\x80a&Ra&X\x92\x91a0\xCEV[\x90a\x189V[\x90\x83a&\x8Aa&\x84a&\x7F_a&y\x81\x8C\x01a&t\x89\x91a$\xF3V[a49V[\x01a%\x15V[a\x0CwV[\x91a\x0CwV[\x10_\x14a&\x9BWP\x90[\x90_a%\xCDV[\x91Pa&\xB1\x90a&\xAB`\x01a%%V[\x90a\x17\0V[a&\x94V[\x80a&\xD1a&\xCBa&\xC6_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a&\xEDWa&\xEB\x91a&\xE3_a\x16\xD8V[\x91\x90\x91a,\x05V[V[a'\x10a&\xF9_a\x16\xD8V[_\x91\x82\x91c\xECD/\x05`\xE0\x1B\x83R`\x04\x83\x01a\x08\x99V[\x03\x90\xFD[\x90\x81a'0a'*a'%_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a'LWa'J\x91\x90a'C_a\x16\xD8V[\x90\x91a,\x05V[V[a'oa'X_a\x16\xD8V[_\x91\x82\x91cKc~\x8F`\xE1\x1B\x83R`\x04\x83\x01a\x08\x99V[\x03\x90\xFD[a'{a\x1B\xECV[Pa'\x85Ca-\xECV[\x90V[\x90a'\x99`\x01\x80`\xA0\x1B\x03\x91a\r\xBDV[\x91\x81\x19\x16\x91\x16\x17\x90V[\x90V[\x90a'\xBBa'\xB6a'\xC2\x92a\x16%V[a'\xA3V[\x82Ta'\x88V[\x90UV[\x90a(O\x91a(Ia'\xD7\x82a\x19\xB9V[a'\xEC\x84a'\xE7`\t\x86\x90a\x19wV[a'\xA6V[\x82\x81\x85\x90a(,a(&a( \x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x95a\x16%V[\x92a\x16%V[\x92a\x16%V[\x92a(5a\x02\x92V[\x80a(?\x81a\x06JV[\x03\x90\xA4\x92\x91a4\xD2V[\x91a5\rV[V[a(xa(sa(na(}\x93a(fa\x19\xEBV[P`\na\x161V[a\x16GV[a6\xBBV[a7:V[\x90V[a(\x92\x90a(\x8Ca\x14TV[Pa7\x8BV[\x90V[\x90V[a(\xA0a\x12\xC3V[Pa(\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a(\xCF`\x06a(\x95V[\x90a8\xA6V[\x90V[a(\xE0a\x12\xC3V[Pa)\x15\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a)\x0F`\x07a(\x95V[\x90a8\xA6V[\x90V[a) a\x11\xEAV[Pa),_\x82\x01a$\xD0V[\x80a)?a)9_a\x16\xE4V[\x91a\x042V[\x14_\x14a)UWPPa)Q_a%sV[[\x90V[a)\x82_\x91a)}a)w\x84a)\x88\x96\x01\x92a)q`\x01a%%V[\x90a\x189V[\x91a$\xF3V[a49V[\x01a%fV[a)RV[a)\xA7\x90a)\x99a\x14\xC6V[Pa)\xA2a\"\xDFV[a8\xF4V[\x90V[\x92a)\xC5\x92a)\xCE\x94a)\xBBa\x19sV[P\x92\x90\x91\x92a9\xBAV[\x90\x92\x91\x92a:\xE5V[\x90V[\x91` a)\xF2\x92\x94\x93a)\xEB`@\x82\x01\x96_\x83\x01\x90a\x08\x8CV[\x01\x90a\x04\xBBV[V[a)\xFD\x81a*\x9BV[\x91a*\x10a*\n\x84a\x042V[\x91a\x042V[\x03a*\x19WPPV[a*3_\x92\x83\x92c\x01\xD4\xB6#`\xE6\x1B\x84R`\x04\x84\x01a)\xD1V[\x03\x90\xFD[`\x01a*C\x91\x01a\x042V[\x90V[\x90a*R_\x19\x91a\r\xBDV[\x91\x81\x19\x16\x91\x16\x17\x90V[a*pa*ka*u\x92a\x042V[a\x0B\xCAV[a\x042V[\x90V[\x90V[\x90a*\x90a*\x8Ba*\x97\x92a*\\V[a*xV[\x82Ta*FV[\x90UV[a*\xAF\x90a*\xA7a\x14TV[P`\x08a\x1A\x04V[a*\xCBa*\xBB\x82a\x14tV[\x91a*\xC5\x83a*7V[\x90a*{V[\x90V[\x90a*\xEEa*\xE9a*\xF3\x93a*\xE1a \rV[P`\na\x161V[a\x16GV[a<[V[\x90V[\x90\x92\x81a+\x13a+\ra+\x08_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a+\xDEW\x83a+3a+-a+(_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a+\xB7Wa+W\x83a+Ra+K`\x01\x86\x90a\x1F\x94V[\x87\x90a\x1A\x04V[a*{V[a+aW[PPPV[\x91\x90\x91a+\xACa+\x9Aa+\x94\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x93a\x16%V[\x93a\x16%V[\x93a+\xA3a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xA3_\x80\x80a+\\V[a+\xDAa+\xC3_a\x16\xD8V[_\x91\x82\x91cJ\x14\x06\xB1`\xE1\x1B\x83R`\x04\x83\x01a\x08\x99V[\x03\x90\xFD[a,\x01a+\xEA_a\x16\xD8V[_\x91\x82\x91c\xE6\x02\xDF\x05`\xE0\x1B\x83R`\x04\x83\x01a\x08\x99V[\x03\x90\xFD[\x91a,\x12\x92\x91\x90\x91a<|V[V[\x91` a,5\x92\x94\x93a,.`@\x82\x01\x96_\x83\x01\x90a\x08\x8CV[\x01\x90a\x05\xC6V[V[\x90a,La,F\x83\x83\x90a\x1B\xC2V[\x15a\x02\xEDV[a,TWPPV[a,n_\x92\x83\x92c\xE2Q}?`\xE0\x1B\x84R`\x04\x84\x01a,\x14V[\x03\x90\xFD[\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x90V[\x90\x95\x94\x92a,\xE1\x94a,\xD0a,\xDA\x92a,\xC6`\x80\x96a,\xBC`\xA0\x88\x01\x9C_\x89\x01\x90a\x05\xC6V[` \x87\x01\x90a\x05\xC6V[`@\x85\x01\x90a\x05\xC6V[``\x83\x01\x90a\x04\xBBV[\x01\x90a\x08\x8CV[V[a,\xEBa\x14\xC6V[Pa,\xF4a,rV[a-k\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a-\\\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Fa-G0a\x1AVV[\x91a-Pa\x02\x92V[\x96\x87\x95` \x87\x01a,\x96V[` \x82\x01\x81\x03\x82R\x03\x82a\x13\xC5V[a-}a-w\x82a\x1D\x12V[\x91a\x1D\x0CV[ \x90V[\x90V[a-\x98a-\x93a-\x9D\x92a-\x81V[a\x0B\xCAV[a\x06\x83V[\x90V[a-\xA9\x90a-\x84V[\x90RV[\x91` a-\xCE\x92\x94\x93a-\xC7`@\x82\x01\x96_\x83\x01\x90a-\xA0V[\x01\x90a\x04\xBBV[V[a-\xE4a-\xDFa-\xE9\x92a\x042V[a\x0B\xCAV[a\x0CwV[\x90V[a-\xF4a\x1B\xECV[P\x80a.\x0Ea.\x08e\xFF\xFF\xFF\xFF\xFF\xFFa$;V[\x91a\x042V[\x11a.\x1FWa.\x1C\x90a-\xD0V[\x90V[`0a.;_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a-\xADV[\x03\x90\xFD[\x90V[a.Va.Qa.[\x92a.?V[a\x0B\xCAV[a\x042V[\x90V[\x90V[a.ua.pa.z\x92a.^V[a\x0B\xCAV[a\x06\x83V[\x90V[\x1C\x90V[a.\xA0\x90a.\x9Aa.\x94a.\xA5\x94a\x06\x83V[\x91a\x042V[\x90a.}V[a\x042V[\x90V[\x90V[a.\xBFa.\xBAa.\xC4\x92a.\xA8V[a\x0B\xCAV[a\x06\x83V[\x90V[\x1B\x90V[a.\xEA\x90a.\xE4a.\xDEa.\xEF\x94a\x06\x83V[\x91a\x042V[\x90a.\xC7V[a\x042V[\x90V[\x90V[a/\ta/\x04a/\x0E\x92a.\xF2V[a\x0B\xCAV[a\x042V[\x90V[\x90V[a/(a/#a/-\x92a/\x11V[a\x0B\xCAV[a\x06\x83V[\x90V[\x90V[a/Ga/Ba/L\x92a/0V[a\x0B\xCAV[a\x042V[\x90V[\x90V[a/fa/aa/k\x92a/OV[a\x0B\xCAV[a\x06\x83V[\x90V[\x90V[a/\x85a/\x80a/\x8A\x92a/nV[a\x0B\xCAV[a\x042V[\x90V[\x90V[a/\xA4a/\x9Fa/\xA9\x92a/\x8DV[a\x0B\xCAV[a\x06\x83V[\x90V[\x90V[a/\xC3a/\xBEa/\xC8\x92a/\xACV[a\x0B\xCAV[a\x042V[\x90V[\x90V[a/\xE2a/\xDDa/\xE7\x92a/\xCBV[a\x0B\xCAV[a\x06\x83V[\x90V[a/\xFEa/\xF9a0\x03\x92a/OV[a\x0B\xCAV[a\x042V[\x90V[\x90V[a0\x1Da0\x18a0\"\x92a0\x06V[a\x0B\xCAV[a\x06\x83V[\x90V[a09a04a0>\x92a/\xCBV[a\x0B\xCAV[a\x042V[\x90V[a0Ua0Pa0Z\x92a%\"V[a\x0B\xCAV[a\x06\x83V[\x90V[\x90V[a0ta0oa0y\x92a0]V[a\x0B\xCAV[a\x042V[\x90V[\x90a0\x87\x91\x02a\x042V[\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a0\xAAa0\xB0\x91a\x042V[\x91a\x042V[\x90\x81\x15a0\xBBW\x04\x90V[a0\x8AV[\x90a0\xCB\x91\x01a\x042V[\x90V[a0\xD6a\x14TV[P\x80a0\xEBa0\xE5`\x01a%%V[\x91a\x042V[\x11\x15a46W\x80a3\0a2\xDDa2\xCDa2\xBDa2\xADa2\x9Da2\x8Da2}a2ma2]a2M\x8Ba2Ga2@a3\x06\x9Fa2 a2\x10a20\x92a12`\x01a%%V[\x90\x80a1Ja1D`\x01`\x80\x1Ba.BV[\x91a\x042V[\x10\x15a4\x08W[\x80a1ma1gh\x01\0\0\0\0\0\0\0\0a.\xF5V[\x91a\x042V[\x10\x15a3\xDAW[\x80a1\x8Ca1\x86d\x01\0\0\0\0a/3V[\x91a\x042V[\x10\x15a3\xACW[\x80a1\xA9a1\xA3b\x01\0\0a/qV[\x91a\x042V[\x10\x15a3~W[\x80a1\xC5a1\xBFa\x01\0a/\xAFV[\x91a\x042V[\x10\x15a3PW[\x80a1\xE0a1\xDA`\x10a/\xEAV[\x91a\x042V[\x10\x15a3\"W[a1\xFAa1\xF4`\x04a0%V[\x91a\x042V[\x10\x15a3\tW[a2\x0B`\x03a0`V[a0|V[a2\x1A`\x01a0AV[\x90a.\x81V[a2*\x81\x86a0\x9EV[\x90a0\xC0V[a2:`\x01a0AV[\x90a.\x81V[\x80\x92a0\x9EV[\x90a0\xC0V[a2W`\x01a0AV[\x90a.\x81V[a2g\x81\x8Ca0\x9EV[\x90a0\xC0V[a2w`\x01a0AV[\x90a.\x81V[a2\x87\x81\x8Aa0\x9EV[\x90a0\xC0V[a2\x97`\x01a0AV[\x90a.\x81V[a2\xA7\x81\x88a0\x9EV[\x90a0\xC0V[a2\xB7`\x01a0AV[\x90a.\x81V[a2\xC7\x81\x86a0\x9EV[\x90a0\xC0V[a2\xD7`\x01a0AV[\x90a.\x81V[\x91a2\xFAa2\xF4a2\xEF\x85\x80\x94a0\x9EV[a\x042V[\x91a\x042V[\x11a=\x0CV[\x90a \xA3V[\x90V[a3\x1D\x90a3\x17`\x01a0AV[\x90a.\xCBV[a2\x01V[a39a3J\x91a33`\x04a/\xCEV[\x90a.\x81V[\x91a3D`\x02a0\tV[\x90a.\xCBV[\x90a1\xE7V[a3ga3x\x91a3a`\x08a/\x90V[\x90a.\x81V[\x91a3r`\x04a/\xCEV[\x90a.\xCBV[\x90a1\xCCV[a3\x95a3\xA6\x91a3\x8F`\x10a/RV[\x90a.\x81V[\x91a3\xA0`\x08a/\x90V[\x90a.\xCBV[\x90a1\xB0V[a3\xC3a3\xD4\x91a3\xBD` a/\x14V[\x90a.\x81V[\x91a3\xCE`\x10a/RV[\x90a.\xCBV[\x90a1\x93V[a3\xF1a4\x02\x91a3\xEB`@a.\xABV[\x90a.\x81V[\x91a3\xFC` a/\x14V[\x90a.\xCBV[\x90a1tV[a4\x1Fa40\x91a4\x19`\x80a.aV[\x90a.\x81V[\x91a4*`@a.\xABV[\x90a.\xCBV[\x90a1QV[\x90V[_R` _ \x01\x90V[\x93\x91\x90\x92a4Oa\x14TV[P[\x81a4da4^\x83a\x042V[\x91a\x042V[\x10\x15a4\xCAWa4u\x82\x82\x90a=XV[\x90a4\x8B_a4\x85\x88\x85\x90a49V[\x01a%\x15V[a4\x9Da4\x97\x87a\x0CwV[\x91a\x0CwV[\x11_\x14a4\xADWP\x91[\x91a4QV[\x92\x91Pa4\xC4\x90a4\xBE`\x01a%%V[\x90a\x17\0V[\x90a4\xA7V[\x92PP\x91P\x90V[a4\xE4\x90a4\xDEa\x14TV[Pa\x1A\x1AV[\x90V[\x90V[\x91` a5\x0B\x92\x94\x93a5\x04`@\x82\x01\x96_\x83\x01\x90a\x04\xBBV[\x01\x90a\x04\xBBV[V[\x91\x90\x91\x80a5#a5\x1D\x85a\x04\x03V[\x91a\x04\x03V[\x14\x15\x80a6\xA1W[a55W[PPPV[\x80a5Pa5Ja5E_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x03a6\x11W[P\x81a5ra5la5g_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x03a5~W[\x80a50V[a5\xC5a5\xB8a5\xBF\x92a5\x94`\n\x86\x90a\x161V[\x90a5\xB2a5\xACa5\xA6`\x01\x93a=\xF1V[\x93a\x16GV[\x91a4\xE7V[\x90a>DV[\x92\x90a\x16JV[\x91a\x16JV[\x91\x90\x91a5\xF2\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x16%V[\x92a6\x07a5\xFEa\x02\x92V[\x92\x83\x92\x83a4\xEAV[\x03\x90\xA2_\x80a5xV[a6Pa6Va6Ia6&`\n\x85\x90a\x161V[`\x02a6Ca6=a67\x89a=\xF1V[\x93a\x16GV[\x91a4\xE7V[\x90a>DV[\x92\x90a\x16JV[\x91a\x16JV[\x91\x90\x91a6\x83\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x92a\x16%V[\x92a6\x98a6\x8Fa\x02\x92V[\x92\x83\x92\x83a4\xEAV[\x03\x90\xA2_a5VV[P\x81a6\xB5a6\xAF_a\x16\xE4V[\x91a\x042V[\x11a5+V[_a6\xCF\x91a6\xC8a\x14TV[P\x01a$\xD0V[\x90V[a6\xE6a6\xE1a6\xEB\x92a\t\x16V[a\x0B\xCAV[a\x042V[\x90V[a6\xF7\x90a/\x14V[\x90RV[\x91` a7\x1C\x92\x94\x93a7\x15`@\x82\x01\x96_\x83\x01\x90a6\xEEV[\x01\x90a\x04\xBBV[V[a72a7-a77\x92a\x042V[a\x0B\xCAV[a\t\x16V[\x90V[a7Ba\x19\xEBV[P\x80a7Za7Tc\xFF\xFF\xFF\xFFa6\xD2V[\x91a\x042V[\x11a7kWa7h\x90a7\x1EV[\x90V[` a7\x87_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a6\xFBV[\x03\x90\xFD[a7\xA2a7\xA7\x91a7\x9Aa\x14TV[P`\x08a\x1A\x04V[a\x14tV[\x90V[\x90V[a7\xC1a7\xBCa7\xC6\x92a7\xAAV[a\r\xBDV[a\x05\x82V[\x90V[a7\xD3`\xFFa7\xADV[\x90V[_R` _ \x90V[\x90_\x92\x91\x80T\x90a7\xF9a7\xF2\x83a\x12\xDCV[\x80\x94a\x13\x06V[\x91`\x01\x81\x16\x90\x81_\x14a8PWP`\x01\x14a8\x14W[PPPV[a8!\x91\x92\x93\x94Pa7\xD6V[\x91_\x92[\x81\x84\x10a88WPP\x01\x90_\x80\x80a8\x0FV[`\x01\x81` \x92\x95\x93\x95T\x84\x86\x01R\x01\x91\x01\x92\x90a8%V[\x92\x94\x95PPP`\xFF\x19\x16\x82R\x15\x15` \x02\x01\x90_\x80\x80a8\x0FV[\x90a8u\x91a7\xDFV[\x90V[\x90a8\x98a8\x91\x92a8\x88a\x02\x92V[\x93\x84\x80\x92a8kV[\x03\x83a\x13\xC5V[V[a8\xA3\x90a8xV[\x90V[\x90a8\xAFa\x12\xC3V[Pa8\xB9\x82a\x14\xCAV[a8\xD2a8\xCCa8\xC7a7\xC9V[a\x05\x82V[\x91a\x05\x82V[\x14\x15_\x14a8\xE7WPa8\xE4\x90a>\xCEV[\x90V[a8\xF1\x91Pa8\x9AV[\x90V[`B\x91a8\xFFa\x14\xC6V[P`@Q\x91\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[_\x90V[a9Ea9J\x91a\x14XV[a*\\V[\x90V[\x90V[a9da9_a9i\x92a9MV[a\x0B\xCAV[a\x042V[\x90V[a9\xA1a9\xA8\x94a9\x97``\x94\x98\x97\x95a9\x8D`\x80\x86\x01\x9A_\x87\x01\x90a\x05\xC6V[` \x85\x01\x90a\x06\x89V[`@\x83\x01\x90a\x05\xC6V[\x01\x90a\x05\xC6V[V[a9\xB2a\x02\x92V[=_\x82>=\x90\xFD[\x93\x92\x93a9\xC5a\x19sV[Pa9\xCEa95V[Pa9\xD7a\x14\xC6V[Pa9\xE1\x85a99V[a:\x13a:\r\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0a9PV[\x91a\x042V[\x11a:\xA0W\x90a:6` \x94\x95_\x94\x93\x92\x93a:-a\x02\x92V[\x94\x85\x94\x85a9lV[\x83\x80R\x03\x90`\x01Z\xFA\x15a:\x9BWa:N_Qa\r\xBDV[\x80a:ia:ca:^_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a:\x7FW_\x91a:y_a\r\xC2V[\x91\x92\x91\x90V[Pa:\x89_a\x16\xD8V[`\x01\x91a:\x95_a\r\xC2V[\x91\x92\x91\x90V[a9\xAAV[PPPa:\xAC_a\x16\xD8V[\x90`\x03\x92\x91\x92\x91\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x04\x11\x15a:\xD4WV[a:\xB6V[\x90a:\xE3\x82a:\xCAV[V[\x80a:\xF8a:\xF2_a:\xD9V[\x91a:\xD9V[\x14_\x14a;\x03WPPV[\x80a;\x17a;\x11`\x01a:\xD9V[\x91a:\xD9V[\x14_\x14a;:W_c\xF6E\xEE\xDF`\xE0\x1B\x81R\x80a;6`\x04\x82\x01a\x06JV[\x03\x90\xFD[\x80a;Na;H`\x02a:\xD9V[\x91a:\xD9V[\x14_\x14a;|Wa;xa;a\x83a99V[_\x91\x82\x91c\xFC\xE6\x98\xF7`\xE0\x1B\x83R`\x04\x83\x01a\x04\xC8V[\x03\x90\xFD[a;\x8Fa;\x89`\x03a:\xD9V[\x91a:\xD9V[\x14a;\x97WPV[a;\xB2\x90_\x91\x82\x91c5\xE2\xF3\x83`\xE2\x1B\x83R`\x04\x83\x01a\x05\xD3V[\x03\x90\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_R` _ \x90V[a;\xDC\x81a$\xD0V[\x82\x10\x15a;\xF6Wa;\xEE`\x01\x91a;\xCAV[\x91\x02\x01\x90_\x90V[a;\xB6V[\x90a<\x05\x90a\x0CwV[\x90RV[\x90a<\x13\x90a\x11_V[\x90RV[\x90a<Ma<D_a<'a\x1F\xD2V[\x94a<>a<6\x83\x83\x01a%\x15V[\x83\x88\x01a;\xFBV[\x01a%fV[` \x84\x01a<\tV[V[a<X\x90a<\x17V[\x90V[a<y\x91_a<s\x92a<la \rV[P\x01a;\xD3V[Pa<OV[\x90V[\x92\x91a<\x8A\x84\x83\x83\x91a>\xFEV[\x83a<\xA5a<\x9Fa<\x9A_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a<\xBAW[a<\xB8\x92\x93\x91\x90\x91a@\x88V[V[a<\xC2a\x14\x81V[\x93a<\xCBa@mV[\x94\x80a<\xDFa<\xD9\x88a\x042V[\x91a\x042V[\x11a<\xECWP\x93Pa<\xABV[\x85\x90a=\x08_\x92\x83\x92c\x0EX\xAE\x93`\xE1\x1B\x84R`\x04\x84\x01a4\xEAV[\x03\x90\xFD[a=\x14a\x14TV[P\x15\x15\x90V[a=.a=)a=3\x92a0\x06V[a\x0B\xCAV[a\x042V[\x90V[a=Ba=H\x91a\x042V[\x91a\x042V[\x90\x81\x15a=SW\x04\x90V[a0\x8AV[a=}a=\x83\x92a=ga\x14TV[P\x82\x81\x16\x92\x18a=w`\x02a=\x1AV[\x90a=6V[\x90a\x17\0V[\x90V[\x90V[a=\x9Da=\x98a=\xA2\x92a=\x86V[a\x0B\xCAV[a\x06\x83V[\x90V[a=\xAE\x90a=\x89V[\x90RV[\x91` a=\xD3\x92\x94\x93a=\xCC`@\x82\x01\x96_\x83\x01\x90a=\xA5V[\x01\x90a\x04\xBBV[V[a=\xE9a=\xE4a=\xEE\x92a\x042V[a\x0B\xCAV[a\x11_V[\x90V[a=\xF9a\x11\xEAV[P\x80a>\x13a>\r`\x01\x80`\xD0\x1B\x03a\x16JV[\x91a\x042V[\x11a>$Wa>!\x90a=\xD5V[\x90V[`\xD0a>@_\x92\x83\x92c\x06\xDF\xCCe`\xE4\x1B\x84R`\x04\x84\x01a=\xB2V[\x03\x90\xFD[\x90a>za>\x80\x93\x92a>Ua\x11\xEAV[Pa>^a\x11\xEAV[P\x80\x93a>sa>la\x1B\xF0V[\x94\x92a)\x18V[\x90\x91aE\x03V[\x91aAGV[\x91\x90\x91\x90V[a>\x9Aa>\x95a>\x9F\x92a/\x11V[a\x0B\xCAV[a\x042V[\x90V[6\x907V[\x90a>\xCCa>\xB4\x83a\x18\xBBV[\x92` \x80a>\xC2\x86\x93a\x18\x98V[\x92\x01\x91\x03\x90a>\xA2V[V[a>\xD6a\x12\xC3V[Pa>\xE0\x81aA\xB1V[\x90a>\xF3a>\xEE` a>\x86V[a>\xA7V[\x91\x82R` \x82\x01R\x90V[\x91\x90\x91\x80a?\x1Ca?\x16a?\x11_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14_\x14a?\xFDWa?@a?9\x83a?4`\x02a\x14tV[a\x17\0V[`\x02a*{V[[\x82a?\\a?Va?Q_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14_\x14a?\xD1Wa?\x80a?y\x83a?t`\x02a\x14tV[a \xA3V[`\x02a*{V[[\x91\x90\x91a?\xCCa?\xBAa?\xB4\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93a\x16%V[\x93a\x16%V[\x93a?\xC3a\x02\x92V[\x91\x82\x91\x82a\x04\xC8V[\x03\x90\xA3V[a?\xF8\x82a?\xF2a?\xE3_\x87\x90a\x1A\x04V[\x91a?\xED\x83a\x14tV[a0\xC0V[\x90a*{V[a?\x81V[a@\x10a@\x0B_\x83\x90a\x1A\x04V[a\x14tV[\x80a@#a@\x1D\x85a\x042V[\x91a\x042V[\x10a@KWa@6a@F\x91\x84\x90a \xA3V[a@A_\x84\x90a\x1A\x04V[a*{V[a?AV[\x90a@i\x90\x91\x92_\x93\x84\x93c9\x144\xE3`\xE2\x1B\x85R`\x04\x85\x01a qV[\x03\x90\xFD[a@ua\x14TV[Pa@\x85`\x01\x80`\xD0\x1B\x03a\x16JV[\x90V[\x91a@\xE0a@\xDAa@\xE7\x94\x80a@\xAEa@\xA8a@\xA3_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14aA\x18W[\x84a@\xCFa@\xC9a@\xC4_a\x16\xD8V[a\x04\x03V[\x91a\x04\x03V[\x14a@\xE9W[a\x19\xB9V[\x92a\x19\xB9V[\x90\x91a5\rV[V[aA\x11`\x0B`\x02aA\x0BaA\x05a@\xFF\x89a=\xF1V[\x93a\x16GV[\x91a4\xE7V[\x90a>DV[PPa@\xD5V[aA@`\x0B`\x01aA:aA4aA.\x89a=\xF1V[\x93a\x16GV[\x91a4\xE7V[\x90a>DV[PPa@\xB4V[\x91aAk_aAp\x94aAXa\x11\xEAV[PaAaa\x11\xEAV[P\x01\x92\x91\x92a$\xF3V[aC\xB5V[\x91\x90\x91\x90V[aA\x8AaA\x85aA\x8F\x92a7\xAAV[a\x0B\xCAV[a\x042V[\x90V[\x90V[aA\xA9aA\xA4aA\xAE\x92aA\x92V[a\x0B\xCAV[a\x042V[\x90V[aA\xC6aA\xCB\x91aA\xC0a\x14TV[Pa\x14\xCAV[a99V[aA\xD5`\xFFaAvV[\x16\x80aA\xEAaA\xE4`\x1FaA\x95V[\x91a\x042V[\x11aA\xF2W\x90V[_c,\xD4J\xC3`\xE2\x1B\x81R\x80aB\n`\x04\x82\x01a\x06JV[\x03\x90\xFD[T\x90V[aB\x1C`@a\x18\x83V[\x90V[_R` _ \x90V[aB1\x81aB\x0EV[\x82\x10\x15aBKWaBC`\x01\x91aB\x1FV[\x91\x02\x01\x90_\x90V[a;\xB6V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[aBm\x90Qa\x0CwV[\x90V[\x90aB\x81e\xFF\xFF\xFF\xFF\xFF\xFF\x91a\r\xBDV[\x91\x81\x19\x16\x91\x16\x17\x90V[aB\x9FaB\x9AaB\xA4\x92a\x0CwV[a\x0B\xCAV[a\x0CwV[\x90V[\x90V[\x90aB\xBFaB\xBAaB\xC6\x92aB\x8BV[aB\xA7V[\x82TaBpV[\x90UV[aB\xD4\x90Qa\x11_V[\x90V[`0\x1B\x90V[\x90aB\xEFe\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91aB\xD7V[\x91\x81\x19\x16\x91\x16\x17\x90V[aC\raC\x08aC\x12\x92a\x11_V[a\x0B\xCAV[a\x11_V[\x90V[\x90V[\x90aC-aC(aC4\x92aB\xF9V[aC\x15V[\x82TaB\xDDV[\x90UV[\x90aCb` _aCh\x94aCZ\x82\x82\x01aCT\x84\x88\x01aBcV[\x90aB\xAAV[\x01\x92\x01aB\xCAV[\x90aC\x18V[V[\x91\x90aC{WaCy\x91aC8V[V[aBPV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15aC\xB0W\x82aC\xA8\x91`\x01aC\xAE\x95\x01\x81UaB(V[\x90aCjV[V[a\x13\xB1V[\x90\x92\x91\x92aC\xC1a\x11\xEAV[PaC\xCAa\x11\xEAV[PaC\xD4\x82aB\x0EV[\x80aC\xE7aC\xE1_a\x16\xE4V[\x91a\x042V[\x11_\x14aD\xB7WaD\r\x90aD\x07\x84\x91aD\x01`\x01a%%V[\x90a\x189V[\x90a49V[\x90aD\x19_\x83\x01a%\x15V[\x92aD%_\x84\x01a%fV[\x93\x80aD9aD3\x85a\x0CwV[\x91a\x0CwV[\x11aD\x9BWaDPaDJ\x84a\x0CwV[\x91a\x0CwV[\x14_\x14aDkWPPaDf\x90_\x85\x91\x01aC\x18V[[\x91\x90V[aD\x96\x92PaD\x91\x86aD\x88aD\x7FaB\x12V[\x94_\x86\x01a;\xFBV[` \x84\x01a<\tV[aC\x80V[aDgV[_c% `\x1D`\xE0\x1B\x81R\x80aD\xB3`\x04\x82\x01a\x06JV[\x03\x90\xFD[PaD\xE2\x91aD\xDD\x85aD\xD4aD\xCBaB\x12V[\x94_\x86\x01a;\xFBV[` \x84\x01a<\tV[aC\x80V[aD\xEB_a%sV[\x91\x90V[cNH{q`\xE0\x1B_R`Q`\x04R`$_\xFD[\x91\x90\x91\x80`\x01\x14aE\"W`\x02\x03aD\xEFWaE\x1E\x91a\x12iV[\x90[V[PaE,\x91a\x12*V[\x90aE V",
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
    ///Container for all the [`SyndicateToken`](self) function calls.
    pub enum SyndicateTokenCalls {
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
        supportsInterface(supportsInterfaceCall),
        #[allow(missing_docs)]
        symbol(symbolCall),
        #[allow(missing_docs)]
        totalSupply(totalSupplyCall),
        #[allow(missing_docs)]
        transfer(transferCall),
        #[allow(missing_docs)]
        transferFrom(transferFromCall),
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
            [88u8, 124u8, 222u8, 30u8],
            [92u8, 25u8, 169u8, 92u8],
            [111u8, 207u8, 255u8, 69u8],
            [112u8, 160u8, 130u8, 49u8],
            [126u8, 206u8, 190u8, 0u8],
            [132u8, 176u8, 25u8, 110u8],
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
        const COUNT: usize = 39usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
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
                    fn burn(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SyndicateTokenCalls> {
                        <burnCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SyndicateTokenCalls::burn)
                    }
                    burn
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
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
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
            [179u8, 81u8, 43u8, 12u8],
            [215u8, 139u8, 206u8, 12u8],
            [217u8, 46u8, 35u8, 61u8],
            [226u8, 81u8, 125u8, 63u8],
            [228u8, 80u8, 211u8, 140u8],
            [230u8, 2u8, 223u8, 5u8],
            [236u8, 68u8, 47u8, 5u8],
            [236u8, 211u8, 248u8, 30u8],
            [246u8, 69u8, 238u8, 223u8],
            [251u8, 143u8, 65u8, 178u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SyndicateTokenErrors {
        const NAME: &'static str = "SyndicateTokenErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 25usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AccessControlBadConfirmation(_) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AccessControlUnauthorizedAccount(_) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
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
        Transfer(Transfer),
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
        const COUNT: usize = 8usize;
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
                Some(<Transfer as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Transfer as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Transfer)
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
                Self::Transfer(inner) => {
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
                Self::Transfer(inner) => {
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
        ///Creates a new call builder for the [`burn`] function.
        pub fn burn(
            &self,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, burnCall, N> {
            self.call_builder(&burnCall { amount })
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
        ///Creates a new event filter for the [`Transfer`] event.
        pub fn Transfer_filter(&self) -> alloy_contract::Event<T, &P, Transfer, N> {
            self.event_filter::<Transfer>()
        }
    }
}
