<h1>
  <img src="https://ipfs.io/ipfs/QmRcFsCvTgGrB52UGpp9P2bSDmnYNTAATdRf4NBj8SKf77/rust-ipfs-logo-256w.png" width="128" /><br />
  Rust IPFS
</h1>

> The Interplanetary File System (IPFS), implemented in Rust
## Table of Contents

- [Description](#description)
    - [Project Status](#project-status---alpha)
- [Install](#install)
    - [Dependencies](#dependencies)
    - [Rust IPFS](#install-rust-ipfs-itself)
- [Getting Started](#getting-started)
    - [Running the tests](#running-the-tests)
    - [Contributing](#contributing)
- [Roadmap](#roadmap)
    - [Completed](#completed-work)
    - [In progress](#work-in-progress)
    - [Still required](#work-still-required)
- [Maintainers](#maintainers)
- [Alternatives](#alternatives-and-other-cool-related-projects)
- [Contributors](#contributors)
- [License](#license)
- [Trademarks](#trademarks)

## Description

This repository is a fork of [rust-ipfs](https://github.com/rs-ipfs/rust-ipfs), which contains the crates for the IPFS core implementation which includes a blockstore, a libp2p integration which includes DHT content discovery and pubsub support Our goal is to leverage both the unique properties of Rust to create powerful, performant software that works even in resource-constrained environments, while also maximizing interoperability with the other "flavors" of IPFS, namely JavaScript and Go.

### Project Status - `Alpha`

This project is a WIP and everything is subject to change

For more information about IPFS see: https://docs.ipfs.io/introduction/overview/

## Install

Rust IPFS depends on `protoc`.

**Note: This will change in the future.**

### Dependencies

First, install the dependencies.

With apt:

```bash
$ apt-get install protobuf-compiler libssl-dev zlib1g-dev
```

With yum:

```bash
$ yum install protobuf-compiler libssl-dev zlib1g-dev
```

### Install `rust-ipfs` itself

The `rust-ipfs` binaries can be built from source. Our goal is to always be compatible with the **stable** release of Rust.

```bash
$ git clone https://github.com/dariusc93/rust-ipfs && cd rust-ipfs
$ cargo build --workspace
```

You will then find the binaries inside of the project root's `/target/debug` folder.

## Getting started

We recommend browsing the [examples](https://github.com/dariusc93/rust-ipfs/tree/libp2p-next/examples) and [tests](https://github.com/dariusc93/rust-ipfs/tree/libp2p-next/tests) in order to see how to use Rust-IPFS in different scenarios.

**Note: Test are a WIP**

### Running the tests


**For information on running test, please see the [archived readme](./archived/README.md). This may be outdated but this section will be updated in the future** 


### Contributing

See [the contributing docs](./CONTRIBUTING.md) for more info.

If you have any questions on the use of the library or other inquiries, you are welcome to submit an issue.

## Roadmap

### Completed Work

TBD

**For previous completed work, please see the [archived readme](./archived/README.md).**

## Maintainers

Rust IPFS was originally authored by @dvc94ch and was maintained by @koivunej, and @aphelionz, but now is maintained by @dariusc93. 

**For maintainers please see the [archived readme](./archived/README.md).**

## Alternatives and other cool, related projects

It’s been noted that the Rust-IPFS name and popularity may serve its organization from a "first-mover" perspective. However, alternatives with different philosophies do exist, and we believe that supporting a diverse IPFS community is important and will ultimately help produce the best solution possible.

- Parity's [`rust-libp2p`](https://github.com/libp2p/rust-libp2p), which does a lot the of heavy lifting here
- Iroh [`iroh`](https://github.com/n0-computer/iroh) - Another rust implementation of IPFS
- [`ipfs-embed`](https://github.com/ipfs-rust/ipfs-embed/) - An implementation based on [`sled`](https://github.com/ipfs-rust/ipfs-embed/)
- [`rust-ipfs-api`](https://github.com/ferristseng/rust-ipfs-api) - A Rust client for an existing IPFS HTTP API. Supports both hyper and actix.
- [`rust-ipld`](https://github.com/ipfs-rust/rust-ipld) - Basic rust ipld library supporting `dag-cbor`, `dag-json` and `dag-pb` formats.
- PolkaX's own [`rust-ipfs`](https://github.com/PolkaX/rust-ipfs)


If you know of another implementation or another cool project adjacent to these efforts, let us know!

## Contributors

**For previous/original contributors please see the [archived readme](./archived/README.md).**
### Code Contributors

This project exists thanks to all the people who previously contributed to [rust-ipfs](https://github.com/rs-ipfs/rust-ipfs). [[Contribute](CONTRIBUTING.md)].

<a href="https://github.com/rs-ipfs/rust-ipfs/graphs/contributors"><img src="https://opencollective.com/rs-ipfs/contributors.svg?width=890&button=false" /></a>

**For previous/original contributors, please see the [archived readme](./archived/README.md).**

## License

Dual licensed under MIT or Apache License (Version 2.0). See [LICENSE-MIT](./LICENSE-MIT) and [LICENSE-APACHE](./LICENSE-APACHE) for more details.

## Trademarks

The [Rust logo and wordmark](https://www.rust-lang.org/policies/media-guide) are trademarks owned and protected by the [Mozilla Foundation](https://mozilla.org). The Rust and Cargo logos (bitmap and vector) are owned by Mozilla and distributed under the terms of the [Creative Commons Attribution license (CC-BY)](https://creativecommons.org/licenses/by/4.0/).
