@testset "Quality assurance tests" begin
    JET.report_file(joinpath(@__DIR__, "../src/OgnBackend.jl"))
    Aqua.test_all(
        OgnBackend;
        ambiguities = false,
        unbound_args = false,
        stale_deps = (; ignore = [:Aqua, :JET, :ProgressMeter, :DefaultApplication]),
    )
end
