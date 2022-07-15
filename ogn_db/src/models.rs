use diesel_derive_newtype::DieselNewType;
use serde_json::Value;

use super::schema::documents;

#[derive(Clone, Copy, Debug, DieselNewType)]
pub struct DocumentId(pub i32);

#[derive(Queryable)]
pub struct Document {
    pub id: DocumentId,
    pub title: String,
    pub document_details: Option<Value>,
}


#[derive(Insertable)]
#[diesel(table_name = documents)]
pub struct NewDocument {
    pub id: DocumentId,
    pub title: String,
    pub document_details: Option<Value>,
}