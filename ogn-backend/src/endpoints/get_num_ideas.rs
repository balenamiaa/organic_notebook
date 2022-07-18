common_endpoint_imports!();

#[get("/api/ideas/num")]
pub async fn get_num_ideas(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let num_ideas = ogn_db::get_num_ideas(conn.deref_mut())?;

    Ok(web::Json(num_ideas))
}
