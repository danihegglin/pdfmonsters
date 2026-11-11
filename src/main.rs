// Release builds on Windows are GUI apps, so no console window opens behind the app.
// The CLI reattaches to the terminal it was started from (see `attach_parent_console`).
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

fn main() -> std::process::ExitCode {
    pdfmonster::cli::main(pdfmonster::Extensions::default())
}
