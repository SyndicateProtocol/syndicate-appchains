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

interface RollupUserLogic {
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

    event AdminChanged(address previousAdmin, address newAdmin);
    event AssertionConfirmed(bytes32 indexed assertionHash, bytes32 blockHash, bytes32 sendRoot);
    event AssertionCreated(bytes32 indexed assertionHash, bytes32 indexed parentAssertionHash, AssertionInputs assertion, bytes32 afterInboxBatchAcc, uint256 inboxMaxCount, bytes32 wasmModuleRoot, uint256 requiredStake, address challengeManager, uint64 confirmPeriodBlocks);
    event BeaconUpgraded(address indexed beacon);
    event Initialized(uint8 version);
    event Paused(address account);
    event RollupChallengeStarted(uint64 indexed challengeIndex, address asserter, address challenger, uint64 challengedAssertion);
    event RollupInitialized(bytes32 machineHash, uint256 chainId);
    event Unpaused(address account);
    event Upgraded(address indexed implementation);
    event UpgradedSecondary(address indexed implementation);
    event UserStakeUpdated(address indexed user, address indexed withdrawalAddress, uint256 initialBalance, uint256 finalBalance);
    event UserWithdrawableFundsUpdated(address indexed user, uint256 initialBalance, uint256 finalBalance);

