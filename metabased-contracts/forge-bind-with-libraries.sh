#!/bin/bash
# Script to run forge bind with libraries flag for CI

# Run forge bind with the RLPTxDecoder library linked
forge bind --alloy --force -C ./sequencing-modules --libraries src/use-cases/dream/RLP/RLPTxDecoder.sol:RLPTxDecoder:0x1234567890123456789012345678901234567890

# Move lib.rs to mod.rs as required
mv out/bindings/src/lib.rs out/bindings/src/mod.rs
