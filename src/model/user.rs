use schema::users;
use rand::distributions::Alphanumeric;
use rand::Rng;

#[derive(Queryable, Clone)]
/// Describes a user as present in the database
pub struct User {
    /// The unique ID of the user
    pub id: Option<i32>,
    /// The username of the registered user
    pub username: String,
    /// The internal token of our platform. Will be stored both
    /// in the back-end and the front-end
    pub token: String,
    /// The ID of the external authentication provider (as referenced)
    /// in the `AuthProvider` struct
    pub auth_provider: i32,
    /// The *external* token, that allows to communicate with the
    /// public API of an `AuthProvider`
    pub ext_token: String,
}

impl User {
    /// Creates the representation of a user from the InsertedUser
    pub fn new_from_inserted(insert_user: InsertUser) -> Self {
        User {
            id: None,
            username: insert_user.username,
            token: insert_user.token,
            auth_provider: insert_user.auth_provider,
            ext_token: insert_user.ext_token,
        }
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct InsertUser {
    /// The username of the `User` to be inserted
    pub username: String,
    /// The ID of the `AuthProvider` that provides the authentication proof of the user
    pub auth_provider: i32,
    /// The *internal* token of the new user to communicate with our API
    pub token: String,
    /// The *external* token of the new user to communicate with the public API of said service
    pub ext_token: String,
}

impl InsertUser {
    /// Creates a new instance of `InsertUser`, that Diesel will use to crate a given user
    pub fn new(username: String, auth_provider: i32, ext_token: String) -> Self {
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
