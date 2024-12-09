use diesel::prelude::*;
use crate::domain::user::{User, UserRepository};
use crate::schema::users;

pub struct UserRepositoryImpl {
    connection: SqliteConnection,
}

impl UserRepositoryImpl {
    pub fn new(database_url: &str) -> Self {
        let connection = SqliteConnection::establish(database_url)
            .expect("Failed to connect to database");
        Self { connection }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn insert(&self, user: &User) -> Result<(), String> {
        diesel::insert_into(users::table)
            .values(user)
            .execute(&self.connection)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn update(&self, user: &User) -> Result<(), String> {
        diesel::update(users::table.filter(users::id.eq(user.id)))
            .set((
                users::username.eq(&user.username),
                users::email.eq(&user.email),
                users::password_hash.eq(&user.password_hash),
            ))
            .execute(&self.connection)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn delete(&self, user_id: i32) -> Result<(), String> {
        diesel::delete(users::table.filter(users::id.eq(user_id)))
            .execute(&self.connection)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
