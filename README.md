# `Syndicate-Appchains` Monorepo

The monorepo for the Syndicate Appchains stack. Contains the appchain translator, sequencer, smart contracts, RPC node, dev environments, and more!

## Docker

Use the included Dockerfile at repo root to build a Docker image. You should then be able to run the image and pass in the same flags
that you would pass to the Rust binary executable.

Note that this a multi-target Dockerfile, so specify `synd-translator`, `synd-poster`, etc. as the target.

```
docker build --target synd-translator -f Dockerfile . --tag synd-translator

docker run -it -e RUST_LOG=debug synd-translator \ \
  --sequencing-rpc-url <YOUR_URL_HERE> \
  --settlement-rpc-url <YOUR_URL_HERE> \
  --port 8888 \
  ...
  etc.
  ...
```

Run `cargo make print-sample-command` to see all the options for the `synd-translator`.

## Setting up the development environment

You can use the Dev Container in your IDE to get started. Clone the repo and choose "Open in Dev Container" from the command palette of your editor.

The Dev Container will auto-install all needed tooling. This includes Go, Rust, Foundry, Optimism tools, and more.

After creating the Dev Container, several commands will be available, including:

- `op-up`: Start the Optimism devnet
- `op-down`: Stop the Optimism devnet
- `go-install`: Install all Go dependencies in the Dev Container
- `arb-up`: Start the Arbitrum devnet
- `arb-down`: Stop the Arbitrum devnet

If these aliases don't work immediately, try:

1. Open a new terminal in the Dev Container
2. Run `source ~/.bashrc` (or `source ~/.zshrc` if using zsh)
3. Alternatively, run `just --justfile /workspaces/metabased-rollup/.devcontainer/justfile create-aliases` followed by `source ~/.bashrc`

### Sharing git ssh credentials with the Dev Container

You can share your local git ssh credentials with the Dev Container via the ssh-add command.

On the host machine, run: `ssh-add ~/.ssh/id_ed25519` (or the path to your ssh key)

This will automatically make the key available in the Dev Container. To confirm that the key is available, on the Dev Container, run: `ssh-add -l`

Note that this will not persist across restarts of the host machine. To persist across restarts, you can add the following to your `~/.ssh/config` file:

```
Host *
    # This should go below any other content in this entry
    AddKeysToAgent yes
    IdentityFile ~/.ssh/id_ed25519
```

Adding ssh keys via the ssh-add command is not necessary in GitHub Codespace or GitHub Actions, as GitHub handles authentication for you.

### Running in GitHub Codespaces

If you would like, you can run the Dev Container in GitHub Codespaces. This is useful for long-running tasks or development in a browser away from your local machine. To launch a Codespace, navigate to the branch you want to use, click the "Code" button (where repo cloning typically is) and then click "Create codespace on \[branch-name]".

GitHub Codespaces are compatible with VS Code and JetBrains, but not Cursor. You don't need to handle ssh credentials in Codespaces, since they are automatically installed by GitHub.

### Lints
(April 2025)
Here is a terminal alias that will run our CI lint checks locally for ease of development. Note that these are mutating commands and will change your code. If you don't want that, add `--check` flags as necessary

```
alias rt='cargo +nightly fmt --all -- --unstable-features && \
cargo clippy --workspace --all-targets --all-features && \
cargo machete && \
taplo fmt "**/Cargo.toml"'
```
