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
        task |> fetch
    end

    @eval Globals begin
        using HTTP
        global SERVER =
            HTTP.serve!($multithreading_handler, OGN_SERVER_PORT; reuseaddr=true)
    end
    tprintln("{green}Listening on port {blue}$(Globals.OGN_SERVER_PORT){/blue}{/green}")

    handle_nginx()
end

function handle_nginx()
    options = ["start nginx server", "reload nginx configuration"]

    menu = REPL.TerminalMenus.RadioMenu(options, pagesize=4)

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
    nginx_conf_dir = ENV["OGN_NGINX_INSTALL_DIR"]
    nginx_conf = joinpath(nginx_conf_dir, "conf/nginx.conf")
    cd(nginx_conf_dir) do
        if !haskey(ENV, "OGN_NGINX_EXECUTABLE_PATH")
            tprintln("{red}nginx not installed correctly. Aborting...{/red}")
            shutdown_server()
            return
        end

        nginx = ENV["OGN_NGINX_EXECUTABLE_PATH"]

        @async begin
            cd(nginx_conf_dir) do
                run(`$nginx -c $(nginx_conf)`)
            end
        end
    end
end

function reload_nginx()
    tprintln("{green}Reloading nginx configuration{/green}")
    nginx_conf_dir = ENV["OGN_NGINX_INSTALL_DIR"]
    cd(nginx_conf_dir) do
        if !haskey(ENV, "OGN_NGINX_EXECUTABLE_PATH")
            tprintln("{red}nginx not installed correctly. Aborting...{/red}")
            return
        end

        nginx = ENV["OGN_NGINX_EXECUTABLE_PATH"]

        err = IOBuffer()
        try
            run(pipeline(`$nginx -s reload`; stderr=err); wait=true)
        catch
            tprintln(
                "{red}nginx reload failed: {blue}$(err |> take! |> String){/blue}{/red}",
            )
        end
    end
end

end
