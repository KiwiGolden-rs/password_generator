use std::fs::{self, OpenOptions};
use std::io::{Write};
use std::path::{Path, PathBuf};
use rfd::FileDialog;
use serde_json::json;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum FileFormat {
    Txt,
    Csv,
    Json,
}

impl fmt::Display for FileFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let format_str = match self {
            FileFormat::Txt => "TXT",
            FileFormat::Csv => "CSV",
            FileFormat::Json => "JSON",
        };
        write!(f, "{}", format_str)
    }
}

impl Default for FileFormat {
    fn default() -> Self {FileFormat::Txt}
}

pub fn save_password(
    application_name: &str,
    password: &str,
    file_format: &FileFormat,
    save_path: Option<&str>,
) -> Result<(), std::io::Error> {
    let path: PathBuf = if let Some(path_str) = save_path {
        PathBuf::from(path_str)
    } else {
        FileDialog::new()
            .set_file_name(&format!("password.{}", file_format.to_string().to_lowercase()))
            .save_file()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "No file selected"))?
    };
    match file_format {
        FileFormat::Txt => save_txt(&path, application_name, password),
        FileFormat::Csv => save_csv(&path, application_name, password),
        FileFormat::Json => save_json(&path, application_name, password),
    }
}

fn save_txt(path: &Path, app: &str, password: &str) -> std::io::Result<()> {
    let entry = format!("{}: {}\n", app, password);
    OpenOptions::new().create(true).append(true).open(path)?.write_all(entry.as_bytes())
}

fn save_csv(path: &Path, app: &str, password: &str) -> std::io::Result<()> {
    let entry = format!("{}    {}\n", app, password);
    if path.exists() {
        OpenOptions::new().append(true).open(path)?.write_all(entry.as_bytes())
    } else {
        fs::write(path, format!("Application    Password\n{}", entry))
    }
}

fn save_json(path: &Path, app: &str, password: &str) -> std::io::Result<()> {
    let mut entries = if path.exists() {
        let content = fs::read_to_string(path)?;
        serde_json::from_str::<Vec<serde_json::Value>>(&content).unwrap_or_default()
    } else {
        Vec::new()
    };
    entries.push(json!({"application": app, "password": password}));

    fs::write(path, serde_json::to_string_pretty(&entries)?)
}