# metabased-sequencer
The service for the write side of the metabased sequencer

## Example command
Sample CLI invocation of the `interceptor` profile:
```
cargo run -p interceptor -- --chain-rpc-address http://127.0.0.1:8547 --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 --chain-contract-address 0xa6e41ffd769491a42a6e5ce453259b93983a22ef --port_sequencer 9999
```