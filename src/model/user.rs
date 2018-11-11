use diesel::prelude::*;
use rand::distributions::Alphanumeric;
use rand::Rng;
use schema::users::dsl::*;

#[derive(Queryable)]
/// Describes a user as present in the database
pub struct User {
    /// The unique ID of the user
    id: u64,
    /// The username of the registered user
    username: String,
    /// The internal token of our platform. Will be stored both
    /// in the back-end and the front-end
    token: String,
    /// The ID of the external authentication provider (as referenced)
    /// in the `AuthProvider` struct
    auth_provider: u64,
    /// The *external* token, that allows to communicate with the
    /// public API of an `AuthProvider`
    ext_token: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct InsertUser {
    /// The username of the `User` to be inserted
    pub username: String,
    /// The ID of the `AuthProvider` that provides the authentication proof of the user
    pub auth_provider: u64,
    /// The *internal* token of the new user to communicate with our API
    pub token: String,
    /// The *external* token of the new user to communicate with the public API of said service
    pub ext_token: String,
}

impl InsertUser {
    /// Creates a new instance of `InsertUser`, that Diesel will use to crate a given user
    pub fn new(username: String, auth_provider: u64, ext_token: String) -> Self {
        // Random generation of a token
        let token = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect::<String>();

        InsertUser {
            username,
            auth_provider,
            token,
            ext_token,
        }
    }
}
