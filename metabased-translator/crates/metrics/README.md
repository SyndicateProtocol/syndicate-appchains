## Metrics

### Key Metrics
The following metrics are implemented and exposed via the `/metrics` endpoint in the OpenMetrics format:

#### Ingestor
1. **rpc_calls**: Tracks the number of RPC method calls.
   - **Labels**:
     - `label_name`: The RPC identifier or source.
     - `method`: The RPC method name.
   - **Metric Type**: Counter

2. **rpc_calls_latency**: Measures the latency of RPC method call responses.
   - **Labels**:
     - `label_name`: The RPC identifier or source.
     - `method`: The RPC method name.
   - **Metric Type**: Histogram

3. **last_block_fetched**: Tracks the last block number fetched for a specific RPC URL.
   - **Labels**:
     - `label_name`: The RPC identifier or source.
     - `method`: Always set to `"last_block_fetched"`.
   - **Metric Type**: Gauge

#### Slotting

1. **slotting_last_sequencing_block**: Tracks the last block number fetched from the sequencing chain.
   - **Metric Type**: Gauge

2. **slotting_last_settlement_block**: Tracks the last block number fetched from the settlement chain.
   - **Metric Type**: Gauge

3. **slotting_active_slots**: Tracks the number of active slots being processed.
   - **Metric Type**: Gauge

4. **slotting_status**: Indicates the current status of the Slotter (0 = NotStarted, 1 = Started, 2 = Stopped).
   - **Metric Type**: Gauge

#### Block builder

TODO SEQ-505