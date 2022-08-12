const MODE = :debug

macro _dbg(expr1, expr2)
    if MODE == :debug && expr1 == "debug"
        esc(expr2)
    elseif MODE == :release && expr1 == "release"
        esc(expr2)
    end
end

macro impl_id_wrapper(id_tys...)
    impls = Expr[]
    for id_ty in id_tys
        impl = quote
            StructTypes.StructType(::Type{$id_ty}) = StructTypes.NumberType()
            StructTypes.numbertype(::Type{$id_ty}) = UInt32

            function Base.show(io::IO, x::$id_ty)
                write(io, string(x.id))
            end

            function Base.convert(::Type{UInt32}, x::$id_ty)
                x.id
            end
            Base.UInt32(x::$id_ty) = x.id

        end
        push!(impls, impl)
    end

    esc(quote
        $(impls...)
    end)
end

macro impl_structtype(tys...)
    impls = Expr[]
    for ty in tys
        impl = quote
            StructTypes.StructType(::Type{$ty}) = StructTypes.Struct()
        end
        push!(impls, impl)
    end

    esc(quote
        $(impls...)
    end)
end

"""
For use only in a handler context that returns a response.
"""
macro extract_id(req, IdType)
    quote
        let
            id = HTTP.getparams($(esc(req)))["id"]
            id === nothing && return HTTP.Response(Status.BADREQUEST, "missing id")

            id = tryparse(UInt32, id)
            id === nothing && return HTTP.Response(Status.BADREQUEST, "invalid id")
            $(IdType)(id)
        end
    end
end
