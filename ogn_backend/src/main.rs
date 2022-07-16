use std::path::Path;

use actix_web::{App, HttpServer, middleware, Responder, web};
use actix_web::web::Data;
use diesel::{PgConnection, r2d2};
use diesel::r2d2::ConnectionManager;
use dotenv_codegen::dotenv;

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
        .wrap(middleware::Compress::default())
        .wrap(middleware::Logger::default())
        .app_data(Data::new(pool.clone()))
        .service(actix_files::Files::new(
            "/static/",
            dotenv!("DATABASE_DOCUMENT_ROOTDIR"),
        ))
        .service(endpoints::upload_document)
        .service(endpoints::get_documents)
        .service(endpoints::get_document_entry)
        .service(endpoints::get_ideas)
        .service(endpoints::get_idea_entry)
    ).bind(("127.0.0.1", 8080))?.run().await?;

    Ok(())
}
