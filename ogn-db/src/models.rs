use diesel::{Insertable, Queryable, Table};
use diesel::deserialize::FromStaticSqlRow;
use diesel_derive_newtype::DieselNewType;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use super::schema::documents;
use super::schema::ideas;

#[derive(Clone, Copy, Debug, DieselNewType)]
pub struct DocumentId(pub i32);

#[derive(Clone, Copy, Debug, DieselNewType)]
pub struct IdeaId(pub i32);

#[derive(Clone, Debug, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = documents)]
pub struct Document {
    pub id: DocumentId,
    pub title: String,
    pub document_details: Option<Value>,
    pub filetype: String,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct DocumentPage {
    pub document_id: DocumentId,
    pub page_number: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Idea {
    pub id: IdeaId,
    pub doc_page: DocumentPage,
    pub idea_text: String,
    pub idea_details: Option<Value>,
}

impl<DB: diesel::backend::Backend, ST0, ST1, ST2, ST3, ST4> Queryable<(ST0, ST1, ST2, ST3, ST4), DB>
for Idea
    where
        (i32, i32, Option<i32>, String, Option<Value>): FromStaticSqlRow<(ST0, ST1, ST2, ST3, ST4), DB>,
{
    type Row = (i32, i32, Option<i32>, String, Option<Value>);

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Ok(Self {
            id: IdeaId(row.0.try_into()?),
            doc_page: DocumentPage {
                document_id: DocumentId(row.1.try_into()?),
                page_number: row.2.try_into()?,
            },
            idea_text: row.3.try_into()?,
            idea_details: row.4.try_into()?,
        })
    }
}

impl Insertable<ideas::table> for Idea {
    type Values = <(
        Option<diesel::dsl::Eq<ideas::id, i32>>,
        Option<diesel::dsl::Eq<ideas::document_id, i32>>,
        Option<diesel::dsl::Eq<ideas::document_page, i32>>,
        Option<diesel::dsl::Eq<ideas::idea_text, String>>,
        Option<diesel::dsl::Eq<ideas::idea_details, Value>>,
    ) as Insertable<ideas::table>>::Values;

    fn values(self) -> Self::Values {
        use diesel::prelude::*;

        (
            Some(ideas::id.eq(self.id.0)),
            Some(ideas::document_id.eq(self.doc_page.document_id.0)),
            self.doc_page
                .page_number
                .map(|x| ideas::document_page.eq(x)),
            Some(ideas::idea_text.eq(self.idea_text)),
            self.idea_details.map(|x| ideas::idea_details.eq(x)),
        )
            .values()
    }
}