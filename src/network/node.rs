use libp2p::PeerId;
use anyhow::Result;
use crate::protocol::message::{MeshMessage, MeshConfig};
use crate::storage::store::{ConfigStore, MemoryStore, ConfigEntry};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MeshNode {
    pub peer_id: PeerId,
    pub store: Arc<dyn ConfigStore>,
}

impl MeshNode {
    pub fn new(peer_id: PeerId) -> Self {
        Self {
            peer_id,
            store: Arc::new(MemoryStore::new()),
        }
    }

    pub async fn broadcast_config(&self, config_id: String, payload: String) -> MeshMessage {
        MeshMessage::ConfigPush {
            config_id,
            payload,
        }
    }

    pub async fn request_config(&self, config_id: String) -> MeshMessage {
        MeshMessage::QueryConfig { config_id }
    }
}
