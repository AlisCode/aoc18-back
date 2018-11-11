use diesel::prelude::*;
use schema::authprovider::dsl::*;

#[derive(Queryable)]
pub struct AuthProvider<'a> {
    id: u64,
    provider_name: &'a str,
}

pub trait AuthService {
    fn create_user(username: String) -> Result<(), String> {
        Err("Unimplemented".into())
    }

    fn set_token(&mut self, token: String);
    fn get_token() -> String;
}
