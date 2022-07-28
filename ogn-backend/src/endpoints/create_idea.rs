use ogn_db::ideas;

common_endpoint_imports!();

pub(crate) async fn create_idea_handler(
    pool: web::Data<DbPool>,
    json_body: web::Json<NewIdea>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let json_body = json_body.into_inner();
    let idea = ideas::create_idea(conn.deref_mut(), &json_body.label)?;

    Ok(web::Json(idea))
}

#[post("/api/ideas")]
pub async fn create_idea(
    pool: web::Data<DbPool>,
    json_body: web::Json<NewIdea>,
) -> actix_web::Result<impl Responder> {
    create_idea_handler(pool, json_body).await
}
