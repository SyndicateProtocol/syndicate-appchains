# `MockChain`

The `mchain` service is a high-performance blockchain interface used by the Syndicate translator as the parent chain for appchain block derivation.
Instead of a physical blockchain, the mchain uses rocksdb to store batches and delayed messages instead.