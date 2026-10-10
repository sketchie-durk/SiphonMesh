use serde::{Serialize, Deserialize};
use libp2p::PeerId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MeshMessage {
    Handshake {
        node_id: PeerId,
        version: String,
    },
    ConfigPush {
        config_id: String,
        payload: String,
    },
    SignalBroadcast {
        origin: PeerId,
        signal_data: Vec<u8>,
    },
    QueryConfig {
        config_id: String,
    },
    ConfigResponse {
        config_id: String,
        payload: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeshConfig {
    pub id: String,
    pub author: String,
    pub data: String,
    pub signature: Vec<u8>,
}
