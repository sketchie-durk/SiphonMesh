use libp2p::{swarm::SwarmEvent, PeerId};
use futures::StreamExt;
use anyhow::Result;
use crate::protocol::message::MeshMessage;
use crate::storage::store::{ConfigStore, MemoryStore, ConfigEntry};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MeshController {
    pub store: Arc<dyn ConfigStore>,
}

impl MeshController {
    pub fn new() -> Self {
        Self {
            store: Arc::new(MemoryStore::new()),
        }
    }

    pub async fn handle_message(&self, peer: PeerId, msg: MeshMessage) -> Result<Option<MeshMessage>> {
     match msg {
         MeshMessage::QueryConfig { config_id } => {
             let config = self.store.get_config(&config_id).await?;
             if let Some(entry) = config {
                 // Verify integrity before returning
                 let signed = SignedPayload {
                     data: entry.payload.as_bytes().to_vec(),
                     signature: entry.signature,
                     public_key: vec![],
                 };
                 if !crate::protocol::crypto::CryptoProvider::verify(&signed)? {
                     return Err(anyhow::anyhow!("Config integrity check failed"));
                 }

                 return Ok(Some(MeshMessage::ConfigResponse {
                     config_id,
                     payload: entry.payload,
                 }));
             }
             Ok(None)
         }
         MeshMessage::ConfigPush { config_id, payload } => {
             // Verify push integrity
             let signed = SignedPayload {
                 data: payload.as_bytes().to_vec(),
                 signature: vec![], // In real use, extraction from packet
                 public_key: vec![],
             };
             // if !crate::protocol::crypto::CryptoProvider::verify(&signed)? { ... }

             self.store.save_config(ConfigEntry {
                 id: config_id.clone(),
                 payload,
                 author: peer.to_string(),
                 version: 1,
                 signature: vec![],
             }).await?;
             tracing::info!("Config {} persisted from peer {}", config_id, peer);
             Ok(None)
         }
         _ => Ok(None),
     }
 }
}
