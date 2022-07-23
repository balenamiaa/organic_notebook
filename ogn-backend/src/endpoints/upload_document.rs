use std::fs::File;
use std::io::Write;
use std::path::Path;

use ogn_db::documents;

common_endpoint_imports!();

#[post("/api/documents")]
pub async fn upload_document(
    mut files: Multipart,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    while let Some(Ok(mut item)) = files.next().await {
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

        let mut file = File::create(Path::new(DOCUMENT_ROOTDIR).join(format!("{}.{ext}", id.0)))?;
        file.write_all(&bytes)?;

        let _created_document = documents::create_document(conn.deref_mut(), id, &title, &ext);
    }

    Ok("")
}
