use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use anyhow::Result;
use tracing::info;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use crate::protocol::clock::VectorClock;
use crate::protocol::message::ConfigEntry;

pub struct DistributedStore {
    pub data: Arc<RwLock<HashMap<String, ConfigEntry>>>,
    pub clock: Arc<RwLock<VectorClock>>,
    pub node_id: String,
}

impl DistributedStore {
    pub fn new(node_id: String) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            clock: Arc::new(RwLock::new(VectorClock::default())),
            node_id,
        }
    }

    pub async fn reconcile(&self, incoming_entry: ConfigEntry, incoming_clock: &VectorClock) -> Result<bool> {
        let mut data = self.data.write().await;
        let mut clock = self.clock.write().await;

        if let Some(existing) = data.get(&incoming_entry.id) {
            // Conflict Resolution via Vector Clock
            match incoming_clock.partial_cmp(&existing.clock) {
                Some(std::cmp::Ordering::Greater) => {
                    data.insert(incoming_entry.id.clone(), incoming_entry.clone());
                    clock.merge(incoming_clock);
                    Ok(true)
                }
                _ => Ok(false), // Ignore stale or concurrent updates
            }
        } else {
            data.insert(incoming_entry.id.clone(), incoming_entry);
            clock.merge(incoming_clock);
            Ok(true)
        }
    }
}
