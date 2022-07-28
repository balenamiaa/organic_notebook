use ogn_db::documents;

common_endpoint_imports!();

pub(crate) async fn get_num_documents_handler(
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let num_documents = documents::get_num_documents(conn.deref_mut())?;

    Ok(web::Json(num_documents))
}

#[get("/api/documents//num")]
pub async fn get_num_documents(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    get_num_documents_handler(pool).await
}
