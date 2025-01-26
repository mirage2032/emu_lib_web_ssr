#[cfg(not(target_arch = "wasm32"))]
use crate::db::models::schema::programs::dsl;
#[cfg(not(target_arch = "wasm32"))]
use crate::db::DbPool;
#[cfg(not(target_arch = "wasm32"))]
use diesel::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::SystemTime;
use emu_lib::cpu::instruction::InstructionParser;
use emu_lib::cpu::z80::parser::Z80_PARSER;

#[cfg_attr(not(target_arch = "wasm32"), derive(Queryable))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Program {
    pub id: i32,
    pub owner_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub data: String,
    pub compiles: bool,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Insertable))]
#[cfg_attr(not(target_arch = "wasm32"), diesel (table_name = super::schema::programs))]
pub struct NewProgram {
    pub owner_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub data: String,
    pub compiles: bool,
}

pub struct ProgramError {
    line: String,
    line_number: usize,
    error: String,
}

impl NewProgram {
    fn compile_check(program: &str) -> Result<(), Vec<ProgramError>> {
        let mut errors = vec![];
        for (line_number, line) in program.lines().enumerate() {
            if line.is_empty() {
                continue;
            };
            if let Err(error) = Z80_PARSER.ins_from_asm_string(line) {
                errors.push({
                    ProgramError {
                        line: line.to_string(),
                        line_number,
                        error: error.to_string(),
                    }
                });
            }
        }
        if errors.is_empty() {
            return Ok(());
        }
        Err(errors)
    }
    pub fn new(owner_id: i32, name: String, description: Option<String>, data: String) -> Self {
        let compiles = match Self::compile_check(&data) {
            Ok(()) => true,
            _ => false,
        };
        NewProgram {
            owner_id,
            name,
            description,
            data,
            compiles,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Program {
    pub fn new(new_program: NewProgram, pool: &DbPool) -> Result<Program, Box<dyn Error>> {
        let mut conn = pool.get()?;
        let user = diesel::insert_into(dsl::programs)
            .values(&new_program)
            .get_result(&mut conn)?;
        Ok(user)
    }
    pub fn get_by_id(p_id: i32, pool: &DbPool) -> Result<Program, Box<dyn Error>> {
        let mut conn = pool.get()?;
        let user = dsl::programs
            .filter(dsl::id.eq(p_id))
            .first(&mut conn)
            .map_err(|e| e)?;
        Ok(user)
    }

    pub fn get_by_owner_id(p_owner_id: i32, pool: &DbPool) -> Result<Vec<Program>, Box<dyn Error>> {
        let mut conn = pool.get()?;
        let programs = dsl::programs
            .filter(dsl::owner_id.eq(p_owner_id))
            .load(&mut conn)?;

        Ok(programs)
    }

    pub fn delete(&self, pool: &DbPool) -> Result<(), Box<dyn Error>> {
        let mut conn = pool.get()?;
        diesel::delete(dsl::programs.find(self.id)).execute(&mut conn)?;
        Ok(())
    }
}
