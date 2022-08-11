
function create_idea(req::HTTP.Request)
    new_idea = JSON3.read(HTTP.payload(req, String), NewIdea)

    created_idea = create_idea(pool(), new_idea) |> fetch
    created_idea === nothing &&
        return HTTP.Response(Status.INTERNALERROR, "failed to create idea")

    HTTP.Response(Status.OK, created_idea |> JSON3.write)
end
