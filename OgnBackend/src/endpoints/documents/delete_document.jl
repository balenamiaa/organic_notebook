function delete_document(req::HTTP.Request)
    id = @extract_id req DocumentId

    res = delete_document(pool(), id) |> fetch
    res === false && return HTTP.Response(Status.CONFLICT, "document not found")

    HTTP.Response(Status.OK, "")
end
