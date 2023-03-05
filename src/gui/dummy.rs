use std::fmt::{Debug, Display};

pub fn set_gui_panic_hook() {

}

pub fn enable_panic_gui_current_thread(_: bool) {

}

pub fn retryable<T, E: Display + Debug>(title: &str, mut func: impl FnMut() -> Result<T, E>) -> T {
    func().expect(title)
}