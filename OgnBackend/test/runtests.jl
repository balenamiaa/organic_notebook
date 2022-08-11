using OgnBackend
using Test, JET, Aqua, LibPQ, HTTP, JSON3
import OgnBackend.Status


const p = OgnBackend
p.init()
p.reset_db!(p.pool())



include("quality_assurance.jl")
include("base_server.jl")
include("connection_pool.jl")

include("endpoints/documents.jl")
include("endpoints/ideas.jl")
include("endpoints/idea_refs.jl")
include("endpoints/extracted_texts.jl")
