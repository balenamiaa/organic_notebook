function get_idea_refs_for_idea(req::HTTP.Request)
    id = @extract_id req IdeaId

    pagination_result = extract_pagination(req)

    if !isnothing(pagination_result.err_str)
        return HTTP.Response(Status.BADREQUEST, pagination_result.err_str |> something)
    end

    pagination = pagination_result.pagination

    if isnothing(pagination)
        items = get_idea_refs_for_idea(pool(), id) |> fetch
        result = PaginatedResult(items, length(items), 0)

        return HTTP.Response(Status.OK, result |> JSON3.write)
    else
        pagination = something(pagination)

        items =
            get_idea_refs_for_idea(
                pool(),
                id,
                (pagination.page_num, pagination.page_size),
            ) |> fetch
        result = PaginatedResult(
            items,
            length(items),
            get_num_idea_refs_for_idea(pool(), id) |> fetch,
            pagination,
        )

        return HTTP.Response(Status.OK, result |> JSON3.write)
    end
end
