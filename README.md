# Description

Convert solidity ABI to Rust ABI using the [ethers](https://github.com/gakonst/ethers-rs/blob/master/examples/abigen.rs).

# Build

```
cargo build
```

# Run

```
./target/debug/abigen {target contract name} {source solidity abi path}
```

For example,
```
./target/debug/abigen ImageCell solidity_abi/ImageCell.abi.json
```

Then you will find the generated Rust ABI in the folder `rust_abi`.
