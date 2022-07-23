use diesel::PgConnection;
use diesel::prelude::*;

use crate::{Idea, IdeaId, ideas, NewIdea};
use crate::result::Result;
use crate::schema;

pub fn create_idea(conn: &mut PgConnection, label: &str) -> Result<Idea> {
    let new_idea = NewIdea {
        label: label.to_string(),
    };

    diesel::insert_into(schema::ideas::table)
        .values(new_idea)
        .get_result(conn)
        .map_err(|e| str_err!("Error saving new idea {}", e))
}

pub fn get_idea(conn: &mut PgConnection, id: IdeaId) -> Result<Option<Idea>> {
    schema::ideas::table
        .filter(schema::ideas::id.eq(id))
        .get_result(conn)
        .optional()
        .map_err(|e| str_err!("Error loading idea {}", e))
}

pub fn get_ideas(conn: &mut PgConnection, page_index: i64, page_size: i64) -> Result<Vec<Idea>> {
    schema::ideas::table
        .limit(page_size)
        .offset(page_index * page_size)
        .load::<Idea>(conn)
        .map_err(|e| str_err!("Error loading ideas {}", e))
}

pub fn idea_exists(conn: &mut PgConnection, id: IdeaId) -> Result<bool> {
    Ok(get_idea(conn, id)?.is_some())
}

pub fn get_num_ideas(conn: &mut PgConnection) -> Result<i64> {
    schema::ideas::table
        .select(diesel::dsl::count_star())
        .first(conn)
        .map_err(|e| str_err!("Error loading ideas {}", e))
}

pub fn delete_idea(conn: &mut PgConnection, idea_id: IdeaId) -> Result<usize> {
    diesel::delete(schema::ideas::table.filter(schema::ideas::id.eq(idea_id)))
        .execute(conn)
        .map_err(|e| str_err!("Error deleting idea {}", e))
}