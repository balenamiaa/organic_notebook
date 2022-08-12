module OgnBackend

using HTTP,
    LibPQ,
    JSON3,
    DataStructures,
    IterTools,
    CRC32c,
    StructTypes,
    Tables,
    PDFIO,
    Term,
    REPL,
    ThreadPools,
    Downloads,
    Random,
    Poppler_jll


include("../env.jl")
include("./utilities/response_codes.jl")
include("./utilities/content_types.jl")
include("./utilities/macros.jl")
include("./utilities/pg_pool.jl")
include("./Globals.jl")
using .Globals
include("./utilities/init_db.jl")
include("./models.jl")
include("./utilities/pagination.jl")
include("./utilities/pdf_text_extractor.jl")
include("./utilities/onedrive_conversion.jl")
include("./db.jl")
include("./endpoints.jl")

function init()
    Globals.init_env()
    Globals.init_context()

    if !isdir(Globals.DB_DOCUMENTS_DIRPATH)
        mkdir(Globals.DB_DOCUMENTS_DIRPATH)
    end

    nothing
end

@_dbg "debug" begin
    using Revise
    __REVISE_IS_WATCHING = false
    function start_watching()
        global __REVISE_IS_WATCHING
        __REVISE_IS_WATCHING && return

        __REVISE_IS_WATCHING = true
        @async Revise.entr([], [OgnBackend]) do
            Revise.revise()
        end
    end
end

@_dbg "release" begin
    start_watching() = nothing
end


function shutdown_server()
    !isdefined(Globals, :SERVER) && error("server is not running")
    tprintln("{green}Shutting down the server{/green}")
    close(Globals.SERVER)
    close(pool())
end


__mt_handler_result(x::Exception) = throw(x)
__mt_handler_result(x) = x

function main()
    if isdefined(Globals, :SERVER)
        shutdown_server()
    end

    init()
    start_watching()

    router = HTTP.Router()
    register_endpoints!(router)

    function multithreading_handler(req)
        task = ThreadPools.spawnbg() do
            router(req)
        end

        task |> fetch |> __mt_handler_result
    end

    @eval Globals begin
        using HTTP
        global SERVER =
            HTTP.serve!($multithreading_handler, OGN_SERVER_PORT; reuseaddr = true)
    end
    tprintln("{green}Listening on port {blue}$(Globals.OGN_SERVER_PORT){/blue}{/green}")

    handle_nginx()
end

function handle_nginx()
    options = ["start nginx server", "reload nginx configuration"]

    menu = REPL.TerminalMenus.RadioMenu(options, pagesize = 4)

    choice = REPL.TerminalMenus.request("Choose an option:", menu)

    if choice != -1
        if choice == 1
            start_nginx()
        elseif choice == 2
            reload_nginx()
        end
    else
        tprintln("{red}Menu cancelled. Aborting...{/red}")
        shutdown_server()
    end
end

function start_nginx()
    tprintln("{green}Starting nginx server{/green}")

    if !haskey(ENV, "OGN_NGINX_EXECUTABLE_PATH")
        tprintln("{red}nginx is not installed correctly. Aborting...{/red}")
        shutdown_server()
        return
    end

    nginx = ENV["OGN_NGINX_EXECUTABLE_PATH"] |> normpath
    nginx_dir = dirname(nginx)
    nginx_conf = joinpath(nginx_dir, "conf/nginx.conf") |> normpath

    @async run(setenv(`$nginx -c $(nginx_conf)`; dir = nginx_dir))
end

function reload_nginx()
    tprintln("{green}Reloading nginx configuration{/green}")

    if !haskey(ENV, "OGN_NGINX_EXECUTABLE_PATH")
        tprintln("{red}nginx not installed correctly. Aborting...{/red}")
        return
    end

    nginx = ENV["OGN_NGINX_EXECUTABLE_PATH"] |> normpath
    nginx_conf = joinpath(dirname(nginx), "conf/nginx.conf") |> normpath

    err = IOBuffer()
    try
        run(pipeline(`$nginx -c $(nginx_conf) -s reload`; stderr = err); wait = true)
    catch e
        tprintln(
            "{red}nginx reload failed: {blue}$(err |> take! |> String){/blue}\n$e{/red}",
        )
    end
end

end
