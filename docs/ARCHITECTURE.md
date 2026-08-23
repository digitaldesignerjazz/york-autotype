# York Autotype — Architecture

## Position in Nexus

```
Nexus Ecosystem
├── Mesh Substrate ........ nxmesh (Noise + QUIC / libp2p)
├── AI Swarm .............. Lyra · Xen · Elara
├── Blockchain ............ XCoin / QCoin / QNET Runes
└── Prototypes
    ├── Soilnova .......... sensing
    ├── Vista Nova ........ visualization
    ├── Lumia ............. lighting / display
    ├── Grok Launcher ..... UI / control plane
    └── York Autotype ..... automation & intelligent typing  ← this project
```

## Responsibilities

1. **Autonomous task execution**  
   Structured automation pipelines that can be triggered by agents or external events.

2. **Intelligent typing / generation**  
   Context-aware generation of text, configs, code snippets, reports.

3. **Presence & Heartbeat**  
   - Continuous GitHub Actions heartbeat (this repository)
   - **nxmesh `AgentHeartbeat`** messages for the live mesh (fully integrated)

4. **Swarm collaboration**  
   Accept tasks from Xen (technical) or Lyra (creative), report results back via mesh or GitHub.

## Heartbeat Flow (Current)

```
York Node
   │
   ├── GitHub Actions (every 6 h) ──► status/last_heartbeat.json
   │
   └── york-heartbeat binary ───────► AgentHeartbeat JSON
                                         │
                                         ├── written to status/
                                         └── ready for nxmesh publish
                                              topic: nexus/mesh/v0
```

When the `nxmesh` crate is linked, the same binary publishes live onto the mesh and becomes visible to Lyra, Xen and Elara.

## Next Evolution Steps

- [x] Message shape compatible with nxmesh
- [x] Local heartbeat binary
- [x] Documentation
- [ ] Link nxmesh crate and enable live publish
- [ ] Task queue + result reporting over mesh
- [ ] Integration with Elara / Lyra for creative automation loops
- [ ] Optional QNET incentive for completed automation jobs
