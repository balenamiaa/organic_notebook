struct OnedriveResult{T}
    err_msg::Union{String,Nothing}
    content::Union{T,Nothing}
end

err(::Type{OnedriveResult}, msg::String) = OnedriveResult{Nothing}(msg, nothing)
ok(::Type{OnedriveResult}, content::T) where {T} = OnedriveResult{T}(nothing, content)

iserr(x::OnedriveResult) = x.err_msg !== nothing

function extract(x::OnedriveResult)
    @assert x.content !== nothing
    x.content
end

struct UploadSession3
    uploadUrl::String
    expirationDate::Union{Nothing,String}
end

struct DriveFile1
    id::String
    name::String
    size::Int
    file::Any
end

UploadSession = UploadSession3
DriveFile = DriveFile1

StructTypes.StructType(::Type{UploadSession3}) = StructTypes.Struct()
StructTypes.StructType(::Type{DriveFile1}) = StructTypes.Struct()

@inline function __do_request_inner(path, method, headers, body, root)
    path = string(root, path)
    access_token = ENV["ONEDRIVE_ACCESS_TOKEN"]

    wrapped_headers = Dict{String,String}()
    wrapped_headers["Authorization"] = "Bearer $(access_token)"

    for (k, v) in headers
        push!(wrapped_headers, k => v)
    end

    HTTP.request(
        method,
        path,
        wrapped_headers,
        body;
        status_exception = false,
        redirect = false,
    )
end

function refresh_token()
    refresh_token = ENV["ONEDRIVE_REFRESH_TOKEN"]

    body = let
        form_data = (
            "grant_type" => "refresh_token",
            "refresh_token" => refresh_token,
            "client_id" => ENV["ONEDRIVE_CLIENT_ID"],
            "redirect_uri" => ENV["ONEDRIVE_REDIRECT_URL"],
        )
        e = HTTP.escape
        join(map(x -> "$(e(x[1]))=$(e(x[2]))", form_data), "&")
    end

    resp = HTTP.post(
        "https://login.microsoftonline.com/common/oauth2/v2.0/token",
        ("Content-Type" => "application/x-www-form-urlencoded",),
        body,
    )
    body = HTTP.payload(resp, String)

    jobj = body |> JSON3.read

    ENV["ONEDRIVE_ACCESS_TOKEN"] = jobj["access_token"]
    ENV["ONEDRIVE_REFRESH_TOKEN"] = jobj["refresh_token"]
end

function do_request(path, method, headers, body; root = "https://graph.microsoft.com/v1.0")

    first_attempt = __do_request_inner(path, method, headers, body, root)

    if first_attempt.status == 401
        refresh_token()
        __do_request_inner(path, method, headers, body, root)
    else
        first_attempt
    end
end

JSON_CONTENT = "Content-Type" => "application/json"
EMPTY_JSON_OBJ = (;) |> JSON3.write

function create_upload_session(file_name)
    result = do_request(
        "/me/drive/root:/$(file_name):/createUploadSession",
        "POST",
        (JSON_CONTENT,),
        EMPTY_JSON_OBJ,
    )
    body = HTTP.payload(result, String)

    result.status != 200 && return err(OnedriveResult, body)

    ok(OnedriveResult, JSON3.read(body, UploadSession))
end

function __upload_file_headers(chunk, total_length, chunk_start)
    chunk_length = chunk |> length
    chunk_start_0_indexed = chunk_start - 1
    (
        "Content-Length" => string(chunk_length),
        "Content-Range" => string(
            "bytes $(chunk_start_0_indexed)-$(chunk_start_0_indexed + chunk_length - 1)/$(total_length)",
        ),
    )
end

function __upload_file_request(upload_session, chunk, total_length, chunk_start)
    headers = __upload_file_headers(chunk, total_length, chunk_start)
    do_request(upload_session.uploadUrl, "PUT", headers, chunk; root = "")
end

