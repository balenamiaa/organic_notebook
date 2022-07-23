use serde_derive::{Deserialize, Serialize};

use ogn_db::ideas;

common_endpoint_imports!();
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct QueryParams {
    page_num: i64,
    page_size: i64,
}

#[get("/api/ideas")]
pub async fn get_ideas(
    query_params: web::Query<QueryParams>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let query = query_params.into_inner();

    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let ideas = ideas::get_ideas(conn.deref_mut(), query.page_num, query.page_size)?;

    let ideas_json = serde_json::json!({
        "ideas": ideas,
        "num_ideas_retrieved": ideas.len() as i64,
    });

    Ok(web::Json(ideas_json))
}
