use ogn_db::extracted_texts;

common_endpoint_imports!();

pub(crate) async fn get_num_extracted_texts_for_document_handler(
    path: web::Path<(DocumentId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let (id,) = path.into_inner();
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let num_ideas = extracted_texts::get_num_extracted_texts_for_document(conn.deref_mut(), id)?;

    Ok(web::Json(num_ideas))
}

#[get("/api/extracted_texts/document/{document_id}/num")]
pub async fn get_num_extracted_texts_for_document(
    path: web::Path<(DocumentId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    get_num_extracted_texts_for_document_handler(path, pool).await
}
