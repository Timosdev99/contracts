## Foundry

**Foundry is a blazing fast, portable and modular toolkit for Ethereum application development written in Rust.**

Foundry consists of:

- **Forge**: Ethereum testing framework (like Truffle, Hardhat and DappTools).
- **Cast**: Swiss army knife for interacting with EVM smart contracts, sending transactions and getting chain data.
- **Anvil**: Local Ethereum node, akin to Ganache, Hardhat Network.
- **Chisel**: Fast, utilitarian, and verbose solidity REPL.

## Architecture

This codebase implements a fiat on-ramp and off-ramp system with LP staking, escrowed settlement, and optional off-chain orchestration.

### Core Contracts

- `src/core/LPRegistry.sol`  
  LP registration and staking. Tracks active status, stake amounts, and supports slashing.
- `src/core/PaymentEscrow.sol`  
  Off-ramp escrow (crypto-to-fiat). Users lock stablecoins, LPs claim with a permission slip, and an oracle multi-sig confirms settlement to release funds.
- `src/core/OnRampEscrow.sol`  
  On-ramp escrow (fiat-to-crypto). LPs lock crypto for orders, users confirm fiat sent, LPs release crypto, with timeouts and dispute resolution.
- `src/Relayer.sol`  
  Optional relayer controlled by a backend to lock LP funds on their behalf.
- `src/core/RateOracle.sol`  
  Price feed adapter (Chainlink-based at present).

### Key Flows

- **Off-ramp**: User locks stablecoin → LP claims via permission slip → oracle confirms fiat settlement → escrow releases funds to LP.
- **On-ramp**: User creates order → LP locks crypto → user confirms fiat sent → LP releases crypto (with timeout and dispute paths).

### Diagram

```mermaid
flowchart LR
  User((User))
  LP((Liquidity Provider))
  Oracle((Oracle Multi-Sig))
  Backend((Backend/Relayer))

  PaymentEscrow[PaymentEscrow]
  OnRampEscrow[OnRampEscrow]
  LPRegistry[LPRegistry]
  Relayer[Relayer]
  RateOracle[RateOracle]

  User -->|Create payment| PaymentEscrow
  LP -->|Claim (permission slip)| PaymentEscrow
  Oracle -->|Confirm settlement| PaymentEscrow
  PaymentEscrow -->|Check active LP| LPRegistry

  User -->|Create order| OnRampEscrow
  LP -->|Lock funds| OnRampEscrow
  Backend -->|Lock funds for LP| Relayer --> OnRampEscrow
  OnRampEscrow -->|Role/LP checks| LPRegistry

  RateOracle -.-> PaymentEscrow
  RateOracle -.-> OnRampEscrow
```

## Documentation

https://book.getfoundry.sh/

## Usage

### Build

```shell
$ forge build
```

### Test

```shell
$ forge test
```

### Format

```shell
$ forge fmt
```

### Gas Snapshots

```shell
$ forge snapshot
```

### Anvil

```shell
$ anvil
```

### Deploy

```shell
$ forge script script/Counter.s.sol:CounterScript --rpc-url <your_rpc_url> --private-key <your_private_key>
```

### Cast

```shell
$ cast <subcommand>
```

### Help

```shell
$ forge --help
$ anvil --help
$ cast --help
```
