# Assertion
[Git Source](https://github.com/SyndicateProtocol/syndicate-appchains/blob/b28027a30c67e2de9f45368bdf6d7b4aecf3b0cf/src/withdrawal/AssertionPoster.sol)

*Defines the structure of an assertion in the legacy rollup system*


```solidity
struct Assertion {
    ExecutionState beforeState;
    ExecutionState afterState;
    uint64 numBlocks;
}
```

