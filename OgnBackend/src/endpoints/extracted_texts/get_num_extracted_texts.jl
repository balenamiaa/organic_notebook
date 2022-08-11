function get_num_extracted_texts(req::HTTP.Request)
    num = get_num_extracted_texts(pool()) |> fetch
    num === nothing && return HTTP.Response(
        Status.INTERNALERROR,
        "failed to get number of extracted texts",
    )

    return HTTP.Response(Status.OK, string(num))
end
