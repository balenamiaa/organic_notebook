function get_num_idea_refs_for_idea(req::HTTP.Request)
    id = @extract_id req IdeaId

    num = get_num_idea_refs_for_idea(pool(), id) |> fetch
    num === nothing && return HTTP.Response(
        Status.INTERNALERROR,
        "failed to get number of idea_refs for idea id=$id",
    )

    return HTTP.Response(Status.OK, string(num))
end
