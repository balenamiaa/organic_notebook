include("./env.jl")

using HTTP, JSON3, DefaultApplication
import Term: tprintln


get_auth_token(client_id, scope, redirect_url) = HTTP.get(
    "https://login.microsoftonline.com/common/oauth2/v2.0/authorize";
    query = Dict(
        "client_id" => client_id,
        "scope" => scope,
        "response_type" => "code",
        "redirect_uri" => redirect_url,
    ),
)

function get_auth_code(scopes = ["files.readwrite.all", "offline_access"])

    client_id = ENV["ONEDRIVE_CLIENT_ID"]
    redirect_url = ENV["ONEDRIVE_REDIRECT_URL"]

    tprintln("{blue}Acquiring an auth_token with scopes {green}$scopes{/green}{/blue}")
    tprintln("{blue}client-id: {green}$client_id{/green}{/blue}")

    port = parse(Int, HTTP.URIs.URI(redirect_url).port)
    scope = join(scopes, " ")

    resp = get_auth_token(client_id, scope, redirect_url)
    index_html = resp.body |> String

    auth_token = ""
    local server
    close_task = @task begin
        close(server)
    end

    server = HTTP.serve!(port) do req
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

    DefaultApplication.open(redirect_url)

    tprintln("{blue}Please visit the redirected url and enter your credentials{/blue}")
    tprintln(
        "{blue}In the case there've been no redirection, open the following link: {green}$redirect_url{/green}{/blue}",
    )

    wait(server)

    resp = let
        body = let
            form_data = (
                "client_id" => client_id,
                "redirect_uri" => redirect_url,
                "code" => auth_token,
                "grant_type" => "authorization_code",
            )
            e = HTTP.escape
            join(map(x -> "$(e(x[1]))=$(e(x[2]))", form_data), "&")
        end

        resp = HTTP.post(
            "https://login.microsoftonline.com/common/oauth2/v2.0/token",
            ("Content-Type" => "application/x-www-form-urlencoded",),
            body,
        )
        JSON3.read(resp.body |> String)
    end

    (;
        access_token = resp["access_token"]::String,
        refresh_token = resp["refresh_token"]::String,
    )
end

function set_env!(access_token, refresh_token)
    ENV["ONEDRIVE_ACCESS_TOKEN"] = access_token
    ENV["ONEDRIVE_REFRESH_TOKEN"] = refresh_token

    tprintln(
        "{green}Environment variables ONEDRIVE_ACCESS_TOKEN and ONEDRIVE_REFRESH_TOKEN set{/green}",
    )
    nothing
end

set_env!(get_auth_code()...)
