use copypasta::{ClipboardContext, ClipboardProvider};

pub fn copy_to_clipboard(password: &str) -> String {
    if password.is_empty() {
        return "Error: No password to copy.".to_string();
    }

    let mut clipboard = ClipboardContext::new().unwrap_or_else(|_| {
        panic!("Failed to initialize clipboard");
    });

    if clipboard.set_contents(password.to_string()).is_ok() {
        "Password copied successfully to the clipboard!".to_string()
    } else {
        "Error: Failed to copy to the clipboard!".to_string()
    }
}