use ogn_db::idea_refs;

common_endpoint_imports!();

pub(crate) async fn create_idea_ref_handler(
    pool: web::Data<DbPool>,
    json_body: web::Json<NewIdeaRef>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let json_body = json_body.into_inner();
    let idea_ref = idea_refs::create_idea_ref(
        conn.deref_mut(),
        json_body.doc_page,
        json_body.idea_ref,
        json_body.idea_ref_text,
    )?;

    Ok(web::Json(idea_ref))
}

#[post("/api/idea_refs")]
pub async fn create_idea_ref(
    pool: web::Data<DbPool>,
    json_body: web::Json<NewIdeaRef>,
) -> actix_web::Result<impl Responder> {
    create_idea_ref_handler(pool, json_body).await
}
