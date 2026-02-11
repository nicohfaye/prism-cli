mod app;
mod config;
mod k8s;
mod ssh;
mod ui;

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
#[command(name = "prism", about = "K8s cluster monitor over SSH")]
struct Cli {
    /// Launch with sample data (no connection required)
    #[arg(long)]
    demo: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.demo {
        return run_demo().await;
    }

    eprintln!("{}", ui::theme::BANNER);

    // Load configuration.
    let cfg = config::Config::load().context(
        "Could not load config. Create cli/config.toml — see config.example.toml for format.",
    )?;

    // Establish SSH tunnel (prints before TUI takes over).
    eprintln!(
        "  Connecting to {}@{}:{}...",
        cfg.ssh.user, cfg.ssh.host, cfg.ssh.port
    );
    let tunnel =
        ssh::SshTunnel::establish(&cfg.ssh, cfg.kubernetes.local_port, cfg.kubernetes.api_port)
            .await
            .context("Failed to establish SSH tunnel")?;
    eprintln!("  Tunnel ready on localhost:{}", cfg.kubernetes.local_port);

    // Build Kubernetes client through the tunnel.
    let client = k8s::build_client(&cfg)
        .await
        .context("Failed to connect to Kubernetes API")?;
    eprintln!("  Connected to cluster. Launching dashboard...\n");

    // Enter TUI only after connection is established.
    let mut terminal = ratatui::init();
    let mut app = app::App::new();
    let result = app.run(&mut terminal, &client).await;

    // Restore terminal and clean up.
    ratatui::restore();
    tunnel.close().await.ok();

    result
}

async fn run_demo() -> Result<()> {
    eprintln!("{}", ui::theme::BANNER);
    eprintln!("  Running in demo mode...\n");

    // Brief pause so the banner is visible before TUI takes over.
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let mut terminal = ratatui::init();
    let mut app = app::App::new();
    app.pods = vec![
        k8s::PodInfo {
            name: "nginx-7b8d6c5d9-x4k2m".into(),
            namespace: "default".into(),
            status: "Running".into(),
            restarts: 0,
            age: "2d".into(),
        },
        k8s::PodInfo {
            name: "redis-master-0".into(),
            namespace: "default".into(),
            status: "Running".into(),
            restarts: 1,
            age: "5d".into(),
        },
        k8s::PodInfo {
            name: "api-gateway-6f7d8c9-q8n3p".into(),
            namespace: "backend".into(),
            status: "Running".into(),
            restarts: 0,
            age: "12h".into(),
        },
        k8s::PodInfo {
            name: "worker-batch-j7k2x".into(),
            namespace: "jobs".into(),
            status: "Succeeded".into(),
            restarts: 0,
            age: "3h".into(),
        },
        k8s::PodInfo {
            name: "postgres-0".into(),
            namespace: "database".into(),
            status: "Running".into(),
            restarts: 0,
            age: "14d".into(),
        },
        k8s::PodInfo {
            name: "cronjob-cleanup-f9z1l".into(),
            namespace: "jobs".into(),
            status: "CrashLoopBackOff".into(),
            restarts: 12,
            age: "1h".into(),
        },
        k8s::PodInfo {
            name: "monitoring-agent-2v8x4".into(),
            namespace: "monitoring".into(),
            status: "Pending".into(),
            restarts: 0,
            age: "5m".into(),
        },
    ];
    app.deployments = vec![
        k8s::DeploymentInfo {
            name: "nginx".into(),
            namespace: "default".into(),
            ready: "3/3".into(),
            up_to_date: 3,
            age: "2d".into(),
        },
        k8s::DeploymentInfo {
            name: "api-gateway".into(),
            namespace: "backend".into(),
            ready: "2/2".into(),
            up_to_date: 2,
            age: "12h".into(),
        },
        k8s::DeploymentInfo {
            name: "redis".into(),
            namespace: "default".into(),
            ready: "1/1".into(),
            up_to_date: 1,
            age: "5d".into(),
        },
        k8s::DeploymentInfo {
            name: "postgres".into(),
            namespace: "database".into(),
            ready: "1/1".into(),
            up_to_date: 1,
            age: "14d".into(),
        },
        k8s::DeploymentInfo {
            name: "monitoring-agent".into(),
            namespace: "monitoring".into(),
            ready: "0/1".into(),
            up_to_date: 0,
            age: "5m".into(),
        },
    ];

    let result = app.run_demo(&mut terminal).await;
    ratatui::restore();

    result
}
