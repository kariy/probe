# `probe`

A Starknet toolkit.

## Installation

```
cargo install --git https://github.com/kariy/probe-chan --locked --force
```

## Usage

```console
probe
A Starknet toolkit.

Usage: probe <COMMAND>

Commands:
  --to-hex             Convert decimal felt to hexadecimal. [aliases: th]
  --to-dec             Convert hexadecimal felt to decimal. [aliases: td]
  --max-felt           Get the maximum felt value. [aliases: mxf]
  --max-sfelt          Get the maximum signed felt value. [aliases: mxsf]
  --min-sfelt          Get the minimum signed felt value. [aliases: mnsf]
  --from-ascii         Convert from ASCII to Cairo short string. [aliases: fa]
  --to-ascii           Convert Cairo short string to its ASCII format. [aliases: ta]
  --split-u256         Split a uint256 into its low and high components. [aliases: su]
  account              Account management utilities [aliases: acc]
  age                  Get the timestamp of a block.
  balance              Get the ETH balance of an address. [aliases: bal]
  block                Get information about a block. [aliases: b]
  block-number         Get the latest block number. [aliases: bn]
  call                 Call a StarkNet function without creating a transaction.
  chain-id             Get the StarkNet chain ID. [aliases: ci]
  class                Get the contract class definition in the given block associated with the given hash [aliases: cl]
  code                 Get the contract class definition in the given block at the given address [aliases: cd]
  compute-address      Compute the contract address from the given information [aliases: ca]
  contract-class       Get the contract class hash in the given block for the contract deployed at the given address [aliases: cc]
  class-hash           Compute the hash of a contract class. [aliases: ch]
  compiled-class-hash  Compute the compiled class hash of a Sierra contract class. [aliases: cch]
  declare              Declare a new contract class. [aliases: dec]
  deploy               Deploy a new contract. [aliases: dep]
  ecdsa                Perform ECDSA operations over the STARK-friendly elliptic curve. [aliases: ec]
  events               Returns all events matching the given filter [aliases: ev]
  index                Compute the address of a storage variable. [aliases: idx]
  invoke               Submit a new transaction to be added to the chain. [aliases: inv]
  keccak               Hash abritrary data using StarkNet keccak. [aliases: kck]
  legacy-declare       Declare a new legacy contract class. [aliases: ldec]
  nonce                Get the latest nonce associated with the address. [aliases: n1]
  pedersen             Calculate the Pedersen hash on two field elements. [aliases: ped]
  rpc                  Perform a raw JSON-RPC request.
  completions          Generate command completion script for a specific shell. [aliases: com]
  state-update         Get the information about the result of executing the requested block
  storage              Get the value of a contract's storage at the given index [aliases: str]
  syncing              Get the synchronization status of the StarkNet node [aliases: sync]
  tx                   Get information about a transaction.
  tx-count             Get the number of transactions in a block. [aliases: txc]
  tx-pending           Get the transactions in the transaction pool, recognized by the sequencer. [aliases: txp]
  tx-status            Get the status of a transaction. [aliases: txs]
  receipt              Get the receipt of a transaction. [aliases: rct]
  help                 Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
