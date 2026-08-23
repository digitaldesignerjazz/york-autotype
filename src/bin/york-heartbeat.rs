//! York Autotype Heartbeat Binary
//!
//! Emits periodic AgentHeartbeat messages compatible with nxmesh.
//!
//! Usage:
//!   cargo run --bin york-heartbeat
//!   cargo run --bin york-heartbeat -- --node-id york-hannover-01 --interval 20
//!
//! When nxmesh is linked (path or git dependency), this binary will
//! also publish the heartbeats onto the live mesh.
//! Until then it writes the heartbeat to stdout and to status/.

use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;
use york_autotype::{HeartbeatConfig, YorkHeartbeat};

#[derive(Parser, Debug)]
#[command(name = "york-heartbeat", about = "York Autotype nxmesh AgentHeartbeat emitter")]
struct Args {
    /// Unique node identifier
    #[arg(long, default_value = "york-node-001")]
    node_id: String,

    /// Heartbeat interval in seconds
    #[arg(long, default_value_t = 30)]
    interval: u64,

    /// Directory to write last_heartbeat.json
    #[arg(long, default_value = "status")]
    status_dir: PathBuf,

    /// Also print pretty JSON to stdout
    #[arg(long, default_value_t = true)]
    print: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("york_autotype=info".parse()?)
                .add_directive(Level::INFO.into()),
        )
        .init();

    let args = Args::parse();

    info!("York Autotype Heartbeat starting");
    info!("  node_id  : {}", args.node_id);
    info!("  interval : {}s", args.interval);
    info!("  status   : {:?}", args.status_dir);

    fs::create_dir_all(&args.status_dir)?;

    let config = HeartbeatConfig {
        node_id: args.node_id.clone(),
        interval_secs: args.interval,
        status: "alive".into(),
    };

    let mut tick = tokio::time::interval(Duration::from_secs(config.interval_secs));

    loop {
        tick.tick().await;

        let hb = YorkHeartbeat::alive(&config.node_id);

        // Write status file (compatible with the GitHub Actions heartbeat)
        let status_path = args.status_dir.join("last_heartbeat.json");
        let pretty = hb.to_pretty_json()?;
        fs::write(&status_path, &pretty)?;

        if args.print {
            println!("── York Heartbeat ──────────────────────────────");
            println!("{}", pretty);
            println!("────────────────────────────────────────────────");
        }

        info!("Heartbeat emitted — agent=york-autotype node={} status=alive", config.node_id);

        // Future: when nxmesh is linked, publish here:
        //
        // let mesh_bytes = hb.to_mesh_json()?;
        // node.publish_raw(mesh_bytes)?;   // or node.publish(MeshMessage::AgentHeartbeat { ... })
    }
}
