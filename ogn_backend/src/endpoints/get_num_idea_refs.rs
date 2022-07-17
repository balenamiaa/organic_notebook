common_endpoint_imports!();

#[get("/api/idea_refs/num")]
pub async fn get_num_idea_refs(query_params: web::Query<(IdeaId, )>, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let (id, ) = query_params.into_inner();

    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let num_idea_refs = ogn_db::get_num_idea_refs(conn.deref_mut(), id)?;

    Ok(web::Json(num_idea_refs))
}