## Metrics

### Key Metrics
The following metrics are implemented and exposed via the `/metrics` endpoint in the OpenMetrics format:

#### Ingestor
1. **ingestor_rpc_calls**: Tracks the number of RPC method calls.
   - **Labels**:
     - `chain`: Specifies whether the call is related to the Sequencing or Settlement chain.
     - `method`: The RPC method name.
   - **Metric Type**: Counter

2. **ingestor_rpc_calls_latency**: Measures the latency of RPC method call responses.
   - **Labels**:
     - `chain`: Specifies whether the call is related to the Sequencing or Settlement chain.
     - `method`: The RPC method name.
   - **Metric Type**: Histogram

3. **ingestor_last_block_fetched**: Tracks the last block number fetched for a specific RPC URL.
   - **Labels**:
     - `chain`: Specifies whether the call is related to the Sequencing or Settlement chain.
     - `method`: Always set to `"last_block_fetched"`.
   - **Metric Type**: Gauge

#### Slotting

1. **slotting_last_processed_block**: Tracks the last block number processed by the Slotting.
   - **Labels**:
     - `chain`: Specifies whether the block belongs to the Sequencing or Settlement chain.
   - **Metric Type**: Gauge

2. **slotting_active_slots**: Tracks the number of active slots being processed.
   - **Metric Type**: Gauge

3. **slotting_timestamp_lag_ms**: Tracks the timestamp lag (ms) for the sequencing chain.
   - **Labels**:
     - `chain`: Specifies whether the block belongs to the Sequencing or Settlement chain.
   - **Metric Type**: Gauge

4. **slotting_blocks_per_slot**: Tracks the number of blocks processed per slot.
   - **Metric Type**: Histogram

5. **slotting_channel_capacity**: Tracks the capacity of the sequencing chain channel.
   - **Labels**:
     - `chain`: Specifies whether the channel belongs to the Sequencing or Settlement chain.
   - **Metric Type**: Gauge


#### Block Builder

TODO SEQ-505
