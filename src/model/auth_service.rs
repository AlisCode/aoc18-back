use db::DatabaseConn;
use model::user::{User, InsertUser};
use diesel::prelude::*;

#[derive(Queryable)]
pub struct AuthProvider<'a> {
    pub id: Option<i32>,
    pub provider_name: &'a str,
}

pub struct AuthService {
    username: Option<String>,
    id_auth_service: Option<i32>,
    token: Option<String>,
}

impl AuthService {
    /// Creates a new authentication service
    pub fn new() -> Self {
        AuthService {
            username: None,
            id_auth_service: None,
            token: None,
        }
    }

    /// Consumes the service to get the user information, or create it in the database
    pub fn get_user(self, db: &diesel::SqliteConnection) -> Result<User, &str> {
        use schema::users;

        // Extracts data from the service
        let new_username: String = self.username.ok_or("No username given")?;
        let new_auth_service: i32 = self.id_auth_service.ok_or("No auth service given")?;
        let new_token: String = self.token.ok_or("No token given")?;

        // Checks that the username/auth_provider combination isn't already existing in database,
        // Which would mean that an user has already authenticated using this username
        let existing_users: Result<Vec<User>, _> = users::table
            .filter(users::username.eq(&new_username))
            .filter(users::auth_provider.eq(&new_auth_service))
            .load::<User>(db);

        match existing_users {
            // Returns the existing user
            Some(list) if list.len() >= 1 => {
                let user = list.first().unwrap();
                Err("failed")
            }
            // Or create one :)
            _ => {
                let new_user = InsertUser::new(new_username, new_auth_service, new_token);
                let inserted_user = diesel::insert_into(users::table)
                    .values(&new_user)
                    .execute(db)
                    .ok();

                match inserted_user {
                    Some(_) => Ok(User::new_from_inserted(new_user)),
                    _ => Err("Failed to create a new user")
                }
            }
        }
    }

    /// Supposed to return the ID of the `AuthProvider`
    pub fn with_auth_service_id(mut self, id: i32) -> Self {
        AuthService {
            id_auth_service: Some(id),
            ..self
        }
    }

    /// Sets the external token of the user being created
    pub fn set_token(mut self, token: String) -> Self {
        AuthService {
            token: Some(token),
            ..self
        }
    }

    /// Specifies the username of the user to create / authenticate
    pub fn with_username(mut self, username: String) -> Self {
        AuthService {
            username: Some(username),
            ..self
        }
    }
}
