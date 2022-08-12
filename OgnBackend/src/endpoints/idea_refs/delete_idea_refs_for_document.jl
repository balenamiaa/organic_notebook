function delete_idea_refs_for_document(req::HTTP.Request)
    id = @extract_id req DocumentId

    res = delete_idea_refs_for_document(pool(), id) |> fetch
    res === false && return HTTP.Response(Status.BADREQUEST, "document not found")

    HTTP.Response(Status.OK, "")
end
