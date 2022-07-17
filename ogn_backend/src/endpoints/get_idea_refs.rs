common_endpoint_imports!();

#[get("/api/idea_refs/{id}")]
pub async fn get_idea_refs(
    path: web::Path<(IdeaId, )>,
    query_params: web::Query<(i64, i64)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let (id, ) = path.into_inner();
    let (page_number, page_size) = query_params.into_inner();

    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let idea_refs = ogn_db::get_idea_refs(conn.deref_mut(), id, page_number, page_size)?;

    let idea_refs_json = serde_json::json!({
        "idea_refs": idea_refs,
        "num_idea_refs_retrieved": idea_refs.len() as i64,
    });

    Ok(web::Json(idea_refs_json))
}
