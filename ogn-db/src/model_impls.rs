use diesel::deserialize::FromStaticSqlRow;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::models::{ExtractedText, ExtractedTextId, IdeaRefId, NewExtractedText};
use crate::{schema, DocumentId, DocumentPage, Idea, IdeaId, IdeaRef, NewIdea, NewIdeaRef};

impl<'de> Deserialize<'de> for DocumentId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let id = s
            .parse::<i32>()
            .map_err(|_| serde::de::Error::custom("couldn't parse document id from payload"))?;
        Ok(DocumentId(id))
    }
}

impl Serialize for DocumentId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for IdeaId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let id = s
            .parse::<i32>()
            .map_err(|_| serde::de::Error::custom("couldn't parse idea id from payload"))?;
        Ok(IdeaId(id))
    }
}

impl Serialize for IdeaId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for IdeaRefId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let id = s
            .parse::<i32>()
            .map_err(|_| serde::de::Error::custom("couldn't parse idea id from payload"))?;
        Ok(IdeaRefId(id))
    }
}

impl Serialize for IdeaRefId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for ExtractedTextId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let id = s
            .parse::<i32>()
            .map_err(|_| serde::de::Error::custom("couldn't parse idea id from payload"))?;
        Ok(ExtractedTextId(id))
    }
}

impl Serialize for ExtractedTextId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
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
    (i32, i32, Option<i32>, IdeaId, String): FromStaticSqlRow<(ST0, ST1, ST2, ST3, ST4), DB>,
{
    type Row = (i32, i32, Option<i32>, IdeaId, String);

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Ok(Self {
            id: IdeaRefId(row.0.try_into()?),
            doc_page: DocumentPage {
                document_id: DocumentId(row.1.try_into()?),
                page_number: row.2.try_into()?,
            },
            idea_ref: row.3.try_into()?,
            idea_ref_text: row.4.try_into()?,
        })
    }
}

impl Insertable<schema::ideas::table> for NewIdea {
    type Values = <(Option<diesel::dsl::Eq<schema::ideas::label, String>>,) as Insertable<
        schema::ideas::table,
    >>::Values;

    fn values(self) -> Self::Values {
        use diesel::prelude::*;

        (Some(schema::ideas::label.eq(self.label)),).values()
    }
}

impl Insertable<schema::idea_refs::table> for NewIdeaRef {
    type Values = <(
        Option<diesel::dsl::Eq<schema::idea_refs::document_id, i32>>,
        Option<diesel::dsl::Eq<schema::idea_refs::document_page, i32>>,
        Option<diesel::dsl::Eq<schema::idea_refs::idea_ref, i32>>,
        Option<diesel::dsl::Eq<schema::idea_refs::idea_ref_text, String>>,
    ) as Insertable<schema::idea_refs::table>>::Values;

    fn values(self) -> Self::Values {
        use diesel::prelude::*;

        (
            Some(schema::idea_refs::document_id.eq(self.doc_page.document_id.0)),
            self.doc_page
                .page_number
                .map(|x| schema::idea_refs::document_page.eq(x)),
            Some(schema::idea_refs::idea_ref.eq(self.idea_ref.0)),
            Some(schema::idea_refs::idea_ref_text.eq(self.idea_ref_text)),
        )
            .values()
    }
}

impl<DB: diesel::backend::Backend, ST0, ST1, ST2, ST3> Queryable<(ST0, ST1, ST2, ST3), DB>
    for ExtractedText
where
    (ExtractedTextId, String, DocumentId, i32): FromStaticSqlRow<(ST0, ST1, ST2, ST3), DB>,
{
    type Row = (ExtractedTextId, String, DocumentId, i32);

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Ok(Self {
            id: row.0.try_into()?,
            content: row.1.try_into()?,
            doc_page: DocumentPage {
                document_id: row.2.try_into()?,
                page_number: row.3.try_into()?,
            },
        })
    }
}

impl Insertable<schema::extracted_texts::table> for NewExtractedText {
    type Values = <(
        Option<diesel::dsl::Eq<schema::extracted_texts::content, String>>,
        Option<diesel::dsl::Eq<schema::extracted_texts::document_id, DocumentId>>,
        Option<diesel::dsl::Eq<schema::extracted_texts::document_page, i32>>,
    ) as Insertable<schema::extracted_texts::table>>::Values;

    fn values(self) -> Self::Values {
        use diesel::prelude::*;

        (
            Some(schema::extracted_texts::content.eq(self.content)),
            Some(schema::extracted_texts::document_id.eq(self.doc_page.document_id)),
            Some(
                schema::extracted_texts::document_page
                    .eq(self.doc_page.page_number.expect("malformed data")),
            ),
        )
            .values()
    }
}
