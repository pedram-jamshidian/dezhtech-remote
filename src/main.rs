// Specify the windows subsystem to eliminate console window.
// Requires nightly.
// XXX: the main entry point is in lib.rs
#![cfg_attr(
    all(
        not(debug_assertions),
        target_os = "windows",
        not(feature = "cli")
    ),
    windows_subsystem = "windows"
)]

fn main() {
    librustdesk::core_main::core_main();
}
