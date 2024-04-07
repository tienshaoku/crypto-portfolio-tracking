# Crypto Portfolio Tracking

I don't like uploading all my addresses to a website and trust it to track my balances 🫤

Thus, this is an easy command line tool for you to just fill in addresses and rpcs and track your crypto balances on your own end 👌

## Usage

1. Rename `src/constant_example.rs` to `src/constant.rs`. Once this is done, the code can be run!

2. `cargo run`

3. Fill in your addresses and rpcs in `constant.rs`; currently, only Ethereum and Optimism are supported, but more networks can be added by how `ETH_RPC` & `OP_RPC` are inserted in `main.rs`

## Snippet

![](snippet.png)

## Built with

- Rust
- ethers-rs
- ❤️
