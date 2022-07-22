use serde_derive::{Deserialize, Serialize};

common_endpoint_imports!();

#[get("/api/idea_refs/{id}")]
pub async fn get_idea_refs_entry(
    path: web::Path<(IdeaRefId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let (id,) = path.into_inner();
    let idea_ref = ogn_db::get_idea_ref(conn.deref_mut(), id)?;

    Ok(web::Json(idea_ref))
}