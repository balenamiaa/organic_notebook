use std::assert_matches::debug_assert_matches;
use std::path::PathBuf;

pub struct PDFDocument {
    pub path: PathBuf,
}

pub struct NonPDFDocument {
    pub path: PathBuf,
}

impl PDFDocument {
    pub fn new(path: PathBuf) -> Self {
        debug_assert_eq!(path.extension(), Some("pdf".as_ref()));
        Self { path }
    }
}

impl NonPDFDocument {
    pub fn new(path: PathBuf) -> Self {
        if cfg!(debug_assertions) {
            let ext = path.extension().unwrap().to_string_lossy().to_string();
            debug_assert_matches!(ext.as_ref(), "pptx" | "ppt" | "docx" | "doc");
        }

        Self { path }
    }
}
