use std::path::Path;

use actix_web::{App, get, HttpServer, middleware, post, Responder, web};
use actix_web::web::Data;
use diesel::{PgConnection, r2d2};
use diesel::r2d2::ConnectionManager;
use dotenv_codegen::dotenv;

use crate::endpoints::upload_document;

mod endpoints;

pub const DOCUMENT_ROOTDIR: &'static str = dotenv!("DATABASE_DOCUMENT_ROOTDIR");

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    {
        let dir = Path::new(DOCUMENT_ROOTDIR);
        if !dir.is_dir() {
            std::fs::create_dir(dir)?;
        }
    }

    let manager = ConnectionManager::<PgConnection>::new(dotenv!("DATABASE_URL"));
    let pool: DbPool = r2d2::Pool::builder().build(manager)?;

    HttpServer::new(move || App::new()
        .app_data(Data::new(pool.clone()))
        .wrap(middleware::Logger::default())
        .service(upload_document)
    ).bind(("127.0.0.1", 8080))?.run().await?;

    Ok(())
}
