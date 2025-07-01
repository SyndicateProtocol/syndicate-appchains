# ZK proofs for TEE attestations

This directory contains a few components;
 - `aws-nitro` - contains generic and reusable code to validate AWS nitro TEE certificates. The code in this crate can be used on different zk proof engines
 - `sp1` - sp1 implementation for zk proving of TEE attestion documents. Initially only supports AWS nitro.
 - `proof-submitter` - CLI utility used to generate and submit zk proofs
