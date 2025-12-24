# MU/TH/UR Terminal System

A Rust-powered terminal experience for three subsystems:

- **ELIZA**: a conversational assistant that mirrors classic ELIZA patterns while keeping short-term memory of the last message.
- **MAP**: a simple map visualizer that loads cities and connections from `data/california_map.json`, lets you view the graph, compute routes, add pings, and upload new maps at runtime.
- **TASKS**: a lightweight menu for listing or adding tasks (stubs you can extend).

## Running

```bash
cargo run
```

At the prompt, enter `ELIZA`, `MAP`, `TASKS`, or `Quit`.

### Linux quick start

```bash
# Install Rust (on Debian/Ubuntu; see rustup.rs for other distros)
sudo apt update && sudo apt install -y build-essential pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Run the app
cargo run
```

## Testing

```bash
cargo test
```

## Map data

The default map lives at `data/california_map.json`. Use the MAP module's `Upload` command to point to another compatible JSON file shaped like:

```json
{
  "cities": {
    "City Name": { "name": "City Name", "connections": ["Neighbor"] }
  }
}
```

## Project layout

- `src/`: Rust sources for the CLI entrypoint and ELIZA, MAP, and TASKS modules.
- `data/`: sample map JSON consumed by the MAP module.
- Legacy scaffold files from earlier drafts have been removed; the code under `src/` and `data/` now represents the maintained implementation.
