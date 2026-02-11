use anyhow::{Context, Result};
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Pod;
use kube::api::ListParams;
use kube::config::{Config, Kubeconfig};
use kube::{Api, Client};
use std::time::SystemTime;

use crate::config::Config as PrismConfig;

/// Summary of a pod for display.
#[derive(Clone, Debug)]
pub struct PodInfo {
    pub name: String,
    pub namespace: String,
    pub status: String,
    pub restarts: i32,
    pub age: String,
}

/// Summary of a deployment for display.
#[derive(Clone, Debug)]
pub struct DeploymentInfo {
    pub name: String,
    pub namespace: String,
    pub ready: String,
    pub up_to_date: i32,
    pub age: String,
}

/// Build a kube Client that connects through the SSH tunnel.
pub async fn build_client(config: &PrismConfig) -> Result<Client> {
    let kubeconfig_path = config.kubeconfig_path();
    let kubeconfig = Kubeconfig::read_from(&kubeconfig_path)
        .with_context(|| format!("Failed to read kubeconfig at {}", kubeconfig_path.display()))?;

    let mut kube_config =
        Config::try_from(kubeconfig).context("Failed to build kube config from kubeconfig file")?;

    // Route through the SSH tunnel.
    let tunnel_url = format!("https://127.0.0.1:{}", config.kubernetes.local_port);
    kube_config.cluster_url = tunnel_url.parse().context("Invalid tunnel URL")?;

    // The K8s API cert won't match localhost — the SSH tunnel provides security.
    kube_config.accept_invalid_certs = true;

    Client::try_from(kube_config).context("Failed to create Kubernetes client")
}

/// Get the current Unix epoch in seconds.
fn now_epoch_secs() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

/// Compute a human-readable age string from a k8s-openapi jiff Timestamp.
fn age_from_timestamp(ts: &k8s_openapi::apimachinery::pkg::apis::meta::v1::Time) -> String {
    let created = ts.0.as_second();
    let elapsed = now_epoch_secs() - created;
    format_duration(elapsed)
}

/// Fetch all pods across all namespaces.
pub async fn fetch_pods(client: &Client) -> Result<Vec<PodInfo>> {
    let pods: Api<Pod> = Api::all(client.clone());
    let list = pods
        .list(&ListParams::default())
        .await
        .context("Failed to list pods")?;

    let infos = list
        .items
        .into_iter()
        .map(|pod| {
            let meta = &pod.metadata;
            let status = pod.status.as_ref();

            let phase = status
                .and_then(|s| s.phase.clone())
                .unwrap_or_else(|| "Unknown".into());

            let restarts = status
                .and_then(|s| s.container_statuses.as_ref())
                .map(|cs| cs.iter().map(|c| c.restart_count).sum())
                .unwrap_or(0);

            let age = meta
                .creation_timestamp
                .as_ref()
                .map(|ts| age_from_timestamp(ts))
                .unwrap_or_else(|| "-".into());

            PodInfo {
                name: meta.name.clone().unwrap_or_default(),
                namespace: meta.namespace.clone().unwrap_or_else(|| "default".into()),
                status: phase,
                restarts,
                age,
            }
        })
        .collect();

    Ok(infos)
}

/// Fetch all deployments across all namespaces.
pub async fn fetch_deployments(client: &Client) -> Result<Vec<DeploymentInfo>> {
    let deploys: Api<Deployment> = Api::all(client.clone());
    let list = deploys
        .list(&ListParams::default())
        .await
        .context("Failed to list deployments")?;

    let infos = list
        .items
        .into_iter()
        .map(|dep| {
            let meta = &dep.metadata;
            let status = dep.status.as_ref();

            let ready = status.and_then(|s| s.ready_replicas).unwrap_or(0);
            let desired = dep.spec.as_ref().and_then(|s| s.replicas).unwrap_or(0);
            let up_to_date = status.and_then(|s| s.updated_replicas).unwrap_or(0);

            let age = meta
                .creation_timestamp
                .as_ref()
                .map(|ts| age_from_timestamp(ts))
                .unwrap_or_else(|| "-".into());

            DeploymentInfo {
                name: meta.name.clone().unwrap_or_default(),
                namespace: meta.namespace.clone().unwrap_or_else(|| "default".into()),
                ready: format!("{}/{}", ready, desired),
                up_to_date,
                age,
            }
        })
        .collect();

    Ok(infos)
}

/// Format elapsed seconds into a human-friendly string like "2d", "5h", "13m".
fn format_duration(secs: i64) -> String {
    let secs = secs.max(0);
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m", secs / 60)
    } else if secs < 86400 {
        format!("{}h", secs / 3600)
    } else {
        format!("{}d", secs / 86400)
    }
}
