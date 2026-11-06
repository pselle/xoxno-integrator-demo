# xoxno-health-check

A demo integrator contract. It calls XOXNO lending's controller through
`stellar_registry::import_contract!(env, "xoxno/controller")`, with no hand-written
client and no hard-coded address. It's tested with `register_import!`, against both the
real controller Wasm and a native mock.

It currently resolves against a testnet dry-run registry, not the public one:

```bash
export STELLAR_NETWORK=testnet
export STELLAR_REGISTRY_CONTRACT_ID=CC26C2PEXKLYI2Z5ZX3WMMERJ6XZPF4D2C4VNSFW2EYYRKPRZFWOYUJ6
cargo test
stellar contract build
```

Deployed on testnet as `CBGCZFG35AXMKJ4MNHXHFHOC2NNJL26WFW6QTYDAYDP5FFRW4KB7WV7B`.
