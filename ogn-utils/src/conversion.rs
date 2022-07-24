use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use futures::{FutureExt, StreamExt, TryFutureExt};
use http::{Request, Uri};
use hyper::body::HttpBody;
use hyper::{Body, Client};

use crate::documents::NonPDFDocument;
use crate::onedrive::Onedrive;
use crate::result::{Error, Result};

#[async_trait::async_trait]
pub trait ToPdf
where
    Self: Sized,
{
    async fn convert(&self, onedrive: &Onedrive, output_path: &Path) -> Result<()>;
}

#[async_trait::async_trait]
impl ToPdf for NonPDFDocument {
    async fn convert(&self, onedrive: &Onedrive, output_path: &Path) -> Result<()> {
        let root_site = onedrive.get_root_site().await?;
        let drive = onedrive.drive_exists(&root_site.id).await?;

        let upload_session = onedrive.create_upload_file_session(&drive.id).await?;
        let uploaded_file = onedrive.upload_file(&upload_session, &self.path).await?;
        let pdf_url = onedrive.get_url_to_file_as_pdf(&uploaded_file).await?;

        let mut output_file = File::create(output_path)?;

        let mut res = Client::new().get(Uri::from_str(pdf_url.as_ref())?).await?;
        while let Some(chunk) = res.data().await {
            output_file.write_all(chunk?.as_ref())?;
        }

        onedrive.delete_file(&drive.id, &uploaded_file).await?;

        Ok(())
    }
}
