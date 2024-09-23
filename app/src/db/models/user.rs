#[cfg(not(target_arch = "wasm32"))]
use diesel::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use diesel::{Queryable,Insertable};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::{Duration, SystemTime};
#[cfg(not(target_arch = "wasm32"))]
use crate::db::DbPool;
use crate::db::password;
use crate::db::models::session::*;
#[cfg(not(target_arch = "wasm32"))]
use crate::db::models::schema::users::dsl;

#[cfg_attr(not(target_arch = "wasm32"), derive(Queryable))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LoginOption {
    Username(String),
    Email(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserLogin {
    pub username: LoginOption,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserData {
    pub id: i32,
    pub username: String,
    pub email: String,
}

impl From<User> for UserData {
    fn from(user: User) -> Self {
        UserData {
            id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl UserLogin {
    pub fn new(username: LoginOption, password: String) -> Self {
        UserLogin { username, password }
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

    pub fn authenticate(
        &self,
        pool: &DbPool,
        duration: Duration,
    ) -> Result<(User, Session), Box<dyn Error>> {
        let user = match &self.username {
            LoginOption::Username(username) => User::get_by_username(username, pool)?,
            LoginOption::Email(email) => User::get_by_email(email, pool)?,
        };
        if password::verify_password(&self.password, &user.password_hash).is_err() {
            return Err("Invalid password".into());
        }
        let new_session = NewSession::new(user.id, duration);
        let session = Session::create(new_session, pool)?;
        Ok((user, session))
    }
}

#[cfg_attr(not(target_arch = "wasm32"), derive(Insertable))]
#[derive(Debug,Serialize,Deserialize,Clone)]
#[cfg_attr(not(target_arch = "wasm32"), diesel (table_name = super::schema::users))]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}
impl NewUser {
    pub fn new(username: String, email: String, password: String) -> Result<Self, Box<dyn Error>> {
        let password_hash =
            password::hash_password(&password).map_err(|_| "Couldn't hash password")?;
        Ok(NewUser {
            username,
            email,
            password_hash,
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl User {
    pub fn add_user(new_user: NewUser, pool: &DbPool) -> Result<User, Box<dyn Error>> {
        let mut conn = pool.get()?;
        let user = diesel::insert_into(dsl::users)
            .values(&new_user)
            .get_result(&mut conn)?;
        Ok(user)
    }
    pub fn get_by_id(p_id: i32, pool: &DbPool) -> Result<User, Box<dyn Error>> {
        let mut conn = pool.get()?;
        let user = dsl::users
            .filter(dsl::id.eq(p_id))
            .first(&mut conn)
            .map_err(|e| e)?;
        Ok(user)
    }

    pub fn get_by_username(p_username: &str, pool: &DbPool) -> Result<User, Box<dyn Error>> {
        let mut conn = pool.get()?;
        let user = dsl::users
            .filter(dsl::username.eq(p_username))
            .first(&mut conn)
            .map_err(|e| e)?;
        Ok(user)
    }

    pub fn get_by_email(p_email: &str, pool: &DbPool) -> Result<User, Box<dyn Error>> {
        let mut conn = pool.get()?;
        let user = dsl::users.filter(dsl::email.eq(p_email)).first(&mut conn)?;
        Ok(user)
    }

    pub fn get_by_login(login: &str, pool: &DbPool) -> Result<User, Box<dyn Error>> {
        let mut conn = pool.get()?;
        let user = dsl::users
            .filter(
                dsl::username.eq(login)
                    .or(
                        dsl::email.eq(login)
                    )).first(&mut conn)?;
        Ok(user)
    }

    pub fn delete(&self, pool: &DbPool) -> Result<(), Box<dyn Error>> {
        let mut conn = pool.get()?;
        diesel::delete(dsl::users.find(self.id)).execute(&mut conn)?;
        Ok(())
    }
}
