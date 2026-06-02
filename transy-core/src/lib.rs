pub mod capture;
pub mod translate;
pub use capture::capture_text;
pub use translate::{TranslationError, translate};
