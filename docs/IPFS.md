# IPFS Documentation

## Prerequisite
The IPFS functionality of the Substrate framework is encapsulated within the TDS-IPFS-Core as well as the TDS-IPFS pallets. Therefore, make sure, that you have a branch of the framework containing both pallets. They are located in the "frame" folder within the Substrate project structure. Furthermore, the IPFS implementation depends on offchain indexing. Consequently, make sure that the offchain storage is enabled when starting the substrate node. To do so, add "--enable-offchain-indexing=true" as run parameter, like:
```bash
cargo run --bin substrate -- --dev --ws-external --rpc-cors all --tmp --enable-offchain-indexing=true
```

## Tests

To run the in-build unit tests of the IPFS integration, just open a terminal and run:
```bash
cargo test --package pallet-tds-ipfs
```
and
```bash
cargo test --package pallet-tds-ipfs-core
```

## Documentation
The IPFS implementation is done with two pallets, called TDS-IPFS and TDS-IPFS-Core. The core pallet, as its name suggests, contains the basic IPFS functionality such as uploading bytes to the IPFS network. The core pallet does not offer any extrinsic or remote procedure calls. It is designed that way, so that other pallets are able to make use or hook into the pallets core IPFS functionality.

Furthermore, the TDS-IPFS pallet offers a higher level abstraction of the IPFS services. It contains extrinsics to upload data as well as remote procedure calls to retrieve the public ipfs gateway URL of a file.

The IPFS node itself is integrated within the kernel of the Substrate framework. It is notable to mention, that the IPFS node API is only available within the offchain worker context and cannot directly be accessed on chain or within remote procedure calls.

More information about pallets and extrinsics of the Substrate framework can be found here: https://docs.substrate.io/tutorials/work-with-pallets/

### EXTRINSICS

#### AddBytes
The extrinsic `AddBytes` of the IPFS pallet sends the given bytes to the IPFS node and returns the referring contend identifier ("CID"). This content identifier can be used to query the file from the public IPFS gateway (https://ipfs.io).
The resulting file url is fabricated using the pattern `https://ipfs.io/ipfs/{{CID}}`, like
```diff
https://ipfs.io/ipfs/QmQqzMTavQgT4f4T5v6PWBp7XNKtoPmC9jvn12WPT3gkSE
```
where `QmQqzMTavQgT4f4T5v6PWBp7XNKtoPmC9jvn12WPT3gkSE` is the CID.
You can find more about IPFS public gateway and CID here: https://docs.ipfs.tech/concepts/ipfs-gateway/`

In addition, it is possible to specify the CID version (0 or 1) and add some meta data of the file. The meta data is set up as a byte array/string, therefore any kind of data can be passed. For example you want to upload a document, the meta data could be the consignee id.

The CID and the meta data will be stored onchain. As a result, it can be queried and accessed within the blockchain network.

**Important**

There are limitations in the usage of the extrinsic.
The maximum file size is limited to the maximum block size available, so use only small files.
In addition larger data sets could be expensive in terms of balances for the user.
You can configure this by accessing the `frame/tds-ipfs/src/lib` folder within the substrate project, open `lib.rs` and change the
`#[pallet::weight]` setting above the add_bytes extrinsic function.


### Remote Procedure Calls

#### ipfs_getFileURLForCID

The `ipfs_getFileURLForCID` RPC returns the IPFS gateway url, if the file had been uploaded using the `AddBytes` extrinsic. The request usually looks like this:
```bash
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "ipfs_getFileURLForCID",
  "params": [
    "QmQqzMTavQgT4f4T5v6PWBp7XNKtoPmC9jvn12WPT3gkSE"
  ]
}
```

A positive response containing the url has the following format:
```bash
{
  "jsonrpc": "2.0",
  "result": "https://ipfs.io/ipfs/QmQqzMTavQgT4f4T5v6PWBp7XNKtoPmC9jvn12WPT3gkSE",
  "id": 1
}
```

Where else a negative response contains only an empty string in the result property
```bash
{
  "jsonrpc": "2.0",
  "result": "",
  "id": 1
}
```
More informations about how remote procedure calls work in Substrate can be found here: https://docs.substrate.io/build/remote-procedure-calls/


#### ipfs_getFileURLForMetaData
The `ipfs_getFileURLForMetaData` RPC returns IPFS gateway url of a file for its meta data, which where passed when uploading it via `AddBytes` extrinsic. The request usually looks like this:
```bash
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "ipfs_getFileURLForMetaData",
  "params": [
    "My favourite strawberry image"
  ]
}
```

A positive response containing the url has the following format:
```bash
{
  "jsonrpc": "2.0",
  "result": "https://ipfs.io/ipfs/QmQqzMTavQgT4f4T5v6PWBp7XNKtoPmC9jvn12WPT3gkSE",
  "id": 1
}
```

Where else a negative response contains only an empty string in the result property
```bash
{
  "jsonrpc": "2.0",
  "result": "",
  "id": 1
}
```
More informations about how remote procedure calls work in Substrate can be found here: https://docs.substrate.io/build/remote-procedure-calls/
