# 🌌 SiphonMesh: Distributed State & Consistency

## 1. Causality and Consistency (Phase 6)
To manage state across a decentralized mesh without a central authority, SiphonMesh implements **Vector Clocks**. This allows nodes to determine the partial ordering of events and resolve conflicts in a distributed environment.

### Vector Clock Logic
Every `MeshMessage` carries a `VectorClock` (a map of `NodeID -> Counter`).
- **Increment:** When a node originates a message, it increments its own counter.
- **Merge:** Upon receiving a message, the node merges the incoming clock with its local clock by taking the maximum value for each entry.
- **Causality:** A message is considered "stale" if its clock is strictly less than the local clock, and "concurrent" if neither is greater than the other.

## 2. Distributed State Reconciliation (Phase 7)
The `DistributedStore` implements a conflict-resolution strategy based on the Vector Clock.

### Reconciliation Algorithm
When a `ConfigPush` is received:
1. Compare the incoming `VectorClock` with the local version of the configuration.
2. If `incoming_clock > local_clock`, the local store is updated.
3. If clocks are concurrent or the incoming is stale, the update is ignored or flagged for manual merge.

## 3. Network Resilience (Phase 8)
The mesh leverages a hybrid discovery model:
- **mDNS:** Zero-conf discovery for local subnet peers.
- **Kademlia DHT:** Logarithmic routing for global peer discovery and configuration lookups.
- **Noise Protocol:** Ensures that all peer-to-peer communication is encrypted and authenticated, preventing impersonation attacks.

## 4. Operational Summary
`P2P Discovery` $\rightarrow$ `Noise Handshake` $\rightarrow$ `Vector Clock Sync` $\rightarrow$ `Config Reconciliation` $\rightarrow$ `Distributed Consensus`.
