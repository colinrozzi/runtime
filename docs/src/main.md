# Main Program (`main.rs`)

## Purpose
The main program serves as the entry point for the runtime system. It handles command-line argument parsing and initializes the runtime with the specified WebAssembly component.

## Implementation

### Command Line Interface
The program expects exactly two command-line arguments:
1. Path to the WebAssembly component file
2. Port number for the HTTP server

### Core Components
1. **Argument Parsing**
   - Uses standard `std::env::args()`
   - Validates number of arguments
   - Parses port number from string to `u16`

2. **Network Setup**
   - Creates a `SocketAddr` bound to localhost (127.0.0.1)
   - Uses the provided port number

3. **Runtime Initialization**
   - Creates new Runtime instance with provided WASM file
   - Starts the runtime system asynchronously

## Key Decisions

### 1. Error Handling
- Uses `anyhow::Result` for flexible error handling
- Provides clear error messages for incorrect usage
- Early exit with error code on invalid arguments

### 2. Async Runtime
- Uses `tokio` runtime with `#[tokio::main]` attribute
- Enables asynchronous operation for network handling
- Full tokio feature set enabled for maximum flexibility

### 3. Network Binding
- Binds only to localhost for security
- Allows port configuration via command line
- Simple network setup for easy deployment

## Usage Example
```bash
# Run with WASM file and port 8080
cargo run /path/to/component.wasm 8080
```

## Error Cases
1. Incorrect number of arguments
   - Displays usage message
   - Exits with status code 1

2. Invalid port number
   - Returns error through anyhow
   - Includes parse error details

3. Runtime initialization failure
   - Propagates error from runtime
   - Includes context from failure

## Design Philosophy
1. **Simplicity**: Minimal command-line interface with clear purpose
2. **Safety**: Error handling for all failure cases
3. **Flexibility**: Configurable port and WASM file location
4. **Security**: Local-only network binding