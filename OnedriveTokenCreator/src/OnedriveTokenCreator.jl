module OnedriveTokenCreator
using HTTP, JSON3, DefaultApplication, DotEnv

const DOIENV_PATH = joinpath(@__DIR__, "..", "..", ".env")

function get_auth_code(scopes=["files.readwrite.all", "offline_access"])
  client_id = ENV["ONEDRIVE_CLIENT_ID"]::String
  redirect_uri = ENV["ONEDRIVE_REDIRECT_URI"]::String
  port = parse(Int, HTTP.URIs.URI(redirect_uri).port)
  scope = join(scopes, " ") |> HTTP.URIs.escapeuri

  url = "https://login.microsoftonline.com/common/oauth2/v2.0/authorize?client_id=$(client_id)&scope=$(scope)&response_type=code&redirect_uri=$redirect_uri"
  resp = HTTP.get(url)
  index_html = resp.body |> String
  auth_token = ""
  local close_task

  server = HTTP.listen!("127.0.0.1", port; stream=true) do stream
    target = stream.message.target

    if target == "/"
      write(stream, index_html)
    elseif target == "/favicon.ico"
      write(stream, "")
    elseif startswith(target, "/?code=")
      auth_token = target[length("/?code=")+1:end]
      write(stream, "")
      closewrite(stream)
      yieldto(close_task)
    else
      write(stream, "")
    end
  end

  close_task = @task begin
    close(server)
  end

  DefaultApplication.open(redirect_uri)
  wait(server)

  form_data = (
    "client_id" => client_id,
    "redirect_uri" => redirect_uri,
    "code" => auth_token,
    "grant_type" => "authorization_code",
    #"client_secret" => ENV["ONEDRIVE_CLIENT_SECRET"]::String,
  )
  e = HTTP.URIs.escapeuri
  body = join(map(x -> "$(e(x[1]))=$(e(x[2]))", form_data), "&")
  @show body

  resp = HTTP.post("https://login.microsoftonline.com/common/oauth2/v2.0/token", ("Content-Type" => "application/x-www-form-urlencoded",), body)
  resp_json = JSON3.read(resp.body |> String)

  (;
    access_token=resp_json["access_token"]::String,
    refresh_token=resp_json["refresh_token"]::String
  )
end


end
