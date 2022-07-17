common_endpoint_imports!();

#[get("/api/ideas")]
pub async fn get_ideas(query_params: web::Query<(i64, i64)>, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let (page_number, page_size) = query_params.into_inner();

    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let ideas = ogn_db::get_ideas(conn.deref_mut(), page_number, page_size)?;

    let ideas_json = serde_json::json!({
        "ideas": ideas,
        "num_ideas_retrieved": ideas.len() as i64,
    });

    Ok(web::Json(ideas_json))
}