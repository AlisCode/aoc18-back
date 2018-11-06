use reqwest::Client;
use rocket::State;
use state::global_config::GlobalConfig;
use rocket::response::{Flash, Redirect};

/// Handles callback URL from Github's OAUTH Server
/// code: given by github, is the API user code that we can transform to an access_token
#[get("/github?<code>")]
pub fn cb_login_github(code: String, config: State<GlobalConfig>) -> Flash<Redirect> {

    // Gets the Github configuration
    let github_config = config.borrow_github_config();

    // Gets the code given by Github and creates a POST request to Github's OAUTH server
    // Hopefully gets a result containing an acces_token that will later be used to access
    // the Github API.
    let client = Client::new();
    let result_acces_token = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .json(&AccessTokenRequestBody::new(
            github_config.get_client_id().into(),
            github_config.get_secret().into(),
            code,
        ))
        .send();

    // Create a reply from the given Github access token
    let reply: GithubAuthReply =
        match result_acces_token {
            Ok(mut res) => {
                let access_token_response: Result<AccessTokenResponse, _> = res.json();

                match access_token_response {
                    Ok(response) => {
                        GithubAuthReply {
                            success: true,
                            message: response.access_token,
                        }
                    }
                    _ => GithubAuthReply {
                        success: false,
                        message: "Error contacting Github".into(),
                    },
                }
            }
            _ => GithubAuthReply {
                success: false,
                message: "Github's server did not respond".into(),
            }
        };

    let redirect_to: String = format!("{}", github_config.get_redirect());

    // We either failed or success to disconnect, so we Flash the client with a cookie that will
    // be parsed on the front-end part.
    match reply.success {
        true => Flash::success(
            Redirect::to(redirect_to),
            reply.message,
        ),
        false => Flash::error(
            Redirect::to(redirect_to),
            reply.message,
        )
    }
}

/// Represents a response sent to the client, either indicating that we've successfully connected
/// or not
struct GithubAuthReply {
    success: bool,
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccessTokenRequestBody {
    client_id: String,
    client_secret: String,
    code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
    token_type: String,
    scope: String,
}

impl AccessTokenRequestBody {
    pub fn new(id: String, secret: String, code: String) -> Self {
        AccessTokenRequestBody {
            client_id: id,
            client_secret: secret,
            code,
        }
    }
}