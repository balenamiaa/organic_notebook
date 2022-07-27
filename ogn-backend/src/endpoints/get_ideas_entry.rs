use ogn_db::ideas;

common_endpoint_imports!();

pub(crate) async fn get_ideas_entry_handler(
    path: web::Path<(IdeaId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let (id,) = path.into_inner();
    let idea = ideas::get_idea(conn.deref_mut(), id)?;

    Ok(web::Json(idea))
}

#[get("/api/ideas/{id}")]
pub async fn get_ideas_entry(
    path: web::Path<(IdeaId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    get_ideas_entry_handler(path, pool).await
}
