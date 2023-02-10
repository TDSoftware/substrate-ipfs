
# TDS IPFS Core
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Scaffolding for other pallets to easily communicate with IPFS via extrinsics.

## Abstract

This pallet provides all core functionality for IPFS node communication and invoking functionality.
Such functionality can be used from other pallets by invoking IpfsCommand enums
which can be passed to the pallet using a deposit_event method.

You can find an example usage in `tds-ipfs/src/lib.rs`.

## Credits
- [TDSoftware](https://github.com/tdsoftware)
- [WunderbarNetwork](https://github.com/WunderbarNetwork/substrate)
- [rs-ipfs](https://github.com/rs-ipfs/substrate/)