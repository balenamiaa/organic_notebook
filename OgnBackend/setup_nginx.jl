include("./env.jl")
import Term: tprintln
using Downloads, ProgressMeter, Pkg.Artifacts, Tar, CodecZlib, SHA


const WINDOWS_BINARY_NGINX_VERSION = "1.23.1"
const WINDOWS_BINARY_DOWNLOAD_URL = "https://github.com/balenamiaa/organic_notebook/releases/download/0.1.0/nginx-1.23.1-x86_64-windows.tar.gz"


function create_nginx_conf(server_host, server_port, documents_dirpath, nginx_cache_dir)
    """
worker_processes  1;
events {
    worker_connections 1024;
}
http {
    types {
        application/pdf pdf;
        application/json json;
        application/octet-stream bin exe dll;
        application/octet-stream deb;
        application/octet-stream dmg;
        application/octet-stream iso img;
        application/octet-stream msi msp msm;
    }

    default_type  application/octet-stream;
    sendfile on;
    proxy_cache_path $(nginx_cache_dir) keys_zone=cache:10m;

    upstream backend { 
        server $(server_host):$(server_port);
        keepalive 64; 
    } 

    server {
        server_name  localhost;
        listen 80;

        add_header Access-Control-Allow-Origin "*";

        location /api/ {
            
            proxy_cache cache;
            proxy_pass http://backend; 
            proxy_set_header Connection ""; 
            proxy_http_version 1.1; 
        }

        location /static {
            proxy_cache cache;
            default_type application/pdf;
            alias $documents_dirpath;
            autoindex on;
        }
    }
}
    """
end

function download_nginx_windows()
    win_nginx_tarball = let
        "{red}downloading nginx for {blue}windows{/blue} from: {green}$WINDOWS_BINARY_DOWNLOAD_URL{/green}{/red}" |>
        tprintln
        p = ProgressUnknown(; spinner = true)
        win_nginx_tarball = Downloads.download(
            WINDOWS_BINARY_DOWNLOAD_URL;
            progress = (_, _) -> next!(p),
        )
        println()
        win_nginx_tarball
    end

    nginx_extracted_dir = open(win_nginx_tarball) do io
        decompressor = GzipDecompressorStream(io)
        Tar.extract(decompressor; set_permissions = false)
    end

    joinpath(nginx_extracted_dir, "nginx-$(NGINX_VERSION)", "nginx.exe")
end

function install_nginx(nginx_origin_bin_path)
    !isfile(nginx_origin_bin_path) && error("nginx not found at: $(nginx_origin_bin_path)")

    nginx_dir = ENV["OGN_NGINX_INSTALL_DIR"]

    try
        if isdir(nginx_dir)
            tprintln(
                "{red}OGN_NGINX_INSTALL_DIR does not exist. Attempting to delete all and recreate...{/red}",
            )
            rm(nginx_dir; force = true, recursive = true)
        else
            tprintln(
                "{red}OGN_NGINX_INSTALL_DIR does not exist. Attempting to create...{/red}",
            )
        end

        mkdir(nginx_dir)
    catch e
        tprintln("{red}failed to create OGN_NGINX_INSTALL_DIR:{/red}")
        tprintln("{blue}$e{/blue}")
        return
    end
    tprintln("{green}OGN_NGINX_INSTALL_DIR created: {blue} $nginx_dir{/blue}.{/green}")

    nginx_bin_path = joinpath(nginx_dir, if Sys.iswindows()
        "nginx.exe"
    else
        "nginx"
    end)

    try
        tprintln(
            "{green}Attempting to copy nginx from {blue}$(nginx_origin_bin_path){/blue} to {blue}$(nginx_bin_path){/blue}...{/green}",
        )
        cp(nginx_origin_bin_path, nginx_bin_path; force = true, follow_symlinks = true)
    catch e
        tprintln(
            "{red}failed to copy nginx from {blue}$(nginx_origin_bin_path){/blue} to {blue}$(nginx_bin_path){/blue}{/red}:",
        )
        tprintln("{blue}$e{/blue}")
        return
    end
    tprintln("{green}nginx copied to {blue}$(nginx_bin_path){/blue}.{/green}")

    nginx_conf_dir = joinpath(nginx_dir, "conf")
    nginx_logs_dir = joinpath(nginx_dir, "logs")
    nginx_temp_dir = joinpath(nginx_dir, "temp")
    nginx_cache_dir = joinpath(nginx_dir, "cache")

    try
        mkdir(nginx_conf_dir)
        mkdir(nginx_logs_dir)
        mkdir(nginx_temp_dir)
        mkdir(nginx_cache_dir)
    catch e
        tprintln("{red}failed to create nginx directories{/red}")
        error(e)
    end

    tprintln("{green}nginx directories created:{/green}")
    "   {blue}$(nginx_conf_dir){/blue}," |> tprintln
    "   {blue}$(nginx_logs_dir){/blue}," |> tprintln
    "   {blue}$(nginx_temp_dir){/blue}," |> tprintln
    "   {blue}$(nginx_cache_dir){/blue}" |> tprintln

    open(joinpath(nginx_conf_dir, "nginx.conf"); create = true, write = true) do io
        @inline convert_slashes(x) = replace(x, "\\" => "/")
        documents_dirpath = ENV["DB_DOCUMENTS_DIRPATH"]
        server_host = ENV["OGN_SERVER_HOST"]
        server_port = ENV["OGN_SERVER_PORT"]
        write(
            io,
            create_nginx_conf(
                server_host,
                server_port,
                documents_dirpath |> convert_slashes,
                nginx_cache_dir |> convert_slashes,
            ),
        )
    end

    nginx_bin_path
end


function run()
    if isdir(ENV["OGN_NGINX_INSTALL_DIR"])
        nginx_install_dir = ENV["OGN_NGINX_INSTALL_DIR"]
        tprintln(
            "{green}OGN_NGINX_INSTALL_DIR is set to {blue}$(nginx_install_dir){/blue}. Assuming it's setup correctly thus using it. {/green}",
        )

        ext = @static if Sys.iswindows()
            ".exe"
        elseif Sys.islinux()
            ""
        else
            error("Unsupported OS")
        end

        ENV["OGN_NGINX_EXECUTABLE_PATH"] = joinpath(nginx_install_dir, "nginx$ext")
    else
        nginx_origin_bin_path = @static if Sys.iswindows()
            download_nginx_windows()
        else
            tprintln(
                "{blue}Automatic acquisition of nginx binaries is not supported for: {green}$(Sys.KERNEL){/green}{/blue}",
            )
            tprintln("{blue}Please manually enter path to nginx binary:{/blue}")
            nginx_origin_bin_path = readline()
        end

        if !isfile(nginx_origin_bin_path)
            error("nginx not found at: $(nginx_origin_bin_path)")
        end

        nginx_binpath = install_nginx(nginx_origin_bin_path)
        ENV["OGN_NGINX_EXECUTABLE_PATH"] = nginx_binpath
    end

    nothing
end

run()
