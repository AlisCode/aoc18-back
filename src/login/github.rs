use db::DatabaseConn;
use model::auth_service::AuthService;
use reqwest::Client;
use rocket::response::{Flash, Redirect};
use rocket::State;
use serde_json::Value;
use state::global_config::GlobalConfig;

/// Handles callback URL from Github's OAUTH Server
/// code: given by github, is the API user code that we can transform to an access_token
#[get("/github?<code>")]
pub fn cb_login_github(
    code: String,
    config: State<GlobalConfig>,
    db: DatabaseConn,
) -> Flash<Redirect> {
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
    let reply: GithubAuthReply = match result_acces_token {
        Ok(mut res) => {
            let access_token_response: Result<AccessTokenResponse, _> = res.json();

            match access_token_response {
                Ok(response) => GithubAuthReply {
                    success: true,
                    message: response.access_token,
                },
                _ => GithubAuthReply {
                    success: false,
                    message: "Error contacting Github".into(),
                },
            }
        }
        _ => GithubAuthReply {
            success: false,
            message: "Github's server did not respond".into(),
        },
    };

    // Loads from the config the URL that we're redirecting to
    let redirect_to: String = format!("{}", github_config.get_redirect());

    // We will either failed or success to disconnect, so we Flash the client with a cookie that will
    // be parsed on the front-end part.

    // If we didn't succeed to get a correct Github response, we flash the client with an error
    if (!reply.success) {
        return Flash::new(Redirect::to(redirect_to), "auth_failed", reply.message);
    }

    // Otherwise, lets try to connect ourselves :)
    // First, we need to get the user's Github name
    let username_query = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("token {}", reply.message.clone()))
        .header("Accept", "application/json")
        .send();

    // If we got a correct response from Github, we can get the username
    match username_query {
        Ok(mut res) => {
            // Parses the JSON from Github
            let value: Value = res.json().expect("Failed to read JSON");

            // Gets the username; Trims it and removes quotes because Github's username format is weird
            let username: String = format!("{}", value["login"])
                .trim()
                .chars()
                .filter(|&c| c != '\"')
                .collect();

            // Starts the authentication service with our params
            let result_auth = AuthService::new()
                .with_username(username)
                .with_token(reply.message)
                .with_auth_service_id(1)
                .execute(&db);

            // Following the service's response, we communicate the custom token back to the user, using a Flash
            match result_auth {
                Ok(user) => Flash::new(Redirect::to(redirect_to), "auth_success", user.token),
                Err(e) => Flash::new(Redirect::to(redirect_to), "auth_failed", e),
            }
        }
        _ => Flash::new(
            Redirect::to(redirect_to),
            "auth_failed",
            format!("Failed to get the username from Github"),
        ),
    }
}

/// Represents a response sent to the client, either indicating that we've successfully connected
/// or not
struct GithubAuthReply {
    success: bool,
    message: String,
}

/// Struct that is serialized and sent to Github in order to OAUTH an user
#[derive(Serialize, Deserialize)]
pub struct AccessTokenRequestBody {
    client_id: String,
    client_secret: String,
    code: String,
}

/// Parses the Access token response from Github
/// Probably overkill. TODO (refactor): remove
#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
    token_type: String,
    scope: String,
}

impl AccessTokenRequestBody {
    /// Creates a new auth request body
    pub fn new(id: String, secret: String, code: String) -> Self {
        AccessTokenRequestBody {
            client_id: id,
            client_secret: secret,
            code,
        }
    }
}
