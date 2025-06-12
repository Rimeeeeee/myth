# Myth: Minimal Ethereum Beacon Consensus Client

A work-in-progress minimal Ethereum Beacon (consensus layer) client written in Rust.  
The goal is to **sync with the Ethereum mainnet**, validate blocks, and progressively add features — starting with the core.

---

## 🛠️ Project Goals

- Build a lightweight, modular, and understandable Beacon node.
- Sync from a checkpoint and validate the chain.
- Designed for learning and customization over performance.

---

---

## ✅ Task List / Roadmap

> We are progressing in layered steps — from data representation to sync.

### ✅ Phase 1: SSZ Implementation (Completed)

👉 **[r_ssz GitHub Repository](https://github.com/Soubhik-10/r_ssz)**

> ⚠️ This is not production-ready. Any help, suggestions, or feedback are gladly encouraged and appreciated!

- [x] Implement SSZ serialization and deserialization
- [x] Derive hash tree root for basic + composite types
- [x] Support for:
  - [x] `BitList`, `BitVector`, `Vector`
  - [x] `Option<T>`, `List<T>`, `Container` , `Union`
- [x] Test with synthetic and edge-case data
- [x] Compliant with spec hash root calculations

---

### 🟡 Phase 2: Consensus Types (WIP)

> This is a wip phase

- [ ] Define all required types from [Ethereum Consensus Specs](https://github.com/ethereum/consensus-specs):
  - [ ] `BeaconBlock`
  - [ ] `BeaconState`
  - [ ] `Attestation`, `Checkpoint`, `Validator`, etc.
- [ ] Derive SSZ and `hash_tree_root` for each type
- [ ] Add unit tests with reference vectors

## 🔧 Building the Project

```bash
cargo build
```
