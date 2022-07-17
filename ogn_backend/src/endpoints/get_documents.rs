common_endpoint_imports!();

#[get("/api/documents")]
pub async fn get_documents(query_params: web::Query<(i64, i64)>, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let (page_number, page_size) = query_params.into_inner();

    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let documents = ogn_db::get_documents(conn.deref_mut(), page_number, page_size)?;

    let documents_json = serde_json::json!({
        "documents": documents,
        "num_documents_retrieved": documents.len() as i64,
    });

    Ok(web::Json(documents_json))
}