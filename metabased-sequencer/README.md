# metabased-sequencer
The service for the write side of the metabased sequencer

## Example command
Sample CLI invocation of the `interceptor` profile:
```
cargo run -p interceptor -- --chain-rpc-address http://127.0.0.1:8547 --private-key 0x_MyKeyHere --chain-contract-address 0x_MyAddressHere --port 9999
```