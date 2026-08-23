//! York Autotype ↔ nxmesh AgentHeartbeat integration
//!
//! This module produces messages that are fully compatible with
//! `nxmesh::protocol::MeshMessage::AgentHeartbeat`.
//!
//! When the nxmesh crate is linked, these can be published directly
//! onto the mesh topic `nexus/mesh/v0`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Configuration for York heartbeats
#[derive(Debug, Clone)]
pub struct HeartbeatConfig {
    pub node_id: String,
    pub interval_secs: u64,
    pub status: String,
}

impl Default for HeartbeatConfig {
    fn default() -> Self {
        Self {
            node_id: "york-node-001".into(),
            interval_secs: 30,
            status: "alive".into(),
        }
    }
}

/// Fully compatible with nxmesh::protocol::MeshMessage::AgentHeartbeat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YorkHeartbeat {
    pub agent: String,
    pub node_id: String,
    pub status: String,
    pub ts: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<serde_json::Value>,
}

impl YorkHeartbeat {
    /// Create a standard "alive" heartbeat for this York node
    pub fn alive(node_id: impl Into<String>) -> Self {
        Self {
            agent: "york-autotype".into(),
            node_id: node_id.into(),
            status: "alive".into(),
            ts: Utc::now(),
            extra: Some(json!({
                "prototype": "York Autotype",
                "version": env!("CARGO_PKG_VERSION"),
                "capabilities": [
                    "automation",
                    "typing",
                    "task-execution",
                    "mesh-heartbeat"
                ],
                "mesh_substrate": "nxmesh",
                "github": "https://github.com/digitaldesignerjazz/york-autotype"
            })),
        }
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = status.into();
        self
    }

    /// Serialize to the exact JSON shape expected by nxmesh Gossipsub
    pub fn to_mesh_json(&self) -> serde_json::Result<Vec<u8>> {
        // Match the tagged enum shape used by nxmesh::MeshMessage
        let envelope = json!({
            "type": "AgentHeartbeat",
            "payload": {
                "agent": self.agent,
                "node_id": self.node_id,
                "status": self.status,
                "ts": self.ts,
                "extra": self.extra
            }
        });
        serde_json::to_vec(&envelope)
    }

    pub fn to_pretty_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heartbeat_shape_is_valid() {
        let hb = YorkHeartbeat::alive("test-node");
        let bytes = hb.to_mesh_json().unwrap();
        let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(v["type"], "AgentHeartbeat");
        assert_eq!(v["payload"]["agent"], "york-autotype");
        assert!(v["payload"]["extra"].is_object());
    }
}
