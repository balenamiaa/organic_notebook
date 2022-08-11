function get_idea_ref(req::HTTP.Request)
    id = @extract_id req IdeaRefId

    res = get_idea_ref(pool(), id) |> fetch
    res === nothing && return HTTP.Response(Status.BADREQUEST, "idea_ref not found")

    HTTP.Response(Status.OK, res |> JSON3.write)
end
