function delete_idea(req::HTTP.Request)
    id = @extract_id req IdeaId

    res = delete_idea(pool(), id) |> fetch
    res === false && return HTTP.Response(Status.CONFLICT, "idea not found")

    HTTP.Response(Status.OK, "")
end
