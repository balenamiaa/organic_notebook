ENV["DB_HOST"] = "localhost"
ENV["DB_PORT"] = "5432"
ENV["DB_NAME"] = "organic_notebook"
ENV["DB_USER"] = "postgres"
ENV["DB_POOL_CAPACITY"] = "16"
ENV["DB_DOCUMENTS_DIRPATH"] = joinpath(@__DIR__, "documents")

ENV["OGN_SERVER_HOST"] = "127.0.0.1"
ENV["OGN_SERVER_PORT"] = "8080"
ENV["OGN_NGINX_INSTALL_DIR"] = joinpath(@__DIR__, "nginx")

ENV["ONEDRIVE_CLIENT_ID"] = "1a66864e-4ceb-4c66-acf3-16e4b19e9b6a"
ENV["ONEDRIVE_REDIRECT_URL"] = "http://localhost:5000/"

#=
env_secrets.jl is the file that contains credentials required for the backend that isn't stored in the repository. Currently, it should be of the following form:

ENV["DB_PASSWORD"] = "<DB PASSWORD>"
ENV["ONEDRIVE_CLIENT_SECRET"] = "<ONE DRIVE CLIENT SECRET>"
=#
@static if joinpath(@__DIR__, "env_secrets.jl") |> isfile
    include("./env_secrets.jl")
else
    error("no env_secrets.jl file found.")
end
