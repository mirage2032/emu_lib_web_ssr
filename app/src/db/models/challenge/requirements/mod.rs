pub mod memory;

use emu_lib_ui::emu_lib::cpu::z80::Z80;
use emu_lib_ui::emu_lib::emulator::Emulator;
use memory::MemoryRequirement;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Requirement {
    Memory(MemoryRequirement),
    MaxCycles(usize),
}

impl Requirement {
    pub fn check(&self, emulator: &Emulator<Z80>) -> Result<(), String> {
        match self {
            Requirement::Memory(req) => req.check(&emulator.memory),
            Requirement::MaxCycles(cycles) => {
                if *cycles <= emulator.cycles {
                    Ok(())
                } else {
                    Err(format!(
                        "Program took {} cycles, maximum is {}",
                        emulator.cycles, cycles
                    ))
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Requirements {
    pub requirements: Vec<Requirement>,
}

impl Requirements {
    pub fn check_all(&self, emulator: &Emulator<Z80>) -> Result<(), Vec<(usize, String)>> {
        let mut errors = vec![];
        for (index, requirement) in self.requirements.iter().enumerate() {
            if let Err(error) = requirement.check(emulator) {
                errors.push((index, error))
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
}

impl Default for Requirements {
    fn default() -> Self {
        Requirements {
            requirements: vec![],
        }
    }
}
