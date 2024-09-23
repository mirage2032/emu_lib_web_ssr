use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::{Duration, SystemTime};
#[cfg(not(target_arch = "wasm32"))]
use crate::db::DbPool;
#[cfg(not(target_arch = "wasm32"))]
use crate::db::models::schema::sessions::dsl;
#[cfg(not(target_arch = "wasm32"))]
use diesel::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use diesel::{ QueryDsl,  RunQueryDsl,Insertable,Queryable};

#[cfg_attr(not(target_arch = "wasm32"), derive(Queryable))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(Insertable))]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), diesel (table_name = super::schema::sessions))]
pub struct NewSession {
    pub user_id: i32,
    pub token: String,
    pub expires_at: SystemTime,
}
impl NewSession {
    pub fn new(user_id: i32, duration: Duration) -> Self {
        let token = uuid::Uuid::new_v4().to_string();
        let expires_at = SystemTime::now() + duration;
        NewSession {
            user_id,
            token,
            expires_at,
        }
    }
}


#[cfg(not(target_arch = "wasm32"))]
impl Session {
    pub fn create(new_session: NewSession, pool: &DbPool) -> Result<Session, Box<dyn Error>> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        let session = diesel::insert_into(dsl::sessions)
            .values(&new_session)
            .get_result(&mut conn)?;
        Ok(session)
    }

    pub fn get_by_id(p_id: i32, pool: &DbPool) -> Result<Session, Box<dyn Error>> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        let session = dsl::sessions.find(p_id).first(&mut conn)?;
        Ok(session)
    }

    pub fn get_by_token(p_token: &str, pool: &DbPool) -> Result<Session, Box<dyn Error>> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        let session = dsl::sessions
            .filter(dsl::token.eq(p_token))
            .first(&mut conn)?;
        Ok(session)
    }

    pub fn get_by_user_id(p_user_id: i32, pool: &DbPool) -> Result<Vec<Session>, Box<dyn Error>> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        let session = dsl::sessions
            .filter(dsl::user_id.eq(p_user_id))
            .load(&mut conn)?;
        Ok(session)
    }

    pub fn delete(&self, pool: &DbPool) -> Result<(), Box<dyn Error>> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        diesel::delete(dsl::sessions.find(self.id)).execute(&mut conn)?;
        Ok(())
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at < SystemTime::now()
    }
}
