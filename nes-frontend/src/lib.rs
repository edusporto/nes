pub mod arch;
pub mod fps;
pub mod framework;
pub mod game;
pub mod gui;

/// Send events to a `Sender<T>`.
///
/// Example usage:
/// ```rust
/// let (tx, rx) = tokio::sync::mpsc::channel(50);
/// crate::event!(tx, |sender| {
///     sender.send(()).unwrap();
/// });
/// ```
macro_rules! event {
    ($tx:expr, |$sender:ident| $body:expr) => {
        let $sender = $tx.clone();
        crate::arch::spawn(async move { $body });
    };
}

pub(crate) use event;
