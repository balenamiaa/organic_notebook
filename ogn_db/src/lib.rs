#[macro_use]
extern crate diesel;

use std::process::id;

use diesel::PgConnection;
use serde_json::Value;

use models::DocumentId;

use crate::models::{Document, DocumentPage, IdeaId};

pub mod models;
pub mod schema;

pub fn create_document(
    conn: &mut PgConnection,
    id: DocumentId,
    title: &str,
    document_details: Option<Value>,
) -> Document {
    use crate::diesel::RunQueryDsl;
    use schema::documents;

    let new_document = Document {
        id,
        title: title.to_string(),
        document_details,
    };
    diesel::insert_into(documents::table)
        .values(&new_document)
        .get_result(conn)
        .expect("Error saving new document")
}

pub fn document_exists(conn: &mut PgConnection, document_id: DocumentId) -> bool {
    use crate::schema::documents::dsl::*;
    use diesel::prelude::*;

    documents
        .find(document_id.0)
        .first::<Document>(conn)
        .is_ok()
}

pub fn create_idea(
    conn: &mut PgConnection,
    id: IdeaId,
    doc_page: DocumentPage,
    idea_text: &str,
    idea_details: Option<Value>,
) -> models::Idea {
    use crate::diesel::RunQueryDsl;
    use schema::ideas;

    let new_idea = models::Idea {
        id,
        doc_page,
        idea_text: idea_text.to_string(),
        idea_details,
    };
    diesel::insert_into(ideas::table)
        .values(new_idea)
        .get_result(conn)
        .expect("Error saving new idea")
}

pub fn idea_exists(conn: &mut PgConnection, idea_id: IdeaId) -> bool {
    use crate::schema::ideas::dsl::*;
    use diesel::prelude::*;

    ideas
        .find(idea_id.0)
        .first::<models::Idea>(conn)
        .is_ok()
}
