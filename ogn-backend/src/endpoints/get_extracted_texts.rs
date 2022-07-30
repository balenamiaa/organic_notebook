use serde_derive::{Deserialize, Serialize};

use ogn_db::extracted_texts;

common_endpoint_imports!();
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct QueryParams {
    page_num: i64,
    page_size: i64,
}

pub(crate) async fn get_extracted_texts_handler(
    query_params: web::Query<QueryParams>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let query = query_params.into_inner();

    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let extracted_texts =
        extracted_texts::get_extracted_texts(conn.deref_mut(), query.page_num, query.page_size)?;
    let num_extracted_texts = extracted_texts::get_num_extracted_texts(conn.deref_mut())?;

    let extracted_texts_json = serde_json::json!({
        "extracted_texts": extracted_texts,
        "num_retrieved": extracted_texts.len() as i64,
        "num_remaining": num_extracted_texts - extracted_texts.len() as i64,
    });

    Ok(web::Json(extracted_texts_json))
}

#[get("/api/extracted_texts")]
pub async fn get_extracted_texts(
    query_params: web::Query<QueryParams>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    get_extracted_texts_handler(query_params, pool).await
}
