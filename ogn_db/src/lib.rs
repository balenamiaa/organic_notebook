#[macro_use]
extern crate diesel;

use std::process::id;

use diesel::PgConnection;
use serde_json::Value;

use models::DocumentId;

use crate::models::Document;

pub mod models;
pub mod schema;

pub fn create_document(
    conn: &mut PgConnection,
    id: DocumentId,
    title: &str,
    document_details: Option<Value>,
) -> Document {
    use crate::diesel::RunQueryDsl;
    use models::NewDocument;
    use schema::documents;

    let new_document = NewDocument {
        id: id,
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
