function create_idea_ref(doc, page, idea)
    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/idea_refs" |> HTTP.URI
    req.method = "POST"

    req.body =
        JSON3.write(p.NewIdeaRef(p.DocumentPage(doc.id, Some(page)), idea.id, "test")) |>
        Vector{UInt8}

    resp = p.create_idea_ref(req)
    @test resp.status == Status.OK
    created_idea_ref = JSON3.read(HTTP.payload(resp, String), p.IdeaRef)

    @test created_idea_ref.idea_ref_text == "test"
    @test created_idea_ref.doc_page.document_id == doc.id
    @test something(created_idea_ref.doc_page.page_number) == page
    @test created_idea_ref.idea_ref == idea.id

    created_idea_ref
end

function get_idea_ref(created_idea_ref)
    id = created_idea_ref.id

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/idea_refs/$id" |> HTTP.URI
    setindex!(req.context, Dict("id" => string(id)), :params)
    req.method = "GET"

    resp = p.get_idea_ref(req)
    @test resp.status == Status.OK
    result = JSON3.read(HTTP.payload(resp, String), p.IdeaRef)
    @test result == created_idea_ref
end

function get_all_idea_refs(created_idea_ref)
    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/idea_refs" |> HTTP.URI
    req.method = "GET"

    resp = p.get_idea_refs(req)
    @test resp.status == Status.OK
    result = JSON3.read(HTTP.payload(resp, String), Vector{p.IdeaRef})
    @test last(result) == created_idea_ref
end

function get_num_idea_refs()
    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/idea_refs//num" |> HTTP.URI
    req.method = "GET"

    resp = p.get_num_idea_refs(req)
    @test resp.status == Status.OK
    parse(Int, HTTP.payload(resp, String))
end

function get_paginated_idea_refs(created_idea_ref, num_idea_refs)
    page_size = 10
    page_num = ceil(Int, num_idea_refs / page_size) - 1

    req = HTTP.Request()
    req.url = HTTP.URI(
        "http://127.0.0.1:8080/api/idea_refs?page_num=$page_num&page_size=$page_size",
    )
    req.method = "GET"

    resp = p.get_idea_refs(req)
    @test resp.status == Status.OK

    result = JSON3.read(HTTP.payload(resp, String), p.PaginatedResult{p.IdeaRef})
    @test last(result.items) == created_idea_ref
end

function get_all_idea_refs_for_idea(created_idea_ref, idea)
    id = idea.id

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/idea_refs_for_idea/$id" |> HTTP.URI
    setindex!(req.context, Dict("id" => string(id)), :params)
    req.method = "GET"

    resp = p.get_idea_refs_for_idea(req)
    @test resp.status == Status.OK

    result = JSON3.read(HTTP.payload(resp, String), Vector{p.IdeaRef})
    @test last(result) == created_idea_ref
    @test fetch(p.get_idea(p.pool(), id)).id == id
end

function get_num_idea_refs_for_idea(idea)
    id = idea.id

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/idea_refs_for_idea/$id/num" |> HTTP.URI
    setindex!(req.context, Dict("id" => string(id)), :params)
    req.method = "GET"

    resp = p.get_num_idea_refs_for_idea(req)
    @test resp.status == Status.OK
    parse(Int, HTTP.payload(resp, String))
end

function get_paginated_idea_refs_for_idea(created_idea_ref, num_idea_refs_for_idea, idea)
    id = idea.id
    page_size = 10
    page_num = num_idea_refs_for_idea ÷ page_size

    req = HTTP.Request()
    req.url = HTTP.URI(
        "http://127.0.0.1:8080/api/idea_refs_for_idea/$id?page_num=$page_num&page_size=$page_size",
    )
    setindex!(req.context, Dict("id" => string(id)), :params)
    req.method = "GET"

    resp = p.get_idea_refs_for_idea(req)
    @test resp.status == Status.OK

    result = JSON3.read(HTTP.payload(resp, String), p.PaginatedResult{p.IdeaRef})
    @test last(result.items) == created_idea_ref
end

function delete_idea_ref(created_idea_ref)
    id = created_idea_ref.id

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/idea_refs/$id" |> HTTP.URI
    setindex!(req.context, Dict("id" => string(id)), :params)
    req.method = "DELETE"

    resp = p.delete_idea_ref(req)
    @test resp.status == Status.OK
end

function get_after_delete_idea_ref(created_idea_ref)
    id = created_idea_ref.id

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/idea_refs/$id" |> HTTP.URI
    setindex!(req.context, Dict("id" => string(id)), :params)
    req.method = "GET"

    resp = p.get_idea_ref(req)
    @test resp.status != Status.OK
end


@testset "idea_refs endpoint" begin
    doc = upload_document()
    page = 9
    idea = create_idea()
    created_idea_ref = create_idea_ref(doc, page, idea)
    get_idea_ref(created_idea_ref)
    get_all_idea_refs(created_idea_ref)
    num_idea_refs = get_num_idea_refs()
    get_paginated_idea_refs(created_idea_ref, num_idea_refs)

    get_all_idea_refs_for_idea(created_idea_ref, idea)
    num_idea_refs_for_idea = get_num_idea_refs_for_idea(idea)
    get_paginated_idea_refs_for_idea(created_idea_ref, num_idea_refs_for_idea, idea)

    delete_idea_ref(created_idea_ref)
    get_after_delete_idea_ref(created_idea_ref)
    delete_idea(idea)
    delete_document(doc)
end
