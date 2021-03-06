use ogn_db::documents;

common_endpoint_imports!();

pub(crate) async fn get_document_entry_handler(
    path: web::Path<(DocumentId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let (id,) = path.into_inner();
    let document = documents::get_document(conn.deref_mut(), id)?
        .ok_or(actix_web::error::ErrorNotFound(""))?;

    Ok(web::Json(document))
}

#[get("/api/documents/{id}")]
pub async fn get_document_entry(
    path: web::Path<(DocumentId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    get_document_entry_handler(path, pool).await
}
