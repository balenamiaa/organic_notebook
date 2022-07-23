use ogn_db::idea_refs;
use serde_derive::{Deserialize, Serialize};

common_endpoint_imports!();
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct QueryParams {
    page_num: i64,
    page_size: i64,
}

#[get("/api/idea_refs")]
pub async fn get_idea_refs(
    query_params: web::Query<QueryParams>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let query = query_params.into_inner();

    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let idea_refs = idea_refs::get_idea_refs(conn.deref_mut(), query.page_num, query.page_size)?;

    let idea_refs_json = serde_json::json!({
        "idea_refs": idea_refs,
        "num_ideas_retrieved": idea_refs.len() as i64,
    });

    Ok(web::Json(idea_refs_json))
}
