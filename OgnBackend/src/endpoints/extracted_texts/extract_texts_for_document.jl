function extract_texts_for_document(req::HTTP.Request)
    id = @extract_id req DocumentId
    fetch(document_exists(pool(), id)) ||
        return HTTP.Response(Status.BADREQUEST, "document not found")

    fetch(extracted_texts_for_document_exists(pool(), id)) &&
        return HTTP.Response(Status.CONFLICT, "extracted texts for document already exists")

    query_params = HTTP.queryparams(req.url)
    @inline get_query_params(key) = haskey(query_params, key) ? query_params[key] : nothing

    extraction_method =
        if (
            extraction_method_str = get_query_params("extraction_method"); extraction_method_str
        ) === nothing
            PopplerPdfToText
        else
            if extraction_method_str == "poppler"
                PopplerPdfToText
            elseif extraction_method_str == "julia"
                JuliaPdfToText
            else
                return HTTP.Response(Status.BADREQUEST, "invalid extraction method")
            end
        end

    texts = extract_texts_for_document(id, extraction_method)
    new_extracted_texts = [
        NewExtractedText(text, DocumentPage(id, Some(page))) for
        (page, text) in enumerate(texts)
    ]

    extracted_texts = create_extracted_text_bulk(pool(), new_extracted_texts) |> fetch
    extracted_texts === nothing && return HTTP.Response(
        Status.INTERNALERROR,
        "extracted_texts couldn't not be inserted into database",
    )

    HTTP.Response(Status.OK, extracted_texts |> JSON3.write)
end
