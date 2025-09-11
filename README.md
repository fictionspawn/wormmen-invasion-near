# wormmen-invasion-near

cargo-near-new-project-description

## How to Build Locally?

Install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near build
```

## How to Test Locally?

```bash
cargo test
```

## How to Deploy?

Deployment is automated with GitHub Actions CI/CD pipeline.
To deploy manually, install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near deploy build-reproducible-wasm <account-id>
```

## Useful Links

- [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
- [near CLI](https://near.cli.rs) - Interact with NEAR blockchain from command line
- [NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [NEAR Documentation](https://docs.near.org)
- [NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)
- [NEAR Discord](https://near.chat)
- [NEAR Telegram Developers Community Group](https://t.me/neardev)
- NEAR DevHub: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)

A game I'm writing to learn and practice rust smart contract development on NEAR Protocol.

This is proof of concept for a coordinate based smart contract platform game.

WORK IN PROGRESS

The current state can be played by installing near-cli-rs, and make a contract call to wormmen4.testnet: write near in the terminal choose contract, then contract calls, and choose start_game, json arguments, {}. 

Game play is rather slow still, but things will get better.

TODO: Create user owned storage. 
TODO: Introduce NFTs.
TODO: Introduce FTs.

