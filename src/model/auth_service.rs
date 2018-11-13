use db::DatabaseConn;
use diesel::prelude::*;
use model::user::{InsertUser, User};

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
    pub fn execute(self, db: &diesel::SqliteConnection) -> Result<User, String> {
        use schema::users;

        // Extracts data from the service
        let new_username: String = self.username.ok_or(format!("No username given"))?;
        let new_auth_service: i32 = self
            .id_auth_service
            .ok_or(format!("No auth service given"))?;
        let new_token: String = self.token.ok_or(format!("No token given"))?;

        // Checks that the username/auth_provider combination isn't already existing in database,
        // Which would mean that an user has already authenticated using this username
        let existing_users: Option<Vec<User>> = users::table
            .filter(users::username.eq(&new_username))
            .filter(users::auth_provider.eq(&new_auth_service))
            .load::<User>(db)
            .ok();

        match existing_users {
            // Returns the existing user
            Some(ref list) if list.len() >= 1 => {
                let user: &User = list.first().unwrap();
                Ok((*user).clone())
            }
            // Or create one :)
            _ => {
                let new_user = InsertUser::new(new_username, new_auth_service, new_token);
                let inserted_user = diesel::insert_into(users::table)
                    .values(&new_user)
                    .execute(db);

                match inserted_user {
                    Ok(_) => Ok(User::new_from_inserted(new_user)),
                    Err(e) => Err(format!("{}", e)),
                }
            }
        }
    }

    /// Supposed to return the ID of the `AuthProvider`
    pub fn with_auth_service_id(self, id: i32) -> Self {
        AuthService {
            id_auth_service: Some(id),
            ..self
        }
    }

    /// Sets the external token of the user being created
    pub fn with_token(self, token: String) -> Self {
        AuthService {
            token: Some(token),
            ..self
        }
    }

    /// Specifies the username of the user to create / authenticate
    pub fn with_username(self, username: String) -> Self {
        AuthService {
            username: Some(username),
            ..self
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::AuthService;
    use super::User;
    use db::TestDatabase;
    use rocket::http::Status;
    use rocket::local::Client;
    use rocket::Rocket;

    #[get("/test_user/<username>/<auth_provider>/<ext_token>")]
    fn test_user(
        username: String,
        auth_provider: i32,
        ext_token: String,
        db: TestDatabase,
    ) -> String {
        let user: Result<User, String> = AuthService::new()
            .with_username(username)
            .with_auth_service_id(auth_provider)
            .with_token(ext_token)
            .execute(&db);

        match user {
            Ok(user) => "Successfully logged in".into(),
            Err(e) => e.into(),
        }
    }

    fn test_rocket() -> Rocket {
        rocket::ignite()
            .mount("/", routes![test_user])
            .attach(TestDatabase::fairing())
    }

    #[test]
    pub fn create_user() {
        let rocket: Rocket = test_rocket();
        let client: Client = Client::new(rocket).expect("Valid Rocket instance");
        let mut response = client.get("/test_user/test_user/1/test_token").dispatch();

        // Server should repond OK, creating the user
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.body_string(),
            Some("Successfully logged in".into())
        );

        // Sending the same request should successfully connect the user
        let mut response = client.get("/test_user/test_user/1/test_token").dispatch();

        // Server should repond OK, and return the user
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.body_string(),
            Some("Successfully logged in".into())
        );

        // TODO: test - Total number of user should still be one
    }

}
