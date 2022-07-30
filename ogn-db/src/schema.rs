// @generated automatically by Diesel CLI.

diesel::table! {
    documents (id) {
        id -> Int4,
        title -> Text,
        filetype -> Varchar,
    }
}

diesel::table! {
    extracted_texts (id) {
        id -> Int4,
        content -> Text,
        document_id -> Int4,
        document_page -> Int4,
    }
}

diesel::table! {
    idea_refs (id) {
        id -> Int4,
        document_id -> Int4,
        document_page -> Nullable<Int4>,
        idea_ref -> Int4,
        idea_ref_text -> Text,
    }
}

diesel::table! {
    ideas (id) {
        id -> Int4,
        label -> Text,
    }
}

diesel::joinable!(extracted_texts -> documents (document_id));
diesel::joinable!(idea_refs -> documents (document_id));
diesel::joinable!(idea_refs -> ideas (idea_ref));

diesel::allow_tables_to_appear_in_same_query!(documents, extracted_texts, idea_refs, ideas,);
