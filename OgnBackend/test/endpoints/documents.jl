function upload_document()
    form = HTTP.Form(
        Dict("file1" =>
            open(joinpath(@__DIR__(), "../Anas's Grimoire of Red Magic.pdf"))),
    )

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/documents" |> HTTP.URI
    req.method = "POST"
    req.headers = ["Content-Type" => "multipart/form-data; boundary=$(form.boundary)"]
    req.body = read(form)
    resp = p.upload_document(req)
    @test resp.status == Status.OK
    created_documents = JSON3.read(HTTP.payload(resp, String), Vector{p.Document})
    @test length(created_documents) == 1

    created_documents |> first
end

function upload_docx_document()
    form = HTTP.Form(
        Dict("file1" =>
            open(joinpath(@__DIR__(), "../Anas's Grimoire of Black Magic.docx"))),
    )

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/documents" |> HTTP.URI
    req.method = "POST"
    req.headers = ["Content-Type" => "multipart/form-data; boundary=$(form.boundary)"]
    req.body = read(form)
    resp = p.upload_document(req)
    @test resp.status == Status.OK
    created_documents = JSON3.read(HTTP.payload(resp, String), Vector{p.Document})
    @test length(created_documents) == 1

    created_document = created_documents |> first

    converted_filepath = p.get_filepath_for_document(created_document.id)
    reference_filepath = joinpath(@__DIR__(), "../Anas's Grimoire of Black Magic.pdf")

    text_extractor_checksum(input) = p.extract_texts_for_document(input, OgnBackend.PopplerPdfToText) |> join |> crc32c

    converted_checksum = open(text_extractor_checksum, converted_filepath)
    reference_checksum = open(text_extractor_checksum, reference_filepath)

    @test converted_checksum == reference_checksum

    created_document
end

function get_document(created_document)
    id = created_document.id

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/documents/$id" |> HTTP.URI
    setindex!(req.context, Dict("id" => string(id)), :params)
    req.method = "GET"

    resp = p.get_document(req)
    @test resp.status == Status.OK
    got_document = JSON3.read(HTTP.payload(resp, String), p.Document)
    @test got_document == created_document
end

function get_all_documents(created_document)

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/documents" |> HTTP.URI
    req.method = "GET"

    resp = p.get_documents(req)
    @test resp.status == Status.OK
    got_documents = JSON3.read(HTTP.payload(resp, String), Vector{p.Document})
    @test last(got_documents) == created_document
end

function get_num_documents()
    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/documents//num" |> HTTP.URI
    req.method = "GET"

    resp = p.get_num_documents(req)
    @test resp.status == Status.OK
    parse(Int, HTTP.payload(resp, String))
end

function get_paginated_documents(created_document, num_documents)
    page_size = 10
    page_num = ceil(Int, num_documents / page_size) - 1

    req = HTTP.Request()
    req.url = HTTP.URI(
        "http://127.0.0.1:8080/api/documents?page_num=$page_num&page_size=$page_size",
    )
    req.method = "GET"

    resp = p.get_documents(req)
    @test resp.status == Status.OK
    result = JSON3.read(HTTP.payload(resp, String), p.PaginatedResult{p.Document})
    @test last(result.items) == created_document
end

function delete_document(created_document)
    id = created_document.id

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/documents/$id" |> HTTP.URI
    setindex!(req.context, Dict("id" => string(id)), :params)
    req.method = "DELETE"

    resp = p.delete_document(req)
    @test resp.status == Status.OK
end

function get_after_delete_document(created_document)
    id = created_document.id

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/documents/$id" |> HTTP.URI
    setindex!(req.context, Dict("id" => string(id)), :params)
    req.method = "GET"

    resp = p.get_document(req)
    @test resp.status != Status.OK
end


@testset "documents endpoint" begin
    created_document = upload_document()
    get_document(created_document)
    get_all_documents(created_document)
    num_documents = get_num_documents()
    get_paginated_documents(created_document, num_documents)
    delete_document(created_document)
    get_after_delete_document(created_document)

    created_docx_document = upload_docx_document()
    delete_document(created_docx_document)
end
