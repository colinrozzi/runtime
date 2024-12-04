# Runtime Project Documentation

## Overview
The Runtime project is a WebAssembly (Wasm) component execution environment that maintains a hash-based chain of state transitions. It provides a framework for running and managing Wasm components while ensuring state integrity and verification through a hash chain.

## Project Structure
```
src/
├── main.rs      - Entry point and CLI handling
├── lib.rs       - Core runtime implementation
├── chain.rs     - Hash chain implementation
├── network.rs   - HTTP server and API endpoints
└── wasm.rs      - WebAssembly component management
```

## Quick Start
```bash
cargo run <wasm_file> <port>
```

## Documentation Structure
- `docs/src/` - Contains detailed documentation for each source file
- `docs/architecture.md` - High-level architecture and design decisions
- `docs/api.md` - API documentation and usage
- Individual component documentation can be found in the corresponding markdown files in `docs/src/`

For detailed documentation on specific components, please refer to:
- [Main Program Documentation](src/main.md)
- [Core Runtime Documentation](src/lib.md)
- [Hash Chain Documentation](src/chain.md)
- [Network Interface Documentation](src/network.md)
- [WebAssembly Integration Documentation](src/wasm.md)

## Core Features
1. WebAssembly component execution
2. State management with hash chain verification
3. HTTP API for message handling
4. Contract-based state and message verification

## Dependencies
See `Cargo.toml` for a complete list of dependencies.

Key dependencies:
- `wasmtime`: WebAssembly runtime
- `axum`: HTTP server framework
- `tokio`: Async runtime
- `serde`: Serialization
