# Assertion
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/e670fbd66628d486b7f0c62387b907c2a44879ed/src/withdrawal/AssertionPoster.sol)

*Defines the structure of an assertion in the legacy rollup system*


```solidity
struct Assertion {
    ExecutionState beforeState;
    ExecutionState afterState;
    uint64 numBlocks;
}
```

