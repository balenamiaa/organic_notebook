module ConnectionPool
export Pool, getconn!, closeconn!, async_execute
import Base: close, getindex

using DataStructures

mutable struct ConnectionWrapper{Conn,Stmt}
    conn::Conn
    isclosed::Bool
    stmts::Dict{String,Stmt}
end

function getindex(connwrapper::ConnectionWrapper)
    connwrapper.isclosed && error("trying to use a stray connection")

    connwrapper.conn
end

struct Pool{Conn,Stmt}
    connections::CircularDeque{ConnectionWrapper{Conn,Stmt}}
    signal::Threads.Condition

    function Pool{Conn,Stmt}(capacity) where {Conn,Stmt}
        queue = CircularDeque{ConnectionWrapper{Conn,Stmt}}(capacity)

        for _ in Iterators.repeated((), capacity)
            push!(
                queue,
                ConnectionWrapper{Conn,Stmt}(create_connection(Conn), false, Dict()),
            )
        end

        new{Conn,Stmt}(queue, Threads.Condition())
    end
end


function create_connection(::Type{Conn}) where {Conn}
    error("create_connection is not implemented for Pool{$Conn, ...}")
end

function close_connection(conn::Conn) where {Conn}
    error("close_connection is not implemented for Pool{$Conn, ...}")
end

function getconn!(pool::Pool)
    lock(pool.signal) do
        while true
            if isempty(pool.connections)
                wait(pool.signal)
            else
                conn = popfirst!(pool.connections)
                conn.isclosed = false
                return conn
            end
        end
    end
end

function closeconn!(pool::Pool{Conn}, conn::ConnectionWrapper{Conn}) where {Conn}
    if length(pool.connections) == pool.connections.capacity
        error("unmatched closeconn! and getconn! calls")
    end

    lock(pool.signal) do
        conn.isclosed = true
        push!(pool.connections, conn)
        notify(pool.signal; all = true)
    end
end

close(pool::Pool) =
    lock(pool.signal) do
        foreach(pool.connections) do x
            x.isclosed = true
            close_connection(x.conn)
        end
    end


function execute_kernel(stmt::Stmt, args...; kwargs...) where {Stmt}
    error("execute_kernel is not implemented for Pool{?, Stmt}")
end

function prepare_kernel(conn, query)
    error("prepare_kernel is not implemented for Pool{$(conn |> typeof), ...}")
end

function async_execute(pool::Pool{Conn,Stmt}, query, args...; kwargs...) where {Conn,Stmt}
    conn = getconn!(pool)
    stmts = conn.stmts

    if !haskey(stmts, query)
        stmts[query] = prepare_kernel(conn[], query)
    end

    @async begin
        task_result = execute_kernel(stmts[query], args...; kwargs...) |> fetch
        closeconn!(pool, conn)
        task_result
    end
end

end
