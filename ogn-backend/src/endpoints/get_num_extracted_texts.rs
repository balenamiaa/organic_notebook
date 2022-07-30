use ogn_db::extracted_texts;

common_endpoint_imports!();

pub(crate) async fn get_num_extracted_texts_handler(
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;

    let num_extracted_texts = extracted_texts::get_num_extracted_texts(conn.deref_mut())?;

    Ok(web::Json(num_extracted_texts))
}

#[get("/api/extracted_texts//num")]
pub async fn get_num_extracted_texts(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    get_num_extracted_texts_handler(pool).await
}
