include("endpoints/documents/upload_document.jl")
include("endpoints/documents/delete_document.jl")
include("endpoints/documents/get_document.jl")
include("endpoints/documents/get_documents.jl")
include("endpoints/documents/get_num_documents.jl")

include("endpoints/ideas/create_idea.jl")
include("endpoints/ideas/delete_idea.jl")
include("endpoints/ideas/get_idea.jl")
include("endpoints/ideas/get_ideas.jl")
include("endpoints/ideas/get_num_ideas.jl")

include("endpoints/idea_refs/create_idea_ref.jl")
include("endpoints/idea_refs/delete_idea_ref.jl")
include("endpoints/idea_refs/get_idea_ref.jl")
include("endpoints/idea_refs/get_idea_refs.jl")
include("endpoints/idea_refs/get_num_idea_refs.jl")
include("endpoints/idea_refs/get_idea_refs_for_idea.jl")
include("endpoints/idea_refs/get_num_idea_refs_for_idea.jl")

include("endpoints/extracted_texts/extract_texts_for_document.jl")
include("endpoints/extracted_texts/delete_extracted_texts_for_document.jl")
include("endpoints/extracted_texts/get_extracted_texts_for_document.jl")
include("endpoints/extracted_texts/get_extracted_texts_for_document_bulk.jl")
include("endpoints/extracted_texts/get_extracted_texts.jl")
include("endpoints/extracted_texts/get_num_extracted_texts.jl")

function register_endpoints!(router)
    HTTP.register!(router, "POST", "/api/documents", upload_document)
    HTTP.register!(router, "DELETE", "/api/documents/{id}", delete_document)
    HTTP.register!(router, "GET", "/api/documents/{id}", get_document)
    HTTP.register!(router, "GET", "/api/documents", get_documents)
    HTTP.register!(router, "GET", "/api/documents//num", get_num_documents)

    HTTP.register!(router, "POST", "/api/ideas", create_idea)
    HTTP.register!(router, "DELETE", "/api/ideas/{id}", delete_idea)
    HTTP.register!(router, "GET", "/api/ideas/{id}", get_idea)
    HTTP.register!(router, "GET", "/api/ideas", get_ideas)
    HTTP.register!(router, "GET", "/api/ideas//num", get_num_ideas)

    HTTP.register!(router, "POST", "/api/idea_refs", create_idea_ref)
    HTTP.register!(router, "DELETE", "/api/idea_refs/{id}", delete_idea_ref)
    HTTP.register!(router, "GET", "/api/idea_refs/{id}", get_idea_ref)
    HTTP.register!(router, "GET", "/api/idea_refs", get_idea_refs)
    HTTP.register!(router, "GET", "/api/idea_refs//num", get_num_idea_refs)
    HTTP.register!(router, "GET", "/api/idea_refs/ideas/{id}", get_idea_refs_for_idea)
    HTTP.register!(
        router,
        "GET",
        "/api/idea_refs/ideas/{id}/num",
        get_num_idea_refs_for_idea,
    )

    HTTP.register!(
        router,
        "POST",
        "/api/extracted_texts/document/{id}",
        extract_texts_for_document,
    )
    HTTP.register!(
        router,
        "DELETE",
        "/api/extracted_texts/document/{id}",
        delete_extracted_texts_for_document,
    )
    HTTP.register!(
        router,
        "GET",
        "/api/extracted_texts/document/{id}",
        get_extracted_texts_for_document,
    )
    HTTP.register!(
        router,
        "GET",
        "/api/extracted_texts/document",
        get_extracted_texts_for_document_bulk,
    )
    HTTP.register!(router, "GET", "/api/extracted_texts", get_extracted_texts)
    HTTP.register!(router, "GET", "/api/extracted_texts//num", get_num_extracted_texts)
end
