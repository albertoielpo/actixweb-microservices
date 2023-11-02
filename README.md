# Actixweb Microservices
The idea is to create a Rust project, to experiment the actixweb library.
The cargo workspace contains: micro, websocket and the common lib

## Micro
Rust actixweb project with working:
- REST GET example
- SSE GET event stream
- CORS permissive wrap
- Error Handler with custom error body
- Route Panic management
- Log configured via env variable

## Websocket
Rust actixweb websocket actors implementation with working:
- JSON payload communication
- Log configured via env variable

## Common Lib
- Common behavior

## Docker
Three stage build using cargo chef to obtain a lightweight image starting from scratch and to reduce build time if dependecies does not change.

## TODO
- actixweb guard
- jwt token usage
- redis integration