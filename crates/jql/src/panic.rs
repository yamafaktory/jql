use std::{
    panic,
    process::exit,
};

/// Use a custom hook to manage broken pipe errors.
/// See #86.
pub fn use_custom_panic_hook() {
    // Take the hook.
    let hook = panic::take_hook();

    // Register a custom panic hook.
    panic::set_hook(Box::new(move |panic_info| {
        let panic_message = panic_info.to_string();

        // Exit on broken pipe message.
        if panic_message.contains("Broken pipe") || panic_message.contains("os error 32") {
            exit(0);
        }

        // Hook back to default.
        (hook)(panic_info);
    }));
}
