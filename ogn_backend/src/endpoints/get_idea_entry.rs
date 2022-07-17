common_endpoint_imports!();

#[get("/api/ideas/{id}")]
pub async fn get_idea_entry(path: web::Path<(IdeaId, )>, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let (id, ) = path.into_inner();
    let idea = ogn_db::get_idea(conn.deref_mut(), id)?;

    Ok(web::Json(idea))
}