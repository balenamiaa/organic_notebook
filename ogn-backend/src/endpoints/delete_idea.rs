use ogn_db::ideas;

common_endpoint_imports!();

#[delete("/api/ideas/{id}")]
pub async fn delete_idea(
    path: web::Path<(IdeaId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let (id,) = path.into_inner();
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let count_deleted = ideas::delete_idea(conn.deref_mut(), id)?;
    Ok(web::Json(count_deleted))
}
