function extract_texts_for_document(req::HTTP.Request)
    id = @extract_id req DocumentId
    fetch(document_exists(pool(), id)) ||
        return HTTP.Response(Status.BADREQUEST, "document not found")

    texts = extract_texts_for_document(id)
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
