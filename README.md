# York Autotype

**Autonomous automation & typing prototype for the Nexus ecosystem**

York Autotype is a self-improving automation layer that executes tasks, generates structured output, and reports continuous presence via **nxmesh AgentHeartbeat** and GitHub status.

Part of the **Nexus / Esslinger & Co.** stack  
(alongside Soilnova, Vista Nova, Lumia, Grok Launcher, ElysiumOS / ElaraOS)

---

### Status

![Heartbeat](https://github.com/digitaldesignerjazz/york-autotype/actions/workflows/heartbeat.yml/badge.svg)

| Component                     | Status                          |
|-------------------------------|---------------------------------|
| Repository                    | Public, live                    |
| GitHub Actions Heartbeat      | Active (every 6 h)              |
| **nxmesh AgentHeartbeat**     | **Integrated & ready**          |
| `york-heartbeat` binary       | Available                       |
| Task execution engine         | Planned                         |
| Swarm collaboration           | Planned                         |

### Quick Start — Heartbeat

```bash
# Emit local heartbeats (writes status/last_heartbeat.json)
cargo run --bin york-heartbeat -- --node-id york-hannover-01 --interval 20
```

The produced messages are **fully compatible** with `nxmesh::MeshMessage::AgentHeartbeat`.

See [docs/MESH_HEARTBEAT.md](docs/MESH_HEARTBEAT.md) for the complete integration guide.

### Architecture

```
York Autotype
   │
   ├── GitHub Actions ──────────► status/last_heartbeat.json
   │
   └── nxmesh AgentHeartbeat ───► topic: nexus/mesh/v0
                                    (Lyra / Xen / Elara can see it)
```

### Links

- Parent ecosystem: [nexus](https://github.com/digitaldesignerjazz/nexus)
- Mesh substrate: [nxmesh](https://github.com/digitaldesignerjazz/nexus/tree/main/mesh/noise-quic)
- Integration docs: [MESH_HEARTBEAT.md](docs/MESH_HEARTBEAT.md)

### License

MIT OR Apache-2.0  
Esslinger & Co. / Nexus Initiative
