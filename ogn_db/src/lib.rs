#[macro_use]
extern crate diesel;

use std::fmt::{Debug, Display, Formatter};

use anyhow::anyhow;
use diesel::PgConnection;
use serde_json::Value;

use models::DocumentId;

use crate::models::{Document, DocumentPage, Idea, IdeaId};

pub mod models;
pub mod schema;
pub mod model_impls;


#[derive(Debug)]
pub struct Error {
    inner: anyhow::Error,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <anyhow::Error as std::fmt::Display>::fmt(&self.inner, f)?;

        Ok(())
    }
}

impl actix_web::error::ResponseError for Error {}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Error {
        Error { inner: err }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! str_err {
    ($($arg:tt)*) => {
        anyhow!($($arg)*).into()
    };
}

pub fn create_document(
    conn: &mut PgConnection,
    id: DocumentId,
    title: &str,
    document_details: Option<Value>,
) -> Result<Document> {
    use crate::diesel::RunQueryDsl;
    use schema::documents;

    let new_document = Document {
        id,
        title: title.to_string(),
        document_details,
    };
    diesel::insert_into(documents::table).values(&new_document).get_result(conn).map_err(|e| str_err!("Error saving new document {}", e))
}


pub fn get_document(conn: &mut PgConnection, id: DocumentId) -> Result<Option<Document>> {
    use diesel::prelude::*;
    use schema::documents;

    documents::table.filter(documents::id.eq(id)).get_result(conn).optional().map_err(|e| str_err!("Error loading document {}", e))
}

pub fn document_exists(conn: &mut PgConnection, document_id: DocumentId) -> Result<bool> {
    Ok(get_document(conn, document_id)?.is_some())
}

pub fn create_idea(
    conn: &mut PgConnection,
    id: IdeaId,
    doc_page: DocumentPage,
    idea_text: &str,
    idea_details: Option<Value>,
) -> Result<Idea> {
    use crate::diesel::RunQueryDsl;
    use schema::ideas;

    let new_idea = Idea {
        id,
        doc_page,
        idea_text: idea_text.to_string(),
        idea_details,
    };
    diesel::insert_into(ideas::table).values(new_idea).get_result(conn).map_err(|e| str_err!("Error saving new idea {}", e))
}

pub fn get_idea(conn: &mut PgConnection, id: IdeaId) -> Result<Option<Idea>> {
    use diesel::prelude::*;
    use schema::ideas;

    ideas::table.filter(ideas::id.eq(id)).get_result(conn).optional().map_err(|e| str_err!("Error loading idea {}", e))
}

pub fn idea_exists(conn: &mut PgConnection, idea_id: IdeaId) -> Result<bool> {
    Ok(get_idea(conn, idea_id)?.is_some())
}

pub fn get_documents(conn: &mut PgConnection, page_index: i64, page_size: i64) -> Result<Vec<Document>> {
    use diesel::prelude::*;
    use schema::documents;

    documents::table.limit(page_size).offset(page_index * page_size).load::<Document>(conn).map_err(|e| str_err!("Error loading documents {}", e))
}

pub fn get_num_documents(conn: &mut PgConnection) -> Result<i64> {
    use diesel::prelude::*;
    use schema::documents;

    documents::table.select(diesel::dsl::count_star()).first(conn).map_err(|e| str_err!("Error loading documents {}", e))
}

pub fn get_ideas(conn: &mut PgConnection, page_index: i64, page_size: i64) -> Result<Vec<Idea>> {
    use diesel::prelude::*;
    use schema::ideas;

    ideas::table.limit(page_size).offset(page_index * page_size).load::<Idea>(conn).map_err(|e| str_err!("Error loading ideas {}", e))
}

pub fn get_num_ideas(conn: &mut PgConnection) -> Result<i64> {
    use diesel::prelude::*;
    use schema::ideas;

    ideas::table.select(diesel::dsl::count_star()).first(conn).map_err(|e| str_err!("Error loading ideas {}", e))
}


