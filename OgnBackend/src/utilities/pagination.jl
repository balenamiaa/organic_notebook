struct Pagination
    page_num::Int32
    page_size::UInt32
end

struct ExtractedPagination
    pagination::Union{Some{Pagination},Nothing}
    err_str::Union{Some{String},Nothing}
end

struct PaginatedResult{T}
    items::Vector{T}
    num_retrieved::UInt32
    num_remaining::UInt32
end


StructTypes.StructType(::Type{Pagination}) = StructTypes.Struct()
StructTypes.StructType(::Type{<:PaginatedResult}) = StructTypes.Struct()


PaginatedResult(items::Vector{T}, num_retrieved, num_remaining) where {T} =
    PaginatedResult{T}(items, num_retrieved, num_remaining)

function PaginatedResult(items::Vector{T}, num_retrieved, num_total, pagination) where {T}
    num_remaining = max(num_total - (pagination.page_num + 1) * pagination.page_size, 0)

    PaginatedResult(items, num_retrieved, num_remaining)
end

function extract_pagination(req::HTTP.Request)
    query_params = HTTP.queryparams(req.url)
    @inline get_query_params(key) = haskey(query_params, key) ? query_params[key] : nothing

    page_num = get_query_params("page_num")
    page_size = get_query_params("page_size")

    if page_num === nothing
        if page_size !== nothing
            return ExtractedPagination(
                nothing,
                Some("either specify both page_num and page_size or neither"),
            )
        else
            return ExtractedPagination(nothing, nothing)
        end
    else
        if page_size === nothing
            return ExtractedPagination(
                nothing,
                Some("either specify both page_num and page_size or neither"),
            )
        else
            page_num, page_size = (tryparse(Int32, page_num), tryparse(Int32, page_size))

            if page_num === nothing || page_size === nothing
                return ExtractedPagination(nothing, Some("invalid page_num or page_size"))
            else
                return ExtractedPagination(Some(Pagination(page_num, page_size)), nothing)
            end
        end
    end
end
