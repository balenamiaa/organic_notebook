use std::fs::File;
use std::io::Write;
use std::ops::DerefMut;
use std::path::Path;

use actix_multipart::Multipart;
use actix_web::{get, HttpRequest, post, Responder, web};
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::web::Data;
use futures_util::StreamExt;

use ogn_db::models::{DocumentId, IdeaId};

use crate::{DbPool, DOCUMENT_ROOTDIR};

#[post("/api/upload_document")]
pub async fn upload_document(mut files: Multipart, pool: Data<DbPool>) -> actix_web::Result<impl Responder> {
    while let Some(Ok(mut item)) = files.next().await {
        let _mime = item.content_type();
        let mut bytes = vec![];
        while let Some(Ok(bytes_ty)) = item.next().await {
            bytes.extend_from_slice(bytes_ty.as_ref());
        }
        let id = DocumentId(crc32fast::hash(&bytes) as i32);

        let (title, ext) = {
            let filename = item.content_disposition().get_filename().ok_or(ErrorBadRequest("no filename"))?;
            let mut splits = filename.split('.').peekable();

            let mut title = String::new();
            let mut ext = None;
            while let Some(split) = splits.next() {
                if splits.peek().is_some() {
                    title.push_str(split);
                } else {
                    ext = Some(split.to_owned());
                }
            }

            (title, ext.ok_or(ErrorBadRequest("no extension"))?)
        };


        let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
        if ogn_db::document_exists(conn.deref_mut(), id)? {
            return Err(ErrorBadRequest("bad request"));
        }


        let mut file = File::create(Path::new(DOCUMENT_ROOTDIR).join(format!("{}.{ext}", id.0)))?;
        file.write_all(&bytes)?;

        let _created_document = ogn_db::create_document(conn.deref_mut(), id, &title, &ext, None);
    }

    Ok("")
}


#[get("/api/documents/{id}")]
pub async fn get_document_entry(path: web::Path<(DocumentId, )>, pool: Data<DbPool>) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let (id, ) = path.into_inner();
    let document = ogn_db::get_document(conn.deref_mut(), id)?;

    Ok(web::Json(document))
}

#[get("/api/ideas/{id}")]
pub async fn get_idea_entry(path: web::Path<(IdeaId, )>, pool: Data<DbPool>) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let (id, ) = path.into_inner();
    let idea = ogn_db::get_idea(conn.deref_mut(), id)?;

    Ok(web::Json(idea))
}

#[get("/api/ideas")]
pub async fn get_ideas(query_params: web::Query<(i64, i64)>, pool: Data<DbPool>) -> actix_web::Result<impl Responder> {
    let (page_number, page_size) = query_params.into_inner();

    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let ideas = ogn_db::get_ideas(conn.deref_mut(), page_number, page_size)?;

    let num_ideas = ogn_db::get_num_ideas(conn.deref_mut())?;
    let num_ideas_left = num_ideas - (page_number * page_size);

    let ideas_json = serde_json::json!({
        "ideas": ideas,
        "num_ideas": num_ideas,
        "num_ideas_left": num_ideas_left,
        "num_ideas_retrieved": ideas.len() as i64,
    });

    Ok(web::Json(ideas_json))
}

#[get("/api/documents")]
pub async fn get_documents(query_params: web::Query<(i64, i64)>, pool: Data<DbPool>) -> actix_web::Result<impl Responder> {
    let (page_number, page_size) = query_params.into_inner();

    let mut conn = pool.get().map_err(|x| ErrorInternalServerError(x))?;
    let documents = ogn_db::get_documents(conn.deref_mut(), page_number, page_size)?;

    let num_documents = ogn_db::get_num_documents(conn.deref_mut())?;
    let num_documents_left = num_documents - (page_number * page_size);

    let documents_json = serde_json::json!({
        "documents": documents,
        "num_documents": num_documents,
        "num_documents_left": num_documents_left,
        "num_documents_retrieved": documents.len() as i64,
    });

    Ok(web::Json(documents_json))
}



