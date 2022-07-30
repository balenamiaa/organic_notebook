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

pub mod create_idea;
pub mod delete_idea;
pub mod get_ideas;
pub mod get_ideas_entry;
pub mod get_num_ideas;

pub mod create_idea_ref;
pub mod delete_idea_ref;
pub mod get_idea_refs;
pub mod get_idea_refs_entry;
pub mod get_num_idea_refs;

pub mod get_idea_refs_for_idea;
pub mod get_num_idea_refs_for_idea;

pub mod delete_document;
pub mod get_document_entry;
pub mod get_documents;
pub mod get_num_documents;
pub mod upload_document;

pub mod delete_extracted_texts_for_document;
pub mod extract_texts_for_document;
pub mod get_extracted_texts;
pub mod get_extracted_texts_for_document;
pub mod get_extracted_texts_for_document_bulk;
pub mod get_num_extracted_texts;
pub mod get_num_extracted_texts_for_document;
