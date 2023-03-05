#[cfg(target_os = "windows")]
#[path = "win.rs"]
mod platform;

#[cfg(not(target_os = "windows"))]
#[path = "dummy.rs"]
mod platform;

pub use platform::{set_gui_panic_hook, enable_panic_gui_current_thread, retryable};

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use std::fs::File;
    use crate::gui::{enable_panic_gui_current_thread, set_gui_panic_hook};
    use crate::gui::platform::retryable;

    #[test]
    #[serial]
    #[should_panic]
    fn test_panic() {
        set_gui_panic_hook();

        let _ = std::thread::spawn(|| {
            enable_panic_gui_current_thread(false);
            panic!("Helper thread")
        }).join();

        panic!("Main Thread");
    }

    #[test]
    #[serial]
    #[should_panic]
    fn test_retry() {
        let _ = retryable("can not open file", || File::open("hello.c"));
    }

}
