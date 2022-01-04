# cota-registry-aggregator

The aggregator of [CoTA](https://github.com/nervina-labs/ckb-cota-scripts) registry service

### Quick Start

Update `database_url` in `aggregator.toml` with your mysql url

```shell
cargo build

cargo run

cargo test
```

### Usage

```shell
cargo build --release

RUST_LOG=info ./target/release/cota-registry-aggregator -s /tmp/cota-registry-aggregator-test
```

```shell
echo '{
    "id": 2,
    "jsonrpc": "2.0",
    "method": "register_cota_cells",
    "params": ["0xea28c98f38b4a57aa81756b167bb37fa42daf67edbc9863afb8172096ed301c2"]
}' \
| tr -d '\n' \
| curl -H 'content-type: application/json' -d @- \
http://localhost:3050
```