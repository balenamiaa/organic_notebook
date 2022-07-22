use std::error::Error as StdError;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};

use anyhow::anyhow;

#[derive(Debug)]
pub struct Error {
    inner: anyhow::Error,
}

trait ErrorWrapper {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

impl<T> ErrorWrapper for T
    where
        T: std::error::Error + Send + Sync + 'static,
{
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        StdError::source(self)
    }
}

impl<T: ErrorWrapper + Sync + Send + 'static> From<T> for Error {
    fn from(e: T) -> Self {
        Self {
            inner: if let Some(x) = e.source() {
                anyhow!("{}", x)
            } else {
                anyhow!("text extraction error")
            },
        }
    }
}

impl From<Error> for anyhow::Error {
    fn from(e: Error) -> Self {
        e.inner
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct PDFDocument {
    path: PathBuf,
}

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
                return Err(Error {
                    inner: anyhow!("pdf2text failed with status {}", status),
                });
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
