use diesel::{self, PgConnection};

use models::DocumentId;
use result::Result;

use crate::models::{
    Document, DocumentPage, ExtractedText, ExtractedTextId, Idea, IdeaId, IdeaRef, IdeaRefId,
    NewExtractedText, NewIdea, NewIdeaRef,
};

#[macro_use]
pub mod result;
pub mod documents;
pub mod extracted_texts;
pub mod idea_refs;
pub mod ideas;

pub mod model_impls;
pub mod models;
pub mod schema;

pub fn get_idea_refs_for_idea(
    conn: &mut PgConnection,
    id: IdeaId,
    page_index: i64,
    page_size: i64,
) -> Result<Vec<IdeaRef>> {
    use diesel::prelude::*;

    schema::idea_refs::table
        .filter(schema::idea_refs::idea_ref.eq(id))
        .select(schema::idea_refs::all_columns)
        .limit(page_size)
        .offset(page_index * page_size)
        .load(conn)
        .map_err(|e| str_err!("Error loading idea refs {}", e))
}

pub fn get_num_idea_refs_for_idea(conn: &mut PgConnection, id: IdeaId) -> Result<i64> {
    use diesel::prelude::*;

    schema::idea_refs::table
        .filter(schema::idea_refs::idea_ref.eq(id))
        .select(diesel::dsl::count_star())
        .first(conn)
        .map_err(|e| str_err!("Error loading idea refs {}", e))
}
