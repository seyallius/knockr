# knockr

> Keeps knocking until the endpoint gives the answer you're waiting for.  
> Sends the signal, stays out of your way.

A tiny Rust CLI that polls a URL and notifies you (via desktop notification + terminal bell) the **instant** it receives
a response that meets your success condition.

## Why?

I needed to buy a train ticket from `x.y`, but the site kept returning **504 Gateway Timeout**.  
I didn't want to manually refresh the browser tab every few minutes while coding – so `knockr` does it for me.

## How it works

- Polls the given URL with exponential backoff (2s → up to 60s).
- By default, it considers **any HTTP status code other than 504** as "success" and notifies you.
- You can easily change the condition to suit other sites (e.g., check for a specific HTTP 200, or look for a keyword in
  the response body).

## Build & Run

```bash
cargo build --release
./target/release/knockr https://x.y/
```

## Customisation

Edit the `is_success()` function in `src/main.rs` to define your own success criteria.

### Example for alibaba.ir

The site returns `200 OK` even when the train search fails, with an error message in the HTML:

> "<some_message>"

To ignore that and only notify when real results appear, use this condition:

```rust
fn is_success(status: u16, body: &str) -> bool {
    if status == 504 { return false; }
    // If the error message is present, treat as "not ready"
    if body.contains("<some_message>") {
        return false;
    }
    true
}
```

## Notification

- On **Linux**: uses `notify-send` (libnotify).
- On **macOS**: uses `osascript`.
- **Fallback**: terminal bell (`\x07`) – works everywhere.

## License

Do whatever you want with it – it’s just a personal tool.
