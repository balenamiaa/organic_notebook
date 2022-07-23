use serde_derive::{Deserialize, Serialize};

common_endpoint_imports!();

#[get("/api/idea_refs_for_idea/{id}/num")]
pub async fn get_num_idea_refs_for_idea(
    path: web::Path<(IdeaId,)>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let (id,) = path.into_inner();

    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let num_idea_refs = ogn_db::get_num_idea_refs_for_idea(conn.deref_mut(), id)?;

    Ok(web::Json(num_idea_refs))
}
