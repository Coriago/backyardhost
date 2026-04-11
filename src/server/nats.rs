use std::sync::OnceLock;
use std::time::Duration;

use async_nats::Client;
use tokio::process::{Child, Command};
use tokio::time::sleep;

/// Global NATS client singleton, initialized once on server startup.
static NATS_CLIENT: OnceLock<Client> = OnceLock::new();

/// NatsManager owns the spawned nats-server child process and the async-nats client.
pub struct NatsManager {
    child: Child,
}

impl Drop for NatsManager {
    fn drop(&mut self) {
        // start_kill is synchronous (non-async) and sends SIGKILL immediately.
        // This is a safety net for when the manager is moved out of the static
        // and dropped without an explicit async shutdown() call.
        if let Err(e) = self.child.start_kill() {
            eprintln!("[nats] Drop: failed to kill nats-server: {}", e);
        }
    }
}

impl NatsManager {
    /// Spawn nats-server, wait for it to be healthy, connect an async-nats client,
    /// and store it in the global `NATS_CLIENT` singleton.
    pub async fn start() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Spawn nats-server child process
        let child = Command::new("./target/nats-server")
            .args(["--config", "nats-server.conf"])
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| format!("Failed to spawn nats-server: {}", e))?;

        // Wait for NATS to be healthy (poll /healthz up to 10 seconds)
        let timeout = Duration::from_secs(10);
        let poll_interval = Duration::from_millis(200);
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > timeout {
                return Err("NATS server health check timed out after 10 seconds".into());
            }

            match reqwest_health_check().await {
                Ok(true) => break,
                _ => sleep(poll_interval).await,
            }
        }

        // Connect async-nats client
        let client = async_nats::connect("localhost:4222")
            .await
            .map_err(|e| format!("Failed to connect to NATS: {}", e))?;

        // Store in global singleton
        NATS_CLIENT
            .set(client)
            .map_err(|_| "NATS client already initialized")?;

        eprintln!("[nats] Server started and client connected");
        Ok(NatsManager { child })
    }

    /// Get a reference to the global NATS client (initialized by `start()`).
    pub fn client() -> &'static Client {
        NATS_CLIENT
            .get()
            .expect("NATS client not initialized — call NatsManager::start() first")
    }

    /// Gracefully shut down the nats-server child process.
    pub async fn shutdown(mut self) {
        eprintln!("[nats] Shutting down NATS server...");
        if let Err(e) = self.child.kill().await {
            eprintln!("[nats] Failed to kill nats-server: {}", e);
        }
        let _ = self.child.wait().await;
        eprintln!("[nats] NATS server stopped");
    }
}

/// Check NATS health via HTTP monitoring endpoint.
/// Returns Ok(true) when NATS + JetStream is ready.
async fn reqwest_health_check() -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    // Use tokio's net to make a simple HTTP GET to http://127.0.0.1:8222/healthz?js-enabled-only=true
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    let mut stream = TcpStream::connect("127.0.0.1:8222")
        .await
        .map_err(|e| format!("Connection refused: {}", e))?;

    let request = "GET /healthz?js-enabled-only=true HTTP/1.0\r\nHost: 127.0.0.1:8222\r\n\r\n";
    stream.write_all(request.as_bytes()).await?;

    let mut response = String::new();
    stream.read_to_string(&mut response).await?;

    Ok(response.contains("\"status\":\"ok\""))
}
