function delete_idea_ref(req::HTTP.Request)
    id = @extract_id req IdeaRefId

    res = delete_idea_ref(pool(), id) |> fetch
    res === false && return HTTP.Response(Status.BADREQUEST, "idea_ref not found")

    HTTP.Response(Status.OK, "")
end
