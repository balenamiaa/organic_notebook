use diesel::deserialize::FromStaticSqlRow;
use diesel::{Insertable, Queryable};
use diesel_derive_newtype::DieselNewType;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use super::schema::documents;
use super::schema::idea_refs;
use super::schema::ideas;

#[derive(Clone, Copy, Debug, DieselNewType)]
pub struct DocumentId(pub i32);

#[derive(Clone, Copy, Debug, DieselNewType)]
pub struct IdeaId(pub i32);

#[derive(Clone, Copy, Debug, DieselNewType)]
pub struct IdeaRefId(pub i32);

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
    pub label: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdeaRef {
    pub id: IdeaRefId,
    pub doc_page: DocumentPage,
    pub idea_ref: IdeaId,
    pub idea_details: Option<Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewIdea {
    pub label: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewIdeaRef {
    pub doc_page: DocumentPage,
    pub idea_ref: IdeaId,
    pub idea_details: Option<Value>,
}

impl<DB: diesel::backend::Backend, ST0, ST1> Queryable<(ST0, ST1), DB> for Idea
where
    (i32, String): FromStaticSqlRow<(ST0, ST1), DB>,
{
    type Row = (i32, String);

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Ok(Self {
            id: IdeaId(row.0.try_into()?),
            label: row.1.try_into()?,
        })
    }
}

impl<DB: diesel::backend::Backend, ST0, ST1, ST2, ST3, ST4> Queryable<(ST0, ST1, ST2, ST3, ST4), DB>
    for IdeaRef
where
    (i32, i32, Option<i32>, IdeaId, Option<Value>): FromStaticSqlRow<(ST0, ST1, ST2, ST3, ST4), DB>,
{
    type Row = (i32, i32, Option<i32>, IdeaId, Option<Value>);

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Ok(Self {
            id: IdeaRefId(row.0.try_into()?),
            doc_page: DocumentPage {
                document_id: DocumentId(row.1.try_into()?),
                page_number: row.2.try_into()?,
            },
            idea_ref: row.3.try_into()?,
            idea_details: row.4.try_into()?,
        })
    }
}

impl Insertable<ideas::table> for NewIdea {
    type Values =
        <(Option<diesel::dsl::Eq<ideas::label, String>>,) as Insertable<ideas::table>>::Values;

    fn values(self) -> Self::Values {
        use diesel::prelude::*;

        (Some(ideas::label.eq(self.label)),).values()
    }
}

impl Insertable<idea_refs::table> for NewIdeaRef {
    type Values = <(
        Option<diesel::dsl::Eq<idea_refs::document_id, i32>>,
        Option<diesel::dsl::Eq<idea_refs::document_page, i32>>,
        Option<diesel::dsl::Eq<idea_refs::idea_ref, i32>>,
        Option<diesel::dsl::Eq<idea_refs::idea_details, Value>>,
    ) as Insertable<idea_refs::table>>::Values;

    fn values(self) -> Self::Values {
        use diesel::prelude::*;

        (
            Some(idea_refs::document_id.eq(self.doc_page.document_id.0)),
            self.doc_page
                .page_number
                .map(|x| idea_refs::document_page.eq(x)),
            Some(idea_refs::idea_ref.eq(self.idea_ref.0)),
            self.idea_details.map(|x| idea_refs::idea_details.eq(x)),
        )
            .values()
    }
}
