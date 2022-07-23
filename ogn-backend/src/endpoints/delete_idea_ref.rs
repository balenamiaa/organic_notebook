use ogn_db::idea_refs;

common_endpoint_imports!();

#[delete("/api/idea_refs/{id}")]
pub async fn delete_idea_ref(
    path: web::Path<(IdeaRefId, )>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let (id, ) = path.into_inner();

    let count_deleted = idea_refs::delete_idea_refs(conn.deref_mut(), id)?;
    Ok(web::Json(count_deleted))
}
