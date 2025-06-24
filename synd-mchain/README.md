# `MockChain`

The `mchain` service is a high-performance blockchain interface used by the `synd-translator` as the parent chain for appchain block derivation.
Instead of a truly distributed blockchain, the `mchain` uses rocksdb to store batches and delayed messages.