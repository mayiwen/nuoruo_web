# Nuoruo Web Framework

A lightweight frontend framework built with Rust and WebAssembly.

## Getting Started

### Prerequisites

- Rust (latest stable)
- wasm-pack
- trunk

### Installation

```bash
# Install trunk
cargo install trunk

# Install wasm-pack (if not already installed)
cargo install wasm-pack

# Build the project
trunk build

# Serve locally
trunk serve
```

## Features

- Lightweight and fast
- Type-safe with Rust
- WebAssembly powered
- Component-based architecture

## Project Structure

```
nuoruo_web/
├── Cargo.toml
├── Trunk.toml
├── index.html
├── src/
│   ├── lib.rs
│   └── main.rs
└── dist/
```

## License

MIT