use std::process::Command;

pub fn get_mouse_position() -> (i32, i32) {
    #[cfg(target_os = "linux")]
    return get_mouse_linux();

    #[cfg(target_os = "macos")]
    return get_mouse_macos();

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    (0, 0)
}

#[cfg(target_os = "linux")]
fn get_mouse_linux() -> (i32, i32) {
    let Ok(output) = Command::new("xdotool")
        .args(["getmouselocation", "--shell"])
        .output()
    else {
        return (0, 0);
    };

    if !output.status.success() {
        return (0, 0);
    }
    parse_xdotool_output(&String::from_utf8_lossy(&output.stdout))
}

#[cfg(target_os = "linux")]
fn parse_xdotool_output(output: &str) -> (i32, i32) {
    let mut x = 0i32;
    let mut y = 0i32;
    for line in output.lines() {
        if let Some(val) = line.strip_prefix("X=") {
            x = val.trim().parse().unwrap_or(0);
        } else if let Some(val) = line.strip_prefix("Y=") {
            y = val.trim().parse().unwrap_or(0);
        }
    }
    (x, y)
}

#[cfg(target_os = "macos")]
fn get_mouse_macos() -> (i32, i32) {
    let Ok(output) = Command::new("osascript")
        .args([
            "-e",
            "tell application \"System Events\" to get position of mouse",
        ])
        .output()
    else {
        return (0, 0);
    };

    if !output.status.success() {
        return (0, 0);
    }
    parse_osascript_output(&String::from_utf8_lossy(&output.stdout))
}

#[cfg(target_os = "macos")]
fn parse_osascript_output(output: &str) -> (i32, i32) {
    let parts: Vec<&str> = output.trim().split(", ").collect();
    if parts.len() >= 2 {
        let x = parts[0].trim().parse().unwrap_or(0);
        let y = parts[1].trim().parse().unwrap_or(0);
        (x, y)
    } else {
        (0, 0)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "linux")]
    use super::parse_xdotool_output;

    #[cfg(target_os = "linux")]
    #[test]
    fn parse_xdotool_valid() {
        let output = "X=123\nY=456\nSCREEN=0\nWINDOW=12345\n";
        assert_eq!(parse_xdotool_output(output), (123, 456));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn parse_xdotool_invalid_returns_zero() {
        assert_eq!(parse_xdotool_output("garbage"), (0, 0));
    }
}
