use serde::{Serialize, Deserialize};
use libp2p::PeerId;
use crate::protocol::clock::VectorClock;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MeshMessage {
    Handshake {
        node_id: PeerId,
        version: String,
        clock: VectorClock,
    },
    ConfigPush {
        config_id: String,
        payload: String,
        clock: VectorClock,
    },
    SignalBroadcast {
        origin: PeerId,
        signal_data: Vec<u8>,
        clock: VectorClock,
    },
    QueryConfig {
        config_id: String,
        clock: VectorClock,
    },
    ConfigResponse {
        config_id: String,
        payload: String,
        clock: VectorClock,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshConfig {
    pub id: String,
    pub author: String,
    pub data: String,
    pub signature: Vec<u8>,
    pub clock: VectorClock,
}
