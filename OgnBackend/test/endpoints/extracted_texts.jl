function extract_texts_for_document(document, lastpage_content)
    id = document.id

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/extracted_texts/document/$id" |> HTTP.URI
    setindex!(req.context, Dict("id" => string(id)), :params)
    req.method = "POST"

    resp = p.extract_texts_for_document(req)
    @test resp.status == Status.OK

    result = JSON3.read(HTTP.payload(resp, String), Vector{p.ExtractedText})
    @test last(result).content == lastpage_content

    result
end


function get_extracted_texts_for_document(document, extracted_texts_for_document)
    id = document.id

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/extracted_texts/document/$id" |> HTTP.URI
    setindex!(req.context, Dict("id" => string(id)), :params)
    req.method = "GET"

    resp = p.get_extracted_texts_for_document(req)
    @test resp.status == Status.OK

    result = JSON3.read(HTTP.payload(resp, String), Vector{p.ExtractedText})
    @test result == extracted_texts_for_document
end

function get_extracted_texts_for_document_bulk(document, extracted_texts_for_document)
    id = document.id

    res_only_actual_document = let
        req = HTTP.Request()
        req.url = "http://127.0.0.1:8080/api/extracted_texts/document" |> HTTP.URI
        req.body = Vector{UInt8}(string(id))
        req.method = "GET"

        resp = p.get_extracted_texts_for_document_bulk(req)
        @test resp.status == Status.OK

        result = JSON3.read(HTTP.payload(resp, String), Vector{p.ExtractedText})
        @test result == extracted_texts_for_document
        result
    end

    let
        req = HTTP.Request()
        req.url = "http://127.0.0.1:8080/api/extracted_texts/document" |> HTTP.URI
        req.body = Vector{UInt8}(string(id, ", 0"))
        # actual document id + invalid id (0) to test bulk. there should be no extra documents for document id 0
        # which will never exist in the database
        req.method = "GET"

        resp = p.get_extracted_texts_for_document_bulk(req)
        @test resp.status == Status.OK

        result = JSON3.read(HTTP.payload(resp, String), Vector{p.ExtractedText})
        @test result == res_only_actual_document
    end
end

function get_all_extracted_texts(extracted_texts_for_document)

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/extracted_texts" |> HTTP.URI
    req.method = "GET"

    resp = p.get_extracted_texts(req)
    @test resp.status == Status.OK
    result = JSON3.read(HTTP.payload(resp, String), Vector{p.ExtractedText})
    @test all(x ∈ result for x in extracted_texts_for_document)
end

function get_num_extracted_texts()
    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/extracted_texts//num" |> HTTP.URI
    req.method = "GET"

    resp = p.get_num_extracted_texts(req)
    @test resp.status == Status.OK
    parse(Int, HTTP.payload(resp, String))
end

function get_paginated_extracted_texts(extracted_texts_for_document, num_extracted_texts)
    page_size = length(extracted_texts_for_document)
    page_num = ceil(Int, num_extracted_texts / page_size) - 1

    req = HTTP.Request()
    req.url = HTTP.URI(
        "http://127.0.0.1:8080/api/extracted_texts?page_num=$page_num&page_size=$page_size",
    )
    req.method = "GET"

    resp = p.get_extracted_texts(req)
    @test resp.status == Status.OK
    result = JSON3.read(HTTP.payload(resp, String), p.PaginatedResult{p.ExtractedText})
    @test all(x ∈ result.items for x in extracted_texts_for_document)
end

function delete_extracted_texts_for_document(document)
    id = document.id

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/extracted_texts/document/$id" |> HTTP.URI
    setindex!(req.context, Dict("id" => string(id)), :params)
    req.method = "DELETE"

    resp = p.delete_extracted_texts_for_document(req)
    @test resp.status == Status.OK
end




@testset "extracted texts" begin
    document = upload_document()
    lastpage_content = p.extract_texts_for_document(document.id)[end]
    @test lastpage_content |> isempty |> !
    extracted_texts_for_document = extract_texts_for_document(document, lastpage_content)
    get_extracted_texts_for_document(document, extracted_texts_for_document)
    get_extracted_texts_for_document_bulk(document, extracted_texts_for_document)
    get_all_extracted_texts(extracted_texts_for_document)
    num_extracted_texts = get_num_extracted_texts()
    get_paginated_extracted_texts(extracted_texts_for_document, num_extracted_texts)
    delete_extracted_texts_for_document(document)
    delete_document(document)
end
