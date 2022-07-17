common_endpoint_imports!();

#[get("/api/documents/num")]
pub async fn get_num_documents(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let num_documents = ogn_db::get_num_documents(conn.deref_mut())?;

    Ok(web::Json(num_documents))
}