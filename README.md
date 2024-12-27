# Template

## Set up working environment

- Install `Rust`:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Install `rust-src` and `wasm32-unknown-unknown`:
```sh
rustup component add rust-src
rustup target add wasm32-unknown-unknown
```

- Install `omni-node`:
```sh
cargo install --force --git https://github.com/kianenigma/pba-omni-node.git
```

- Build:

```sh
cargo build --release
```

## Start node

### Start omni node

```sh
pba-omni-node --runtime ./target/release/wbuild/minimal-template-runtime/minimal_template_runtime.wasm --tmp --offchain-worker always
```

### Start chain

- Install `chain-spec-builder`:
```sh
cargo install staging-chain-spec-builder
```

- Generate chain spec:
```sh
chain-spec-builder create --chain-name SubstrateTemplate -r ./target/release/wbuild/minimal-template-runtime/minimal_template_runtime.wasm default
```

- Update chain spec:
```json
// on top
"chainType": "Development",
"properties": {
    "tokenDecimals": 1,
    "tokenSymbol": "D"
},

```
```json
// under `balances.balances`
["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 100000],
["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", 100000],
["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", 100000],
["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy", 100000],
["5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw", 100000],
["5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL", 100000],
["5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY", 100000],
["5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc", 100000],
["5Ck5SLSHYac6WFt5UZRSsdJjwmpSZq85fd5TRNAdZQVzEAPT", 100000],
["5HKPmK9GYtE1PSLsS1qiYU9xQ9Si1NcEhdeCq9sw5bqu4ns8", 100000],
["5FCfAonRZgTFrTd9HREEyeJjDpT397KMzizE6T3DvebLFE7n", 100000],
["5CRmqmsiNFExV6VbdmPJViVxrWmkaXXvBrSX8oqBT8R9vmWk", 100000],
["5Fxune7f71ZbpP2FoY3mhYcmM596Erhv1gRue4nsPwkxMR4n", 100000],
["5CUjxa4wVKMj3FqKdqAUf7zcEMr4MYAjXeWmUf44B41neLmJ", 100000]
```

- Start chain:
```sh
pba-omni-node --chain ./chain_spec.json --tmp --offchain-worker always
```
