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
- Jwt token login with fixed credential (auth_routes.http)
- Admin route protected with jwt auth
- Https connection
  
## Websocket
Rust actixweb websocket actors implementation with working:
- JSON payload communication
- Log configured via env variable

## Common Lib
- Common behavior
- Redis provider async
- Redis provider sync

## Docker
Three stage build using cargo chef to obtain a lightweight image starting from scratch and to reduce build time if dependecies does not change.

# Get started

## Build
```
cargo build
# or via docker
docker compose build [micro-service | websocket-service]
```

## Run
```
cargo run --bin micro
cargo run --bin websocket
# or via docker
docker compose up -d [micro-service | websocket-service | redis]
```
## Debug via VSCODE
Requires /target/debug folder to be created and sync 
```
$ cargo build
Finished dev [unoptimized + debuginfo] target(s) in 0.06s
```
then launch the configuration inside .vscode/launch.json. Output in debug console should be something similar to
```
Launching: ~/dev/git/rust/actixweb-microservices/target/debug/micro
Launched process 31395
```
## Examples
### GET
To test the API routes inside the "micro" project see the folder micro/test/http/rate_routes.http. 
```
# GET request
curl --request GET --url http://localhost:3000/rate --header 'accept: application/json'
{"rate":"1.866"}
```
### SSE
To test SSE stream use the browser http://localhost:3000/sse, your favourite tool or directly open a <code>new EventSource(`http://localhost:3000/sse`)</code>

### Websocket
- To test websocket API open a connection via <code>new WebSocket(`http://localhost:3001`)</code>.
- To start the stream send the json payload <code>{ event: "cmd", data: { type: "start" }}</code>
- To stop the stream send the json payload  <code>{ event: "cmd", data: { type: "stop" }} </code>


## Docs
- Actix Web: https://actix.rs
- Learn rust: https://www.rust-lang.org/learn
