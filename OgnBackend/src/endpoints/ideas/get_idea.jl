function get_idea(req::HTTP.Request)
    id = @extract_id req IdeaId

    res = get_idea(pool(), id) |> fetch
    res === nothing && return HTTP.Response(Status.BADREQUEST, "idea not found")

    HTTP.Response(Status.OK, res |> JSON3.write)
end
