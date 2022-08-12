function delete_idea_refs_for_idea(req::HTTP.Request)
    id = @extract_id req IdeaId

    res = delete_idea_refs_for_idea(pool(), id) |> fetch
    res === false && return HTTP.Response(Status.BADREQUEST, "idea not found")

    HTTP.Response(Status.OK, "")
end
