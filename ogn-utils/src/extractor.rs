use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::anyhow;

use crate::documents::PDFDocument;
use crate::result::{Error, Result};

pub const PDF2TEXT_PATH: &'static str = std::env!("PDF2TEXT_PATH");

pub trait TextExtractor
where
    Self: Sized,
{
    fn extract(&self) -> Result<Vec<String>>;
    fn open(filepath: impl AsRef<Path>) -> Result<Self>;
}

impl TextExtractor for PDFDocument {
    fn extract(&self) -> Result<Vec<String>> {
        let mut page_num = 1;
        let mut result = vec![];
        loop {
            let mut command = Command::new(&PDF2TEXT_PATH);
            command
                .arg("-layout")
                .arg("-f")
                .arg(page_num.to_string())
                .arg("-l")
                .arg(page_num.to_string())
                .arg(&self.path)
                .arg("-")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            let output = command.spawn()?.wait_with_output()?;

            let status = output.status.code().unwrap_or(0);

            if status != 0 {
                if String::from_utf8_lossy(&output.stderr).contains("page range") {
                    break;
                } else {
                    return Err(Error::new(anyhow!(
                        "pdf2text failed with status {}",
                        status
                    )));
                }
            } else {
                if output.stdout.is_empty() {
                    break;
                }
                let text = String::from_utf8_lossy(&output.stdout).to_string();
                let text_normalized = text.trim();

                result.push(text_normalized.to_string());
            }

            page_num += 1;
        }

        Ok(result)
    }

    fn open(filepath: impl AsRef<Path>) -> Result<Self> {
        Ok(PDFDocument {
            path: filepath.as_ref().to_path_buf(),
        })
    }
}
