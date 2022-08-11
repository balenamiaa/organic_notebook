
__read_sql_file(filename) = read(joinpath(@__DIR__(), "../", "sqls", filename), String)

function init_db!(pool)
    @async begin
        async_execute(pool, __read_sql_file("create_documents.psql")) |> wait
        async_execute(pool, __read_sql_file("create_ideas.psql")) |> wait
        async_execute(pool, __read_sql_file("create_extracted_texts.psql")) |> wait
        @info "database initialized"
    end
end

function drop_db!(pool)
    @async begin
        async_execute(pool, __read_sql_file("drop_extracted_texts.psql")) |> wait
        async_execute(pool, __read_sql_file("drop_ideas.psql")) |> wait
        async_execute(pool, __read_sql_file("drop_documents.psql")) |> wait
        @info "database dropped"
    end
end

function reset_db!(pool)
    drop_db!(pool) |> wait
    init_db!(pool) |> wait

    @info "database reset"
end
