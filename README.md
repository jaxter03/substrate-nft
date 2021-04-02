# Substrate based Blockchain for NFTs
## About the Project
In this Project, Blockchain is designed to handle NFTs using Substrate Framework. It implements a protocol defined by ERC721.org to allow unique tokens to be managed, owned, and traded. It also has pallets provided by Chainsafe's Chainbridge to transfer an asset from this Blockchain to Ethereum Blockchain and vice-versa.

## Tech
This Project uses a number of open-source projects to work properly:

- [Rust](https://www.rust-lang.org/) - Rust is blazingly fast and memory-efficient. It can power performance-critical services.
- [Substrate](https://github.com/paritytech/substrate) - Substrate is a next-generation framework for blockchain innovation.
- [rocket-rs](https://rocket.rs/) - Rocket is a web framework for Rust that makes it simple to write fast, secure web applications.
- [Chainsafe's Chainbridge](https://github.com/ChainSafe/ChainBridge) - ChainBridge is an extensible cross-chain communication protocol. It currently supports bridging between EVM and Substrate based chains.
## Getting Started

This project contains some configuration files to help get started :hammer_and_wrench:

### Rust Setup

Follow the [Rust setup instructions](./doc/rust-setup.md) before using the included Makefile to
build the Node Template.

### Makefile

This project uses a [Makefile](Makefile) to document helpful commands and make it easier to execute
them. Get started by running these [`make`](https://www.gnu.org/software/make/manual/make.html)
targets:

1. `make init` - Run the [init script](scripts/init.sh) to configure the Rust toolchain for
   [WebAssembly compilation](https://substrate.dev/docs/en/knowledgebase/getting-started/#webassembly-compilation).
1. `make run` - Build and launch this project in development mode.

The init script and Makefile both specify the version of the
[Rust nightly compiler](https://substrate.dev/docs/en/knowledgebase/getting-started/#rust-nightly-toolchain)
that this project depends on.

### Build

The `make run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
make build
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/node-nft-pallet -h
```

## Run

The `make run` command will launch a temporary node and its state will be discarded after you
terminate the process. After the project has been built, there are other ways to launch the node.

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/node-nft-pallet --dev
```

Purge the development chain's state:

```bash
./target/release/node-nft-pallet purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/node-nft-pallet -lruntime=debug --dev
```
Follow the [Prerequisites](https://chainbridge.chainsafe.io/local/) provided by Chainsafe to install Chainbridge.


## Roadmap
- [x] Setting up Blockchain with PoA (Aura) Consensus.
- [x] Install Pallet to handle NFTs.
- [ ] Install Pallet to handle Multiple types of Bidding.
- [ ] Create APIs to interact with Blockchain and Database.
- [ ] Frontend Integration.

## License

Distributed under the MIT License. See `LICENSE` for more information


## Contact

[@ksr30](https://www.linkedin.com/in/krishna-singh-b37671170/) - krishna.singh9926@gmail.com