mod app;
mod config;
mod dummy;
mod k8s;
mod ssh;
mod ui;

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
#[command(name = "prism", about = "K8s cluster monitor over SSH")]
struct Cli {
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

    // load config
    let cfg = config::Config::load().context(
        "Could not load config. Create cli/config.toml — see config.example.toml for format.",
    )?;

    // connect ssh tunnel
    eprintln!(
        "  Connecting to {}@{}:{}...",
        cfg.ssh.user, cfg.ssh.host, cfg.ssh.port
    );
    let tunnel =
        ssh::SshTunnel::establish(&cfg.ssh, cfg.kubernetes.local_port, cfg.kubernetes.api_port)
            .await
            .context("Failed to establish SSH tunnel")?;
    eprintln!("  Tunnel ready on localhost:{}", cfg.kubernetes.local_port);

    // build k8s client
    let client = k8s::build_client(&cfg)
        .await
        .context("Failed to connect to Kubernetes API")?;
    eprintln!("  Connected to cluster. Launching dashboard...\n");

    // enter tui after connection is up
    let mut terminal = ratatui::init();
    let mut app = app::App::new();
    let result = app.run(&mut terminal, &client).await;

    // restore & clean up
    ratatui::restore();
    tunnel.close().await.ok();

    result
}

async fn run_demo() -> Result<()> {
    eprintln!("{}", ui::theme::BANNER);
    eprintln!("  Running in demo mode...\n");

    // sleep for 1 sec to show banner in demo mode
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let mut terminal = ratatui::init();
    let mut app = app::App::new();
    app.pods = dummy::get_pods();
    app.deployments = dummy::get_deployments();

    let result = app.run_demo(&mut terminal).await;
    ratatui::restore();

    result
}
