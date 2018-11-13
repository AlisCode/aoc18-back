use rocket_contrib::databases::diesel;

#[database("sqlite_db")]
pub struct DatabaseConn(diesel::SqliteConnection);

#[cfg(test)]
#[database("test_db")]
pub struct TestDatabase(diesel::SqliteConnection);
