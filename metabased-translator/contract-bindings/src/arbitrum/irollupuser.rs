///Module containing a contract's types and functions.
/**

```solidity
library IRollupCore {
    struct Staker { uint256 amountStaked; bytes32 latestStakedAssertion; uint64 index; bool isStaked; address withdrawalAddress; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IRollupCore {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct Staker { uint256 amountStaked; bytes32 latestStakedAssertion; uint64 index; bool isStaked; address withdrawalAddress; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Staker {
        pub amountStaked: alloy::sol_types::private::primitives::aliases::U256,
        pub latestStakedAssertion: alloy::sol_types::private::FixedBytes<32>,
        pub index: u64,
        pub isStaked: bool,
        pub withdrawalAddress: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::FixedBytes<32>,
            u64,
            bool,
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
        impl ::core::convert::From<Staker> for UnderlyingRustTuple<'_> {
            fn from(value: Staker) -> Self {
                (
                    value.amountStaked,
                    value.latestStakedAssertion,
                    value.index,
                    value.isStaked,
                    value.withdrawalAddress,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Staker {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    amountStaked: tuple.0,
                    latestStakedAssertion: tuple.1,
                    index: tuple.2,
                    isStaked: tuple.3,
                    withdrawalAddress: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Staker {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Staker {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountStaked),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.latestStakedAssertion,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isStaked,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawalAddress,
                    ),
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
        impl alloy_sol_types::SolType for Staker {
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
        impl alloy_sol_types::SolStruct for Staker {
            const NAME: &'static str = "Staker";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Staker(uint256 amountStaked,bytes32 latestStakedAssertion,uint64 index,bool isStaked,address withdrawalAddress)",
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
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amountStaked)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.latestStakedAssertion,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.index)
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.isStaked,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.withdrawalAddress,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Staker {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amountStaked,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.latestStakedAssertion,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.index)
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.isStaked,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.withdrawalAddress,
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
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amountStaked,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.latestStakedAssertion,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.index,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.isStaked,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.withdrawalAddress,
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
    /**Creates a new wrapper around an on-chain [`IRollupCore`](self) contract instance.

See the [wrapper's documentation](`IRollupCoreInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IRollupCoreInstance<T, P, N> {
        IRollupCoreInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IRollupCore`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IRollupCore`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IRollupCoreInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IRollupCoreInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IRollupCoreInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IRollupCoreInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IRollupCore`](self) contract instance.

See the [wrapper's documentation](`IRollupCoreInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IRollupCoreInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IRollupCoreInstance<T, P, N> {
            IRollupCoreInstance {
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
    > IRollupCoreInstance<T, P, N> {
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
    > IRollupCoreInstance<T, P, N> {
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
library IRollupCore {
    struct Staker {
        uint256 amountStaked;
        bytes32 latestStakedAssertion;
        uint64 index;
        bool isStaked;
        address withdrawalAddress;
    }
}

interface IRollupUser {
    type AssertionStatus is uint8;
    type MachineStatus is uint8;
    struct AssertionInputs {
        BeforeStateData beforeStateData;
        AssertionState beforeState;
        AssertionState afterState;
    }
    struct AssertionNode {
        uint64 firstChildBlock;
        uint64 secondChildBlock;
        uint64 createdAtBlock;
        bool isFirstChild;
        AssertionStatus status;
        bytes32 configHash;
    }
    struct AssertionState {
        GlobalState globalState;
        MachineStatus machineStatus;
        bytes32 endHistoryRoot;
    }
    struct BeforeStateData {
        bytes32 prevPrevAssertionHash;
        bytes32 sequencerBatchAcc;
        ConfigData configData;
    }
    struct ConfigData {
        bytes32 wasmModuleRoot;
        uint256 requiredStake;
        address challengeManager;
        uint64 confirmPeriodBlocks;
        uint64 nextInboxPosition;
    }
    struct GlobalState {
        bytes32[2] bytes32Vals;
        uint64[2] u64Vals;
    }

    event AssertionConfirmed(bytes32 indexed assertionHash, bytes32 blockHash, bytes32 sendRoot);
    event AssertionCreated(bytes32 indexed assertionHash, bytes32 indexed parentAssertionHash, AssertionInputs assertion, bytes32 afterInboxBatchAcc, uint256 inboxMaxCount, bytes32 wasmModuleRoot, uint256 requiredStake, address challengeManager, uint64 confirmPeriodBlocks);
    event RollupChallengeStarted(uint64 indexed challengeIndex, address asserter, address challenger, uint64 challengedAssertion);
    event RollupInitialized(bytes32 machineHash, uint256 chainId);
    event UserStakeUpdated(address indexed user, address indexed withdrawalAddress, uint256 initialBalance, uint256 finalBalance);
    event UserWithdrawableFundsUpdated(address indexed user, uint256 initialBalance, uint256 finalBalance);

    function addToDeposit(address stakerAddress, address expectedWithdrawalAddress, uint256 tokenAmount) external;
    function amountStaked(address staker) external view returns (uint256);
    function baseStake() external view returns (uint256);
    function bridge() external view returns (address);
    function chainId() external view returns (uint256);
    function challengeManager() external view returns (address);
    function confirmAssertion(bytes32 assertionHash, bytes32 prevAssertionHash, AssertionState memory confirmState, bytes32 winningEdgeId, ConfigData memory prevConfig, bytes32 inboxAcc) external;
    function confirmPeriodBlocks() external view returns (uint64);
    function genesisAssertionHash() external pure returns (bytes32);
    function getAssertion(bytes32 assertionHash) external view returns (AssertionNode memory);
    function getAssertionCreationBlockForLogLookup(bytes32 assertionHash) external view returns (uint256);
    function getFirstChildCreationBlock(bytes32 assertionHash) external view returns (uint64);
    function getSecondChildCreationBlock(bytes32 assertionHash) external view returns (uint64);
    function getStaker(address staker) external view returns (IRollupCore.Staker memory);
    function getStakerAddress(uint64 stakerNum) external view returns (address);
    function getValidators() external view returns (address[] memory);
    function initialize(address stakeToken) external view;
    function isFirstChild(bytes32 assertionHash) external view returns (bool);
    function isPending(bytes32 assertionHash) external view returns (bool);
    function isStaked(address staker) external view returns (bool);
    function isValidator(address) external view returns (bool);
    function latestConfirmed() external view returns (bytes32);
    function latestStakedAssertion(address staker) external view returns (bytes32);
    function loserStakeEscrow() external view returns (address);
    function minimumAssertionPeriod() external view returns (uint256);
    function newStake(uint256 tokenAmount, address withdrawalAddress) external;
    function newStakeOnNewAssertion(uint256 tokenAmount, AssertionInputs memory assertion, bytes32 expectedAssertionHash, address withdrawalAddress) external;
    function newStakeOnNewAssertion(uint256 tokenAmount, AssertionInputs memory assertion, bytes32 expectedAssertionHash) external;
    function outbox() external view returns (address);
    function owner() external view returns (address);
    function reduceDeposit(uint256 target) external;
    function removeWhitelistAfterFork() external;
    function removeWhitelistAfterValidatorAfk() external;
    function returnOldDeposit() external;
    function returnOldDepositFor(address stakerAddress) external;
    function rollupEventInbox() external view returns (address);
    function sequencerInbox() external view returns (address);
    function stakeOnNewAssertion(AssertionInputs memory assertion, bytes32 expectedAssertionHash) external;
    function stakeToken() external view returns (address);
    function stakerCount() external view returns (uint64);
    function validateAssertionHash(bytes32 assertionHash, AssertionState memory state, bytes32 prevAssertionHash, bytes32 inboxAcc) external view;
    function validateConfig(bytes32 assertionHash, ConfigData memory configData) external view;
    function validatorAfkBlocks() external view returns (uint64);
    function validatorWhitelistDisabled() external view returns (bool);
    function wasmModuleRoot() external view returns (bytes32);
    function withdrawStakerFunds() external returns (uint256);
    function withdrawableFunds(address owner) external view returns (uint256);
    function withdrawalAddress(address staker) external view returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "addToDeposit",
    "inputs": [
      {
        "name": "stakerAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "expectedWithdrawalAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "amountStaked",
    "inputs": [
      {
        "name": "staker",
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
    "name": "baseStake",
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
    "name": "bridge",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IBridge"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "chainId",
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
    "name": "challengeManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IEdgeChallengeManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "confirmAssertion",
    "inputs": [
      {
        "name": "assertionHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "prevAssertionHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "confirmState",
        "type": "tuple",
        "internalType": "struct AssertionState",
        "components": [
          {
            "name": "globalState",
            "type": "tuple",
            "internalType": "struct GlobalState",
            "components": [
              {
                "name": "bytes32Vals",
                "type": "bytes32[2]",
                "internalType": "bytes32[2]"
              },
              {
                "name": "u64Vals",
                "type": "uint64[2]",
                "internalType": "uint64[2]"
              }
            ]
          },
          {
            "name": "machineStatus",
            "type": "uint8",
            "internalType": "enum MachineStatus"
          },
          {
            "name": "endHistoryRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "winningEdgeId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "prevConfig",
        "type": "tuple",
        "internalType": "struct ConfigData",
        "components": [
          {
            "name": "wasmModuleRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "requiredStake",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "challengeManager",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "confirmPeriodBlocks",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "nextInboxPosition",
            "type": "uint64",
            "internalType": "uint64"
          }
        ]
      },
      {
        "name": "inboxAcc",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "confirmPeriodBlocks",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "genesisAssertionHash",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "getAssertion",
    "inputs": [
      {
        "name": "assertionHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct AssertionNode",
        "components": [
          {
            "name": "firstChildBlock",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "secondChildBlock",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "createdAtBlock",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "isFirstChild",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "status",
            "type": "uint8",
            "internalType": "enum AssertionStatus"
          },
          {
            "name": "configHash",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAssertionCreationBlockForLogLookup",
    "inputs": [
      {
        "name": "assertionHash",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "getFirstChildCreationBlock",
    "inputs": [
      {
        "name": "assertionHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getSecondChildCreationBlock",
    "inputs": [
      {
        "name": "assertionHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStaker",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IRollupCore.Staker",
        "components": [
          {
            "name": "amountStaked",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "latestStakedAssertion",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "index",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "isStaked",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "withdrawalAddress",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStakerAddress",
    "inputs": [
      {
        "name": "stakerNum",
        "type": "uint64",
        "internalType": "uint64"
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
    "name": "getValidators",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "stakeToken",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isFirstChild",
    "inputs": [
      {
        "name": "assertionHash",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "isPending",
    "inputs": [
      {
        "name": "assertionHash",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "isStaked",
    "inputs": [
      {
        "name": "staker",
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
    "name": "isValidator",
    "inputs": [
      {
        "name": "",
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
    "name": "latestConfirmed",
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
    "name": "latestStakedAssertion",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
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
    "name": "loserStakeEscrow",
    "inputs": [],
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
    "name": "minimumAssertionPeriod",
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
    "name": "newStake",
    "inputs": [
      {
        "name": "tokenAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "withdrawalAddress",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "newStakeOnNewAssertion",
    "inputs": [
      {
        "name": "tokenAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "assertion",
        "type": "tuple",
        "internalType": "struct AssertionInputs",
        "components": [
          {
            "name": "beforeStateData",
            "type": "tuple",
            "internalType": "struct BeforeStateData",
            "components": [
              {
                "name": "prevPrevAssertionHash",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "sequencerBatchAcc",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "configData",
                "type": "tuple",
                "internalType": "struct ConfigData",
                "components": [
                  {
                    "name": "wasmModuleRoot",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "requiredStake",
                    "type": "uint256",
                    "internalType": "uint256"
                  },
                  {
                    "name": "challengeManager",
                    "type": "address",
                    "internalType": "address"
                  },
                  {
                    "name": "confirmPeriodBlocks",
                    "type": "uint64",
                    "internalType": "uint64"
                  },
                  {
                    "name": "nextInboxPosition",
                    "type": "uint64",
                    "internalType": "uint64"
                  }
                ]
              }
            ]
          },
          {
            "name": "beforeState",
            "type": "tuple",
            "internalType": "struct AssertionState",
            "components": [
              {
                "name": "globalState",
                "type": "tuple",
                "internalType": "struct GlobalState",
                "components": [
                  {
                    "name": "bytes32Vals",
                    "type": "bytes32[2]",
                    "internalType": "bytes32[2]"
                  },
                  {
                    "name": "u64Vals",
                    "type": "uint64[2]",
                    "internalType": "uint64[2]"
                  }
                ]
              },
              {
                "name": "machineStatus",
                "type": "uint8",
                "internalType": "enum MachineStatus"
              },
              {
                "name": "endHistoryRoot",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "afterState",
            "type": "tuple",
            "internalType": "struct AssertionState",
            "components": [
              {
                "name": "globalState",
                "type": "tuple",
                "internalType": "struct GlobalState",
                "components": [
                  {
                    "name": "bytes32Vals",
                    "type": "bytes32[2]",
                    "internalType": "bytes32[2]"
                  },
                  {
                    "name": "u64Vals",
                    "type": "uint64[2]",
                    "internalType": "uint64[2]"
                  }
                ]
              },
              {
                "name": "machineStatus",
                "type": "uint8",
                "internalType": "enum MachineStatus"
              },
              {
                "name": "endHistoryRoot",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          }
        ]
      },
      {
        "name": "expectedAssertionHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "withdrawalAddress",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "newStakeOnNewAssertion",
    "inputs": [
      {
        "name": "tokenAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "assertion",
        "type": "tuple",
        "internalType": "struct AssertionInputs",
        "components": [
          {
            "name": "beforeStateData",
            "type": "tuple",
            "internalType": "struct BeforeStateData",
            "components": [
              {
                "name": "prevPrevAssertionHash",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "sequencerBatchAcc",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "configData",
                "type": "tuple",
                "internalType": "struct ConfigData",
                "components": [
                  {
                    "name": "wasmModuleRoot",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "requiredStake",
                    "type": "uint256",
                    "internalType": "uint256"
                  },
                  {
                    "name": "challengeManager",
                    "type": "address",
                    "internalType": "address"
                  },
                  {
                    "name": "confirmPeriodBlocks",
                    "type": "uint64",
                    "internalType": "uint64"
                  },
                  {
                    "name": "nextInboxPosition",
                    "type": "uint64",
                    "internalType": "uint64"
                  }
                ]
              }
            ]
          },
          {
            "name": "beforeState",
            "type": "tuple",
            "internalType": "struct AssertionState",
            "components": [
              {
                "name": "globalState",
                "type": "tuple",
                "internalType": "struct GlobalState",
                "components": [
                  {
                    "name": "bytes32Vals",
                    "type": "bytes32[2]",
                    "internalType": "bytes32[2]"
                  },
                  {
                    "name": "u64Vals",
                    "type": "uint64[2]",
                    "internalType": "uint64[2]"
                  }
                ]
              },
              {
                "name": "machineStatus",
                "type": "uint8",
                "internalType": "enum MachineStatus"
              },
              {
                "name": "endHistoryRoot",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "afterState",
            "type": "tuple",
            "internalType": "struct AssertionState",
            "components": [
              {
                "name": "globalState",
                "type": "tuple",
                "internalType": "struct GlobalState",
                "components": [
                  {
                    "name": "bytes32Vals",
                    "type": "bytes32[2]",
                    "internalType": "bytes32[2]"
                  },
                  {
                    "name": "u64Vals",
                    "type": "uint64[2]",
                    "internalType": "uint64[2]"
                  }
                ]
              },
              {
                "name": "machineStatus",
                "type": "uint8",
                "internalType": "enum MachineStatus"
              },
              {
                "name": "endHistoryRoot",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          }
        ]
      },
      {
        "name": "expectedAssertionHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "outbox",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IOutbox"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
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
    "name": "reduceDeposit",
    "inputs": [
      {
        "name": "target",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removeWhitelistAfterFork",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removeWhitelistAfterValidatorAfk",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "returnOldDeposit",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "returnOldDepositFor",
    "inputs": [
      {
        "name": "stakerAddress",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "rollupEventInbox",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IRollupEventInbox"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "sequencerInbox",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ISequencerInbox"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "stakeOnNewAssertion",
    "inputs": [
      {
        "name": "assertion",
        "type": "tuple",
        "internalType": "struct AssertionInputs",
        "components": [
          {
            "name": "beforeStateData",
            "type": "tuple",
            "internalType": "struct BeforeStateData",
            "components": [
              {
                "name": "prevPrevAssertionHash",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "sequencerBatchAcc",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "configData",
                "type": "tuple",
                "internalType": "struct ConfigData",
                "components": [
                  {
                    "name": "wasmModuleRoot",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "requiredStake",
                    "type": "uint256",
                    "internalType": "uint256"
                  },
                  {
                    "name": "challengeManager",
                    "type": "address",
                    "internalType": "address"
                  },
                  {
                    "name": "confirmPeriodBlocks",
                    "type": "uint64",
                    "internalType": "uint64"
                  },
                  {
                    "name": "nextInboxPosition",
                    "type": "uint64",
                    "internalType": "uint64"
                  }
                ]
              }
            ]
          },
          {
            "name": "beforeState",
            "type": "tuple",
            "internalType": "struct AssertionState",
            "components": [
              {
                "name": "globalState",
                "type": "tuple",
                "internalType": "struct GlobalState",
                "components": [
                  {
                    "name": "bytes32Vals",
                    "type": "bytes32[2]",
                    "internalType": "bytes32[2]"
                  },
                  {
                    "name": "u64Vals",
                    "type": "uint64[2]",
                    "internalType": "uint64[2]"
                  }
                ]
              },
              {
                "name": "machineStatus",
                "type": "uint8",
                "internalType": "enum MachineStatus"
              },
              {
                "name": "endHistoryRoot",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "afterState",
            "type": "tuple",
            "internalType": "struct AssertionState",
            "components": [
              {
                "name": "globalState",
                "type": "tuple",
                "internalType": "struct GlobalState",
                "components": [
                  {
                    "name": "bytes32Vals",
                    "type": "bytes32[2]",
                    "internalType": "bytes32[2]"
                  },
                  {
                    "name": "u64Vals",
                    "type": "uint64[2]",
                    "internalType": "uint64[2]"
                  }
                ]
              },
              {
                "name": "machineStatus",
                "type": "uint8",
                "internalType": "enum MachineStatus"
              },
              {
                "name": "endHistoryRoot",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          }
        ]
      },
      {
        "name": "expectedAssertionHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "stakeToken",
    "inputs": [],
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
    "name": "stakerCount",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "validateAssertionHash",
    "inputs": [
      {
        "name": "assertionHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "state",
        "type": "tuple",
        "internalType": "struct AssertionState",
        "components": [
          {
            "name": "globalState",
            "type": "tuple",
            "internalType": "struct GlobalState",
            "components": [
              {
                "name": "bytes32Vals",
                "type": "bytes32[2]",
                "internalType": "bytes32[2]"
              },
              {
                "name": "u64Vals",
                "type": "uint64[2]",
                "internalType": "uint64[2]"
              }
            ]
          },
          {
            "name": "machineStatus",
            "type": "uint8",
            "internalType": "enum MachineStatus"
          },
          {
            "name": "endHistoryRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "prevAssertionHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "inboxAcc",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "validateConfig",
    "inputs": [
      {
        "name": "assertionHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "configData",
        "type": "tuple",
        "internalType": "struct ConfigData",
        "components": [
          {
            "name": "wasmModuleRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "requiredStake",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "challengeManager",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "confirmPeriodBlocks",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "nextInboxPosition",
            "type": "uint64",
            "internalType": "uint64"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "validatorAfkBlocks",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "validatorWhitelistDisabled",
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
    "name": "wasmModuleRoot",
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
    "name": "withdrawStakerFunds",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdrawableFunds",
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
    "name": "withdrawalAddress",
    "inputs": [
      {
        "name": "staker",
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
    "type": "event",
    "name": "AssertionConfirmed",
    "inputs": [
      {
        "name": "assertionHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "blockHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "sendRoot",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "AssertionCreated",
    "inputs": [
      {
        "name": "assertionHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "parentAssertionHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "assertion",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct AssertionInputs",
        "components": [
          {
            "name": "beforeStateData",
            "type": "tuple",
            "internalType": "struct BeforeStateData",
            "components": [
              {
                "name": "prevPrevAssertionHash",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "sequencerBatchAcc",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "configData",
                "type": "tuple",
                "internalType": "struct ConfigData",
                "components": [
                  {
                    "name": "wasmModuleRoot",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "requiredStake",
                    "type": "uint256",
                    "internalType": "uint256"
                  },
                  {
                    "name": "challengeManager",
                    "type": "address",
                    "internalType": "address"
                  },
                  {
                    "name": "confirmPeriodBlocks",
                    "type": "uint64",
                    "internalType": "uint64"
                  },
                  {
                    "name": "nextInboxPosition",
                    "type": "uint64",
                    "internalType": "uint64"
                  }
                ]
              }
            ]
          },
          {
            "name": "beforeState",
            "type": "tuple",
            "internalType": "struct AssertionState",
            "components": [
              {
                "name": "globalState",
                "type": "tuple",
                "internalType": "struct GlobalState",
                "components": [
                  {
                    "name": "bytes32Vals",
                    "type": "bytes32[2]",
                    "internalType": "bytes32[2]"
                  },
                  {
                    "name": "u64Vals",
                    "type": "uint64[2]",
                    "internalType": "uint64[2]"
                  }
                ]
              },
              {
                "name": "machineStatus",
                "type": "uint8",
                "internalType": "enum MachineStatus"
              },
              {
                "name": "endHistoryRoot",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "afterState",
            "type": "tuple",
            "internalType": "struct AssertionState",
            "components": [
              {
                "name": "globalState",
                "type": "tuple",
                "internalType": "struct GlobalState",
                "components": [
                  {
                    "name": "bytes32Vals",
                    "type": "bytes32[2]",
                    "internalType": "bytes32[2]"
                  },
                  {
                    "name": "u64Vals",
                    "type": "uint64[2]",
                    "internalType": "uint64[2]"
                  }
                ]
              },
              {
                "name": "machineStatus",
                "type": "uint8",
                "internalType": "enum MachineStatus"
              },
              {
                "name": "endHistoryRoot",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          }
        ]
      },
      {
        "name": "afterInboxBatchAcc",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "inboxMaxCount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "wasmModuleRoot",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "requiredStake",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "challengeManager",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "confirmPeriodBlocks",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RollupChallengeStarted",
    "inputs": [
      {
        "name": "challengeIndex",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "asserter",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "challenger",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "challengedAssertion",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RollupInitialized",
    "inputs": [
      {
        "name": "machineHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "chainId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UserStakeUpdated",
    "inputs": [
      {
        "name": "user",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "withdrawalAddress",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "initialBalance",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "finalBalance",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UserWithdrawableFundsUpdated",
    "inputs": [
      {
        "name": "user",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "initialBalance",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "finalBalance",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
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
pub mod IRollupUser {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AssertionStatus(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<AssertionStatus> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl AssertionStatus {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for AssertionStatus {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AssertionStatus {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MachineStatus(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<MachineStatus> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl MachineStatus {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for MachineStatus {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for MachineStatus {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    /**```solidity
struct AssertionInputs { BeforeStateData beforeStateData; AssertionState beforeState; AssertionState afterState; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AssertionInputs {
        pub beforeStateData: <BeforeStateData as alloy::sol_types::SolType>::RustType,
        pub beforeState: <AssertionState as alloy::sol_types::SolType>::RustType,
        pub afterState: <AssertionState as alloy::sol_types::SolType>::RustType,
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
        type UnderlyingSolTuple<'a> = (BeforeStateData, AssertionState, AssertionState);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <BeforeStateData as alloy::sol_types::SolType>::RustType,
            <AssertionState as alloy::sol_types::SolType>::RustType,
            <AssertionState as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<AssertionInputs> for UnderlyingRustTuple<'_> {
            fn from(value: AssertionInputs) -> Self {
                (value.beforeStateData, value.beforeState, value.afterState)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AssertionInputs {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    beforeStateData: tuple.0,
                    beforeState: tuple.1,
                    afterState: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AssertionInputs {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AssertionInputs {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <BeforeStateData as alloy_sol_types::SolType>::tokenize(
                        &self.beforeStateData,
                    ),
                    <AssertionState as alloy_sol_types::SolType>::tokenize(
                        &self.beforeState,
                    ),
                    <AssertionState as alloy_sol_types::SolType>::tokenize(
                        &self.afterState,
                    ),
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
        impl alloy_sol_types::SolType for AssertionInputs {
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
        impl alloy_sol_types::SolStruct for AssertionInputs {
            const NAME: &'static str = "AssertionInputs";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AssertionInputs(BeforeStateData beforeStateData,AssertionState beforeState,AssertionState afterState)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(3);
                components
                    .push(
                        <BeforeStateData as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BeforeStateData as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <AssertionState as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <AssertionState as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <AssertionState as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <AssertionState as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <BeforeStateData as alloy_sol_types::SolType>::eip712_data_word(
                            &self.beforeStateData,
                        )
                        .0,
                    <AssertionState as alloy_sol_types::SolType>::eip712_data_word(
                            &self.beforeState,
                        )
                        .0,
                    <AssertionState as alloy_sol_types::SolType>::eip712_data_word(
                            &self.afterState,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AssertionInputs {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <BeforeStateData as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.beforeStateData,
                    )
                    + <AssertionState as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.beforeState,
                    )
                    + <AssertionState as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.afterState,
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
                <BeforeStateData as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.beforeStateData,
                    out,
                );
                <AssertionState as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.beforeState,
                    out,
                );
                <AssertionState as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.afterState,
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
    /**```solidity
struct AssertionNode { uint64 firstChildBlock; uint64 secondChildBlock; uint64 createdAtBlock; bool isFirstChild; AssertionStatus status; bytes32 configHash; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AssertionNode {
        pub firstChildBlock: u64,
        pub secondChildBlock: u64,
        pub createdAtBlock: u64,
        pub isFirstChild: bool,
        pub status: <AssertionStatus as alloy::sol_types::SolType>::RustType,
        pub configHash: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Bool,
            AssertionStatus,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
            u64,
            u64,
            bool,
            <AssertionStatus as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<AssertionNode> for UnderlyingRustTuple<'_> {
            fn from(value: AssertionNode) -> Self {
                (
                    value.firstChildBlock,
                    value.secondChildBlock,
                    value.createdAtBlock,
                    value.isFirstChild,
                    value.status,
                    value.configHash,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AssertionNode {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    firstChildBlock: tuple.0,
                    secondChildBlock: tuple.1,
                    createdAtBlock: tuple.2,
                    isFirstChild: tuple.3,
                    status: tuple.4,
                    configHash: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AssertionNode {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AssertionNode {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.firstChildBlock),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.secondChildBlock),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.createdAtBlock),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isFirstChild,
                    ),
                    <AssertionStatus as alloy_sol_types::SolType>::tokenize(
                        &self.status,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.configHash),
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
        impl alloy_sol_types::SolType for AssertionNode {
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
        impl alloy_sol_types::SolStruct for AssertionNode {
            const NAME: &'static str = "AssertionNode";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AssertionNode(uint64 firstChildBlock,uint64 secondChildBlock,uint64 createdAtBlock,bool isFirstChild,uint8 status,bytes32 configHash)",
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
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.firstChildBlock,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.secondChildBlock,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.createdAtBlock,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.isFirstChild,
                        )
                        .0,
                    <AssertionStatus as alloy_sol_types::SolType>::eip712_data_word(
                            &self.status,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.configHash)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AssertionNode {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.firstChildBlock,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.secondChildBlock,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.createdAtBlock,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.isFirstChild,
                    )
                    + <AssertionStatus as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.status,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.configHash,
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
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.firstChildBlock,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.secondChildBlock,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.createdAtBlock,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.isFirstChild,
                    out,
                );
                <AssertionStatus as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.status,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.configHash,
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
    /**```solidity
struct AssertionState { GlobalState globalState; MachineStatus machineStatus; bytes32 endHistoryRoot; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AssertionState {
        pub globalState: <GlobalState as alloy::sol_types::SolType>::RustType,
        pub machineStatus: <MachineStatus as alloy::sol_types::SolType>::RustType,
        pub endHistoryRoot: alloy::sol_types::private::FixedBytes<32>,
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
            GlobalState,
            MachineStatus,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <GlobalState as alloy::sol_types::SolType>::RustType,
            <MachineStatus as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<AssertionState> for UnderlyingRustTuple<'_> {
            fn from(value: AssertionState) -> Self {
                (value.globalState, value.machineStatus, value.endHistoryRoot)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AssertionState {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    globalState: tuple.0,
                    machineStatus: tuple.1,
                    endHistoryRoot: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AssertionState {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AssertionState {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <GlobalState as alloy_sol_types::SolType>::tokenize(
                        &self.globalState,
                    ),
                    <MachineStatus as alloy_sol_types::SolType>::tokenize(
                        &self.machineStatus,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.endHistoryRoot),
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
        impl alloy_sol_types::SolType for AssertionState {
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
        impl alloy_sol_types::SolStruct for AssertionState {
            const NAME: &'static str = "AssertionState";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AssertionState(GlobalState globalState,uint8 machineStatus,bytes32 endHistoryRoot)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <GlobalState as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <GlobalState as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <GlobalState as alloy_sol_types::SolType>::eip712_data_word(
                            &self.globalState,
                        )
                        .0,
                    <MachineStatus as alloy_sol_types::SolType>::eip712_data_word(
                            &self.machineStatus,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.endHistoryRoot,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AssertionState {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <GlobalState as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.globalState,
                    )
                    + <MachineStatus as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.machineStatus,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.endHistoryRoot,
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
                <GlobalState as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.globalState,
                    out,
                );
                <MachineStatus as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.machineStatus,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.endHistoryRoot,
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
    /**```solidity
struct BeforeStateData { bytes32 prevPrevAssertionHash; bytes32 sequencerBatchAcc; ConfigData configData; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BeforeStateData {
        pub prevPrevAssertionHash: alloy::sol_types::private::FixedBytes<32>,
        pub sequencerBatchAcc: alloy::sol_types::private::FixedBytes<32>,
        pub configData: <ConfigData as alloy::sol_types::SolType>::RustType,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            ConfigData,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            <ConfigData as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<BeforeStateData> for UnderlyingRustTuple<'_> {
            fn from(value: BeforeStateData) -> Self {
                (value.prevPrevAssertionHash, value.sequencerBatchAcc, value.configData)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BeforeStateData {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    prevPrevAssertionHash: tuple.0,
                    sequencerBatchAcc: tuple.1,
                    configData: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BeforeStateData {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BeforeStateData {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.prevPrevAssertionHash,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.sequencerBatchAcc),
                    <ConfigData as alloy_sol_types::SolType>::tokenize(&self.configData),
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
        impl alloy_sol_types::SolType for BeforeStateData {
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
        impl alloy_sol_types::SolStruct for BeforeStateData {
            const NAME: &'static str = "BeforeStateData";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BeforeStateData(bytes32 prevPrevAssertionHash,bytes32 sequencerBatchAcc,ConfigData configData)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <ConfigData as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <ConfigData as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.prevPrevAssertionHash,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sequencerBatchAcc,
                        )
                        .0,
                    <ConfigData as alloy_sol_types::SolType>::eip712_data_word(
                            &self.configData,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BeforeStateData {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.prevPrevAssertionHash,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sequencerBatchAcc,
                    )
                    + <ConfigData as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.configData,
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
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.prevPrevAssertionHash,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sequencerBatchAcc,
                    out,
                );
                <ConfigData as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.configData,
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
    /**```solidity
struct ConfigData { bytes32 wasmModuleRoot; uint256 requiredStake; address challengeManager; uint64 confirmPeriodBlocks; uint64 nextInboxPosition; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ConfigData {
        pub wasmModuleRoot: alloy::sol_types::private::FixedBytes<32>,
        pub requiredStake: alloy::sol_types::private::primitives::aliases::U256,
        pub challengeManager: alloy::sol_types::private::Address,
        pub confirmPeriodBlocks: u64,
        pub nextInboxPosition: u64,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
            u64,
            u64,
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
        impl ::core::convert::From<ConfigData> for UnderlyingRustTuple<'_> {
            fn from(value: ConfigData) -> Self {
                (
                    value.wasmModuleRoot,
                    value.requiredStake,
                    value.challengeManager,
                    value.confirmPeriodBlocks,
                    value.nextInboxPosition,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ConfigData {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    wasmModuleRoot: tuple.0,
                    requiredStake: tuple.1,
                    challengeManager: tuple.2,
                    confirmPeriodBlocks: tuple.3,
                    nextInboxPosition: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ConfigData {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ConfigData {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.wasmModuleRoot),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.requiredStake),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.challengeManager,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.confirmPeriodBlocks),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.nextInboxPosition),
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
        impl alloy_sol_types::SolType for ConfigData {
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
        impl alloy_sol_types::SolStruct for ConfigData {
            const NAME: &'static str = "ConfigData";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ConfigData(bytes32 wasmModuleRoot,uint256 requiredStake,address challengeManager,uint64 confirmPeriodBlocks,uint64 nextInboxPosition)",
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.wasmModuleRoot,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.requiredStake)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.challengeManager,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.confirmPeriodBlocks,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nextInboxPosition,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ConfigData {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.wasmModuleRoot,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requiredStake,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.challengeManager,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.confirmPeriodBlocks,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nextInboxPosition,
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
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.wasmModuleRoot,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requiredStake,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.challengeManager,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.confirmPeriodBlocks,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nextInboxPosition,
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
    /**```solidity
struct GlobalState { bytes32[2] bytes32Vals; uint64[2] u64Vals; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GlobalState {
        pub bytes32Vals: [alloy::sol_types::private::FixedBytes<32>; 2usize],
        pub u64Vals: [u64; 2usize],
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
            alloy::sol_types::sol_data::FixedArray<
                alloy::sol_types::sol_data::FixedBytes<32>,
                2usize,
            >,
            alloy::sol_types::sol_data::FixedArray<
                alloy::sol_types::sol_data::Uint<64>,
                2usize,
            >,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            [alloy::sol_types::private::FixedBytes<32>; 2usize],
            [u64; 2usize],
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
        impl ::core::convert::From<GlobalState> for UnderlyingRustTuple<'_> {
            fn from(value: GlobalState) -> Self {
                (value.bytes32Vals, value.u64Vals)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GlobalState {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    bytes32Vals: tuple.0,
                    u64Vals: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for GlobalState {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for GlobalState {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.bytes32Vals),
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<64>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.u64Vals),
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
        impl alloy_sol_types::SolType for GlobalState {
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
        impl alloy_sol_types::SolStruct for GlobalState {
            const NAME: &'static str = "GlobalState";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "GlobalState(bytes32[2] bytes32Vals,uint64[2] u64Vals)",
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
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.bytes32Vals)
                        .0,
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<64>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.u64Vals)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for GlobalState {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.bytes32Vals,
                    )
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<64>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.u64Vals,
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
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.bytes32Vals,
                    out,
                );
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<64>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.u64Vals,
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
    /**Event with signature `AssertionConfirmed(bytes32,bytes32,bytes32)` and selector `0xfc42829b29c259a7370ab56c8f69fce23b5f351a9ce151da453281993ec0090c`.
```solidity
event AssertionConfirmed(bytes32 indexed assertionHash, bytes32 blockHash, bytes32 sendRoot);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AssertionConfirmed {
        #[allow(missing_docs)]
        pub assertionHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub blockHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub sendRoot: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for AssertionConfirmed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "AssertionConfirmed(bytes32,bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                252u8,
                66u8,
                130u8,
                155u8,
                41u8,
                194u8,
                89u8,
                167u8,
                55u8,
                10u8,
                181u8,
                108u8,
                143u8,
                105u8,
                252u8,
                226u8,
                59u8,
                95u8,
                53u8,
                26u8,
                156u8,
                225u8,
                81u8,
                218u8,
                69u8,
                50u8,
                129u8,
                153u8,
                62u8,
                192u8,
                9u8,
                12u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    assertionHash: topics.1,
                    blockHash: data.0,
                    sendRoot: data.1,
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockHash),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.sendRoot),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.assertionHash.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.assertionHash);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for AssertionConfirmed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AssertionConfirmed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AssertionConfirmed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `AssertionCreated(bytes32,bytes32,((bytes32,bytes32,(bytes32,uint256,address,uint64,uint64)),((bytes32[2],uint64[2]),uint8,bytes32),((bytes32[2],uint64[2]),uint8,bytes32)),bytes32,uint256,bytes32,uint256,address,uint64)` and selector `0x901c3aee23cf4478825462caaab375c606ab83516060388344f0650340753630`.
```solidity
event AssertionCreated(bytes32 indexed assertionHash, bytes32 indexed parentAssertionHash, AssertionInputs assertion, bytes32 afterInboxBatchAcc, uint256 inboxMaxCount, bytes32 wasmModuleRoot, uint256 requiredStake, address challengeManager, uint64 confirmPeriodBlocks);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AssertionCreated {
        #[allow(missing_docs)]
        pub assertionHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub parentAssertionHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub assertion: <AssertionInputs as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub afterInboxBatchAcc: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub inboxMaxCount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub wasmModuleRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub requiredStake: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub challengeManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub confirmPeriodBlocks: u64,
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
        impl alloy_sol_types::SolEvent for AssertionCreated {
            type DataTuple<'a> = (
                AssertionInputs,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "AssertionCreated(bytes32,bytes32,((bytes32,bytes32,(bytes32,uint256,address,uint64,uint64)),((bytes32[2],uint64[2]),uint8,bytes32),((bytes32[2],uint64[2]),uint8,bytes32)),bytes32,uint256,bytes32,uint256,address,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                144u8,
                28u8,
                58u8,
                238u8,
                35u8,
                207u8,
                68u8,
                120u8,
                130u8,
                84u8,
                98u8,
                202u8,
                170u8,
                179u8,
                117u8,
                198u8,
                6u8,
                171u8,
                131u8,
                81u8,
                96u8,
                96u8,
                56u8,
                131u8,
                68u8,
                240u8,
                101u8,
                3u8,
                64u8,
                117u8,
                54u8,
                48u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    assertionHash: topics.1,
                    parentAssertionHash: topics.2,
                    assertion: data.0,
                    afterInboxBatchAcc: data.1,
                    inboxMaxCount: data.2,
                    wasmModuleRoot: data.3,
                    requiredStake: data.4,
                    challengeManager: data.5,
                    confirmPeriodBlocks: data.6,
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
                    <AssertionInputs as alloy_sol_types::SolType>::tokenize(
                        &self.assertion,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.afterInboxBatchAcc),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.inboxMaxCount),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.wasmModuleRoot),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.requiredStake),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.challengeManager,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.confirmPeriodBlocks),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.assertionHash.clone(),
                    self.parentAssertionHash.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.assertionHash);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.parentAssertionHash,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for AssertionCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AssertionCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AssertionCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RollupChallengeStarted(uint64,address,address,uint64)` and selector `0x6db7dc2f507647d135035469b27aa79cea90582779d084a7821d6cd092cbd873`.
```solidity
event RollupChallengeStarted(uint64 indexed challengeIndex, address asserter, address challenger, uint64 challengedAssertion);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RollupChallengeStarted {
        #[allow(missing_docs)]
        pub challengeIndex: u64,
        #[allow(missing_docs)]
        pub asserter: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub challenger: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub challengedAssertion: u64,
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
        impl alloy_sol_types::SolEvent for RollupChallengeStarted {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            const SIGNATURE: &'static str = "RollupChallengeStarted(uint64,address,address,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                109u8,
                183u8,
                220u8,
                47u8,
                80u8,
                118u8,
                71u8,
                209u8,
                53u8,
                3u8,
                84u8,
                105u8,
                178u8,
                122u8,
                167u8,
                156u8,
                234u8,
                144u8,
                88u8,
                39u8,
                121u8,
                208u8,
                132u8,
                167u8,
                130u8,
                29u8,
                108u8,
                208u8,
                146u8,
                203u8,
                216u8,
                115u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    challengeIndex: topics.1,
                    asserter: data.0,
                    challenger: data.1,
                    challengedAssertion: data.2,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asserter,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.challenger,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.challengedAssertion),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.challengeIndex.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.challengeIndex);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RollupChallengeStarted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RollupChallengeStarted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RollupChallengeStarted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RollupInitialized(bytes32,uint256)` and selector `0xfc1b83c11d99d08a938e0b82a0bd45f822f71ff5abf23f999c93c4533d752464`.
```solidity
event RollupInitialized(bytes32 machineHash, uint256 chainId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RollupInitialized {
        #[allow(missing_docs)]
        pub machineHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for RollupInitialized {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "RollupInitialized(bytes32,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                252u8,
                27u8,
                131u8,
                193u8,
                29u8,
                153u8,
                208u8,
                138u8,
                147u8,
                142u8,
                11u8,
                130u8,
                160u8,
                189u8,
                69u8,
                248u8,
                34u8,
                247u8,
                31u8,
                245u8,
                171u8,
                242u8,
                63u8,
                153u8,
                156u8,
                147u8,
                196u8,
                83u8,
                61u8,
                117u8,
                36u8,
                100u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    machineHash: data.0,
                    chainId: data.1,
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.machineHash),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
                )
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
        impl alloy_sol_types::private::IntoLogData for RollupInitialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RollupInitialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RollupInitialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `UserStakeUpdated(address,address,uint256,uint256)` and selector `0xd957cf2340073335d256f72a9ef89cf1a43c31143341a6a53575ef33e987beb8`.
```solidity
event UserStakeUpdated(address indexed user, address indexed withdrawalAddress, uint256 initialBalance, uint256 finalBalance);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UserStakeUpdated {
        #[allow(missing_docs)]
        pub user: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub withdrawalAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub initialBalance: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub finalBalance: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for UserStakeUpdated {
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
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "UserStakeUpdated(address,address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                217u8,
                87u8,
                207u8,
                35u8,
                64u8,
                7u8,
                51u8,
                53u8,
                210u8,
                86u8,
                247u8,
                42u8,
                158u8,
                248u8,
                156u8,
                241u8,
                164u8,
                60u8,
                49u8,
                20u8,
                51u8,
                65u8,
                166u8,
                165u8,
                53u8,
                117u8,
                239u8,
                51u8,
                233u8,
                135u8,
                190u8,
                184u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    user: topics.1,
                    withdrawalAddress: topics.2,
                    initialBalance: data.0,
                    finalBalance: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.initialBalance),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.finalBalance),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.user.clone(),
                    self.withdrawalAddress.clone(),
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
                    &self.user,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.withdrawalAddress,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UserStakeUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UserStakeUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UserStakeUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `UserWithdrawableFundsUpdated(address,uint256,uint256)` and selector `0xa740af14c56e4e04a617b1de1eb20de73270decbaaead14f142aabf3038e5ae2`.
```solidity
event UserWithdrawableFundsUpdated(address indexed user, uint256 initialBalance, uint256 finalBalance);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UserWithdrawableFundsUpdated {
        #[allow(missing_docs)]
        pub user: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub initialBalance: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub finalBalance: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for UserWithdrawableFundsUpdated {
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
            const SIGNATURE: &'static str = "UserWithdrawableFundsUpdated(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                167u8,
                64u8,
                175u8,
                20u8,
                197u8,
                110u8,
                78u8,
                4u8,
                166u8,
                23u8,
                177u8,
                222u8,
                30u8,
                178u8,
                13u8,
                231u8,
                50u8,
                112u8,
                222u8,
                203u8,
                170u8,
                234u8,
                209u8,
                79u8,
                20u8,
                42u8,
                171u8,
                243u8,
                3u8,
                142u8,
                90u8,
                226u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    user: topics.1,
                    initialBalance: data.0,
                    finalBalance: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.initialBalance),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.finalBalance),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.user.clone())
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
                    &self.user,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UserWithdrawableFundsUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UserWithdrawableFundsUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &UserWithdrawableFundsUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `addToDeposit(address,address,uint256)` and selector `0x685f5ecc`.
```solidity
function addToDeposit(address stakerAddress, address expectedWithdrawalAddress, uint256 tokenAmount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addToDepositCall {
        pub stakerAddress: alloy::sol_types::private::Address,
        pub expectedWithdrawalAddress: alloy::sol_types::private::Address,
        pub tokenAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`addToDeposit(address,address,uint256)`](addToDepositCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addToDepositReturn {}
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
            impl ::core::convert::From<addToDepositCall> for UnderlyingRustTuple<'_> {
                fn from(value: addToDepositCall) -> Self {
                    (
                        value.stakerAddress,
                        value.expectedWithdrawalAddress,
                        value.tokenAmount,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addToDepositCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        stakerAddress: tuple.0,
                        expectedWithdrawalAddress: tuple.1,
                        tokenAmount: tuple.2,
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
            impl ::core::convert::From<addToDepositReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addToDepositReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addToDepositReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addToDepositCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addToDepositReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addToDeposit(address,address,uint256)";
            const SELECTOR: [u8; 4] = [104u8, 95u8, 94u8, 204u8];
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
                        &self.stakerAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.expectedWithdrawalAddress,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenAmount),
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
    /**Function with signature `amountStaked(address)` and selector `0xef40a670`.
```solidity
function amountStaked(address staker) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct amountStakedCall {
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`amountStaked(address)`](amountStakedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct amountStakedReturn {
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
            impl ::core::convert::From<amountStakedCall> for UnderlyingRustTuple<'_> {
                fn from(value: amountStakedCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for amountStakedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
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
            impl ::core::convert::From<amountStakedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: amountStakedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for amountStakedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for amountStakedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = amountStakedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "amountStaked(address)";
            const SELECTOR: [u8; 4] = [239u8, 64u8, 166u8, 112u8];
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
                        &self.staker,
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
    /**Function with signature `baseStake()` and selector `0x76e7e23b`.
```solidity
function baseStake() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct baseStakeCall {}
    ///Container type for the return parameters of the [`baseStake()`](baseStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct baseStakeReturn {
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
            impl ::core::convert::From<baseStakeCall> for UnderlyingRustTuple<'_> {
                fn from(value: baseStakeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for baseStakeCall {
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
            impl ::core::convert::From<baseStakeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: baseStakeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for baseStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for baseStakeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = baseStakeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "baseStake()";
            const SELECTOR: [u8; 4] = [118u8, 231u8, 226u8, 59u8];
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
    /**Function with signature `bridge()` and selector `0xe78cea92`.
```solidity
function bridge() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bridgeCall {}
    ///Container type for the return parameters of the [`bridge()`](bridgeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bridgeReturn {
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
            impl ::core::convert::From<bridgeCall> for UnderlyingRustTuple<'_> {
                fn from(value: bridgeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bridgeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<bridgeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: bridgeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bridgeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bridgeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bridgeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "bridge()";
            const SELECTOR: [u8; 4] = [231u8, 140u8, 234u8, 146u8];
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
    /**Function with signature `chainId()` and selector `0x9a8a0592`.
```solidity
function chainId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct chainIdCall {}
    ///Container type for the return parameters of the [`chainId()`](chainIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct chainIdReturn {
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
            impl ::core::convert::From<chainIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: chainIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for chainIdCall {
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
            impl ::core::convert::From<chainIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: chainIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for chainIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for chainIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = chainIdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "chainId()";
            const SELECTOR: [u8; 4] = [154u8, 138u8, 5u8, 146u8];
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
    /**Function with signature `challengeManager()` and selector `0x023a96fe`.
```solidity
function challengeManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengeManagerCall {}
    ///Container type for the return parameters of the [`challengeManager()`](challengeManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengeManagerReturn {
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
            impl ::core::convert::From<challengeManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: challengeManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for challengeManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<challengeManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: challengeManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for challengeManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for challengeManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = challengeManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "challengeManager()";
            const SELECTOR: [u8; 4] = [2u8, 58u8, 150u8, 254u8];
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
    /**Function with signature `confirmAssertion(bytes32,bytes32,((bytes32[2],uint64[2]),uint8,bytes32),bytes32,(bytes32,uint256,address,uint64,uint64),bytes32)` and selector `0x10b98a35`.
```solidity
function confirmAssertion(bytes32 assertionHash, bytes32 prevAssertionHash, AssertionState memory confirmState, bytes32 winningEdgeId, ConfigData memory prevConfig, bytes32 inboxAcc) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmAssertionCall {
        pub assertionHash: alloy::sol_types::private::FixedBytes<32>,
        pub prevAssertionHash: alloy::sol_types::private::FixedBytes<32>,
        pub confirmState: <AssertionState as alloy::sol_types::SolType>::RustType,
        pub winningEdgeId: alloy::sol_types::private::FixedBytes<32>,
        pub prevConfig: <ConfigData as alloy::sol_types::SolType>::RustType,
        pub inboxAcc: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`confirmAssertion(bytes32,bytes32,((bytes32[2],uint64[2]),uint8,bytes32),bytes32,(bytes32,uint256,address,uint64,uint64),bytes32)`](confirmAssertionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmAssertionReturn {}
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                AssertionState,
                alloy::sol_types::sol_data::FixedBytes<32>,
                ConfigData,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
                <AssertionState as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
                <ConfigData as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<confirmAssertionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: confirmAssertionCall) -> Self {
                    (
                        value.assertionHash,
                        value.prevAssertionHash,
                        value.confirmState,
                        value.winningEdgeId,
                        value.prevConfig,
                        value.inboxAcc,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for confirmAssertionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        assertionHash: tuple.0,
                        prevAssertionHash: tuple.1,
                        confirmState: tuple.2,
                        winningEdgeId: tuple.3,
                        prevConfig: tuple.4,
                        inboxAcc: tuple.5,
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
            impl ::core::convert::From<confirmAssertionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: confirmAssertionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for confirmAssertionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for confirmAssertionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                AssertionState,
                alloy::sol_types::sol_data::FixedBytes<32>,
                ConfigData,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = confirmAssertionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "confirmAssertion(bytes32,bytes32,((bytes32[2],uint64[2]),uint8,bytes32),bytes32,(bytes32,uint256,address,uint64,uint64),bytes32)";
            const SELECTOR: [u8; 4] = [16u8, 185u8, 138u8, 53u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.assertionHash),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.prevAssertionHash),
                    <AssertionState as alloy_sol_types::SolType>::tokenize(
                        &self.confirmState,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.winningEdgeId),
                    <ConfigData as alloy_sol_types::SolType>::tokenize(&self.prevConfig),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.inboxAcc),
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
    /**Function with signature `confirmPeriodBlocks()` and selector `0x2e7acfa6`.
```solidity
function confirmPeriodBlocks() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmPeriodBlocksCall {}
    ///Container type for the return parameters of the [`confirmPeriodBlocks()`](confirmPeriodBlocksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmPeriodBlocksReturn {
        pub _0: u64,
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
            impl ::core::convert::From<confirmPeriodBlocksCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: confirmPeriodBlocksCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for confirmPeriodBlocksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<confirmPeriodBlocksReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: confirmPeriodBlocksReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for confirmPeriodBlocksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for confirmPeriodBlocksCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = confirmPeriodBlocksReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "confirmPeriodBlocks()";
            const SELECTOR: [u8; 4] = [46u8, 122u8, 207u8, 166u8];
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
    /**Function with signature `genesisAssertionHash()` and selector `0x353325e0`.
```solidity
function genesisAssertionHash() external pure returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct genesisAssertionHashCall {}
    ///Container type for the return parameters of the [`genesisAssertionHash()`](genesisAssertionHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct genesisAssertionHashReturn {
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
            impl ::core::convert::From<genesisAssertionHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: genesisAssertionHashCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for genesisAssertionHashCall {
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
            impl ::core::convert::From<genesisAssertionHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: genesisAssertionHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for genesisAssertionHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for genesisAssertionHashCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = genesisAssertionHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "genesisAssertionHash()";
            const SELECTOR: [u8; 4] = [53u8, 51u8, 37u8, 224u8];
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
    /**Function with signature `getAssertion(bytes32)` and selector `0x88302884`.
```solidity
function getAssertion(bytes32 assertionHash) external view returns (AssertionNode memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAssertionCall {
        pub assertionHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getAssertion(bytes32)`](getAssertionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAssertionReturn {
        pub _0: <AssertionNode as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAssertionCall> for UnderlyingRustTuple<'_> {
                fn from(value: getAssertionCall) -> Self {
                    (value.assertionHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAssertionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { assertionHash: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (AssertionNode,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <AssertionNode as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAssertionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getAssertionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAssertionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAssertionCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAssertionReturn;
            type ReturnTuple<'a> = (AssertionNode,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAssertion(bytes32)";
            const SELECTOR: [u8; 4] = [136u8, 48u8, 40u8, 132u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.assertionHash),
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
    /**Function with signature `getAssertionCreationBlockForLogLookup(bytes32)` and selector `0x13c56ca7`.
```solidity
function getAssertionCreationBlockForLogLookup(bytes32 assertionHash) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAssertionCreationBlockForLogLookupCall {
        pub assertionHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getAssertionCreationBlockForLogLookup(bytes32)`](getAssertionCreationBlockForLogLookupCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAssertionCreationBlockForLogLookupReturn {
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
            impl ::core::convert::From<getAssertionCreationBlockForLogLookupCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAssertionCreationBlockForLogLookupCall) -> Self {
                    (value.assertionHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAssertionCreationBlockForLogLookupCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { assertionHash: tuple.0 }
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
            impl ::core::convert::From<getAssertionCreationBlockForLogLookupReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAssertionCreationBlockForLogLookupReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAssertionCreationBlockForLogLookupReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAssertionCreationBlockForLogLookupCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAssertionCreationBlockForLogLookupReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAssertionCreationBlockForLogLookup(bytes32)";
            const SELECTOR: [u8; 4] = [19u8, 197u8, 108u8, 167u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.assertionHash),
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
    /**Function with signature `getFirstChildCreationBlock(bytes32)` and selector `0x11715585`.
```solidity
function getFirstChildCreationBlock(bytes32 assertionHash) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getFirstChildCreationBlockCall {
        pub assertionHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getFirstChildCreationBlock(bytes32)`](getFirstChildCreationBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getFirstChildCreationBlockReturn {
        pub _0: u64,
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
            impl ::core::convert::From<getFirstChildCreationBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getFirstChildCreationBlockCall) -> Self {
                    (value.assertionHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getFirstChildCreationBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { assertionHash: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getFirstChildCreationBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getFirstChildCreationBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getFirstChildCreationBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getFirstChildCreationBlockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getFirstChildCreationBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getFirstChildCreationBlock(bytes32)";
            const SELECTOR: [u8; 4] = [17u8, 113u8, 85u8, 133u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.assertionHash),
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
    /**Function with signature `getSecondChildCreationBlock(bytes32)` and selector `0x56bbc9e6`.
```solidity
function getSecondChildCreationBlock(bytes32 assertionHash) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSecondChildCreationBlockCall {
        pub assertionHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getSecondChildCreationBlock(bytes32)`](getSecondChildCreationBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSecondChildCreationBlockReturn {
        pub _0: u64,
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
            impl ::core::convert::From<getSecondChildCreationBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getSecondChildCreationBlockCall) -> Self {
                    (value.assertionHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getSecondChildCreationBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { assertionHash: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getSecondChildCreationBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getSecondChildCreationBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getSecondChildCreationBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSecondChildCreationBlockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSecondChildCreationBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSecondChildCreationBlock(bytes32)";
            const SELECTOR: [u8; 4] = [86u8, 187u8, 201u8, 230u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.assertionHash),
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
    /**Function with signature `getStaker(address)` and selector `0xa23c44b1`.
```solidity
function getStaker(address staker) external view returns (IRollupCore.Staker memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakerCall {
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getStaker(address)`](getStakerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakerReturn {
        pub _0: <IRollupCore::Staker as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getStakerCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStakerCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRollupCore::Staker,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRollupCore::Staker as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getStakerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStakerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakerReturn;
            type ReturnTuple<'a> = (IRollupCore::Staker,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStaker(address)";
            const SELECTOR: [u8; 4] = [162u8, 60u8, 68u8, 177u8];
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
                        &self.staker,
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
    /**Function with signature `getStakerAddress(uint64)` and selector `0x6ddd3744`.
```solidity
function getStakerAddress(uint64 stakerNum) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakerAddressCall {
        pub stakerNum: u64,
    }
    ///Container type for the return parameters of the [`getStakerAddress(uint64)`](getStakerAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakerAddressReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStakerAddressCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakerAddressCall) -> Self {
                    (value.stakerNum,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakerAddressCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { stakerNum: tuple.0 }
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
            impl ::core::convert::From<getStakerAddressReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakerAddressReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakerAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakerAddressCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakerAddressReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStakerAddress(uint64)";
            const SELECTOR: [u8; 4] = [109u8, 221u8, 55u8, 68u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.stakerNum),
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
    /**Function with signature `getValidators()` and selector `0xb7ab4db5`.
```solidity
function getValidators() external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorsCall {}
    ///Container type for the return parameters of the [`getValidators()`](getValidatorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorsReturn {
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getValidatorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getValidatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getValidatorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getValidatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getValidatorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getValidatorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getValidators()";
            const SELECTOR: [u8; 4] = [183u8, 171u8, 77u8, 181u8];
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
    /**Function with signature `initialize(address)` and selector `0xc4d66de8`.
```solidity
function initialize(address stakeToken) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub stakeToken: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address)`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value.stakeToken,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { stakeToken: tuple.0 }
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
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address)";
            const SELECTOR: [u8; 4] = [196u8, 214u8, 109u8, 232u8];
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
                        &self.stakeToken,
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
    /**Function with signature `isFirstChild(bytes32)` and selector `0x30836228`.
```solidity
function isFirstChild(bytes32 assertionHash) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isFirstChildCall {
        pub assertionHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isFirstChild(bytes32)`](isFirstChildCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isFirstChildReturn {
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
            impl ::core::convert::From<isFirstChildCall> for UnderlyingRustTuple<'_> {
                fn from(value: isFirstChildCall) -> Self {
                    (value.assertionHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isFirstChildCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { assertionHash: tuple.0 }
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
            impl ::core::convert::From<isFirstChildReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isFirstChildReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isFirstChildReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isFirstChildCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isFirstChildReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isFirstChild(bytes32)";
            const SELECTOR: [u8; 4] = [48u8, 131u8, 98u8, 40u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.assertionHash),
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
    /**Function with signature `isPending(bytes32)` and selector `0xe531d8c7`.
```solidity
function isPending(bytes32 assertionHash) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isPendingCall {
        pub assertionHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isPending(bytes32)`](isPendingCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isPendingReturn {
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
            impl ::core::convert::From<isPendingCall> for UnderlyingRustTuple<'_> {
                fn from(value: isPendingCall) -> Self {
                    (value.assertionHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isPendingCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { assertionHash: tuple.0 }
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
            impl ::core::convert::From<isPendingReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isPendingReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isPendingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isPendingCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isPendingReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isPending(bytes32)";
            const SELECTOR: [u8; 4] = [229u8, 49u8, 216u8, 199u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.assertionHash),
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
    /**Function with signature `isStaked(address)` and selector `0x6177fd18`.
```solidity
function isStaked(address staker) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isStakedCall {
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isStaked(address)`](isStakedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isStakedReturn {
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
            impl ::core::convert::From<isStakedCall> for UnderlyingRustTuple<'_> {
                fn from(value: isStakedCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isStakedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
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
            impl ::core::convert::From<isStakedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isStakedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isStakedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isStakedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isStakedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isStaked(address)";
            const SELECTOR: [u8; 4] = [97u8, 119u8, 253u8, 24u8];
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
                        &self.staker,
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
    /**Function with signature `isValidator(address)` and selector `0xfacd743b`.
```solidity
function isValidator(address) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidatorCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isValidator(address)`](isValidatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidatorReturn {
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
            impl ::core::convert::From<isValidatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: isValidatorCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isValidatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<isValidatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isValidatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isValidatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isValidatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isValidatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isValidator(address)";
            const SELECTOR: [u8; 4] = [250u8, 205u8, 116u8, 59u8];
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
                        &self._0,
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
    /**Function with signature `latestConfirmed()` and selector `0x65f7f80d`.
```solidity
function latestConfirmed() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestConfirmedCall {}
    ///Container type for the return parameters of the [`latestConfirmed()`](latestConfirmedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestConfirmedReturn {
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
            impl ::core::convert::From<latestConfirmedCall> for UnderlyingRustTuple<'_> {
                fn from(value: latestConfirmedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for latestConfirmedCall {
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
            impl ::core::convert::From<latestConfirmedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: latestConfirmedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for latestConfirmedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for latestConfirmedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = latestConfirmedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "latestConfirmed()";
            const SELECTOR: [u8; 4] = [101u8, 247u8, 248u8, 13u8];
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
    /**Function with signature `latestStakedAssertion(address)` and selector `0x2abdd230`.
```solidity
function latestStakedAssertion(address staker) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestStakedAssertionCall {
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`latestStakedAssertion(address)`](latestStakedAssertionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestStakedAssertionReturn {
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
            impl ::core::convert::From<latestStakedAssertionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: latestStakedAssertionCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for latestStakedAssertionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
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
            impl ::core::convert::From<latestStakedAssertionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: latestStakedAssertionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for latestStakedAssertionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for latestStakedAssertionCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = latestStakedAssertionReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "latestStakedAssertion(address)";
            const SELECTOR: [u8; 4] = [42u8, 189u8, 210u8, 48u8];
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
                        &self.staker,
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
    /**Function with signature `loserStakeEscrow()` and selector `0xf065de3f`.
```solidity
function loserStakeEscrow() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct loserStakeEscrowCall {}
    ///Container type for the return parameters of the [`loserStakeEscrow()`](loserStakeEscrowCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct loserStakeEscrowReturn {
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
            impl ::core::convert::From<loserStakeEscrowCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: loserStakeEscrowCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for loserStakeEscrowCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<loserStakeEscrowReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: loserStakeEscrowReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for loserStakeEscrowReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for loserStakeEscrowCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = loserStakeEscrowReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "loserStakeEscrow()";
            const SELECTOR: [u8; 4] = [240u8, 101u8, 222u8, 63u8];
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
    /**Function with signature `minimumAssertionPeriod()` and selector `0x45e38b64`.
```solidity
function minimumAssertionPeriod() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumAssertionPeriodCall {}
    ///Container type for the return parameters of the [`minimumAssertionPeriod()`](minimumAssertionPeriodCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumAssertionPeriodReturn {
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
            impl ::core::convert::From<minimumAssertionPeriodCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: minimumAssertionPeriodCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for minimumAssertionPeriodCall {
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
            impl ::core::convert::From<minimumAssertionPeriodReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: minimumAssertionPeriodReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for minimumAssertionPeriodReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for minimumAssertionPeriodCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = minimumAssertionPeriodReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "minimumAssertionPeriod()";
            const SELECTOR: [u8; 4] = [69u8, 227u8, 139u8, 100u8];
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
    /**Function with signature `newStake(uint256,address)` and selector `0x68129b14`.
```solidity
function newStake(uint256 tokenAmount, address withdrawalAddress) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct newStakeCall {
        pub tokenAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub withdrawalAddress: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`newStake(uint256,address)`](newStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct newStakeReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<newStakeCall> for UnderlyingRustTuple<'_> {
                fn from(value: newStakeCall) -> Self {
                    (value.tokenAmount, value.withdrawalAddress)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for newStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenAmount: tuple.0,
                        withdrawalAddress: tuple.1,
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
            impl ::core::convert::From<newStakeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: newStakeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for newStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for newStakeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = newStakeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "newStake(uint256,address)";
            const SELECTOR: [u8; 4] = [104u8, 18u8, 155u8, 20u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenAmount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawalAddress,
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
    /**Function with signature `newStakeOnNewAssertion(uint256,((bytes32,bytes32,(bytes32,uint256,address,uint64,uint64)),((bytes32[2],uint64[2]),uint8,bytes32),((bytes32[2],uint64[2]),uint8,bytes32)),bytes32,address)` and selector `0x50f32f68`.
```solidity
function newStakeOnNewAssertion(uint256 tokenAmount, AssertionInputs memory assertion, bytes32 expectedAssertionHash, address withdrawalAddress) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct newStakeOnNewAssertion_0Call {
        pub tokenAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub assertion: <AssertionInputs as alloy::sol_types::SolType>::RustType,
        pub expectedAssertionHash: alloy::sol_types::private::FixedBytes<32>,
        pub withdrawalAddress: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`newStakeOnNewAssertion(uint256,((bytes32,bytes32,(bytes32,uint256,address,uint64,uint64)),((bytes32[2],uint64[2]),uint8,bytes32),((bytes32[2],uint64[2]),uint8,bytes32)),bytes32,address)`](newStakeOnNewAssertion_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct newStakeOnNewAssertion_0Return {}
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
                alloy::sol_types::sol_data::Uint<256>,
                AssertionInputs,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <AssertionInputs as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<newStakeOnNewAssertion_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: newStakeOnNewAssertion_0Call) -> Self {
                    (
                        value.tokenAmount,
                        value.assertion,
                        value.expectedAssertionHash,
                        value.withdrawalAddress,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for newStakeOnNewAssertion_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenAmount: tuple.0,
                        assertion: tuple.1,
                        expectedAssertionHash: tuple.2,
                        withdrawalAddress: tuple.3,
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
            impl ::core::convert::From<newStakeOnNewAssertion_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: newStakeOnNewAssertion_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for newStakeOnNewAssertion_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for newStakeOnNewAssertion_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                AssertionInputs,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = newStakeOnNewAssertion_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "newStakeOnNewAssertion(uint256,((bytes32,bytes32,(bytes32,uint256,address,uint64,uint64)),((bytes32[2],uint64[2]),uint8,bytes32),((bytes32[2],uint64[2]),uint8,bytes32)),bytes32,address)";
            const SELECTOR: [u8; 4] = [80u8, 243u8, 47u8, 104u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenAmount),
                    <AssertionInputs as alloy_sol_types::SolType>::tokenize(
                        &self.assertion,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.expectedAssertionHash,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawalAddress,
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
    /**Function with signature `newStakeOnNewAssertion(uint256,((bytes32,bytes32,(bytes32,uint256,address,uint64,uint64)),((bytes32[2],uint64[2]),uint8,bytes32),((bytes32[2],uint64[2]),uint8,bytes32)),bytes32)` and selector `0x7300201c`.
```solidity
function newStakeOnNewAssertion(uint256 tokenAmount, AssertionInputs memory assertion, bytes32 expectedAssertionHash) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct newStakeOnNewAssertion_1Call {
        pub tokenAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub assertion: <AssertionInputs as alloy::sol_types::SolType>::RustType,
        pub expectedAssertionHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`newStakeOnNewAssertion(uint256,((bytes32,bytes32,(bytes32,uint256,address,uint64,uint64)),((bytes32[2],uint64[2]),uint8,bytes32),((bytes32[2],uint64[2]),uint8,bytes32)),bytes32)`](newStakeOnNewAssertion_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct newStakeOnNewAssertion_1Return {}
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
                alloy::sol_types::sol_data::Uint<256>,
                AssertionInputs,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <AssertionInputs as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<newStakeOnNewAssertion_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: newStakeOnNewAssertion_1Call) -> Self {
                    (value.tokenAmount, value.assertion, value.expectedAssertionHash)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for newStakeOnNewAssertion_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenAmount: tuple.0,
                        assertion: tuple.1,
                        expectedAssertionHash: tuple.2,
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
            impl ::core::convert::From<newStakeOnNewAssertion_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: newStakeOnNewAssertion_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for newStakeOnNewAssertion_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for newStakeOnNewAssertion_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                AssertionInputs,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = newStakeOnNewAssertion_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "newStakeOnNewAssertion(uint256,((bytes32,bytes32,(bytes32,uint256,address,uint64,uint64)),((bytes32[2],uint64[2]),uint8,bytes32),((bytes32[2],uint64[2]),uint8,bytes32)),bytes32)";
            const SELECTOR: [u8; 4] = [115u8, 0u8, 32u8, 28u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenAmount),
                    <AssertionInputs as alloy_sol_types::SolType>::tokenize(
                        &self.assertion,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.expectedAssertionHash),
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
    /**Function with signature `outbox()` and selector `0xce11e6ab`.
```solidity
function outbox() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct outboxCall {}
    ///Container type for the return parameters of the [`outbox()`](outboxCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct outboxReturn {
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
            impl ::core::convert::From<outboxCall> for UnderlyingRustTuple<'_> {
                fn from(value: outboxCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for outboxCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<outboxReturn> for UnderlyingRustTuple<'_> {
                fn from(value: outboxReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for outboxReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for outboxCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = outboxReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "outbox()";
            const SELECTOR: [u8; 4] = [206u8, 17u8, 230u8, 171u8];
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
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
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
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
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
    /**Function with signature `reduceDeposit(uint256)` and selector `0x1e83d30f`.
```solidity
function reduceDeposit(uint256 target) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reduceDepositCall {
        pub target: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`reduceDeposit(uint256)`](reduceDepositCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reduceDepositReturn {}
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
            impl ::core::convert::From<reduceDepositCall> for UnderlyingRustTuple<'_> {
                fn from(value: reduceDepositCall) -> Self {
                    (value.target,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for reduceDepositCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { target: tuple.0 }
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
            impl ::core::convert::From<reduceDepositReturn> for UnderlyingRustTuple<'_> {
                fn from(value: reduceDepositReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for reduceDepositReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for reduceDepositCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = reduceDepositReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reduceDeposit(uint256)";
            const SELECTOR: [u8; 4] = [30u8, 131u8, 211u8, 15u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.target),
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
    /**Function with signature `removeWhitelistAfterFork()` and selector `0xc2c2e68e`.
```solidity
function removeWhitelistAfterFork() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeWhitelistAfterForkCall {}
    ///Container type for the return parameters of the [`removeWhitelistAfterFork()`](removeWhitelistAfterForkCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeWhitelistAfterForkReturn {}
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
            impl ::core::convert::From<removeWhitelistAfterForkCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeWhitelistAfterForkCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeWhitelistAfterForkCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<removeWhitelistAfterForkReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeWhitelistAfterForkReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeWhitelistAfterForkReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeWhitelistAfterForkCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeWhitelistAfterForkReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeWhitelistAfterFork()";
            const SELECTOR: [u8; 4] = [194u8, 194u8, 230u8, 142u8];
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
    /**Function with signature `removeWhitelistAfterValidatorAfk()` and selector `0x18baaab9`.
```solidity
function removeWhitelistAfterValidatorAfk() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeWhitelistAfterValidatorAfkCall {}
    ///Container type for the return parameters of the [`removeWhitelistAfterValidatorAfk()`](removeWhitelistAfterValidatorAfkCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeWhitelistAfterValidatorAfkReturn {}
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
            impl ::core::convert::From<removeWhitelistAfterValidatorAfkCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeWhitelistAfterValidatorAfkCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeWhitelistAfterValidatorAfkCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<removeWhitelistAfterValidatorAfkReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeWhitelistAfterValidatorAfkReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeWhitelistAfterValidatorAfkReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeWhitelistAfterValidatorAfkCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeWhitelistAfterValidatorAfkReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeWhitelistAfterValidatorAfk()";
            const SELECTOR: [u8; 4] = [24u8, 186u8, 170u8, 185u8];
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
    /**Function with signature `returnOldDeposit()` and selector `0x57ef4ab9`.
```solidity
function returnOldDeposit() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct returnOldDepositCall {}
    ///Container type for the return parameters of the [`returnOldDeposit()`](returnOldDepositCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct returnOldDepositReturn {}
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
            impl ::core::convert::From<returnOldDepositCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: returnOldDepositCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for returnOldDepositCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<returnOldDepositReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: returnOldDepositReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for returnOldDepositReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for returnOldDepositCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = returnOldDepositReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "returnOldDeposit()";
            const SELECTOR: [u8; 4] = [87u8, 239u8, 74u8, 185u8];
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
    /**Function with signature `returnOldDepositFor(address)` and selector `0x588c7a16`.
```solidity
function returnOldDepositFor(address stakerAddress) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct returnOldDepositForCall {
        pub stakerAddress: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`returnOldDepositFor(address)`](returnOldDepositForCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct returnOldDepositForReturn {}
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
            impl ::core::convert::From<returnOldDepositForCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: returnOldDepositForCall) -> Self {
                    (value.stakerAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for returnOldDepositForCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { stakerAddress: tuple.0 }
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
            impl ::core::convert::From<returnOldDepositForReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: returnOldDepositForReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for returnOldDepositForReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for returnOldDepositForCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = returnOldDepositForReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "returnOldDepositFor(address)";
            const SELECTOR: [u8; 4] = [88u8, 140u8, 122u8, 22u8];
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
                        &self.stakerAddress,
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
    /**Function with signature `rollupEventInbox()` and selector `0xaa38a6e7`.
```solidity
function rollupEventInbox() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rollupEventInboxCall {}
    ///Container type for the return parameters of the [`rollupEventInbox()`](rollupEventInboxCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rollupEventInboxReturn {
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
            impl ::core::convert::From<rollupEventInboxCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: rollupEventInboxCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rollupEventInboxCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<rollupEventInboxReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: rollupEventInboxReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rollupEventInboxReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rollupEventInboxCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = rollupEventInboxReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rollupEventInbox()";
            const SELECTOR: [u8; 4] = [170u8, 56u8, 166u8, 231u8];
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
    /**Function with signature `sequencerInbox()` and selector `0xee35f327`.
```solidity
function sequencerInbox() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sequencerInboxCall {}
    ///Container type for the return parameters of the [`sequencerInbox()`](sequencerInboxCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sequencerInboxReturn {
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
            impl ::core::convert::From<sequencerInboxCall> for UnderlyingRustTuple<'_> {
                fn from(value: sequencerInboxCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for sequencerInboxCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<sequencerInboxReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: sequencerInboxReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for sequencerInboxReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for sequencerInboxCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = sequencerInboxReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "sequencerInbox()";
            const SELECTOR: [u8; 4] = [238u8, 53u8, 243u8, 39u8];
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
    /**Function with signature `stakeOnNewAssertion(((bytes32,bytes32,(bytes32,uint256,address,uint64,uint64)),((bytes32[2],uint64[2]),uint8,bytes32),((bytes32[2],uint64[2]),uint8,bytes32)),bytes32)` and selector `0x3b86de19`.
```solidity
function stakeOnNewAssertion(AssertionInputs memory assertion, bytes32 expectedAssertionHash) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeOnNewAssertionCall {
        pub assertion: <AssertionInputs as alloy::sol_types::SolType>::RustType,
        pub expectedAssertionHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`stakeOnNewAssertion(((bytes32,bytes32,(bytes32,uint256,address,uint64,uint64)),((bytes32[2],uint64[2]),uint8,bytes32),((bytes32[2],uint64[2]),uint8,bytes32)),bytes32)`](stakeOnNewAssertionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeOnNewAssertionReturn {}
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
                AssertionInputs,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <AssertionInputs as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<stakeOnNewAssertionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: stakeOnNewAssertionCall) -> Self {
                    (value.assertion, value.expectedAssertionHash)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for stakeOnNewAssertionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        assertion: tuple.0,
                        expectedAssertionHash: tuple.1,
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
            impl ::core::convert::From<stakeOnNewAssertionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: stakeOnNewAssertionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for stakeOnNewAssertionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakeOnNewAssertionCall {
            type Parameters<'a> = (
                AssertionInputs,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeOnNewAssertionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakeOnNewAssertion(((bytes32,bytes32,(bytes32,uint256,address,uint64,uint64)),((bytes32[2],uint64[2]),uint8,bytes32),((bytes32[2],uint64[2]),uint8,bytes32)),bytes32)";
            const SELECTOR: [u8; 4] = [59u8, 134u8, 222u8, 25u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <AssertionInputs as alloy_sol_types::SolType>::tokenize(
                        &self.assertion,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.expectedAssertionHash),
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
    /**Function with signature `stakeToken()` and selector `0x51ed6a30`.
```solidity
function stakeToken() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeTokenCall {}
    ///Container type for the return parameters of the [`stakeToken()`](stakeTokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeTokenReturn {
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
            impl ::core::convert::From<stakeTokenCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakeTokenCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeTokenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<stakeTokenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakeTokenReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeTokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakeTokenCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeTokenReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakeToken()";
            const SELECTOR: [u8; 4] = [81u8, 237u8, 106u8, 48u8];
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
    /**Function with signature `stakerCount()` and selector `0xdff69787`.
```solidity
function stakerCount() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakerCountCall {}
    ///Container type for the return parameters of the [`stakerCount()`](stakerCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakerCountReturn {
        pub _0: u64,
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
            impl ::core::convert::From<stakerCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakerCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakerCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakerCountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakerCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakerCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakerCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakerCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakerCount()";
            const SELECTOR: [u8; 4] = [223u8, 246u8, 151u8, 135u8];
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
    /**Function with signature `validateAssertionHash(bytes32,((bytes32[2],uint64[2]),uint8,bytes32),bytes32,bytes32)` and selector `0xe51019a6`.
```solidity
function validateAssertionHash(bytes32 assertionHash, AssertionState memory state, bytes32 prevAssertionHash, bytes32 inboxAcc) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validateAssertionHashCall {
        pub assertionHash: alloy::sol_types::private::FixedBytes<32>,
        pub state: <AssertionState as alloy::sol_types::SolType>::RustType,
        pub prevAssertionHash: alloy::sol_types::private::FixedBytes<32>,
        pub inboxAcc: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`validateAssertionHash(bytes32,((bytes32[2],uint64[2]),uint8,bytes32),bytes32,bytes32)`](validateAssertionHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validateAssertionHashReturn {}
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
                AssertionState,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <AssertionState as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<validateAssertionHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: validateAssertionHashCall) -> Self {
                    (
                        value.assertionHash,
                        value.state,
                        value.prevAssertionHash,
                        value.inboxAcc,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validateAssertionHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        assertionHash: tuple.0,
                        state: tuple.1,
                        prevAssertionHash: tuple.2,
                        inboxAcc: tuple.3,
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
            impl ::core::convert::From<validateAssertionHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: validateAssertionHashReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validateAssertionHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validateAssertionHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                AssertionState,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = validateAssertionHashReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validateAssertionHash(bytes32,((bytes32[2],uint64[2]),uint8,bytes32),bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [229u8, 16u8, 25u8, 166u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.assertionHash),
                    <AssertionState as alloy_sol_types::SolType>::tokenize(&self.state),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.prevAssertionHash),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.inboxAcc),
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
    /**Function with signature `validateConfig(bytes32,(bytes32,uint256,address,uint64,uint64))` and selector `0x04972af9`.
```solidity
function validateConfig(bytes32 assertionHash, ConfigData memory configData) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validateConfigCall {
        pub assertionHash: alloy::sol_types::private::FixedBytes<32>,
        pub configData: <ConfigData as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`validateConfig(bytes32,(bytes32,uint256,address,uint64,uint64))`](validateConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validateConfigReturn {}
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
                ConfigData,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <ConfigData as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<validateConfigCall> for UnderlyingRustTuple<'_> {
                fn from(value: validateConfigCall) -> Self {
                    (value.assertionHash, value.configData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validateConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        assertionHash: tuple.0,
                        configData: tuple.1,
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
            impl ::core::convert::From<validateConfigReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: validateConfigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validateConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validateConfigCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                ConfigData,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = validateConfigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validateConfig(bytes32,(bytes32,uint256,address,uint64,uint64))";
            const SELECTOR: [u8; 4] = [4u8, 151u8, 42u8, 249u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.assertionHash),
                    <ConfigData as alloy_sol_types::SolType>::tokenize(&self.configData),
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
    /**Function with signature `validatorAfkBlocks()` and selector `0xe6b3082c`.
```solidity
function validatorAfkBlocks() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorAfkBlocksCall {}
    ///Container type for the return parameters of the [`validatorAfkBlocks()`](validatorAfkBlocksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorAfkBlocksReturn {
        pub _0: u64,
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
            impl ::core::convert::From<validatorAfkBlocksCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatorAfkBlocksCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatorAfkBlocksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorAfkBlocksReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatorAfkBlocksReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatorAfkBlocksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatorAfkBlocksCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = validatorAfkBlocksReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validatorAfkBlocks()";
            const SELECTOR: [u8; 4] = [230u8, 179u8, 8u8, 44u8];
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
    /**Function with signature `validatorWhitelistDisabled()` and selector `0x12ab3d3b`.
```solidity
function validatorWhitelistDisabled() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorWhitelistDisabledCall {}
    ///Container type for the return parameters of the [`validatorWhitelistDisabled()`](validatorWhitelistDisabledCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorWhitelistDisabledReturn {
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
            impl ::core::convert::From<validatorWhitelistDisabledCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatorWhitelistDisabledCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatorWhitelistDisabledCall {
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
            impl ::core::convert::From<validatorWhitelistDisabledReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatorWhitelistDisabledReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatorWhitelistDisabledReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatorWhitelistDisabledCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = validatorWhitelistDisabledReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validatorWhitelistDisabled()";
            const SELECTOR: [u8; 4] = [18u8, 171u8, 61u8, 59u8];
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
    /**Function with signature `wasmModuleRoot()` and selector `0x8ee1a126`.
```solidity
function wasmModuleRoot() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct wasmModuleRootCall {}
    ///Container type for the return parameters of the [`wasmModuleRoot()`](wasmModuleRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct wasmModuleRootReturn {
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
            impl ::core::convert::From<wasmModuleRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: wasmModuleRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for wasmModuleRootCall {
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
            impl ::core::convert::From<wasmModuleRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: wasmModuleRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for wasmModuleRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for wasmModuleRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = wasmModuleRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "wasmModuleRoot()";
            const SELECTOR: [u8; 4] = [142u8, 225u8, 161u8, 38u8];
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
    /**Function with signature `withdrawStakerFunds()` and selector `0x61373919`.
```solidity
function withdrawStakerFunds() external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawStakerFundsCall {}
    ///Container type for the return parameters of the [`withdrawStakerFunds()`](withdrawStakerFundsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawStakerFundsReturn {
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
            impl ::core::convert::From<withdrawStakerFundsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawStakerFundsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawStakerFundsCall {
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
            impl ::core::convert::From<withdrawStakerFundsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawStakerFundsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawStakerFundsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawStakerFundsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawStakerFundsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawStakerFunds()";
            const SELECTOR: [u8; 4] = [97u8, 55u8, 57u8, 25u8];
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
    /**Function with signature `withdrawableFunds(address)` and selector `0x2f30cabd`.
```solidity
function withdrawableFunds(address owner) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawableFundsCall {
        pub owner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`withdrawableFunds(address)`](withdrawableFundsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawableFundsReturn {
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
            impl ::core::convert::From<withdrawableFundsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawableFundsCall) -> Self {
                    (value.owner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawableFundsCall {
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
            impl ::core::convert::From<withdrawableFundsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawableFundsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawableFundsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawableFundsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawableFundsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawableFunds(address)";
            const SELECTOR: [u8; 4] = [47u8, 48u8, 202u8, 189u8];
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
    /**Function with signature `withdrawalAddress(address)` and selector `0x84728cd0`.
```solidity
function withdrawalAddress(address staker) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawalAddressCall {
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`withdrawalAddress(address)`](withdrawalAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawalAddressReturn {
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
            impl ::core::convert::From<withdrawalAddressCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawalAddressCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawalAddressCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
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
            impl ::core::convert::From<withdrawalAddressReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawalAddressReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawalAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawalAddressCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawalAddressReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawalAddress(address)";
            const SELECTOR: [u8; 4] = [132u8, 114u8, 140u8, 208u8];
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
                        &self.staker,
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
    ///Container for all the [`IRollupUser`](self) function calls.
    pub enum IRollupUserCalls {
        addToDeposit(addToDepositCall),
        amountStaked(amountStakedCall),
        baseStake(baseStakeCall),
        bridge(bridgeCall),
        chainId(chainIdCall),
        challengeManager(challengeManagerCall),
        confirmAssertion(confirmAssertionCall),
        confirmPeriodBlocks(confirmPeriodBlocksCall),
        genesisAssertionHash(genesisAssertionHashCall),
        getAssertion(getAssertionCall),
        getAssertionCreationBlockForLogLookup(getAssertionCreationBlockForLogLookupCall),
        getFirstChildCreationBlock(getFirstChildCreationBlockCall),
        getSecondChildCreationBlock(getSecondChildCreationBlockCall),
        getStaker(getStakerCall),
        getStakerAddress(getStakerAddressCall),
        getValidators(getValidatorsCall),
        initialize(initializeCall),
        isFirstChild(isFirstChildCall),
        isPending(isPendingCall),
        isStaked(isStakedCall),
        isValidator(isValidatorCall),
        latestConfirmed(latestConfirmedCall),
        latestStakedAssertion(latestStakedAssertionCall),
        loserStakeEscrow(loserStakeEscrowCall),
        minimumAssertionPeriod(minimumAssertionPeriodCall),
        newStake(newStakeCall),
        newStakeOnNewAssertion_0(newStakeOnNewAssertion_0Call),
        newStakeOnNewAssertion_1(newStakeOnNewAssertion_1Call),
        outbox(outboxCall),
        owner(ownerCall),
        reduceDeposit(reduceDepositCall),
        removeWhitelistAfterFork(removeWhitelistAfterForkCall),
        removeWhitelistAfterValidatorAfk(removeWhitelistAfterValidatorAfkCall),
        returnOldDeposit(returnOldDepositCall),
        returnOldDepositFor(returnOldDepositForCall),
        rollupEventInbox(rollupEventInboxCall),
        sequencerInbox(sequencerInboxCall),
        stakeOnNewAssertion(stakeOnNewAssertionCall),
        stakeToken(stakeTokenCall),
        stakerCount(stakerCountCall),
        validateAssertionHash(validateAssertionHashCall),
        validateConfig(validateConfigCall),
        validatorAfkBlocks(validatorAfkBlocksCall),
        validatorWhitelistDisabled(validatorWhitelistDisabledCall),
        wasmModuleRoot(wasmModuleRootCall),
        withdrawStakerFunds(withdrawStakerFundsCall),
        withdrawableFunds(withdrawableFundsCall),
        withdrawalAddress(withdrawalAddressCall),
    }
    #[automatically_derived]
    impl IRollupUserCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [2u8, 58u8, 150u8, 254u8],
            [4u8, 151u8, 42u8, 249u8],
            [16u8, 185u8, 138u8, 53u8],
            [17u8, 113u8, 85u8, 133u8],
            [18u8, 171u8, 61u8, 59u8],
            [19u8, 197u8, 108u8, 167u8],
            [24u8, 186u8, 170u8, 185u8],
            [30u8, 131u8, 211u8, 15u8],
            [42u8, 189u8, 210u8, 48u8],
            [46u8, 122u8, 207u8, 166u8],
            [47u8, 48u8, 202u8, 189u8],
            [48u8, 131u8, 98u8, 40u8],
            [53u8, 51u8, 37u8, 224u8],
            [59u8, 134u8, 222u8, 25u8],
            [69u8, 227u8, 139u8, 100u8],
            [80u8, 243u8, 47u8, 104u8],
            [81u8, 237u8, 106u8, 48u8],
            [86u8, 187u8, 201u8, 230u8],
            [87u8, 239u8, 74u8, 185u8],
            [88u8, 140u8, 122u8, 22u8],
            [97u8, 55u8, 57u8, 25u8],
            [97u8, 119u8, 253u8, 24u8],
            [101u8, 247u8, 248u8, 13u8],
            [104u8, 18u8, 155u8, 20u8],
            [104u8, 95u8, 94u8, 204u8],
            [109u8, 221u8, 55u8, 68u8],
            [115u8, 0u8, 32u8, 28u8],
            [118u8, 231u8, 226u8, 59u8],
            [132u8, 114u8, 140u8, 208u8],
            [136u8, 48u8, 40u8, 132u8],
            [141u8, 165u8, 203u8, 91u8],
            [142u8, 225u8, 161u8, 38u8],
            [154u8, 138u8, 5u8, 146u8],
            [162u8, 60u8, 68u8, 177u8],
            [170u8, 56u8, 166u8, 231u8],
            [183u8, 171u8, 77u8, 181u8],
            [194u8, 194u8, 230u8, 142u8],
            [196u8, 214u8, 109u8, 232u8],
            [206u8, 17u8, 230u8, 171u8],
            [223u8, 246u8, 151u8, 135u8],
            [229u8, 16u8, 25u8, 166u8],
            [229u8, 49u8, 216u8, 199u8],
            [230u8, 179u8, 8u8, 44u8],
            [231u8, 140u8, 234u8, 146u8],
            [238u8, 53u8, 243u8, 39u8],
            [239u8, 64u8, 166u8, 112u8],
            [240u8, 101u8, 222u8, 63u8],
            [250u8, 205u8, 116u8, 59u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IRollupUserCalls {
        const NAME: &'static str = "IRollupUserCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 48usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::addToDeposit(_) => {
                    <addToDepositCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::amountStaked(_) => {
                    <amountStakedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::baseStake(_) => {
                    <baseStakeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::bridge(_) => <bridgeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::chainId(_) => <chainIdCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::challengeManager(_) => {
                    <challengeManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::confirmAssertion(_) => {
                    <confirmAssertionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::confirmPeriodBlocks(_) => {
                    <confirmPeriodBlocksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::genesisAssertionHash(_) => {
                    <genesisAssertionHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAssertion(_) => {
                    <getAssertionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAssertionCreationBlockForLogLookup(_) => {
                    <getAssertionCreationBlockForLogLookupCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getFirstChildCreationBlock(_) => {
                    <getFirstChildCreationBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getSecondChildCreationBlock(_) => {
                    <getSecondChildCreationBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStaker(_) => {
                    <getStakerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakerAddress(_) => {
                    <getStakerAddressCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getValidators(_) => {
                    <getValidatorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isFirstChild(_) => {
                    <isFirstChildCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isPending(_) => {
                    <isPendingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isStaked(_) => <isStakedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::isValidator(_) => {
                    <isValidatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::latestConfirmed(_) => {
                    <latestConfirmedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::latestStakedAssertion(_) => {
                    <latestStakedAssertionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::loserStakeEscrow(_) => {
                    <loserStakeEscrowCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::minimumAssertionPeriod(_) => {
                    <minimumAssertionPeriodCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::newStake(_) => <newStakeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::newStakeOnNewAssertion_0(_) => {
                    <newStakeOnNewAssertion_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::newStakeOnNewAssertion_1(_) => {
                    <newStakeOnNewAssertion_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::outbox(_) => <outboxCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::reduceDeposit(_) => {
                    <reduceDepositCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeWhitelistAfterFork(_) => {
                    <removeWhitelistAfterForkCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeWhitelistAfterValidatorAfk(_) => {
                    <removeWhitelistAfterValidatorAfkCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::returnOldDeposit(_) => {
                    <returnOldDepositCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::returnOldDepositFor(_) => {
                    <returnOldDepositForCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rollupEventInbox(_) => {
                    <rollupEventInboxCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::sequencerInbox(_) => {
                    <sequencerInboxCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeOnNewAssertion(_) => {
                    <stakeOnNewAssertionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeToken(_) => {
                    <stakeTokenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakerCount(_) => {
                    <stakerCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validateAssertionHash(_) => {
                    <validateAssertionHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validateConfig(_) => {
                    <validateConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validatorAfkBlocks(_) => {
                    <validatorAfkBlocksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validatorWhitelistDisabled(_) => {
                    <validatorWhitelistDisabledCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::wasmModuleRoot(_) => {
                    <wasmModuleRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawStakerFunds(_) => {
                    <withdrawStakerFundsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawableFunds(_) => {
                    <withdrawableFundsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawalAddress(_) => {
                    <withdrawalAddressCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<IRollupUserCalls>] = &[
                {
                    fn challengeManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <challengeManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::challengeManager)
                    }
                    challengeManager
                },
                {
                    fn validateConfig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <validateConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::validateConfig)
                    }
                    validateConfig
                },
                {
                    fn confirmAssertion(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <confirmAssertionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::confirmAssertion)
                    }
                    confirmAssertion
                },
                {
                    fn getFirstChildCreationBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <getFirstChildCreationBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::getFirstChildCreationBlock)
                    }
                    getFirstChildCreationBlock
                },
                {
                    fn validatorWhitelistDisabled(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <validatorWhitelistDisabledCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::validatorWhitelistDisabled)
                    }
                    validatorWhitelistDisabled
                },
                {
                    fn getAssertionCreationBlockForLogLookup(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <getAssertionCreationBlockForLogLookupCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::getAssertionCreationBlockForLogLookup)
                    }
                    getAssertionCreationBlockForLogLookup
                },
                {
                    fn removeWhitelistAfterValidatorAfk(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <removeWhitelistAfterValidatorAfkCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::removeWhitelistAfterValidatorAfk)
                    }
                    removeWhitelistAfterValidatorAfk
                },
                {
                    fn reduceDeposit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <reduceDepositCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::reduceDeposit)
                    }
                    reduceDeposit
                },
                {
                    fn latestStakedAssertion(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <latestStakedAssertionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::latestStakedAssertion)
                    }
                    latestStakedAssertion
                },
                {
                    fn confirmPeriodBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <confirmPeriodBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::confirmPeriodBlocks)
                    }
                    confirmPeriodBlocks
                },
                {
                    fn withdrawableFunds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <withdrawableFundsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::withdrawableFunds)
                    }
                    withdrawableFunds
                },
                {
                    fn isFirstChild(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <isFirstChildCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::isFirstChild)
                    }
                    isFirstChild
                },
                {
                    fn genesisAssertionHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <genesisAssertionHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::genesisAssertionHash)
                    }
                    genesisAssertionHash
                },
                {
                    fn stakeOnNewAssertion(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <stakeOnNewAssertionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::stakeOnNewAssertion)
                    }
                    stakeOnNewAssertion
                },
                {
                    fn minimumAssertionPeriod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <minimumAssertionPeriodCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::minimumAssertionPeriod)
                    }
                    minimumAssertionPeriod
                },
                {
                    fn newStakeOnNewAssertion_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <newStakeOnNewAssertion_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::newStakeOnNewAssertion_0)
                    }
                    newStakeOnNewAssertion_0
                },
                {
                    fn stakeToken(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <stakeTokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::stakeToken)
                    }
                    stakeToken
                },
                {
                    fn getSecondChildCreationBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <getSecondChildCreationBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::getSecondChildCreationBlock)
                    }
                    getSecondChildCreationBlock
                },
                {
                    fn returnOldDeposit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <returnOldDepositCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::returnOldDeposit)
                    }
                    returnOldDeposit
                },
                {
                    fn returnOldDepositFor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <returnOldDepositForCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::returnOldDepositFor)
                    }
                    returnOldDepositFor
                },
                {
                    fn withdrawStakerFunds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <withdrawStakerFundsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::withdrawStakerFunds)
                    }
                    withdrawStakerFunds
                },
                {
                    fn isStaked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <isStakedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::isStaked)
                    }
                    isStaked
                },
                {
                    fn latestConfirmed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <latestConfirmedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::latestConfirmed)
                    }
                    latestConfirmed
                },
                {
                    fn newStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <newStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::newStake)
                    }
                    newStake
                },
                {
                    fn addToDeposit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <addToDepositCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::addToDeposit)
                    }
                    addToDeposit
                },
                {
                    fn getStakerAddress(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <getStakerAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::getStakerAddress)
                    }
                    getStakerAddress
                },
                {
                    fn newStakeOnNewAssertion_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <newStakeOnNewAssertion_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::newStakeOnNewAssertion_1)
                    }
                    newStakeOnNewAssertion_1
                },
                {
                    fn baseStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <baseStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::baseStake)
                    }
                    baseStake
                },
                {
                    fn withdrawalAddress(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <withdrawalAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::withdrawalAddress)
                    }
                    withdrawalAddress
                },
                {
                    fn getAssertion(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <getAssertionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::getAssertion)
                    }
                    getAssertion
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::owner)
                    }
                    owner
                },
                {
                    fn wasmModuleRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <wasmModuleRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::wasmModuleRoot)
                    }
                    wasmModuleRoot
                },
                {
                    fn chainId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <chainIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::chainId)
                    }
                    chainId
                },
                {
                    fn getStaker(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <getStakerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::getStaker)
                    }
                    getStaker
                },
                {
                    fn rollupEventInbox(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <rollupEventInboxCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::rollupEventInbox)
                    }
                    rollupEventInbox
                },
                {
                    fn getValidators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <getValidatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::getValidators)
                    }
                    getValidators
                },
                {
                    fn removeWhitelistAfterFork(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <removeWhitelistAfterForkCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::removeWhitelistAfterFork)
                    }
                    removeWhitelistAfterFork
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::initialize)
                    }
                    initialize
                },
                {
                    fn outbox(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <outboxCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::outbox)
                    }
                    outbox
                },
                {
                    fn stakerCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <stakerCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::stakerCount)
                    }
                    stakerCount
                },
                {
                    fn validateAssertionHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <validateAssertionHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::validateAssertionHash)
                    }
                    validateAssertionHash
                },
                {
                    fn isPending(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <isPendingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::isPending)
                    }
                    isPending
                },
                {
                    fn validatorAfkBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <validatorAfkBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::validatorAfkBlocks)
                    }
                    validatorAfkBlocks
                },
                {
                    fn bridge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <bridgeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::bridge)
                    }
                    bridge
                },
                {
                    fn sequencerInbox(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <sequencerInboxCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::sequencerInbox)
                    }
                    sequencerInbox
                },
                {
                    fn amountStaked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <amountStakedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::amountStaked)
                    }
                    amountStaked
                },
                {
                    fn loserStakeEscrow(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <loserStakeEscrowCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::loserStakeEscrow)
                    }
                    loserStakeEscrow
                },
                {
                    fn isValidator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IRollupUserCalls> {
                        <isValidatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IRollupUserCalls::isValidator)
                    }
                    isValidator
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
                Self::addToDeposit(inner) => {
                    <addToDepositCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::amountStaked(inner) => {
                    <amountStakedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::baseStake(inner) => {
                    <baseStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::bridge(inner) => {
                    <bridgeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::chainId(inner) => {
                    <chainIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::challengeManager(inner) => {
                    <challengeManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::confirmAssertion(inner) => {
                    <confirmAssertionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::confirmPeriodBlocks(inner) => {
                    <confirmPeriodBlocksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::genesisAssertionHash(inner) => {
                    <genesisAssertionHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAssertion(inner) => {
                    <getAssertionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAssertionCreationBlockForLogLookup(inner) => {
                    <getAssertionCreationBlockForLogLookupCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getFirstChildCreationBlock(inner) => {
                    <getFirstChildCreationBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getSecondChildCreationBlock(inner) => {
                    <getSecondChildCreationBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStaker(inner) => {
                    <getStakerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getStakerAddress(inner) => {
                    <getStakerAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getValidators(inner) => {
                    <getValidatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isFirstChild(inner) => {
                    <isFirstChildCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isPending(inner) => {
                    <isPendingCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isStaked(inner) => {
                    <isStakedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isValidator(inner) => {
                    <isValidatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::latestConfirmed(inner) => {
                    <latestConfirmedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::latestStakedAssertion(inner) => {
                    <latestStakedAssertionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::loserStakeEscrow(inner) => {
                    <loserStakeEscrowCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::minimumAssertionPeriod(inner) => {
                    <minimumAssertionPeriodCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::newStake(inner) => {
                    <newStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::newStakeOnNewAssertion_0(inner) => {
                    <newStakeOnNewAssertion_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::newStakeOnNewAssertion_1(inner) => {
                    <newStakeOnNewAssertion_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::outbox(inner) => {
                    <outboxCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::reduceDeposit(inner) => {
                    <reduceDepositCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeWhitelistAfterFork(inner) => {
                    <removeWhitelistAfterForkCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeWhitelistAfterValidatorAfk(inner) => {
                    <removeWhitelistAfterValidatorAfkCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::returnOldDeposit(inner) => {
                    <returnOldDepositCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::returnOldDepositFor(inner) => {
                    <returnOldDepositForCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::rollupEventInbox(inner) => {
                    <rollupEventInboxCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::sequencerInbox(inner) => {
                    <sequencerInboxCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakeOnNewAssertion(inner) => {
                    <stakeOnNewAssertionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakeToken(inner) => {
                    <stakeTokenCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::stakerCount(inner) => {
                    <stakerCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validateAssertionHash(inner) => {
                    <validateAssertionHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validateConfig(inner) => {
                    <validateConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validatorAfkBlocks(inner) => {
                    <validatorAfkBlocksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validatorWhitelistDisabled(inner) => {
                    <validatorWhitelistDisabledCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::wasmModuleRoot(inner) => {
                    <wasmModuleRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdrawStakerFunds(inner) => {
                    <withdrawStakerFundsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdrawableFunds(inner) => {
                    <withdrawableFundsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdrawalAddress(inner) => {
                    <withdrawalAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::addToDeposit(inner) => {
                    <addToDepositCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::amountStaked(inner) => {
                    <amountStakedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::baseStake(inner) => {
                    <baseStakeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::bridge(inner) => {
                    <bridgeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::chainId(inner) => {
                    <chainIdCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::challengeManager(inner) => {
                    <challengeManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::confirmAssertion(inner) => {
                    <confirmAssertionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::confirmPeriodBlocks(inner) => {
                    <confirmPeriodBlocksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::genesisAssertionHash(inner) => {
                    <genesisAssertionHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAssertion(inner) => {
                    <getAssertionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAssertionCreationBlockForLogLookup(inner) => {
                    <getAssertionCreationBlockForLogLookupCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getFirstChildCreationBlock(inner) => {
                    <getFirstChildCreationBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getSecondChildCreationBlock(inner) => {
                    <getSecondChildCreationBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStaker(inner) => {
                    <getStakerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakerAddress(inner) => {
                    <getStakerAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getValidators(inner) => {
                    <getValidatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isFirstChild(inner) => {
                    <isFirstChildCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isPending(inner) => {
                    <isPendingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isStaked(inner) => {
                    <isStakedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isValidator(inner) => {
                    <isValidatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::latestConfirmed(inner) => {
                    <latestConfirmedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::latestStakedAssertion(inner) => {
                    <latestStakedAssertionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::loserStakeEscrow(inner) => {
                    <loserStakeEscrowCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::minimumAssertionPeriod(inner) => {
                    <minimumAssertionPeriodCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::newStake(inner) => {
                    <newStakeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::newStakeOnNewAssertion_0(inner) => {
                    <newStakeOnNewAssertion_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::newStakeOnNewAssertion_1(inner) => {
                    <newStakeOnNewAssertion_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::outbox(inner) => {
                    <outboxCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::reduceDeposit(inner) => {
                    <reduceDepositCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeWhitelistAfterFork(inner) => {
                    <removeWhitelistAfterForkCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeWhitelistAfterValidatorAfk(inner) => {
                    <removeWhitelistAfterValidatorAfkCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::returnOldDeposit(inner) => {
                    <returnOldDepositCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::returnOldDepositFor(inner) => {
                    <returnOldDepositForCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::rollupEventInbox(inner) => {
                    <rollupEventInboxCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::sequencerInbox(inner) => {
                    <sequencerInboxCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stakeOnNewAssertion(inner) => {
                    <stakeOnNewAssertionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stakeToken(inner) => {
                    <stakeTokenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stakerCount(inner) => {
                    <stakerCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validateAssertionHash(inner) => {
                    <validateAssertionHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validateConfig(inner) => {
                    <validateConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validatorAfkBlocks(inner) => {
                    <validatorAfkBlocksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validatorWhitelistDisabled(inner) => {
                    <validatorWhitelistDisabledCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::wasmModuleRoot(inner) => {
                    <wasmModuleRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawStakerFunds(inner) => {
                    <withdrawStakerFundsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawableFunds(inner) => {
                    <withdrawableFundsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawalAddress(inner) => {
                    <withdrawalAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`IRollupUser`](self) events.
    pub enum IRollupUserEvents {
        AssertionConfirmed(AssertionConfirmed),
        AssertionCreated(AssertionCreated),
        RollupChallengeStarted(RollupChallengeStarted),
        RollupInitialized(RollupInitialized),
        UserStakeUpdated(UserStakeUpdated),
        UserWithdrawableFundsUpdated(UserWithdrawableFundsUpdated),
    }
    #[automatically_derived]
    impl IRollupUserEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                109u8,
                183u8,
                220u8,
                47u8,
                80u8,
                118u8,
                71u8,
                209u8,
                53u8,
                3u8,
                84u8,
                105u8,
                178u8,
                122u8,
                167u8,
                156u8,
                234u8,
                144u8,
                88u8,
                39u8,
                121u8,
                208u8,
                132u8,
                167u8,
                130u8,
                29u8,
                108u8,
                208u8,
                146u8,
                203u8,
                216u8,
                115u8,
            ],
            [
                144u8,
                28u8,
                58u8,
                238u8,
                35u8,
                207u8,
                68u8,
                120u8,
                130u8,
                84u8,
                98u8,
                202u8,
                170u8,
                179u8,
                117u8,
                198u8,
                6u8,
                171u8,
                131u8,
                81u8,
                96u8,
                96u8,
                56u8,
                131u8,
                68u8,
                240u8,
                101u8,
                3u8,
                64u8,
                117u8,
                54u8,
                48u8,
            ],
            [
                167u8,
                64u8,
                175u8,
                20u8,
                197u8,
                110u8,
                78u8,
                4u8,
                166u8,
                23u8,
                177u8,
                222u8,
                30u8,
                178u8,
                13u8,
                231u8,
                50u8,
                112u8,
                222u8,
                203u8,
                170u8,
                234u8,
                209u8,
                79u8,
                20u8,
                42u8,
                171u8,
                243u8,
                3u8,
                142u8,
                90u8,
                226u8,
            ],
            [
                217u8,
                87u8,
                207u8,
                35u8,
                64u8,
                7u8,
                51u8,
                53u8,
                210u8,
                86u8,
                247u8,
                42u8,
                158u8,
                248u8,
                156u8,
                241u8,
                164u8,
                60u8,
                49u8,
                20u8,
                51u8,
                65u8,
                166u8,
                165u8,
                53u8,
                117u8,
                239u8,
                51u8,
                233u8,
                135u8,
                190u8,
                184u8,
            ],
            [
                252u8,
                27u8,
                131u8,
                193u8,
                29u8,
                153u8,
                208u8,
                138u8,
                147u8,
                142u8,
                11u8,
                130u8,
                160u8,
                189u8,
                69u8,
                248u8,
                34u8,
                247u8,
                31u8,
                245u8,
                171u8,
                242u8,
                63u8,
                153u8,
                156u8,
                147u8,
                196u8,
                83u8,
                61u8,
                117u8,
                36u8,
                100u8,
            ],
            [
                252u8,
                66u8,
                130u8,
                155u8,
                41u8,
                194u8,
                89u8,
                167u8,
                55u8,
                10u8,
                181u8,
                108u8,
                143u8,
                105u8,
                252u8,
                226u8,
                59u8,
                95u8,
                53u8,
                26u8,
                156u8,
                225u8,
                81u8,
                218u8,
                69u8,
                50u8,
                129u8,
                153u8,
                62u8,
                192u8,
                9u8,
                12u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for IRollupUserEvents {
        const NAME: &'static str = "IRollupUserEvents";
        const COUNT: usize = 6usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <AssertionConfirmed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <AssertionConfirmed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::AssertionConfirmed)
                }
                Some(<AssertionCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AssertionCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::AssertionCreated)
                }
                Some(
                    <RollupChallengeStarted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RollupChallengeStarted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RollupChallengeStarted)
                }
                Some(
                    <RollupInitialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RollupInitialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RollupInitialized)
                }
                Some(<UserStakeUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <UserStakeUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::UserStakeUpdated)
                }
                Some(
                    <UserWithdrawableFundsUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UserWithdrawableFundsUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::UserWithdrawableFundsUpdated)
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
    impl alloy_sol_types::private::IntoLogData for IRollupUserEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AssertionConfirmed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::AssertionCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RollupChallengeStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RollupInitialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UserStakeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UserWithdrawableFundsUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AssertionConfirmed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::AssertionCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RollupChallengeStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RollupInitialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UserStakeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UserWithdrawableFundsUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IRollupUser`](self) contract instance.

See the [wrapper's documentation](`IRollupUserInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IRollupUserInstance<T, P, N> {
        IRollupUserInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<IRollupUserInstance<T, P, N>>,
    > {
        IRollupUserInstance::<T, P, N>::deploy(provider)
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
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        IRollupUserInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`IRollupUser`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IRollupUser`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IRollupUserInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IRollupUserInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IRollupUserInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IRollupUserInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IRollupUser`](self) contract instance.

See the [wrapper's documentation](`IRollupUserInstance`) for more details.*/
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
        ) -> alloy_contract::Result<IRollupUserInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
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
    impl<T, P: ::core::clone::Clone, N> IRollupUserInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IRollupUserInstance<T, P, N> {
            IRollupUserInstance {
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
    > IRollupUserInstance<T, P, N> {
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
        ///Creates a new call builder for the [`addToDeposit`] function.
        pub fn addToDeposit(
            &self,
            stakerAddress: alloy::sol_types::private::Address,
            expectedWithdrawalAddress: alloy::sol_types::private::Address,
            tokenAmount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, addToDepositCall, N> {
            self.call_builder(
                &addToDepositCall {
                    stakerAddress,
                    expectedWithdrawalAddress,
                    tokenAmount,
                },
            )
        }
        ///Creates a new call builder for the [`amountStaked`] function.
        pub fn amountStaked(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, amountStakedCall, N> {
            self.call_builder(&amountStakedCall { staker })
        }
        ///Creates a new call builder for the [`baseStake`] function.
        pub fn baseStake(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, baseStakeCall, N> {
            self.call_builder(&baseStakeCall {})
        }
        ///Creates a new call builder for the [`bridge`] function.
        pub fn bridge(&self) -> alloy_contract::SolCallBuilder<T, &P, bridgeCall, N> {
            self.call_builder(&bridgeCall {})
        }
        ///Creates a new call builder for the [`chainId`] function.
        pub fn chainId(&self) -> alloy_contract::SolCallBuilder<T, &P, chainIdCall, N> {
            self.call_builder(&chainIdCall {})
        }
        ///Creates a new call builder for the [`challengeManager`] function.
        pub fn challengeManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, challengeManagerCall, N> {
            self.call_builder(&challengeManagerCall {})
        }
        ///Creates a new call builder for the [`confirmAssertion`] function.
        pub fn confirmAssertion(
            &self,
            assertionHash: alloy::sol_types::private::FixedBytes<32>,
            prevAssertionHash: alloy::sol_types::private::FixedBytes<32>,
            confirmState: <AssertionState as alloy::sol_types::SolType>::RustType,
            winningEdgeId: alloy::sol_types::private::FixedBytes<32>,
            prevConfig: <ConfigData as alloy::sol_types::SolType>::RustType,
            inboxAcc: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, confirmAssertionCall, N> {
            self.call_builder(
                &confirmAssertionCall {
                    assertionHash,
                    prevAssertionHash,
                    confirmState,
                    winningEdgeId,
                    prevConfig,
                    inboxAcc,
                },
            )
        }
        ///Creates a new call builder for the [`confirmPeriodBlocks`] function.
        pub fn confirmPeriodBlocks(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, confirmPeriodBlocksCall, N> {
            self.call_builder(&confirmPeriodBlocksCall {})
        }
        ///Creates a new call builder for the [`genesisAssertionHash`] function.
        pub fn genesisAssertionHash(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, genesisAssertionHashCall, N> {
            self.call_builder(&genesisAssertionHashCall {})
        }
        ///Creates a new call builder for the [`getAssertion`] function.
        pub fn getAssertion(
            &self,
            assertionHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAssertionCall, N> {
            self.call_builder(&getAssertionCall { assertionHash })
        }
        ///Creates a new call builder for the [`getAssertionCreationBlockForLogLookup`] function.
        pub fn getAssertionCreationBlockForLogLookup(
            &self,
            assertionHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getAssertionCreationBlockForLogLookupCall,
            N,
        > {
            self.call_builder(
                &getAssertionCreationBlockForLogLookupCall {
                    assertionHash,
                },
            )
        }
        ///Creates a new call builder for the [`getFirstChildCreationBlock`] function.
        pub fn getFirstChildCreationBlock(
            &self,
            assertionHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getFirstChildCreationBlockCall, N> {
            self.call_builder(
                &getFirstChildCreationBlockCall {
                    assertionHash,
                },
            )
        }
        ///Creates a new call builder for the [`getSecondChildCreationBlock`] function.
        pub fn getSecondChildCreationBlock(
            &self,
            assertionHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getSecondChildCreationBlockCall, N> {
            self.call_builder(
                &getSecondChildCreationBlockCall {
                    assertionHash,
                },
            )
        }
        ///Creates a new call builder for the [`getStaker`] function.
        pub fn getStaker(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakerCall, N> {
            self.call_builder(&getStakerCall { staker })
        }
        ///Creates a new call builder for the [`getStakerAddress`] function.
        pub fn getStakerAddress(
            &self,
            stakerNum: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakerAddressCall, N> {
            self.call_builder(&getStakerAddressCall { stakerNum })
        }
        ///Creates a new call builder for the [`getValidators`] function.
        pub fn getValidators(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getValidatorsCall, N> {
            self.call_builder(&getValidatorsCall {})
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            stakeToken: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall { stakeToken })
        }
        ///Creates a new call builder for the [`isFirstChild`] function.
        pub fn isFirstChild(
            &self,
            assertionHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, isFirstChildCall, N> {
            self.call_builder(&isFirstChildCall { assertionHash })
        }
        ///Creates a new call builder for the [`isPending`] function.
        pub fn isPending(
            &self,
            assertionHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, isPendingCall, N> {
            self.call_builder(&isPendingCall { assertionHash })
        }
        ///Creates a new call builder for the [`isStaked`] function.
        pub fn isStaked(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isStakedCall, N> {
            self.call_builder(&isStakedCall { staker })
        }
        ///Creates a new call builder for the [`isValidator`] function.
        pub fn isValidator(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isValidatorCall, N> {
            self.call_builder(&isValidatorCall { _0 })
        }
        ///Creates a new call builder for the [`latestConfirmed`] function.
        pub fn latestConfirmed(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, latestConfirmedCall, N> {
            self.call_builder(&latestConfirmedCall {})
        }
        ///Creates a new call builder for the [`latestStakedAssertion`] function.
        pub fn latestStakedAssertion(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, latestStakedAssertionCall, N> {
            self.call_builder(
                &latestStakedAssertionCall {
                    staker,
                },
            )
        }
        ///Creates a new call builder for the [`loserStakeEscrow`] function.
        pub fn loserStakeEscrow(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, loserStakeEscrowCall, N> {
            self.call_builder(&loserStakeEscrowCall {})
        }
        ///Creates a new call builder for the [`minimumAssertionPeriod`] function.
        pub fn minimumAssertionPeriod(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, minimumAssertionPeriodCall, N> {
            self.call_builder(&minimumAssertionPeriodCall {})
        }
        ///Creates a new call builder for the [`newStake`] function.
        pub fn newStake(
            &self,
            tokenAmount: alloy::sol_types::private::primitives::aliases::U256,
            withdrawalAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, newStakeCall, N> {
            self.call_builder(
                &newStakeCall {
                    tokenAmount,
                    withdrawalAddress,
                },
            )
        }
        ///Creates a new call builder for the [`newStakeOnNewAssertion_0`] function.
        pub fn newStakeOnNewAssertion_0(
            &self,
            tokenAmount: alloy::sol_types::private::primitives::aliases::U256,
            assertion: <AssertionInputs as alloy::sol_types::SolType>::RustType,
            expectedAssertionHash: alloy::sol_types::private::FixedBytes<32>,
            withdrawalAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, newStakeOnNewAssertion_0Call, N> {
            self.call_builder(
                &newStakeOnNewAssertion_0Call {
                    tokenAmount,
                    assertion,
                    expectedAssertionHash,
                    withdrawalAddress,
                },
            )
        }
        ///Creates a new call builder for the [`newStakeOnNewAssertion_1`] function.
        pub fn newStakeOnNewAssertion_1(
            &self,
            tokenAmount: alloy::sol_types::private::primitives::aliases::U256,
            assertion: <AssertionInputs as alloy::sol_types::SolType>::RustType,
            expectedAssertionHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, newStakeOnNewAssertion_1Call, N> {
            self.call_builder(
                &newStakeOnNewAssertion_1Call {
                    tokenAmount,
                    assertion,
                    expectedAssertionHash,
                },
            )
        }
        ///Creates a new call builder for the [`outbox`] function.
        pub fn outbox(&self) -> alloy_contract::SolCallBuilder<T, &P, outboxCall, N> {
            self.call_builder(&outboxCall {})
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`reduceDeposit`] function.
        pub fn reduceDeposit(
            &self,
            target: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, reduceDepositCall, N> {
            self.call_builder(&reduceDepositCall { target })
        }
        ///Creates a new call builder for the [`removeWhitelistAfterFork`] function.
        pub fn removeWhitelistAfterFork(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeWhitelistAfterForkCall, N> {
            self.call_builder(&removeWhitelistAfterForkCall {})
        }
        ///Creates a new call builder for the [`removeWhitelistAfterValidatorAfk`] function.
        pub fn removeWhitelistAfterValidatorAfk(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            removeWhitelistAfterValidatorAfkCall,
            N,
        > {
            self.call_builder(
                &removeWhitelistAfterValidatorAfkCall {
                },
            )
        }
        ///Creates a new call builder for the [`returnOldDeposit`] function.
        pub fn returnOldDeposit(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, returnOldDepositCall, N> {
            self.call_builder(&returnOldDepositCall {})
        }
        ///Creates a new call builder for the [`returnOldDepositFor`] function.
        pub fn returnOldDepositFor(
            &self,
            stakerAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, returnOldDepositForCall, N> {
            self.call_builder(
                &returnOldDepositForCall {
                    stakerAddress,
                },
            )
        }
        ///Creates a new call builder for the [`rollupEventInbox`] function.
        pub fn rollupEventInbox(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, rollupEventInboxCall, N> {
            self.call_builder(&rollupEventInboxCall {})
        }
        ///Creates a new call builder for the [`sequencerInbox`] function.
        pub fn sequencerInbox(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, sequencerInboxCall, N> {
            self.call_builder(&sequencerInboxCall {})
        }
        ///Creates a new call builder for the [`stakeOnNewAssertion`] function.
        pub fn stakeOnNewAssertion(
            &self,
            assertion: <AssertionInputs as alloy::sol_types::SolType>::RustType,
            expectedAssertionHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, stakeOnNewAssertionCall, N> {
            self.call_builder(
                &stakeOnNewAssertionCall {
                    assertion,
                    expectedAssertionHash,
                },
            )
        }
        ///Creates a new call builder for the [`stakeToken`] function.
        pub fn stakeToken(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, stakeTokenCall, N> {
            self.call_builder(&stakeTokenCall {})
        }
        ///Creates a new call builder for the [`stakerCount`] function.
        pub fn stakerCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, stakerCountCall, N> {
            self.call_builder(&stakerCountCall {})
        }
        ///Creates a new call builder for the [`validateAssertionHash`] function.
        pub fn validateAssertionHash(
            &self,
            assertionHash: alloy::sol_types::private::FixedBytes<32>,
            state: <AssertionState as alloy::sol_types::SolType>::RustType,
            prevAssertionHash: alloy::sol_types::private::FixedBytes<32>,
            inboxAcc: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, validateAssertionHashCall, N> {
            self.call_builder(
                &validateAssertionHashCall {
                    assertionHash,
                    state,
                    prevAssertionHash,
                    inboxAcc,
                },
            )
        }
        ///Creates a new call builder for the [`validateConfig`] function.
        pub fn validateConfig(
            &self,
            assertionHash: alloy::sol_types::private::FixedBytes<32>,
            configData: <ConfigData as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, validateConfigCall, N> {
            self.call_builder(
                &validateConfigCall {
                    assertionHash,
                    configData,
                },
            )
        }
        ///Creates a new call builder for the [`validatorAfkBlocks`] function.
        pub fn validatorAfkBlocks(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, validatorAfkBlocksCall, N> {
            self.call_builder(&validatorAfkBlocksCall {})
        }
        ///Creates a new call builder for the [`validatorWhitelistDisabled`] function.
        pub fn validatorWhitelistDisabled(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, validatorWhitelistDisabledCall, N> {
            self.call_builder(&validatorWhitelistDisabledCall {})
        }
        ///Creates a new call builder for the [`wasmModuleRoot`] function.
        pub fn wasmModuleRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, wasmModuleRootCall, N> {
            self.call_builder(&wasmModuleRootCall {})
        }
        ///Creates a new call builder for the [`withdrawStakerFunds`] function.
        pub fn withdrawStakerFunds(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawStakerFundsCall, N> {
            self.call_builder(&withdrawStakerFundsCall {})
        }
        ///Creates a new call builder for the [`withdrawableFunds`] function.
        pub fn withdrawableFunds(
            &self,
            owner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawableFundsCall, N> {
            self.call_builder(&withdrawableFundsCall { owner })
        }
        ///Creates a new call builder for the [`withdrawalAddress`] function.
        pub fn withdrawalAddress(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawalAddressCall, N> {
            self.call_builder(&withdrawalAddressCall { staker })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IRollupUserInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`AssertionConfirmed`] event.
        pub fn AssertionConfirmed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AssertionConfirmed, N> {
            self.event_filter::<AssertionConfirmed>()
        }
        ///Creates a new event filter for the [`AssertionCreated`] event.
        pub fn AssertionCreated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AssertionCreated, N> {
            self.event_filter::<AssertionCreated>()
        }
        ///Creates a new event filter for the [`RollupChallengeStarted`] event.
        pub fn RollupChallengeStarted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RollupChallengeStarted, N> {
            self.event_filter::<RollupChallengeStarted>()
        }
        ///Creates a new event filter for the [`RollupInitialized`] event.
        pub fn RollupInitialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RollupInitialized, N> {
            self.event_filter::<RollupInitialized>()
        }
        ///Creates a new event filter for the [`UserStakeUpdated`] event.
        pub fn UserStakeUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, UserStakeUpdated, N> {
            self.event_filter::<UserStakeUpdated>()
        }
        ///Creates a new event filter for the [`UserWithdrawableFundsUpdated`] event.
        pub fn UserWithdrawableFundsUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, UserWithdrawableFundsUpdated, N> {
            self.event_filter::<UserWithdrawableFundsUpdated>()
        }
    }
}
