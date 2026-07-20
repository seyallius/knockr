use notify_rust::Notification;

pub fn notify(title: &str, message: &str) {
    // Try desktop notification (Linux & macOS)
    if let Err(_) = Notification::new().summary(title).body(message).show() {
        // Fallback: terminal bell
        print!("\x07");
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }
}
