# 🌌 SiphonMesh

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Build: Passing](https://img.shields.io/badge/Build-Passing-brightgreen.svg)]()
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)]()

**STATUS: ALPHA | VERSION: 0.1.0 | CODENAME: NEURAL_NET**

`SiphonMesh` is a high-density decentralized orchestration mesh designed for the secure sharing of signal configurations and distributed coordination. Built on `libp2p`, it enables a peer-to-peer infrastructure where research nodes can synchronize without reliance on central authority.

## 🛠 Technical Specification
The system implements a modular P2P stack:
- **Transport:** TCP with Noise encryption and Yamux multiplexing.
- **Discovery:** mDNS for local discovery, Kademlia DHT for global routing.
- **Protocol:** Custom binary serialization for `MeshMessage` types.
- **Security:** Identity-based peer authentication.

## 🏗 Architecture
`Node` $\rightarrow$ `Swarm` $\rightarrow$ `NetworkBehaviour` $\rightarrow$ `ProtocolHandler` $\rightarrow$ `LocalStore`

## 🚀 Quick Start
```bash
cargo build --release
./target/release/siphonmesh
```

---
*Decentralized. Encrypted. Autonomous.*
