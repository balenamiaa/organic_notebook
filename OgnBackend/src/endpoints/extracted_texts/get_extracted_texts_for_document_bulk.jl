function get_extracted_texts_for_document_bulk(req::HTTP.Request)
    body = HTTP.payload(req, String)

    ids = [
        DocumentId(tryparse(Int, x)) for
        x in split(body, ",") if tryparse(Int, x) !== nothing
    ]

    extracted_texts = get_extracted_texts_for_document_bulk(pool(), ids) |> fetch

    HTTP.Response(Status.OK, extracted_texts |> JSON3.write)
end
