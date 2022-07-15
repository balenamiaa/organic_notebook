// @generated automatically by Diesel CLI.

diesel::table! {
    documents (id) {
        id -> Int4,
        title -> Text,
        document_details -> Nullable<Json>,
    }
}
