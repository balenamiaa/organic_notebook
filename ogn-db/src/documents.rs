use diesel::PgConnection;
use diesel::prelude::*;
use serde_json::Value;

use crate::{Document, DocumentId};
use crate::result::Result;
use crate::schema;

pub fn create_document(
    conn: &mut PgConnection,
    id: DocumentId,
    title: &str,
    filetype: &str,
    document_details: Option<Value>,
) -> Result<Document> {
    let new_document = Document {
        id,
        title: title.to_string(),
        filetype: filetype.to_string(),
        document_details,
    };
    diesel::insert_into(schema::documents::table)
        .values(&new_document)
        .get_result(conn)
        .map_err(|e| str_err!("Error saving new document {}", e))
}

pub fn get_document(conn: &mut PgConnection, id: DocumentId) -> Result<Option<Document>> {
    schema::documents::table
        .filter(schema::documents::id.eq(id))
        .get_result(conn)
        .optional()
        .map_err(|e| str_err!("Error loading document {}", e))
}

pub fn get_documents(
    conn: &mut PgConnection,
    page_index: i64,
    page_size: i64,
) -> Result<Vec<Document>> {
    schema::documents::table
        .limit(page_size)
        .offset(page_index * page_size)
        .load::<Document>(conn)
        .map_err(|e| str_err!("Error loading documents {}", e))
}

pub fn document_exists(conn: &mut PgConnection, document_id: DocumentId) -> Result<bool> {
    Ok(get_document(conn, document_id)?.is_some())
}

pub fn get_num_documents(conn: &mut PgConnection) -> Result<i64> {
    schema::documents::table
        .select(diesel::dsl::count_star())
        .first(conn)
        .map_err(|e| str_err!("Error loading documents {}", e))
}

pub fn delete_document(conn: &mut PgConnection, document_id: DocumentId) -> Result<usize> {
    diesel::delete(schema::documents::table.filter(schema::documents::id.eq(document_id)))
        .execute(conn)
        .map_err(|e| str_err!("Error deleting document {}", e))
}