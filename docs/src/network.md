# Network Interface (`network.rs`)

## Purpose
The network module provides HTTP-based communication with the runtime system. It handles incoming messages, manages concurrent access to the runtime, and provides a REST API for state transitions.

## Implementation

### Core Components

#### SharedRuntime
```rust
struct SharedRuntime(Arc<Mutex<Runtime>>);
```
Provides thread-safe shared access to the runtime.

#### Message Format
```rust
struct Message {
    data: Value
}

struct Response {
    hash: String,
    state: Value
}
```

### Key Operations

#### Server Setup (`serve`)
1. Creates shared runtime instance
2. Sets up Axum router
3. Binds to specified address
4. Starts HTTP server

#### Message Handling
1. Receives JSON message
2. Acquires runtime lock
3. Processes message
4. Returns hash and new state

## Key Decisions

### 1. Concurrency Management
- Uses `Arc<Mutex<>>` for safe sharing
- Single runtime instance
- Serialized message processing

### 2. API Design
- Simple POST endpoint
- JSON request/response
- Clear error responses

### 3. Framework Choice
- Axum for modern async handling
- Built on Tokio
- Type-safe routing

### 4. Error Handling
- HTTP status codes
- Detailed error messages
- Clean error propagation

## Design Philosophy

### 1. Simplicity
- Single endpoint
- Clear message format
- Straightforward error handling

### 2. Safety
- Thread-safe runtime access
- Proper error handling
- Type-safe interfaces

### 3. Performance
- Async processing
- Efficient locking
- Minimal overhead

## Usage Examples

### Server Setup
```rust
serve(runtime, addr).await?;
```

### API Usage
```bash
# Send message
curl -X POST http://localhost:8080/ \
     -H "Content-Type: application/json" \
     -d '{"data": {"key": "value"}}'
```

## API Documentation

### POST /
Handles state transition messages.

#### Request
```json
{
    "data": <JSON Value>
}
```

#### Response
```json
{
    "hash": "<state_hash>",
    "state": <JSON Value>
}
```

#### Status Codes
- 200: Success
- 400: Invalid message/state
- 500: Internal error

## Error Cases
1. Invalid JSON
2. Message processing failures
3. Runtime errors
4. Network binding issues

## Future Considerations

### 1. API Enhancement
- Additional endpoints
- WebSocket support
- Streaming updates

### 2. Security
- Authentication
- Authorization
- Rate limiting

### 3. Monitoring
- Health checks
- Metrics
- Logging

### 4. Performance
- Connection pooling
- Request batching
- Caching