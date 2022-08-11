include("ConnectionPool.jl")
using .ConnectionPool


const async_execute = ConnectionPool.async_execute

function ConnectionPool.create_connection(::Type{LibPQ.Connection})
    LibPQ.Connection(
        "host=$(Globals.DB_HOST) port=$(Globals.DB_PORT) dbname=$(Globals.DB_NAME) user=$(Globals.DB_USER) password=$(Globals.DB_PASSWORD)",
    )
end
ConnectionPool.close_connection(conn::LibPQ.Connection) = LibPQ.close(conn)

struct NoPrepared
    conn::LibPQ.Connection
    query::String
end

function ConnectionPool.execute_kernel(stmt::LibPQ.Statement, args...; kwargs...)
    LibPQ.execute(stmt, args...; kwargs...)
end

function ConnectionPool.execute_kernel(stmt::NoPrepared, args...; kwargs...)
    LibPQ.execute(stmt.conn, stmt.query, args...; kwargs...)
end

function ConnectionPool.prepare_kernel(conn::LibPQ.Connection, query)
    if count(";", query) > 1 #can't prepare multiple statement queries
        NoPrepared(conn, query)
    else
        LibPQ.prepare(conn, query)
    end
end

const PgPool = ConnectionPool.Pool{LibPQ.Connection,Union{NoPrepared,LibPQ.Statement}}

function bulk_insert!(pool::PgPool, query_table, table_values)
    @inline unwrap(x) = isnothing(x) ? "" : x
    @inline unwrap(x::Union{Nothing,<:Some}) = isnothing(x) ? "" : unwrap(something(x))
    @inline access(ncol, nrow) =
        Iterators.drop(table_values[ncol], nrow - 1) |> first |> unwrap

    row_indices = table_values |> first |> eachindex
    for other in Iterators.drop(table_values, 1)
        @assert eachindex(other) == row_indices
    end

    col_indices = table_values |> eachindex

    row_strings = [
        join(("$(access(ncol, nrow))" for ncol in col_indices), ',') * '\n' for
        nrow in row_indices
    ]

    copyin = LibPQ.CopyIn("COPY $query_table FROM STDIN (FORMAT CSV);", row_strings)

    conn = getconn!(pool)
    @async begin
        result = LibPQ.execute(conn[], copyin)
        closeconn!(pool, conn)
        length(row_indices)
    end
end
