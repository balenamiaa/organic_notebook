function get_num_documents(::HTTP.Request)
    num = get_num_documents(pool()) |> fetch
    num === nothing &&
        return HTTP.Response(Status.INTERNALERROR, "failed to get number of documents")

    return HTTP.Response(Status.OK, string(num))
end
