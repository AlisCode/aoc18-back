use rocket_contrib::databases::diesel;

#[database("sqlite_db")]
pub struct DatabaseConn(diesel::SqliteConnection);
