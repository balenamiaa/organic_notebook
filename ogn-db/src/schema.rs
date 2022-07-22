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
    idea_refs (id) {
        id -> Int4,
        document_id -> Int4,
        document_page -> Nullable<Int4>,
        idea_ref -> Int4,
        idea_details -> Nullable<Json>,
    }
}

diesel::table! {
    ideas (id) {
        id -> Int4,
        label -> Text,
    }
}

diesel::joinable!(idea_refs -> documents (document_id));
diesel::joinable!(idea_refs -> ideas (idea_ref));

diesel::allow_tables_to_appear_in_same_query!(documents, idea_refs, ideas,);
