# Graphql wasm extension

This is an exmaple of how to use graphql in an envoy wasm extension. 


## Prior art

- https://github.com/proxy-wasm/spec
- https://github.com/proxy-wasm/proxy-wasm-rust-sdk (specifically https://github.com/proxy-wasm/proxy-wasm-rust-sdk/tree/main/examples/http_body)
- https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/advanced/wasm.html


## Getting started

### Prerequisites 
- Have rust installed
```
rustup target add wasm32-wasip1`
```

### Running
```
cargo build --target wasm32-wasip1 --release && docker compose up

curl -X POST localhost:10000/graphql -H "Content-Type: application/json" -d '{hero{name}}'
```

