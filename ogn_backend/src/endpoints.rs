macro_rules! common_endpoint_imports {
    () => {
        #[allow(unused_imports)]
        use std::ops::DerefMut;
        #[allow(unused_imports)]
        use actix_multipart::Multipart;
        #[allow(unused_imports)]
        use actix_web::{get, post, Responder, web};
        #[allow(unused_imports)]
        use actix_web::error::{ErrorInternalServerError, ErrorBadRequest};
        #[allow(unused_imports)]
        use futures_util::StreamExt;
        #[allow(unused_imports)]
        use ogn_db::models::*;
        #[allow(unused_imports)]
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






















