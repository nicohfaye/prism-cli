# Prism-CLI

Terminal UI for monitoring Kubernetes clusters over SSH.

## Setup

1. Copy `cli/config.example.toml` to `cli/config.toml`
2. Configure your SSH and Kubernetes settings

## Usage

```bash
cd cli
cargo run
```

Run in demo mode (no connection required):

```bash
cargo run -- --demo
```

## Build

```bash
cd cli
cargo build --release
```
