#[derive(Deserialize, Debug)]
pub struct GitlabAuth {
    /// Client ID of the Github app
    client_id: String,
    /// Secret identifier of the Github app
    secret: String,
    /// Address to redirect to after a login attempt
    redirect: String,
    /// Address to redirect for the API
    redirect_api: String,
}

impl GitlabAuth {
    /// Gets the client ID from the config
    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }

    /// Gets the secret identifier of the Github app from the config
    pub fn get_secret(&self) -> &str {
        &self.secret
    }

    /// Gets the address to redirect to after a login attempt
    pub fn get_redirect(&self) -> &str {
        &self.redirect
    }

    /// Gets the API address to redirect to after a login attempt
    pub fn get_redirect_api(&self) -> &str {
        &self.redirect_api
    }
}
