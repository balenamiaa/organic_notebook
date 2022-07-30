use std::path::Path;

use actix_web::error::ErrorNotFound;

use ogn_db::extracted_texts;
use ogn_utils::documents::PDFDocument;
use ogn_utils::extractor::TextExtractor;

common_endpoint_imports!();

pub(crate) async fn extract_texts_for_document_handler(
    path: web::Path<(DocumentId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let (id,) = path.into_inner();
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let document = ogn_db::documents::get_document(conn.deref_mut(), id)?
        .ok_or(ErrorNotFound("document not found"))?;

    let document_filepath = Path::new(DOCUMENT_ROOTDIR).join(format!("{}.pdf", document.id.0));

    let extracted_texts = PDFDocument::open(document_filepath)?.extract()?;
    let docpages = extracted_texts
        .iter()
        .enumerate()
        .map(|(page_number, _)| DocumentPage {
            document_id: id,
            page_number: Some(page_number as i32 + 1),
        })
        .collect::<Vec<_>>();

    let extracted_texts =
        extracted_texts::create_extracted_text_bulk(conn.deref_mut(), &extracted_texts, &docpages)?;

    Ok(web::Json(extracted_texts))
}

#[post("/api/extracted_texts/document/{document_id}")]
pub async fn extract_texts_for_document(
    path: web::Path<(DocumentId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    extract_texts_for_document_handler(path, pool).await
}
