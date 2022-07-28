use std::env::temp_dir;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use ogn_db::documents;
use ogn_utils::conversion::ToPdf;
use ogn_utils::documents::NonPDFDocument;
use ogn_utils::onedrive::Onedrive;

common_endpoint_imports!();

pub(crate) async fn upload_document_handler(
    mut files: Multipart,
    pool: web::Data<DbPool>,
    onedrive: web::Data<Onedrive>,
) -> actix_web::Result<impl Responder> {
    let mut has_file = false;
    let mut created_documents = vec![];
    while let Some(Ok(mut item)) = files.next().await {
        has_file = true;

        let _mime = item.content_type();
        let mut bytes = vec![];
        while let Some(Ok(bytes_ty)) = item.next().await {
            bytes.extend_from_slice(bytes_ty.as_ref());
        }
        let id = DocumentId(crc32fast::hash(&bytes) as i32);

        let (title, ext) = {
            let filename = item
                .content_disposition()
                .get_filename()
                .ok_or(ErrorBadRequest("no filename"))?;
            let mut splits = filename.split('.').peekable();

            let mut title = String::new();
            let mut ext = None;
            while let Some(split) = splits.next() {
                if splits.peek().is_some() {
                    title.push_str(split);
                } else {
                    ext = Some(split.to_owned());
                }
            }

            (title, ext.ok_or(ErrorBadRequest("no extension"))?)
        };

        let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
        if documents::document_exists(conn.deref_mut(), id)? {
            return Err(ErrorBadRequest("bad request"));
        }

        match ext.as_ref() {
            "pdf" => {
                let document_filepath = Path::new(DOCUMENT_ROOTDIR).join(format!("{}.pdf", id.0));
                let mut document_file = File::create(&document_filepath)?;
                document_file.write_all(&bytes)?;
            }
            _ => {
                let document_filepath = Path::new(DOCUMENT_ROOTDIR).join(format!("{}.pdf", id.0));
                let temp_filepath = temp_dir().join(format!("{}.{}", id.0, ext));

                let mut temp_file = File::create(&temp_filepath)?;
                temp_file.write_all(&bytes)?;

                let temp_document = NonPDFDocument::new(temp_filepath);
                temp_document
                    .convert_to_pdf(onedrive.get_ref(), document_filepath.as_path())
                    .await
                    .map_err(|_| ErrorInternalServerError("Couldn't convert document to pdf"))?;
            }
        }

        let _created_document = documents::create_document(conn.deref_mut(), id, &title, &ext)?;
        created_documents.push(_created_document);
    }

    if created_documents.is_empty() {
        Err(ErrorBadRequest("no file"))
    } else {
        Ok(web::Json(created_documents))
    }
}

#[post("/api/documents")]
pub async fn upload_document(
    files: Multipart,
    pool: web::Data<DbPool>,
    onedrive: web::Data<Onedrive>,
) -> actix_web::Result<impl Responder> {
    upload_document_handler(files, pool, onedrive).await
}
