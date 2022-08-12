function upload_document(req::HTTP.Request)
    parts = HTTP.parse_multipart_form(req)
    parts === nothing && return HTTP.Response(Status.BADREQUEST)
    length(parts) == 0 && return HTTP.Response(Status.BADREQUEST)

    # aggregate parts into those of same filename
    parts_aggregated = let
        result = Vector{HTTP.Multipart}[]
        prev_filename = ""
        for part in parts
            current_filename = part.filename
            !isequal(prev_filename, current_filename) && push!(result, []) # set tail to new array
            push!(result[end], part) # append part to tail array
        end

        result
    end

    @assert let # check that all aggregated parts have the same filename
        all_equal = all(parts_aggregated) do parts
            all_parts_equal = all(IterTools.partition(parts, 2, 1)) do a, b
                isequal(a.filename, b.filename)
            end
            all_parts_equal
        end

        all_equal
    end

    results = Dict{Int,Document}()
    #TODO: make threadsafe
    @sync Threads.@threads for parts in parts_aggregated
        title, ext = let
            filename = first(parts).filename
            splits = split(filename, ".")
            join(splits[1:end-1]), splits[end]
        end

        bytes_collected = let
            bytes_collected = UInt8[]
            for part in parts
                append!(bytes_collected, read(part))
            end
            bytes_collected
        end

        pdf_bytes = if ext ∈ (
            "docx", "doc",
            "pptx", "ppt"
        )
            io = IOBuffer(bytes_collected)
            out = IOBuffer()

            convert_document(io, length(bytes_collected), out, ext => "pdf")

            out |> take!
        else
            bytes_collected
        end

        id = CRC32c.crc32c(bytes_collected)

        fetch(document_exists(pool(), DocumentId(id))) && continue

        open(
            joinpath(Globals.DB_DOCUMENTS_DIRPATH, "$(id).pdf");
            create=true,
            write=true
        ) do io
            write(io, pdf_bytes)
        end

        doc = Document(DocumentId(id), title, ext)

        created_doc = fetch(create_document(pool(), doc))
        @assert created_doc == doc

        results[Threads.threadid()] = created_doc
    end

    documents_created = values(results) |> collect

    if isempty(documents_created)
        return HTTP.Response(
            Status.BADREQUEST,
            "no documents created. possible reasons: no files uploaded, or all files already exist",
        )
    end

    HTTP.Response(Status.OK, documents_created |> JSON3.write)
end
