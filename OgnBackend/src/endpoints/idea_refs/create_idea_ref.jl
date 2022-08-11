function create_idea_ref(req::HTTP.Request)
    new_idea_ref = JSON3.read(HTTP.payload(req, String), NewIdeaRef)

    created_idea_ref = create_idea_ref(pool(), new_idea_ref) |> fetch
    created_idea_ref === nothing &&
        return HTTP.Response(Status.INTERNALERROR, "failed to create idea ref")

    HTTP.Response(Status.OK, created_idea_ref |> JSON3.write)
end
