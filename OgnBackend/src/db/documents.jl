@inline function create_document(pool, document::Document)
    query =
        raw"INSERT INTO documents (id, title, filetype) VALUES ($1, $2, $3) RETURNING id;"
    result = async_execute(pool, query, (document.id, document.title, document.filetype))

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        if isempty(cols.id)
            nothing
        else
            Document(DocumentId(cols.id |> first), document.title, document.filetype)
        end
    end
end

@inline function get_document(pool, document_id::DocumentId)
    query = raw"SELECT * FROM documents WHERE id = $1;"

    result = async_execute(pool, query, (document_id,))

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        if isempty(cols.id)
            nothing
        else
            @assert length(cols.id) == 1
            Document(
                DocumentId(cols.id |> first),
                cols.title |> first,
                cols.filetype |> first,
            )
        end
    end
end

@inline function get_documents(pool)
    query = raw"SELECT * FROM documents;"

    result = async_execute(pool, query)

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        [
            Document(DocumentId(id), title, filetype) for
            (id, title, filetype) in zip(cols.id, cols.title, cols.filetype)
        ]
    end
end

@inline function get_documents(pool, page_num_size)
    page_num, page_size = page_num_size

    query = raw"SELECT * FROM documents LIMIT $1 OFFSET $2;"

    result = async_execute(pool, query, (page_size, page_size * page_num))

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        [
            Document(DocumentId(id), title, filetype) for
            (id, title, filetype) in zip(cols.id, cols.title, cols.filetype)
        ]
    end
end

@inline function get_num_documents(pool)
    query = raw"SELECT COUNT(*) FROM documents;"

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

document_exists(pool, document_id::DocumentId) = @async begin
    result = get_document(pool, document_id) |> fetch
    result === nothing ? false : true
end

@inline function delete_document(pool, document_id::DocumentId)
    query = raw"DELETE FROM documents WHERE id = $1;"
    result = async_execute(pool, query, (document_id,))

    @async begin
        result = result |> fetch
        nothing
    end
end

@inline function get_filepath_for_document(document_id::DocumentId)
    path = joinpath(Globals.DB_DOCUMENTS_DIRPATH, "$document_id.pdf")
    isfile(path) ? path : nothing
end
