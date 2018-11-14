use db::DatabaseConn;
use model::auth_service::AuthService;
use reqwest::Client;
use rocket::response::{Flash, Redirect};
use rocket::State;
use serde_json::Value;
use state::gitlab::GitlabAuth;
use state::global_config::GlobalConfig;

/// Handles callback URL from Github's OAUTH Server
/// code: given by github, is the API user code that we can transform to an access_token
#[get("/gitlab?<code>")]
pub fn cb_login_gitlab(
    code: String,
    config: State<GlobalConfig>,
    db: DatabaseConn,
) -> Flash<Redirect> {
    // Gets the Github configuration
    let gitlab_config = config.borrow_gitlab_config();

    // Gets the code given by Gitlab and creates a POST request to Gitlab's OAUTH server
    // Hopefully gets a result containing an acces_token that will later be used to access
    // the Github API.
    let client = Client::new();
    let result_acces_token = client
        .post("https://gitlab.com/oauth/token")
        .header("Accept", "application/json")
        .json(&AccessTokenRequestBody::new(
            gitlab_config.get_client_id().into(),
            gitlab_config.get_secret().into(),
            code,
            gitlab_config.get_redirect_api().into(),
        ))
        .send();

    // Create a reply from the given Gitlab access token
    let reply: GitlabAuthReply = match result_acces_token {
        Ok(mut res) => {
            let access_token_response: Result<Value, _> = res.json();
            match access_token_response {
                Ok(response) => GitlabAuthReply {
                    success: true,
                    message: response["access_token"]
                        .to_string()
                        .trim()
                        .chars()
                        .filter(|&c| c != '\"')
                        .collect(),
                },
                _ => GitlabAuthReply {
                    success: false,
                    message: "Error contacting Gitlab".into(),
                },
            }
        }
        _ => GitlabAuthReply {
            success: false,
            message: "Gitlab's server did not respond".into(),
        },
    };

    // Loads from the config the URL that we're redirecting to
    let redirect_to: String = format!("{}", gitlab_config.get_redirect());

    // We will either failed or success to disconnect, so we Flash the client with a cookie that will
    // be parsed on the front-end part.

    // If we didn't succeed to get a correct Gitlab response, we flash the client with an error
    if (!reply.success) {
        return Flash::new(Redirect::to(redirect_to), "auth_failed", reply.message);
    }

    // Otherwise, lets try to connect ourselves :)
    // First, we need to get the user's Gitlab name
    let username_query = client
        .get("https://gitlab.com/api/v3/user")
        .header("Authorization", format!("Bearer {}", reply.message.clone()))
        .header("Accept", "application/json")
        .send();

    // If we got a correct response from Gitlab, we can get the username
    match username_query {
        Ok(mut res) => {
            // Parses the JSON from Gitlab
            let value: Value = res.json().expect("Failed to read JSON");

            // Gets the username; Trims it and removes quotes because Gitlab's username format is weird
            let username: String = value["username"]
                .to_string()
                .trim()
                .chars()
                .filter(|&c| c != '\"')
                .collect();

            // Starts the authentication service with our params
            let result_auth = AuthService::new()
                .with_username(username)
                .with_token(reply.message)
                .with_auth_service_id(2)
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
            format!("Failed to get the username from Gitlab"),
        ),
    }
}

/// Struct that is serialized and sent to Gitlab in order to OAUTH an user
#[derive(Serialize, Deserialize)]
struct AccessTokenRequestBody {
    client_id: String,
    client_secret: String,
    code: String,
    grant_type: String,
    redirect_uri: String,
}

impl AccessTokenRequestBody {
    /// Creates a new auth request body
    pub fn new(id: String, secret: String, code: String, redirect_uri: String) -> Self {
        AccessTokenRequestBody {
            client_id: id,
            client_secret: secret,
            code,
            grant_type: "authorization_code".into(),
            redirect_uri,
        }
    }
}

/// Parses the Access token response from Gitlab
/// Probably overkill. TODO (refactor): remove
#[derive(Serialize, Deserialize, Debug)]
struct AccessTokenResponse {
    pub access_token: String,
    token_type: String,
    refresh_token: String,
    expires_in: i32,
}

/// Represents a response sent to the client, either indicating that we've successfully connected
/// or not
struct GitlabAuthReply {
    success: bool,
    message: String,
}
