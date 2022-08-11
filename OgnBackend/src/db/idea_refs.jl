@inline function create_idea_ref(pool, idea_ref::NewIdeaRef)
    query = raw"""
    INSERT INTO idea_refs 
        (document_id, document_page, idea_ref, idea_ref_text) 
    VALUES 
    ($1, $2, $3, $4) RETURNING id;
    """

    document_page = if isnothing(idea_ref.doc_page.page_number)
        nothing
    else
        something(idea_ref.doc_page.page_number)
    end

    result = async_execute(
        pool,
        query,
        (
            idea_ref.doc_page.document_id,
            document_page,
            idea_ref.idea_ref,
            idea_ref.idea_ref_text,
        ),
    )

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        if isempty(cols.id)
            nothing
        else
            IdeaRef(
                IdeaRefId(cols.id |> first),
                idea_ref.doc_page,
                idea_ref.idea_ref,
                idea_ref.idea_ref_text,
            )
        end
    end
end

@inline function get_idea_ref(pool, idea_ref_id::IdeaRefId)
    query = raw"SELECT * FROM idea_refs WHERE id = $1;"

    result = async_execute(pool, query, (idea_ref_id,))

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        document_page = cols.document_page |> first
        document_page = ismissing(document_page) ? nothing : Some(document_page)

        if isempty(cols.id)
            nothing
        else
            @assert length(cols.id) == 1
            IdeaRef(
                IdeaRefId(cols.id |> first),
                DocumentPage(DocumentId(cols.document_id |> first), document_page),
                IdeaId(cols.idea_ref |> first),
                cols.idea_ref_text |> first,
            )
        end
    end
end

@inline function get_idea_refs(pool)
    query = raw"SELECT * FROM idea_refs;"

    result = async_execute(pool, query)

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        [
            IdeaRef(
                IdeaRefId(id),
                DocumentPage(
                    DocumentId(document_id),
                    ismissing(document_page) ? nothing : Some(document_page),
                ),
                IdeaId(idea_ref),
                idea_ref_text,
            ) for (id, document_id, document_page, idea_ref, idea_ref_text) in zip(
                cols.id,
                cols.document_id,
                cols.document_page,
                cols.idea_ref,
                cols.idea_ref_text,
            )
        ]
    end
end

@inline function get_idea_refs(pool, page_num_size)
    page_num, page_size = page_num_size

    query = raw"SELECT * FROM idea_refs LIMIT $1 OFFSET $2;"

    result = async_execute(pool, query, (page_size, page_size * page_num))

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        [
            IdeaRef(
                IdeaRefId(id),
                DocumentPage(
                    DocumentId(document_id),
                    ismissing(document_page) ? nothing : Some(document_page),
                ),
                IdeaId(idea_ref),
                idea_ref_text,
            ) for (id, document_id, document_page, idea_ref, idea_ref_text) in zip(
                cols.id,
                cols.document_id,
                cols.document_page,
                cols.idea_ref,
                cols.idea_ref_text,
            )
        ]
    end
end

@inline function get_num_idea_refs(pool)
    query = raw"SELECT COUNT(*) FROM idea_refs;"

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

idea_ref_exists(pool, idea_ref_id::IdeaRefId) = @async begin
    result = get_idea_ref(pool, idea_ref_id) |> fetch
    result === nothing ? false : true
end

@inline function delete_idea_ref(pool, idea_ref_id::IdeaRefId)
    query = raw"DELETE FROM idea_refs WHERE id = $1;"
    result = async_execute(pool, query, (idea_ref_id,))

    @async begin
        result = result |> fetch
        nothing
    end
end


@inline function get_idea_refs_for_idea(pool, idea_id::IdeaId)
    query = raw"SELECT * FROM idea_refs WHERE idea_ref = $1;"
    result = async_execute(pool, query, (idea_id,))

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        [
            IdeaRef(
                IdeaRefId(id),
                DocumentPage(
                    DocumentId(document_id),
                    isnothing(document_page) ? nothing : Some(document_page),
                ),
                IdeaId(idea_ref),
                idea_ref_text,
            ) for (id, document_id, document_page, idea_ref, idea_ref_text) in zip(
                cols.id,
                cols.document_id,
                cols.document_page,
                cols.idea_ref,
                cols.idea_ref_text,
            )
        ]
    end
end

@inline function get_idea_refs_for_idea(pool, idea_id::IdeaId, page_num_size)
    page_num, page_size = page_num_size

    query = raw"SELECT * FROM idea_refs WHERE idea_ref = $1 LIMIT $2 OFFSET $3;"
    result = async_execute(pool, query, (idea_id, page_size, page_size * page_num))

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        [
            IdeaRef(
                IdeaRefId(id),
                DocumentPage(
                    DocumentId(document_id),
                    isnothing(document_page) ? nothing : Some(document_page),
                ),
                IdeaId(idea_ref),
                idea_ref_text,
            ) for (id, document_id, document_page, idea_ref, idea_ref_text) in zip(
                cols.id,
                cols.document_id,
                cols.document_page,
                cols.idea_ref,
                cols.idea_ref_text,
            )
        ]
    end
end

@inline function get_num_idea_refs_for_idea(pool, idea_id::IdeaId)
    query = raw"SELECT COUNT(*) FROM idea_refs WHERE idea_ref = $1;"
    result = async_execute(pool, query, (idea_id,))

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
