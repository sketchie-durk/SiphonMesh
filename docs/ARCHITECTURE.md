# 🌌 SiphonMesh: Technical Architecture & Operational Manual

## 1. P2P Topology
SiphonMesh implements a **Structured Overlay Network** using a Kademlia-based Distributed Hash Table (DHT). This ensures that any configuration or signal can be located in $O(\log N)$ hops across the mesh.

### Transport Stack
- **L1:** TCP / UDP
- **L2:** Noise Protocol (XX Handshake) for end-to-end encryption.
- **L3:** Yamux for stream multiplexing.
- **L4:** Custom `MeshMessage` binary protocol.

## 2. Distributed Storage Model
Configurations are not stored centrally but are partitioned across the mesh. 
- **Symmetry:** Every node acts as both a client (requester) and a server (provider).
- **Consistency:** Versions are tracked via a monotonically increasing integer, ensuring the mesh converges on the latest "Siphon" config.

## 3. Cryptographic Integrity
To prevent poisoning attacks, every `ConfigPush` must be accompanied by a digital signature.
- **Verification:** Nodes verify signatures using the author's public key before persisting the data to the `MemoryStore`.
- **Trust Model:** Implicit trust is based on PeerID verification via the Noise handshake.

## 4. Operational Flow
`Peer Discovery (mDNS)` $\rightarrow$ `Kademlia Join` $\rightarrow$ `Config Sync (Push/Query)` $\rightarrow$ `Signal Broadcast`.
