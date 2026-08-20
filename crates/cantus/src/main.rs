use cantus::app::{
    platform::{Current as Platform, Platform as _},
    run,
};
use std::env;

fn main() {
    if env::args().any(|arg| arg == "--launcher") {
        Platform::trigger_launcher();
    }
    run();
}
