function get_num_idea_refs(::HTTP.Request)
    num = get_num_idea_refs(pool()) |> fetch
    num === nothing &&
        return HTTP.Response(Status.INTERNALERROR, "failed to get number of idea_refs")

    return HTTP.Response(Status.OK, string(num))
end
