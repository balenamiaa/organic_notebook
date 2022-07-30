use serde_derive::{Deserialize, Serialize};

use ogn_db::ideas;

common_endpoint_imports!();
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct QueryParams {
    page_num: i64,
    page_size: i64,
}

pub(crate) async fn get_ideas_handler(
    query_params: web::Query<QueryParams>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let query = query_params.into_inner();

    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let ideas = ideas::get_ideas(conn.deref_mut(), query.page_num, query.page_size)?;
    let num_ideas = ideas::get_num_ideas(conn.deref_mut())?;

    let ideas_json = serde_json::json!({
        "ideas": ideas,
        "num_retrieved": ideas.len() as i64,
        "num_remaining": num_ideas - ideas.len() as i64,
    });

    Ok(web::Json(ideas_json))
}

#[get("/api/ideas")]
pub async fn get_ideas(
    query_params: web::Query<QueryParams>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    get_ideas_handler(query_params, pool).await
}
