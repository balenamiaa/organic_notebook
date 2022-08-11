function get_extracted_texts(req::HTTP.Request)
    pagination_result = extract_pagination(req)

    if !isnothing(pagination_result.err_str)
        return HTTP.Response(Status.BADREQUEST, something(pagination_result.err_str))
    end

    pagination = pagination_result.pagination

    if isnothing(pagination)
        result = get_extracted_texts(pool()) |> fetch

        return HTTP.Response(Status.OK, result |> JSON3.write)
    else
        pagination = something(pagination)

        result =
            get_extracted_texts(pool(), (pagination.page_num, pagination.page_size)) |>
            fetch
        result = PaginatedResult(
            result,
            length(result),
            get_num_documents(pool()) |> fetch,
            pagination,
        )

        return HTTP.Response(Status.OK, result |> JSON3.write)
    end
end
