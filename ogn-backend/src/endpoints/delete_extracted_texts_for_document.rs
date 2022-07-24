use ogn_db::extracted_texts;

common_endpoint_imports!();

#[delete("/api/extracted_texts/{document_id}")]
pub async fn delete_extracted_texts_for_document(
    path: web::Path<(DocumentId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let (id,) = path.into_inner();
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let count_deleted = extracted_texts::delete_extracted_texts_for_document(conn.deref_mut(), id)?;
    Ok(web::Json(count_deleted))
}
