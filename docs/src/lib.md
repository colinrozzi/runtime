# Core Runtime (`lib.rs`)

## Purpose
The core runtime module coordinates all major components of the system, managing the relationship between the WebAssembly component, state management, and the hash chain. It serves as the central orchestrator for the entire runtime system.

## Implementation

### Core Structure
The `Runtime` struct contains three key components:
1. `chain: HashChain` - Manages state history and verification
2. `wasm: WasmComponent` - Handles WebAssembly execution
3. `current_state: Option<Value>` - Maintains current system state

### Key Operations

#### Initialization (`new`)
1. Creates new WebAssembly component
2. Initializes hash chain with component hash
3. Returns ready-to-use runtime instance

#### Starting (`start`)
1. Initializes WASM component
2. Gets and stores initial state
3. Adds initial state to hash chain
4. Starts network server

#### Message Handling (`handle_message`)
1. Retrieves current state
2. Processes message through WASM component
3. Updates hash chain with new state
4. Updates current state
5. Returns hash and new state

## Key Decisions

### 1. State Management
- Uses `Option<Value>` for current state
  - Ensures explicit initialization check
  - Provides clear error cases
- JSON-based state representation
  - Flexible data structure
  - Easy serialization/deserialization

### 2. Component Coordination
- Clear separation of concerns:
  - WASM handling
  - State management
  - Chain verification
- Minimal coupling between components

### 3. Error Handling
- Uses `anyhow::Result` throughout
- Propagates errors from sub-components
- Maintains context for debugging

### 4. State Flow
- Unidirectional state updates:
  1. Message received
  2. WASM processes
  3. State updated
  4. Chain updated
- Ensures consistency and traceability

## Design Philosophy

### 1. Separation of Concerns
- Each component has a specific responsibility
- Clear interfaces between components
- Minimal cross-component dependencies

### 2. State Integrity
- All state changes tracked in chain
- Current state always matches chain head
- Verifiable state history

### 3. Component Integration
- Loose coupling between components
- Easy to modify or replace components
- Clear dependency flow

## Usage Examples

### Creating New Runtime
```rust
let runtime = Runtime::new("path/to/wasm")?;
```

### Handling Messages
```rust
let (hash, new_state) = runtime.handle_message(message).await?;
```

## Error Handling
1. WASM initialization failures
2. State management errors
3. Chain verification issues
4. Network communication errors

## Future Considerations
1. State persistence
2. Multiple WASM component support
3. Enhanced state verification
4. Distributed runtime support