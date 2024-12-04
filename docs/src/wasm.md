# WebAssembly Integration (`wasm.rs`)

## Purpose
The WebAssembly module manages the lifecycle and execution of WebAssembly components. It provides the interface between the runtime system and WebAssembly components, handling initialization, message processing, and contract verification.

## Implementation

### Core Components

#### RuntimeImpl
Implements host functions provided to WebAssembly:
- Logging functionality (`log`)
- Message sending capability (`send`)

#### WasmComponent
Manages WebAssembly instance:
- Store management
- Component instantiation
- Interface implementation

### Key Operations

#### Component Creation
1. Reads WASM file
2. Generates component hash
3. Sets up Wasmtime engine
4. Creates component instance

#### Message Processing
1. Verifies message against contract
2. Verifies state against contract
3. Processes message
4. Returns new state

## Key Decisions

### 1. Interface Design
- WIT-based interface definition
- Clear contract verification
- Simple message handling

### 2. Component Model
- Uses Wasmtime component model
- Strong typing through WIT
- Clear host/guest separation

### 3. State Management
- JSON-based state
- Contract verification
- Explicit initialization

### 4. Security
- Component hashing
- Contract enforcement
- Sandboxed execution

## Design Philosophy

### 1. Safety
- Strong contract checking
- Explicit state verification
- Controlled environment

### 2. Flexibility
- JSON-based messaging
- Extensible interface
- Configurable runtime

### 3. Reliability
- Clear error handling
- Consistent state management
- Verified transitions

## Component Interface

### Required Functions
1. `init() -> Value`
   - Initialize component state
   - Returns initial state

2. `handle(msg: Value, state: Value) -> Value`
   - Process incoming message
   - Update state
   - Return new state

3. `message_contract(msg: Value, state: Value) -> bool`
   - Verify message validity
   - Check against current state
   - Return verification result

4. `state_contract(state: Value) -> bool`
   - Verify state validity
   - Ensure state invariants
   - Return verification result

### Host Functions
1. `log(msg: &str)`
   - Log message from component
   - Debug and monitoring support

2. `send(actor_id: &str, msg: &Value)`
   - Send message to another actor
   - Future cross-component communication

## Usage Examples

### Component Creation
```rust
let (component, hash) = WasmComponent::new("path/to/wasm")?;
```

### Message Handling
```rust
let new_state = component.handle(message, current_state)?;
```

### Contract Verification
```rust
if !component.message_contract(&message, &state)? {
    anyhow::bail!("Invalid message");
}
```

## Error Cases
1. WASM file reading failures
2. Component instantiation errors
3. Contract verification failures
4. Message handling errors
5. State verification failures

## Future Considerations

### 1. Performance
- Caching mechanisms
- Optimized contract checking
- State serialization improvements

### 2. Features
- Multi-component communication
- Enhanced contract system
- State rollback capability

### 3. Security
- Enhanced sandboxing
- Resource limits
- Stronger verification

### 4. Tooling
- Contract generation
- Testing utilities
- Development tools

## Important Notes
1. Components must be built with component model support
2. WIT interface must match exactly
3. JSON serialization must be consistent
4. Contract violations are fatal errors