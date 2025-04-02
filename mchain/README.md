# `Metachain`

The `mchain` service is a high-performance blockchain interface used by the metabased translator as the parent chain for rollup block derivation.
Instead of a physical blockchain, the mchain uses rocksdb to store batches and delayed messages instead.