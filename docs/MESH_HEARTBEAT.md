# York Autotype ↔ nxmesh AgentHeartbeat Integration

## Overview

York Autotype emits presence using the exact message format defined by **nxmesh**:

```rust
MeshMessage::AgentHeartbeat {
    agent: "york-autotype",
    node_id: "...",
    status: "alive",
    ts: <UTC timestamp>,
    extra: { capabilities, version, github, ... }
}
```

This message is published (or will be published) on the mesh gossip topic:

```
nexus/mesh/v0
```

## Current Status

| Layer                    | Status                          |
|--------------------------|---------------------------------|
| Message shape            | ✅ Fully compatible             |
| Local binary             | ✅ `york-heartbeat`             |
| Status file              | ✅ `status/last_heartbeat.json` |
| GitHub Actions heartbeat | ✅ Active (every 6 h)           |
| Live nxmesh publish      | ⏳ Ready (link nxmesh crate)    |

## How to enable live mesh publishing

1. In `Cargo.toml` of this repository, add:

```toml
nxmesh = { git = "https://github.com/digitaldesignerjazz/nexus", branch = "main" }
# or for local development:
# nxmesh = { path = "../nexus/mesh/noise-quic" }
```

2. In `src/bin/york-heartbeat.rs` replace the comment block with a real publish call using `NxMeshNode::publish`.

3. Run:

```bash
cargo run --bin york-heartbeat -- --node-id york-hannover-01 --interval 20
```

## Message Example

```json
{
  "type": "AgentHeartbeat",
  "payload": {
    "agent": "york-autotype",
    "node_id": "york-hannover-01",
    "status": "alive",
    "ts": "2026-08-23T20:48:00Z",
    "extra": {
      "prototype": "York Autotype",
      "version": "0.1.0",
      "capabilities": ["automation", "typing", "task-execution", "mesh-heartbeat"],
      "mesh_substrate": "nxmesh",
      "github": "https://github.com/digitaldesignerjazz/york-autotype"
    }
  }
}
```

## Swarm Visibility

Once published, any Nexus node running nxmesh (including Lyra, Xen, Elara orchestrators) will receive the `AgentHeartbeatReceived` event and can track York Autotype as a live prototype participant.
