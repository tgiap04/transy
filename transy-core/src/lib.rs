pub mod capture;
pub mod translate;
pub use capture::capture_text;
pub use translate::{TranslationError, translate};

use tokio::runtime::Builder;

pub fn block_on<F: std::future::Future<Output = T>, T>(future: F) -> T {
    Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime")
        .block_on(future)
}
