function get_num_ideas(::HTTP.Request)
    num = get_num_ideas(pool()) |> fetch
    num === nothing &&
        return HTTP.Response(Status.INTERNALERROR, "failed to get number of ideas")

    return HTTP.Response(Status.OK, string(num))
end
