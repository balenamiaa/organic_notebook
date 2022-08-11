function get_idea_refs(req::HTTP.Request)
    pagination_result = extract_pagination(req)

    if !isnothing(pagination_result.err_str)
        return HTTP.Response(Status.BADREQUEST, pagination_result.err_str |> something)
    end

    pagination = pagination_result.pagination

    if isnothing(pagination)
        result = get_idea_refs(pool()) |> fetch

        return HTTP.Response(Status.OK, result |> JSON3.write)
    else
        pagination = something(pagination)

        result = get_idea_refs(pool(), (pagination.page_num, pagination.page_size)) |> fetch
        result = PaginatedResult(
            result,
            length(result),
            get_num_idea_refs(pool()) |> fetch,
            pagination,
        )

        return HTTP.Response(Status.OK, result |> JSON3.write)
    end
end