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
                        // In a real async test harness, we might block here if channel full,
                        // but for tests it is usually fine.
                        let _ = reader_tx.blocking_send(val);
                    }
                } else {
                    // EOF or broken pipe
                    // break; // For robustness, we might want to break, but let's keep retrying or just exit?
                    // If read_line returns 0, it is EOF.
                    // The above loop condition handles `> 0`. If it's 0, we exit outer loop?
                    // Wait, verify logic.
                    // If buffer is empty at start of loop body (after read_line), we check return value.
                    // Actually let's refine this loop slightly effectively.
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

        // Wait for response with matching ID
        // Note: This simple implementation effectively blocks other notifications while waiting for response.
        // For simple sequential tests, this is fine. For complex interleaved tests, need a better loop.
        // BUT, given the "helper method" request, we probably want to queue notifications separately?
        // The implementation below consumes messages. If it's a notification, we might lose it if we don't store it.
        // Wait... user suggestion said: "LspClient は前の骨格（Content-Lengthフレーミング＋pending response＋notif queue）でOK"
        // I should probably start a background tokio task to route messages, but `send_request` needs to be easy.

        // Let's implement a loop here that buffers notifications if they are not the response.
        // Wait, self.receiver only has one consumer (this test code).
        // Since we are inside `send_request`, we can loop `self.receiver`, if it is a notification, we buffer it (where?), if response matches, return.
        // BUT `buffer` ownership is tricky if we are mutable.

        // SIMPLIFICATION:
        // We will assume that `send_request` waits for response.
        // If we receive a notification while waiting, we can put it into an internal queue?
        // Ah, `self` is `&mut`.

        // Actually, for the tests, we often "wait for notification" OR "send request".
        // But if `publishDiagnostics` arrives exactly while we await `hover` response, we must not drop it.
        // So we need a shared `notification_queue`.

        // Refactoring:
        // We really want `next_response(id)` and `next_notification()`.
        // To make this robust without rigorous background tasks in the Test struct itself (which is async):
        // We can just Peek? No mpsc is not peekable.

        // Let's rely on the simple pattern:
        // Reads from `receiver` loop.
        // If msg.id == id -> Return.
        // If msg has no id (notification) -> Push to internal `pending_notifications`.
        // If msg has other id -> Panic (or buffer? usually means error in test logic).

        loop {
            // Check pending notifications first? No, we are looking for response.

            // Read from channel
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
                // It is a notification (no id)
                // We must store it so `wait_notification` can find it.
                // But `self` is borrowed mutably here.
                // We can't easily push to a field if we don't have one or if strict borrowck.
                // Let's add `pending_notifications` field to struct.
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
                    // Unexpected response?
                    eprintln!("Unexpected response in next_notification: {:?}", msg);
                    None // or Loop?
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
                        // Uh oh, response arriving uninvited. Buffer it?
                        // Implementation simplified: we won't handle uninvited responses here well.
                        eprintln!("Warning: Received uninvited response");
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

// Add the field
// We need to re-declare struct with all fields
