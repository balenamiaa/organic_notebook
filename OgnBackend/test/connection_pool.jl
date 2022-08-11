@testset "ConnectionPool" begin
    Threads.@threads for _ = 1:5000
        p.async_execute(p.pool(), "SELECT 1")
    end
end
