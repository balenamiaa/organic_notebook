function get_idea_refs_for_idea(req::HTTP.Request)
    id = @extract_id req IdeaId

    pagination_result = extract_pagination(req)

    if !isnothing(pagination_result.err_str)
        return HTTP.Response(Status.BADREQUEST, pagination_result.err_str |> something)
    end

    pagination = pagination_result.pagination

    if isnothing(pagination)
        result = get_idea_refs_for_idea(pool(), id) |> fetch

        return HTTP.Response(Status.OK, result |> JSON3.write)
    else
        pagination = something(pagination)

        result =
            get_idea_refs_for_idea(
                pool(),
                id,
                (pagination.page_num, pagination.page_size),
            ) |> fetch
        result = PaginatedResult(
            result,
            length(result),
            get_num_idea_refs_for_idea(pool(), id) |> fetch,
            pagination,
        )

        return HTTP.Response(Status.OK, result |> JSON3.write)
    end
end
