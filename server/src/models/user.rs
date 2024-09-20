use std::time::SystemTime;
use serde::{Deserialize, Serialize};
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

#[derive(Serialize,Deserialize)]
pub enum LoginOption {
    Username(String),
    Email(String),
}

#[derive(Serialize,Deserialize)]
pub struct UserLogin {
    pub username: LoginOption,
    pub password: String,
}

impl UserLogin {
    pub fn new(username: LoginOption, password: String) -> Self {
        UserLogin {
            username,
            password,
        }
    }

    pub fn new_with_username(username: String, password: String) -> Self {
        UserLogin {
            username: LoginOption::Username(username),
            password,
        }
    }

    pub fn new_with_email(email: String, password: String) -> Self {
        UserLogin {
            username: LoginOption::Email(email),
            password,
        }
    }

    pub fn authenticate(&self, pool: &DbPool) -> Result<User,String> {
        let user = match &self.username {
            LoginOption::Username(username) => {
                User::get_by_username(username, pool).map_err(|e| e.to_string())?
            }
            LoginOption::Email(email) => {
                User::get_by_email(email, pool).map_err(|e| e.to_string())?
            }
        };
        if app::password::verify_password(&self.password, &user.password_hash).is_ok() {
            Ok(user)
        } else {
            Err("Invalid password".to_string())
        }
    }
}

#[derive(Insertable, Deserialize)]
#[diesel (table_name = super::super::schema::users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl NewUser {
    pub fn new(username: String, email: String, password: String) -> Result<Self,Box<dyn std::error::Error>> {
        let password_hash = app::password::hash_password(&password).map_err(|e| "Couldn't hash password")?;
        Ok(NewUser {
            username,
            email,
            password_hash,
        })
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

    pub fn get_by_username(p_username: &str, pool: &DbPool) -> Result<User, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        let mut conn = pool.get().unwrap();
        let user = users.filter(username.eq(p_username)).first(&mut conn).map_err(|e| e)?;
        Ok(user)
    }

    pub fn get_by_email(p_email: &str, pool: &DbPool) -> Result<User, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        let mut conn = pool.get().unwrap();
        let user = users.filter(email.eq(p_email)).first(&mut conn).map_err(|e| e)?;
        Ok(user)
    }
}