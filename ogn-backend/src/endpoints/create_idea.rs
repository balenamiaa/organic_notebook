common_endpoint_imports!();

#[post("/api/create_idea")]
pub async fn create_idea(
    pool: web::Data<DbPool>,
    json_body: web::Json<NewIdea>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let json_body = json_body.into_inner();
    let idea = ogn_db::create_idea(
        conn.deref_mut(),
        json_body.doc_page,
        &json_body.idea_text,
        json_body.idea_details,
    )?;

    Ok(web::Json(idea))
}
