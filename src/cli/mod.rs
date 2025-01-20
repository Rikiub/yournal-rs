mod args;

use args::Cli;
use chrono::Utc;
use clap::Parser;
use cliclack::log;
use std::{fs, io, thread, time::Duration};

use crate::date::format_output;

/// `format!` with paris color formatting.
macro_rules! colorize {
    ($($arg:tt)*) => {
        paris::formatter::colorize_string(format!($($arg)*))
    }
}
extern crate paris;
pub(crate) use colorize;

fn log_canceled() -> io::Result<()> {
    log::error(colorize!("<red><bold><italic>Canceled"))?;
    Ok(())
}

pub fn run() -> io::Result<()> {
    let args = Cli::parse();

    let date = Utc::now();

    let output = args.output;
    let output = match format_output(date.date_naive(), &output) {
        Ok(path) => path,
        Err(error) => {
            log::error(colorize!(
                "<red><bold>Invalid output: {:?}\n<red><bold>{}",
                &output,
                error.to_string()
            ))?;
            return Err(error);
        }
    };

    // Create file if don't exists.
    if !output.exists() {
        log::step(colorize!("<blue><bold><italic>Creating new entry..."))?;

        // Handle template
        let template: String = match args.template {
            Some(template) => match fs::read_to_string(&template) {
                Ok(result) => result,
                Err(error) => {
                    match error.kind() {
                        io::ErrorKind::NotFound => {
                            log::warning(colorize!(
                                "<red><bold>Template path:</> {}\n<red><bold>Not founded",
                                template.to_string_lossy(),
                            ))?;
                        }
                        io::ErrorKind::IsADirectory => {
                            log::warning(colorize!(
                                "<red><bold>Template path:</> {}\n<red><bold>Is a directory, not a file",
                                template.to_string_lossy()
                            ))?;
                        }
                        _ => return Err(error),
                    };

                    if cliclack::confirm(colorize!("<yellow><bold><italic>Continue?"))
                        .initial_value(false)
                        .interact()?
                    {
                        log::warning(colorize!(
                            "<yellow><bold><italic>Creating without template...",
                        ))?;
                        thread::sleep(Duration::from_secs(1));
                    } else {
                        log_canceled()?;
                        return Err(error);
                    }

                    String::new()
                }
            },
            None => String::new(),
        };

        if let Err(error) = fs::write(&output, &template) {
            log::error(colorize!(
                "<red><bold>Unable to create file:</> {}\n<red><bold>{}",
                &output.to_string_lossy(),
                error.to_string()
            ))?;

            return Err(error);
        }
    }

    log::success(colorize!(
        "<blue><bold>Opening date:</> {} <bright-black>({})</>",
        date.format("%d %B, %Y"),
        output.file_name().unwrap_or_default().to_string_lossy()
    ))?;

    edit::edit_file(output)?;

    return Ok(());
}
