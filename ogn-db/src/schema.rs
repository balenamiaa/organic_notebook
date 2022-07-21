// @generated automatically by Diesel CLI.

diesel::table! {
    documents (id) {
        id -> Int4,
        title -> Text,
        document_details -> Nullable<Json>,
        filetype -> Varchar,
    }
}

diesel::table! {
    ideas (id) {
        id -> Int4,
        document_id -> Int4,
        document_page -> Nullable<Int4>,
        idea_text -> Text,
        idea_details -> Nullable<Json>,
    }
}

diesel::joinable!(ideas -> documents (document_id));

diesel::allow_tables_to_appear_in_same_query!(
    documents,
    ideas,
);
