# York Autotype — Container Notes

## Build & Run (Docker or Podman)

```bash
# Build
podman build -t york-autotype:latest -f Containerfile .
# or
docker build -t york-autotype:latest -f Containerfile .

# Run
podman run -d \
  --name york-01 \
  -v york-status:/status \
  -e YORK_NODE_ID=york-hannover-01 \
  york-autotype:latest \
  --node-id york-hannover-01 --interval 20 --status-dir /status
```

The container emits AgentHeartbeat-compatible JSON into `/status/last_heartbeat.json`.

When nxmesh is later linked, the same binary can publish live onto the mesh.
