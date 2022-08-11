function get_extracted_texts_for_document(req::HTTP.Request)
    id = @extract_id req DocumentId

    extracted_texts = get_extracted_texts_for_document(pool(), id) |> fetch

    HTTP.Response(Status.OK, extracted_texts |> JSON3.write)
end
