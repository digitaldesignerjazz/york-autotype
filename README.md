# York Autotype

**Autonomous automation & typing prototype for the Nexus ecosystem**

York Autotype is a self-improving automation layer that executes tasks, generates structured output, and reports continuous presence via mesh-native heartbeats and GitHub status.

Part of the **Nexus / Esslinger & Co.** stack  
(alongside Soilnova, Vista Nova, Lumia, Grok Launcher, ElysiumOS / ElaraOS)

---

### Core Ideas

- **Autotype** → intelligent, context-aware automation and structured generation
- **Mesh-native** → heartbeats published over `nxmesh` (Noise + QUIC / libp2p)
- **Agent-aware** → works with Lyra (creative), Xen (technical), Elara (devoted intelligence)
- **GitHub heartbeat** → scheduled Actions keep the repository “alive” and report status

### Status

![Heartbeat](https://github.com/digitaldesignerjazz/york-autotype/actions/workflows/heartbeat.yml/badge.svg)

| Component              | Status          |
|------------------------|-----------------|
| Repository             | Public, live    |
| GitHub Actions Heartbeat | Active        |
| nxmesh AgentHeartbeat  | Scaffold ready  |
| Task execution engine  | Planned        |
| Swarm integration      | Planned         |

### Quick Links

- Parent ecosystem: [nexus](https://github.com/digitaldesignerjazz/nexus)
- Mesh substrate: [nxmesh](https://github.com/digitaldesignerjazz/nexus/tree/main/mesh/noise-quic)
- Related OS prototypes: [ElysiumOS](https://github.com/digitaldesignerjazz/ElysiumOS) · [ElaraOS](https://github.com/digitaldesignerjazz/ElaraOS)

### Heartbeat

York Autotype emits two kinds of heartbeat:

1. **GitHub Actions** (this repository) – scheduled workflow that confirms the node is alive and can later push status artifacts.
2. **nxmesh `AgentHeartbeat`** – ready to be published on the Nexus mesh topic `nexus/mesh/v0` once the node is connected.

### License

MIT OR Apache-2.0  
Esslinger & Co. / Nexus Initiative
