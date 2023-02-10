# Substrate &middot; [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE) [![GitLab Status](https://gitlab.parity.io/parity/substrate/badges/master/pipeline.svg)](https://gitlab.parity.io/parity/substrate/pipelines) [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](docs/CONTRIBUTING.adoc) [![Stack Exchange](https://img.shields.io/badge/Substrate-Community%20&%20Support-24CC85?logo=stackexchange)](https://substrate.stackexchange.com/)
<p align="center">
  <img src="/docs/media/sub.gif">
</p>

Substrate is a next-generation framework for blockchain innovation 🚀.

## Getting Started

Head to [docs.substrate.io](https://docs.substrate.io) and follow the [installation](https://docs.substrate.io/install/) instructions.
Then try out one of the [tutorials](https://docs.substrate.io/tutorials/).

## Community & Support

Join the highly active and supportive community on the [Substrate Stack Exchange](https://substrate.stackexchange.com/) to ask questions about use and problems you run into using this software.
Please do report bugs and [issues here](https://github.com/paritytech/substrate/issues) for anything you suspect requires action in the source. 

## Contributions & Code of Conduct

Please follow the contributions guidelines as outlined in [`docs/CONTRIBUTING.adoc`](docs/CONTRIBUTING.adoc).
In all communications and contributions, this project follows the [Contributor Covenant Code of Conduct](docs/CODE_OF_CONDUCT.md).

## Security

The security policy and procedures can be found in [`docs/SECURITY.md`](docs/SECURITY.md).

## IPFS

This version of Substrate contains a working IPFS node and IPFS integration.
Just install and start this implementation like any other substrate blockchain. 
```bash
# E.g. run the blockchain in development mode
cargo run -- --dev
```

Once started, you should see the connecting peer id in the console:
```bash
net: starting with peer id 12D3KooWRhtRwRq6Jo9cTTNjgC5MNRVyb33jXwkX8r3hpENqAmP8
IPFS: Node started with PeerId 12D3KooWRhtRwRq6Jo9cTTNjgC5MNRVyb33jXwkX8r3hpENqAmP8 and address ["/ip4/127.0.0.1/tcp/49595/p2p/12D3KooWRhtRwRq6Jo9cTTNjgC5MNRVyb33jXwkX8r3hpENqAmP8"]  
```
If you are interested, feel free to get in contact:
[tdsoftware.de](https://tdsoftware.de)

For details about the implementation, check the following project folders:
```
/client/offchain
/frame/tds-ipfs-core
/frame/tds-ipfs
/primitives/runtime/src/offchain
```

## License

- Substrate Primitives (`sp-*`), Frame (`frame-*`) and the pallets (`pallets-*`), binaries (`/bin`) and all other utilities are licensed under [Apache 2.0](LICENSE-APACHE2).
- Substrate Client (`/client/*` / `sc-*`) is licensed under [GPL v3.0 with a classpath linking exception](LICENSE-GPL3).

The reason for the split-licensing is to ensure that for the vast majority of teams using Substrate to create feature-chains, then all changes can be made entirely in Apache2-licensed code, allowing teams full freedom over what and how they release and giving licensing clarity to commercial teams.

In the interests of the community, we require any deeper improvements made to Substrate's core logic (e.g. Substrate's internal consensus, crypto or database code) to be contributed back so everyone can benefit.

