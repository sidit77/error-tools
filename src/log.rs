use std::fmt::Display;

pub trait LogResultExt<T> {
    fn log(self, msg: &str) -> Self;
    fn log_ok(self, msg: &str) -> Option<T>;
}

impl<T, E: Display> LogResultExt<T> for Result<T, E> {
    fn log(self, msg: &str) -> Self {
        if let Err(err) = &self {
            log::warn!("{}: {}", msg, err);
        }
        self
    }

    fn log_ok(self, msg: &str) -> Option<T> {
        self.log(msg).ok()
    }
}
