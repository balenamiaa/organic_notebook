#![feature(int_roundings)]

use std::path::Path;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};
use dotenv_codegen::dotenv;

use autoservice::auto_service;
use futures_util::lock::Mutex;
use ogn_utils::onedrive::Onedrive;

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

    let onedrive = Arc::new(Mutex::new(Onedrive::new())); // TODO: don't hardcode access token.

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        let mut app = App::new()
            .wrap(cors)
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(onedrive.clone()))
            .service(actix_files::Files::new(
                "/static/",
                dotenv!("DATABASE_DOCUMENT_ROOTDIR"),
            ));

        auto_service!(app; "endpoints");
        app
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
