struct PopplerPdfToText end
struct JuliaPdfToText end

function extract_texts_for_document(filepath::AbstractString, ::Type{JuliaPdfToText})
    doc = pdDocOpen(filepath)

    num_page = pdDocGetPageCount(doc)

    page_texts = Vector{String}(undef, num_page)
    @inbounds for i = 1:num_page
        page = pdDocGetPage(doc, i)
        page_texts[i] = String(take!(pdPageExtractText(IOBuffer(), page)))
    end

    pdDocClose(doc)

    page_texts
end

function extract_texts_for_document(document_id::DocumentId, ::Type{JuliaPdfToText})
    filepath = get_filepath_for_document(document_id)
    filepath === nothing && return nothing

    extract_texts_for_document(filepath, JuliaPdfToText)
end

function extract_texts_for_document(input::IO, ::Type{PopplerPdfToText})
    output = IOBuffer()

    try
        run(
            pipeline(
                `$(Poppler_jll.pdftotext()) - -`; stdin=input, stdout=output
            )
        )
    catch e
        @show e
        return nothing
    end

    output_bytes = take!(output)
    page_texts = String[]

    current_position = 1
    for new_page_carriage in findall(==('\f' |> UInt8), output_bytes)
        push!(page_texts,
            String(output_bytes[current_position:new_page_carriage])
        )

        current_position = new_page_carriage + 1
    end

    page_texts
end

function extract_texts_for_document(document_id::DocumentId, ::Type{PopplerPdfToText})
    filepath = get_filepath_for_document(document_id)
    filepath === nothing && return nothing

    open(filepath; read=true) do input
        extract_texts_for_document(input, PopplerPdfToText)
    end
end