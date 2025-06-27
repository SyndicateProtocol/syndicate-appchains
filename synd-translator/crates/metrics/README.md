## `Synd-translator` Metrics

### Key Metrics
The following metrics are implemented and exposed via the `/metrics` endpoint in the OpenMetrics format:

#### `synd-slotter`

1. **slotter_last_processed_block**: Tracks the last block number processed by the Slotter.
   - **Labels**:
     - `chain`: Specifies whether the block belongs to the Sequencing or Settlement chain.
   - **Metric Type**: Gauge

2. **slotter_timestamp_lag_ms**: Tracks the timestamp lag (ms) for the both chains.
   - **Labels**:
     - `chain`: Specifies whether the block belongs to the Sequencing or Settlement chain.
   - **Metric Type**: Gauge

3. **slotter_blocks_per_slot**: Tracks the number of blocks processed per slot.
   - **Metric Type**: Histogram

4. **slotter_last_slot_created**: Tracks the last slot number created by the Slotter.
   - **Metric Type**: Gauge
