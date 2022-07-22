use serde_derive::{Deserialize, Serialize};

common_endpoint_imports!();

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct QueryParams {
    id: IdeaId,
}

#[get("/api/idea_refs/num")]
pub async fn get_num_idea_refs(
    query_params: web::Query<QueryParams>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let query = query_params.into_inner();

    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let num_idea_refs = ogn_db::get_num_idea_refs(conn.deref_mut(), query.id)?;

    Ok(web::Json(num_idea_refs))
}
