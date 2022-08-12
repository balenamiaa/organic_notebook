@inline function create_extracted_text(pool, extracted_text::NewExtractedText)
    query =
        raw"INSERT INTO extracted_texts (content, document_id, document_page) VALUES ($1, $2, $3) RETURNING id;"
    result = async_execute(
        pool,
        query,
        (
            extracted_text.content,
            extracted_text.doc_page.document_id,
            extracted_text.doc_page.page_number,
        ),
    )

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        if isempty(cols.id)
            nothing
        else
            ExtractedText(
                ExtractedTextId(cols.id |> first),
                extracted_text.content,
                extracted_text.doc_page,
            )
        end
    end
end

@inline function create_extracted_text_bulk(pool, extracted_texts::Vector{NewExtractedText})
    result = bulk_insert!(
        pool,
        "extracted_texts(content, document_id, document_page)",
        (
            ("\"$(x.content)\"" for x in extracted_texts),
            (x.doc_page.document_id for x in extracted_texts),
            (x.doc_page.page_number for x in extracted_texts),
        ),
    )

    @async begin
        nrows_inserted = result |> fetch
        result =
            async_execute(
                pool,
                raw"SELECT * FROM ( SELECT * FROM extracted_texts ORDER BY id DESC LIMIT $1) AS _ ORDER BY id ASC;",
                (nrows_inserted,),
            ) |> fetch

        cols = Tables.columns(result)

        if isempty(cols.id)
            nothing
        else
            [
                ExtractedText(
                    ExtractedTextId(id),
                    content,
                    DocumentPage(
                        DocumentId(document_id),
                        ismissing(document_page) ? nothing : Some(document_page),
                    ),
                ) for (id, content, document_id, document_page) in
                zip(cols.id, cols.content, cols.document_id, cols.document_page)
            ]
        end
    end
end

@inline function get_extracted_text(pool, extracted_text_id::ExtractedTextId)
    query = raw"SELECT * FROM extracted_texts WHERE id = $1;"

    result = async_execute(pool, query, (extracted_text_id,))

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        document_page = cols.document_page |> first
        document_page = ismissing(document_page) ? nothing : Some(document_page)

        if isempty(cols.id)
            nothing
        else
            @assert length(cols.id) == 1
            ExtractedText(
                ExtractedTextId(cols.id |> first),
                cols.content |> first,
                DocumentPage(DocumentId(cols.document_id |> first), document_page),
            )
        end
    end
end

@inline function get_extracted_texts(pool)
    query = raw"SELECT * FROM extracted_texts ORDER BY document_page ASC;"

    result = async_execute(pool, query)

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        [
            ExtractedText(
                ExtractedTextId(id),
                content,
                DocumentPage(
                    DocumentId(document_id),
                    ismissing(document_page) ? nothing : Some(document_page),
                ),
            ) for (id, content, document_id, document_page) in
            zip(cols.id, cols.content, cols.document_id, cols.document_page)
        ]
    end
end

@inline function get_extracted_texts(pool, page_num_size)
    page_num, page_size = page_num_size

    query =
        raw"SELECT * FROM extracted_texts ORDER BY document_page ASC LIMIT $1 OFFSET $2;"

    result = async_execute(pool, query, (page_size, page_size * page_num))

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        [
            ExtractedText(
                ExtractedTextId(id),
                content,
                DocumentPage(
                    DocumentId(document_id),
                    ismissing(document_page) ? nothing : Some(document_page),
                ),
            ) for (id, content, document_id, document_page) in
            zip(cols.id, cols.content, cols.document_id, cols.document_page)
        ]
    end
end

@inline function get_extracted_texts_for_document(pool, document_id::DocumentId)
    query =
        raw"SELECT * FROM extracted_texts WHERE document_id = $1 ORDER BY document_page ASC;"

    result = async_execute(pool, query, (document_id,))

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        [
            ExtractedText(
                ExtractedTextId(id),
                content,
                DocumentPage(
                    DocumentId(document_id),
                    ismissing(document_page) ? nothing : Some(document_page),
                ),
            ) for (id, content, document_id, document_page) in
            zip(cols.id, cols.content, cols.document_id, cols.document_page)
        ]
    end
end

@inline function get_extracted_texts_for_document_bulk(
    pool,
    document_ids::Vector{DocumentId},
)
    query =
        raw"SELECT * FROM extracted_texts WHERE document_id = ANY($1) ORDER BY document_page ASC;"

    result = async_execute(pool, query, (document_ids,))

    @async begin
        result = result |> fetch
        cols = Tables.columns(result)

        [
            ExtractedText(
                ExtractedTextId(id),
                content,
                DocumentPage(
                    DocumentId(document_id),
                    ismissing(document_page) ? nothing : Some(document_page),
                ),
            ) for (id, content, document_id, document_page) in
            zip(cols.id, cols.content, cols.document_id, cols.document_page)
        ]
    end
end

@inline function get_num_extracted_texts(pool)
    query = raw"SELECT COUNT(*) FROM extracted_texts;"

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

extracted_text_exists(pool, extracted_text_id::ExtractedTextId) = @async begin
    result = get_extracted_text(pool, extracted_text_id) |> fetch
    result === nothing ? false : true
end

@inline function delete_extracted_text(pool, extracted_text_id::ExtractedTextId)
    query = raw"DELETE FROM extracted_texts WHERE id = $1;"
    result = async_execute(pool, query, (extracted_text_id,))

    @async begin
        result = result |> fetch
        nothing
    end
end

@inline function delete_extracted_texts_for_document(pool, document_id::DocumentId)
    query = raw"DELETE FROM extracted_texts WHERE document_id = $1;"
    result = async_execute(pool, query, (document_id,))

    @async begin
        result = result |> fetch
        nothing
    end
end
