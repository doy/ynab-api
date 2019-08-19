use snafu::{OptionExt, ResultExt};
use std::io::Read;

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    #[snafu(display("couldn't find config path for project {}", name))]
    FindConfigDir { name: String },

    #[snafu(display(
        "couldn't open file {}: {}",
        file.to_string_lossy(),
        source
    ))]
    OpenFile {
        file: std::path::PathBuf,
        source: std::io::Error,
    },

    #[snafu(display(
        "couldn't read file {}: {}",
        file.to_string_lossy(),
        source
    ))]
    ReadFile {
        file: std::path::PathBuf,
        source: std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

const PROJECT_NAME: &str = "ynab";

pub fn api_key() -> Result<std::path::PathBuf> {
    Ok(directories::ProjectDirs::from("", "", PROJECT_NAME)
        .with_context(|| FindConfigDir {
            name: PROJECT_NAME.to_string(),
        })?
        .config_dir()
        .join("api-key"))
}

pub fn read_api_key() -> Result<String> {
    let mut key = String::new();
    let key_file = api_key()?;
    std::fs::File::open(key_file.clone())
        .with_context(|| OpenFile {
            file: key_file.clone(),
        })?
        .read_to_string(&mut key)
        .with_context(|| ReadFile {
            file: key_file.clone(),
        })?;
    let key = key.trim();
    Ok(key.to_string())
}
