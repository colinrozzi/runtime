# Runtime

## Overview
The runtime is responsible for exposing an actor to the outside world and tracking its state changes. The goal is to build a chain of computations that can be replayed and verified by any other actor.
So, all data that makes up the state of the actor at any given point in time must be tracked in its chain.
There are three main components of the Actor. The network interface, the chain, and the wasm component.
The network interface will accept POST requests at /. The body of the POST request should be a JSON object. The body should be parsed and added to the hash chain, and then the wasm component should be called with the received data. The resulting state of the wasm component should be hashed into the hash chain.
The wasm component will expose two functions to advance the state of the actor, init to be called on setup and return the initial state, and handle to accept a message and advance the state.
The wasm component will also expose two contract functions, to ensure that state and messages are well-formed and valid.
The hash chain will be a collection of objects. Each object should point to its parent.
```json
{
    parent: "hash-of-parent"
    data: data-goes-here
}
```
Each object will be referenced by the hash of the stringified object. Objects should be stored in a hash-map, and the current HEAD of the chain should be stored in a separate variable. Adding a hash to the chain will consist of:
Creating the object out of the data submitted and the current HEAD.
Stringifiying and hashing the object.
Putting the object into the hash-map at its identifier.
Store the identifier as the current HEAD.


This document describes the actor runtime. The runtime is responsible for managing the lifecycle of a single actor.
The runtime will be started with a compiled wasm component and a network address.

The runtime will set up the wasm component with the host functions, and make sure the init, handle, and contract functions exist in the component. The WIT interface for a wasm component is below:
```wit
package ntwk:simple-actor;

interface actor {

    type state;
    type message;

    state-contract: func(state: state) -> bool;
    message-contract: func(msg: message, state: state) -> bool;

    // The core handler function
    handle: func(msg: message, state: state) -> state;

    // Optionally, an initialization function
    init: func() -> state;
}

interface runtime {
    log: func(msg: string) -> ();
    send: func(actor-id: string, msg: message) -> ();
}

world first-actor {
    import runtime;
    export actor;
}
```

The runtime will then set up the hash chain and make the first commit to it. The first commit should contain the hash of the contents of the wasm component:
{
    component-hash: #####
}

The runtime should then set up the webserver at the address given.
