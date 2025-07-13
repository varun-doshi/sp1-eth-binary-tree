# SP1-ETH-Binary_tree

This proves each leaf in the `eth-binary-tree` using SP1. <br/>
See implementation of [EIP-7864 eth-binary-tree](https://github.com/varun-doshi/eth-binary-tree)


## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://docs.succinct.xyz/docs/sp1/getting-started/install)

To run the program without generating a proof:

```sh
cd script
cargo run --release -- --execute
```

To generate an SP1 [core proof](https://docs.succinct.xyz/docs/sp1/generating-proofs/proof-types#core-default) for your program:

```sh
cd script
cargo run --release -- --prove
```
