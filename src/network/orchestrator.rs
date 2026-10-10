use libp2p::{kad, swarm::SwarmEvent};
use futures::StreamExt;
use anyhow::Result;
use crate::network::controller::MeshController;
use std::sync::Arc;

pub struct MeshOrchestrator {
    pub controller: Arc<MeshController>,
}

impl MeshOrchestrator {
    pub fn new() -> Self {
        Self {
            controller: Arc::new(MeshController::new()),
        }
    }

    pub async fn route_event(&self, peer: libp2p::PeerId, msg: crate::protocol::message::MeshMessage) -> Result<Option<crate::protocol::message::MeshMessage>> {
        self.controller.handle_message(peer, msg).await
    }
}
