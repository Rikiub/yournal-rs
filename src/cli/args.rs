use clap::{
    builder::styling::{AnsiColor as Ansi, Styles},
    command, Parser,
};
use std::{ffi::OsString, path::PathBuf};

#[derive(Parser)]
#[command(styles=STYLES)]
pub struct Cli {
    #[arg(short, long, default_value = default_output(), help = "Output format where save notes.")]
    pub output: PathBuf,

    #[arg(short, long, help = "Template file to use.")]
    pub template: Option<PathBuf>,
}

fn default_output() -> OsString {
    let mut current = PathBuf::from("");
    current.push(DEFAULT_TEMPLATE);
    return current.into_os_string();
}

const DEFAULT_TEMPLATE: &str = "%Y-%m-%d.md";
const STYLES: Styles = Styles::styled()
    .header(Ansi::Cyan.on_default().bold())
    .usage(Ansi::Cyan.on_default().bold())
    .literal(Ansi::Blue.on_default().bold())
    .placeholder(Ansi::Blue.on_default());
