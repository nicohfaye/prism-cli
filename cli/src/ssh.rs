use anyhow::{Context, Result};
use std::process::Stdio;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::process::{Child, Command};
use tokio::time::{sleep, timeout};

use crate::config::SshConfig;

pub struct SshTunnel {
    child: Child,
}

impl SshTunnel {
    /// Create an ssh tunnel that forwards `local_port` to `remote_host:remote_port`.
    pub async fn establish(ssh: &SshConfig, local_port: u16, remote_port: u16) -> Result<Self> {
        let forward = format!("{}:localhost:{}", local_port, remote_port);

        let mut cmd = Command::new("ssh");
        cmd.arg("-N") // no remote command
            .arg("-L")  // bind/forward ports?
            .arg(&forward)
            .arg("-p")  // port to connect to on remote host
            .arg(ssh.port.to_string())
            .arg("-o")
            .arg("StrictHostKeyChecking=accept-new")
            .arg("-o")
            .arg("ConnectTimeout=10");

        if let Some(ref key) = ssh.key_path {
            let expanded = if let Some(stripped) = key.strip_prefix("~/") {
                dirs::home_dir()
                    .map(|h| h.join(stripped).to_string_lossy().into_owned())
                    .unwrap_or_else(|| key.clone())
            } else {
                key.clone()
            };
            cmd.arg("-i").arg(expanded);
        }

        cmd.arg(format!("{}@{}", ssh.user, ssh.host));

        // capture stderr to prevent leaks into tui
        cmd.stderr(Stdio::piped());
        cmd.stdout(Stdio::null());

        // should not outlive app
        cmd.kill_on_drop(true);

        let child = cmd
            .spawn()
            .context("Failed to spawn ssh process. Is ssh installed?")?;

        let mut tunnel = SshTunnel { child };

        // wait until the local port is reachable.
        if let Err(e) = tunnel.wait_for_port(local_port).await {
            let stderr_msg = tunnel.read_stderr().await;
            let detail = if stderr_msg.is_empty() {
                e.to_string()
            } else {
                stderr_msg
            };
            anyhow::bail!("SSH tunnel failed: {}", detail.trim());
        }

        Ok(tunnel)
    }

    /// Probe the local port until it accepts a TCP connection.
    async fn wait_for_port(&self, port: u16) -> Result<()> {
        let addr = format!("127.0.0.1:{}", port);
        let total_timeout = Duration::from_secs(15);

        timeout(total_timeout, async {
            loop {
                if TcpStream::connect(&addr).await.is_ok() {
                    return Ok(());
                }
                sleep(Duration::from_millis(250)).await;
            }
        })
        .await
        .map_err(|_| anyhow::anyhow!("SSH tunnel did not become ready within 15s"))?
    }

    /// Try to read any stderr output from the ssh process.
    async fn read_stderr(&mut self) -> String {
        if let Some(mut stderr) = self.child.stderr.take() {
            let mut buf = String::new();
            stderr.read_to_string(&mut buf).await.ok();
            buf
        } else {
            String::new()
        }
    }

    /// Shut down the tunnel.
    pub async fn close(mut self) -> Result<()> {
        self.child.kill().await.ok();
        Ok(())
    }
}

impl Drop for SshTunnel {
    fn drop(&mut self) {
        // idk
        #[allow(unused_must_use)]
        {
            self.child.start_kill().ok();
        }
    }
}
