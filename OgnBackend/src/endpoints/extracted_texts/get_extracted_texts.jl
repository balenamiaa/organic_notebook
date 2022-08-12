function get_extracted_texts(req::HTTP.Request)
    pagination_result = extract_pagination(req)

    if !isnothing(pagination_result.err_str)
        return HTTP.Response(Status.BADREQUEST, something(pagination_result.err_str))
    end

    pagination = pagination_result.pagination

    if isnothing(pagination)
        items = get_extracted_texts(pool()) |> fetch
        result = PaginatedResult(items, length(items), 0)

        return HTTP.Response(Status.OK, result |> JSON3.write)
    else
        pagination = something(pagination)

        items =
            get_extracted_texts(pool(), (pagination.page_num, pagination.page_size)) |>
            fetch
        result = PaginatedResult(
            items,
            length(items),
            get_num_documents(pool()) |> fetch,
            pagination,
        )

        return HTTP.Response(Status.OK, result |> JSON3.write)
    end
end
