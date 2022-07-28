use ogn_db::idea_refs;

common_endpoint_imports!();

pub(crate) async fn get_idea_refs_entry_handler(
    path: web::Path<(IdeaRefId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let (id,) = path.into_inner();
    let idea_ref = idea_refs::get_idea_ref(conn.deref_mut(), id)?
        .ok_or(actix_web::error::ErrorNotFound(""))?;

    Ok(web::Json(idea_ref))
}

#[get("/api/idea_refs/{id}")]
pub async fn get_idea_refs_entry(
    path: web::Path<(IdeaRefId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    get_idea_refs_entry_handler(path, pool).await
}
