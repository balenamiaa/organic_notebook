function create_idea()
    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/ideas" |> HTTP.URI
    req.method = "POST"

    req.body = JSON3.write(p.NewIdea("magic")) |> Vector{UInt8}
    resp = p.create_idea(req)
    @test resp.status == Status.OK
    created_idea = JSON3.read(HTTP.payload(resp, String), p.Idea)
    @test created_idea.label == "magic"
    created_idea
end

function get_idea(created_idea)
    id = created_idea.id

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/ideas/$id" |> HTTP.URI
    setindex!(req.context, Dict("id" => string(id)), :params)
    req.method = "GET"

    resp = p.get_idea(req)
    @test resp.status == Status.OK
    got_idea = JSON3.read(HTTP.payload(resp, String), p.Idea)
    @test got_idea == created_idea
end

function get_all_ideas(created_idea)

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/ideas" |> HTTP.URI
    req.method = "GET"

    resp = p.get_ideas(req)
    @test resp.status == Status.OK
    got_ideas = JSON3.read(HTTP.payload(resp, String), Vector{p.Idea})
    @test last(got_ideas) == created_idea
end

function get_num_ideas()
    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/ideas//num" |> HTTP.URI
    req.method = "GET"

    resp = p.get_num_ideas(req)
    @test resp.status == Status.OK
    parse(Int, HTTP.payload(resp, String))
end

function get_paginated_ideas(created_idea, num_ideas)
    page_size = 10
    page_num = ceil(Int, num_ideas / page_size) - 1

    req = HTTP.Request()
    req.url =
        HTTP.URI("http://127.0.0.1:8080/api/ideas?page_num=$page_num&page_size=$page_size")
    req.method = "GET"

    resp = p.get_ideas(req)
    @test resp.status == Status.OK
    result = JSON3.read(HTTP.payload(resp, String), p.PaginatedResult{p.Idea})
    @test last(result.items) == created_idea
end

function delete_idea(created_idea)
    id = created_idea.id

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/ideas/$id" |> HTTP.URI
    setindex!(req.context, Dict("id" => string(id)), :params)
    req.method = "DELETE"

    resp = p.delete_idea(req)
    @test resp.status == Status.OK
end

function get_after_delete_idea(created_idea)
    id = created_idea.id

    req = HTTP.Request()
    req.url = "http://127.0.0.1:8080/api/ideas/$id" |> HTTP.URI
    setindex!(req.context, Dict("id" => string(id)), :params)
    req.method = "GET"

    resp = p.get_idea(req)
    @test resp.status != Status.OK
end


@testset "ideas endpoint" begin
    created_idea = create_idea()
    get_idea(created_idea)
    get_all_ideas(created_idea)
    num_ideas = get_num_ideas()
    get_paginated_ideas(created_idea, num_ideas)
    delete_idea(created_idea)
    get_after_delete_idea(created_idea)
end
