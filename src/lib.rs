//! pdfmonster as a library: the PDF engines behind every monster, the app and
//! the command line. The `pdfmonster` binary is a thin wrapper around
//! [`cli::main`], so other binaries can build on the same pieces.

pub mod app;
pub mod chain;
pub mod cli;
pub mod content;
pub mod deskew;
pub mod hush;
pub mod pages;
pub mod pdf;
pub mod peek;
pub mod render;
pub mod snip;
pub mod squish;
pub mod std_widths;
pub mod stitch;
pub mod watch;

/// What a build adds on top of the free app. The free `pdfmonster` binary adds nothing.
#[derive(Default)]
pub struct Extensions {
    /// Extra monsters, shown on the home screen after the pack.
    pub monsters: Vec<app::ExtraMonster>,
    /// Extra chain steps, offered after the free ones.
    pub steps: Vec<&'static dyn chain::ChainStep>,
    /// Extra subcommands, tried after the free ones.
    pub commands: Vec<cli::Command>,
    /// Extra menus, after File and View.
    pub menus: Vec<gpui::Menu>,
    /// Called once when the app starts, after the free app's actions and key
    /// bindings are set up: a place to add your own.
    pub setup: Option<fn(&mut gpui::App)>,
    pub branding: Branding,
}

/// What the app calls itself.
pub struct Branding {
    /// In the window title, the app menu and error messages.
    pub name: &'static str,
    /// The app id the desktop knows it by (Linux `.desktop` file, Wayland).
    /// A different id lets two builds be installed side by side.
    pub app_id: &'static str,
}

impl Default for Branding {
    fn default() -> Self {
        Branding { name: "pdfmonster", app_id: "pdfmonster" }
    }
}

/// A byte count for people: `1.6 KB`, `3.2 MB`.
pub fn human_size(bytes: u64) -> String {
    match bytes {
        b if b >= 1 << 20 => format!("{:.1} MB", b as f64 / (1 << 20) as f64),
        b if b >= 1 << 10 => format!("{:.0} KB", b as f64 / (1 << 10) as f64),
        b => format!("{b} B"),
    }
}
