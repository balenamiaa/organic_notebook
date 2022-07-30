use std::num::ParseIntError;

use ogn_db::extracted_texts;

common_endpoint_imports!();

pub(crate) async fn get_extracted_texts_for_document_bulk_handler(
    body: web::Bytes,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let body = String::from_utf8_lossy(body.as_ref());

    let ids = if body.contains(' ') {
        body.split(' ')
            .map(|x| x.parse::<i32>().map(|x| DocumentId(x)))
            .collect::<Result<Vec<_>, ParseIntError>>()
            .map_err(|x| ErrorInternalServerError(x))?
    } else {
        vec![body
            .parse::<i32>()
            .map(|x| DocumentId(x))
            .map_err(|x| ErrorInternalServerError(x))?]
    };

    let extracted_texts =
        extracted_texts::get_extracted_texts_for_document_bulk(conn.deref_mut(), &ids)?;

    Ok(web::Json(extracted_texts))
}

#[get("/api/extracted_texts/document/")]
pub async fn get_extracted_texts_for_document_bulk(
    body: web::Bytes,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    get_extracted_texts_for_document_bulk_handler(body, pool).await
}