    function _stakerMap(address) external view returns (uint256 amountStaked, bytes32 latestStakedAssertion, uint64 index, bool isStaked, address withdrawalAddress);
    function addToDeposit(address stakerAddress, address expectedWithdrawalAddress, uint256 tokenAmount) external;
    function amountStaked(address staker) external view returns (uint256);
    function anyTrustFastConfirmer() external view returns (address);
    function baseStake() external view returns (uint256);
    function bridge() external view returns (address);
    function chainId() external view returns (uint256);
    function challengeGracePeriodBlocks() external view returns (uint64);
    function challengeManager() external view returns (address);
    function computeAssertionHash(bytes32 prevAssertionHash, AssertionState memory state, bytes32 inboxAcc) external pure returns (bytes32);
    function confirmAssertion(bytes32 assertionHash, bytes32 prevAssertionHash, AssertionState memory confirmState, bytes32 winningEdgeId, ConfigData memory prevConfig, bytes32 inboxAcc) external;
    function confirmPeriodBlocks() external view returns (uint64);
    function fastConfirmAssertion(bytes32 assertionHash, bytes32 parentAssertionHash, AssertionState memory confirmState, bytes32 inboxAcc) external;
    function fastConfirmNewAssertion(AssertionInputs memory assertion, bytes32 expectedAssertionHash) external;
    function genesisAssertionHash() external pure returns (bytes32);
    function getAssertion(bytes32 assertionHash) external view returns (AssertionNode memory);
    function getAssertionCreationBlockForLogLookup(bytes32 assertionHash) external view returns (uint256);
    function getFirstChildCreationBlock(bytes32 assertionHash) external view returns (uint64);
    function getSecondChildCreationBlock(bytes32 assertionHash) external view returns (uint64);
    function getStaker(address staker) external view returns (IRollupCore.Staker memory);
    function getStakerAddress(uint64 stakerNum) external view returns (address);
    function getValidators() external view returns (address[] memory);
    function inbox() external view returns (address);
    function initialize(address _stakeToken) external view;
    function isFirstChild(bytes32 assertionHash) external view returns (bool);
    function isPending(bytes32 assertionHash) external view returns (bool);
    function isStaked(address staker) external view returns (bool);
    function isValidator(address validator) external view returns (bool);
    function latestConfirmed() external view returns (bytes32);
    function latestStakedAssertion(address staker) external view returns (bytes32);
    function loserStakeEscrow() external view returns (address);
    function minimumAssertionPeriod() external view returns (uint256);
    function newStake(uint256 tokenAmount, address _withdrawalAddress) external;
    function newStakeOnNewAssertion(uint256 tokenAmount, AssertionInputs memory assertion, bytes32 expectedAssertionHash, address _withdrawalAddress) external;
    function newStakeOnNewAssertion(uint256 tokenAmount, AssertionInputs memory assertion, bytes32 expectedAssertionHash) external;
    function outbox() external view returns (address);
    function owner() external view returns (address);
    function paused() external view returns (bool);
    function proxiableUUID() external view returns (bytes32);
    function reduceDeposit(uint256 target) external;
    function removeWhitelistAfterFork() external;
    function removeWhitelistAfterValidatorAfk() external;
    function returnOldDeposit() external;
    function returnOldDepositFor(address stakerAddress) external;
    function rollupDeploymentBlock() external view returns (uint256);
    function rollupEventInbox() external view returns (address);
    function sequencerInbox() external view returns (address);
    function stakeOnNewAssertion(AssertionInputs memory assertion, bytes32 expectedAssertionHash) external;
    function stakeToken() external view returns (address);
    function stakerCount() external view returns (uint64);
    function totalWithdrawableFunds() external view returns (uint256);
    function validateAssertionHash(bytes32 assertionHash, AssertionState memory state, bytes32 prevAssertionHash, bytes32 inboxAcc) external pure;
    function validateConfig(bytes32 assertionHash, ConfigData memory configData) external view;
    function validatorAfkBlocks() external view returns (uint64);
    function validatorWalletCreator() external view returns (address);
    function validatorWhitelistDisabled() external view returns (bool);
    function wasmModuleRoot() external view returns (bytes32);
    function withdrawStakerFunds() external returns (uint256);
    function withdrawableFunds(address user) external view returns (uint256);
    function withdrawalAddress(address staker) external view returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "_stakerMap",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
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
    ],
    "stateMutability": "view"
  },
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
    "name": "anyTrustFastConfirmer",
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
    "name": "challengeGracePeriodBlocks",
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
    "name": "computeAssertionHash",
    "inputs": [
      {
        "name": "prevAssertionHash",
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
        "name": "inboxAcc",
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
    "stateMutability": "pure"
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
    "name": "fastConfirmAssertion",
    "inputs": [
      {
        "name": "assertionHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "parentAssertionHash",
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
    "name": "fastConfirmNewAssertion",
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
    "name": "inbox",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IInboxBase"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "_stakeToken",
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
        "name": "validator",
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
        "name": "_withdrawalAddress",
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
        "name": "_withdrawalAddress",
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
    "name": "paused",
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
    "name": "proxiableUUID",
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
    "name": "rollupDeploymentBlock",
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
    "name": "totalWithdrawableFunds",
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
    "stateMutability": "pure"
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
    "name": "validatorWalletCreator",
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
        "name": "user",
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
    "name": "AdminChanged",
    "inputs": [
      {
        "name": "previousAdmin",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newAdmin",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
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
    "name": "BeaconUpgraded",
    "inputs": [
      {
        "name": "beacon",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Paused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": false,
        "internalType": "address"
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
    "name": "Unpaused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Upgraded",
    "inputs": [
      {
        "name": "implementation",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UpgradedSecondary",
    "inputs": [
      {
        "name": "implementation",
        "type": "address",
        "indexed": true,
        "internalType": "address"
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
pub mod RollupUserLogic {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60e060405261000c61002a565b15156080523060a0524660c052348015610024575f80fd5b506100d4565b60408051600481526024810182526020810180516001600160e01b03166302881c7960e11b17905290515f918291829160649161006791906100be565b5f60405180830381855afa9150503d805f811461009f576040519150601f19603f3d011682016040523d82523d5f602084013e6100a4565b606091505b50915091508180156100b7575080516020145b9250505090565b5f82518060208501845e5f920191825250919050565b60805160a05160c0516149866101135f395f611c0b01525f818161152001528181611c810152611d1701525f8181610dfb0152612fab01526149865ff3fe608060405234801561000f575f80fd5b5060043610610393575f3560e01c806365f7f80d116101df578063bc45e0ae11610109578063e6b3082c116100a9578063ef40a67011610079578063ef40a67014610982578063f065de3f146109aa578063facd743b146109bd578063fb0e722b146109d0575f80fd5b8063e6b3082c146108b1578063e78cea92146108cc578063e8bd4922146108df578063ee35f3271461097a575f80fd5b8063ce11e6ab116100e4578063ce11e6ab14610870578063dff6978714610883578063e51019a61461088b578063e531d8c71461089e575f80fd5b8063bc45e0ae14610842578063c2c2e68e14610855578063c4d66de81461085d575f80fd5b806384728cd01161017f5780639a8a05921161014f5780639a8a05921461070e578063a23c44b114610717578063aa38a6e71461081a578063b7ab4db51461082d575f80fd5b806384728cd0146106a257806388302884146106dd5780638da5cb5b146106fd5780638ee1a12614610705575f80fd5b80636ddd3744116101ba5780636ddd37441461066a57806371ef232c1461067d5780637300201c1461068657806376e7e23b14610699575f80fd5b806365f7f80d1461063c57806368129b1414610644578063685f5ecc14610657575f80fd5b80633b86de19116102c057806356bbc9e6116102605780636096686d116102305780636096686d146105d957806361373919146105ec5780636177fd18146105f45780636420fb9f14610629575f80fd5b806356bbc9e6146105a057806357ef4ab9146105b3578063588c7a16146105bb5780635c975abb146105ce575f80fd5b806350f32f681161029b57806350f32f681461055a57806351ed6a301461056d57806352d1902d1461058057806355840a5814610588575f80fd5b80633b86de19146105125780633be680ea1461052557806345e38b6414610551575f80fd5b80631b1689e9116103365780632f30cabd116103065780632f30cabd146104bc57806330836228146104e457806333635fc2146104f7578063353325e01461050a575f80fd5b80631b1689e9146104615780631e83d30f1461046a5780632abdd2301461047d5780632e7acfa6146104a8575f80fd5b8063117155851161037157806311715585146103ef57806312ab3d3b1461041b57806313c56ca71461043857806318baaab914610459575f80fd5b8063023a96fe1461039757806304972af9146103c757806310b98a35146103dc575b5f80fd5b6069546103aa906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b6103da6103d5366004613f52565b6109e3565b005b6103da6103ea366004613f8d565b6109fd565b6104026103fd366004613fe7565b610ddd565b60405167ffffffffffffffff90911681526020016103be565b607b546104289060ff1681565b60405190151581526020016103be565b61044b610446366004613fe7565b610df8565b6040519081526020016103be565b6103da610f3d565b61044b607a5481565b6103da610478366004613fe7565b611011565b61044b61048b366004614012565b6001600160a01b03165f9081526077602052604090206001015490565b6066546104029067ffffffffffffffff1681565b61044b6104ca366004614012565b6001600160a01b03165f9081526078602052604090205490565b6104286104f2366004613fe7565b61108b565b61044b61050536600461402d565b6110a6565b61044b6110ca565b6103da610520366004614072565b611104565b6069546104029074010000000000000000000000000000000000000000900467ffffffffffffffff1681565b61044b60715481565b6103da61056836600461409d565b61149b565b6070546103aa906001600160a01b031681565b61044b611514565b607b546103aa9061010090046001600160a01b031681565b6104026105ae366004613fe7565b6115d8565b6103da6115fa565b6103da6105c9366004614012565b611668565b60335460ff16610428565b6103da6105e73660046140e6565b611756565b61044b6117c9565b610428610602366004614012565b6001600160a01b03165f90815260776020526040902060020154600160401b900460ff1690565b6103da610637366004614072565b611849565b60745461044b565b6103da610652366004614123565b6119fe565b6103da610665366004614151565b611a75565b6103aa6106783660046141a4565b611a91565b61044b60795481565b6103da6106943660046141bf565b611ac9565b61044b60675481565b6103aa6106b0366004614012565b6001600160a01b039081165f90815260776020526040902060020154690100000000000000000090041690565b6106f06106eb366004613fe7565b611ad5565b6040516103be9190614224565b6103aa611ba0565b61044b60685481565b61044b60655481565b6107c4610725366004614012565b6040805160a0810182525f80825260208201819052918101829052606081018290526080810191909152506001600160a01b039081165f90815260776020908152604091829020825160a081018452815481526001820154928101929092526002015467ffffffffffffffff81169282019290925260ff600160401b830416151560608201526901000000000000000000909104909116608082015290565b6040516103be91905f60a082019050825182526020830151602083015267ffffffffffffffff60408401511660408301526060830151151560608301526001600160a01b03608084015116608083015292915050565b606d546103aa906001600160a01b031681565b610835611ba9565b6040516103be9190614284565b606e546103aa906001600160a01b031681565b6103da611bb5565b6103da61086b366004614012565b611c77565b606c546103aa906001600160a01b031681565b607654610402565b6103da6108993660046142d0565b611e19565b6104286108ac366004613fe7565b611e80565b60665461040290600160401b900467ffffffffffffffff1681565b606b546103aa906001600160a01b031681565b61093a6108ed366004614012565b60776020525f908152604090208054600182015460029092015490919067ffffffffffffffff811690600160401b810460ff1690690100000000000000000090046001600160a01b031685565b60408051958652602086019490945267ffffffffffffffff9092169284019290925290151560608301526001600160a01b0316608082015260a0016103be565b6103aa611eaf565b61044b610990366004614012565b6001600160a01b03165f9081526077602052604090205490565b606f546103aa906001600160a01b031681565b6104286109cb366004614012565b611f33565b606a546103aa906001600160a01b031681565b6109f9816109f084611f52565b60010154611fb1565b5050565b335f81815260736020526040902054151580610a1b5750607b5460ff165b610a5c5760405162461bcd60e51b815260206004820152600d60248201526c2727aa2fab20a624a220aa27a960991b60448201526064015b60405180910390fd5b610a646120b6565b5f610a6e88611f52565b90505f610a7a88611f52565b9050610a8a858260010154611fb1565b610a9a60808601606087016141a4565b8254610ab79190600160801b900467ffffffffffffffff1661431f565b67ffffffffffffffff16431015610b105760405162461bcd60e51b815260206004820152600f60248201527f4245464f52455f444541444c494e4500000000000000000000000000000000006044820152606401610a53565b6074548814610b615760405162461bcd60e51b815260206004820152601960248201527f505245565f4e4f545f4c41544553545f434f4e4649524d4544000000000000006044820152606401610a53565b8054600160401b900467ffffffffffffffff1615610dc6575f610b8a6060870160408801614012565b6001600160a01b031663fda2892e886040518263ffffffff1660e01b8152600401610bb791815260200190565b6101e060405180830381865afa158015610bd3573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610bf791906143e4565b9050898160e0015114610c4c5760405162461bcd60e51b815260206004820152600a60248201527f4e4f545f57494e4e4552000000000000000000000000000000000000000000006044820152606401610a53565b60018161016001516001811115610c6557610c656141f4565b14610cb25760405162461bcd60e51b815260206004820152601260248201527f454447455f4e4f545f434f4e4649524d454400000000000000000000000000006044820152606401610a53565b80610140015167ffffffffffffffff165f03610d105760405162461bcd60e51b815260206004820152601760248201527f5a45524f5f434f4e4649524d45445f41545f424c4f434b0000000000000000006044820152606401610a53565b606954610140820151610d459174010000000000000000000000000000000000000000900467ffffffffffffffff169061431f565b67ffffffffffffffff16431015610dc45760405162461bcd60e51b815260206004820152602160248201527f4348414c4c454e47455f47524143455f504552494f445f4e4f545f504153534560448201527f44000000000000000000000000000000000000000000000000000000000000006064820152608401610a53565b505b610dd28989898761210b565b505050505050505050565b5f610de782611f52565b5467ffffffffffffffff1692915050565b5f7f000000000000000000000000000000000000000000000000000000000000000015610e80575f828152607c602052604090205480610e7a5760405162461bcd60e51b815260206004820152600c60248201527f4e4f5f415353455254494f4e00000000000000000000000000000000000000006044820152606401610a53565b92915050565b5f610e8a83611f52565b6040805160c081018252825467ffffffffffffffff8082168352600160401b820481166020840152600160801b8204169282019290925260ff600160c01b8304811615156060830152929350610f2092909184916080840191600160c81b9004166002811115610efc57610efc6141f4565b6002811115610f0d57610f0d6141f4565b815260200160018201548152505061231a565b54600160801b900467ffffffffffffffff1692915050565b919050565b607b5460ff1615610f905760405162461bcd60e51b815260206004820152601260248201527f57484954454c4953545f44495341424c454400000000000000000000000000006044820152606401610a53565b610f9861237e565b610fe45760405162461bcd60e51b815260206004820152601160248201527f56414c494441544f525f4e4f545f41464b0000000000000000000000000000006044820152606401610a53565b607b80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00166001179055565b335f8181526073602052604090205415158061102f5750607b5460ff165b61106b5760405162461bcd60e51b815260206004820152600d60248201526c2727aa2fab20a624a220aa27a960991b6044820152606401610a53565b6110736120b6565b61107c336124ad565b611086338361259c565b505050565b5f61109582611f52565b54600160c01b900460ff1692915050565b5f6110c0846110ba368690038601866145c7565b8461269b565b90505b9392505050565b5f6110d3613ef9565b60408051606081018252828152600160208201525f91810182905290806110fb81848161269b565b94505050505090565b335f818152607360205260409020541515806111225750607b5460ff165b61115e5760405162461bcd60e51b815260206004820152600d60248201526c2727aa2fab20a624a220aa27a960991b6044820152606401610a53565b6111666120b6565b81158061119657505f61117883611f52565b54600160c81b900460ff166002811115611194576111946141f4565b145b6111e25760405162461bcd60e51b815260206004820152601760248201527f45585045435445445f415353455254494f4e5f5345454e0000000000000000006044820152606401610a53565b335f90815260776020526040902060020154600160401b900460ff166112375760405162461bcd60e51b815260206004820152600a6024820152691393d517d4d51052d15160b21b6044820152606401610a53565b335f90815260776020526040902054606084013511156112995760405162461bcd60e51b815260206004820152601260248201527f494e53554646494349454e545f5354414b4500000000000000000000000000006044820152606401610a53565b5f6112bb84356112b136879003870160e088016145c7565b602087013561269b565b90506113346112c982611f52565b6040805160c081018252825467ffffffffffffffff8082168352600160401b820481166020840152600160801b8204169282019290925260ff600160c01b83048116151560608301529092916080840191600160c81b909104166002811115610efc57610efc6141f4565b335f908152607760205260409020600101548181148061136657505f61135982611f52565b5467ffffffffffffffff16115b6113b25760405162461bcd60e51b815260206004820152601860248201527f5354414b45445f4f4e5f414e4f544845525f4252414e434800000000000000006044820152606401610a53565b5f806113bf8785886126de565b335f908152607760205260409020600101829055909250905080611459575f6113e785611f52565b5461140390600160801b900467ffffffffffffffff164361462c565b90506071548110156114575760405162461bcd60e51b815260206004820152600a60248201527f54494d455f44454c5441000000000000000000000000000000000000000000006044820152606401610a53565b505b61146282611f52565b54600160c01b900460ff1661149257606f54607054611492916001600160a01b03918216911660608a013561304b565b50505050505050565b6001600160a01b0381166114f15760405162461bcd60e51b815260206004820152601860248201527f454d5054595f5749544844524157414c5f4144445245535300000000000000006044820152606401610a53565b6114fb84826130f4565b6115058383611104565b61150e846131ca565b50505050565b5f306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146115b35760405162461bcd60e51b815260206004820152603b60248201527f555550534e6f745570677261646561626c653a206d757374206e6f742062652060448201527f63616c6c6564207468726f7567682064656c656761746563616c6c00000000006064820152608401610a53565b507f2b1dbce74324248c222f0ec2d5ed7bd323cfc425b336f0253c5ccfda7265546d90565b5f6115e282611f52565b54600160401b900467ffffffffffffffff1692915050565b335f818152607360205260409020541515806116185750607b5460ff165b6116545760405162461bcd60e51b815260206004820152600d60248201526c2727aa2fab20a624a220aa27a960991b6044820152606401610a53565b61165c6120b6565b611665336131e2565b50565b6001600160a01b0381165f9081526073602052604090205481901515806116915750607b5460ff165b6116cd5760405162461bcd60e51b815260206004820152600d60248201526c2727aa2fab20a624a220aa27a960991b6044820152606401610a53565b6116d56120b6565b6001600160a01b038281165f908152607760205260409020600201546901000000000000000000900416331461174d5760405162461bcd60e51b815260206004820152601660248201527f4e4f545f5749544844524157414c5f41444452455353000000000000000000006044820152606401610a53565b6109f9826131e2565b61175e6120b6565b607b5461010090046001600160a01b031633146117bd5760405162461bcd60e51b815260206004820152601260248201527f4e4f545f464153545f434f4e4649524d455200000000000000000000000000006044820152606401610a53565b61150e8484848461210b565b5f6117d26120b6565b5f6117dc336131f4565b90505f811161182d5760405162461bcd60e51b815260206004820152601460248201527f4e4f5f46554e44535f544f5f57495448445241570000000000000000000000006044820152606401610a53565b607054611844906001600160a01b0316338361304b565b905090565b6118516120b6565b8061189e5760405162461bcd60e51b815260206004820152601760248201527f45585045435445445f415353455254494f4e5f484153480000000000000000006044820152606401610a53565b5f6118a882611f52565b54600160c81b900460ff1690505f6118cd84356112b136879003870160e088016145c7565b90506118db6112c982611f52565b5f8260028111156118ee576118ee6141f4565b0361193d575f6118ff8583866126de565b50905061190b81611f52565b54600160c01b900460ff1661193b57606f5460705461193b916001600160a01b039182169116606088013561304b565b505b606b5461150e90849083906101a08801906001600160a01b03166316bf55796001611975611970368d90038d018661463f565b613273565b61197f9190614659565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815267ffffffffffffffff9091166004820152602401602060405180830381865afa1580156119da573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105e7919061467a565b611a066120b6565b6001600160a01b038116611a5c5760405162461bcd60e51b815260206004820152601860248201527f454d5054595f5749544844524157414c5f4144445245535300000000000000006044820152606401610a53565b611a6682826130f4565b81156109f9576109f9826131ca565b611a7d6120b6565b611a88838383613287565b611086816131ca565b5f60768267ffffffffffffffff1681548110611aaf57611aaf614691565b5f918252602090912001546001600160a01b031692915050565b6110868383833361149b565b6040805160c0810182525f80825260208201819052918101829052606081018290526080810182905260a0810191909152611b0f82611f52565b6040805160c081018252825467ffffffffffffffff8082168352600160401b820481166020840152600160801b8204169282019290925260ff600160c01b83048116151560608301529092916080840191600160c81b909104166002811115611b7a57611b7a6141f4565b6002811115611b8b57611b8b6141f4565b81526020016001820154815250509050919050565b5f6118446133ed565b6060611844607261341f565b607b5460ff1615611c085760405162461bcd60e51b815260206004820152601260248201527f57484954454c4953545f44495341424c454400000000000000000000000000006044820152606401610a53565b467f000000000000000000000000000000000000000000000000000000000000000003610fe45760405162461bcd60e51b815260206004820152601460248201527f434841494e5f49445f4e4f545f4348414e4745440000000000000000000000006044820152606401610a53565b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163003611d155760405162461bcd60e51b815260206004820152602c60248201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060448201527f64656c656761746563616c6c00000000000000000000000000000000000000006064820152608401610a53565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316611d4761342b565b6001600160a01b031614611dc35760405162461bcd60e51b815260206004820152602c60248201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060448201527f6163746976652070726f787900000000000000000000000000000000000000006064820152608401610a53565b6001600160a01b0381166116655760405162461bcd60e51b815260206004820152601060248201527f4e4545445f5354414b455f544f4b454e000000000000000000000000000000006044820152606401610a53565b611e3282611e2c368690038601866145c7565b8361269b565b841461150e5760405162461bcd60e51b815260206004820152601660248201527f494e56414c49445f415353455254494f4e5f48415348000000000000000000006044820152606401610a53565b5f6001611e8c83611f52565b54600160c81b900460ff166002811115611ea857611ea86141f4565b1492915050565b606b54604080517fee35f32700000000000000000000000000000000000000000000000000000000815290515f926001600160a01b03169163ee35f3279160048083019260209291908290030181865afa158015611f0f573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061184491906146a5565b6001600160a01b0381165f908152607360205260408120541515610e7a565b5f81611fa05760405162461bcd60e51b815260206004820152601b60248201527f415353455254494f4e5f49445f43414e4e4f545f42455f5a45524f00000000006044820152606401610a53565b505f90815260756020526040902090565b61206882356020840135611fcb6060860160408701614012565b611fdb60808701606088016141a4565b611feb60a08801608089016141a4565b60408051602080820197909752808201959095526bffffffffffffffffffffffff19606094851b16938501939093527fffffffffffffffff00000000000000000000000000000000000000000000000060c092831b81166074860152911b16607c8301528051606481840301815260849092019052805191012090565b81146109f95760405162461bcd60e51b815260206004820152601460248201527f434f4e4649475f484153485f4d49534d415443480000000000000000000000006044820152606401610a53565b60335460ff16156121095760405162461bcd60e51b815260206004820152601060248201527f5061757361626c653a20706175736564000000000000000000000000000000006044820152606401610a53565b565b5f61211585611f52565b905060018154600160c81b900460ff166002811115612136576121366141f4565b146121835760405162461bcd60e51b815260206004820152600b60248201527f4e4f545f50454e44494e470000000000000000000000000000000000000000006044820152606401610a53565b612196846110ba368690038601866145c7565b85146121e45760405162461bcd60e51b815260206004820152600c60248201527f434f4e4649524d5f4441544100000000000000000000000000000000000000006044820152606401610a53565b5f6121fc6121f73686900386018661463f565b613452565b90505f6122166122113687900387018761463f565b61345c565b606c546040517fa04cee6000000000000000000000000000000000000000000000000000000000815260048101839052602481018590529192506001600160a01b03169063a04cee60906044015f604051808303815f87803b15801561227a575f80fd5b505af115801561228c573d5f803e3d5ffd5b50505060748890555082547fffffffffffff00ffffffffffffffffffffffffffffffffffffffffffffffffff167902000000000000000000000000000000000000000000000000001783556040805183815260208101839052815189927ffc42829b29c259a7370ab56c8f69fce23b5f351a9ce151da453281993ec0090c928290030190a250505050505050565b5f81608001516002811115612331576123316141f4565b036116655760405162461bcd60e51b815260206004820152601360248201527f415353455254494f4e5f4e4f545f4558495354000000000000000000000000006044820152606401610a53565b5f8061239161238c60745490565b611f52565b6040805160c081018252825467ffffffffffffffff8082168352600160401b820481166020840152600160801b8204169282019290925260ff600160c01b83048116151560608301529092916080840191600160c81b9091041660028111156123fc576123fc6141f4565b600281111561240d5761240d6141f4565b815260019190910154602090910152606654909150600160401b900467ffffffffffffffff165f819003612443575f9250505090565b816040015167ffffffffffffffff165f03612460575f9250505090565b815167ffffffffffffffff1615612492578151439061248a90839067ffffffffffffffff166146c0565b109250505090565b4381836040015167ffffffffffffffff1661248a91906146c0565b6001600160a01b0381165f90815260776020526040902060020154600160401b900460ff1661250b5760405162461bcd60e51b815260206004820152600a6024820152691393d517d4d51052d15160b21b6044820152606401610a53565b6001600160a01b0381165f908152607760205260408120600101546074549091908214908061253984611f52565b5467ffffffffffffffff1611905081806125505750805b61150e5760405162461bcd60e51b815260206004820152600c60248201527f5354414b455f41435449564500000000000000000000000000000000000000006044820152606401610a53565b6001600160a01b038083165f9081526077602052604081206002810154815492939192690100000000000000000090910490911690808511156126215760405162461bcd60e51b815260206004820152601060248201527f544f4f5f4c4954544c455f5354414b45000000000000000000000000000000006044820152606401610a53565b5f61262c868361462c565b868555905061263b8382613467565b826001600160a01b0316876001600160a01b03167fd957cf2340073335d256f72a9ef89cf1a43c31143341a6a53575ef33e987beb88489604051612689929190918252602082015260400190565b60405180910390a39695505050505050565b5f6110c0846126a98561350a565b604080516020808201949094528082019290925260608083018790528151808403909101815260809092019052805191012090565b5f806126f0604086016109f086611f52565b6001612704610240870161022088016146d3565b6002811115612715576127156141f4565b148061274357506002612730610240870161022088016146d3565b6002811115612741576127416141f4565b145b61278f5760405162461bcd60e51b815260206004820152601060248201527f4241445f41465445525f535441545553000000000000000000000000000000006044820152606401610a53565b836127b186356127a736899003890160e08a016145c7565b602089013561269b565b146127fe5760405162461bcd60e51b815260206004820152601460248201527f494e56414c49445f4245464f52455f53544154450000000000000000000000006044820152606401610a53565b6001612812610180870161016088016146d3565b6002811115612823576128236141f4565b146128705760405162461bcd60e51b815260206004820152600f60248201527f4241445f505245565f53544154555300000000000000000000000000000000006044820152606401610a53565b5f61287a85611f52565b90505f806101a0880160e08901826128928383613539565b12156128e05760405162461bcd60e51b815260206004820152600f60248201527f494e424f585f4241434b574152445300000000000000000000000000000000006044820152606401610a53565b5f6129056128f460e08d0160c08e016141a4565b849067ffffffffffffffff1661363f565b90505f8113156129575760405162461bcd60e51b815260206004820152600d60248201527f494e424f585f544f4f5f464152000000000000000000000000000000000000006044820152606401610a53565b600261296b6102408d016102208e016146d3565b600281111561297c5761297c6141f4565b1415801561298957505f81125b156129ea57600196505f61299d8484613539565b136129ea5760405162461bcd60e51b815260206004820152601360248201527f4f564552464c4f575f5354414e445354494c4c000000000000000000000000006044820152606401610a53565b606b54604080517e84120c00000000000000000000000000000000000000000000000000000000815290515f926001600160a01b0316916284120c9160048083019260209291908290030181865afa158015612a48573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612a6c919061467a565b90505f612a79858361363f565b1315612ac75760405162461bcd60e51b815260206004820152600e60248201527f494e424f585f504153545f454e440000000000000000000000000000000000006044820152606401610a53565b80612ad860e08e0160c08f016141a4565b67ffffffffffffffff161115612b305760405162461bcd60e51b815260206004820152601360248201527f494e424f585f4e4f545f504f50554c41544544000000000000000000000000006044820152606401610a53565b5f612b436119703687900387018761463f565b67ffffffffffffffff169050818103612b6857612b618260016146c0565b9650612b6c565b8196505b805f03612bbb5760405162461bcd60e51b815260206004820152601160248201527f454d5054595f494e424f585f434f554e540000000000000000000000000000006044820152606401610a53565b606b546001600160a01b03166316bf5579612bd760018461462c565b6040518263ffffffff1660e01b8152600401612bf591815260200190565b602060405180830381865afa158015612c10573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c34919061467a565b95505050505050612c5487896101a001803603810190611e2c91906145c7565b945085851480612c62575085155b612cae5760405162461bcd60e51b815260206004820152601960248201527f554e45585045435445445f415353455254494f4e5f48415348000000000000006044820152606401610a53565b5f612cb886611f52565b54600160c81b900460ff166002811115612cd457612cd46141f4565b14612d215760405162461bcd60e51b815260206004820152600e60248201527f415353455254494f4e5f5345454e0000000000000000000000000000000000006044820152606401610a53565b825460685460675460695460665460408051602080820196909652808201949094526bffffffffffffffffffffffff19606093841b16838501527fffffffffffffffff00000000000000000000000000000000000000000000000060c092831b8116607486015288831b16607c8501528051606481860301815260848501808352815191870191909120610144860183525f9182905260a4860182905260c4860182905260e4860182905261010486018290526101249095018190528151928301825280835294820185905267ffffffffffffffff43811691830191909152909416159084015260a08301526001608083015250612e1e846136ca565b5f8681526075602090815260409182902083518154928501519385015160608601511515600160c01b027fffffffffffffff00ffffffffffffffffffffffffffffffffffffffffffffffff67ffffffffffffffff928316600160801b02167fffffffffffffff000000000000000000ffffffffffffffffffffffffffffffff968316600160401b027fffffffffffffffffffffffffffffffff00000000000000000000000000000000909616929093169190911793909317939093169290921717808255608083015183929182907fffffffffffff00ffffffffffffffffffffffffffffffffffffffffffffffffff16600160c81b836002811115612f2557612f256141f4565b021790555060a0820151816001015590505087867f901c3aee23cf4478825462caaab375c606ab83516060388344f06503407536308b858760685460675460695f9054906101000a90046001600160a01b031660665f9054906101000a900467ffffffffffffffff16604051612fa19796959493929190614763565b60405180910390a37f00000000000000000000000000000000000000000000000000000000000000001561303f5760646001600160a01b031663a3b1b31d6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561300c573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613030919061467a565b5f878152607c60205260409020555b50505050935093915050565b6040516001600160a01b0383166024820152604481018290526110869084907fa9059cbb00000000000000000000000000000000000000000000000000000000906064015b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fffffffff000000000000000000000000000000000000000000000000000000009093169290921790915261374c565b335f818152607360205260409020541515806131125750607b5460ff165b61314e5760405162461bcd60e51b815260206004820152600d60248201526c2727aa2fab20a624a220aa27a960991b6044820152606401610a53565b6131566120b6565b335f90815260776020526040902060020154600160401b900460ff16156131bf5760405162461bcd60e51b815260206004820152600e60248201527f414c52454144595f5354414b45440000000000000000000000000000000000006044820152606401610a53565b611086338484613830565b607054611665906001600160a01b03163330846139ab565b6131eb816124ad565b611665816139fc565b6001600160a01b0381165f90815260786020526040812080549082905560798054829190849061322590849061462c565b9091555050604080518281525f60208201526001600160a01b038516917fa740af14c56e4e04a617b1de1eb20de73270decbaaead14f142aabf3038e5ae2910160405180910390a292915050565b60208101515f90815b602002015192915050565b6001600160a01b0383165f9081526073602052604090205483901515806132b05750607b5460ff165b6132ec5760405162461bcd60e51b815260206004820152600d60248201526c2727aa2fab20a624a220aa27a960991b6044820152606401610a53565b6132f46120b6565b6001600160a01b0384165f90815260776020526040902060020154600160401b900460ff166133525760405162461bcd60e51b815260206004820152600a6024820152691393d517d4d51052d15160b21b6044820152606401610a53565b826001600160a01b031661338d856001600160a01b039081165f90815260776020526040902060020154690100000000000000000090041690565b6001600160a01b0316146133e35760405162461bcd60e51b815260206004820152601860248201527f57524f4e475f5749544844524157414c5f4144445245535300000000000000006044820152606401610a53565b61150e8483613a85565b5f7fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61035b546001600160a01b0316919050565b60605f6110c383613b12565b5f7f2b1dbce74324248c222f0ec2d5ed7bd323cfc425b336f0253c5ccfda7265546d613410565b80515f908161327c565b80515f90600161327c565b6001600160a01b0382165f908152607860205260408120549061348a83836146c0565b6001600160a01b0385165f9081526078602052604081208290556079805492935085929091906134bb9084906146c0565b909155505060408051838152602081018390526001600160a01b038616917fa740af14c56e4e04a617b1de1eb20de73270decbaaead14f142aabf3038e5ae2910160405180910390a250505050565b5f8160405160200161351c919061484e565b604051602081830303815290604052805190602001209050919050565b5f8061354d6119703686900386018661463f565b90505f6135626119703686900386018661463f565b90508067ffffffffffffffff168267ffffffffffffffff16101561358b575f1992505050610e7a565b8067ffffffffffffffff168267ffffffffffffffff1611156135b257600192505050610e7a565b5f6135ca6135c53688900388018861463f565b613b6b565b90505f6135df6135c53688900388018861463f565b90508067ffffffffffffffff168267ffffffffffffffff16101561360a575f19945050505050610e7a565b8067ffffffffffffffff168267ffffffffffffffff161115613633576001945050505050610e7a565b5f945050505050610e7a565b5f806136536119703686900386018661463f565b9050828167ffffffffffffffff161015613671575f19915050610e7a565b828167ffffffffffffffff16111561368d576001915050610e7a565b5f6136a06135c53687900387018761463f565b67ffffffffffffffff1611156136ba576001915050610e7a565b5f915050610e7a565b5092915050565b805467ffffffffffffffff165f036136f957805467ffffffffffffffff19164367ffffffffffffffff16179055565b8054600160401b900467ffffffffffffffff165f036116655780547fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff16600160401b4367ffffffffffffffff1602179055565b5f6137a0826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b0316613b799092919063ffffffff16565b80519091501561108657808060200190518101906137be91906148d8565b6110865760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e60448201527f6f742073756363656564000000000000000000000000000000000000000000006064820152608401610a53565b6076805460018082019092557fb5732705f5241370a28908c2fe1303cb223f03b90d857fd0573f003f79fefed4810180547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b038781169182179092556040805160a081018252878152607454602080830191825267ffffffffffffffff808816848601908152606085018a81528b8916608087018181525f8a8152607787528981209851895596519c88019c909c5591516002909601805491519b51969093167fffffffffffffffffffffffffffffffffffffffffffffff00000000000000000090911617600160401b9a15159a909a02999099177fffffff0000000000000000000000000000000000000000ffffffffffffffffff166901000000000000000000949097169390930295909517909155815190815292830187905292939290917fd957cf2340073335d256f72a9ef89cf1a43c31143341a6a53575ef33e987beb891015b60405180910390a350505050565b6040516001600160a01b038085166024830152831660448201526064810182905261150e9085907f23b872dd0000000000000000000000000000000000000000000000000000000090608401613090565b6001600160a01b038082165f90815260776020526040902060028101548154919269010000000000000000009091041690613a378282613467565b613a4084613b87565b604080518281525f60208201526001600160a01b0380851692908716917fd957cf2340073335d256f72a9ef89cf1a43c31143341a6a53575ef33e987beb8910161399d565b6001600160a01b0382165f90815260776020526040812080549091613aaa84836146c0565b808455600284015460408051858152602081018490529293506001600160a01b036901000000000000000000909204821692918816917fd957cf2340073335d256f72a9ef89cf1a43c31143341a6a53575ef33e987beb8910160405180910390a35050505050565b6060815f01805480602002602001604051908101604052809291908181526020018280548015613b5f57602002820191905f5260205f20905b815481526020019060010190808311613b4b575b50505050509050919050565b60208101515f90600161327c565b60606110c084845f85613d7c565b6001600160a01b0381165f9081526077602052604090206002810154600160401b900460ff16613be65760405162461bcd60e51b815260206004820152600a6024820152691393d517d4d51052d15160b21b6044820152606401610a53565b60028101546076805467ffffffffffffffff90921691613c089060019061462c565b81548110613c1857613c18614691565b5f91825260209091200154607680546001600160a01b039092169167ffffffffffffffff8416908110613c4d57613c4d614691565b905f5260205f20015f6101000a8154816001600160a01b0302191690836001600160a01b031602179055508060775f60768467ffffffffffffffff1681548110613c9957613c99614691565b5f918252602080832091909101546001600160a01b031683528201929092526040019020600201805467ffffffffffffffff191667ffffffffffffffff929092169190911790556076805480613cf157613cf16148f1565b5f828152602080822083015f1990810180547fffffffffffffffffffffffff00000000000000000000000000000000000000001690559092019092556001600160a01b039490941681526077909352505060408120818155600181019190915560020180547fffffff0000000000000000000000000000000000000000000000000000000000169055565b606082471015613df45760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f60448201527f722063616c6c00000000000000000000000000000000000000000000000000006064820152608401610a53565b6001600160a01b0385163b613e4b5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401610a53565b5f80866001600160a01b03168587604051613e669190614905565b5f6040518083038185875af1925050503d805f8114613ea0576040519150601f19603f3d011682016040523d82523d5f602084013e613ea5565b606091505b5091509150613eb5828286613ec0565b979650505050505050565b60608315613ecf5750816110c3565b825115613edf5782518084602001fd5b8160405162461bcd60e51b8152600401610a53919061491b565b6040518060400160405280613f0c613f1e565b8152602001613f19613f1e565b905290565b60405180604001604052806002906020820280368337509192915050565b5f60a08284031215613f4c575f80fd5b50919050565b5f8060c08385031215613f63575f80fd5b82359150613f748460208501613f3c565b90509250929050565b5f60c08284031215613f4c575f80fd5b5f805f805f806101e08789031215613fa3575f80fd5b8635955060208701359450613fbb8860408901613f7d565b93506101008701359250613fd3886101208901613f3c565b91506101c087013590509295509295509295565b5f60208284031215613ff7575f80fd5b5035919050565b6001600160a01b0381168114611665575f80fd5b5f60208284031215614022575f80fd5b81356110c381613ffe565b5f805f6101008486031215614040575f80fd5b833592506140518560208601613f7d565b915060e084013590509250925092565b5f6102608284031215613f4c575f80fd5b5f806102808385031215614084575f80fd5b61408e8484614061565b94610260939093013593505050565b5f805f806102c085870312156140b1575f80fd5b843593506140c28660208701614061565b925061028085013591506102a08501356140db81613ffe565b939692955090935050565b5f805f8061012085870312156140fa575f80fd5b84359350602085013592506141128660408701613f7d565b939692955092936101000135925050565b5f8060408385031215614134575f80fd5b82359150602083013561414681613ffe565b809150509250929050565b5f805f60608486031215614163575f80fd5b833561416e81613ffe565b9250602084013561417e81613ffe565b929592945050506040919091013590565b67ffffffffffffffff81168114611665575f80fd5b5f602082840312156141b4575f80fd5b81356110c38161418f565b5f805f6102a084860312156141d2575f80fd5b833592506141e38560208601614061565b915061028084013590509250925092565b634e487b7160e01b5f52602160045260245ffd5b6003811061166557634e487b7160e01b5f52602160045260245ffd5b5f60c08201905067ffffffffffffffff80845116835280602085015116602084015280604085015116604084015250606083015115156060830152608083015161426d81614208565b8060808401525060a083015160a083015292915050565b602080825282518282018190525f9190848201906040850190845b818110156142c45783516001600160a01b03168352928401929184019160010161429f565b50909695505050505050565b5f805f8061012085870312156142e4575f80fd5b843593506142f58660208701613f7d565b939693955050505060e082013591610100013590565b634e487b7160e01b5f52601160045260245ffd5b67ffffffffffffffff8181168382160190808211156136c3576136c361430b565b634e487b7160e01b5f52604160045260245ffd5b6040516101e0810167ffffffffffffffff8111828210171561437857614378614340565b60405290565b6040805190810167ffffffffffffffff8111828210171561437857614378614340565b8051610f3881613ffe565b8051610f388161418f565b805160028110610f38575f80fd5b805160ff81168114610f38575f80fd5b80518015158114610f38575f80fd5b5f6101e082840312156143f5575f80fd5b6143fd614354565b825181526020830151602082015260408301516040820152606083015160608201526080830151608082015260a083015160a082015260c083015160c082015260e083015160e08201526101006144558185016143a1565b908201526101206144678482016143ac565b908201526101406144798482016143ac565b9082015261016061448b8482016143b7565b9082015261018061449d8482016143c5565b908201526101a06144af8482016143d5565b908201526101c06144c18482016143ac565b908201529392505050565b5f82601f8301126144db575f80fd5b6144e361437e565b8060408401858111156144f4575f80fd5b845b818110156145175780356145098161418f565b8452602093840193016144f6565b509095945050505050565b5f60808284031215614532575f80fd5b6040516040810181811067ffffffffffffffff8211171561455557614555614340565b604052905080601f83018413614569575f80fd5b61457161437e565b806040850186811115614582575f80fd5b855b8181101561459c578035845260209384019301614584565b508184526145aa87826144cc565b60208501525050505092915050565b803560038110610f38575f80fd5b5f60c082840312156145d7575f80fd5b6040516060810181811067ffffffffffffffff821117156145fa576145fa614340565b6040526146078484614522565b8152614615608084016145b9565b602082015260a09290920135604083015250919050565b81810381811115610e7a57610e7a61430b565b5f6080828403121561464f575f80fd5b6110c38383614522565b67ffffffffffffffff8281168282160390808211156136c3576136c361430b565b5f6020828403121561468a575f80fd5b5051919050565b634e487b7160e01b5f52603260045260245ffd5b5f602082840312156146b5575f80fd5b81516110c381613ffe565b80820180821115610e7a57610e7a61430b565b5f602082840312156146e3575f80fd5b6110c3826145b9565b6146f581614208565b9052565b604081833760408201604082015f5b600281101561473b57813561471c8161418f565b67ffffffffffffffff1683526020928301929190910190600101614708565b50505061474a608082016145b9565b61475381614208565b608083015260a090810135910152565b5f6103208201905088358252602089013560208301526040890135604083015260608901356060830152608089013561479b81613ffe565b6001600160a01b0316608083015260a08901356147b78161418f565b67ffffffffffffffff90811660a084015260c08a0135906147d78261418f565b1660c08301526147ed60e0808401908b016146f9565b6101a06147fe818401828c016146f9565b508761026083015286610280830152856102a0830152846102c08301526148316102e08301856001600160a01b03169052565b67ffffffffffffffff831661030083015298975050505050505050565b8151805160c083019190835f5b600281101561487a57825182526020928301929091019060010161485b565b50505060209081015190604084015f5b60028110156148b157835167ffffffffffffffff168252928201929082019060010161488a565b5050505060208301516148c760808401826146ec565b50604083015160a083015292915050565b5f602082840312156148e8575f80fd5b6110c3826143d5565b634e487b7160e01b5f52603160045260245ffd5b5f82518060208501845e5f920191825250919050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f8301168401019150509291505056fea2646970667358221220b02a75c27b281ab51b1a9b90faaa010271b8993b32b302599560ad12cddd57e464736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@Ra\0\x0Ca\0*V[\x15\x15`\x80R0`\xA0RF`\xC0R4\x80\x15a\0$W_\x80\xFD[Pa\0\xD4V[`@\x80Q`\x04\x81R`$\x81\x01\x82R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x02\x88\x1Cy`\xE1\x1B\x17\x90R\x90Q_\x91\x82\x91\x82\x91`d\x91a\0g\x91\x90a\0\xBEV[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a\0\x9FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\0\xA4V[``\x91P[P\x91P\x91P\x81\x80\x15a\0\xB7WP\x80Q` \x14[\x92PPP\x90V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[`\x80Q`\xA0Q`\xC0QaI\x86a\x01\x13_9_a\x1C\x0B\x01R_\x81\x81a\x15 \x01R\x81\x81a\x1C\x81\x01Ra\x1D\x17\x01R_\x81\x81a\r\xFB\x01Ra/\xAB\x01RaI\x86_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x03\x93W_5`\xE0\x1C\x80ce\xF7\xF8\r\x11a\x01\xDFW\x80c\xBCE\xE0\xAE\x11a\x01\tW\x80c\xE6\xB3\x08,\x11a\0\xA9W\x80c\xEF@\xA6p\x11a\0yW\x80c\xEF@\xA6p\x14a\t\x82W\x80c\xF0e\xDE?\x14a\t\xAAW\x80c\xFA\xCDt;\x14a\t\xBDW\x80c\xFB\x0Er+\x14a\t\xD0W_\x80\xFD[\x80c\xE6\xB3\x08,\x14a\x08\xB1W\x80c\xE7\x8C\xEA\x92\x14a\x08\xCCW\x80c\xE8\xBDI\"\x14a\x08\xDFW\x80c\xEE5\xF3'\x14a\tzW_\x80\xFD[\x80c\xCE\x11\xE6\xAB\x11a\0\xE4W\x80c\xCE\x11\xE6\xAB\x14a\x08pW\x80c\xDF\xF6\x97\x87\x14a\x08\x83W\x80c\xE5\x10\x19\xA6\x14a\x08\x8BW\x80c\xE51\xD8\xC7\x14a\x08\x9EW_\x80\xFD[\x80c\xBCE\xE0\xAE\x14a\x08BW\x80c\xC2\xC2\xE6\x8E\x14a\x08UW\x80c\xC4\xD6m\xE8\x14a\x08]W_\x80\xFD[\x80c\x84r\x8C\xD0\x11a\x01\x7FW\x80c\x9A\x8A\x05\x92\x11a\x01OW\x80c\x9A\x8A\x05\x92\x14a\x07\x0EW\x80c\xA2<D\xB1\x14a\x07\x17W\x80c\xAA8\xA6\xE7\x14a\x08\x1AW\x80c\xB7\xABM\xB5\x14a\x08-W_\x80\xFD[\x80c\x84r\x8C\xD0\x14a\x06\xA2W\x80c\x880(\x84\x14a\x06\xDDW\x80c\x8D\xA5\xCB[\x14a\x06\xFDW\x80c\x8E\xE1\xA1&\x14a\x07\x05W_\x80\xFD[\x80cm\xDD7D\x11a\x01\xBAW\x80cm\xDD7D\x14a\x06jW\x80cq\xEF#,\x14a\x06}W\x80cs\0 \x1C\x14a\x06\x86W\x80cv\xE7\xE2;\x14a\x06\x99W_\x80\xFD[\x80ce\xF7\xF8\r\x14a\x06<W\x80ch\x12\x9B\x14\x14a\x06DW\x80ch_^\xCC\x14a\x06WW_\x80\xFD[\x80c;\x86\xDE\x19\x11a\x02\xC0W\x80cV\xBB\xC9\xE6\x11a\x02`W\x80c`\x96hm\x11a\x020W\x80c`\x96hm\x14a\x05\xD9W\x80ca79\x19\x14a\x05\xECW\x80caw\xFD\x18\x14a\x05\xF4W\x80cd \xFB\x9F\x14a\x06)W_\x80\xFD[\x80cV\xBB\xC9\xE6\x14a\x05\xA0W\x80cW\xEFJ\xB9\x14a\x05\xB3W\x80cX\x8Cz\x16\x14a\x05\xBBW\x80c\\\x97Z\xBB\x14a\x05\xCEW_\x80\xFD[\x80cP\xF3/h\x11a\x02\x9BW\x80cP\xF3/h\x14a\x05ZW\x80cQ\xEDj0\x14a\x05mW\x80cR\xD1\x90-\x14a\x05\x80W\x80cU\x84\nX\x14a\x05\x88W_\x80\xFD[\x80c;\x86\xDE\x19\x14a\x05\x12W\x80c;\xE6\x80\xEA\x14a\x05%W\x80cE\xE3\x8Bd\x14a\x05QW_\x80\xFD[\x80c\x1B\x16\x89\xE9\x11a\x036W\x80c/0\xCA\xBD\x11a\x03\x06W\x80c/0\xCA\xBD\x14a\x04\xBCW\x80c0\x83b(\x14a\x04\xE4W\x80c3c_\xC2\x14a\x04\xF7W\x80c53%\xE0\x14a\x05\nW_\x80\xFD[\x80c\x1B\x16\x89\xE9\x14a\x04aW\x80c\x1E\x83\xD3\x0F\x14a\x04jW\x80c*\xBD\xD20\x14a\x04}W\x80c.z\xCF\xA6\x14a\x04\xA8W_\x80\xFD[\x80c\x11qU\x85\x11a\x03qW\x80c\x11qU\x85\x14a\x03\xEFW\x80c\x12\xAB=;\x14a\x04\x1BW\x80c\x13\xC5l\xA7\x14a\x048W\x80c\x18\xBA\xAA\xB9\x14a\x04YW_\x80\xFD[\x80c\x02:\x96\xFE\x14a\x03\x97W\x80c\x04\x97*\xF9\x14a\x03\xC7W\x80c\x10\xB9\x8A5\x14a\x03\xDCW[_\x80\xFD[`iTa\x03\xAA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\xDAa\x03\xD56`\x04a?RV[a\t\xE3V[\0[a\x03\xDAa\x03\xEA6`\x04a?\x8DV[a\t\xFDV[a\x04\x02a\x03\xFD6`\x04a?\xE7V[a\r\xDDV[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03\xBEV[`{Ta\x04(\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03\xBEV[a\x04Ka\x04F6`\x04a?\xE7V[a\r\xF8V[`@Q\x90\x81R` \x01a\x03\xBEV[a\x03\xDAa\x0F=V[a\x04K`zT\x81V[a\x03\xDAa\x04x6`\x04a?\xE7V[a\x10\x11V[a\x04Ka\x04\x8B6`\x04a@\x12V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`w` R`@\x90 `\x01\x01T\x90V[`fTa\x04\x02\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x04Ka\x04\xCA6`\x04a@\x12V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`x` R`@\x90 T\x90V[a\x04(a\x04\xF26`\x04a?\xE7V[a\x10\x8BV[a\x04Ka\x05\x056`\x04a@-V[a\x10\xA6V[a\x04Ka\x10\xCAV[a\x03\xDAa\x05 6`\x04a@rV[a\x11\x04V[`iTa\x04\x02\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x04K`qT\x81V[a\x03\xDAa\x05h6`\x04a@\x9DV[a\x14\x9BV[`pTa\x03\xAA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04Ka\x15\x14V[`{Ta\x03\xAA\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\x02a\x05\xAE6`\x04a?\xE7V[a\x15\xD8V[a\x03\xDAa\x15\xFAV[a\x03\xDAa\x05\xC96`\x04a@\x12V[a\x16hV[`3T`\xFF\x16a\x04(V[a\x03\xDAa\x05\xE76`\x04a@\xE6V[a\x17VV[a\x04Ka\x17\xC9V[a\x04(a\x06\x026`\x04a@\x12V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`w` R`@\x90 `\x02\x01T`\x01`@\x1B\x90\x04`\xFF\x16\x90V[a\x03\xDAa\x0676`\x04a@rV[a\x18IV[`tTa\x04KV[a\x03\xDAa\x06R6`\x04aA#V[a\x19\xFEV[a\x03\xDAa\x06e6`\x04aAQV[a\x1AuV[a\x03\xAAa\x06x6`\x04aA\xA4V[a\x1A\x91V[a\x04K`yT\x81V[a\x03\xDAa\x06\x946`\x04aA\xBFV[a\x1A\xC9V[a\x04K`gT\x81V[a\x03\xAAa\x06\xB06`\x04a@\x12V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`w` R`@\x90 `\x02\x01Ti\x01\0\0\0\0\0\0\0\0\0\x90\x04\x16\x90V[a\x06\xF0a\x06\xEB6`\x04a?\xE7V[a\x1A\xD5V[`@Qa\x03\xBE\x91\x90aB$V[a\x03\xAAa\x1B\xA0V[a\x04K`hT\x81V[a\x04K`eT\x81V[a\x07\xC4a\x07%6`\x04a@\x12V[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91RP`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`w` \x90\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R`\x02\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x92\x82\x01\x92\x90\x92R`\xFF`\x01`@\x1B\x83\x04\x16\x15\x15``\x82\x01Ri\x01\0\0\0\0\0\0\0\0\0\x90\x91\x04\x90\x91\x16`\x80\x82\x01R\x90V[`@Qa\x03\xBE\x91\x90_`\xA0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x84\x01Q\x16`@\x83\x01R``\x83\x01Q\x15\x15``\x83\x01R`\x01`\x01`\xA0\x1B\x03`\x80\x84\x01Q\x16`\x80\x83\x01R\x92\x91PPV[`mTa\x03\xAA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x085a\x1B\xA9V[`@Qa\x03\xBE\x91\x90aB\x84V[`nTa\x03\xAA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\xDAa\x1B\xB5V[a\x03\xDAa\x08k6`\x04a@\x12V[a\x1CwV[`lTa\x03\xAA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`vTa\x04\x02V[a\x03\xDAa\x08\x996`\x04aB\xD0V[a\x1E\x19V[a\x04(a\x08\xAC6`\x04a?\xE7V[a\x1E\x80V[`fTa\x04\x02\x90`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`kTa\x03\xAA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\t:a\x08\xED6`\x04a@\x12V[`w` R_\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x90\x92\x01T\x90\x91\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90`\x01`@\x1B\x81\x04`\xFF\x16\x90i\x01\0\0\0\0\0\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x85V[`@\x80Q\x95\x86R` \x86\x01\x94\x90\x94Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x92\x84\x01\x92\x90\x92R\x90\x15\x15``\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16`\x80\x82\x01R`\xA0\x01a\x03\xBEV[a\x03\xAAa\x1E\xAFV[a\x04Ka\t\x906`\x04a@\x12V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`w` R`@\x90 T\x90V[`oTa\x03\xAA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04(a\t\xCB6`\x04a@\x12V[a\x1F3V[`jTa\x03\xAA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\t\xF9\x81a\t\xF0\x84a\x1FRV[`\x01\x01Ta\x1F\xB1V[PPV[3_\x81\x81R`s` R`@\x90 T\x15\x15\x80a\n\x1BWP`{T`\xFF\x16[a\n\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl''\xAA/\xAB \xA6$\xA2 \xAA'\xA9`\x99\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\nda \xB6V[_a\nn\x88a\x1FRV[\x90P_a\nz\x88a\x1FRV[\x90Pa\n\x8A\x85\x82`\x01\x01Ta\x1F\xB1V[a\n\x9A`\x80\x86\x01``\x87\x01aA\xA4V[\x82Ta\n\xB7\x91\x90`\x01`\x80\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aC\x1FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x10\x15a\x0B\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FBEFORE_DEADLINE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`tT\x88\x14a\x0BaW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FPREV_NOT_LATEST_CONFIRMED\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[\x80T`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\r\xC6W_a\x0B\x8A``\x87\x01`@\x88\x01a@\x12V[`\x01`\x01`\xA0\x1B\x03\x16c\xFD\xA2\x89.\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xB7\x91\x81R` \x01\x90V[a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xD3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xF7\x91\x90aC\xE4V[\x90P\x89\x81`\xE0\x01Q\x14a\x0CLW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01R\x7FNOT_WINNER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`\x01\x81a\x01`\x01Q`\x01\x81\x11\x15a\x0CeWa\x0CeaA\xF4V[\x14a\x0C\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FEDGE_NOT_CONFIRMED\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[\x80a\x01@\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a\r\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FZERO_CONFIRMED_AT_BLOCK\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`iTa\x01@\x82\x01Qa\rE\x91t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90aC\x1FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x10\x15a\r\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FCHALLENGE_GRACE_PERIOD_NOT_PASSE`D\x82\x01R\x7FD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nSV[P[a\r\xD2\x89\x89\x89\x87a!\x0BV[PPPPPPPPPV[_a\r\xE7\x82a\x1FRV[Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x0E\x80W_\x82\x81R`|` R`@\x90 T\x80a\x0EzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNO_ASSERTION\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[\x92\x91PPV[_a\x0E\x8A\x83a\x1FRV[`@\x80Q`\xC0\x81\x01\x82R\x82Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x82\x04\x16\x92\x82\x01\x92\x90\x92R`\xFF`\x01`\xC0\x1B\x83\x04\x81\x16\x15\x15``\x83\x01R\x92\x93Pa\x0F \x92\x90\x91\x84\x91`\x80\x84\x01\x91`\x01`\xC8\x1B\x90\x04\x16`\x02\x81\x11\x15a\x0E\xFCWa\x0E\xFCaA\xF4V[`\x02\x81\x11\x15a\x0F\rWa\x0F\raA\xF4V[\x81R` \x01`\x01\x82\x01T\x81RPPa#\x1AV[T`\x01`\x80\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[\x91\x90PV[`{T`\xFF\x16\x15a\x0F\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FWHITELIST_DISABLED\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[a\x0F\x98a#~V[a\x0F\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FVALIDATOR_NOT_AFK\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`{\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UV[3_\x81\x81R`s` R`@\x90 T\x15\x15\x80a\x10/WP`{T`\xFF\x16[a\x10kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl''\xAA/\xAB \xA6$\xA2 \xAA'\xA9`\x99\x1B`D\x82\x01R`d\x01a\nSV[a\x10sa \xB6V[a\x10|3a$\xADV[a\x10\x863\x83a%\x9CV[PPPV[_a\x10\x95\x82a\x1FRV[T`\x01`\xC0\x1B\x90\x04`\xFF\x16\x92\x91PPV[_a\x10\xC0\x84a\x10\xBA6\x86\x90\x03\x86\x01\x86aE\xC7V[\x84a&\x9BV[\x90P[\x93\x92PPPV[_a\x10\xD3a>\xF9V[`@\x80Q``\x81\x01\x82R\x82\x81R`\x01` \x82\x01R_\x91\x81\x01\x82\x90R\x90\x80a\x10\xFB\x81\x84\x81a&\x9BV[\x94PPPPP\x90V[3_\x81\x81R`s` R`@\x90 T\x15\x15\x80a\x11\"WP`{T`\xFF\x16[a\x11^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl''\xAA/\xAB \xA6$\xA2 \xAA'\xA9`\x99\x1B`D\x82\x01R`d\x01a\nSV[a\x11fa \xB6V[\x81\x15\x80a\x11\x96WP_a\x11x\x83a\x1FRV[T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x11\x94Wa\x11\x94aA\xF4V[\x14[a\x11\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FEXPECTED_ASSERTION_SEEN\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[3_\x90\x81R`w` R`@\x90 `\x02\x01T`\x01`@\x1B\x90\x04`\xFF\x16a\x127W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri\x13\x93\xD5\x17\xD4\xD5\x10R\xD1Q`\xB2\x1B`D\x82\x01R`d\x01a\nSV[3_\x90\x81R`w` R`@\x90 T``\x84\x015\x11\x15a\x12\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FINSUFFICIENT_STAKE\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_a\x12\xBB\x845a\x12\xB16\x87\x90\x03\x87\x01`\xE0\x88\x01aE\xC7V[` \x87\x015a&\x9BV[\x90Pa\x134a\x12\xC9\x82a\x1FRV[`@\x80Q`\xC0\x81\x01\x82R\x82Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x82\x04\x16\x92\x82\x01\x92\x90\x92R`\xFF`\x01`\xC0\x1B\x83\x04\x81\x16\x15\x15``\x83\x01R\x90\x92\x91`\x80\x84\x01\x91`\x01`\xC8\x1B\x90\x91\x04\x16`\x02\x81\x11\x15a\x0E\xFCWa\x0E\xFCaA\xF4V[3_\x90\x81R`w` R`@\x90 `\x01\x01T\x81\x81\x14\x80a\x13fWP_a\x13Y\x82a\x1FRV[Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11[a\x13\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FSTAKED_ON_ANOTHER_BRANCH\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_\x80a\x13\xBF\x87\x85\x88a&\xDEV[3_\x90\x81R`w` R`@\x90 `\x01\x01\x82\x90U\x90\x92P\x90P\x80a\x14YW_a\x13\xE7\x85a\x1FRV[Ta\x14\x03\x90`\x01`\x80\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16CaF,V[\x90P`qT\x81\x10\x15a\x14WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01R\x7FTIME_DELTA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[P[a\x14b\x82a\x1FRV[T`\x01`\xC0\x1B\x90\x04`\xFF\x16a\x14\x92W`oT`pTa\x14\x92\x91`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16``\x8A\x015a0KV[PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x14\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FEMPTY_WITHDRAWAL_ADDRESS\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[a\x14\xFB\x84\x82a0\xF4V[a\x15\x05\x83\x83a\x11\x04V[a\x15\x0E\x84a1\xCAV[PPPPV[_0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x15\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FUUPSNotUpgradeable: must not be `D\x82\x01R\x7Fcalled through delegatecall\0\0\0\0\0`d\x82\x01R`\x84\x01a\nSV[P\x7F+\x1D\xBC\xE7C$$\x8C\"/\x0E\xC2\xD5\xED{\xD3#\xCF\xC4%\xB36\xF0%<\\\xCF\xDAreTm\x90V[_a\x15\xE2\x82a\x1FRV[T`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[3_\x81\x81R`s` R`@\x90 T\x15\x15\x80a\x16\x18WP`{T`\xFF\x16[a\x16TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl''\xAA/\xAB \xA6$\xA2 \xAA'\xA9`\x99\x1B`D\x82\x01R`d\x01a\nSV[a\x16\\a \xB6V[a\x16e3a1\xE2V[PV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`s` R`@\x90 T\x81\x90\x15\x15\x80a\x16\x91WP`{T`\xFF\x16[a\x16\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl''\xAA/\xAB \xA6$\xA2 \xAA'\xA9`\x99\x1B`D\x82\x01R`d\x01a\nSV[a\x16\xD5a \xB6V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x90\x81R`w` R`@\x90 `\x02\x01Ti\x01\0\0\0\0\0\0\0\0\0\x90\x04\x163\x14a\x17MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FNOT_WITHDRAWAL_ADDRESS\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[a\t\xF9\x82a1\xE2V[a\x17^a \xB6V[`{Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FNOT_FAST_CONFIRMER\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[a\x15\x0E\x84\x84\x84\x84a!\x0BV[_a\x17\xD2a \xB6V[_a\x17\xDC3a1\xF4V[\x90P_\x81\x11a\x18-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FNO_FUNDS_TO_WITHDRAW\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`pTa\x18D\x90`\x01`\x01`\xA0\x1B\x03\x163\x83a0KV[\x90P\x90V[a\x18Qa \xB6V[\x80a\x18\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FEXPECTED_ASSERTION_HASH\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_a\x18\xA8\x82a\x1FRV[T`\x01`\xC8\x1B\x90\x04`\xFF\x16\x90P_a\x18\xCD\x845a\x12\xB16\x87\x90\x03\x87\x01`\xE0\x88\x01aE\xC7V[\x90Pa\x18\xDBa\x12\xC9\x82a\x1FRV[_\x82`\x02\x81\x11\x15a\x18\xEEWa\x18\xEEaA\xF4V[\x03a\x19=W_a\x18\xFF\x85\x83\x86a&\xDEV[P\x90Pa\x19\x0B\x81a\x1FRV[T`\x01`\xC0\x1B\x90\x04`\xFF\x16a\x19;W`oT`pTa\x19;\x91`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16``\x88\x015a0KV[P[`kTa\x15\x0E\x90\x84\x90\x83\x90a\x01\xA0\x88\x01\x90`\x01`\x01`\xA0\x1B\x03\x16c\x16\xBFUy`\x01a\x19ua\x19p6\x8D\x90\x03\x8D\x01\x86aF?V[a2sV[a\x19\x7F\x91\x90aFYV[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xDAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xE7\x91\x90aFzV[a\x1A\x06a \xB6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1A\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FEMPTY_WITHDRAWAL_ADDRESS\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[a\x1Af\x82\x82a0\xF4V[\x81\x15a\t\xF9Wa\t\xF9\x82a1\xCAV[a\x1A}a \xB6V[a\x1A\x88\x83\x83\x83a2\x87V[a\x10\x86\x81a1\xCAV[_`v\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1A\xAFWa\x1A\xAFaF\x91V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[a\x10\x86\x83\x83\x833a\x14\x9BV[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x1B\x0F\x82a\x1FRV[`@\x80Q`\xC0\x81\x01\x82R\x82Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x82\x04\x16\x92\x82\x01\x92\x90\x92R`\xFF`\x01`\xC0\x1B\x83\x04\x81\x16\x15\x15``\x83\x01R\x90\x92\x91`\x80\x84\x01\x91`\x01`\xC8\x1B\x90\x91\x04\x16`\x02\x81\x11\x15a\x1BzWa\x1BzaA\xF4V[`\x02\x81\x11\x15a\x1B\x8BWa\x1B\x8BaA\xF4V[\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x91\x90PV[_a\x18Da3\xEDV[``a\x18D`ra4\x1FV[`{T`\xFF\x16\x15a\x1C\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FWHITELIST_DISABLED\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03a\x0F\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FCHAIN_ID_NOT_CHANGED\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x1D\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nSV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x1DGa4+V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1D\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Factive proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nSV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x16eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FNEED_STAKE_TOKEN\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[a\x1E2\x82a\x1E,6\x86\x90\x03\x86\x01\x86aE\xC7V[\x83a&\x9BV[\x84\x14a\x15\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FINVALID_ASSERTION_HASH\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_`\x01a\x1E\x8C\x83a\x1FRV[T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1E\xA8Wa\x1E\xA8aA\xF4V[\x14\x92\x91PPV[`kT`@\x80Q\x7F\xEE5\xF3'\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xEE5\xF3'\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1F\x0FW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18D\x91\x90aF\xA5V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`s` R`@\x81 T\x15\x15a\x0EzV[_\x81a\x1F\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FASSERTION_ID_CANNOT_BE_ZERO\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[P_\x90\x81R`u` R`@\x90 \x90V[a h\x825` \x84\x015a\x1F\xCB``\x86\x01`@\x87\x01a@\x12V[a\x1F\xDB`\x80\x87\x01``\x88\x01aA\xA4V[a\x1F\xEB`\xA0\x88\x01`\x80\x89\x01aA\xA4V[`@\x80Q` \x80\x82\x01\x97\x90\x97R\x80\x82\x01\x95\x90\x95Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x94\x85\x1B\x16\x93\x85\x01\x93\x90\x93R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0\x92\x83\x1B\x81\x16`t\x86\x01R\x91\x1B\x16`|\x83\x01R\x80Q`d\x81\x84\x03\x01\x81R`\x84\x90\x92\x01\x90R\x80Q\x91\x01 \x90V[\x81\x14a\t\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FCONFIG_HASH_MISMATCH\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`3T`\xFF\x16\x15a!\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FPausable: paused\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[V[_a!\x15\x85a\x1FRV[\x90P`\x01\x81T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a!6Wa!6aA\xF4V[\x14a!\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7FNOT_PENDING\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[a!\x96\x84a\x10\xBA6\x86\x90\x03\x86\x01\x86aE\xC7V[\x85\x14a!\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FCONFIRM_DATA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_a!\xFCa!\xF76\x86\x90\x03\x86\x01\x86aF?V[a4RV[\x90P_a\"\x16a\"\x116\x87\x90\x03\x87\x01\x87aF?V[a4\\V[`lT`@Q\x7F\xA0L\xEE`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x85\x90R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA0L\xEE`\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\"zW_\x80\xFD[PZ\xF1\x15\x80\x15a\"\x8CW=_\x80>=_\xFD[PPP`t\x88\x90UP\x82T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16y\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x83U`@\x80Q\x83\x81R` \x81\x01\x83\x90R\x81Q\x89\x92\x7F\xFCB\x82\x9B)\xC2Y\xA77\n\xB5l\x8Fi\xFC\xE2;_5\x1A\x9C\xE1Q\xDAE2\x81\x99>\xC0\t\x0C\x92\x82\x90\x03\x01\x90\xA2PPPPPPPV[_\x81`\x80\x01Q`\x02\x81\x11\x15a#1Wa#1aA\xF4V[\x03a\x16eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FASSERTION_NOT_EXIST\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_\x80a#\x91a#\x8C`tT\x90V[a\x1FRV[`@\x80Q`\xC0\x81\x01\x82R\x82Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x82\x04\x16\x92\x82\x01\x92\x90\x92R`\xFF`\x01`\xC0\x1B\x83\x04\x81\x16\x15\x15``\x83\x01R\x90\x92\x91`\x80\x84\x01\x91`\x01`\xC8\x1B\x90\x91\x04\x16`\x02\x81\x11\x15a#\xFCWa#\xFCaA\xF4V[`\x02\x81\x11\x15a$\rWa$\raA\xF4V[\x81R`\x01\x91\x90\x91\x01T` \x90\x91\x01R`fT\x90\x91P`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x81\x90\x03a$CW_\x92PPP\x90V[\x81`@\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a$`W_\x92PPP\x90V[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a$\x92W\x81QC\x90a$\x8A\x90\x83\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aF\xC0V[\x10\x92PPP\x90V[C\x81\x83`@\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a$\x8A\x91\x90aF\xC0V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`w` R`@\x90 `\x02\x01T`\x01`@\x1B\x90\x04`\xFF\x16a%\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri\x13\x93\xD5\x17\xD4\xD5\x10R\xD1Q`\xB2\x1B`D\x82\x01R`d\x01a\nSV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`w` R`@\x81 `\x01\x01T`tT\x90\x91\x90\x82\x14\x90\x80a%9\x84a\x1FRV[Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x90P\x81\x80a%PWP\x80[a\x15\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FSTAKE_ACTIVE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x90\x81R`w` R`@\x81 `\x02\x81\x01T\x81T\x92\x93\x91\x92i\x01\0\0\0\0\0\0\0\0\0\x90\x91\x04\x90\x91\x16\x90\x80\x85\x11\x15a&!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FTOO_LITTLE_STAKE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_a&,\x86\x83aF,V[\x86\x85U\x90Pa&;\x83\x82a4gV[\x82`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x7F\xD9W\xCF#@\x0735\xD2V\xF7*\x9E\xF8\x9C\xF1\xA4<1\x143A\xA6\xA55u\xEF3\xE9\x87\xBE\xB8\x84\x89`@Qa&\x89\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA3\x96\x95PPPPPPV[_a\x10\xC0\x84a&\xA9\x85a5\nV[`@\x80Q` \x80\x82\x01\x94\x90\x94R\x80\x82\x01\x92\x90\x92R``\x80\x83\x01\x87\x90R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\x80\x90\x92\x01\x90R\x80Q\x91\x01 \x90V[_\x80a&\xF0`@\x86\x01a\t\xF0\x86a\x1FRV[`\x01a'\x04a\x02@\x87\x01a\x02 \x88\x01aF\xD3V[`\x02\x81\x11\x15a'\x15Wa'\x15aA\xF4V[\x14\x80a'CWP`\x02a'0a\x02@\x87\x01a\x02 \x88\x01aF\xD3V[`\x02\x81\x11\x15a'AWa'AaA\xF4V[\x14[a'\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FBAD_AFTER_STATUS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[\x83a'\xB1\x865a'\xA76\x89\x90\x03\x89\x01`\xE0\x8A\x01aE\xC7V[` \x89\x015a&\x9BV[\x14a'\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FINVALID_BEFORE_STATE\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`\x01a(\x12a\x01\x80\x87\x01a\x01`\x88\x01aF\xD3V[`\x02\x81\x11\x15a(#Wa(#aA\xF4V[\x14a(pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FBAD_PREV_STATUS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_a(z\x85a\x1FRV[\x90P_\x80a\x01\xA0\x88\x01`\xE0\x89\x01\x82a(\x92\x83\x83a59V[\x12\x15a(\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FINBOX_BACKWARDS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_a)\x05a(\xF4`\xE0\x8D\x01`\xC0\x8E\x01aA\xA4V[\x84\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a6?V[\x90P_\x81\x13\x15a)WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7FINBOX_TOO_FAR\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`\x02a)ka\x02@\x8D\x01a\x02 \x8E\x01aF\xD3V[`\x02\x81\x11\x15a)|Wa)|aA\xF4V[\x14\x15\x80\x15a)\x89WP_\x81\x12[\x15a)\xEAW`\x01\x96P_a)\x9D\x84\x84a59V[\x13a)\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FOVERFLOW_STANDSTILL\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`kT`@\x80Q~\x84\x12\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91b\x84\x12\x0C\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a*HW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*l\x91\x90aFzV[\x90P_a*y\x85\x83a6?V[\x13\x15a*\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FINBOX_PAST_END\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[\x80a*\xD8`\xE0\x8E\x01`\xC0\x8F\x01aA\xA4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a+0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FINBOX_NOT_POPULATED\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_a+Ca\x19p6\x87\x90\x03\x87\x01\x87aF?V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81\x81\x03a+hWa+a\x82`\x01aF\xC0V[\x96Pa+lV[\x81\x96P[\x80_\x03a+\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FEMPTY_INBOX_COUNT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`kT`\x01`\x01`\xA0\x1B\x03\x16c\x16\xBFUya+\xD7`\x01\x84aF,V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\xF5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\x10W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,4\x91\x90aFzV[\x95PPPPPPa,T\x87\x89a\x01\xA0\x01\x806\x03\x81\x01\x90a\x1E,\x91\x90aE\xC7V[\x94P\x85\x85\x14\x80a,bWP\x85\x15[a,\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FUNEXPECTED_ASSERTION_HASH\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_a,\xB8\x86a\x1FRV[T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a,\xD4Wa,\xD4aA\xF4V[\x14a-!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FASSERTION_SEEN\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[\x82T`hT`gT`iT`fT`@\x80Q` \x80\x82\x01\x96\x90\x96R\x80\x82\x01\x94\x90\x94Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x93\x84\x1B\x16\x83\x85\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0\x92\x83\x1B\x81\x16`t\x86\x01R\x88\x83\x1B\x16`|\x85\x01R\x80Q`d\x81\x86\x03\x01\x81R`\x84\x85\x01\x80\x83R\x81Q\x91\x87\x01\x91\x90\x91 a\x01D\x86\x01\x83R_\x91\x82\x90R`\xA4\x86\x01\x82\x90R`\xC4\x86\x01\x82\x90R`\xE4\x86\x01\x82\x90Ra\x01\x04\x86\x01\x82\x90Ra\x01$\x90\x95\x01\x81\x90R\x81Q\x92\x83\x01\x82R\x80\x83R\x94\x82\x01\x85\x90Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x81\x16\x91\x83\x01\x91\x90\x91R\x90\x94\x16\x15\x90\x84\x01R`\xA0\x83\x01R`\x01`\x80\x83\x01RPa.\x1E\x84a6\xCAV[_\x86\x81R`u` \x90\x81R`@\x91\x82\x90 \x83Q\x81T\x92\x85\x01Q\x93\x85\x01Q``\x86\x01Q\x15\x15`\x01`\xC0\x1B\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\x01`\x80\x1B\x02\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96\x83\x16`\x01`@\x1B\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x96\x16\x92\x90\x93\x16\x91\x90\x91\x17\x93\x90\x93\x17\x93\x90\x93\x16\x92\x90\x92\x17\x17\x80\x82U`\x80\x83\x01Q\x83\x92\x91\x82\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xC8\x1B\x83`\x02\x81\x11\x15a/%Wa/%aA\xF4V[\x02\x17\x90UP`\xA0\x82\x01Q\x81`\x01\x01U\x90PP\x87\x86\x7F\x90\x1C:\xEE#\xCFDx\x82Tb\xCA\xAA\xB3u\xC6\x06\xAB\x83Q``8\x83D\xF0e\x03@u60\x8B\x85\x87`hT`gT`i_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`f_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Qa/\xA1\x97\x96\x95\x94\x93\x92\x91\x90aGcV[`@Q\x80\x91\x03\x90\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a0?W`d`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xB1\xB3\x1D`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0\x0CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a00\x91\x90aFzV[_\x87\x81R`|` R`@\x90 U[PPPP\x93P\x93\x91PPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x10\x86\x90\x84\x90\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`d\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra7LV[3_\x81\x81R`s` R`@\x90 T\x15\x15\x80a1\x12WP`{T`\xFF\x16[a1NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl''\xAA/\xAB \xA6$\xA2 \xAA'\xA9`\x99\x1B`D\x82\x01R`d\x01a\nSV[a1Va \xB6V[3_\x90\x81R`w` R`@\x90 `\x02\x01T`\x01`@\x1B\x90\x04`\xFF\x16\x15a1\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FALREADY_STAKED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[a\x10\x863\x84\x84a80V[`pTa\x16e\x90`\x01`\x01`\xA0\x1B\x03\x1630\x84a9\xABV[a1\xEB\x81a$\xADV[a\x16e\x81a9\xFCV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`x` R`@\x81 \x80T\x90\x82\x90U`y\x80T\x82\x91\x90\x84\x90a2%\x90\x84\x90aF,V[\x90\x91UPP`@\x80Q\x82\x81R_` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x7F\xA7@\xAF\x14\xC5nN\x04\xA6\x17\xB1\xDE\x1E\xB2\r\xE72p\xDE\xCB\xAA\xEA\xD1O\x14*\xAB\xF3\x03\x8EZ\xE2\x91\x01`@Q\x80\x91\x03\x90\xA2\x92\x91PPV[` \x81\x01Q_\x90\x81[` \x02\x01Q\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`s` R`@\x90 T\x83\x90\x15\x15\x80a2\xB0WP`{T`\xFF\x16[a2\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl''\xAA/\xAB \xA6$\xA2 \xAA'\xA9`\x99\x1B`D\x82\x01R`d\x01a\nSV[a2\xF4a \xB6V[`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`w` R`@\x90 `\x02\x01T`\x01`@\x1B\x90\x04`\xFF\x16a3RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri\x13\x93\xD5\x17\xD4\xD5\x10R\xD1Q`\xB2\x1B`D\x82\x01R`d\x01a\nSV[\x82`\x01`\x01`\xA0\x1B\x03\x16a3\x8D\x85`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`w` R`@\x90 `\x02\x01Ti\x01\0\0\0\0\0\0\0\0\0\x90\x04\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a3\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FWRONG_WITHDRAWAL_ADDRESS\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[a\x15\x0E\x84\x83a:\x85V[_\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03[T`\x01`\x01`\xA0\x1B\x03\x16\x91\x90PV[``_a\x10\xC3\x83a;\x12V[_\x7F+\x1D\xBC\xE7C$$\x8C\"/\x0E\xC2\xD5\xED{\xD3#\xCF\xC4%\xB36\xF0%<\\\xCF\xDAreTma4\x10V[\x80Q_\x90\x81a2|V[\x80Q_\x90`\x01a2|V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`x` R`@\x81 T\x90a4\x8A\x83\x83aF\xC0V[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`x` R`@\x81 \x82\x90U`y\x80T\x92\x93P\x85\x92\x90\x91\x90a4\xBB\x90\x84\x90aF\xC0V[\x90\x91UPP`@\x80Q\x83\x81R` \x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x86\x16\x91\x7F\xA7@\xAF\x14\xC5nN\x04\xA6\x17\xB1\xDE\x1E\xB2\r\xE72p\xDE\xCB\xAA\xEA\xD1O\x14*\xAB\xF3\x03\x8EZ\xE2\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPV[_\x81`@Q` \x01a5\x1C\x91\x90aHNV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[_\x80a5Ma\x19p6\x86\x90\x03\x86\x01\x86aF?V[\x90P_a5ba\x19p6\x86\x90\x03\x86\x01\x86aF?V[\x90P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a5\x8BW_\x19\x92PPPa\x0EzV[\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a5\xB2W`\x01\x92PPPa\x0EzV[_a5\xCAa5\xC56\x88\x90\x03\x88\x01\x88aF?V[a;kV[\x90P_a5\xDFa5\xC56\x88\x90\x03\x88\x01\x88aF?V[\x90P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a6\nW_\x19\x94PPPPPa\x0EzV[\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a63W`\x01\x94PPPPPa\x0EzV[_\x94PPPPPa\x0EzV[_\x80a6Sa\x19p6\x86\x90\x03\x86\x01\x86aF?V[\x90P\x82\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a6qW_\x19\x91PPa\x0EzV[\x82\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a6\x8DW`\x01\x91PPa\x0EzV[_a6\xA0a5\xC56\x87\x90\x03\x87\x01\x87aF?V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a6\xBAW`\x01\x91PPa\x0EzV[_\x91PPa\x0EzV[P\x92\x91PPV[\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a6\xF9W\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90UV[\x80T`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a\x16eW\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`@\x1BCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UV[_a7\xA0\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a;y\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x10\x86W\x80\x80` \x01\x90Q\x81\x01\x90a7\xBE\x91\x90aH\xD8V[a\x10\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01R\x7Fot succeed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nSV[`v\x80T`\x01\x80\x82\x01\x90\x92U\x7F\xB5s'\x05\xF5$\x13p\xA2\x89\x08\xC2\xFE\x13\x03\xCB\"?\x03\xB9\r\x85\x7F\xD0W?\0?y\xFE\xFE\xD4\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x91\x82\x17\x90\x92U`@\x80Q`\xA0\x81\x01\x82R\x87\x81R`tT` \x80\x83\x01\x91\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x84\x86\x01\x90\x81R``\x85\x01\x8A\x81R\x8B\x89\x16`\x80\x87\x01\x81\x81R_\x8A\x81R`w\x87R\x89\x81 \x98Q\x89U\x96Q\x9C\x88\x01\x9C\x90\x9CU\x91Q`\x02\x90\x96\x01\x80T\x91Q\x9BQ\x96\x90\x93\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x90\x91\x16\x17`\x01`@\x1B\x9A\x15\x15\x9A\x90\x9A\x02\x99\x90\x99\x17\x7F\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16i\x01\0\0\0\0\0\0\0\0\0\x94\x90\x97\x16\x93\x90\x93\x02\x95\x90\x95\x17\x90\x91U\x81Q\x90\x81R\x92\x83\x01\x87\x90R\x92\x93\x92\x90\x91\x7F\xD9W\xCF#@\x0735\xD2V\xF7*\x9E\xF8\x9C\xF1\xA4<1\x143A\xA6\xA55u\xEF3\xE9\x87\xBE\xB8\x91\x01[`@Q\x80\x91\x03\x90\xA3PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x15\x0E\x90\x85\x90\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x84\x01a0\x90V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`w` R`@\x90 `\x02\x81\x01T\x81T\x91\x92i\x01\0\0\0\0\0\0\0\0\0\x90\x91\x04\x16\x90a:7\x82\x82a4gV[a:@\x84a;\x87V[`@\x80Q\x82\x81R_` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x92\x90\x87\x16\x91\x7F\xD9W\xCF#@\x0735\xD2V\xF7*\x9E\xF8\x9C\xF1\xA4<1\x143A\xA6\xA55u\xEF3\xE9\x87\xBE\xB8\x91\x01a9\x9DV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`w` R`@\x81 \x80T\x90\x91a:\xAA\x84\x83aF\xC0V[\x80\x84U`\x02\x84\x01T`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x92\x93P`\x01`\x01`\xA0\x1B\x03i\x01\0\0\0\0\0\0\0\0\0\x90\x92\x04\x82\x16\x92\x91\x88\x16\x91\x7F\xD9W\xCF#@\x0735\xD2V\xF7*\x9E\xF8\x9C\xF1\xA4<1\x143A\xA6\xA55u\xEF3\xE9\x87\xBE\xB8\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a;_W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a;KW[PPPPP\x90P\x91\x90PV[` \x81\x01Q_\x90`\x01a2|V[``a\x10\xC0\x84\x84_\x85a=|V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`w` R`@\x90 `\x02\x81\x01T`\x01`@\x1B\x90\x04`\xFF\x16a;\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri\x13\x93\xD5\x17\xD4\xD5\x10R\xD1Q`\xB2\x1B`D\x82\x01R`d\x01a\nSV[`\x02\x81\x01T`v\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91a<\x08\x90`\x01\x90aF,V[\x81T\x81\x10a<\x18Wa<\x18aF\x91V[_\x91\x82R` \x90\x91 \x01T`v\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x10a<MWa<MaF\x91V[\x90_R` _ \x01_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`w_`v\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a<\x99Wa<\x99aF\x91V[_\x91\x82R` \x80\x83 \x91\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83R\x82\x01\x92\x90\x92R`@\x01\x90 `\x02\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U`v\x80T\x80a<\xF1Wa<\xF1aH\xF1V[_\x82\x81R` \x80\x82 \x83\x01_\x19\x90\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U\x90\x92\x01\x90\x92U`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x81R`w\x90\x93RPP`@\x81 \x81\x81U`\x01\x81\x01\x91\x90\x91U`\x02\x01\x80T\x7F\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90UV[``\x82G\x10\x15a=\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01R\x7Fr call\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nSV[`\x01`\x01`\xA0\x1B\x03\x85\x16;a>KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\nSV[_\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa>f\x91\x90aI\x05V[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a>\xA0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a>\xA5V[``\x91P[P\x91P\x91Pa>\xB5\x82\x82\x86a>\xC0V[\x97\x96PPPPPPPV[``\x83\x15a>\xCFWP\x81a\x10\xC3V[\x82Q\x15a>\xDFW\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nS\x91\x90aI\x1BV[`@Q\x80`@\x01`@R\x80a?\x0Ca?\x1EV[\x81R` \x01a?\x19a?\x1EV[\x90R\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[_`\xA0\x82\x84\x03\x12\x15a?LW_\x80\xFD[P\x91\x90PV[_\x80`\xC0\x83\x85\x03\x12\x15a?cW_\x80\xFD[\x825\x91Pa?t\x84` \x85\x01a?<V[\x90P\x92P\x92\x90PV[_`\xC0\x82\x84\x03\x12\x15a?LW_\x80\xFD[_\x80_\x80_\x80a\x01\xE0\x87\x89\x03\x12\x15a?\xA3W_\x80\xFD[\x865\x95P` \x87\x015\x94Pa?\xBB\x88`@\x89\x01a?}V[\x93Pa\x01\0\x87\x015\x92Pa?\xD3\x88a\x01 \x89\x01a?<V[\x91Pa\x01\xC0\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[_` \x82\x84\x03\x12\x15a?\xF7W_\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x16eW_\x80\xFD[_` \x82\x84\x03\x12\x15a@\"W_\x80\xFD[\x815a\x10\xC3\x81a?\xFEV[_\x80_a\x01\0\x84\x86\x03\x12\x15a@@W_\x80\xFD[\x835\x92Pa@Q\x85` \x86\x01a?}V[\x91P`\xE0\x84\x015\x90P\x92P\x92P\x92V[_a\x02`\x82\x84\x03\x12\x15a?LW_\x80\xFD[_\x80a\x02\x80\x83\x85\x03\x12\x15a@\x84W_\x80\xFD[a@\x8E\x84\x84a@aV[\x94a\x02`\x93\x90\x93\x015\x93PPPV[_\x80_\x80a\x02\xC0\x85\x87\x03\x12\x15a@\xB1W_\x80\xFD[\x845\x93Pa@\xC2\x86` \x87\x01a@aV[\x92Pa\x02\x80\x85\x015\x91Pa\x02\xA0\x85\x015a@\xDB\x81a?\xFEV[\x93\x96\x92\x95P\x90\x93PPV[_\x80_\x80a\x01 \x85\x87\x03\x12\x15a@\xFAW_\x80\xFD[\x845\x93P` \x85\x015\x92PaA\x12\x86`@\x87\x01a?}V[\x93\x96\x92\x95P\x92\x93a\x01\0\x015\x92PPV[_\x80`@\x83\x85\x03\x12\x15aA4W_\x80\xFD[\x825\x91P` \x83\x015aAF\x81a?\xFEV[\x80\x91PP\x92P\x92\x90PV[_\x80_``\x84\x86\x03\x12\x15aAcW_\x80\xFD[\x835aAn\x81a?\xFEV[\x92P` \x84\x015aA~\x81a?\xFEV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x16eW_\x80\xFD[_` \x82\x84\x03\x12\x15aA\xB4W_\x80\xFD[\x815a\x10\xC3\x81aA\x8FV[_\x80_a\x02\xA0\x84\x86\x03\x12\x15aA\xD2W_\x80\xFD[\x835\x92PaA\xE3\x85` \x86\x01a@aV[\x91Pa\x02\x80\x84\x015\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x81\x10a\x16eWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_`\xC0\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01R\x80`@\x85\x01Q\x16`@\x84\x01RP``\x83\x01Q\x15\x15``\x83\x01R`\x80\x83\x01QaBm\x81aB\x08V[\x80`\x80\x84\x01RP`\xA0\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aB\xC4W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aB\x9FV[P\x90\x96\x95PPPPPPV[_\x80_\x80a\x01 \x85\x87\x03\x12\x15aB\xE4W_\x80\xFD[\x845\x93PaB\xF5\x86` \x87\x01a?}V[\x93\x96\x93\x95PPPP`\xE0\x82\x015\x91a\x01\0\x015\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a6\xC3Wa6\xC3aC\x0BV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Qa\x01\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aCxWaCxaC@V[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aCxWaCxaC@V[\x80Qa\x0F8\x81a?\xFEV[\x80Qa\x0F8\x81aA\x8FV[\x80Q`\x02\x81\x10a\x0F8W_\x80\xFD[\x80Q`\xFF\x81\x16\x81\x14a\x0F8W_\x80\xFD[\x80Q\x80\x15\x15\x81\x14a\x0F8W_\x80\xFD[_a\x01\xE0\x82\x84\x03\x12\x15aC\xF5W_\x80\xFD[aC\xFDaCTV[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R`\x80\x83\x01Q`\x80\x82\x01R`\xA0\x83\x01Q`\xA0\x82\x01R`\xC0\x83\x01Q`\xC0\x82\x01R`\xE0\x83\x01Q`\xE0\x82\x01Ra\x01\0aDU\x81\x85\x01aC\xA1V[\x90\x82\x01Ra\x01 aDg\x84\x82\x01aC\xACV[\x90\x82\x01Ra\x01@aDy\x84\x82\x01aC\xACV[\x90\x82\x01Ra\x01`aD\x8B\x84\x82\x01aC\xB7V[\x90\x82\x01Ra\x01\x80aD\x9D\x84\x82\x01aC\xC5V[\x90\x82\x01Ra\x01\xA0aD\xAF\x84\x82\x01aC\xD5V[\x90\x82\x01Ra\x01\xC0aD\xC1\x84\x82\x01aC\xACV[\x90\x82\x01R\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aD\xDBW_\x80\xFD[aD\xE3aC~V[\x80`@\x84\x01\x85\x81\x11\x15aD\xF4W_\x80\xFD[\x84[\x81\x81\x10\x15aE\x17W\x805aE\t\x81aA\x8FV[\x84R` \x93\x84\x01\x93\x01aD\xF6V[P\x90\x95\x94PPPPPV[_`\x80\x82\x84\x03\x12\x15aE2W_\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aEUWaEUaC@V[`@R\x90P\x80`\x1F\x83\x01\x84\x13aEiW_\x80\xFD[aEqaC~V[\x80`@\x85\x01\x86\x81\x11\x15aE\x82W_\x80\xFD[\x85[\x81\x81\x10\x15aE\x9CW\x805\x84R` \x93\x84\x01\x93\x01aE\x84V[P\x81\x84RaE\xAA\x87\x82aD\xCCV[` \x85\x01RPPPP\x92\x91PPV[\x805`\x03\x81\x10a\x0F8W_\x80\xFD[_`\xC0\x82\x84\x03\x12\x15aE\xD7W_\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aE\xFAWaE\xFAaC@V[`@RaF\x07\x84\x84aE\"V[\x81RaF\x15`\x80\x84\x01aE\xB9V[` \x82\x01R`\xA0\x92\x90\x92\x015`@\x83\x01RP\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x0EzWa\x0EzaC\x0BV[_`\x80\x82\x84\x03\x12\x15aFOW_\x80\xFD[a\x10\xC3\x83\x83aE\"V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a6\xC3Wa6\xC3aC\x0BV[_` \x82\x84\x03\x12\x15aF\x8AW_\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aF\xB5W_\x80\xFD[\x81Qa\x10\xC3\x81a?\xFEV[\x80\x82\x01\x80\x82\x11\x15a\x0EzWa\x0EzaC\x0BV[_` \x82\x84\x03\x12\x15aF\xE3W_\x80\xFD[a\x10\xC3\x82aE\xB9V[aF\xF5\x81aB\x08V[\x90RV[`@\x81\x837`@\x82\x01`@\x82\x01_[`\x02\x81\x10\x15aG;W\x815aG\x1C\x81aA\x8FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01aG\x08V[PPPaGJ`\x80\x82\x01aE\xB9V[aGS\x81aB\x08V[`\x80\x83\x01R`\xA0\x90\x81\x015\x91\x01RV[_a\x03 \x82\x01\x90P\x885\x82R` \x89\x015` \x83\x01R`@\x89\x015`@\x83\x01R``\x89\x015``\x83\x01R`\x80\x89\x015aG\x9B\x81a?\xFEV[`\x01`\x01`\xA0\x1B\x03\x16`\x80\x83\x01R`\xA0\x89\x015aG\xB7\x81aA\x8FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\xA0\x84\x01R`\xC0\x8A\x015\x90aG\xD7\x82aA\x8FV[\x16`\xC0\x83\x01RaG\xED`\xE0\x80\x84\x01\x90\x8B\x01aF\xF9V[a\x01\xA0aG\xFE\x81\x84\x01\x82\x8C\x01aF\xF9V[P\x87a\x02`\x83\x01R\x86a\x02\x80\x83\x01R\x85a\x02\xA0\x83\x01R\x84a\x02\xC0\x83\x01RaH1a\x02\xE0\x83\x01\x85`\x01`\x01`\xA0\x1B\x03\x16\x90RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x03\0\x83\x01R\x98\x97PPPPPPPPV[\x81Q\x80Q`\xC0\x83\x01\x91\x90\x83_[`\x02\x81\x10\x15aHzW\x82Q\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01aH[V[PPP` \x90\x81\x01Q\x90`@\x84\x01_[`\x02\x81\x10\x15aH\xB1W\x83Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R\x92\x82\x01\x92\x90\x82\x01\x90`\x01\x01aH\x8AV[PPPP` \x83\x01QaH\xC7`\x80\x84\x01\x82aF\xECV[P`@\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[_` \x82\x84\x03\x12\x15aH\xE8W_\x80\xFD[a\x10\xC3\x82aC\xD5V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xB0*u\xC2{(\x1A\xB5\x1B\x1A\x9B\x90\xFA\xAA\x01\x02q\xB8\x99;2\xB3\x02Y\x95`\xAD\x12\xCD\xDDW\xE4dsolcC\0\x08\x19\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060043610610393575f3560e01c806365f7f80d116101df578063bc45e0ae11610109578063e6b3082c116100a9578063ef40a67011610079578063ef40a67014610982578063f065de3f146109aa578063facd743b146109bd578063fb0e722b146109d0575f80fd5b8063e6b3082c146108b1578063e78cea92146108cc578063e8bd4922146108df578063ee35f3271461097a575f80fd5b8063ce11e6ab116100e4578063ce11e6ab14610870578063dff6978714610883578063e51019a61461088b578063e531d8c71461089e575f80fd5b8063bc45e0ae14610842578063c2c2e68e14610855578063c4d66de81461085d575f80fd5b806384728cd01161017f5780639a8a05921161014f5780639a8a05921461070e578063a23c44b114610717578063aa38a6e71461081a578063b7ab4db51461082d575f80fd5b806384728cd0146106a257806388302884146106dd5780638da5cb5b146106fd5780638ee1a12614610705575f80fd5b80636ddd3744116101ba5780636ddd37441461066a57806371ef232c1461067d5780637300201c1461068657806376e7e23b14610699575f80fd5b806365f7f80d1461063c57806368129b1414610644578063685f5ecc14610657575f80fd5b80633b86de19116102c057806356bbc9e6116102605780636096686d116102305780636096686d146105d957806361373919146105ec5780636177fd18146105f45780636420fb9f14610629575f80fd5b806356bbc9e6146105a057806357ef4ab9146105b3578063588c7a16146105bb5780635c975abb146105ce575f80fd5b806350f32f681161029b57806350f32f681461055a57806351ed6a301461056d57806352d1902d1461058057806355840a5814610588575f80fd5b80633b86de19146105125780633be680ea1461052557806345e38b6414610551575f80fd5b80631b1689e9116103365780632f30cabd116103065780632f30cabd146104bc57806330836228146104e457806333635fc2146104f7578063353325e01461050a575f80fd5b80631b1689e9146104615780631e83d30f1461046a5780632abdd2301461047d5780632e7acfa6146104a8575f80fd5b8063117155851161037157806311715585146103ef57806312ab3d3b1461041b57806313c56ca71461043857806318baaab914610459575f80fd5b8063023a96fe1461039757806304972af9146103c757806310b98a35146103dc575b5f80fd5b6069546103aa906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b6103da6103d5366004613f52565b6109e3565b005b6103da6103ea366004613f8d565b6109fd565b6104026103fd366004613fe7565b610ddd565b60405167ffffffffffffffff90911681526020016103be565b607b546104289060ff1681565b60405190151581526020016103be565b61044b610446366004613fe7565b610df8565b6040519081526020016103be565b6103da610f3d565b61044b607a5481565b6103da610478366004613fe7565b611011565b61044b61048b366004614012565b6001600160a01b03165f9081526077602052604090206001015490565b6066546104029067ffffffffffffffff1681565b61044b6104ca366004614012565b6001600160a01b03165f9081526078602052604090205490565b6104286104f2366004613fe7565b61108b565b61044b61050536600461402d565b6110a6565b61044b6110ca565b6103da610520366004614072565b611104565b6069546104029074010000000000000000000000000000000000000000900467ffffffffffffffff1681565b61044b60715481565b6103da61056836600461409d565b61149b565b6070546103aa906001600160a01b031681565b61044b611514565b607b546103aa9061010090046001600160a01b031681565b6104026105ae366004613fe7565b6115d8565b6103da6115fa565b6103da6105c9366004614012565b611668565b60335460ff16610428565b6103da6105e73660046140e6565b611756565b61044b6117c9565b610428610602366004614012565b6001600160a01b03165f90815260776020526040902060020154600160401b900460ff1690565b6103da610637366004614072565b611849565b60745461044b565b6103da610652366004614123565b6119fe565b6103da610665366004614151565b611a75565b6103aa6106783660046141a4565b611a91565b61044b60795481565b6103da6106943660046141bf565b611ac9565b61044b60675481565b6103aa6106b0366004614012565b6001600160a01b039081165f90815260776020526040902060020154690100000000000000000090041690565b6106f06106eb366004613fe7565b611ad5565b6040516103be9190614224565b6103aa611ba0565b61044b60685481565b61044b60655481565b6107c4610725366004614012565b6040805160a0810182525f80825260208201819052918101829052606081018290526080810191909152506001600160a01b039081165f90815260776020908152604091829020825160a081018452815481526001820154928101929092526002015467ffffffffffffffff81169282019290925260ff600160401b830416151560608201526901000000000000000000909104909116608082015290565b6040516103be91905f60a082019050825182526020830151602083015267ffffffffffffffff60408401511660408301526060830151151560608301526001600160a01b03608084015116608083015292915050565b606d546103aa906001600160a01b031681565b610835611ba9565b6040516103be9190614284565b606e546103aa906001600160a01b031681565b6103da611bb5565b6103da61086b366004614012565b611c77565b606c546103aa906001600160a01b031681565b607654610402565b6103da6108993660046142d0565b611e19565b6104286108ac366004613fe7565b611e80565b60665461040290600160401b900467ffffffffffffffff1681565b606b546103aa906001600160a01b031681565b61093a6108ed366004614012565b60776020525f908152604090208054600182015460029092015490919067ffffffffffffffff811690600160401b810460ff1690690100000000000000000090046001600160a01b031685565b60408051958652602086019490945267ffffffffffffffff9092169284019290925290151560608301526001600160a01b0316608082015260a0016103be565b6103aa611eaf565b61044b610990366004614012565b6001600160a01b03165f9081526077602052604090205490565b606f546103aa906001600160a01b031681565b6104286109cb366004614012565b611f33565b606a546103aa906001600160a01b031681565b6109f9816109f084611f52565b60010154611fb1565b5050565b335f81815260736020526040902054151580610a1b5750607b5460ff165b610a5c5760405162461bcd60e51b815260206004820152600d60248201526c2727aa2fab20a624a220aa27a960991b60448201526064015b60405180910390fd5b610a646120b6565b5f610a6e88611f52565b90505f610a7a88611f52565b9050610a8a858260010154611fb1565b610a9a60808601606087016141a4565b8254610ab79190600160801b900467ffffffffffffffff1661431f565b67ffffffffffffffff16431015610b105760405162461bcd60e51b815260206004820152600f60248201527f4245464f52455f444541444c494e4500000000000000000000000000000000006044820152606401610a53565b6074548814610b615760405162461bcd60e51b815260206004820152601960248201527f505245565f4e4f545f4c41544553545f434f4e4649524d4544000000000000006044820152606401610a53565b8054600160401b900467ffffffffffffffff1615610dc6575f610b8a6060870160408801614012565b6001600160a01b031663fda2892e886040518263ffffffff1660e01b8152600401610bb791815260200190565b6101e060405180830381865afa158015610bd3573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610bf791906143e4565b9050898160e0015114610c4c5760405162461bcd60e51b815260206004820152600a60248201527f4e4f545f57494e4e4552000000000000000000000000000000000000000000006044820152606401610a53565b60018161016001516001811115610c6557610c656141f4565b14610cb25760405162461bcd60e51b815260206004820152601260248201527f454447455f4e4f545f434f4e4649524d454400000000000000000000000000006044820152606401610a53565b80610140015167ffffffffffffffff165f03610d105760405162461bcd60e51b815260206004820152601760248201527f5a45524f5f434f4e4649524d45445f41545f424c4f434b0000000000000000006044820152606401610a53565b606954610140820151610d459174010000000000000000000000000000000000000000900467ffffffffffffffff169061431f565b67ffffffffffffffff16431015610dc45760405162461bcd60e51b815260206004820152602160248201527f4348414c4c454e47455f47524143455f504552494f445f4e4f545f504153534560448201527f44000000000000000000000000000000000000000000000000000000000000006064820152608401610a53565b505b610dd28989898761210b565b505050505050505050565b5f610de782611f52565b5467ffffffffffffffff1692915050565b5f7f000000000000000000000000000000000000000000000000000000000000000015610e80575f828152607c602052604090205480610e7a5760405162461bcd60e51b815260206004820152600c60248201527f4e4f5f415353455254494f4e00000000000000000000000000000000000000006044820152606401610a53565b92915050565b5f610e8a83611f52565b6040805160c081018252825467ffffffffffffffff8082168352600160401b820481166020840152600160801b8204169282019290925260ff600160c01b8304811615156060830152929350610f2092909184916080840191600160c81b9004166002811115610efc57610efc6141f4565b6002811115610f0d57610f0d6141f4565b815260200160018201548152505061231a565b54600160801b900467ffffffffffffffff1692915050565b919050565b607b5460ff1615610f905760405162461bcd60e51b815260206004820152601260248201527f57484954454c4953545f44495341424c454400000000000000000000000000006044820152606401610a53565b610f9861237e565b610fe45760405162461bcd60e51b815260206004820152601160248201527f56414c494441544f525f4e4f545f41464b0000000000000000000000000000006044820152606401610a53565b607b80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00166001179055565b335f8181526073602052604090205415158061102f5750607b5460ff165b61106b5760405162461bcd60e51b815260206004820152600d60248201526c2727aa2fab20a624a220aa27a960991b6044820152606401610a53565b6110736120b6565b61107c336124ad565b611086338361259c565b505050565b5f61109582611f52565b54600160c01b900460ff1692915050565b5f6110c0846110ba368690038601866145c7565b8461269b565b90505b9392505050565b5f6110d3613ef9565b60408051606081018252828152600160208201525f91810182905290806110fb81848161269b565b94505050505090565b335f818152607360205260409020541515806111225750607b5460ff165b61115e5760405162461bcd60e51b815260206004820152600d60248201526c2727aa2fab20a624a220aa27a960991b6044820152606401610a53565b6111666120b6565b81158061119657505f61117883611f52565b54600160c81b900460ff166002811115611194576111946141f4565b145b6111e25760405162461bcd60e51b815260206004820152601760248201527f45585045435445445f415353455254494f4e5f5345454e0000000000000000006044820152606401610a53565b335f90815260776020526040902060020154600160401b900460ff166112375760405162461bcd60e51b815260206004820152600a6024820152691393d517d4d51052d15160b21b6044820152606401610a53565b335f90815260776020526040902054606084013511156112995760405162461bcd60e51b815260206004820152601260248201527f494e53554646494349454e545f5354414b4500000000000000000000000000006044820152606401610a53565b5f6112bb84356112b136879003870160e088016145c7565b602087013561269b565b90506113346112c982611f52565b6040805160c081018252825467ffffffffffffffff8082168352600160401b820481166020840152600160801b8204169282019290925260ff600160c01b83048116151560608301529092916080840191600160c81b909104166002811115610efc57610efc6141f4565b335f908152607760205260409020600101548181148061136657505f61135982611f52565b5467ffffffffffffffff16115b6113b25760405162461bcd60e51b815260206004820152601860248201527f5354414b45445f4f4e5f414e4f544845525f4252414e434800000000000000006044820152606401610a53565b5f806113bf8785886126de565b335f908152607760205260409020600101829055909250905080611459575f6113e785611f52565b5461140390600160801b900467ffffffffffffffff164361462c565b90506071548110156114575760405162461bcd60e51b815260206004820152600a60248201527f54494d455f44454c5441000000000000000000000000000000000000000000006044820152606401610a53565b505b61146282611f52565b54600160c01b900460ff1661149257606f54607054611492916001600160a01b03918216911660608a013561304b565b50505050505050565b6001600160a01b0381166114f15760405162461bcd60e51b815260206004820152601860248201527f454d5054595f5749544844524157414c5f4144445245535300000000000000006044820152606401610a53565b6114fb84826130f4565b6115058383611104565b61150e846131ca565b50505050565b5f306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146115b35760405162461bcd60e51b815260206004820152603b60248201527f555550534e6f745570677261646561626c653a206d757374206e6f742062652060448201527f63616c6c6564207468726f7567682064656c656761746563616c6c00000000006064820152608401610a53565b507f2b1dbce74324248c222f0ec2d5ed7bd323cfc425b336f0253c5ccfda7265546d90565b5f6115e282611f52565b54600160401b900467ffffffffffffffff1692915050565b335f818152607360205260409020541515806116185750607b5460ff165b6116545760405162461bcd60e51b815260206004820152600d60248201526c2727aa2fab20a624a220aa27a960991b6044820152606401610a53565b61165c6120b6565b611665336131e2565b50565b6001600160a01b0381165f9081526073602052604090205481901515806116915750607b5460ff165b6116cd5760405162461bcd60e51b815260206004820152600d60248201526c2727aa2fab20a624a220aa27a960991b6044820152606401610a53565b6116d56120b6565b6001600160a01b038281165f908152607760205260409020600201546901000000000000000000900416331461174d5760405162461bcd60e51b815260206004820152601660248201527f4e4f545f5749544844524157414c5f41444452455353000000000000000000006044820152606401610a53565b6109f9826131e2565b61175e6120b6565b607b5461010090046001600160a01b031633146117bd5760405162461bcd60e51b815260206004820152601260248201527f4e4f545f464153545f434f4e4649524d455200000000000000000000000000006044820152606401610a53565b61150e8484848461210b565b5f6117d26120b6565b5f6117dc336131f4565b90505f811161182d5760405162461bcd60e51b815260206004820152601460248201527f4e4f5f46554e44535f544f5f57495448445241570000000000000000000000006044820152606401610a53565b607054611844906001600160a01b0316338361304b565b905090565b6118516120b6565b8061189e5760405162461bcd60e51b815260206004820152601760248201527f45585045435445445f415353455254494f4e5f484153480000000000000000006044820152606401610a53565b5f6118a882611f52565b54600160c81b900460ff1690505f6118cd84356112b136879003870160e088016145c7565b90506118db6112c982611f52565b5f8260028111156118ee576118ee6141f4565b0361193d575f6118ff8583866126de565b50905061190b81611f52565b54600160c01b900460ff1661193b57606f5460705461193b916001600160a01b039182169116606088013561304b565b505b606b5461150e90849083906101a08801906001600160a01b03166316bf55796001611975611970368d90038d018661463f565b613273565b61197f9190614659565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815267ffffffffffffffff9091166004820152602401602060405180830381865afa1580156119da573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105e7919061467a565b611a066120b6565b6001600160a01b038116611a5c5760405162461bcd60e51b815260206004820152601860248201527f454d5054595f5749544844524157414c5f4144445245535300000000000000006044820152606401610a53565b611a6682826130f4565b81156109f9576109f9826131ca565b611a7d6120b6565b611a88838383613287565b611086816131ca565b5f60768267ffffffffffffffff1681548110611aaf57611aaf614691565b5f918252602090912001546001600160a01b031692915050565b6110868383833361149b565b6040805160c0810182525f80825260208201819052918101829052606081018290526080810182905260a0810191909152611b0f82611f52565b6040805160c081018252825467ffffffffffffffff8082168352600160401b820481166020840152600160801b8204169282019290925260ff600160c01b83048116151560608301529092916080840191600160c81b909104166002811115611b7a57611b7a6141f4565b6002811115611b8b57611b8b6141f4565b81526020016001820154815250509050919050565b5f6118446133ed565b6060611844607261341f565b607b5460ff1615611c085760405162461bcd60e51b815260206004820152601260248201527f57484954454c4953545f44495341424c454400000000000000000000000000006044820152606401610a53565b467f000000000000000000000000000000000000000000000000000000000000000003610fe45760405162461bcd60e51b815260206004820152601460248201527f434841494e5f49445f4e4f545f4348414e4745440000000000000000000000006044820152606401610a53565b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163003611d155760405162461bcd60e51b815260206004820152602c60248201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060448201527f64656c656761746563616c6c00000000000000000000000000000000000000006064820152608401610a53565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316611d4761342b565b6001600160a01b031614611dc35760405162461bcd60e51b815260206004820152602c60248201527f46756e6374696f6e206d7573742062652063616c6c6564207468726f7567682060448201527f6163746976652070726f787900000000000000000000000000000000000000006064820152608401610a53565b6001600160a01b0381166116655760405162461bcd60e51b815260206004820152601060248201527f4e4545445f5354414b455f544f4b454e000000000000000000000000000000006044820152606401610a53565b611e3282611e2c368690038601866145c7565b8361269b565b841461150e5760405162461bcd60e51b815260206004820152601660248201527f494e56414c49445f415353455254494f4e5f48415348000000000000000000006044820152606401610a53565b5f6001611e8c83611f52565b54600160c81b900460ff166002811115611ea857611ea86141f4565b1492915050565b606b54604080517fee35f32700000000000000000000000000000000000000000000000000000000815290515f926001600160a01b03169163ee35f3279160048083019260209291908290030181865afa158015611f0f573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061184491906146a5565b6001600160a01b0381165f908152607360205260408120541515610e7a565b5f81611fa05760405162461bcd60e51b815260206004820152601b60248201527f415353455254494f4e5f49445f43414e4e4f545f42455f5a45524f00000000006044820152606401610a53565b505f90815260756020526040902090565b61206882356020840135611fcb6060860160408701614012565b611fdb60808701606088016141a4565b611feb60a08801608089016141a4565b60408051602080820197909752808201959095526bffffffffffffffffffffffff19606094851b16938501939093527fffffffffffffffff00000000000000000000000000000000000000000000000060c092831b81166074860152911b16607c8301528051606481840301815260849092019052805191012090565b81146109f95760405162461bcd60e51b815260206004820152601460248201527f434f4e4649475f484153485f4d49534d415443480000000000000000000000006044820152606401610a53565b60335460ff16156121095760405162461bcd60e51b815260206004820152601060248201527f5061757361626c653a20706175736564000000000000000000000000000000006044820152606401610a53565b565b5f61211585611f52565b905060018154600160c81b900460ff166002811115612136576121366141f4565b146121835760405162461bcd60e51b815260206004820152600b60248201527f4e4f545f50454e44494e470000000000000000000000000000000000000000006044820152606401610a53565b612196846110ba368690038601866145c7565b85146121e45760405162461bcd60e51b815260206004820152600c60248201527f434f4e4649524d5f4441544100000000000000000000000000000000000000006044820152606401610a53565b5f6121fc6121f73686900386018661463f565b613452565b90505f6122166122113687900387018761463f565b61345c565b606c546040517fa04cee6000000000000000000000000000000000000000000000000000000000815260048101839052602481018590529192506001600160a01b03169063a04cee60906044015f604051808303815f87803b15801561227a575f80fd5b505af115801561228c573d5f803e3d5ffd5b50505060748890555082547fffffffffffff00ffffffffffffffffffffffffffffffffffffffffffffffffff167902000000000000000000000000000000000000000000000000001783556040805183815260208101839052815189927ffc42829b29c259a7370ab56c8f69fce23b5f351a9ce151da453281993ec0090c928290030190a250505050505050565b5f81608001516002811115612331576123316141f4565b036116655760405162461bcd60e51b815260206004820152601360248201527f415353455254494f4e5f4e4f545f4558495354000000000000000000000000006044820152606401610a53565b5f8061239161238c60745490565b611f52565b6040805160c081018252825467ffffffffffffffff8082168352600160401b820481166020840152600160801b8204169282019290925260ff600160c01b83048116151560608301529092916080840191600160c81b9091041660028111156123fc576123fc6141f4565b600281111561240d5761240d6141f4565b815260019190910154602090910152606654909150600160401b900467ffffffffffffffff165f819003612443575f9250505090565b816040015167ffffffffffffffff165f03612460575f9250505090565b815167ffffffffffffffff1615612492578151439061248a90839067ffffffffffffffff166146c0565b109250505090565b4381836040015167ffffffffffffffff1661248a91906146c0565b6001600160a01b0381165f90815260776020526040902060020154600160401b900460ff1661250b5760405162461bcd60e51b815260206004820152600a6024820152691393d517d4d51052d15160b21b6044820152606401610a53565b6001600160a01b0381165f908152607760205260408120600101546074549091908214908061253984611f52565b5467ffffffffffffffff1611905081806125505750805b61150e5760405162461bcd60e51b815260206004820152600c60248201527f5354414b455f41435449564500000000000000000000000000000000000000006044820152606401610a53565b6001600160a01b038083165f9081526077602052604081206002810154815492939192690100000000000000000090910490911690808511156126215760405162461bcd60e51b815260206004820152601060248201527f544f4f5f4c4954544c455f5354414b45000000000000000000000000000000006044820152606401610a53565b5f61262c868361462c565b868555905061263b8382613467565b826001600160a01b0316876001600160a01b03167fd957cf2340073335d256f72a9ef89cf1a43c31143341a6a53575ef33e987beb88489604051612689929190918252602082015260400190565b60405180910390a39695505050505050565b5f6110c0846126a98561350a565b604080516020808201949094528082019290925260608083018790528151808403909101815260809092019052805191012090565b5f806126f0604086016109f086611f52565b6001612704610240870161022088016146d3565b6002811115612715576127156141f4565b148061274357506002612730610240870161022088016146d3565b6002811115612741576127416141f4565b145b61278f5760405162461bcd60e51b815260206004820152601060248201527f4241445f41465445525f535441545553000000000000000000000000000000006044820152606401610a53565b836127b186356127a736899003890160e08a016145c7565b602089013561269b565b146127fe5760405162461bcd60e51b815260206004820152601460248201527f494e56414c49445f4245464f52455f53544154450000000000000000000000006044820152606401610a53565b6001612812610180870161016088016146d3565b6002811115612823576128236141f4565b146128705760405162461bcd60e51b815260206004820152600f60248201527f4241445f505245565f53544154555300000000000000000000000000000000006044820152606401610a53565b5f61287a85611f52565b90505f806101a0880160e08901826128928383613539565b12156128e05760405162461bcd60e51b815260206004820152600f60248201527f494e424f585f4241434b574152445300000000000000000000000000000000006044820152606401610a53565b5f6129056128f460e08d0160c08e016141a4565b849067ffffffffffffffff1661363f565b90505f8113156129575760405162461bcd60e51b815260206004820152600d60248201527f494e424f585f544f4f5f464152000000000000000000000000000000000000006044820152606401610a53565b600261296b6102408d016102208e016146d3565b600281111561297c5761297c6141f4565b1415801561298957505f81125b156129ea57600196505f61299d8484613539565b136129ea5760405162461bcd60e51b815260206004820152601360248201527f4f564552464c4f575f5354414e445354494c4c000000000000000000000000006044820152606401610a53565b606b54604080517e84120c00000000000000000000000000000000000000000000000000000000815290515f926001600160a01b0316916284120c9160048083019260209291908290030181865afa158015612a48573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612a6c919061467a565b90505f612a79858361363f565b1315612ac75760405162461bcd60e51b815260206004820152600e60248201527f494e424f585f504153545f454e440000000000000000000000000000000000006044820152606401610a53565b80612ad860e08e0160c08f016141a4565b67ffffffffffffffff161115612b305760405162461bcd60e51b815260206004820152601360248201527f494e424f585f4e4f545f504f50554c41544544000000000000000000000000006044820152606401610a53565b5f612b436119703687900387018761463f565b67ffffffffffffffff169050818103612b6857612b618260016146c0565b9650612b6c565b8196505b805f03612bbb5760405162461bcd60e51b815260206004820152601160248201527f454d5054595f494e424f585f434f554e540000000000000000000000000000006044820152606401610a53565b606b546001600160a01b03166316bf5579612bd760018461462c565b6040518263ffffffff1660e01b8152600401612bf591815260200190565b602060405180830381865afa158015612c10573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c34919061467a565b95505050505050612c5487896101a001803603810190611e2c91906145c7565b945085851480612c62575085155b612cae5760405162461bcd60e51b815260206004820152601960248201527f554e45585045435445445f415353455254494f4e5f48415348000000000000006044820152606401610a53565b5f612cb886611f52565b54600160c81b900460ff166002811115612cd457612cd46141f4565b14612d215760405162461bcd60e51b815260206004820152600e60248201527f415353455254494f4e5f5345454e0000000000000000000000000000000000006044820152606401610a53565b825460685460675460695460665460408051602080820196909652808201949094526bffffffffffffffffffffffff19606093841b16838501527fffffffffffffffff00000000000000000000000000000000000000000000000060c092831b8116607486015288831b16607c8501528051606481860301815260848501808352815191870191909120610144860183525f9182905260a4860182905260c4860182905260e4860182905261010486018290526101249095018190528151928301825280835294820185905267ffffffffffffffff43811691830191909152909416159084015260a08301526001608083015250612e1e846136ca565b5f8681526075602090815260409182902083518154928501519385015160608601511515600160c01b027fffffffffffffff00ffffffffffffffffffffffffffffffffffffffffffffffff67ffffffffffffffff928316600160801b02167fffffffffffffff000000000000000000ffffffffffffffffffffffffffffffff968316600160401b027fffffffffffffffffffffffffffffffff00000000000000000000000000000000909616929093169190911793909317939093169290921717808255608083015183929182907fffffffffffff00ffffffffffffffffffffffffffffffffffffffffffffffffff16600160c81b836002811115612f2557612f256141f4565b021790555060a0820151816001015590505087867f901c3aee23cf4478825462caaab375c606ab83516060388344f06503407536308b858760685460675460695f9054906101000a90046001600160a01b031660665f9054906101000a900467ffffffffffffffff16604051612fa19796959493929190614763565b60405180910390a37f00000000000000000000000000000000000000000000000000000000000000001561303f5760646001600160a01b031663a3b1b31d6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561300c573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613030919061467a565b5f878152607c60205260409020555b50505050935093915050565b6040516001600160a01b0383166024820152604481018290526110869084907fa9059cbb00000000000000000000000000000000000000000000000000000000906064015b60408051601f198184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fffffffff000000000000000000000000000000000000000000000000000000009093169290921790915261374c565b335f818152607360205260409020541515806131125750607b5460ff165b61314e5760405162461bcd60e51b815260206004820152600d60248201526c2727aa2fab20a624a220aa27a960991b6044820152606401610a53565b6131566120b6565b335f90815260776020526040902060020154600160401b900460ff16156131bf5760405162461bcd60e51b815260206004820152600e60248201527f414c52454144595f5354414b45440000000000000000000000000000000000006044820152606401610a53565b611086338484613830565b607054611665906001600160a01b03163330846139ab565b6131eb816124ad565b611665816139fc565b6001600160a01b0381165f90815260786020526040812080549082905560798054829190849061322590849061462c565b9091555050604080518281525f60208201526001600160a01b038516917fa740af14c56e4e04a617b1de1eb20de73270decbaaead14f142aabf3038e5ae2910160405180910390a292915050565b60208101515f90815b602002015192915050565b6001600160a01b0383165f9081526073602052604090205483901515806132b05750607b5460ff165b6132ec5760405162461bcd60e51b815260206004820152600d60248201526c2727aa2fab20a624a220aa27a960991b6044820152606401610a53565b6132f46120b6565b6001600160a01b0384165f90815260776020526040902060020154600160401b900460ff166133525760405162461bcd60e51b815260206004820152600a6024820152691393d517d4d51052d15160b21b6044820152606401610a53565b826001600160a01b031661338d856001600160a01b039081165f90815260776020526040902060020154690100000000000000000090041690565b6001600160a01b0316146133e35760405162461bcd60e51b815260206004820152601860248201527f57524f4e475f5749544844524157414c5f4144445245535300000000000000006044820152606401610a53565b61150e8483613a85565b5f7fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61035b546001600160a01b0316919050565b60605f6110c383613b12565b5f7f2b1dbce74324248c222f0ec2d5ed7bd323cfc425b336f0253c5ccfda7265546d613410565b80515f908161327c565b80515f90600161327c565b6001600160a01b0382165f908152607860205260408120549061348a83836146c0565b6001600160a01b0385165f9081526078602052604081208290556079805492935085929091906134bb9084906146c0565b909155505060408051838152602081018390526001600160a01b038616917fa740af14c56e4e04a617b1de1eb20de73270decbaaead14f142aabf3038e5ae2910160405180910390a250505050565b5f8160405160200161351c919061484e565b604051602081830303815290604052805190602001209050919050565b5f8061354d6119703686900386018661463f565b90505f6135626119703686900386018661463f565b90508067ffffffffffffffff168267ffffffffffffffff16101561358b575f1992505050610e7a565b8067ffffffffffffffff168267ffffffffffffffff1611156135b257600192505050610e7a565b5f6135ca6135c53688900388018861463f565b613b6b565b90505f6135df6135c53688900388018861463f565b90508067ffffffffffffffff168267ffffffffffffffff16101561360a575f19945050505050610e7a565b8067ffffffffffffffff168267ffffffffffffffff161115613633576001945050505050610e7a565b5f945050505050610e7a565b5f806136536119703686900386018661463f565b9050828167ffffffffffffffff161015613671575f19915050610e7a565b828167ffffffffffffffff16111561368d576001915050610e7a565b5f6136a06135c53687900387018761463f565b67ffffffffffffffff1611156136ba576001915050610e7a565b5f915050610e7a565b5092915050565b805467ffffffffffffffff165f036136f957805467ffffffffffffffff19164367ffffffffffffffff16179055565b8054600160401b900467ffffffffffffffff165f036116655780547fffffffffffffffffffffffffffffffff0000000000000000ffffffffffffffff16600160401b4367ffffffffffffffff1602179055565b5f6137a0826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b0316613b799092919063ffffffff16565b80519091501561108657808060200190518101906137be91906148d8565b6110865760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e60448201527f6f742073756363656564000000000000000000000000000000000000000000006064820152608401610a53565b6076805460018082019092557fb5732705f5241370a28908c2fe1303cb223f03b90d857fd0573f003f79fefed4810180547fffffffffffffffffffffffff0000000000000000000000000000000000000000166001600160a01b038781169182179092556040805160a081018252878152607454602080830191825267ffffffffffffffff808816848601908152606085018a81528b8916608087018181525f8a8152607787528981209851895596519c88019c909c5591516002909601805491519b51969093167fffffffffffffffffffffffffffffffffffffffffffffff00000000000000000090911617600160401b9a15159a909a02999099177fffffff0000000000000000000000000000000000000000ffffffffffffffffff166901000000000000000000949097169390930295909517909155815190815292830187905292939290917fd957cf2340073335d256f72a9ef89cf1a43c31143341a6a53575ef33e987beb891015b60405180910390a350505050565b6040516001600160a01b038085166024830152831660448201526064810182905261150e9085907f23b872dd0000000000000000000000000000000000000000000000000000000090608401613090565b6001600160a01b038082165f90815260776020526040902060028101548154919269010000000000000000009091041690613a378282613467565b613a4084613b87565b604080518281525f60208201526001600160a01b0380851692908716917fd957cf2340073335d256f72a9ef89cf1a43c31143341a6a53575ef33e987beb8910161399d565b6001600160a01b0382165f90815260776020526040812080549091613aaa84836146c0565b808455600284015460408051858152602081018490529293506001600160a01b036901000000000000000000909204821692918816917fd957cf2340073335d256f72a9ef89cf1a43c31143341a6a53575ef33e987beb8910160405180910390a35050505050565b6060815f01805480602002602001604051908101604052809291908181526020018280548015613b5f57602002820191905f5260205f20905b815481526020019060010190808311613b4b575b50505050509050919050565b60208101515f90600161327c565b60606110c084845f85613d7c565b6001600160a01b0381165f9081526077602052604090206002810154600160401b900460ff16613be65760405162461bcd60e51b815260206004820152600a6024820152691393d517d4d51052d15160b21b6044820152606401610a53565b60028101546076805467ffffffffffffffff90921691613c089060019061462c565b81548110613c1857613c18614691565b5f91825260209091200154607680546001600160a01b039092169167ffffffffffffffff8416908110613c4d57613c4d614691565b905f5260205f20015f6101000a8154816001600160a01b0302191690836001600160a01b031602179055508060775f60768467ffffffffffffffff1681548110613c9957613c99614691565b5f918252602080832091909101546001600160a01b031683528201929092526040019020600201805467ffffffffffffffff191667ffffffffffffffff929092169190911790556076805480613cf157613cf16148f1565b5f828152602080822083015f1990810180547fffffffffffffffffffffffff00000000000000000000000000000000000000001690559092019092556001600160a01b039490941681526077909352505060408120818155600181019190915560020180547fffffff0000000000000000000000000000000000000000000000000000000000169055565b606082471015613df45760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f60448201527f722063616c6c00000000000000000000000000000000000000000000000000006064820152608401610a53565b6001600160a01b0385163b613e4b5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401610a53565b5f80866001600160a01b03168587604051613e669190614905565b5f6040518083038185875af1925050503d805f8114613ea0576040519150601f19603f3d011682016040523d82523d5f602084013e613ea5565b606091505b5091509150613eb5828286613ec0565b979650505050505050565b60608315613ecf5750816110c3565b825115613edf5782518084602001fd5b8160405162461bcd60e51b8152600401610a53919061491b565b6040518060400160405280613f0c613f1e565b8152602001613f19613f1e565b905290565b60405180604001604052806002906020820280368337509192915050565b5f60a08284031215613f4c575f80fd5b50919050565b5f8060c08385031215613f63575f80fd5b82359150613f748460208501613f3c565b90509250929050565b5f60c08284031215613f4c575f80fd5b5f805f805f806101e08789031215613fa3575f80fd5b8635955060208701359450613fbb8860408901613f7d565b93506101008701359250613fd3886101208901613f3c565b91506101c087013590509295509295509295565b5f60208284031215613ff7575f80fd5b5035919050565b6001600160a01b0381168114611665575f80fd5b5f60208284031215614022575f80fd5b81356110c381613ffe565b5f805f6101008486031215614040575f80fd5b833592506140518560208601613f7d565b915060e084013590509250925092565b5f6102608284031215613f4c575f80fd5b5f806102808385031215614084575f80fd5b61408e8484614061565b94610260939093013593505050565b5f805f806102c085870312156140b1575f80fd5b843593506140c28660208701614061565b925061028085013591506102a08501356140db81613ffe565b939692955090935050565b5f805f8061012085870312156140fa575f80fd5b84359350602085013592506141128660408701613f7d565b939692955092936101000135925050565b5f8060408385031215614134575f80fd5b82359150602083013561414681613ffe565b809150509250929050565b5f805f60608486031215614163575f80fd5b833561416e81613ffe565b9250602084013561417e81613ffe565b929592945050506040919091013590565b67ffffffffffffffff81168114611665575f80fd5b5f602082840312156141b4575f80fd5b81356110c38161418f565b5f805f6102a084860312156141d2575f80fd5b833592506141e38560208601614061565b915061028084013590509250925092565b634e487b7160e01b5f52602160045260245ffd5b6003811061166557634e487b7160e01b5f52602160045260245ffd5b5f60c08201905067ffffffffffffffff80845116835280602085015116602084015280604085015116604084015250606083015115156060830152608083015161426d81614208565b8060808401525060a083015160a083015292915050565b602080825282518282018190525f9190848201906040850190845b818110156142c45783516001600160a01b03168352928401929184019160010161429f565b50909695505050505050565b5f805f8061012085870312156142e4575f80fd5b843593506142f58660208701613f7d565b939693955050505060e082013591610100013590565b634e487b7160e01b5f52601160045260245ffd5b67ffffffffffffffff8181168382160190808211156136c3576136c361430b565b634e487b7160e01b5f52604160045260245ffd5b6040516101e0810167ffffffffffffffff8111828210171561437857614378614340565b60405290565b6040805190810167ffffffffffffffff8111828210171561437857614378614340565b8051610f3881613ffe565b8051610f388161418f565b805160028110610f38575f80fd5b805160ff81168114610f38575f80fd5b80518015158114610f38575f80fd5b5f6101e082840312156143f5575f80fd5b6143fd614354565b825181526020830151602082015260408301516040820152606083015160608201526080830151608082015260a083015160a082015260c083015160c082015260e083015160e08201526101006144558185016143a1565b908201526101206144678482016143ac565b908201526101406144798482016143ac565b9082015261016061448b8482016143b7565b9082015261018061449d8482016143c5565b908201526101a06144af8482016143d5565b908201526101c06144c18482016143ac565b908201529392505050565b5f82601f8301126144db575f80fd5b6144e361437e565b8060408401858111156144f4575f80fd5b845b818110156145175780356145098161418f565b8452602093840193016144f6565b509095945050505050565b5f60808284031215614532575f80fd5b6040516040810181811067ffffffffffffffff8211171561455557614555614340565b604052905080601f83018413614569575f80fd5b61457161437e565b806040850186811115614582575f80fd5b855b8181101561459c578035845260209384019301614584565b508184526145aa87826144cc565b60208501525050505092915050565b803560038110610f38575f80fd5b5f60c082840312156145d7575f80fd5b6040516060810181811067ffffffffffffffff821117156145fa576145fa614340565b6040526146078484614522565b8152614615608084016145b9565b602082015260a09290920135604083015250919050565b81810381811115610e7a57610e7a61430b565b5f6080828403121561464f575f80fd5b6110c38383614522565b67ffffffffffffffff8281168282160390808211156136c3576136c361430b565b5f6020828403121561468a575f80fd5b5051919050565b634e487b7160e01b5f52603260045260245ffd5b5f602082840312156146b5575f80fd5b81516110c381613ffe565b80820180821115610e7a57610e7a61430b565b5f602082840312156146e3575f80fd5b6110c3826145b9565b6146f581614208565b9052565b604081833760408201604082015f5b600281101561473b57813561471c8161418f565b67ffffffffffffffff1683526020928301929190910190600101614708565b50505061474a608082016145b9565b61475381614208565b608083015260a090810135910152565b5f6103208201905088358252602089013560208301526040890135604083015260608901356060830152608089013561479b81613ffe565b6001600160a01b0316608083015260a08901356147b78161418f565b67ffffffffffffffff90811660a084015260c08a0135906147d78261418f565b1660c08301526147ed60e0808401908b016146f9565b6101a06147fe818401828c016146f9565b508761026083015286610280830152856102a0830152846102c08301526148316102e08301856001600160a01b03169052565b67ffffffffffffffff831661030083015298975050505050505050565b8151805160c083019190835f5b600281101561487a57825182526020928301929091019060010161485b565b50505060209081015190604084015f5b60028110156148b157835167ffffffffffffffff168252928201929082019060010161488a565b5050505060208301516148c760808401826146ec565b50604083015160a083015292915050565b5f602082840312156148e8575f80fd5b6110c3826143d5565b634e487b7160e01b5f52603160045260245ffd5b5f82518060208501845e5f920191825250919050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f8301168401019150509291505056fea2646970667358221220b02a75c27b281ab51b1a9b90faaa010271b8993b32b302599560ad12cddd57e464736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x03\x93W_5`\xE0\x1C\x80ce\xF7\xF8\r\x11a\x01\xDFW\x80c\xBCE\xE0\xAE\x11a\x01\tW\x80c\xE6\xB3\x08,\x11a\0\xA9W\x80c\xEF@\xA6p\x11a\0yW\x80c\xEF@\xA6p\x14a\t\x82W\x80c\xF0e\xDE?\x14a\t\xAAW\x80c\xFA\xCDt;\x14a\t\xBDW\x80c\xFB\x0Er+\x14a\t\xD0W_\x80\xFD[\x80c\xE6\xB3\x08,\x14a\x08\xB1W\x80c\xE7\x8C\xEA\x92\x14a\x08\xCCW\x80c\xE8\xBDI\"\x14a\x08\xDFW\x80c\xEE5\xF3'\x14a\tzW_\x80\xFD[\x80c\xCE\x11\xE6\xAB\x11a\0\xE4W\x80c\xCE\x11\xE6\xAB\x14a\x08pW\x80c\xDF\xF6\x97\x87\x14a\x08\x83W\x80c\xE5\x10\x19\xA6\x14a\x08\x8BW\x80c\xE51\xD8\xC7\x14a\x08\x9EW_\x80\xFD[\x80c\xBCE\xE0\xAE\x14a\x08BW\x80c\xC2\xC2\xE6\x8E\x14a\x08UW\x80c\xC4\xD6m\xE8\x14a\x08]W_\x80\xFD[\x80c\x84r\x8C\xD0\x11a\x01\x7FW\x80c\x9A\x8A\x05\x92\x11a\x01OW\x80c\x9A\x8A\x05\x92\x14a\x07\x0EW\x80c\xA2<D\xB1\x14a\x07\x17W\x80c\xAA8\xA6\xE7\x14a\x08\x1AW\x80c\xB7\xABM\xB5\x14a\x08-W_\x80\xFD[\x80c\x84r\x8C\xD0\x14a\x06\xA2W\x80c\x880(\x84\x14a\x06\xDDW\x80c\x8D\xA5\xCB[\x14a\x06\xFDW\x80c\x8E\xE1\xA1&\x14a\x07\x05W_\x80\xFD[\x80cm\xDD7D\x11a\x01\xBAW\x80cm\xDD7D\x14a\x06jW\x80cq\xEF#,\x14a\x06}W\x80cs\0 \x1C\x14a\x06\x86W\x80cv\xE7\xE2;\x14a\x06\x99W_\x80\xFD[\x80ce\xF7\xF8\r\x14a\x06<W\x80ch\x12\x9B\x14\x14a\x06DW\x80ch_^\xCC\x14a\x06WW_\x80\xFD[\x80c;\x86\xDE\x19\x11a\x02\xC0W\x80cV\xBB\xC9\xE6\x11a\x02`W\x80c`\x96hm\x11a\x020W\x80c`\x96hm\x14a\x05\xD9W\x80ca79\x19\x14a\x05\xECW\x80caw\xFD\x18\x14a\x05\xF4W\x80cd \xFB\x9F\x14a\x06)W_\x80\xFD[\x80cV\xBB\xC9\xE6\x14a\x05\xA0W\x80cW\xEFJ\xB9\x14a\x05\xB3W\x80cX\x8Cz\x16\x14a\x05\xBBW\x80c\\\x97Z\xBB\x14a\x05\xCEW_\x80\xFD[\x80cP\xF3/h\x11a\x02\x9BW\x80cP\xF3/h\x14a\x05ZW\x80cQ\xEDj0\x14a\x05mW\x80cR\xD1\x90-\x14a\x05\x80W\x80cU\x84\nX\x14a\x05\x88W_\x80\xFD[\x80c;\x86\xDE\x19\x14a\x05\x12W\x80c;\xE6\x80\xEA\x14a\x05%W\x80cE\xE3\x8Bd\x14a\x05QW_\x80\xFD[\x80c\x1B\x16\x89\xE9\x11a\x036W\x80c/0\xCA\xBD\x11a\x03\x06W\x80c/0\xCA\xBD\x14a\x04\xBCW\x80c0\x83b(\x14a\x04\xE4W\x80c3c_\xC2\x14a\x04\xF7W\x80c53%\xE0\x14a\x05\nW_\x80\xFD[\x80c\x1B\x16\x89\xE9\x14a\x04aW\x80c\x1E\x83\xD3\x0F\x14a\x04jW\x80c*\xBD\xD20\x14a\x04}W\x80c.z\xCF\xA6\x14a\x04\xA8W_\x80\xFD[\x80c\x11qU\x85\x11a\x03qW\x80c\x11qU\x85\x14a\x03\xEFW\x80c\x12\xAB=;\x14a\x04\x1BW\x80c\x13\xC5l\xA7\x14a\x048W\x80c\x18\xBA\xAA\xB9\x14a\x04YW_\x80\xFD[\x80c\x02:\x96\xFE\x14a\x03\x97W\x80c\x04\x97*\xF9\x14a\x03\xC7W\x80c\x10\xB9\x8A5\x14a\x03\xDCW[_\x80\xFD[`iTa\x03\xAA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\xDAa\x03\xD56`\x04a?RV[a\t\xE3V[\0[a\x03\xDAa\x03\xEA6`\x04a?\x8DV[a\t\xFDV[a\x04\x02a\x03\xFD6`\x04a?\xE7V[a\r\xDDV[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03\xBEV[`{Ta\x04(\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03\xBEV[a\x04Ka\x04F6`\x04a?\xE7V[a\r\xF8V[`@Q\x90\x81R` \x01a\x03\xBEV[a\x03\xDAa\x0F=V[a\x04K`zT\x81V[a\x03\xDAa\x04x6`\x04a?\xE7V[a\x10\x11V[a\x04Ka\x04\x8B6`\x04a@\x12V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`w` R`@\x90 `\x01\x01T\x90V[`fTa\x04\x02\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x04Ka\x04\xCA6`\x04a@\x12V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`x` R`@\x90 T\x90V[a\x04(a\x04\xF26`\x04a?\xE7V[a\x10\x8BV[a\x04Ka\x05\x056`\x04a@-V[a\x10\xA6V[a\x04Ka\x10\xCAV[a\x03\xDAa\x05 6`\x04a@rV[a\x11\x04V[`iTa\x04\x02\x90t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x04K`qT\x81V[a\x03\xDAa\x05h6`\x04a@\x9DV[a\x14\x9BV[`pTa\x03\xAA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04Ka\x15\x14V[`{Ta\x03\xAA\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\x02a\x05\xAE6`\x04a?\xE7V[a\x15\xD8V[a\x03\xDAa\x15\xFAV[a\x03\xDAa\x05\xC96`\x04a@\x12V[a\x16hV[`3T`\xFF\x16a\x04(V[a\x03\xDAa\x05\xE76`\x04a@\xE6V[a\x17VV[a\x04Ka\x17\xC9V[a\x04(a\x06\x026`\x04a@\x12V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`w` R`@\x90 `\x02\x01T`\x01`@\x1B\x90\x04`\xFF\x16\x90V[a\x03\xDAa\x0676`\x04a@rV[a\x18IV[`tTa\x04KV[a\x03\xDAa\x06R6`\x04aA#V[a\x19\xFEV[a\x03\xDAa\x06e6`\x04aAQV[a\x1AuV[a\x03\xAAa\x06x6`\x04aA\xA4V[a\x1A\x91V[a\x04K`yT\x81V[a\x03\xDAa\x06\x946`\x04aA\xBFV[a\x1A\xC9V[a\x04K`gT\x81V[a\x03\xAAa\x06\xB06`\x04a@\x12V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`w` R`@\x90 `\x02\x01Ti\x01\0\0\0\0\0\0\0\0\0\x90\x04\x16\x90V[a\x06\xF0a\x06\xEB6`\x04a?\xE7V[a\x1A\xD5V[`@Qa\x03\xBE\x91\x90aB$V[a\x03\xAAa\x1B\xA0V[a\x04K`hT\x81V[a\x04K`eT\x81V[a\x07\xC4a\x07%6`\x04a@\x12V[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91RP`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`w` \x90\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R`\x02\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x92\x82\x01\x92\x90\x92R`\xFF`\x01`@\x1B\x83\x04\x16\x15\x15``\x82\x01Ri\x01\0\0\0\0\0\0\0\0\0\x90\x91\x04\x90\x91\x16`\x80\x82\x01R\x90V[`@Qa\x03\xBE\x91\x90_`\xA0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x84\x01Q\x16`@\x83\x01R``\x83\x01Q\x15\x15``\x83\x01R`\x01`\x01`\xA0\x1B\x03`\x80\x84\x01Q\x16`\x80\x83\x01R\x92\x91PPV[`mTa\x03\xAA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x085a\x1B\xA9V[`@Qa\x03\xBE\x91\x90aB\x84V[`nTa\x03\xAA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\xDAa\x1B\xB5V[a\x03\xDAa\x08k6`\x04a@\x12V[a\x1CwV[`lTa\x03\xAA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`vTa\x04\x02V[a\x03\xDAa\x08\x996`\x04aB\xD0V[a\x1E\x19V[a\x04(a\x08\xAC6`\x04a?\xE7V[a\x1E\x80V[`fTa\x04\x02\x90`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`kTa\x03\xAA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\t:a\x08\xED6`\x04a@\x12V[`w` R_\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x90\x92\x01T\x90\x91\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90`\x01`@\x1B\x81\x04`\xFF\x16\x90i\x01\0\0\0\0\0\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x85V[`@\x80Q\x95\x86R` \x86\x01\x94\x90\x94Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x92\x84\x01\x92\x90\x92R\x90\x15\x15``\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16`\x80\x82\x01R`\xA0\x01a\x03\xBEV[a\x03\xAAa\x1E\xAFV[a\x04Ka\t\x906`\x04a@\x12V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`w` R`@\x90 T\x90V[`oTa\x03\xAA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04(a\t\xCB6`\x04a@\x12V[a\x1F3V[`jTa\x03\xAA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\t\xF9\x81a\t\xF0\x84a\x1FRV[`\x01\x01Ta\x1F\xB1V[PPV[3_\x81\x81R`s` R`@\x90 T\x15\x15\x80a\n\x1BWP`{T`\xFF\x16[a\n\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl''\xAA/\xAB \xA6$\xA2 \xAA'\xA9`\x99\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\nda \xB6V[_a\nn\x88a\x1FRV[\x90P_a\nz\x88a\x1FRV[\x90Pa\n\x8A\x85\x82`\x01\x01Ta\x1F\xB1V[a\n\x9A`\x80\x86\x01``\x87\x01aA\xA4V[\x82Ta\n\xB7\x91\x90`\x01`\x80\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aC\x1FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x10\x15a\x0B\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FBEFORE_DEADLINE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`tT\x88\x14a\x0BaW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FPREV_NOT_LATEST_CONFIRMED\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[\x80T`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\r\xC6W_a\x0B\x8A``\x87\x01`@\x88\x01a@\x12V[`\x01`\x01`\xA0\x1B\x03\x16c\xFD\xA2\x89.\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xB7\x91\x81R` \x01\x90V[a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xD3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xF7\x91\x90aC\xE4V[\x90P\x89\x81`\xE0\x01Q\x14a\x0CLW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01R\x7FNOT_WINNER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`\x01\x81a\x01`\x01Q`\x01\x81\x11\x15a\x0CeWa\x0CeaA\xF4V[\x14a\x0C\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FEDGE_NOT_CONFIRMED\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[\x80a\x01@\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a\r\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FZERO_CONFIRMED_AT_BLOCK\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`iTa\x01@\x82\x01Qa\rE\x91t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90aC\x1FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16C\x10\x15a\r\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FCHALLENGE_GRACE_PERIOD_NOT_PASSE`D\x82\x01R\x7FD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nSV[P[a\r\xD2\x89\x89\x89\x87a!\x0BV[PPPPPPPPPV[_a\r\xE7\x82a\x1FRV[Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\x0E\x80W_\x82\x81R`|` R`@\x90 T\x80a\x0EzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FNO_ASSERTION\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[\x92\x91PPV[_a\x0E\x8A\x83a\x1FRV[`@\x80Q`\xC0\x81\x01\x82R\x82Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x82\x04\x16\x92\x82\x01\x92\x90\x92R`\xFF`\x01`\xC0\x1B\x83\x04\x81\x16\x15\x15``\x83\x01R\x92\x93Pa\x0F \x92\x90\x91\x84\x91`\x80\x84\x01\x91`\x01`\xC8\x1B\x90\x04\x16`\x02\x81\x11\x15a\x0E\xFCWa\x0E\xFCaA\xF4V[`\x02\x81\x11\x15a\x0F\rWa\x0F\raA\xF4V[\x81R` \x01`\x01\x82\x01T\x81RPPa#\x1AV[T`\x01`\x80\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[\x91\x90PV[`{T`\xFF\x16\x15a\x0F\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FWHITELIST_DISABLED\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[a\x0F\x98a#~V[a\x0F\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FVALIDATOR_NOT_AFK\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`{\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UV[3_\x81\x81R`s` R`@\x90 T\x15\x15\x80a\x10/WP`{T`\xFF\x16[a\x10kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl''\xAA/\xAB \xA6$\xA2 \xAA'\xA9`\x99\x1B`D\x82\x01R`d\x01a\nSV[a\x10sa \xB6V[a\x10|3a$\xADV[a\x10\x863\x83a%\x9CV[PPPV[_a\x10\x95\x82a\x1FRV[T`\x01`\xC0\x1B\x90\x04`\xFF\x16\x92\x91PPV[_a\x10\xC0\x84a\x10\xBA6\x86\x90\x03\x86\x01\x86aE\xC7V[\x84a&\x9BV[\x90P[\x93\x92PPPV[_a\x10\xD3a>\xF9V[`@\x80Q``\x81\x01\x82R\x82\x81R`\x01` \x82\x01R_\x91\x81\x01\x82\x90R\x90\x80a\x10\xFB\x81\x84\x81a&\x9BV[\x94PPPPP\x90V[3_\x81\x81R`s` R`@\x90 T\x15\x15\x80a\x11\"WP`{T`\xFF\x16[a\x11^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl''\xAA/\xAB \xA6$\xA2 \xAA'\xA9`\x99\x1B`D\x82\x01R`d\x01a\nSV[a\x11fa \xB6V[\x81\x15\x80a\x11\x96WP_a\x11x\x83a\x1FRV[T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x11\x94Wa\x11\x94aA\xF4V[\x14[a\x11\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FEXPECTED_ASSERTION_SEEN\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[3_\x90\x81R`w` R`@\x90 `\x02\x01T`\x01`@\x1B\x90\x04`\xFF\x16a\x127W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri\x13\x93\xD5\x17\xD4\xD5\x10R\xD1Q`\xB2\x1B`D\x82\x01R`d\x01a\nSV[3_\x90\x81R`w` R`@\x90 T``\x84\x015\x11\x15a\x12\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FINSUFFICIENT_STAKE\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_a\x12\xBB\x845a\x12\xB16\x87\x90\x03\x87\x01`\xE0\x88\x01aE\xC7V[` \x87\x015a&\x9BV[\x90Pa\x134a\x12\xC9\x82a\x1FRV[`@\x80Q`\xC0\x81\x01\x82R\x82Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x82\x04\x16\x92\x82\x01\x92\x90\x92R`\xFF`\x01`\xC0\x1B\x83\x04\x81\x16\x15\x15``\x83\x01R\x90\x92\x91`\x80\x84\x01\x91`\x01`\xC8\x1B\x90\x91\x04\x16`\x02\x81\x11\x15a\x0E\xFCWa\x0E\xFCaA\xF4V[3_\x90\x81R`w` R`@\x90 `\x01\x01T\x81\x81\x14\x80a\x13fWP_a\x13Y\x82a\x1FRV[Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11[a\x13\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FSTAKED_ON_ANOTHER_BRANCH\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_\x80a\x13\xBF\x87\x85\x88a&\xDEV[3_\x90\x81R`w` R`@\x90 `\x01\x01\x82\x90U\x90\x92P\x90P\x80a\x14YW_a\x13\xE7\x85a\x1FRV[Ta\x14\x03\x90`\x01`\x80\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16CaF,V[\x90P`qT\x81\x10\x15a\x14WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01R\x7FTIME_DELTA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[P[a\x14b\x82a\x1FRV[T`\x01`\xC0\x1B\x90\x04`\xFF\x16a\x14\x92W`oT`pTa\x14\x92\x91`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16``\x8A\x015a0KV[PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x14\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FEMPTY_WITHDRAWAL_ADDRESS\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[a\x14\xFB\x84\x82a0\xF4V[a\x15\x05\x83\x83a\x11\x04V[a\x15\x0E\x84a1\xCAV[PPPPV[_0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x15\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FUUPSNotUpgradeable: must not be `D\x82\x01R\x7Fcalled through delegatecall\0\0\0\0\0`d\x82\x01R`\x84\x01a\nSV[P\x7F+\x1D\xBC\xE7C$$\x8C\"/\x0E\xC2\xD5\xED{\xD3#\xCF\xC4%\xB36\xF0%<\\\xCF\xDAreTm\x90V[_a\x15\xE2\x82a\x1FRV[T`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[3_\x81\x81R`s` R`@\x90 T\x15\x15\x80a\x16\x18WP`{T`\xFF\x16[a\x16TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl''\xAA/\xAB \xA6$\xA2 \xAA'\xA9`\x99\x1B`D\x82\x01R`d\x01a\nSV[a\x16\\a \xB6V[a\x16e3a1\xE2V[PV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`s` R`@\x90 T\x81\x90\x15\x15\x80a\x16\x91WP`{T`\xFF\x16[a\x16\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl''\xAA/\xAB \xA6$\xA2 \xAA'\xA9`\x99\x1B`D\x82\x01R`d\x01a\nSV[a\x16\xD5a \xB6V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x90\x81R`w` R`@\x90 `\x02\x01Ti\x01\0\0\0\0\0\0\0\0\0\x90\x04\x163\x14a\x17MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FNOT_WITHDRAWAL_ADDRESS\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[a\t\xF9\x82a1\xE2V[a\x17^a \xB6V[`{Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FNOT_FAST_CONFIRMER\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[a\x15\x0E\x84\x84\x84\x84a!\x0BV[_a\x17\xD2a \xB6V[_a\x17\xDC3a1\xF4V[\x90P_\x81\x11a\x18-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FNO_FUNDS_TO_WITHDRAW\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`pTa\x18D\x90`\x01`\x01`\xA0\x1B\x03\x163\x83a0KV[\x90P\x90V[a\x18Qa \xB6V[\x80a\x18\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FEXPECTED_ASSERTION_HASH\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_a\x18\xA8\x82a\x1FRV[T`\x01`\xC8\x1B\x90\x04`\xFF\x16\x90P_a\x18\xCD\x845a\x12\xB16\x87\x90\x03\x87\x01`\xE0\x88\x01aE\xC7V[\x90Pa\x18\xDBa\x12\xC9\x82a\x1FRV[_\x82`\x02\x81\x11\x15a\x18\xEEWa\x18\xEEaA\xF4V[\x03a\x19=W_a\x18\xFF\x85\x83\x86a&\xDEV[P\x90Pa\x19\x0B\x81a\x1FRV[T`\x01`\xC0\x1B\x90\x04`\xFF\x16a\x19;W`oT`pTa\x19;\x91`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16``\x88\x015a0KV[P[`kTa\x15\x0E\x90\x84\x90\x83\x90a\x01\xA0\x88\x01\x90`\x01`\x01`\xA0\x1B\x03\x16c\x16\xBFUy`\x01a\x19ua\x19p6\x8D\x90\x03\x8D\x01\x86aF?V[a2sV[a\x19\x7F\x91\x90aFYV[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xDAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xE7\x91\x90aFzV[a\x1A\x06a \xB6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1A\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FEMPTY_WITHDRAWAL_ADDRESS\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[a\x1Af\x82\x82a0\xF4V[\x81\x15a\t\xF9Wa\t\xF9\x82a1\xCAV[a\x1A}a \xB6V[a\x1A\x88\x83\x83\x83a2\x87V[a\x10\x86\x81a1\xCAV[_`v\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1A\xAFWa\x1A\xAFaF\x91V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[a\x10\x86\x83\x83\x833a\x14\x9BV[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x1B\x0F\x82a\x1FRV[`@\x80Q`\xC0\x81\x01\x82R\x82Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x82\x04\x16\x92\x82\x01\x92\x90\x92R`\xFF`\x01`\xC0\x1B\x83\x04\x81\x16\x15\x15``\x83\x01R\x90\x92\x91`\x80\x84\x01\x91`\x01`\xC8\x1B\x90\x91\x04\x16`\x02\x81\x11\x15a\x1BzWa\x1BzaA\xF4V[`\x02\x81\x11\x15a\x1B\x8BWa\x1B\x8BaA\xF4V[\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x91\x90PV[_a\x18Da3\xEDV[``a\x18D`ra4\x1FV[`{T`\xFF\x16\x15a\x1C\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FWHITELIST_DISABLED\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03a\x0F\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FCHAIN_ID_NOT_CHANGED\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x1D\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Fdelegatecall\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nSV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x1DGa4+V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1D\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FFunction must be called through `D\x82\x01R\x7Factive proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nSV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x16eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FNEED_STAKE_TOKEN\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[a\x1E2\x82a\x1E,6\x86\x90\x03\x86\x01\x86aE\xC7V[\x83a&\x9BV[\x84\x14a\x15\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7FINVALID_ASSERTION_HASH\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_`\x01a\x1E\x8C\x83a\x1FRV[T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1E\xA8Wa\x1E\xA8aA\xF4V[\x14\x92\x91PPV[`kT`@\x80Q\x7F\xEE5\xF3'\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xEE5\xF3'\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1F\x0FW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18D\x91\x90aF\xA5V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`s` R`@\x81 T\x15\x15a\x0EzV[_\x81a\x1F\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FASSERTION_ID_CANNOT_BE_ZERO\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[P_\x90\x81R`u` R`@\x90 \x90V[a h\x825` \x84\x015a\x1F\xCB``\x86\x01`@\x87\x01a@\x12V[a\x1F\xDB`\x80\x87\x01``\x88\x01aA\xA4V[a\x1F\xEB`\xA0\x88\x01`\x80\x89\x01aA\xA4V[`@\x80Q` \x80\x82\x01\x97\x90\x97R\x80\x82\x01\x95\x90\x95Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x94\x85\x1B\x16\x93\x85\x01\x93\x90\x93R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0\x92\x83\x1B\x81\x16`t\x86\x01R\x91\x1B\x16`|\x83\x01R\x80Q`d\x81\x84\x03\x01\x81R`\x84\x90\x92\x01\x90R\x80Q\x91\x01 \x90V[\x81\x14a\t\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FCONFIG_HASH_MISMATCH\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`3T`\xFF\x16\x15a!\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FPausable: paused\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[V[_a!\x15\x85a\x1FRV[\x90P`\x01\x81T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a!6Wa!6aA\xF4V[\x14a!\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7FNOT_PENDING\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[a!\x96\x84a\x10\xBA6\x86\x90\x03\x86\x01\x86aE\xC7V[\x85\x14a!\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FCONFIRM_DATA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_a!\xFCa!\xF76\x86\x90\x03\x86\x01\x86aF?V[a4RV[\x90P_a\"\x16a\"\x116\x87\x90\x03\x87\x01\x87aF?V[a4\\V[`lT`@Q\x7F\xA0L\xEE`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x85\x90R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA0L\xEE`\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\"zW_\x80\xFD[PZ\xF1\x15\x80\x15a\"\x8CW=_\x80>=_\xFD[PPP`t\x88\x90UP\x82T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16y\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x83U`@\x80Q\x83\x81R` \x81\x01\x83\x90R\x81Q\x89\x92\x7F\xFCB\x82\x9B)\xC2Y\xA77\n\xB5l\x8Fi\xFC\xE2;_5\x1A\x9C\xE1Q\xDAE2\x81\x99>\xC0\t\x0C\x92\x82\x90\x03\x01\x90\xA2PPPPPPPV[_\x81`\x80\x01Q`\x02\x81\x11\x15a#1Wa#1aA\xF4V[\x03a\x16eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FASSERTION_NOT_EXIST\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_\x80a#\x91a#\x8C`tT\x90V[a\x1FRV[`@\x80Q`\xC0\x81\x01\x82R\x82Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x82\x04\x16\x92\x82\x01\x92\x90\x92R`\xFF`\x01`\xC0\x1B\x83\x04\x81\x16\x15\x15``\x83\x01R\x90\x92\x91`\x80\x84\x01\x91`\x01`\xC8\x1B\x90\x91\x04\x16`\x02\x81\x11\x15a#\xFCWa#\xFCaA\xF4V[`\x02\x81\x11\x15a$\rWa$\raA\xF4V[\x81R`\x01\x91\x90\x91\x01T` \x90\x91\x01R`fT\x90\x91P`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x81\x90\x03a$CW_\x92PPP\x90V[\x81`@\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a$`W_\x92PPP\x90V[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a$\x92W\x81QC\x90a$\x8A\x90\x83\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aF\xC0V[\x10\x92PPP\x90V[C\x81\x83`@\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a$\x8A\x91\x90aF\xC0V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`w` R`@\x90 `\x02\x01T`\x01`@\x1B\x90\x04`\xFF\x16a%\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri\x13\x93\xD5\x17\xD4\xD5\x10R\xD1Q`\xB2\x1B`D\x82\x01R`d\x01a\nSV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`w` R`@\x81 `\x01\x01T`tT\x90\x91\x90\x82\x14\x90\x80a%9\x84a\x1FRV[Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x90P\x81\x80a%PWP\x80[a\x15\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FSTAKE_ACTIVE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x90\x81R`w` R`@\x81 `\x02\x81\x01T\x81T\x92\x93\x91\x92i\x01\0\0\0\0\0\0\0\0\0\x90\x91\x04\x90\x91\x16\x90\x80\x85\x11\x15a&!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FTOO_LITTLE_STAKE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_a&,\x86\x83aF,V[\x86\x85U\x90Pa&;\x83\x82a4gV[\x82`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x7F\xD9W\xCF#@\x0735\xD2V\xF7*\x9E\xF8\x9C\xF1\xA4<1\x143A\xA6\xA55u\xEF3\xE9\x87\xBE\xB8\x84\x89`@Qa&\x89\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA3\x96\x95PPPPPPV[_a\x10\xC0\x84a&\xA9\x85a5\nV[`@\x80Q` \x80\x82\x01\x94\x90\x94R\x80\x82\x01\x92\x90\x92R``\x80\x83\x01\x87\x90R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\x80\x90\x92\x01\x90R\x80Q\x91\x01 \x90V[_\x80a&\xF0`@\x86\x01a\t\xF0\x86a\x1FRV[`\x01a'\x04a\x02@\x87\x01a\x02 \x88\x01aF\xD3V[`\x02\x81\x11\x15a'\x15Wa'\x15aA\xF4V[\x14\x80a'CWP`\x02a'0a\x02@\x87\x01a\x02 \x88\x01aF\xD3V[`\x02\x81\x11\x15a'AWa'AaA\xF4V[\x14[a'\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7FBAD_AFTER_STATUS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[\x83a'\xB1\x865a'\xA76\x89\x90\x03\x89\x01`\xE0\x8A\x01aE\xC7V[` \x89\x015a&\x9BV[\x14a'\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FINVALID_BEFORE_STATE\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`\x01a(\x12a\x01\x80\x87\x01a\x01`\x88\x01aF\xD3V[`\x02\x81\x11\x15a(#Wa(#aA\xF4V[\x14a(pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FBAD_PREV_STATUS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_a(z\x85a\x1FRV[\x90P_\x80a\x01\xA0\x88\x01`\xE0\x89\x01\x82a(\x92\x83\x83a59V[\x12\x15a(\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FINBOX_BACKWARDS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_a)\x05a(\xF4`\xE0\x8D\x01`\xC0\x8E\x01aA\xA4V[\x84\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a6?V[\x90P_\x81\x13\x15a)WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7FINBOX_TOO_FAR\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`\x02a)ka\x02@\x8D\x01a\x02 \x8E\x01aF\xD3V[`\x02\x81\x11\x15a)|Wa)|aA\xF4V[\x14\x15\x80\x15a)\x89WP_\x81\x12[\x15a)\xEAW`\x01\x96P_a)\x9D\x84\x84a59V[\x13a)\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FOVERFLOW_STANDSTILL\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`kT`@\x80Q~\x84\x12\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91b\x84\x12\x0C\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a*HW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*l\x91\x90aFzV[\x90P_a*y\x85\x83a6?V[\x13\x15a*\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FINBOX_PAST_END\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[\x80a*\xD8`\xE0\x8E\x01`\xC0\x8F\x01aA\xA4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a+0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FINBOX_NOT_POPULATED\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_a+Ca\x19p6\x87\x90\x03\x87\x01\x87aF?V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81\x81\x03a+hWa+a\x82`\x01aF\xC0V[\x96Pa+lV[\x81\x96P[\x80_\x03a+\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FEMPTY_INBOX_COUNT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[`kT`\x01`\x01`\xA0\x1B\x03\x16c\x16\xBFUya+\xD7`\x01\x84aF,V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\xF5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\x10W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,4\x91\x90aFzV[\x95PPPPPPa,T\x87\x89a\x01\xA0\x01\x806\x03\x81\x01\x90a\x1E,\x91\x90aE\xC7V[\x94P\x85\x85\x14\x80a,bWP\x85\x15[a,\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FUNEXPECTED_ASSERTION_HASH\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[_a,\xB8\x86a\x1FRV[T`\x01`\xC8\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a,\xD4Wa,\xD4aA\xF4V[\x14a-!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FASSERTION_SEEN\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[\x82T`hT`gT`iT`fT`@\x80Q` \x80\x82\x01\x96\x90\x96R\x80\x82\x01\x94\x90\x94Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x93\x84\x1B\x16\x83\x85\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0\x92\x83\x1B\x81\x16`t\x86\x01R\x88\x83\x1B\x16`|\x85\x01R\x80Q`d\x81\x86\x03\x01\x81R`\x84\x85\x01\x80\x83R\x81Q\x91\x87\x01\x91\x90\x91 a\x01D\x86\x01\x83R_\x91\x82\x90R`\xA4\x86\x01\x82\x90R`\xC4\x86\x01\x82\x90R`\xE4\x86\x01\x82\x90Ra\x01\x04\x86\x01\x82\x90Ra\x01$\x90\x95\x01\x81\x90R\x81Q\x92\x83\x01\x82R\x80\x83R\x94\x82\x01\x85\x90Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x81\x16\x91\x83\x01\x91\x90\x91R\x90\x94\x16\x15\x90\x84\x01R`\xA0\x83\x01R`\x01`\x80\x83\x01RPa.\x1E\x84a6\xCAV[_\x86\x81R`u` \x90\x81R`@\x91\x82\x90 \x83Q\x81T\x92\x85\x01Q\x93\x85\x01Q``\x86\x01Q\x15\x15`\x01`\xC0\x1B\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\x01`\x80\x1B\x02\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96\x83\x16`\x01`@\x1B\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x96\x16\x92\x90\x93\x16\x91\x90\x91\x17\x93\x90\x93\x17\x93\x90\x93\x16\x92\x90\x92\x17\x17\x80\x82U`\x80\x83\x01Q\x83\x92\x91\x82\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`\xC8\x1B\x83`\x02\x81\x11\x15a/%Wa/%aA\xF4V[\x02\x17\x90UP`\xA0\x82\x01Q\x81`\x01\x01U\x90PP\x87\x86\x7F\x90\x1C:\xEE#\xCFDx\x82Tb\xCA\xAA\xB3u\xC6\x06\xAB\x83Q``8\x83D\xF0e\x03@u60\x8B\x85\x87`hT`gT`i_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`f_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Qa/\xA1\x97\x96\x95\x94\x93\x92\x91\x90aGcV[`@Q\x80\x91\x03\x90\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a0?W`d`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xB1\xB3\x1D`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0\x0CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a00\x91\x90aFzV[_\x87\x81R`|` R`@\x90 U[PPPP\x93P\x93\x91PPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x10\x86\x90\x84\x90\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`d\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra7LV[3_\x81\x81R`s` R`@\x90 T\x15\x15\x80a1\x12WP`{T`\xFF\x16[a1NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl''\xAA/\xAB \xA6$\xA2 \xAA'\xA9`\x99\x1B`D\x82\x01R`d\x01a\nSV[a1Va \xB6V[3_\x90\x81R`w` R`@\x90 `\x02\x01T`\x01`@\x1B\x90\x04`\xFF\x16\x15a1\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FALREADY_STAKED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[a\x10\x863\x84\x84a80V[`pTa\x16e\x90`\x01`\x01`\xA0\x1B\x03\x1630\x84a9\xABV[a1\xEB\x81a$\xADV[a\x16e\x81a9\xFCV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`x` R`@\x81 \x80T\x90\x82\x90U`y\x80T\x82\x91\x90\x84\x90a2%\x90\x84\x90aF,V[\x90\x91UPP`@\x80Q\x82\x81R_` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x7F\xA7@\xAF\x14\xC5nN\x04\xA6\x17\xB1\xDE\x1E\xB2\r\xE72p\xDE\xCB\xAA\xEA\xD1O\x14*\xAB\xF3\x03\x8EZ\xE2\x91\x01`@Q\x80\x91\x03\x90\xA2\x92\x91PPV[` \x81\x01Q_\x90\x81[` \x02\x01Q\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`s` R`@\x90 T\x83\x90\x15\x15\x80a2\xB0WP`{T`\xFF\x16[a2\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl''\xAA/\xAB \xA6$\xA2 \xAA'\xA9`\x99\x1B`D\x82\x01R`d\x01a\nSV[a2\xF4a \xB6V[`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`w` R`@\x90 `\x02\x01T`\x01`@\x1B\x90\x04`\xFF\x16a3RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri\x13\x93\xD5\x17\xD4\xD5\x10R\xD1Q`\xB2\x1B`D\x82\x01R`d\x01a\nSV[\x82`\x01`\x01`\xA0\x1B\x03\x16a3\x8D\x85`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`w` R`@\x90 `\x02\x01Ti\x01\0\0\0\0\0\0\0\0\0\x90\x04\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a3\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FWRONG_WITHDRAWAL_ADDRESS\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nSV[a\x15\x0E\x84\x83a:\x85V[_\x7F\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03[T`\x01`\x01`\xA0\x1B\x03\x16\x91\x90PV[``_a\x10\xC3\x83a;\x12V[_\x7F+\x1D\xBC\xE7C$$\x8C\"/\x0E\xC2\xD5\xED{\xD3#\xCF\xC4%\xB36\xF0%<\\\xCF\xDAreTma4\x10V[\x80Q_\x90\x81a2|V[\x80Q_\x90`\x01a2|V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`x` R`@\x81 T\x90a4\x8A\x83\x83aF\xC0V[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`x` R`@\x81 \x82\x90U`y\x80T\x92\x93P\x85\x92\x90\x91\x90a4\xBB\x90\x84\x90aF\xC0V[\x90\x91UPP`@\x80Q\x83\x81R` \x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x86\x16\x91\x7F\xA7@\xAF\x14\xC5nN\x04\xA6\x17\xB1\xDE\x1E\xB2\r\xE72p\xDE\xCB\xAA\xEA\xD1O\x14*\xAB\xF3\x03\x8EZ\xE2\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPV[_\x81`@Q` \x01a5\x1C\x91\x90aHNV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[_\x80a5Ma\x19p6\x86\x90\x03\x86\x01\x86aF?V[\x90P_a5ba\x19p6\x86\x90\x03\x86\x01\x86aF?V[\x90P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a5\x8BW_\x19\x92PPPa\x0EzV[\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a5\xB2W`\x01\x92PPPa\x0EzV[_a5\xCAa5\xC56\x88\x90\x03\x88\x01\x88aF?V[a;kV[\x90P_a5\xDFa5\xC56\x88\x90\x03\x88\x01\x88aF?V[\x90P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a6\nW_\x19\x94PPPPPa\x0EzV[\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a63W`\x01\x94PPPPPa\x0EzV[_\x94PPPPPa\x0EzV[_\x80a6Sa\x19p6\x86\x90\x03\x86\x01\x86aF?V[\x90P\x82\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a6qW_\x19\x91PPa\x0EzV[\x82\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a6\x8DW`\x01\x91PPa\x0EzV[_a6\xA0a5\xC56\x87\x90\x03\x87\x01\x87aF?V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a6\xBAW`\x01\x91PPa\x0EzV[_\x91PPa\x0EzV[P\x92\x91PPV[\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a6\xF9W\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x17\x90UV[\x80T`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a\x16eW\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`@\x1BCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UV[_a7\xA0\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a;y\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x10\x86W\x80\x80` \x01\x90Q\x81\x01\x90a7\xBE\x91\x90aH\xD8V[a\x10\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01R\x7Fot succeed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nSV[`v\x80T`\x01\x80\x82\x01\x90\x92U\x7F\xB5s'\x05\xF5$\x13p\xA2\x89\x08\xC2\xFE\x13\x03\xCB\"?\x03\xB9\r\x85\x7F\xD0W?\0?y\xFE\xFE\xD4\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x91\x82\x17\x90\x92U`@\x80Q`\xA0\x81\x01\x82R\x87\x81R`tT` \x80\x83\x01\x91\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x84\x86\x01\x90\x81R``\x85\x01\x8A\x81R\x8B\x89\x16`\x80\x87\x01\x81\x81R_\x8A\x81R`w\x87R\x89\x81 \x98Q\x89U\x96Q\x9C\x88\x01\x9C\x90\x9CU\x91Q`\x02\x90\x96\x01\x80T\x91Q\x9BQ\x96\x90\x93\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x90\x91\x16\x17`\x01`@\x1B\x9A\x15\x15\x9A\x90\x9A\x02\x99\x90\x99\x17\x7F\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16i\x01\0\0\0\0\0\0\0\0\0\x94\x90\x97\x16\x93\x90\x93\x02\x95\x90\x95\x17\x90\x91U\x81Q\x90\x81R\x92\x83\x01\x87\x90R\x92\x93\x92\x90\x91\x7F\xD9W\xCF#@\x0735\xD2V\xF7*\x9E\xF8\x9C\xF1\xA4<1\x143A\xA6\xA55u\xEF3\xE9\x87\xBE\xB8\x91\x01[`@Q\x80\x91\x03\x90\xA3PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x15\x0E\x90\x85\x90\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x84\x01a0\x90V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`w` R`@\x90 `\x02\x81\x01T\x81T\x91\x92i\x01\0\0\0\0\0\0\0\0\0\x90\x91\x04\x16\x90a:7\x82\x82a4gV[a:@\x84a;\x87V[`@\x80Q\x82\x81R_` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x92\x90\x87\x16\x91\x7F\xD9W\xCF#@\x0735\xD2V\xF7*\x9E\xF8\x9C\xF1\xA4<1\x143A\xA6\xA55u\xEF3\xE9\x87\xBE\xB8\x91\x01a9\x9DV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`w` R`@\x81 \x80T\x90\x91a:\xAA\x84\x83aF\xC0V[\x80\x84U`\x02\x84\x01T`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x92\x93P`\x01`\x01`\xA0\x1B\x03i\x01\0\0\0\0\0\0\0\0\0\x90\x92\x04\x82\x16\x92\x91\x88\x16\x91\x7F\xD9W\xCF#@\x0735\xD2V\xF7*\x9E\xF8\x9C\xF1\xA4<1\x143A\xA6\xA55u\xEF3\xE9\x87\xBE\xB8\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a;_W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a;KW[PPPPP\x90P\x91\x90PV[` \x81\x01Q_\x90`\x01a2|V[``a\x10\xC0\x84\x84_\x85a=|V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`w` R`@\x90 `\x02\x81\x01T`\x01`@\x1B\x90\x04`\xFF\x16a;\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri\x13\x93\xD5\x17\xD4\xD5\x10R\xD1Q`\xB2\x1B`D\x82\x01R`d\x01a\nSV[`\x02\x81\x01T`v\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91a<\x08\x90`\x01\x90aF,V[\x81T\x81\x10a<\x18Wa<\x18aF\x91V[_\x91\x82R` \x90\x91 \x01T`v\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x10a<MWa<MaF\x91V[\x90_R` _ \x01_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`w_`v\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a<\x99Wa<\x99aF\x91V[_\x91\x82R` \x80\x83 \x91\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83R\x82\x01\x92\x90\x92R`@\x01\x90 `\x02\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U`v\x80T\x80a<\xF1Wa<\xF1aH\xF1V[_\x82\x81R` \x80\x82 \x83\x01_\x19\x90\x81\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U\x90\x92\x01\x90\x92U`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x81R`w\x90\x93RPP`@\x81 \x81\x81U`\x01\x81\x01\x91\x90\x91U`\x02\x01\x80T\x7F\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90UV[``\x82G\x10\x15a=\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01R\x7Fr call\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nSV[`\x01`\x01`\xA0\x1B\x03\x85\x16;a>KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\nSV[_\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa>f\x91\x90aI\x05V[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a>\xA0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a>\xA5V[``\x91P[P\x91P\x91Pa>\xB5\x82\x82\x86a>\xC0V[\x97\x96PPPPPPPV[``\x83\x15a>\xCFWP\x81a\x10\xC3V[\x82Q\x15a>\xDFW\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nS\x91\x90aI\x1BV[`@Q\x80`@\x01`@R\x80a?\x0Ca?\x1EV[\x81R` \x01a?\x19a?\x1EV[\x90R\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[_`\xA0\x82\x84\x03\x12\x15a?LW_\x80\xFD[P\x91\x90PV[_\x80`\xC0\x83\x85\x03\x12\x15a?cW_\x80\xFD[\x825\x91Pa?t\x84` \x85\x01a?<V[\x90P\x92P\x92\x90PV[_`\xC0\x82\x84\x03\x12\x15a?LW_\x80\xFD[_\x80_\x80_\x80a\x01\xE0\x87\x89\x03\x12\x15a?\xA3W_\x80\xFD[\x865\x95P` \x87\x015\x94Pa?\xBB\x88`@\x89\x01a?}V[\x93Pa\x01\0\x87\x015\x92Pa?\xD3\x88a\x01 \x89\x01a?<V[\x91Pa\x01\xC0\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[_` \x82\x84\x03\x12\x15a?\xF7W_\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x16eW_\x80\xFD[_` \x82\x84\x03\x12\x15a@\"W_\x80\xFD[\x815a\x10\xC3\x81a?\xFEV[_\x80_a\x01\0\x84\x86\x03\x12\x15a@@W_\x80\xFD[\x835\x92Pa@Q\x85` \x86\x01a?}V[\x91P`\xE0\x84\x015\x90P\x92P\x92P\x92V[_a\x02`\x82\x84\x03\x12\x15a?LW_\x80\xFD[_\x80a\x02\x80\x83\x85\x03\x12\x15a@\x84W_\x80\xFD[a@\x8E\x84\x84a@aV[\x94a\x02`\x93\x90\x93\x015\x93PPPV[_\x80_\x80a\x02\xC0\x85\x87\x03\x12\x15a@\xB1W_\x80\xFD[\x845\x93Pa@\xC2\x86` \x87\x01a@aV[\x92Pa\x02\x80\x85\x015\x91Pa\x02\xA0\x85\x015a@\xDB\x81a?\xFEV[\x93\x96\x92\x95P\x90\x93PPV[_\x80_\x80a\x01 \x85\x87\x03\x12\x15a@\xFAW_\x80\xFD[\x845\x93P` \x85\x015\x92PaA\x12\x86`@\x87\x01a?}V[\x93\x96\x92\x95P\x92\x93a\x01\0\x015\x92PPV[_\x80`@\x83\x85\x03\x12\x15aA4W_\x80\xFD[\x825\x91P` \x83\x015aAF\x81a?\xFEV[\x80\x91PP\x92P\x92\x90PV[_\x80_``\x84\x86\x03\x12\x15aAcW_\x80\xFD[\x835aAn\x81a?\xFEV[\x92P` \x84\x015aA~\x81a?\xFEV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x16eW_\x80\xFD[_` \x82\x84\x03\x12\x15aA\xB4W_\x80\xFD[\x815a\x10\xC3\x81aA\x8FV[_\x80_a\x02\xA0\x84\x86\x03\x12\x15aA\xD2W_\x80\xFD[\x835\x92PaA\xE3\x85` \x86\x01a@aV[\x91Pa\x02\x80\x84\x015\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x81\x10a\x16eWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_`\xC0\x82\x01\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01R\x80`@\x85\x01Q\x16`@\x84\x01RP``\x83\x01Q\x15\x15``\x83\x01R`\x80\x83\x01QaBm\x81aB\x08V[\x80`\x80\x84\x01RP`\xA0\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aB\xC4W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aB\x9FV[P\x90\x96\x95PPPPPPV[_\x80_\x80a\x01 \x85\x87\x03\x12\x15aB\xE4W_\x80\xFD[\x845\x93PaB\xF5\x86` \x87\x01a?}V[\x93\x96\x93\x95PPPP`\xE0\x82\x015\x91a\x01\0\x015\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a6\xC3Wa6\xC3aC\x0BV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Qa\x01\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aCxWaCxaC@V[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aCxWaCxaC@V[\x80Qa\x0F8\x81a?\xFEV[\x80Qa\x0F8\x81aA\x8FV[\x80Q`\x02\x81\x10a\x0F8W_\x80\xFD[\x80Q`\xFF\x81\x16\x81\x14a\x0F8W_\x80\xFD[\x80Q\x80\x15\x15\x81\x14a\x0F8W_\x80\xFD[_a\x01\xE0\x82\x84\x03\x12\x15aC\xF5W_\x80\xFD[aC\xFDaCTV[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R`\x80\x83\x01Q`\x80\x82\x01R`\xA0\x83\x01Q`\xA0\x82\x01R`\xC0\x83\x01Q`\xC0\x82\x01R`\xE0\x83\x01Q`\xE0\x82\x01Ra\x01\0aDU\x81\x85\x01aC\xA1V[\x90\x82\x01Ra\x01 aDg\x84\x82\x01aC\xACV[\x90\x82\x01Ra\x01@aDy\x84\x82\x01aC\xACV[\x90\x82\x01Ra\x01`aD\x8B\x84\x82\x01aC\xB7V[\x90\x82\x01Ra\x01\x80aD\x9D\x84\x82\x01aC\xC5V[\x90\x82\x01Ra\x01\xA0aD\xAF\x84\x82\x01aC\xD5V[\x90\x82\x01Ra\x01\xC0aD\xC1\x84\x82\x01aC\xACV[\x90\x82\x01R\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aD\xDBW_\x80\xFD[aD\xE3aC~V[\x80`@\x84\x01\x85\x81\x11\x15aD\xF4W_\x80\xFD[\x84[\x81\x81\x10\x15aE\x17W\x805aE\t\x81aA\x8FV[\x84R` \x93\x84\x01\x93\x01aD\xF6V[P\x90\x95\x94PPPPPV[_`\x80\x82\x84\x03\x12\x15aE2W_\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aEUWaEUaC@V[`@R\x90P\x80`\x1F\x83\x01\x84\x13aEiW_\x80\xFD[aEqaC~V[\x80`@\x85\x01\x86\x81\x11\x15aE\x82W_\x80\xFD[\x85[\x81\x81\x10\x15aE\x9CW\x805\x84R` \x93\x84\x01\x93\x01aE\x84V[P\x81\x84RaE\xAA\x87\x82aD\xCCV[` \x85\x01RPPPP\x92\x91PPV[\x805`\x03\x81\x10a\x0F8W_\x80\xFD[_`\xC0\x82\x84\x03\x12\x15aE\xD7W_\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aE\xFAWaE\xFAaC@V[`@RaF\x07\x84\x84aE\"V[\x81RaF\x15`\x80\x84\x01aE\xB9V[` \x82\x01R`\xA0\x92\x90\x92\x015`@\x83\x01RP\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x0EzWa\x0EzaC\x0BV[_`\x80\x82\x84\x03\x12\x15aFOW_\x80\xFD[a\x10\xC3\x83\x83aE\"V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a6\xC3Wa6\xC3aC\x0BV[_` \x82\x84\x03\x12\x15aF\x8AW_\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aF\xB5W_\x80\xFD[\x81Qa\x10\xC3\x81a?\xFEV[\x80\x82\x01\x80\x82\x11\x15a\x0EzWa\x0EzaC\x0BV[_` \x82\x84\x03\x12\x15aF\xE3W_\x80\xFD[a\x10\xC3\x82aE\xB9V[aF\xF5\x81aB\x08V[\x90RV[`@\x81\x837`@\x82\x01`@\x82\x01_[`\x02\x81\x10\x15aG;W\x815aG\x1C\x81aA\x8FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01aG\x08V[PPPaGJ`\x80\x82\x01aE\xB9V[aGS\x81aB\x08V[`\x80\x83\x01R`\xA0\x90\x81\x015\x91\x01RV[_a\x03 \x82\x01\x90P\x885\x82R` \x89\x015` \x83\x01R`@\x89\x015`@\x83\x01R``\x89\x015``\x83\x01R`\x80\x89\x015aG\x9B\x81a?\xFEV[`\x01`\x01`\xA0\x1B\x03\x16`\x80\x83\x01R`\xA0\x89\x015aG\xB7\x81aA\x8FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\xA0\x84\x01R`\xC0\x8A\x015\x90aG\xD7\x82aA\x8FV[\x16`\xC0\x83\x01RaG\xED`\xE0\x80\x84\x01\x90\x8B\x01aF\xF9V[a\x01\xA0aG\xFE\x81\x84\x01\x82\x8C\x01aF\xF9V[P\x87a\x02`\x83\x01R\x86a\x02\x80\x83\x01R\x85a\x02\xA0\x83\x01R\x84a\x02\xC0\x83\x01RaH1a\x02\xE0\x83\x01\x85`\x01`\x01`\xA0\x1B\x03\x16\x90RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x03\0\x83\x01R\x98\x97PPPPPPPPV[\x81Q\x80Q`\xC0\x83\x01\x91\x90\x83_[`\x02\x81\x10\x15aHzW\x82Q\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01aH[V[PPP` \x90\x81\x01Q\x90`@\x84\x01_[`\x02\x81\x10\x15aH\xB1W\x83Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R\x92\x82\x01\x92\x90\x82\x01\x90`\x01\x01aH\x8AV[PPPP` \x83\x01QaH\xC7`\x80\x84\x01\x82aF\xECV[P`@\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[_` \x82\x84\x03\x12\x15aH\xE8W_\x80\xFD[a\x10\xC3\x82aC\xD5V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xB0*u\xC2{(\x1A\xB5\x1B\x1A\x9B\x90\xFA\xAA\x01\x02q\xB8\x99;2\xB3\x02Y\x95`\xAD\x12\xCD\xDDW\xE4dsolcC\0\x08\x19\x003",
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
    /**Event with signature `AdminChanged(address,address)` and selector `0x7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f`.
```solidity
event AdminChanged(address previousAdmin, address newAdmin);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AdminChanged {
        #[allow(missing_docs)]
        pub previousAdmin: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newAdmin: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for AdminChanged {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AdminChanged(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                126u8,
                100u8,
                77u8,
                121u8,
                66u8,
                47u8,
                23u8,
                192u8,
                30u8,
                72u8,
                148u8,
                181u8,
                244u8,
                245u8,
                136u8,
                211u8,
                49u8,
                235u8,
                250u8,
                40u8,
                101u8,
                61u8,
                66u8,
                174u8,
                131u8,
                45u8,
                197u8,
                158u8,
                56u8,
                201u8,
                121u8,
                143u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousAdmin: data.0,
                    newAdmin: data.1,
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
                        &self.previousAdmin,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newAdmin,
                    ),
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
        impl alloy_sol_types::private::IntoLogData for AdminChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AdminChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AdminChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
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
    /**Event with signature `BeaconUpgraded(address)` and selector `0x1cf3b03a6cf19fa2baba4df148e9dcabedea7f8a5c07840e207e5c089be95d3e`.
```solidity
event BeaconUpgraded(address indexed beacon);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BeaconUpgraded {
        #[allow(missing_docs)]
        pub beacon: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for BeaconUpgraded {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "BeaconUpgraded(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                28u8,
                243u8,
                176u8,
                58u8,
                108u8,
                241u8,
                159u8,
                162u8,
                186u8,
                186u8,
                77u8,
                241u8,
                72u8,
                233u8,
                220u8,
                171u8,
                237u8,
                234u8,
                127u8,
                138u8,
                92u8,
                7u8,
                132u8,
                14u8,
                32u8,
                126u8,
                92u8,
                8u8,
                155u8,
                233u8,
                93u8,
                62u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { beacon: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.beacon.clone())
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
                    &self.beacon,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for BeaconUpgraded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BeaconUpgraded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BeaconUpgraded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Initialized(uint8)` and selector `0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498`.
```solidity
event Initialized(uint8 version);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u8,
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
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                127u8,
                38u8,
                184u8,
                63u8,
                249u8,
                110u8,
                31u8,
                43u8,
                106u8,
                104u8,
                47u8,
                19u8,
                56u8,
                82u8,
                246u8,
                121u8,
                138u8,
                9u8,
                196u8,
                101u8,
                218u8,
                149u8,
                146u8,
                20u8,
                96u8,
                206u8,
                251u8,
                56u8,
                71u8,
                64u8,
                36u8,
                152u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
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
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
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
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Paused(address)` and selector `0x62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a258`.
```solidity
event Paused(address account);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Paused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for Paused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Paused(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                98u8,
                231u8,
                140u8,
                234u8,
                1u8,
                190u8,
                227u8,
                32u8,
                205u8,
                78u8,
                66u8,
                2u8,
                112u8,
                181u8,
                234u8,
                116u8,
                0u8,
                13u8,
                17u8,
                176u8,
                201u8,
                247u8,
                71u8,
                84u8,
                235u8,
                219u8,
                252u8,
                84u8,
                75u8,
                5u8,
                162u8,
                88u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { account: data.0 }
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
                        &self.account,
                    ),
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
        impl alloy_sol_types::private::IntoLogData for Paused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Paused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Paused) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `Unpaused(address)` and selector `0x5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa`.
```solidity
event Unpaused(address account);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Unpaused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for Unpaused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Unpaused(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                93u8,
                185u8,
                238u8,
                10u8,
                73u8,
                91u8,
                242u8,
                230u8,
                255u8,
                156u8,
                145u8,
                167u8,
                131u8,
                76u8,
                27u8,
                164u8,
                253u8,
                210u8,
                68u8,
                165u8,
                232u8,
                170u8,
                78u8,
                83u8,
                123u8,
                211u8,
                138u8,
                234u8,
                228u8,
                176u8,
                115u8,
                170u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { account: data.0 }
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
                        &self.account,
                    ),
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
        impl alloy_sol_types::private::IntoLogData for Unpaused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Unpaused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Unpaused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Upgraded(address)` and selector `0xbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b`.
```solidity
event Upgraded(address indexed implementation);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Upgraded {
        #[allow(missing_docs)]
        pub implementation: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for Upgraded {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Upgraded(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                188u8,
                124u8,
                215u8,
                90u8,
                32u8,
                238u8,
                39u8,
                253u8,
                154u8,
                222u8,
                186u8,
                179u8,
                32u8,
                65u8,
                247u8,
                85u8,
                33u8,
                77u8,
                188u8,
                107u8,
                255u8,
                169u8,
                12u8,
                192u8,
                34u8,
                91u8,
                57u8,
                218u8,
                46u8,
                92u8,
                45u8,
                59u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { implementation: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.implementation.clone())
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
                    &self.implementation,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Upgraded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Upgraded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Upgraded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `UpgradedSecondary(address)` and selector `0xf7eed2a7fabbf1bec8d55ed5e785cc76622376dde5df4ff15470551e030b8134`.
```solidity
event UpgradedSecondary(address indexed implementation);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UpgradedSecondary {
        #[allow(missing_docs)]
        pub implementation: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for UpgradedSecondary {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "UpgradedSecondary(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                247u8,
                238u8,
                210u8,
                167u8,
                250u8,
                187u8,
                241u8,
                190u8,
                200u8,
                213u8,
                94u8,
                213u8,
                231u8,
                133u8,
                204u8,
                118u8,
                98u8,
                35u8,
                118u8,
                221u8,
                229u8,
                223u8,
                79u8,
                241u8,
                84u8,
                112u8,
                85u8,
                30u8,
                3u8,
                11u8,
                129u8,
                52u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { implementation: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.implementation.clone())
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
                    &self.implementation,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UpgradedSecondary {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UpgradedSecondary> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UpgradedSecondary) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `_stakerMap(address)` and selector `0xe8bd4922`.
```solidity
function _stakerMap(address) external view returns (uint256 amountStaked, bytes32 latestStakedAssertion, uint64 index, bool isStaked, address withdrawalAddress);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct _stakerMapCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`_stakerMap(address)`](_stakerMapCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct _stakerMapReturn {
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
            impl ::core::convert::From<_stakerMapCall> for UnderlyingRustTuple<'_> {
                fn from(value: _stakerMapCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for _stakerMapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
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
            impl ::core::convert::From<_stakerMapReturn> for UnderlyingRustTuple<'_> {
                fn from(value: _stakerMapReturn) -> Self {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for _stakerMapReturn {
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
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for _stakerMapCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = _stakerMapReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Address,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "_stakerMap(address)";
            const SELECTOR: [u8; 4] = [232u8, 189u8, 73u8, 34u8];
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
    /**Function with signature `anyTrustFastConfirmer()` and selector `0x55840a58`.
```solidity
function anyTrustFastConfirmer() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct anyTrustFastConfirmerCall {}
    ///Container type for the return parameters of the [`anyTrustFastConfirmer()`](anyTrustFastConfirmerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct anyTrustFastConfirmerReturn {
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
            impl ::core::convert::From<anyTrustFastConfirmerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: anyTrustFastConfirmerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for anyTrustFastConfirmerCall {
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
            impl ::core::convert::From<anyTrustFastConfirmerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: anyTrustFastConfirmerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for anyTrustFastConfirmerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for anyTrustFastConfirmerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = anyTrustFastConfirmerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "anyTrustFastConfirmer()";
            const SELECTOR: [u8; 4] = [85u8, 132u8, 10u8, 88u8];
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
    /**Function with signature `challengeGracePeriodBlocks()` and selector `0x3be680ea`.
```solidity
function challengeGracePeriodBlocks() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengeGracePeriodBlocksCall {}
    ///Container type for the return parameters of the [`challengeGracePeriodBlocks()`](challengeGracePeriodBlocksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct challengeGracePeriodBlocksReturn {
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
            impl ::core::convert::From<challengeGracePeriodBlocksCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: challengeGracePeriodBlocksCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for challengeGracePeriodBlocksCall {
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
            impl ::core::convert::From<challengeGracePeriodBlocksReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: challengeGracePeriodBlocksReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for challengeGracePeriodBlocksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for challengeGracePeriodBlocksCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = challengeGracePeriodBlocksReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "challengeGracePeriodBlocks()";
            const SELECTOR: [u8; 4] = [59u8, 230u8, 128u8, 234u8];
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
    /**Function with signature `computeAssertionHash(bytes32,((bytes32[2],uint64[2]),uint8,bytes32),bytes32)` and selector `0x33635fc2`.
```solidity
function computeAssertionHash(bytes32 prevAssertionHash, AssertionState memory state, bytes32 inboxAcc) external pure returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct computeAssertionHashCall {
        pub prevAssertionHash: alloy::sol_types::private::FixedBytes<32>,
        pub state: <AssertionState as alloy::sol_types::SolType>::RustType,
        pub inboxAcc: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`computeAssertionHash(bytes32,((bytes32[2],uint64[2]),uint8,bytes32),bytes32)`](computeAssertionHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct computeAssertionHashReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                AssertionState,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <AssertionState as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<computeAssertionHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: computeAssertionHashCall) -> Self {
                    (value.prevAssertionHash, value.state, value.inboxAcc)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for computeAssertionHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        prevAssertionHash: tuple.0,
                        state: tuple.1,
                        inboxAcc: tuple.2,
                    }
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
            impl ::core::convert::From<computeAssertionHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: computeAssertionHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for computeAssertionHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for computeAssertionHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                AssertionState,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = computeAssertionHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "computeAssertionHash(bytes32,((bytes32[2],uint64[2]),uint8,bytes32),bytes32)";
            const SELECTOR: [u8; 4] = [51u8, 99u8, 95u8, 194u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prevAssertionHash),
                    <AssertionState as alloy_sol_types::SolType>::tokenize(&self.state),
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
    /**Function with signature `fastConfirmAssertion(bytes32,bytes32,((bytes32[2],uint64[2]),uint8,bytes32),bytes32)` and selector `0x6096686d`.
```solidity
function fastConfirmAssertion(bytes32 assertionHash, bytes32 parentAssertionHash, AssertionState memory confirmState, bytes32 inboxAcc) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fastConfirmAssertionCall {
        pub assertionHash: alloy::sol_types::private::FixedBytes<32>,
        pub parentAssertionHash: alloy::sol_types::private::FixedBytes<32>,
        pub confirmState: <AssertionState as alloy::sol_types::SolType>::RustType,
        pub inboxAcc: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`fastConfirmAssertion(bytes32,bytes32,((bytes32[2],uint64[2]),uint8,bytes32),bytes32)`](fastConfirmAssertionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fastConfirmAssertionReturn {}
    #[allow(
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
                <AssertionState as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<fastConfirmAssertionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: fastConfirmAssertionCall) -> Self {
                    (
                        value.assertionHash,
                        value.parentAssertionHash,
                        value.confirmState,
                        value.inboxAcc,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for fastConfirmAssertionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        assertionHash: tuple.0,
                        parentAssertionHash: tuple.1,
                        confirmState: tuple.2,
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
            impl ::core::convert::From<fastConfirmAssertionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: fastConfirmAssertionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for fastConfirmAssertionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for fastConfirmAssertionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                AssertionState,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = fastConfirmAssertionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "fastConfirmAssertion(bytes32,bytes32,((bytes32[2],uint64[2]),uint8,bytes32),bytes32)";
            const SELECTOR: [u8; 4] = [96u8, 150u8, 104u8, 109u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.parentAssertionHash),
                    <AssertionState as alloy_sol_types::SolType>::tokenize(
                        &self.confirmState,
                    ),
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
    /**Function with signature `fastConfirmNewAssertion(((bytes32,bytes32,(bytes32,uint256,address,uint64,uint64)),((bytes32[2],uint64[2]),uint8,bytes32),((bytes32[2],uint64[2]),uint8,bytes32)),bytes32)` and selector `0x6420fb9f`.
```solidity
function fastConfirmNewAssertion(AssertionInputs memory assertion, bytes32 expectedAssertionHash) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fastConfirmNewAssertionCall {
        pub assertion: <AssertionInputs as alloy::sol_types::SolType>::RustType,
        pub expectedAssertionHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`fastConfirmNewAssertion(((bytes32,bytes32,(bytes32,uint256,address,uint64,uint64)),((bytes32[2],uint64[2]),uint8,bytes32),((bytes32[2],uint64[2]),uint8,bytes32)),bytes32)`](fastConfirmNewAssertionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fastConfirmNewAssertionReturn {}
    #[allow(
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
            impl ::core::convert::From<fastConfirmNewAssertionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: fastConfirmNewAssertionCall) -> Self {
                    (value.assertion, value.expectedAssertionHash)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for fastConfirmNewAssertionCall {
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
            impl ::core::convert::From<fastConfirmNewAssertionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: fastConfirmNewAssertionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for fastConfirmNewAssertionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for fastConfirmNewAssertionCall {
            type Parameters<'a> = (
                AssertionInputs,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = fastConfirmNewAssertionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "fastConfirmNewAssertion(((bytes32,bytes32,(bytes32,uint256,address,uint64,uint64)),((bytes32[2],uint64[2]),uint8,bytes32),((bytes32[2],uint64[2]),uint8,bytes32)),bytes32)";
            const SELECTOR: [u8; 4] = [100u8, 32u8, 251u8, 159u8];
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
    /**Function with signature `inbox()` and selector `0xfb0e722b`.
```solidity
function inbox() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct inboxCall {}
    ///Container type for the return parameters of the [`inbox()`](inboxCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct inboxReturn {
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
            impl ::core::convert::From<inboxCall> for UnderlyingRustTuple<'_> {
                fn from(value: inboxCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for inboxCall {
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
            impl ::core::convert::From<inboxReturn> for UnderlyingRustTuple<'_> {
                fn from(value: inboxReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for inboxReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for inboxCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = inboxReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "inbox()";
            const SELECTOR: [u8; 4] = [251u8, 14u8, 114u8, 43u8];
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
function initialize(address _stakeToken) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub _stakeToken: alloy::sol_types::private::Address,
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
                    (value._stakeToken,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _stakeToken: tuple.0 }
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
                        &self._stakeToken,
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
function isValidator(address validator) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidatorCall {
        pub validator: alloy::sol_types::private::Address,
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
                    (value.validator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isValidatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { validator: tuple.0 }
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
                        &self.validator,
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
function newStake(uint256 tokenAmount, address _withdrawalAddress) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct newStakeCall {
        pub tokenAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub _withdrawalAddress: alloy::sol_types::private::Address,
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
                    (value.tokenAmount, value._withdrawalAddress)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for newStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenAmount: tuple.0,
                        _withdrawalAddress: tuple.1,
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
                        &self._withdrawalAddress,
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
function newStakeOnNewAssertion(uint256 tokenAmount, AssertionInputs memory assertion, bytes32 expectedAssertionHash, address _withdrawalAddress) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct newStakeOnNewAssertion_0Call {
        pub tokenAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub assertion: <AssertionInputs as alloy::sol_types::SolType>::RustType,
        pub expectedAssertionHash: alloy::sol_types::private::FixedBytes<32>,
        pub _withdrawalAddress: alloy::sol_types::private::Address,
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
                        value._withdrawalAddress,
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
                        _withdrawalAddress: tuple.3,
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
                        &self._withdrawalAddress,
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
    /**Function with signature `paused()` and selector `0x5c975abb`.
```solidity
function paused() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedCall {}
    ///Container type for the return parameters of the [`paused()`](pausedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedReturn {
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
            impl ::core::convert::From<pausedCall> for UnderlyingRustTuple<'_> {
                fn from(value: pausedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedCall {
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
            impl ::core::convert::From<pausedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pausedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pausedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pausedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 90u8, 187u8];
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
    /**Function with signature `proxiableUUID()` and selector `0x52d1902d`.
```solidity
function proxiableUUID() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxiableUUIDCall {}
    ///Container type for the return parameters of the [`proxiableUUID()`](proxiableUUIDCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxiableUUIDReturn {
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
            impl ::core::convert::From<proxiableUUIDCall> for UnderlyingRustTuple<'_> {
                fn from(value: proxiableUUIDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proxiableUUIDCall {
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
            impl ::core::convert::From<proxiableUUIDReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proxiableUUIDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proxiableUUIDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proxiableUUIDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proxiableUUIDReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proxiableUUID()";
            const SELECTOR: [u8; 4] = [82u8, 209u8, 144u8, 45u8];
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
    /**Function with signature `rollupDeploymentBlock()` and selector `0x1b1689e9`.
```solidity
function rollupDeploymentBlock() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rollupDeploymentBlockCall {}
    ///Container type for the return parameters of the [`rollupDeploymentBlock()`](rollupDeploymentBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rollupDeploymentBlockReturn {
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
            impl ::core::convert::From<rollupDeploymentBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: rollupDeploymentBlockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rollupDeploymentBlockCall {
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
            impl ::core::convert::From<rollupDeploymentBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: rollupDeploymentBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rollupDeploymentBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rollupDeploymentBlockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = rollupDeploymentBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rollupDeploymentBlock()";
            const SELECTOR: [u8; 4] = [27u8, 22u8, 137u8, 233u8];
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
    /**Function with signature `totalWithdrawableFunds()` and selector `0x71ef232c`.
```solidity
function totalWithdrawableFunds() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalWithdrawableFundsCall {}
    ///Container type for the return parameters of the [`totalWithdrawableFunds()`](totalWithdrawableFundsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalWithdrawableFundsReturn {
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
            impl ::core::convert::From<totalWithdrawableFundsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: totalWithdrawableFundsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for totalWithdrawableFundsCall {
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
            impl ::core::convert::From<totalWithdrawableFundsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: totalWithdrawableFundsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for totalWithdrawableFundsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for totalWithdrawableFundsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = totalWithdrawableFundsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "totalWithdrawableFunds()";
            const SELECTOR: [u8; 4] = [113u8, 239u8, 35u8, 44u8];
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
function validateAssertionHash(bytes32 assertionHash, AssertionState memory state, bytes32 prevAssertionHash, bytes32 inboxAcc) external pure;
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
    /**Function with signature `validatorWalletCreator()` and selector `0xbc45e0ae`.
```solidity
function validatorWalletCreator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorWalletCreatorCall {}
    ///Container type for the return parameters of the [`validatorWalletCreator()`](validatorWalletCreatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorWalletCreatorReturn {
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
            impl ::core::convert::From<validatorWalletCreatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatorWalletCreatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatorWalletCreatorCall {
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
            impl ::core::convert::From<validatorWalletCreatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatorWalletCreatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatorWalletCreatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatorWalletCreatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = validatorWalletCreatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validatorWalletCreator()";
            const SELECTOR: [u8; 4] = [188u8, 69u8, 224u8, 174u8];
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
function withdrawableFunds(address user) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawableFundsCall {
        pub user: alloy::sol_types::private::Address,
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
                    (value.user,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawableFundsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { user: tuple.0 }
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
                        &self.user,
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
    ///Container for all the [`RollupUserLogic`](self) function calls.
    pub enum RollupUserLogicCalls {
        _stakerMap(_stakerMapCall),
        addToDeposit(addToDepositCall),
        amountStaked(amountStakedCall),
        anyTrustFastConfirmer(anyTrustFastConfirmerCall),
        baseStake(baseStakeCall),
        bridge(bridgeCall),
        chainId(chainIdCall),
        challengeGracePeriodBlocks(challengeGracePeriodBlocksCall),
        challengeManager(challengeManagerCall),
        computeAssertionHash(computeAssertionHashCall),
        confirmAssertion(confirmAssertionCall),
        confirmPeriodBlocks(confirmPeriodBlocksCall),
        fastConfirmAssertion(fastConfirmAssertionCall),
        fastConfirmNewAssertion(fastConfirmNewAssertionCall),
        genesisAssertionHash(genesisAssertionHashCall),
        getAssertion(getAssertionCall),
        getAssertionCreationBlockForLogLookup(getAssertionCreationBlockForLogLookupCall),
        getFirstChildCreationBlock(getFirstChildCreationBlockCall),
        getSecondChildCreationBlock(getSecondChildCreationBlockCall),
        getStaker(getStakerCall),
        getStakerAddress(getStakerAddressCall),
        getValidators(getValidatorsCall),
        inbox(inboxCall),
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
        paused(pausedCall),
        proxiableUUID(proxiableUUIDCall),
        reduceDeposit(reduceDepositCall),
        removeWhitelistAfterFork(removeWhitelistAfterForkCall),
        removeWhitelistAfterValidatorAfk(removeWhitelistAfterValidatorAfkCall),
        returnOldDeposit(returnOldDepositCall),
        returnOldDepositFor(returnOldDepositForCall),
        rollupDeploymentBlock(rollupDeploymentBlockCall),
        rollupEventInbox(rollupEventInboxCall),
        sequencerInbox(sequencerInboxCall),
        stakeOnNewAssertion(stakeOnNewAssertionCall),
        stakeToken(stakeTokenCall),
        stakerCount(stakerCountCall),
        totalWithdrawableFunds(totalWithdrawableFundsCall),
        validateAssertionHash(validateAssertionHashCall),
        validateConfig(validateConfigCall),
        validatorAfkBlocks(validatorAfkBlocksCall),
        validatorWalletCreator(validatorWalletCreatorCall),
        validatorWhitelistDisabled(validatorWhitelistDisabledCall),
        wasmModuleRoot(wasmModuleRootCall),
        withdrawStakerFunds(withdrawStakerFundsCall),
        withdrawableFunds(withdrawableFundsCall),
        withdrawalAddress(withdrawalAddressCall),
    }
    #[automatically_derived]
    impl RollupUserLogicCalls {
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
            [27u8, 22u8, 137u8, 233u8],
            [30u8, 131u8, 211u8, 15u8],
            [42u8, 189u8, 210u8, 48u8],
            [46u8, 122u8, 207u8, 166u8],
            [47u8, 48u8, 202u8, 189u8],
            [48u8, 131u8, 98u8, 40u8],
            [51u8, 99u8, 95u8, 194u8],
            [53u8, 51u8, 37u8, 224u8],
            [59u8, 134u8, 222u8, 25u8],
            [59u8, 230u8, 128u8, 234u8],
            [69u8, 227u8, 139u8, 100u8],
            [80u8, 243u8, 47u8, 104u8],
            [81u8, 237u8, 106u8, 48u8],
            [82u8, 209u8, 144u8, 45u8],
            [85u8, 132u8, 10u8, 88u8],
            [86u8, 187u8, 201u8, 230u8],
            [87u8, 239u8, 74u8, 185u8],
            [88u8, 140u8, 122u8, 22u8],
            [92u8, 151u8, 90u8, 187u8],
            [96u8, 150u8, 104u8, 109u8],
            [97u8, 55u8, 57u8, 25u8],
            [97u8, 119u8, 253u8, 24u8],
            [100u8, 32u8, 251u8, 159u8],
            [101u8, 247u8, 248u8, 13u8],
            [104u8, 18u8, 155u8, 20u8],
            [104u8, 95u8, 94u8, 204u8],
            [109u8, 221u8, 55u8, 68u8],
            [113u8, 239u8, 35u8, 44u8],
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
            [188u8, 69u8, 224u8, 174u8],
            [194u8, 194u8, 230u8, 142u8],
            [196u8, 214u8, 109u8, 232u8],
            [206u8, 17u8, 230u8, 171u8],
            [223u8, 246u8, 151u8, 135u8],
            [229u8, 16u8, 25u8, 166u8],
            [229u8, 49u8, 216u8, 199u8],
            [230u8, 179u8, 8u8, 44u8],
            [231u8, 140u8, 234u8, 146u8],
            [232u8, 189u8, 73u8, 34u8],
            [238u8, 53u8, 243u8, 39u8],
            [239u8, 64u8, 166u8, 112u8],
            [240u8, 101u8, 222u8, 63u8],
            [250u8, 205u8, 116u8, 59u8],
            [251u8, 14u8, 114u8, 43u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for RollupUserLogicCalls {
        const NAME: &'static str = "RollupUserLogicCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 60usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::_stakerMap(_) => {
                    <_stakerMapCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addToDeposit(_) => {
                    <addToDepositCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::amountStaked(_) => {
                    <amountStakedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::anyTrustFastConfirmer(_) => {
                    <anyTrustFastConfirmerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::baseStake(_) => {
                    <baseStakeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::bridge(_) => <bridgeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::chainId(_) => <chainIdCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::challengeGracePeriodBlocks(_) => {
                    <challengeGracePeriodBlocksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::challengeManager(_) => {
                    <challengeManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::computeAssertionHash(_) => {
                    <computeAssertionHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::confirmAssertion(_) => {
                    <confirmAssertionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::confirmPeriodBlocks(_) => {
                    <confirmPeriodBlocksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::fastConfirmAssertion(_) => {
                    <fastConfirmAssertionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::fastConfirmNewAssertion(_) => {
                    <fastConfirmNewAssertionCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::inbox(_) => <inboxCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::paused(_) => <pausedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
                Self::rollupDeploymentBlock(_) => {
                    <rollupDeploymentBlockCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::totalWithdrawableFunds(_) => {
                    <totalWithdrawableFundsCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::validatorWalletCreator(_) => {
                    <validatorWalletCreatorCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<RollupUserLogicCalls>] = &[
                {
                    fn challengeManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <challengeManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::challengeManager)
                    }
                    challengeManager
                },
                {
                    fn validateConfig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <validateConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::validateConfig)
                    }
                    validateConfig
                },
                {
                    fn confirmAssertion(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <confirmAssertionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::confirmAssertion)
                    }
                    confirmAssertion
                },
                {
                    fn getFirstChildCreationBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <getFirstChildCreationBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::getFirstChildCreationBlock)
                    }
                    getFirstChildCreationBlock
                },
                {
                    fn validatorWhitelistDisabled(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <validatorWhitelistDisabledCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::validatorWhitelistDisabled)
                    }
                    validatorWhitelistDisabled
                },
                {
                    fn getAssertionCreationBlockForLogLookup(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <getAssertionCreationBlockForLogLookupCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RollupUserLogicCalls::getAssertionCreationBlockForLogLookup,
                            )
                    }
                    getAssertionCreationBlockForLogLookup
                },
                {
                    fn removeWhitelistAfterValidatorAfk(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <removeWhitelistAfterValidatorAfkCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::removeWhitelistAfterValidatorAfk)
                    }
                    removeWhitelistAfterValidatorAfk
                },
                {
                    fn rollupDeploymentBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <rollupDeploymentBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::rollupDeploymentBlock)
                    }
                    rollupDeploymentBlock
                },
                {
                    fn reduceDeposit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <reduceDepositCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::reduceDeposit)
                    }
                    reduceDeposit
                },
                {
                    fn latestStakedAssertion(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <latestStakedAssertionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::latestStakedAssertion)
                    }
                    latestStakedAssertion
                },
                {
                    fn confirmPeriodBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <confirmPeriodBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::confirmPeriodBlocks)
                    }
                    confirmPeriodBlocks
                },
                {
                    fn withdrawableFunds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <withdrawableFundsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::withdrawableFunds)
                    }
                    withdrawableFunds
                },
                {
                    fn isFirstChild(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <isFirstChildCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::isFirstChild)
                    }
                    isFirstChild
                },
                {
                    fn computeAssertionHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <computeAssertionHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::computeAssertionHash)
                    }
                    computeAssertionHash
                },
                {
                    fn genesisAssertionHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <genesisAssertionHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::genesisAssertionHash)
                    }
                    genesisAssertionHash
                },
                {
                    fn stakeOnNewAssertion(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <stakeOnNewAssertionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::stakeOnNewAssertion)
                    }
                    stakeOnNewAssertion
                },
                {
                    fn challengeGracePeriodBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <challengeGracePeriodBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::challengeGracePeriodBlocks)
                    }
                    challengeGracePeriodBlocks
                },
                {
                    fn minimumAssertionPeriod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <minimumAssertionPeriodCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::minimumAssertionPeriod)
                    }
                    minimumAssertionPeriod
                },
                {
                    fn newStakeOnNewAssertion_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <newStakeOnNewAssertion_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::newStakeOnNewAssertion_0)
                    }
                    newStakeOnNewAssertion_0
                },
                {
                    fn stakeToken(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <stakeTokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::stakeToken)
                    }
                    stakeToken
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn anyTrustFastConfirmer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <anyTrustFastConfirmerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::anyTrustFastConfirmer)
                    }
                    anyTrustFastConfirmer
                },
                {
                    fn getSecondChildCreationBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <getSecondChildCreationBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::getSecondChildCreationBlock)
                    }
                    getSecondChildCreationBlock
                },
                {
                    fn returnOldDeposit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <returnOldDepositCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::returnOldDeposit)
                    }
                    returnOldDeposit
                },
                {
                    fn returnOldDepositFor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <returnOldDepositForCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::returnOldDepositFor)
                    }
                    returnOldDepositFor
                },
                {
                    fn paused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::paused)
                    }
                    paused
                },
                {
                    fn fastConfirmAssertion(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <fastConfirmAssertionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::fastConfirmAssertion)
                    }
                    fastConfirmAssertion
                },
                {
                    fn withdrawStakerFunds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <withdrawStakerFundsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::withdrawStakerFunds)
                    }
                    withdrawStakerFunds
                },
                {
                    fn isStaked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <isStakedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::isStaked)
                    }
                    isStaked
                },
                {
                    fn fastConfirmNewAssertion(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <fastConfirmNewAssertionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::fastConfirmNewAssertion)
                    }
                    fastConfirmNewAssertion
                },
                {
                    fn latestConfirmed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <latestConfirmedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::latestConfirmed)
                    }
                    latestConfirmed
                },
                {
                    fn newStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <newStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::newStake)
                    }
                    newStake
                },
                {
                    fn addToDeposit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <addToDepositCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::addToDeposit)
                    }
                    addToDeposit
                },
                {
                    fn getStakerAddress(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <getStakerAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::getStakerAddress)
                    }
                    getStakerAddress
                },
                {
                    fn totalWithdrawableFunds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <totalWithdrawableFundsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::totalWithdrawableFunds)
                    }
                    totalWithdrawableFunds
                },
                {
                    fn newStakeOnNewAssertion_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <newStakeOnNewAssertion_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::newStakeOnNewAssertion_1)
                    }
                    newStakeOnNewAssertion_1
                },
                {
                    fn baseStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <baseStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::baseStake)
                    }
                    baseStake
                },
                {
                    fn withdrawalAddress(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <withdrawalAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::withdrawalAddress)
                    }
                    withdrawalAddress
                },
                {
                    fn getAssertion(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <getAssertionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::getAssertion)
                    }
                    getAssertion
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::owner)
                    }
                    owner
                },
                {
                    fn wasmModuleRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <wasmModuleRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::wasmModuleRoot)
                    }
                    wasmModuleRoot
                },
                {
                    fn chainId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <chainIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::chainId)
                    }
                    chainId
                },
                {
                    fn getStaker(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <getStakerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::getStaker)
                    }
                    getStaker
                },
                {
                    fn rollupEventInbox(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <rollupEventInboxCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::rollupEventInbox)
                    }
                    rollupEventInbox
                },
                {
                    fn getValidators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <getValidatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::getValidators)
                    }
                    getValidators
                },
                {
                    fn validatorWalletCreator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <validatorWalletCreatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::validatorWalletCreator)
                    }
                    validatorWalletCreator
                },
                {
                    fn removeWhitelistAfterFork(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <removeWhitelistAfterForkCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::removeWhitelistAfterFork)
                    }
                    removeWhitelistAfterFork
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::initialize)
                    }
                    initialize
                },
                {
                    fn outbox(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <outboxCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::outbox)
                    }
                    outbox
                },
                {
                    fn stakerCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <stakerCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::stakerCount)
                    }
                    stakerCount
                },
                {
                    fn validateAssertionHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <validateAssertionHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::validateAssertionHash)
                    }
                    validateAssertionHash
                },
                {
                    fn isPending(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <isPendingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::isPending)
                    }
                    isPending
                },
                {
                    fn validatorAfkBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <validatorAfkBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::validatorAfkBlocks)
                    }
                    validatorAfkBlocks
                },
                {
                    fn bridge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <bridgeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::bridge)
                    }
                    bridge
                },
                {
                    fn _stakerMap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <_stakerMapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::_stakerMap)
                    }
                    _stakerMap
                },
                {
                    fn sequencerInbox(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <sequencerInboxCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::sequencerInbox)
                    }
                    sequencerInbox
                },
                {
                    fn amountStaked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <amountStakedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::amountStaked)
                    }
                    amountStaked
                },
                {
                    fn loserStakeEscrow(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <loserStakeEscrowCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::loserStakeEscrow)
                    }
                    loserStakeEscrow
                },
                {
                    fn isValidator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <isValidatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::isValidator)
                    }
                    isValidator
                },
                {
                    fn inbox(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RollupUserLogicCalls> {
                        <inboxCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RollupUserLogicCalls::inbox)
                    }
                    inbox
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
                Self::_stakerMap(inner) => {
                    <_stakerMapCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
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
                Self::anyTrustFastConfirmer(inner) => {
                    <anyTrustFastConfirmerCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::challengeGracePeriodBlocks(inner) => {
                    <challengeGracePeriodBlocksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::challengeManager(inner) => {
                    <challengeManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::computeAssertionHash(inner) => {
                    <computeAssertionHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::fastConfirmAssertion(inner) => {
                    <fastConfirmAssertionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::fastConfirmNewAssertion(inner) => {
                    <fastConfirmNewAssertionCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::inbox(inner) => {
                    <inboxCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::rollupDeploymentBlock(inner) => {
                    <rollupDeploymentBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::totalWithdrawableFunds(inner) => {
                    <totalWithdrawableFundsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::validatorWalletCreator(inner) => {
                    <validatorWalletCreatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::_stakerMap(inner) => {
                    <_stakerMapCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
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
                Self::anyTrustFastConfirmer(inner) => {
                    <anyTrustFastConfirmerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::challengeGracePeriodBlocks(inner) => {
                    <challengeGracePeriodBlocksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::challengeManager(inner) => {
                    <challengeManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::computeAssertionHash(inner) => {
                    <computeAssertionHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::fastConfirmAssertion(inner) => {
                    <fastConfirmAssertionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::fastConfirmNewAssertion(inner) => {
                    <fastConfirmNewAssertionCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::inbox(inner) => {
                    <inboxCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
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
                Self::rollupDeploymentBlock(inner) => {
                    <rollupDeploymentBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::totalWithdrawableFunds(inner) => {
                    <totalWithdrawableFundsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::validatorWalletCreator(inner) => {
                    <validatorWalletCreatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`RollupUserLogic`](self) events.
    pub enum RollupUserLogicEvents {
        AdminChanged(AdminChanged),
        AssertionConfirmed(AssertionConfirmed),
        AssertionCreated(AssertionCreated),
        BeaconUpgraded(BeaconUpgraded),
        Initialized(Initialized),
        Paused(Paused),
        RollupChallengeStarted(RollupChallengeStarted),
        RollupInitialized(RollupInitialized),
        Unpaused(Unpaused),
        Upgraded(Upgraded),
        UpgradedSecondary(UpgradedSecondary),
        UserStakeUpdated(UserStakeUpdated),
        UserWithdrawableFundsUpdated(UserWithdrawableFundsUpdated),
    }
    #[automatically_derived]
    impl RollupUserLogicEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                28u8,
                243u8,
                176u8,
                58u8,
                108u8,
                241u8,
                159u8,
                162u8,
                186u8,
                186u8,
                77u8,
                241u8,
                72u8,
                233u8,
                220u8,
                171u8,
                237u8,
                234u8,
                127u8,
                138u8,
                92u8,
                7u8,
                132u8,
                14u8,
                32u8,
                126u8,
                92u8,
                8u8,
                155u8,
                233u8,
                93u8,
                62u8,
            ],
            [
                93u8,
                185u8,
                238u8,
                10u8,
                73u8,
                91u8,
                242u8,
                230u8,
                255u8,
                156u8,
                145u8,
                167u8,
                131u8,
                76u8,
                27u8,
                164u8,
                253u8,
                210u8,
                68u8,
                165u8,
                232u8,
                170u8,
                78u8,
                83u8,
                123u8,
                211u8,
                138u8,
                234u8,
                228u8,
                176u8,
                115u8,
                170u8,
            ],
            [
                98u8,
                231u8,
                140u8,
                234u8,
                1u8,
                190u8,
                227u8,
                32u8,
                205u8,
                78u8,
                66u8,
                2u8,
                112u8,
                181u8,
                234u8,
                116u8,
                0u8,
                13u8,
                17u8,
                176u8,
                201u8,
                247u8,
                71u8,
                84u8,
                235u8,
                219u8,
                252u8,
                84u8,
                75u8,
                5u8,
                162u8,
                88u8,
            ],
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
                126u8,
                100u8,
                77u8,
                121u8,
                66u8,
                47u8,
                23u8,
                192u8,
                30u8,
                72u8,
                148u8,
                181u8,
                244u8,
                245u8,
                136u8,
                211u8,
                49u8,
                235u8,
                250u8,
                40u8,
                101u8,
                61u8,
                66u8,
                174u8,
                131u8,
                45u8,
                197u8,
                158u8,
                56u8,
                201u8,
                121u8,
                143u8,
            ],
            [
                127u8,
                38u8,
                184u8,
                63u8,
                249u8,
                110u8,
                31u8,
                43u8,
                106u8,
                104u8,
                47u8,
                19u8,
                56u8,
                82u8,
                246u8,
                121u8,
                138u8,
                9u8,
                196u8,
                101u8,
                218u8,
                149u8,
                146u8,
                20u8,
                96u8,
                206u8,
                251u8,
                56u8,
                71u8,
                64u8,
                36u8,
                152u8,
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
                188u8,
                124u8,
                215u8,
                90u8,
                32u8,
                238u8,
                39u8,
                253u8,
                154u8,
                222u8,
                186u8,
                179u8,
                32u8,
                65u8,
                247u8,
                85u8,
                33u8,
                77u8,
                188u8,
                107u8,
                255u8,
                169u8,
                12u8,
                192u8,
                34u8,
                91u8,
                57u8,
                218u8,
                46u8,
                92u8,
                45u8,
                59u8,
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
                247u8,
                238u8,
                210u8,
                167u8,
                250u8,
                187u8,
                241u8,
                190u8,
                200u8,
                213u8,
                94u8,
                213u8,
                231u8,
                133u8,
                204u8,
                118u8,
                98u8,
                35u8,
                118u8,
                221u8,
                229u8,
                223u8,
                79u8,
                241u8,
                84u8,
                112u8,
                85u8,
                30u8,
                3u8,
                11u8,
                129u8,
                52u8,
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
    impl alloy_sol_types::SolEventInterface for RollupUserLogicEvents {
        const NAME: &'static str = "RollupUserLogicEvents";
        const COUNT: usize = 13usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<AdminChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AdminChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::AdminChanged)
                }
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
                Some(<BeaconUpgraded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <BeaconUpgraded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::BeaconUpgraded)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Paused)
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
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Unpaused)
                }
                Some(<Upgraded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Upgraded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Upgraded)
                }
                Some(
                    <UpgradedSecondary as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UpgradedSecondary as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::UpgradedSecondary)
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
    impl alloy_sol_types::private::IntoLogData for RollupUserLogicEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::AssertionConfirmed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::AssertionCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::BeaconUpgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RollupChallengeStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RollupInitialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UpgradedSecondary(inner) => {
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
                Self::AdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::AssertionConfirmed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::AssertionCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::BeaconUpgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RollupChallengeStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RollupInitialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UpgradedSecondary(inner) => {
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
    /**Creates a new wrapper around an on-chain [`RollupUserLogic`](self) contract instance.

See the [wrapper's documentation](`RollupUserLogicInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> RollupUserLogicInstance<T, P, N> {
        RollupUserLogicInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<RollupUserLogicInstance<T, P, N>>,
    > {
        RollupUserLogicInstance::<T, P, N>::deploy(provider)
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
        RollupUserLogicInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`RollupUserLogic`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`RollupUserLogic`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct RollupUserLogicInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for RollupUserLogicInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("RollupUserLogicInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > RollupUserLogicInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`RollupUserLogic`](self) contract instance.

See the [wrapper's documentation](`RollupUserLogicInstance`) for more details.*/
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
        ) -> alloy_contract::Result<RollupUserLogicInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> RollupUserLogicInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> RollupUserLogicInstance<T, P, N> {
            RollupUserLogicInstance {
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
    > RollupUserLogicInstance<T, P, N> {
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
        ///Creates a new call builder for the [`_stakerMap`] function.
        pub fn _stakerMap(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, _stakerMapCall, N> {
            self.call_builder(&_stakerMapCall { _0 })
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
        ///Creates a new call builder for the [`anyTrustFastConfirmer`] function.
        pub fn anyTrustFastConfirmer(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, anyTrustFastConfirmerCall, N> {
            self.call_builder(&anyTrustFastConfirmerCall {})
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
        ///Creates a new call builder for the [`challengeGracePeriodBlocks`] function.
        pub fn challengeGracePeriodBlocks(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, challengeGracePeriodBlocksCall, N> {
            self.call_builder(&challengeGracePeriodBlocksCall {})
        }
        ///Creates a new call builder for the [`challengeManager`] function.
        pub fn challengeManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, challengeManagerCall, N> {
            self.call_builder(&challengeManagerCall {})
        }
        ///Creates a new call builder for the [`computeAssertionHash`] function.
        pub fn computeAssertionHash(
            &self,
            prevAssertionHash: alloy::sol_types::private::FixedBytes<32>,
            state: <AssertionState as alloy::sol_types::SolType>::RustType,
            inboxAcc: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, computeAssertionHashCall, N> {
            self.call_builder(
                &computeAssertionHashCall {
                    prevAssertionHash,
                    state,
                    inboxAcc,
                },
            )
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
        ///Creates a new call builder for the [`fastConfirmAssertion`] function.
        pub fn fastConfirmAssertion(
            &self,
            assertionHash: alloy::sol_types::private::FixedBytes<32>,
            parentAssertionHash: alloy::sol_types::private::FixedBytes<32>,
            confirmState: <AssertionState as alloy::sol_types::SolType>::RustType,
            inboxAcc: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, fastConfirmAssertionCall, N> {
            self.call_builder(
                &fastConfirmAssertionCall {
                    assertionHash,
                    parentAssertionHash,
                    confirmState,
                    inboxAcc,
                },
            )
        }
        ///Creates a new call builder for the [`fastConfirmNewAssertion`] function.
        pub fn fastConfirmNewAssertion(
            &self,
            assertion: <AssertionInputs as alloy::sol_types::SolType>::RustType,
            expectedAssertionHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, fastConfirmNewAssertionCall, N> {
            self.call_builder(
                &fastConfirmNewAssertionCall {
                    assertion,
                    expectedAssertionHash,
                },
            )
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
        ///Creates a new call builder for the [`inbox`] function.
        pub fn inbox(&self) -> alloy_contract::SolCallBuilder<T, &P, inboxCall, N> {
            self.call_builder(&inboxCall {})
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _stakeToken: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall { _stakeToken })
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
            validator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isValidatorCall, N> {
            self.call_builder(&isValidatorCall { validator })
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
            _withdrawalAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, newStakeCall, N> {
            self.call_builder(
                &newStakeCall {
                    tokenAmount,
                    _withdrawalAddress,
                },
            )
        }
        ///Creates a new call builder for the [`newStakeOnNewAssertion_0`] function.
        pub fn newStakeOnNewAssertion_0(
            &self,
            tokenAmount: alloy::sol_types::private::primitives::aliases::U256,
            assertion: <AssertionInputs as alloy::sol_types::SolType>::RustType,
            expectedAssertionHash: alloy::sol_types::private::FixedBytes<32>,
            _withdrawalAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, newStakeOnNewAssertion_0Call, N> {
            self.call_builder(
                &newStakeOnNewAssertion_0Call {
                    tokenAmount,
                    assertion,
                    expectedAssertionHash,
                    _withdrawalAddress,
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
        ///Creates a new call builder for the [`paused`] function.
        pub fn paused(&self) -> alloy_contract::SolCallBuilder<T, &P, pausedCall, N> {
            self.call_builder(&pausedCall {})
        }
        ///Creates a new call builder for the [`proxiableUUID`] function.
        pub fn proxiableUUID(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, proxiableUUIDCall, N> {
            self.call_builder(&proxiableUUIDCall {})
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
        ///Creates a new call builder for the [`rollupDeploymentBlock`] function.
        pub fn rollupDeploymentBlock(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, rollupDeploymentBlockCall, N> {
            self.call_builder(&rollupDeploymentBlockCall {})
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
        ///Creates a new call builder for the [`totalWithdrawableFunds`] function.
        pub fn totalWithdrawableFunds(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, totalWithdrawableFundsCall, N> {
            self.call_builder(&totalWithdrawableFundsCall {})
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
        ///Creates a new call builder for the [`validatorWalletCreator`] function.
        pub fn validatorWalletCreator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, validatorWalletCreatorCall, N> {
            self.call_builder(&validatorWalletCreatorCall {})
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
            user: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawableFundsCall, N> {
            self.call_builder(&withdrawableFundsCall { user })
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
    > RollupUserLogicInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`AdminChanged`] event.
        pub fn AdminChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AdminChanged, N> {
            self.event_filter::<AdminChanged>()
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
        ///Creates a new event filter for the [`BeaconUpgraded`] event.
        pub fn BeaconUpgraded_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, BeaconUpgraded, N> {
            self.event_filter::<BeaconUpgraded>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`Paused`] event.
        pub fn Paused_filter(&self) -> alloy_contract::Event<T, &P, Paused, N> {
            self.event_filter::<Paused>()
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
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<T, &P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
        ///Creates a new event filter for the [`UpgradedSecondary`] event.
        pub fn UpgradedSecondary_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, UpgradedSecondary, N> {
            self.event_filter::<UpgradedSecondary>()
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
