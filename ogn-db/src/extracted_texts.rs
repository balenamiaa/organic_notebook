use diesel::prelude::*;
use diesel::PgConnection;

use crate::result::Result;
use crate::{schema, DocumentId};
use crate::{DocumentPage, ExtractedText, ExtractedTextId, NewExtractedText};

pub fn create_extracted_text(
    conn: &mut PgConnection,
    content: String,
    doc_page: DocumentPage,
) -> Result<ExtractedText> {
    let new_extracted_text = NewExtractedText { content, doc_page };

    diesel::insert_into(schema::extracted_texts::table)
        .values(new_extracted_text)
        .get_result(conn)
        .map_err(|e| str_err!("Error saving new extracted text {}", e))
}

pub fn create_extracted_text_bulk(
    conn: &mut PgConnection,
    content: &[String],
    doc_page: &[DocumentPage],
) -> Result<Vec<ExtractedText>> {
    assert_eq!(content.len(), doc_page.len());

    let new_extracted_texts = content
        .iter()
        .zip(doc_page.iter())
        .map(|(a, b)| {
            (
                schema::extracted_texts::content.eq(a),
                schema::extracted_texts::document_id.eq(b.document_id),
                schema::extracted_texts::document_page.eq(b.page_number.expect("malformed data")),
            )
        })
        .collect::<Vec<_>>();

    diesel::insert_into(schema::extracted_texts::table)
        .values(new_extracted_texts)
        .get_results(conn)
        .map_err(|e| str_err!("Error saving new extracted texts {}", e))
}

pub fn get_extracted_text(
    conn: &mut PgConnection,
    id: ExtractedTextId,
) -> Result<Option<ExtractedText>> {
    schema::extracted_texts::table
        .filter(schema::extracted_texts::id.eq(id))
        .get_result(conn)
        .optional()
        .map_err(|e| str_err!("Error loading extracted text {}", e))
}

pub fn get_extracted_text_bulk(
    conn: &mut PgConnection,
    ids: &[ExtractedTextId],
) -> Result<Vec<ExtractedText>> {
    schema::extracted_texts::table
        .filter(schema::extracted_texts::id.eq_any(ids))
        .get_results(conn)
        .map_err(|e| str_err!("Error loading extracted texts {}", e))
}

pub fn get_extracted_text_ids_for_document(
    conn: &mut PgConnection,
    document_id: DocumentId,
) -> Result<Vec<ExtractedTextId>> {
    schema::extracted_texts::table
        .filter(schema::extracted_texts::document_id.eq(document_id))
        .select(schema::extracted_texts::id)
        .get_results(conn)
        .map_err(|e| str_err!("Error loading extracted texts {}", e))
}

pub fn get_extracted_text_ids_for_document_bulk(
    conn: &mut PgConnection,
    document_ids: &[DocumentId],
) -> Result<Vec<ExtractedTextId>> {
    schema::extracted_texts::table
        .filter(schema::extracted_texts::document_id.eq_any(document_ids))
        .select(schema::extracted_texts::id)
        .get_results(conn)
        .map_err(|e| str_err!("Error loading extracted texts {}", e))
}

pub fn get_extracted_texts_for_document(
    conn: &mut PgConnection,
    doc_id: DocumentId,
) -> Result<Vec<ExtractedText>> {
    schema::extracted_texts::table
        .filter(schema::extracted_texts::document_id.eq(doc_id))
        .get_results(conn)
        .map_err(|e| str_err!("Error loading extracted texts {}", e))
}

pub fn get_extracted_texts_for_document_bulk(
    conn: &mut PgConnection,
    doc_ids: &[DocumentId],
) -> Result<Vec<ExtractedText>> {
    schema::extracted_texts::table
        .filter(schema::extracted_texts::document_id.eq_any(doc_ids))
        .get_results(conn)
        .map_err(|e| str_err!("Error loading extracted texts {}", e))
}

pub fn get_extracted_texts(
    conn: &mut PgConnection,
    page_index: i64,
    page_size: i64,
) -> Result<Vec<ExtractedText>> {
    schema::extracted_texts::table
        .limit(page_size)
        .offset(page_index * page_size)
        .load::<ExtractedText>(conn)
        .map_err(|e| str_err!("Error loading extracted text {}", e))
}

pub fn extracted_text_exists(conn: &mut PgConnection, id: ExtractedTextId) -> Result<bool> {
    Ok(get_extracted_text(conn, id)?.is_some())
}

pub fn get_num_extracted_texts(conn: &mut PgConnection) -> Result<i64> {
    schema::extracted_texts::table
        .select(diesel::dsl::count_star())
        .first(conn)
        .map_err(|e| str_err!("Error loading extracted texts {}", e))
}

pub fn get_num_extracted_texts_for_document(
    conn: &mut PgConnection,
    id: DocumentId,
) -> Result<i64> {
    schema::extracted_texts::table
        .filter(schema::extracted_texts::document_id.eq(id))
        .select(diesel::dsl::count_star())
        .first(conn)
        .map_err(|e| str_err!("Error loading extracted texts {}", e))
}

pub fn delete_extracted_text(conn: &mut PgConnection, id: ExtractedTextId) -> Result<usize> {
    diesel::delete(schema::extracted_texts::table.filter(schema::extracted_texts::id.eq(id)))
        .execute(conn)
        .map_err(|e| str_err!("Error deleting extracted text {}", e))
}

pub fn delete_extracted_texts_for_document(
    conn: &mut PgConnection,
    id: DocumentId,
) -> Result<usize> {
    diesel::delete(
        schema::extracted_texts::table.filter(schema::extracted_texts::document_id.eq(id)),
    )
    .execute(conn)
    .map_err(|e| str_err!("Error deleting extracted texts for document {}", e))
}
