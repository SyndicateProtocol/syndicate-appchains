# op-translator

This is a service that runs a proxy server to receive JSON-RPC calls.

Run tests with `make test`

# Config priority
The priority of config flag settings is as follows:
1. Command line
2. Dotenv (.env) file
3. Defaults from code

This means that if `port` is set to `1234` in the .env file, but `--port 5678` is passed on the command line, the server will listen on port `5678`.