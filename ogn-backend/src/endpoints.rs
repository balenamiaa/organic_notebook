macro_rules! common_endpoint_imports {
    () => {
        #[allow(unused_imports)]
        use crate::{DbPool, DOCUMENT_ROOTDIR};
        #[allow(unused_imports)]
        use actix_multipart::Multipart;
        #[allow(unused_imports)]
        use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
        #[allow(unused_imports)]
        use actix_web::{delete, get, post, put, web, Responder};
        #[allow(unused_imports)]
        use futures_util::StreamExt;
        #[allow(unused_imports)]
        use ogn_db::models::*;
        #[allow(unused_imports)]
        use std::ops::DerefMut;
    };
}

pub mod create_idea; // #[post("/api/ideas")]
pub mod get_ideas; // #[get("/api/ideas")]
pub mod get_ideas_entry; // #[get("/api/ideas/{id}")]
pub mod get_num_ideas; // #[get("/api/ideas/num")]
pub mod delete_idea; // #[delete("/api/ideas/{id}")]

pub mod create_idea_ref; // #[post("/api/idea_refs")]
pub mod get_idea_refs; // #[get("/api/idea_refs")]
pub mod get_idea_refs_entry; // #[get("/api/idea_refs/{id}")]
pub mod get_num_idea_refs; // #[get("/api/idea_refs/num")]
pub mod delete_idea_ref; // #[delete("/api/idea_refs/{id}")]

pub mod get_idea_refs_for_idea; // #[get("/api/idea_refs_for_idea/{id}")]
pub mod get_num_idea_refs_for_idea; // #[get("/api/idea_refs_for_idea/{id}/num")]

pub mod upload_document; // #[post("/api/documents")]
pub mod get_documents; // #[get("/api/documents")]
pub mod get_document_entry; // #[get("/api/documents/{id}")]
pub mod get_num_documents; // #[get("/api/documents/num")]
pub mod delete_document; // #[delete("/api/documents/{id}")]

pub mod extract_texts_for_document; // #[post("/api/extracted_texts/{document_id}")]
pub mod delete_extracted_texts_for_document; // #[delete("/api/extracted_texts/{document_id}")]
