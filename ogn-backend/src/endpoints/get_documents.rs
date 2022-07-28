use serde_derive::{Deserialize, Serialize};

use ogn_db::documents;

common_endpoint_imports!();

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct QueryParams {
    page_num: i64,
    page_size: i64,
}

pub(crate) async fn get_documents_handler(
    query_params: web::Query<QueryParams>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let query = query_params.into_inner();

    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let documents = documents::get_documents(conn.deref_mut(), query.page_num, query.page_size)?;

    let documents_json = serde_json::json!({
        "documents": documents,
        "num_documents_retrieved": documents.len() as i64,
    });

    Ok(web::Json(documents_json))
}

#[get("/api/documents")]
pub async fn get_documents(
    query_params: web::Query<QueryParams>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    get_documents_handler(query_params, pool).await
}
