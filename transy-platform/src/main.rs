use transy_core::capture_text;

fn main() {
    let Some(text) = capture_text() else {
        std::process::exit(0);
    };
    // TODO Phase 2: pass `text` to translation engine
    println!("Captured: {text}");
}
