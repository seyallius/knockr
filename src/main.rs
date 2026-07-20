mod notify;

use anyhow::Result;
use reqwest::blocking::Client;
use std::{
    cmp, thread,
    time::{Duration, Instant},
};

/// -------- CUSTOMISE THIS FUNCTION --------
/// Returns true if the response is considered "successful".
/// Default: anything except 504 is good.
/// For alibaba.ir, we also check that the error message is NOT present.
fn is_success(status: u16, body: &str) -> bool {
    // 1. Gateway timeout → definitely not ready
    if status == 504 {
        return false;
    }

    // 2. Anything not 200-300 range -> definitely not ready
    if status < 200 && status > 400 {
        return false;
    }

    // 3. message-specific: if the body contains the error message,
    //    treat it as "still unavailable" even though status is 200.
    if body.contains("<some_message>") {
        return false;
    }

    // 4. Everything else → success (you can tighten this to `status == 200` if you want)
    true
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <URL>", args[0]);
        std::process::exit(1);
    }
    let url = &args[1];

    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    let mut backoff = Duration::from_secs(2);
    const MAX_BACKOFF: Duration = Duration::from_secs(60);

    println!("🔍 Monitoring {} until it becomes available...", url);
    println!("(Will notify when condition is met)\n");

    let start = Instant::now();

    loop {
        println!();
        println!("Requesting URL: <{url}>");
        let response = client.get(url).send();

        match response {
            Ok(resp) => {
                let status = resp.status().as_u16();
                // Read the body (we need it for alibaba's check)
                let body = match resp.text() {
                    Ok(t) => t,
                    Err(e) => {
                        println!("[Error reading body] {}", e);
                        // If we can't read the body, treat as failure (retry)
                        thread::sleep(backoff);
                        backoff = cmp::min(backoff * 2, MAX_BACKOFF);
                        continue;
                    }
                };

                println!("[{:.1}s] Status: {}", start.elapsed().as_secs_f64(), status);

                if is_success(status, &body) {
                    for _ in 0..5 {
                        notify::notify(
                            &format!("🎉 Knockr: Server [{url}] is back!"),
                            &format!("Status: {}", status),
                        );
                        println!("✅ Success condition met. Exiting.");
                        // return Ok(());
                    }
                }
            }
            Err(e) => {
                eprintln!("[Error] {}", e);
            }
        }

        // Still not successful – wait with exponential backoff
        println!("Will wait {:?} before next retry", backoff);
        thread::sleep(backoff);
        backoff = cmp::min(backoff * 2, MAX_BACKOFF);
    }
}
