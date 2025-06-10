# Dogecoin Electrs

An Electrum server for Dogecoin, written in Rust. Based on [romanz/electrs](https://github.com/romanz/electrs) and [Esplora](https://github.com/Blockstream/esplora).

Provides a backend for Electrum-DOGE wallets and a block explorer API.

API documentation [is available here](https://github.com/blockstream/esplora/blob/master/API.md).

Documentation for the database schema and indexing process [is available here](doc/schema.md).

### Installing & indexing

First, install Rust and Dogecoin Core. Then, you can build and run `dogecoin-electrs`:

```bash
# Clone the repository
$ git clone https://github.com/hieusats/dogecoin-electrs.git
$ cd dogecoin-electrs

# Build the project
$ cargo build --release

# Run for Dogecoin mainnet
$ ./target/release/electrs -vvvv --network mainnet --daemon-dir ~/.dogecoin --daemon-rpc-addr 127.0.0.1:22555

# Run for Dogecoin testnet
$ ./target/release/electrs -vvvv --network testnet --daemon-dir ~/.dogecoin/testnet3 --daemon-rpc-addr 127.0.0.1:44555
```

The server needs to be connected to a running `dogecoind` instance. Make sure to configure your `dogecoin.conf` with `rpcuser` and `rpcpassword`, or a RPC cookie file.

The indexes can take up a significant amount of disk space and may take several hours to build initially.

To deploy with Docker, follow the [instructions here](https://github.com/Blockstream/esplora#how-to-build-the-docker-image) (Note: you will need to adapt the instructions for this Dogecoin version).

### Light mode

For personal or low-volume use, you may set `--lightmode` to reduce disk storage requirements
by roughly 50% at the cost of slower and more expensive lookups.

With this option set, raw transactions and metadata associated with blocks will not be kept in the database,
but instead queried from `dogecoind` on demand.

### Notable changes from Electrs:

- HTTP REST API in addition to the Electrum JSON-RPC protocol, with extended transaction information
  (previous outputs, spending transactions, script asm and more).

- Extended indexes and database storage for improved performance under high load:

  - A full transaction store mapping txids to raw transactions is kept in the database under the prefix `t`.
  - An index of all spendable transaction outputs is kept under the prefix `O`.
  - An index of all addresses (encoded as string) is kept under the prefix `a` to enable by-prefix address search.
  - A map of blockhash to txids is kept in the database under the prefix `X`.
  - Block stats metadata (number of transactions, size and weight) is kept in the database under the prefix `M`.

  With these new indexes, `dogecoind` is no longer queried to serve user requests and is only polled
  periodically for new blocks and for syncing the mempool.

### CLI options

A summary of important configuration options:

- `--daemon-dir <dir>` - Dogecoin data directory (default: `~/.bitcoin`). It's recommended to set this to `~/.dogecoin`.
- `--daemon-rpc-addr <addr:port>` - Dogecoin daemon JSONRPC address to connect to (e.g. `127.0.0.1:22555` for mainnet).
- `--cookie <user:pass>` - JSONRPC authentication cookie (`USER:PASSWORD`).
- `--auth <user:pass>` - JSONRPC authentication credentials (`USER:PASSWORD`). Overrides `--cookie`.
- `--network <network>` - Select network type (`mainnet`, `testnet`, `regtest`).
- `--http-addr <addr:port>` - HTTP server address/port to listen on (default: `127.0.0.1:3000`).
- `--lightmode` - enable light mode (see above).
- `--address-search` - enables the by-prefix address search index.
- `--index-unspendables` - enables indexing of provably unspendable outputs.
- `--utxos-limit <num>` - maximum number of utxos to return per address.
- `--electrum-txs-limit <num>` - maximum number of txs to return per address in the electrum server (does not apply for the http api).
- `--electrum-banner <text>` - welcome banner text for electrum server.

Additional options with the `electrum-discovery` feature:
- `--electrum-hosts <json>` - a json map of the public hosts where the electrum server is reachable, in the [`server.features` format](https://electrumx.readthedocs.io/en/latest/protocol-methods.html#server.features).
- `--electrum-announce` - announce the electrum server on the electrum p2p server discovery network.

See `./target/release/electrs --help` for the full list of options.

## License

MIT
