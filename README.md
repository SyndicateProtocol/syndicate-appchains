# Metabased Rollup Monorepo

The monorepo for the Metabased rollup stack. Containers the sequencer, smart contracts, RPC node, dev environments, and more!

## Setting up the development environment

You can use the Dev Container in your IDE to get started. Clone the repo and choose "Open in Dev Container" from the command palette of your editor.

The Dev Container will auto-install all needed tooling. This includes Go, Rust, Foundry, Optimism tools, and more.

After creating the Dev Container, the following commands will be available:

- `op-up`: Start the Optimism devnet
- `op-down`: Stop the Optimism devnet
- `go-install`: Install all Go dependencies in the Dev Container

### Sharing git ssh credentials with the Dev Container

You can share your local git ssh credentials with the Dev Container via the ssh-add command.

On the host machine, run: `ssh-add ~/.ssh/id_ed25519` (or the path to your ssh key)

This will automatically make the key available in the Dev Container. To confirm that the key is available, on the Dev Container, run: `ssh-add -l`

### Running in GitHub Codespaces

If you would like, you can run the Dev Container in GitHub Codespaces. This is useful for long-running tasks or development in a browser away from your local machine. To launch a Codespace, navigate to the branch you want to use, click the "Code" button (where repo cloning typically is) and then click "Create codespace on \[branch-name]".
