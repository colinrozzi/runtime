# API Documentation

## HTTP API

### POST /
Process a message and update system state.

#### Request
- Method: `POST`
- Content-Type: `application/json`
- Body:
```json
{
    "data": <JSON Value>
}
```

#### Response
- Status: 200 OK
- Content-Type: `application/json`
- Body:
```json
{
    "hash": "<state_hash>",
    "state": <JSON Value>
}
```

#### Error Responses
- Status: 400 Bad Request
  - Invalid message format
  - Contract violation
  - State verification failure
- Status: 500 Internal Server Error
  - Runtime errors
  - WASM execution errors

## WebAssembly Interface

### Component Interface
Required implementation for WASM components.

#### `init() -> Value`
Initialize component state.
- Returns: Initial state as JSON value

#### `handle(msg: Value, state: Value) -> Value`
Process a message and update state.
- Parameters:
  - msg: Message data as JSON
  - state: Current state as JSON
- Returns: New state as JSON value

#### `message_contract(msg: Value, state: Value) -> bool`
Verify message validity.
- Parameters:
  - msg: Message to verify
  - state: Current state
- Returns: true if valid, false if invalid

#### `state_contract(state: Value) -> bool`
Verify state validity.
- Parameters:
  - state: State to verify
- Returns: true if valid, false if invalid

### Host Functions
Functions provided by the runtime to components.

#### `log(msg: &str)`
Log a message from the component.
- Parameters:
  - msg: Message to log

#### `send(actor_id: &str, msg: &Value)`
Send a message to another actor.
- Parameters:
  - actor_id: Target actor identifier
  - msg: Message to send

## Usage Examples

### Send Message
```bash
curl -X POST http://localhost:8080/ \
     -H "Content-Type: application/json" \
     -d '{
           "data": {
             "action": "update",
             "value": 42
           }
         }'
```

### Response
```json
{
    "hash": "7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069",
    "state": {
        "counter": 42,
        "last_update": "2024-12-04T10:00:00Z"
    }
}
```

## Best Practices

### Message Design
1. Use clear action identifiers
2. Include necessary data only
3. Validate before sending
4. Handle errors appropriately

### State Management
1. Keep states minimal
2. Validate all transitions
3. Maintain data integrity
4. Consider performance impact

### Error Handling
1. Check response status
2. Parse error messages
3. Implement retries when appropriate
4. Log failures for debugging

## Rate Limiting
- No explicit rate limiting currently implemented
- Consider application-level throttling
- Monitor system resources

## Security Notes
1. Local-only HTTP server
2. No authentication currently
3. Input validation required
4. Contract enforcement critical