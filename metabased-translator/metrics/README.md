## Metrics

This module provides a robust metrics system for monitoring and analyzing key performance indicators in your application using Prometheus.

### Key Metrics
The following metrics are implemented and exposed via the `/metrics` endpoint in the OpenMetrics format:

### Ingestor
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
