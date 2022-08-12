## Prerequisites

 - nightly version of [Julia](https://github.com/JuliaLang/julia/)
 - nginx on Linux(automatically acquired on Windows)
 - [PostgreSQL](https://www.postgresql.org/)

## Usage

 1. `OgnBackend> julia -t auto,auto`
 2. `julia> import Pkg; Pkg.activate(".")`
 3. `julia> Pkg.instantiate()` for the first time only
 4. `julia> include("setup_nginx.jl")` for every new Julia session or hardcode OGN_NGINX_EXECUTABLE_PATH into env.jl to the path of a _properly setup_ nginx executable
 5. `julia> include("acquire_onedrive_token.jl")` for every new Julia session or hardcode ONEDRIVE_ACCESS_TOKEN and ONEDRIVE_REFRESH_TOKEN into env_secrets.jl
 6. `julia> using OgnBackend`
 7. `julia> OgnBackend.main()`

Add step `julia> using Revise` before step 6 when developing
Add step ` julia> Pkg.test()` before step 6 to test OgnBackend (recommended for the first time)

Do not close your Julia session lest you spend an eternity recompiling — if you have a trash computer that is.