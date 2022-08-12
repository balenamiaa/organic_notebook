module Globals

export pool
import OgnBackend: PgPool, @_dbg, Poppler_jll

@_dbg "debug" begin
    DB_HOST::String = ""
    DB_PORT::String = ""
    DB_NAME::String = ""
    DB_USER::String = ""
    DB_PASSWORD::String = ""
    DB_POOL_CAPACITY::Int = 0
    DB_DOCUMENTS_DIRPATH::String = ""
    OGN_SERVER_HOST::String = ""
    OGN_SERVER_PORT::Int = 0
end

@_dbg "release" begin
    const DB_HOST::String = ENV["DB_HOST"]
    const DB_PORT::String = ENV["DB_PORT"]
    const DB_NAME::String = ENV["DB_NAME"]
    const DB_USER::String = ENV["DB_USER"]
    const DB_PASSWORD::String = ENV["DB_PASSWORD"]
    const DB_POOL_CAPACITY::Int = parse(Int, ENV["DB_POOL_CAPACITY"])
    const DB_DOCUMENTS_DIRPATH::String = ENV["DB_DOCUMENTS_DIRPATH"]

    const OGN_SERVER_HOST::String = ENV["OGN_SERVER_HOST"]
    const OGN_SERVER_PORT::Int = parse(Int, ENV["OGN_SERVER_PORT"])::Int
end

@_dbg "debug" function init_env()
    @eval include(joinpath(@__DIR__(), "../env.jl"))
    global DB_HOST = ENV["DB_HOST"]::String
    global DB_PORT = ENV["DB_PORT"]::String
    global DB_NAME = ENV["DB_NAME"]::String
    global DB_USER = ENV["DB_USER"]::String
    global DB_PASSWORD = ENV["DB_PASSWORD"]::String
    global DB_POOL_CAPACITY = parse(Int, ENV["DB_POOL_CAPACITY"])::Int
    global OGN_SERVER_HOST = ENV["OGN_SERVER_HOST"]::String
    global OGN_SERVER_PORT = parse(Int, ENV["OGN_SERVER_PORT"])::Int
    global DB_DOCUMENTS_DIRPATH = ENV["DB_DOCUMENTS_DIRPATH"]::String
end
@_dbg "release" init_env() = nothing

mutable struct Context
    pool::PgPool
    Context() = new()
end

CONTEXT = Context()

pool() = CONTEXT.pool

function init_context()
    if isdefined(CONTEXT, :pool)
        close(CONTEXT.pool)
    end

    global CONTEXT.pool = PgPool(DB_POOL_CAPACITY)
end

end
