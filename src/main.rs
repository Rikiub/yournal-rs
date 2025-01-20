use std::process::ExitCode;

mod cli;
mod date;

fn main() -> ExitCode {
    match cli::run() {
        Ok(_) => return ExitCode::SUCCESS,
        Err(_) => return ExitCode::FAILURE,
    }
}
