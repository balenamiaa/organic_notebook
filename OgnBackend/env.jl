@inline function ⫸(key, value)
    if !haskey(ENV, key)
        ENV[key] = value
    end
end

"DB_HOST" ⫸ "localhost"
"DB_PORT" ⫸ "5432"
"DB_NAME" ⫸ "organic_notebook"
"DB_USER" ⫸ "postgres"
"DB_POOL_CAPACITY" ⫸ "16"
"DB_DOCUMENTS_DIRPATH" ⫸ joinpath(@__DIR__, "documents")
"OGN_SERVER_HOST" ⫸ "127.0.0.1"
"OGN_SERVER_PORT" ⫸ "8080"
"OGN_NGINX_INSTALL_DIR" ⫸ joinpath(@__DIR__, "nginx")
"ONEDRIVE_CLIENT_ID" ⫸ "1a66864e-4ceb-4c66-acf3-16e4b19e9b6a"
"ONEDRIVE_REDIRECT_URL" ⫸ "http://localhost:5000/"

#=
env_secrets.jl is the file that contains credentials required for the backend that isn't stored in the repository. Currently, it should be of the following form:

"DB_PASSWORD" ⫸ "<DB PASSWORD>"
=#
@static if joinpath(@__DIR__, "env_secrets.jl") |> isfile
    include("./env_secrets.jl")
else
    @warn "no env_secrets.jl file loaded. create the file then execute include(\"env_secrets.jl\")"
end
