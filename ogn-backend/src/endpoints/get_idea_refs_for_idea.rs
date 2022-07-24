use serde_derive::{Deserialize, Serialize};

common_endpoint_imports!();
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct QueryParams {
    page_num: i64,
    page_size: i64,
}

#[get("/api/idea_refs_for_idea/{id}")]
pub async fn get_idea_refs_for_idea(
    path: web::Path<(IdeaId,)>,
    query_params: web::Query<QueryParams>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let (id,) = path.into_inner();
    let query = query_params.into_inner();

    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let idea_refs =
        ogn_db::get_idea_refs_for_idea(conn.deref_mut(), id, query.page_num, query.page_size)?;

    let idea_refs_json = serde_json::json!({
        "idea_refs": idea_refs,
        "num_idea_refs_retrieved": idea_refs.len() as i64,
    });

    Ok(web::Json(idea_refs_json))
}
