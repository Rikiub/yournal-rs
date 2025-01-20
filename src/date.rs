use chrono::NaiveDate;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn format_output(date: NaiveDate, output: impl AsRef<Path>) -> io::Result<PathBuf> {
    let string = output.as_ref().to_str().unwrap();
    let format = date.format(string).to_string();
    let output = PathBuf::from(format);

    create_output(&output)?;

    Ok(output)
}

fn create_output(output: impl AsRef<Path>) -> io::Result<()> {
    let output = output.as_ref();

    fn except(msg: &str) -> io::Error {
        return io::Error::new(io::ErrorKind::InvalidInput, msg);
    }

    if let None = output.extension() {
        return Err(except("Should end with a file extension"));
    }

    match output.parent() {
        Some(p) => fs::create_dir_all(p)?,
        None => return Err(except("Cannot create the directory")),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    pub fn current_date() -> NaiveDate {
        return Utc::now().date_naive();
    }

    #[test]
    fn invalid_path() {
        if let Ok(_) = format_output(current_date(), "tests/") {
            assert!(false)
        }
    }

    #[test]
    fn permission_error() {
        if let Ok(_) = format_output(current_date(), "/error/error.md") {
            assert!(false)
        }
    }
}
