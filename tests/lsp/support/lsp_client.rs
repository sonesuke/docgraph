#![allow(dead_code)]
use serde_json::{Value, json};
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicI64, Ordering};
use tokio::sync::mpsc;
use tokio::time::{Duration, timeout};

pub struct LspClient {
    child: Child,
    stdin: std::process::ChildStdin,
    request_id: AtomicI64,
    sender: mpsc::Sender<Value>,
    receiver: mpsc::Receiver<Value>,
    reader_thread: Option<std::thread::JoinHandle<()>>,
    pending_notifications: Vec<Value>,
}

impl LspClient {
    pub async fn spawn(bin_path: &str, args: &[&str]) -> anyhow::Result<Self> {
        let mut child = Command::new(bin_path)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        let (tx, rx) = mpsc::channel(100);
        let reader_tx = tx.clone();

        let reader_thread = std::thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                // Read Content-Length
                let mut size = None;
                let mut buffer = String::new();

                // Read headers
                while reader.read_line(&mut buffer).unwrap() > 0 {
                    if buffer == "\r\n" {
                        break;
                    }
                    if let Some(len) = buffer
                        .strip_prefix("Content-Length: ")
                        .and_then(|s| s.trim().parse::<usize>().ok())
                    {
                        size = Some(len);
                    }
                    buffer.clear();
                }

                if let Some(len) = size {
                    let mut body_buf = vec![0; len];
                    if let (Ok(()), Ok(val)) = (
                        reader.read_exact(&mut body_buf),
                        serde_json::from_slice::<Value>(&body_buf),
                    ) {
                        let _ = reader_tx.blocking_send(val);
                    }
                } else {
                    // EOF or broken pipe
                    break;
                }
            }
        });

        Ok(Self {
            child,
            stdin,
            request_id: AtomicI64::new(1),
            sender: tx,
            receiver: rx,
            reader_thread: Some(reader_thread),
            pending_notifications: Vec::new(),
        })
    }

    pub async fn send_request(&mut self, method: &str, params: Value) -> anyhow::Result<Value> {
        let id = self.request_id.fetch_add(1, Ordering::SeqCst);
        let req = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });
        self.write(req)?;

        // Read messages until we get a response with the matching ID.
        // Notifications arriving in the meantime are buffered in `pending_notifications`.
        loop {
            let msg = match timeout(Duration::from_secs(5), self.receiver.recv()).await {
                Ok(Some(m)) => m,
                Ok(None) => anyhow::bail!("Channel closed"),
                Err(_) => anyhow::bail!("Timeout waiting for response to {}", method),
            };

            if let Some(res_id) = msg.get("id").and_then(|i| i.as_i64()) {
                if res_id == id {
                    return Ok(msg);
                } else {
                    // Response for other ID? unexpected in sequential tests
                    eprintln!(
                        "Received response for ID {} while waiting for {}",
                        res_id, id
                    );
                }
            } else {
                // Buffer notifications so `wait_notification` can retrieve them later.
                self.pending_notifications.push(msg);
            }
        }
    }

    pub fn write(&mut self, val: Value) -> anyhow::Result<()> {
        let body = serde_json::to_string(&val)?;
        let msg = format!("Content-Length: {}\r\n\r\n{}", body.len(), body);
        self.stdin.write_all(msg.as_bytes())?;
        self.stdin.flush()?;
        Ok(())
    }

    pub async fn send_notification(&mut self, method: &str, params: Value) -> anyhow::Result<()> {
        let req = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params
        });
        self.write(req)
    }

    pub async fn next_notification(&mut self) -> Option<Value> {
        // First check pending
        if !self.pending_notifications.is_empty() {
            return Some(self.pending_notifications.remove(0));
        }

        // Then read from channel
        match timeout(Duration::from_secs(5), self.receiver.recv()).await {
            Ok(Some(msg)) => {
                if msg.get("id").is_none() {
                    Some(msg)
                } else {
                    eprintln!("Unexpected response in next_notification: {:?}", msg);
                    None
                }
            }
            _ => None,
        }
    }

    pub async fn wait_notification(
        &mut self,
        method: &str,
        dur: Duration,
    ) -> anyhow::Result<Value> {
        let start = std::time::Instant::now();
        loop {
            if start.elapsed() > dur {
                anyhow::bail!("Timeout waiting for notification: {}", method);
            }

            // Check if present in pending BEFORE reading new
            if let Some(idx) = self
                .pending_notifications
                .iter()
                .position(|val| val.get("method").and_then(|m| m.as_str()) == Some(method))
            {
                return Ok(self.pending_notifications.remove(idx));
            }

            // Read new message
            let tick = Duration::from_millis(100);
            match timeout(tick, self.receiver.recv()).await {
                Ok(Some(msg)) => {
                    if msg.get("id").is_some() {
                        eprintln!(
                            "Warning: received unexpected response while waiting for notification"
                        );
                        continue;
                    }

                    if msg.get("method").and_then(|m| m.as_str()) == Some(method) {
                        return Ok(msg);
                    } else {
                        // Other notification, buffer it
                        self.pending_notifications.push(msg);
                    }
                }
                Ok(None) => anyhow::bail!("Channel closed"),
                Err(_) => {
                    // Timeout on recv, just loop to check overall timeout
                    continue;
                }
            }
        }
    }
}

impl Drop for LspClient {
    fn drop(&mut self) {
        // stdin is closed automatically when dropped, signaling EOF to the server.

        // Try to wait gracefully for a short time
        let start = std::time::Instant::now();
        while start.elapsed() < std::time::Duration::from_millis(100) {
            if let Ok(Some(_)) = self.child.try_wait() {
                // Process has exited
                if let Some(handle) = self.reader_thread.take() {
                    let _ = handle.join();
                }
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        // If still running, kill it forcefully
        if let Ok(None) = self.child.try_wait() {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }

        // Join the reader thread (don't block indefinitely during test cleanup)
        if let Some(handle) = self.reader_thread.take() {
            let _ = handle.join();
        }
    }
}
