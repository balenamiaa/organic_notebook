use diesel::prelude::*;
use diesel::PgConnection;
use serde_json::Value;

use crate::result::Result;
use crate::schema;
use crate::{DocumentPage, IdeaId, IdeaRef, IdeaRefId, NewIdeaRef};

pub fn create_idea_ref(
    conn: &mut PgConnection,
    doc_page: DocumentPage,
    idea_ref: IdeaId,
    idea_details: Option<Value>,
) -> Result<IdeaRef> {
    let new_idea = NewIdeaRef {
        doc_page,
        idea_ref,
        idea_details,
    };

    diesel::insert_into(schema::idea_refs::table)
        .values(new_idea)
        .get_result(conn)
        .map_err(|e| str_err!("Error saving new idea ref {}", e))
}

pub fn get_idea_ref(conn: &mut PgConnection, id: IdeaRefId) -> Result<Option<IdeaRef>> {
    schema::idea_refs::table
        .filter(schema::idea_refs::id.eq(id))
        .get_result(conn)
        .optional()
        .map_err(|e| str_err!("Error loading idea ref {}", e))
}

pub fn get_idea_refs(
    conn: &mut PgConnection,
    page_index: i64,
    page_size: i64,
) -> Result<Vec<IdeaRef>> {
    schema::idea_refs::table
        .limit(page_size)
        .offset(page_index * page_size)
        .load::<IdeaRef>(conn)
        .map_err(|e| str_err!("Error loading idea refs {}", e))
}

pub fn idea_ref_exists(conn: &mut PgConnection, id: IdeaRefId) -> Result<bool> {
    Ok(get_idea_ref(conn, id)?.is_some())
}

pub fn get_num_idea_refs(conn: &mut PgConnection) -> Result<i64> {
    schema::idea_refs::table
        .select(diesel::dsl::count_star())
        .first(conn)
        .map_err(|e| str_err!("Error loading idea refs {}", e))
}

pub fn delete_idea_refs(conn: &mut PgConnection, idea_refs_id: IdeaRefId) -> Result<usize> {
    diesel::delete(schema::idea_refs::table.filter(schema::idea_refs::id.eq(idea_refs_id)))
        .execute(conn)
        .map_err(|e| str_err!("Error deleting idea_refs {}", e))
}
