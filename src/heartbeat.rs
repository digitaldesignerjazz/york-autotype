//! York Autotype — Mesh Heartbeat Stub
//!
//! This module prepares the AgentHeartbeat message that will be published
//! over nxmesh once the node is connected to the Nexus mesh.
//!
//! See: https://github.com/digitaldesignerjazz/nexus/tree/main/mesh/noise-quic

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Compatible with nxmesh::protocol::MeshMessage::AgentHeartbeat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YorkHeartbeat {
    pub agent: String,
    pub node_id: String,
    pub status: String,
    pub ts: DateTime<Utc>,
    pub extra: Option<serde_json::Value>,
}

impl YorkHeartbeat {
    pub fn alive(node_id: impl Into<String>) -> Self {
        Self {
            agent: "york-autotype".into(),
            node_id: node_id.into(),
            status: "alive".into(),
            ts: Utc::now(),
            extra: Some(serde_json::json!({
                "prototype": "York Autotype",
                "capabilities": ["automation", "typing", "task-execution", "mesh-heartbeat"]
            })),
        }
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heartbeat_serializes() {
        let hb = YorkHeartbeat::alive("test-node-001");
        let json = hb.to_json().unwrap();
        assert!(json.contains("york-autotype"));
        assert!(json.contains("alive"));
    }
}
