<p align="center">
  <img src="./github-banner.png" alt="luum banner" width="100%" />
</p>

<p align="center">
  <a href="https://x.com/luumdotli">
    <img src="https://img.shields.io/badge/Twitter-@luumdotli-1DA1F2?style=flat-square&logo=twitter&logoColor=white" alt="Twitter" />
  </a>
  <a href="https://luum.li">
    <img src="https://img.shields.io/badge/Website-luum.li-F5A623?style=flat-square&logo=vercel&logoColor=white" alt="Website" />
  </a>
  <a href="https://github.com/luum-labs/luum/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/luum-labs/luum/ci.yml?style=flat-square&label=CI" alt="CI" />
  </a>
  <img src="https://img.shields.io/badge/License-MIT-green?style=flat-square" alt="License" />
</p>

<h3 align="center">High-frequency x402 micro-payment analysis engine for Solana AI agents</h3>

---

## Architecture

```mermaid
graph TD
    A[Solana RPC / Helius] -->|x402 Transactions| B[luum_core]
    B -->|Parsed Flows| C[luum_math]
    C -->|Sankey Clusters| D[SDK / Client]
    D -->|Visualization Data| E[Web Dashboard]
    B -->|Delegation Check| F[Revoke Engine]
    F -->|On-chain TX| A

    subgraph On-Chain Program
        B
        F
    end

    subgraph Off-Chain Libraries
        C
        D
    end
```

## Features

| Feature | Description | Module |
|---------|-------------|--------|
| Micro-transaction parsing | Decode x402 USDC transfers from Solana agent wallets | `luum_core` |
| Receiver clustering | Group payment destinations by frequency and amount | `luum_math` |
| Sankey flow generation | Build weighted directed graphs for visualization | `luum_math` |
| Delegation revoke | Sever leaking spending authorities on-chain | `luum_core` |
| CLI analysis | Terminal-based wallet analysis and reporting | `cli` |
| TypeScript SDK | Programmatic access to parsing and clustering | `sdk` |

## Installation

```bash
git clone https://github.com/luum-labs/luum.git
cd luum
```

### Build the on-chain program

```bash
anchor build
```

### Build the CLI

```bash
cargo build --release -p cli
```

### Install SDK dependencies

```bash
cd sdk && npm install
```

## Usage

### Analyze an agent wallet via CLI

```bash
./target/release/cli analyze --address <WALLET_ADDRESS> --days 7
```

### Use the TypeScript SDK

```typescript
import { LuumClient } from "./luum-sdk";

const client = new LuumClient("https://api.helius.xyz/v0");
const flows = await client.analyzeWallet("AgentWa11et...", { days: 7 });
console.log(flows.sankey);
```

## Project Structure

```
luum/
  programs/luum_core/   Anchor program (on-chain)
  libs/luum_math/       Clustering and Sankey engine
  cli/                            Command-line analysis tool
  sdk/                            TypeScript client library
  tests/                          Integration tests
```

## License

MIT
<!-- docs --> v70
