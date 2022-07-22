common_endpoint_imports!();

#[post("/api/create_idea_ref")]
pub async fn create_idea_ref(
    pool: web::Data<DbPool>,
    json_body: web::Json<NewIdeaRef>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let json_body = json_body.into_inner();
    let idea_ref = ogn_db::create_idea_ref(
        conn.deref_mut(),
        json_body.doc_page,
        json_body.idea_ref,
        json_body.idea_details,
    )?;

    Ok(web::Json(idea_ref))
}