mod notify;

use anyhow::Result;
use reqwest::blocking::Client;
use std::{cmp, thread, time::{Duration, Instant}};

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
    println!("(Will notify when status is NOT 504)\n");

    let start = Instant::now();

    loop {
        let response = client.get(url).send();

        match response {
            Ok(resp) => {
                let status = resp.status().as_u16();
                println!("[{:.1}s] Status: {}", start.elapsed().as_secs_f64(), status);

                // ----- CHANGE THIS CONDITION AS NEEDED -----
                // Option 1: break on ANY non‑504 (default)
                if status != 504 {
                    loop {
                        notify::notify("Server is back!", &format!("Status: {}", status));
                        println!("🎉 Server responded with {}. Exiting.", status);
                        thread::sleep(Duration::from_secs(5))
                        // return Ok(());
                    }
                }

                // Option 2: break ONLY on 200 OK (uncomment and comment the above)
                /*
                if status == 200 {
                    loop {
                        notify::notify("Server is back!", &format!("Status: {}", status));
                        println!("✅ Server returned 200 OK. Exiting.");
                        thread::sleep(Duration::from_secs(5))
                        // return Ok(());
                    }
                }
                */

                // Option 3: break on any 2xx or 3xx
                /*
                if status >= 200 && status < 400 {
                    notify(&format!("Server is back!"), &format!("Status: {}", status));
                    println!("✅ Server returned {}. Exiting.", status);
                    return Ok(());
                }
                */
            }
            Err(e) => {
                println!("[Error] {}", e);
            }
        }

        // Still 504 (or error) – wait with exponential backoff
        thread::sleep(backoff);
        backoff = cmp::min(backoff * 2, MAX_BACKOFF);
    }
}
