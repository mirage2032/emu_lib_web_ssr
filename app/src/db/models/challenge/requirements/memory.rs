use emu_lib_ui::emu_lib::memory::Memory;
use emu_lib_ui::emu_lib::memory::MemoryDevice;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MemoryConditionNumOperator {
    U8(u8),
    U16(u16),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MemoryConditionOperator {
    Num(MemoryConditionNumOperator),
    Vec(Vec<u8>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MemoryCondition {
    BiggerThan(MemoryConditionNumOperator),
    BiggerThanOrEq(MemoryConditionNumOperator),
    Equal(MemoryConditionOperator),
    LowerThan(MemoryConditionNumOperator),
    LowerThanOrEq(MemoryConditionNumOperator),
}

impl MemoryCondition {
    pub fn check(&self, location: u16, memory: &Memory) -> Result<(), String> {
        match self {
            MemoryCondition::BiggerThan(val) => match val {
                MemoryConditionNumOperator::U8(val) => {
                    if memory.read_8(location)? > *val {
                        Ok(())
                    } else {
                        Err("Comparison failed".to_string())
                    }
                }
                MemoryConditionNumOperator::U16(val) => {
                    if memory.read_16(location)? > *val {
                        Ok(())
                    } else {
                        Err("Comparison failed".to_string())
                    }
                }
            },
            MemoryCondition::BiggerThanOrEq(val) => match val {
                MemoryConditionNumOperator::U8(val) => {
                    if memory.read_8(location)? >= *val {
                        Ok(())
                    } else {
                        Err("Comparison failed".to_string())
                    }
                }
                MemoryConditionNumOperator::U16(val) => {
                    if memory.read_16(location)? >= *val {
                        Ok(())
                    } else {
                        Err("Comparison failed".to_string())
                    }
                }
            },
            MemoryCondition::LowerThan(val) => match val {
                MemoryConditionNumOperator::U8(val) => {
                    if memory.read_8(location)? < *val {
                        Ok(())
                    } else {
                        Err("Comparison failed".to_string())
                    }
                }
                MemoryConditionNumOperator::U16(val) => {
                    if memory.read_16(location)? < *val {
                        Ok(())
                    } else {
                        Err("Comparison failed".to_string())
                    }
                }
            },
            MemoryCondition::LowerThanOrEq(val) => match val {
                MemoryConditionNumOperator::U8(val) => {
                    if memory.read_8(location)? <= *val {
                        Ok(())
                    } else {
                        Err("Comparison failed".to_string())
                    }
                }
                MemoryConditionNumOperator::U16(val) => {
                    if memory.read_16(location)? <= *val {
                        Ok(())
                    } else {
                        Err("Comparison failed".to_string())
                    }
                }
            },
            MemoryCondition::Equal(val) => match val {
                MemoryConditionOperator::Num(MemoryConditionNumOperator::U8(val)) => {
                    if memory.read_8(location)? == *val {
                        Ok(())
                    } else {
                        Err("Comparison failed".to_string())
                    }
                }
                MemoryConditionOperator::Num(MemoryConditionNumOperator::U16(val)) => {
                    if memory.read_16(location)? == *val {
                        Ok(())
                    } else {
                        Err("Comparison failed".to_string())
                    }
                }
                MemoryConditionOperator::Vec(vec) => {
                    for (index, byte) in vec.iter().enumerate() {
                        if *byte != memory.read_8(location.wrapping_add(index as u16))? {
                            return Err("Comparison failed".to_string());
                        }
                    }
                    Ok(())
                }
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MemoryRequirement {
    location: u16,
    condition: MemoryCondition,
}

impl MemoryRequirement {
    pub fn check(&self, memory: &Memory) -> Result<(), String> {
        self.condition.check(self.location, memory)
    }
}
