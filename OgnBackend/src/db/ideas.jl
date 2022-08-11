@inline function create_idea(pool, idea::NewIdea)
    query = raw"INSERT INTO ideas (label) VALUES ($1) RETURNING id;"
    result = async_execute(pool, query, (idea.label,))

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        if isempty(cols.id)
            nothing
        else
            Idea(IdeaId(cols.id |> first), idea.label)
        end
    end
end

@inline function get_idea(pool, idea_id::IdeaId)
    query = raw"SELECT * FROM ideas WHERE id = $1;"

    result = async_execute(pool, query, (idea_id,))

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        if isempty(cols.id)
            nothing
        else
            @assert length(cols.id) == 1
            Idea(IdeaId(cols.id |> first), cols.label |> first)
        end
    end
end

@inline function get_ideas(pool)
    query = raw"SELECT * FROM ideas;"

    result = async_execute(pool, query)

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        [Idea(IdeaId(id), label) for (id, label) in zip(cols.id, cols.label)]
    end
end

@inline function get_ideas(pool, page_num_size)
    page_num, page_size = page_num_size

    query = raw"SELECT * FROM ideas LIMIT $1 OFFSET $2;"

    result = async_execute(pool, query, (page_size, page_size * page_num))

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        [Idea(IdeaId(id), label) for (id, label) in zip(cols.id, cols.label)]
    end
end

@inline function get_num_ideas(pool)
    query = raw"SELECT COUNT(*) FROM ideas;"

    result = async_execute(pool, query)

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        if isempty(cols.count)
            nothing
        else
            @assert length(cols.count) == 1
            cols.count |> first
        end
    end
end

idea_exists(pool, idea_id::IdeaId) = @async begin
    result = get_idea(pool, idea_id) |> fetch
    result === nothing ? false : true
end

@inline function delete_idea(pool, idea_id::IdeaId)
    query = raw"DELETE FROM ideas WHERE id = $1;"
    result = async_execute(pool, query, (idea_id,))

    @async begin
        result = result |> fetch
        nothing
    end
end
