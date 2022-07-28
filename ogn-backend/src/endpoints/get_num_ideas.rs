use ogn_db::ideas;

common_endpoint_imports!();

pub(crate) async fn get_num_ideas_handler(
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let num_ideas = ideas::get_num_ideas(conn.deref_mut())?;

    Ok(web::Json(num_ideas))
}

#[get("/api/ideas//num")]
pub async fn get_num_ideas(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    get_num_ideas_handler(pool).await
}
