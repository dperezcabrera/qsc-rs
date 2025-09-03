# 🗺️ QSC-RS Roadmap (Learning Edition)

This roadmap is designed to guide QSC-RS as a **learning project**.  
Each milestone introduces new concepts in blockchain, Rust, and cryptography, while keeping the system simple and approachable.

---

## ✅ Current Foundation
- Single-node and multi-node **Proof of Authority (PoA)** demo.
- **Post-quantum signatures** (Dilithium3) for transactions and blocks.
- Basic **token contract** with `mint` and `transfer`.
- REST API built with **Actix-web**.
- Dockerized environment for easy setup.
- CLI (`qsc-tools`) for keys and signatures.

These features already allow you to explore:
- How a blockchain maintains state.
- How consensus works with multiple validators.
- How cryptography secures transactions.

---

## 🔜 Step 1: Transaction IDs with BLAKE3
**Goal:** Learn how blockchains identify transactions uniquely.  
- Replace ad-hoc IDs with **BLAKE3 hashes**.  
- Display transaction IDs in API and CLI.  
- Show how hashing guarantees immutability and traceability.  

*Concepts to learn:* hashing functions, immutability, integrity.  

---

## 🪙 Step 2: Token Standardization (ERC-20 Style)
**Goal:** Understand token standards and interoperability.  
- Extend the token contract with `total_supply`, `approve`, `transfer_from`, `allowance`.  
- Compare with Ethereum’s ERC-20 standard.  

*Concepts to learn:* smart contract interfaces, fungible tokens, compatibility.  

---

## ⚙️ Step 3: Developer Ergonomics
**Goal:** Improve usability and debugging.  
- Add better error messages and logs.  
- CLI improvements: JSON output, colored text, examples.  

*Concepts to learn:* developer experience, observability, usability.  

---

## 🛡️ Step 4: Consensus & Security
**Goal:** Explore advanced consensus and defenses.  
- Extend PoA with configurable validator sets.  
- Add rate-limiting and transaction pool limits.  
- Introduce metrics endpoint (Prometheus).  

*Concepts to learn:* consensus models, denial-of-service prevention, monitoring.  

---

## 🗄️ Step 5: Storage Evolution
**Goal:** Learn about blockchain persistence.  
- Migrate from JSONL to an embedded DB (e.g., `sled` or `rocksdb`).  
- Add indexes by transaction ID and block number.  

*Concepts to learn:* data storage, indexing, performance.  

---

## 🚀 Step 6: Proof of Stake (PoS) Prototype
**Goal:** Experiment with stake-based security.  
- Simple staking contract.  
- Validator selection by stake.  
- Introduce basic slashing rules.  

*Concepts to learn:* economic security, incentives, validator selection.  

---

## 🧩 Step 7: Smart Contract Playground
**Goal:** Build a foundation for user-defined logic.  
- Add support for **WASM-based smart contracts**.  
- Provide a Rust SDK for simple contracts.  

*Concepts to learn:* virtual machines, sandboxing, contract deployment.  

---

## 🌐 Step 8: Networking
**Goal:** Move beyond Docker-only clusters.  
- Add basic **p2p networking** (libp2p or custom).  
- Peer discovery and gossip-based block sync.  

*Concepts to learn:* peer-to-peer systems, networking protocols, synchronization.  

---

## 🎯 Final Vision
QSC-RS will remain a **minimal, educational blockchain**:
- Simple enough for students and hobbyists to understand.  
- Modular enough to extend with new experiments.  
- A safe playground for learning **Rust, cryptography, and blockchain design**.  

---

## 📅 Suggested Learning Path
1. Start with **transaction IDs (BLAKE3)** → grasp immutability.  
2. Add **ERC-20 token features** → learn standards.  
3. Improve **logs and CLI** → developer experience.  
4. Explore **consensus & security** → real-world challenges.  
5. Migrate to **embedded DB** → performance lessons.  
6. Prototype **PoS** → economic models.  
7. Add **WASM contracts** → smart contract sandbox.  
8. Enable **p2p networking** → distributed systems.  

At each step, the project grows with your knowledge ✨

