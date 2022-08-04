module OnedriveTokenCreator
using HTTP, JSON3, DefaultApplication, DotEnv

const DOTENV_PATH = joinpath(@__DIR__, "..", "..", ".env")

function get_auth_code(scopes=["files.readwrite.all", "offline_access"])
  DotEnv.config(path=DOTENV_PATH)
  client_id = ENV["ONEDRIVE_CLIENT_ID"]::String
  redirect_uri = ENV["ONEDRIVE_REDIRECT_URI"]::String

  port = parse(Int, HTTP.URIs.URI(redirect_uri).port)
  scope = join(scopes, " ") |> HTTP.URIs.escapeuri

  resp = HTTP.get("https://login.microsoftonline.com/common/oauth2/v2.0/authorize?client_id=$(client_id)&scope=$(scope)&response_type=code&redirect_uri=$redirect_uri")
  index_html = resp.body |> String

  auth_token = ""
  local server
  close_task = @task begin
    close(server)
  end

  server = HTTP.serve!("127.0.0.1", port) do req
    target = req.target

    if target == "/"
      return index_html
    elseif startswith(target, "/?code=")
      auth_token = target[length("/?code=")+1:end]
      schedule(close_task)
      return "Authorization code received. Closing server."
    end

    return ""
  end

  DefaultApplication.open(redirect_uri)
  wait(server)

  body = let
    form_data = (
      "client_id" => client_id,
      "redirect_uri" => redirect_uri,
      "code" => auth_token,
      "grant_type" => "authorization_code",
      #"client_secret" => ENV["ONEDRIVE_CLIENT_SECRET"]::String,
    )
    e = HTTP.URIs.escapeuri
    join(map(x -> "$(e(x[1]))=$(e(x[2]))", form_data), "&")
  end

  resp = let
    resp = HTTP.post("https://login.microsoftonline.com/common/oauth2/v2.0/token", ("Content-Type" => "application/x-www-form-urlencoded",), body)
    JSON3.read(resp.body |> String)
  end

  (;
    access_token=resp["access_token"]::String,
    refresh_token=resp["refresh_token"]::String
  )
end

function set_dotenv!(access_token, refresh_token)
  dotenv_file_content = read(DOTENV_PATH, String) |> collect

  @inline function get_startend(token, chars)
    content = String(chars) # does not copy

    start = let
      start_range = findfirst(token, content)
      @assert start_range !== nothing
      start_range.start
    end

    _end = let
      _end_range = findnext(r"\n|\r\n", content, start)
      if _end_range === nothing
        length(content) # assume it's the last line, and there are no new lines
      else
        _end_range.start
      end
    end
    @assert _end !== nothing

    start:_end
  end

  access_token_range = get_startend("ONEDRIVE_ACCESS_TOKEN", dotenv_file_content)
  deleteat!(dotenv_file_content, access_token_range)

  refresh_token_range = get_startend("ONEDRIVE_REFRESH_TOKEN", dotenv_file_content)
  deleteat!(dotenv_file_content, refresh_token_range)

  open(DOTENV_PATH; create=true, truncate=true, write=true) do io
    out_str = String(dotenv_file_content)
    write(io, out_str)

    if out_str[end] != '\n'
      write(io, "\n")
    end

    write(io, "ONEDRIVE_ACCESS_TOKEN=\"$(access_token)\"\n")
    write(io, "ONEDRIVE_REFRESH_TOKEN=\"$(refresh_token)\"\n")
  end

  nothing
end


end
