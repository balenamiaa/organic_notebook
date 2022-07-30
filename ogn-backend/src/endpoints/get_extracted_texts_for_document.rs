use ogn_db::extracted_texts;

common_endpoint_imports!();

pub(crate) async fn get_extracted_texts_for_document_handler(
    path: web::Path<(DocumentId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let (id,) = path.into_inner();

    let extracted_texts = extracted_texts::get_extracted_texts_for_document(conn.deref_mut(), id)?;
    Ok(web::Json(extracted_texts))
}

#[get("/api/extracted_texts/document/{document_id}")]
pub async fn get_extracted_texts_for_document(
    path: web::Path<(DocumentId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    get_extracted_texts_for_document_handler(path, pool).await
}
