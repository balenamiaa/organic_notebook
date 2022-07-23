use diesel::{Insertable, Queryable};
use diesel_derive_newtype::DieselNewType;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, DieselNewType)]
pub struct DocumentId(pub i32);

#[derive(Clone, Copy, Debug, DieselNewType)]
pub struct IdeaId(pub i32);

#[derive(Clone, Copy, Debug, DieselNewType)]
pub struct IdeaRefId(pub i32);

#[derive(Clone, Copy, Debug, DieselNewType)]
pub struct ExtractedTextId(pub i32);

#[derive(Clone, Debug, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::documents)]
pub struct Document {
    pub id: DocumentId,
    pub title: String,
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
    pub idea_ref_text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewIdea {
    pub label: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewIdeaRef {
    pub doc_page: DocumentPage,
    pub idea_ref: IdeaId,
    pub idea_ref_text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtractedText {
    pub id: ExtractedTextId,
    pub content: String,
    pub doc_page: DocumentPage,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewExtractedText {
    pub content: String,
    pub doc_page: DocumentPage,
}
