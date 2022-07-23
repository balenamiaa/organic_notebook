use ogn_db::documents;

common_endpoint_imports!();

#[delete("/api/documents/{id}")]
pub async fn delete_document(
    path: web::Path<(DocumentId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let (id,) = path.into_inner();

    let count_deleted = documents::delete_document(conn.deref_mut(), id)?;
    Ok(web::Json(count_deleted))
}
