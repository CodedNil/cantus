use cantus::app::{run, trigger_launcher};
use std::env;

fn main() {
    if env::args().any(|arg| arg == "--launcher") {
        trigger_launcher();
    }
    run();
}
