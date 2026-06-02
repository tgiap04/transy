#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;

pub fn capture_text() -> Option<String> {
    #[cfg(target_os = "linux")]
    return linux::capture_linux();

    #[cfg(target_os = "macos")]
    return macos::capture_macos();

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    None
}
