struct DocumentId
    id::Int32
end

struct IdeaId
    id::Int32
end

struct IdeaRefId
    id::Int32
end

struct ExtractedTextId
    id::Int32
end

struct DocumentPage
    document_id::DocumentId
    page_number::Union{Nothing,Some{Int32}}
end

struct Document
    id::DocumentId
    title::String
    filetype::String
end

struct Idea
    id::IdeaId
    label::String
end

struct IdeaRef
    id::IdeaRefId
    doc_page::DocumentPage
    idea_ref::IdeaId
    idea_ref_text::String
end

struct ExtractedText
    id::ExtractedTextId
    content::String
    doc_page::DocumentPage
end

struct NewIdea
    label::String
end

struct NewIdeaRef
    doc_page::DocumentPage
    idea_ref::IdeaId
    idea_ref_text::String
end

struct NewExtractedText
    content::String
    doc_page::DocumentPage
end

@impl_id_wrapper(DocumentId, IdeaId, IdeaRefId, ExtractedTextId)
@impl_structtype(
    Document,
    DocumentPage,
    Idea,
    IdeaRef,
    ExtractedText,
    NewIdea,
    NewIdeaRef,
    NewExtractedText
)
