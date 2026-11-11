//! A build that adds to the free app through `Extensions`, the way
//! pdfmonsters-pro does: one extra monster, one chain step, one command and
//! its own name.
//!
//!     cargo run --example extended               # the app, with Echo on the home screen
//!     cargo run --example extended -- stamp a.pdf
//!     cargo run --example extended -- chain "munch > stamp" a.pdf

use std::{path::PathBuf, process::ExitCode};

use gpui::{ClickEvent, Context, FocusHandle, SharedString, Window, div, img, prelude::*, px};
use lopdf::{Document, Object, dictionary};
use pdfmonster::{
    Branding, Extensions,
    app::{self, ExtraMonster, MonsterScreen, pack::Kind},
    chain::ChainStep,
    cli::Command,
};

/// A monster screen that lists whatever is dropped on it.
struct Echo {
    focus: FocusHandle,
    files: Vec<PathBuf>,
}

impl Echo {
    fn new(cx: &mut Context<Self>) -> Self {
        Echo { focus: cx.focus_handle(), files: Vec::new() }
    }
}

impl MonsterScreen for Echo {
    fn focus_handle(&self) -> FocusHandle {
        self.focus.clone()
    }

    fn feed(&mut self, paths: Vec<PathBuf>, cx: &mut Context<Self>) {
        self.files.extend(paths);
        cx.notify();
    }
}

impl Render for Echo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let portrait = img(Kind::Peek.portrait()).w(px(34.)).h(px(35.));
        let clear = app::button("clear", "Clear", false).on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
            this.files.clear();
            cx.notify();
        }));
        let status: SharedString = format!("{} file{} dropped", self.files.len(), app::plural(self.files.len())).into();
        div()
            .track_focus(&self.focus)
            .size_full()
            .flex()
            .flex_col()
            .child(app::toolbar_with(portrait, "Echo", String::new(), [clear.into_any_element()]))
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .children(self.files.iter().map(|f| div().child(f.display().to_string()))),
            )
            .child(app::status_bar(status, false, Some("esc: all monsters")))
    }
}

/// A chain step that writes a note into the document's metadata.
struct Stamp;

impl ChainStep for Stamp {
    fn id(&self) -> &'static str {
        "stamp"
    }

    fn name(&self) -> &'static str {
        "Stamp"
    }

    fn job(&self) -> String {
        "marks it as checked".into()
    }

    fn color(&self) -> u32 {
        0xe8590c
    }

    fn run(&self, doc: &mut Document) -> anyhow::Result<String> {
        stamp(doc);
        Ok("Stamp marked it as checked".into())
    }
}

fn stamp(doc: &mut Document) {
    let info = doc.add_object(dictionary! { "Subject" => Object::string_literal("Checked") });
    doc.trailer.set("Info", info);
}

fn stamp_command(args: &[String]) -> anyhow::Result<()> {
    let [input] = args else { anyhow::bail!("usage: extended stamp <IN.pdf>") };
    let mut doc = Document::load(input)?;
    stamp(&mut doc);
    doc.save(input)?;
    println!("stamped {input}");
    Ok(())
}

fn main() -> ExitCode {
    pdfmonster::cli::main(Extensions {
        monsters: vec![ExtraMonster {
            name: "Echo",
            job: "Lists what you drop",
            blurb: "An example monster from another build. Drop anything on it.",
            color: 0xe8590c,
            portrait: || Kind::Peek.portrait(),
            takes_folders: true,
            open: |cx| app::open_screen(cx, Echo::new),
        }],
        steps: vec![&Stamp],
        commands: vec![Command {
            name: "stamp",
            usage: "extended stamp <IN.pdf>                    mark a PDF as checked",
            run: stamp_command,
        }],
        branding: Branding { name: "pdfmonster extended", app_id: "pdfmonster-extended" },
        ..Default::default()
    })
}
