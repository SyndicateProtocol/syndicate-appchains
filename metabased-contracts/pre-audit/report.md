# Pre audit report

## Coverage command: 
Run: [run](./coverage_run.png)
```
forge coverage --no-match-coverage "(script|test|use-cases|BidStructured|TokenBalanceSequencingModule|SealedBidAuctionSequencingModule|MetafillerStorage)"
```

## Analyzer command: 
Run: [run](./analyzer_run.png)

```
olympix_release analyze -w .  --no-incomplete-constructor-tests --no-unfuzzed-local-variables --no-insufficient-parameter-assertion --no-no-parameter-validation-in-constructor --no-unfuzzed-variables --no-missing-revert-reason-tests --no-call-without-gas-budget --no-signature-replay-attacks --no-uninitialized-state-variable --no-unused-return-function-call
```


## Mutation stats across all included files:
Runs: [run1](./mutation_run_1.png), [run2](./mutation_run_2.png), [run3](./mutation_run_3.png),

```
Total mutations introduced: 78
Total mutations survived  : 17
Total mutations killed    : 61
Mutation score            : 78.20%
```