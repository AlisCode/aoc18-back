use state::database_config::DatabaseConfig;
use state::github::GithubAuth;
use std::fs::File;
use std::io::Read;
use toml::de::from_slice;

#[derive(Deserialize, Debug)]
pub struct GlobalConfig {
    github: GithubAuth,
    database: DatabaseConfig,
}

impl GlobalConfig {
    /// Loads the configuration, searching for a valid TOML file in config/config.toml
    pub fn load() -> GlobalConfig {
        let mut config_file =
            File::open("config/config.toml").expect("No config file in config/config.toml");
        let mut buf: Vec<u8> = Vec::new();
        config_file
            .read_to_end(&mut buf)
            .expect("Failed to read config file");
        from_slice(&buf).expect("Config file not formatted correctly")
    }

    /// Gets a borrow to the github part of the configuration
    pub fn borrow_github_config(&self) -> &GithubAuth {
        &self.github
    }

    /// Gets a borrow to the database part of the configuration
    pub fn borrow_database_config(&self) -> &DatabaseConfig {
        &self.database
    }
}
