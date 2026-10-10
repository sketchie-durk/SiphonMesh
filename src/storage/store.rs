use libp2p::{kad::{Kademlia, KademliaEvent, Store}, PeerId};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigEntry {
    pub id: String,
    pub payload: String,
    pub version: u32,
}

#[async_trait]
pub trait ConfigStore: Send + Sync {
    async fn save_config(&self, entry: ConfigEntry) -> Result<()>;
    async fn get_config(&self, id: &str) -> Result<Option<ConfigEntry>>;
    async fn list_configs(&self) -> Result<Vec<ConfigEntry>>;
}

pub struct MemoryStore {
    data: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, ConfigEntry>>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            data: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }
}

#[async_trait]
impl ConfigStore for MemoryStore {
    async fn save_config(&self, entry: ConfigEntry) -> Result<()> {
        let mut lock = self.data.write().await;
        lock.insert(entry.id.clone(), entry);
        Ok(())
    }

    async fn get_config(&self, id: &str) -> Result<Option<ConfigEntry>> {
        let lock = self.data.read().await;
        Ok(lock.get(id).cloned())
    }

    async fn list_configs(&self) -> Result<Vec<ConfigEntry>> {
        let lock = self.data.read().await;
        Ok(lock.values().cloned().collect())
    }
}
