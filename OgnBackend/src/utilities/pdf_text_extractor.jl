function extract_texts_for_document(document_id::DocumentId)
    filepath = get_filepath_for_document(document_id)
    filepath === nothing && return nothing


    doc = pdDocOpen(filepath)

    num_page = pdDocGetPageCount(doc)

    page_texts = Vector{String}(undef, num_page)
    @inbounds for i = 1:num_page
        page = pdDocGetPage(doc, i)
        page_texts[i] = String(take!(pdPageExtractText(IOBuffer(), page)))
    end

    pdDocClose(doc)

    page_texts
end
