function get_document(req::HTTP.Request)
    id = @extract_id req DocumentId

    res = get_document(pool(), id) |> fetch
    res === nothing && return HTTP.Response(Status.BADREQUEST, "document not found")

    HTTP.Response(Status.OK, res |> JSON3.write)
end
