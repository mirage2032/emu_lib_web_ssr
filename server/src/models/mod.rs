use std::time::SystemTime;
use serde::Deserialize;
use diesel::prelude::*;
use crate::api::DbPool;
use diesel::Queryable;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable, Deserialize)]
#[diesel (table_name = super::schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}

impl<'a> NewUser<'a> {
    pub fn new(username: &'a str, email: &'a str, password_hash: &'a str) -> Self {
        NewUser {
            username,
            email,
            password_hash,
        }
    }
}

impl User {
    pub fn add_user(new_user: NewUser, pool: &DbPool) -> Result<User, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        let mut conn = pool.get().unwrap();
        let user = diesel::insert_into(users).values(&new_user).get_result(&mut conn).map_err(|e| e)?;
        Ok(user)
    }
    pub fn get_by_id(p_id: i32, pool: &DbPool ) -> Result<User, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        let mut conn = pool.get().unwrap();
        let user = users.filter(id.eq(p_id)).first(&mut conn).map_err(|e| e)?;
        Ok(user)
    }
}