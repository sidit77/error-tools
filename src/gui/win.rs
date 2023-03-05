use std::fmt::{Debug, Display};
use std::iter::once;
use std::panic::{set_hook, take_hook};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use windows_sys::w;
use windows_sys::Win32::UI::WindowsAndMessaging::{IDRETRY, IDTRYAGAIN, MB_ICONERROR, MB_OK, MB_RETRYCANCEL, MessageBoxW};

thread_local! {
    static ENABLED: AtomicBool = AtomicBool::new(true);
}

pub fn set_gui_panic_hook() {
    let default_hook = take_hook();
    set_hook(Box::new(move |info| {
        default_hook(info);
        let enabled = ENABLED.try_with(|enabled| enabled.load(Ordering::SeqCst))
            .unwrap_or(false);
        if enabled {
            let thread = thread::current()
                .name()
                .unwrap_or("<unnamed>")
                .to_string();
            let msg = encode_wide(format!("thread '{}' {}", thread, info));
            unsafe {
                MessageBoxW(0, msg.as_ptr(), w!("Unexpected Panic"), MB_OK | MB_ICONERROR);
            }
        }
    }))
}

pub fn enable_panic_gui_current_thread(enabled: bool) {
    ENABLED.with(|inner| inner.store(enabled, Ordering::SeqCst));
}

pub fn retryable<T, E: Display + Debug>(title: &str, mut func: impl FnMut() -> Result<T, E>) -> T {
    loop {
        match func() {
            Ok(result) => break result,
            Err(err) => {
                #[cfg(feature = "log")]
                log::error!("{}: {}", title, err);

                let orig = encode_wide(title);
                let msg = encode_wide(err.to_string());

                unsafe {
                    match MessageBoxW(0, msg.as_ptr(), orig.as_ptr(), MB_RETRYCANCEL | MB_ICONERROR) {
                        IDRETRY | IDTRYAGAIN => continue,
                        _ => panic!("{}", title)
                    }
                }

            }
        }
    }

}

fn encode_wide<T: AsRef<str>>(string: T) -> Vec<u16> {
    string
        .as_ref()
        .encode_utf16()
        .chain(once(0))
        .collect()
}