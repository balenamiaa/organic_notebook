use std::fmt::{Debug, Display, Formatter};

use anyhow::anyhow;
use diesel;
use diesel::PgConnection;
use serde_json::Value;

use models::DocumentId;

use crate::models::{
    Document, DocumentPage, Idea, IdeaId, IdeaRef, IdeaRefId, NewIdea, NewIdeaRef,
};

pub mod model_impls;
pub mod models;
pub mod schema;

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
        Error::from(anyhow!($($arg)*))
    };
}

//-- IDEAS TABLE

pub fn create_idea(conn: &mut PgConnection, label: &str) -> Result<Idea> {
    use crate::diesel::RunQueryDsl;
    use schema::ideas;

    let new_idea = NewIdea {
        label: label.to_string(),
    };

    diesel::insert_into(ideas::table)
        .values(new_idea)
        .get_result(conn)
        .map_err(|e| str_err!("Error saving new idea {}", e))
}

pub fn get_idea(conn: &mut PgConnection, id: IdeaId) -> Result<Option<Idea>> {
    use diesel::prelude::*;
    use schema::ideas;

    ideas::table
        .filter(ideas::id.eq(id))
        .get_result(conn)
        .optional()
        .map_err(|e| str_err!("Error loading idea {}", e))
}

pub fn get_ideas(conn: &mut PgConnection, page_index: i64, page_size: i64) -> Result<Vec<Idea>> {
    use diesel::prelude::*;
    use schema::ideas;

    ideas::table
        .limit(page_size)
        .offset(page_index * page_size)
        .load::<Idea>(conn)
        .map_err(|e| str_err!("Error loading ideas {}", e))
}

pub fn idea_exists(conn: &mut PgConnection, id: IdeaId) -> Result<bool> {
    Ok(get_idea(conn, id)?.is_some())
}

pub fn get_num_ideas(conn: &mut PgConnection) -> Result<i64> {
    use diesel::prelude::*;
    use schema::ideas;

    ideas::table
        .select(diesel::dsl::count_star())
        .first(conn)
        .map_err(|e| str_err!("Error loading ideas {}", e))
}

//-- IDEAS TABLE

//-- IDEA_REFS TABLE

pub fn create_idea_ref(
    conn: &mut PgConnection,
    doc_page: DocumentPage,
    idea_ref: IdeaId,
    idea_details: Option<Value>,
) -> Result<IdeaRef> {
    use crate::diesel::RunQueryDsl;
    use schema::idea_refs;

    let new_idea = NewIdeaRef {
        doc_page,
        idea_ref,
        idea_details,
    };

    diesel::insert_into(idea_refs::table)
        .values(new_idea)
        .get_result(conn)
        .map_err(|e| str_err!("Error saving new idea ref {}", e))
}

pub fn get_idea_ref(conn: &mut PgConnection, id: IdeaRefId) -> Result<Option<IdeaRef>> {
    use diesel::prelude::*;
    use schema::idea_refs;

    idea_refs::table
        .filter(idea_refs::id.eq(id))
        .get_result(conn)
        .optional()
        .map_err(|e| str_err!("Error loading idea ref {}", e))
}

pub fn get_idea_refs(
    conn: &mut PgConnection,
    page_index: i64,
    page_size: i64,
) -> Result<Vec<IdeaRef>> {
    use diesel::prelude::*;
    use schema::idea_refs;

    idea_refs::table
        .limit(page_size)
        .offset(page_index * page_size)
        .load::<IdeaRef>(conn)
        .map_err(|e| str_err!("Error loading idea refs {}", e))
}

pub fn idea_ref_exists(conn: &mut PgConnection, id: IdeaRefId) -> Result<bool> {
    Ok(get_idea_ref(conn, id)?.is_some())
}

pub fn get_num_idea_refs(conn: &mut PgConnection) -> Result<i64> {
    use diesel::prelude::*;
    use schema::idea_refs;

    idea_refs::table
        .select(diesel::dsl::count_star())
        .first(conn)
        .map_err(|e| str_err!("Error loading idea refs {}", e))
}

//-- IDEA_REFS TABLE

//-- DOCUMENTS TABLE

pub fn create_document(
    conn: &mut PgConnection,
    id: DocumentId,
    title: &str,
    filetype: &str,
    document_details: Option<Value>,
) -> Result<Document> {
    use crate::diesel::RunQueryDsl;
    use schema::documents;

    let new_document = Document {
        id,
        title: title.to_string(),
        filetype: filetype.to_string(),
        document_details,
    };
    diesel::insert_into(documents::table)
        .values(&new_document)
        .get_result(conn)
        .map_err(|e| str_err!("Error saving new document {}", e))
}

pub fn get_document(conn: &mut PgConnection, id: DocumentId) -> Result<Option<Document>> {
    use diesel::prelude::*;
    use schema::documents;

    documents::table
        .filter(documents::id.eq(id))
        .get_result(conn)
        .optional()
        .map_err(|e| str_err!("Error loading document {}", e))
}

pub fn get_documents(
    conn: &mut PgConnection,
    page_index: i64,
    page_size: i64,
) -> Result<Vec<Document>> {
    use diesel::prelude::*;
    use schema::documents;

    documents::table
        .limit(page_size)
        .offset(page_index * page_size)
        .load::<Document>(conn)
        .map_err(|e| str_err!("Error loading documents {}", e))
}

pub fn document_exists(conn: &mut PgConnection, document_id: DocumentId) -> Result<bool> {
    Ok(get_document(conn, document_id)?.is_some())
}

pub fn get_num_documents(conn: &mut PgConnection) -> Result<i64> {
    use diesel::prelude::*;
    use schema::documents;

    documents::table
        .select(diesel::dsl::count_star())
        .first(conn)
        .map_err(|e| str_err!("Error loading documents {}", e))
}

//-- DOCUMENTS TABLE

pub fn get_idea_refs_for_idea(
    conn: &mut PgConnection,
    id: IdeaId,
    page_index: i64,
    page_size: i64,
) -> Result<Vec<IdeaRef>> {
    use diesel::prelude::*;
    use schema::idea_refs;

    idea_refs::table
        .filter(idea_refs::id.eq(id))
        .select(idea_refs::all_columns)
        .limit(page_size)
        .offset(page_index * page_size)
        .load(conn)
        .map_err(|e| str_err!("Error loading idea refs {}", e))
}

pub fn get_num_idea_refs_for_idea(conn: &mut PgConnection, id: IdeaId) -> Result<i64> {
    use diesel::prelude::*;
    use schema::idea_refs;

    idea_refs::table
        .filter(idea_refs::id.eq(id))
        .select(diesel::dsl::count_star())
        .first(conn)
        .map_err(|e| str_err!("Error loading idea refs {}", e))
}
