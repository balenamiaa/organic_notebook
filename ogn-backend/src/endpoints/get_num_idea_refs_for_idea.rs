common_endpoint_imports!();

pub(crate) async fn get_num_idea_refs_for_idea_handler(
    path: web::Path<(IdeaId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let (id,) = path.into_inner();

    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let num_idea_refs = ogn_db::get_num_idea_refs_for_idea(conn.deref_mut(), id)?;

    Ok(web::Json(num_idea_refs))
}

#[get("/api/idea_refs_for_idea/{id}/num")]
pub async fn get_num_idea_refs_for_idea(
    path: web::Path<(IdeaId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    get_num_idea_refs_for_idea_handler(path, pool).await
}
