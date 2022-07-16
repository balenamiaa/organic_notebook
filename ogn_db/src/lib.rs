#[macro_use]
extern crate diesel;

use diesel::PgConnection;
use serde_json::Value;

use models::DocumentId;

use crate::models::{Document, DocumentPage, Idea, IdeaId};

pub mod models;
pub mod schema;
pub mod model_impls;

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


pub fn get_document(conn: &mut PgConnection, id: DocumentId) -> Option<Document> {
    use diesel::prelude::*;
    use schema::documents;

    documents::table
        .filter(documents::id.eq(id))
        .get_result(conn)
        .optional()
        .expect("Error loading document")
}

pub fn document_exists(conn: &mut PgConnection, document_id: DocumentId) -> bool {
    get_document(conn, document_id).is_some()
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

pub fn get_idea(conn: &mut PgConnection, id: IdeaId) -> Option<Idea> {
    use diesel::prelude::*;
    use schema::ideas;

    ideas::table
        .filter(ideas::id.eq(id))
        .get_result(conn)
        .optional()
        .expect("Error loading idea")
}

pub fn idea_exists(conn: &mut PgConnection, idea_id: IdeaId) -> bool {
    get_idea(conn, idea_id).is_some()
}

pub fn get_documents(conn: &mut PgConnection, page_index: i64, page_size: i64) -> Vec<Document> {
    use diesel::prelude::*;
    use schema::documents;

    documents::table
        .limit(page_size)
        .offset(page_index * page_size)
        .load::<Document>(conn)
        .expect("Error loading documents")
}

pub fn get_num_documents(conn: &mut PgConnection) -> i64 {
    use diesel::prelude::*;
    use schema::documents;

    documents::table
        .select(diesel::dsl::count_star())
        .first(conn)
        .expect("Error loading documents")
}

pub fn get_ideas(conn: &mut PgConnection, page_index: i64, page_size: i64) -> Vec<Idea> {
    use diesel::prelude::*;
    use schema::ideas;

    ideas::table
        .limit(page_size)
        .offset(page_index * page_size)
        .load::<Idea>(conn)
        .expect("Error loading ideas")
}

pub fn get_num_ideas(conn: &mut PgConnection) -> i64 {
    use diesel::prelude::*;
    use schema::ideas;

    ideas::table
        .select(diesel::dsl::count_star())
        .first(conn)
        .expect("Error loading ideas")
}