function __upload_file_extract_missing_chunks(resp, total_length)
    body = HTTP.payload(resp, String)
    jobj = body |> JSON3.read
    haskey(jobj, "nextExpectedRanges") || return nothing

    missing_chunks = jobj["nextExpectedRanges"]

    map(missing_chunks) do x
        segments = split(x, '-')

        if endswith(x, '-')
            # open ended
            start = parse(Int, segments[1])
            _end = total_length
            (start, _end)
        elseif startswith(x, '-')
            # open tailed
            start = 1
            _end = parse(Int, segments[2])
            (start, _end)
        else
            # closed
            start = parse(Int, segments[1])
            _end = parse(Int, segments[2])
            (start, _end)
        end
    end
end

__upload_file_offseted_position(io) = position(io) + 1

function upload_file(upload_session, io::IO, total_length)
    seekstart(io)
    BUFFER_SIZE = 1024^2 # 1MB

    local resp
    while !eof(io)
        chunk_start = __upload_file_offseted_position(io)
        chunk = read(io, BUFFER_SIZE)

        resp = __upload_file_request(upload_session, chunk, total_length, chunk_start)

        if resp.status != 202
            body = HTTP.payload(resp, String)
            if resp.status == 200 || resp.status == 201
                return ok(OnedriveResult, JSON3.read(body, DriveFile))
            else
                return err(OnedriveResult, body)
            end
        end
    end

    # handle missing chunks
    if resp.status == 202
        seekstart(io)
        missing_chunks = __upload_file_extract_missing_chunks(resp, total_length)

        for (start, _end) in missing_chunks
            seek(io, start)

            while (pos = __upload_file_offseted_position(io)) < _end
                chunk_start = pos

                _buffer_size = if chunk_start + BUFFER_SIZE > _end
                    _end - chunk_start
                else
                    BUFFER_SIZE
                end

                chunk = read(io, _buffer_size)
                resp =
                    __upload_file_request(upload_session, chunk, total_length, chunk_start)
                resp.status ∉ (202, 201, 200) &&
                    return err(OnedriveResult, HTTP.payload(resp, String))
            end
        end

        if resp.status ∉ (201, 200)
            return err(
                OnedriveResult,
                string("couldn't send missing chunks: ", HTTP.payload(resp, String)),
            )
        else
            return ok(OnedriveResult, JSON3.read(HTTP.payload(resp, String), DriveFile))
        end
    end
end


function download_file_url(file::DriveFile, format)
    format = format
    item_id = file.id

    resp = do_request("/drive/items/$(item_id)/content?format=$format", "GET", (), ())
    if resp.status == 302
        download_url = HTTP.header(resp, "Location")
        return ok(OnedriveResult, download_url)
    else
        return err(
            OnedriveResult,
            string("couldn't download file: ", HTTP.payload(resp, String)),
        )
    end
end


function delete_file(file::DriveFile)
    item_id = file.id
    resp = do_request("/me/drive/items/$(item_id)", "DELETE", (), ())

    resp.status == 204 ? ok(OnedriveResult, ()) :
    err(OnedriveResult, string("couldn't delete file: ", HTTP.payload(resp, String)))
end

function convert_document(io::IO, total_length, out, conversion)
    convert_from = conversion[1]
    convert_to = conversion[2]
    filename = randstring(12)
    upload_session = create_upload_session("$filename.$(convert_from)")
    iserr(upload_session) && return upload_session.err_msg
    upload_session = upload_session |> extract

    uploaded_file = upload_file(upload_session, io, total_length)
    iserr(uploaded_file) && return uploaded_file.err_msg
    uploaded_file = uploaded_file |> extract

    url = download_file_url(uploaded_file, convert_to)
    iserr(url) && return url.err_msg
    url = url |> extract

    Downloads.download(string(url), out)

    delete_file(uploaded_file)

    out
end

convert_file(file::IOStream, out, conversion) =
    convert_document(file, filesize(file), out, conversion)

convert_file(filename::AbstractString, out, conversion) =
    open(filename; read = true) do io
        convert_file(io, out, conversion)
    end
