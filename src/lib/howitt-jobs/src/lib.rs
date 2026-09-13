use howitt::jobs::{Job, QueueMessage};

#[cfg(target_arch = "wasm32")]
pub async fn enqueue(env: &worker::Env, job: Job) -> worker::Result<()> {
    env.queue("JOBS")?.send(QueueMessage::V1(job)).await
}

/// CLI publishing uses Cloudflare's API, never an unauthenticated Worker route.
/// Read credentials only for queue commands, not for unrelated CLI operations.
#[cfg(not(target_arch = "wasm32"))]
pub async fn enqueue(job: Job) -> anyhow::Result<()> {
    use anyhow::{Context, ensure};

    let account =
        std::env::var("CLOUDFLARE_ACCOUNT_ID").context("Missing CLOUDFLARE_ACCOUNT_ID")?;
    let queue = std::env::var("HOWITT_QUEUE_ID").context("Missing HOWITT_QUEUE_ID")?;
    let token = std::env::var("CLOUDFLARE_API_TOKEN").context("Missing CLOUDFLARE_API_TOKEN")?;
    ensure!(
        [&account, &queue]
            .iter()
            .all(|id| id.len() == 32 && id.bytes().all(|byte| byte.is_ascii_hexdigit())),
        "Cloudflare account and queue IDs must be 32 hexadecimal characters"
    );
    publish(
        &format!("https://api.cloudflare.com/client/v4/accounts/{account}/queues/{queue}/messages"),
        &token,
        job,
    )
    .await
}

#[cfg(not(target_arch = "wasm32"))]
async fn publish(url: &str, token: &str, job: Job) -> anyhow::Result<()> {
    let response = reqwest::Client::new()
        .post(url)
        .bearer_auth(token)
        .json(&serde_json::json!({
            "body": QueueMessage::V1(job),
            "content_type": "json"
        }))
        .send()
        .await?
        .error_for_status()?;
    let result: serde_json::Value = response.json().await?;
    anyhow::ensure!(
        result["success"] == true,
        "Cloudflare did not confirm queue publication"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{Read, Write},
        net::TcpListener,
    };

    #[tokio::test]
    async fn publishes_json_objects_and_requires_confirmed_success() {
        for (status, response_body, succeeds) in [
            (200, r#"{"success":true}"#, true),
            (200, r#"{"success":false}"#, false),
            (503, r#"{"success":true}"#, false),
        ] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let url = format!("http://{}/messages", listener.local_addr().unwrap());
            let server = std::thread::spawn(move || {
                let (mut socket, _) = listener.accept().unwrap();
                socket
                    .set_read_timeout(Some(std::time::Duration::from_secs(5)))
                    .unwrap();
                let mut bytes = Vec::new();
                let header_end = loop {
                    let mut byte = [0];
                    socket.read_exact(&mut byte).unwrap();
                    bytes.push(byte[0]);
                    if bytes.ends_with(b"\r\n\r\n") {
                        break bytes.len();
                    }
                };
                let headers = String::from_utf8(bytes.clone()).unwrap().to_lowercase();
                assert!(headers.contains("authorization: bearer synthetic-token\r\n"));
                let length: usize = headers
                    .lines()
                    .find_map(|line| line.strip_prefix("content-length: "))
                    .unwrap()
                    .parse()
                    .unwrap();
                bytes.resize(header_end + length, 0);
                socket.read_exact(&mut bytes[header_end..]).unwrap();
                let body: serde_json::Value = serde_json::from_slice(&bytes[header_end..]).unwrap();
                assert_eq!(
                    body,
                    serde_json::json!({"content_type": "json", "body": {
                        "version": "1", "job": {"Rwgps": {"SyncHistory": {"user_id": "USER#00000000-0000-0000-0000-000000000007"}}}
                    }})
                );
                write!(socket, "HTTP/1.1 {status} Test\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response_body}", response_body.len()).unwrap();
            });
            let job: Job = serde_json::from_value(serde_json::json!({"Rwgps": {"SyncHistory": {"user_id": "USER#00000000-0000-0000-0000-000000000007"}}})).unwrap();
            assert_eq!(
                publish(&url, "synthetic-token", job).await.is_ok(),
                succeeds
            );
            server.join().unwrap();
        }
    }
}
