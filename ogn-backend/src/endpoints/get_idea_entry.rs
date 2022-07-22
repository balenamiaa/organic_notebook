use serde_derive::{Deserialize, Serialize};

common_endpoint_imports!();
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct QueryParams {
    page_num: i64,
    page_size: i64,
}

#[get("/api/ideas/")]
pub async fn get_idea_entry(
    path: web::Path<(IdeaId, )>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let (id, ) = path.into_inner();
    let idea = ogn_db::get_idea(conn.deref_mut(), id)?;

    Ok(web::Json(idea))
}
