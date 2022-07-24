use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::anyhow;

use crate::documents::PDFDocument;
use crate::result::{Error, Result};

pub trait TextExtractor
where
    Self: Sized,
{
    fn extract(&self) -> Result<Vec<String>>;
    fn open(filepath: impl AsRef<Path>) -> Result<Self>;
}

impl TextExtractor for PDFDocument {
    fn extract(&self) -> Result<Vec<String>> {
        let pdf2text_path = std::env::var("PDF2TEXT_PATH")?;

        let mut page_num = 1;
        let mut result = vec![];
        loop {
            let mut command = Command::new(&pdf2text_path);
            command
                .arg("-layout")
                .arg("-f")
                .arg(page_num.to_string())
                .arg("-l")
                .arg(page_num.to_string())
                .arg(&self.path)
                .arg("-")
                .stdout(Stdio::piped());
            let output = command.spawn()?.wait_with_output()?;

            let status = output.status.code().unwrap_or(0);

            if status != 0 {
                return Err(Error::new(anyhow!(
                    "pdf2text failed with status {}",
                    status
                )));
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
