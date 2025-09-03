# QSC-RS: Minimal Blockchain with Post-Quantum Signatures ⛓️💎

**QSC-RS (Quantum-Safe Contracts in Rust)** is a lightweight and experimental blockchain implementation that uses **post-quantum digital signatures** to ensure the integrity of transactions. It is designed as a simple, modular, and easy-to-deploy system—ideal for researching and prototyping *quantum-resistant blockchain* concepts.

The project core leverages the **ML-DSA (Dilithium)** signature algorithm, one of the NIST-selected standards for PQC.

## ✨ Key Features

* **Post-Quantum Signatures**: Uses **ML-DSA-3 (Dilithium3)** for all transaction and block signatures, ensuring long-term security.
* **Proof of Authority (PoA) Consensus**: Implements a simple and configurable round-robin consensus mechanism for N validators.
* **Simple Token Contract**: Includes a basic token contract with `mint` and `transfer` functionalities, with safe arithmetic and validations.
* **Modern HTTP API**: Exposes a RESTful API built with `Actix-web` to interact with the chain (send transactions, query state, etc.).
* **Security and Robustness**: Features anti-replay protection (`nonce` and `chain_id`), anti-spam (transaction limits), and canonical serialization for signatures.
* **Fully Containerized**: The entire environment, from build to runtime and tools, is managed with Docker for maximum portability.
* **Auxiliary Tools**: Includes `qsc-tools`, a CLI to generate keys, derive addresses, and sign payloads off-chain.

## 🚀 Quick Start: Single-Node Demo

These steps let you build the Docker image, run a local node, and perform some basic transactions.

### 1. Requirements

* **Docker**: Must have Docker installed and the daemon running.
* **Bash** and **cURL/jq**: Used in helper scripts to interact with the node.

### 2. Run the Automated Demo

The `demo.sh` script encapsulates the whole process. It’s the fastest way to see everything in action!

```bash
bash scripts/demo.sh
```

This script will:

1. **Build** the Docker image `qsc-rs-simple-contracts`.
2. **Start** a node at `http://localhost:8000`.
3. **Generate** a keypair for a user named "alice".
4. **Mint** 1000 tokens for "alice" (signing the transaction with her key).
5. **Transfer** 150 tokens from "alice" to "bob".
6. **Query** the final balances of "alice" and "bob".

### 3. Manual Steps (Alternative)

If you prefer running each step manually:

```bash
# 1. Build the Docker image
bash scripts/build.sh

# 2. Start the node in the background
# (Use -it instead of -d to see logs in the foreground)
docker run -d --rm --name qsc-node -p 8000:8000 \
  -v "$(pwd)/data:/data" \
  qsc-rs-simple-contracts

# 3. Generate keys
bash scripts/keygen.sh alice

# 4. Submit a transaction (mint 1000 tokens for alice)
bash scripts/submit_addr.sh alice '{"contract":"token","method":"mint","args":{"to":"SELF","amount":1000}}'

# 5. Query alice’s balance
bash scripts/query.sh token balance_of '{"who":"<alice_address>"}'
# Note: The address is derived from the public key.

# 6. Stop and clean up the node
bash scripts/stop.sh && bash scripts/rm.sh
```

## 🌐 Multi-Node Deployment (PoA)

The project supports a Proof of Authority (PoA) validator cluster. Here’s an example with 3 nodes.

1. **Build the image and generate keys**:
   The `build.sh` script handles it all.

   ```bash
   bash scripts/build.sh
   ```

   This creates the Docker image and generates validator keys `v1`, `v2`, and `v3` in the `keys/` directory.

2. **Start the validator nodes**:
   Each `run-N.sh` script launches a node configured to participate in PoA consensus.

   ```bash
   bash scripts/run-1.sh
   bash scripts/run-2.sh
   bash scripts/run-3.sh
   ```

   * Each node runs on a different port (8001, 8002, 8003).
   * All share the same validator configuration (`QSC_VALIDATORS_JSON`) and Docker network (`qsc-net`).
   * They will automatically propose and synchronize blocks among themselves.

3. **Interact with the cluster**:
   You can submit transactions to any node. For example, to send to node 1 (port 8001):

   ```bash
   # (Make sure you’ve created keys for 'alice' first)
   bash scripts/keygen.sh alice

   # Submit the transaction (v3 script uses nonce and chain_id)
   bash scripts/submit_addr_v3.sh alice '{"contract":"token","method":"mint","args":{"to":"SELF","amount":500}}'
   ```

## 🛠️ Architecture and Components

The project is structured into clear, decoupled Rust modules:

* **`main.rs`**: Defines the HTTP API with Actix-web. Node entry point.
* **`runtime.rs`**: The chain’s core. Manages state (`Ctx`), the mempool, block production, and transaction execution.
* **`contracts/`**: Smart contract logic.

  * `token.rs`: Fungible token implementation.
* **`consensus.rs`**: PoA consensus logic, including leader selection and block validation.
* **`pq.rs`**: Abstraction for post-quantum crypto operations (keygen, sign, verify) using the `pqcrypto-dilithium` library.
* **`storage.rs`**: Manages chain persistence (`chain.jsonl`) and state (`state.json`) on disk.
* **`bin/qsc-tools.rs`**: A standalone CLI tool for cryptographic tasks, useful for clients and scripts.

## 🗺️ Roadmap and Future Improvements

The project provides a solid foundation, but many interesting improvements are planned. See [**ROADMAP.md**](https://www.google.com/search?q=ROADMAP.md) for the next steps, including:

1. **Cryptographic Transaction IDs**: Formalize transaction hashing.
2. **Token Standardization**: Add compatibility with ERC-20-like interfaces.
3. **Evolution to PoS**: Migrate from Proof of Authority to Proof of Stake.
4. **Security and Ergonomics Enhancements**: Add rate-limiting, metrics, and improve the developer experience.

