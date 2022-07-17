macro_rules! common_endpoint_imports {
    () => {
        #[allow(unused_imports)]
        use crate::{DbPool, DOCUMENT_ROOTDIR};
        #[allow(unused_imports)]
        use actix_multipart::Multipart;
        #[allow(unused_imports)]
        use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
        #[allow(unused_imports)]
        use actix_web::{get, post, web, Responder};
        #[allow(unused_imports)]
        use futures_util::StreamExt;
        #[allow(unused_imports)]
        use ogn_db::models::*;
        #[allow(unused_imports)]
        use std::ops::DerefMut;
    };
}

pub mod get_document_entry;
pub mod get_documents;
pub mod get_idea_entry;
pub mod get_idea_refs;
pub mod get_ideas;
pub mod get_num_documents;
pub mod get_num_idea_refs;
pub mod get_num_ideas;
pub mod upload_document;
