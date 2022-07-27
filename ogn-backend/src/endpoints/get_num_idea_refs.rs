use ogn_db::idea_refs;

common_endpoint_imports!();

pub(crate) async fn get_num_idea_refs_handler(
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let num_idea_refs = idea_refs::get_num_idea_refs(conn.deref_mut())?;

    Ok(web::Json(num_idea_refs))
}

#[get("/api/idea_refs//num")]
pub async fn get_num_idea_refs(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    get_num_idea_refs_handler(pool).await
}
