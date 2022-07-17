macro_rules! common_endpoint_imports {
    () => {
        use std::ops::DerefMut;
        use actix_multipart::Multipart;
        use actix_web::{get, post, Responder, web};
        use actix_web::error::{ErrorInternalServerError, ErrorBadRequest};
        use futures_util::StreamExt;

        use ogn_db::models::*;

        use crate::{DbPool, DOCUMENT_ROOTDIR};
    };
}

pub mod upload_document;
pub mod get_num_ideas;
pub mod get_num_documents;
pub mod get_ideas;
pub mod get_document_entry;
pub mod get_idea_entry;
pub mod get_documents;
pub mod get_num_idea_refs;
pub mod get_idea_refs;






















