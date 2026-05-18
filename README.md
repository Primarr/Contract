# Primer — Smart Contracts

> **On-chain infrastructure for agent-to-agent commerce on Stellar.**  
> Soroban contracts that power service discovery, autonomous spending limits, and settlement for the Primer SDK.

[![Stellar](https://img.shields.io/badge/Stellar-Soroban-7D00FF)](https://soroban.stellar.org)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)

**Primer** is a B2B SDK that lets AI agents pay each other for services — data, compute, API calls, model inference — settled instantly on Stellar, with no human in the loop. This repository contains the **Soroban smart contracts** that make that possible: a public service registry, programmable spending guardrails, and protocol-level settlement hooks aligned with Stellar's agentic payments roadmap (including x402).

---

## Why these contracts exist

When Agent A (owned by Company X) needs inference from Agent B (owned by Company Y), Web2 billing does not scale: API keys, monthly invoices, and manual approval cannot keep up with thousands of micro-decisions per hour. Primer moves money at machine speed — **pay-per-call, sub-second settlement, USDC on Stellar** — and these contracts are the trust layer enterprises need before they deploy autonomous spending agents.

| Contract | Role |
|----------|------|
| **Service Registry** | Agents publish capabilities, prices, and payout addresses on-chain. Other agents discover and pay without a central directory operator. |
| **Budget Vault** | Per-session, per-task, or time-window spending caps enforced on-chain before funds leave the agent's vault. |
| **Settlement Router** | Atomic pay-on-invoke flow: payment clears on Stellar before the service handler returns (x402-compatible pattern). |
| **Protocol Treasury** | Configurable protocol fee (e.g. 0.2%) routed on each agent-to-agent transaction. |

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Primer Agent Runtime                     │
│              (LangChain, CrewAI, custom orchestrator)        │
└──────────────────────────┬──────────────────────────────────┘
                           │ SDK calls
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                   Primer Backend + SDK                       │
│         build txs · sign · submit · registry queries         │
└──────────────────────────┬──────────────────────────────────┘
                           │ invoke / read
                           ▼
┌─────────────────────────────────────────────────────────────┐
│              THIS REPO — Soroban Contracts                   │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────────────┐ │
│  │   Registry   │ │ Budget Vault │ │  Settlement Router   │ │
│  └──────────────┘ └──────────────┘ └──────────────────────┘ │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ▼
              Stellar Network (USDC / native assets)
```

### Service Registry

Developers register an agent service with:

- **Service ID** — unique identifier (e.g. `inference:gpt-wrapper:v1`)
- **Capability metadata** — human- and machine-readable description (URI or on-chain blob hash)
- **Price per invocation** — fixed or tiered, denominated in USDC (or configured asset)
- **Payout address** — Stellar account receiving payments
- **Status** — active, paused, deprecated

Other agents query the registry to discover services and quoted prices before calling `primer.pay()` in the [Backend SDK](https://github.com/Primarr/Backend).

### Budget Vault

Enterprise teams set policies Soroban enforces without per-tx human approval:

| Policy type | Example |
|-------------|---------|
| Session cap | Max 5 USDC per agent session |
| Task cap | Max 0.50 USDC per workflow run |
| Rate limit | Max 100 payments / hour |
| Allowlist | Only pay registered service IDs |

When a payment would exceed policy, the contract **reverts** and the Backend emits a webhook for optional human override — the compliance layer that makes autonomous agents deployable in production.

### Settlement Router

Implements the **pay-before-serve** pattern:

1. Caller locks payment amount (+ protocol fee) in contract or escrows via authorized transfer
2. Service provider's handler is invoked (off-chain attestation or on-chain callback)
3. On success, funds release to provider; on failure/timeout, funds return to caller

Designed for compatibility with **x402-style agentic payment flows** on Stellar as the ecosystem standardizes HTTP 402 + payment headers for AI agents.

---

## Tech stack

| Layer | Choice |
|-------|--------|
| Smart contracts | **Rust** → Soroban WASM |
| Tooling | `soroban-cli`, Stellar Scaffold (planned) |
| Assets | USDC (Stellar mainnet / testnet) |
| Testing | `soroban-test-helpers`, local sandbox |
| Deployment | Testnet → Futurenet → Mainnet (phased) |

---

## Repository layout (planned)

```
contracts/
├── registry/          # Service discovery & pricing
├── budget-vault/      # Spending policies & caps
├── settlement/        # Pay-on-invoke router
└── treasury/          # Protocol fee collection

tests/
├── integration/       # Cross-contract flows
└── fuzz/              # Policy edge cases

scripts/
├── deploy-testnet.sh
└── initialize-registry.sh
```

---

## Getting started

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)
- Stellar test account with XLM for fees ([Friendbot](https://laboratory.stellar.org/#account-creator?network=test) on testnet)

### Build & test

```bash
# Install Soroban CLI (if needed)
cargo install --locked soroban-cli

# Clone and enter this repo
git clone https://github.com/Primarr/Contract.git
cd Contract

# Build contracts (once scaffolded)
soroban contract build

# Run tests
cargo test
```

### Deploy to testnet

```bash
soroban keys generate --global primer-deployer --network testnet
soroban network fetch testnet

# Example — deploy registry (command will be updated post-scaffold)
# soroban contract deploy --wasm target/wasm32-unknown-unknown/release/registry.wasm --network testnet
```

---

## Alignment with Stellar Development Foundation priorities

Primer directly supports SDF's stated direction toward **agentic payments** and **x402 on Stellar**:

- **Machine-speed settlement** — Stellar's ~5s finality and negligible per-payment cost vs. L1 gas chains
- **Soroban programmability** — spending policies and registry logic that cannot live in a centralized API alone
- **USDC-native** — the asset AI API marketplaces already price in
- **Open infrastructure** — contracts are the composable layer any agent framework can integrate

We are building the **first SDK + on-chain registry stack** purpose-built for agent-to-agent commerce, ahead of the demand curve SDF has identified for 2026.

---

## Security & audit roadmap

| Phase | Milestone |
|-------|-----------|
| Alpha | Internal review, testnet only |
| Beta | Third-party audit (budgeted in grant request) |
| Mainnet | Bug bounty + immutable registry version |

Contracts will follow Soroban best practices: minimal storage writes, explicit authorization, no unchecked external calls, and upgrade path via admin multisig (not proxy ambiguity).

---

## Related repositories

| Repo | Description |
|------|-------------|
| [Backend](https://github.com/Primarr/Backend) | Node.js SDK (`primer.pay`, registry client, webhooks) |
| [Frontend](https://github.com/Primarr/Frontend) | Developer dashboard — registry browser, analytics, budget UI |

---

## Contributing

We welcome contributors focused on Soroban, payment protocols, and agent infrastructure. Open an issue before large PRs. See `CONTRIBUTING.md` (coming soon).

---

## License

Apache 2.0 — see [LICENSE](LICENSE).

---

## Contact

**Organisation:** [Primarr](https://github.com/Primarr)  
**Programme:** Stellar Community Fund / SDF ecosystem build  
**Questions:** Open a GitHub issue or reach out via your SCF application thread.

*Primer — payment rails for the agent economy.*
