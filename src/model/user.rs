use db::DatabaseConn;
use diesel::prelude::*;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rocket::http::Cookie;
use rocket::http::Status;
use rocket::request::{FromRequest, Request};
use rocket::Outcome;
use schema::users;

#[derive(Queryable, Clone, Serialize, Deserialize, Debug)]
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

    pub fn find_by_token(token: String, db: &diesel::SqliteConnection) -> Result<Self, ()> {
        let queried = users::table
            .filter(users::token.eq(token))
            .load::<Self>(db)
            .ok();

        match queried {
            Some(ref list) if list.len() >= 1 => {
                let user: &User = list.first().unwrap();
                Ok((*user).clone())
            }
            _ => Err(()),
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

pub struct APIUser {
    pub id: i32,
    pub auth_provider: i32,
    pub username: String,
}

impl APIUser {
    pub fn new_from_user(user: User) -> Self {
        APIUser {
            id: user.id.unwrap(),
            auth_provider: user.auth_provider,
            username: user.username,
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for APIUser {
    type Error = &'a str;

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Self, Self::Error> {
        let db = request.guard::<DatabaseConn>();
        match db {
            Outcome::Failure(_) => {
                return Outcome::Failure((
                    Status::InternalServerError,
                    "Failed to connect to database",
                ))
            }
            Outcome::Forward(_) => {
                return Outcome::Failure((Status::InternalServerError, "Got forwarded on DB query"))
            }
            _ => (),
        };

        let api_token: String = request
            .cookies()
            .get("api_token")
            .unwrap_or(&Cookie::new("api_token", ""))
            .value()
            .into();

        let db: DatabaseConn = db.unwrap();
        match User::find_by_token(api_token, &db) {
            Ok(user) => Outcome::Success(APIUser::new_from_user(user)),
            Err(_) => Outcome::Failure((Status::NotFound, "No user found")),
        }
    }
}

#[get("/username")]
pub fn get_username(api_user: APIUser) -> String {
    api_user.username
}
