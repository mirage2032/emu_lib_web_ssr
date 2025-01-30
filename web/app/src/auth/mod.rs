use std::collections::HashMap;
use std::time::Duration;
use emu_lib::cpu::instruction::InstructionParser;
use emu_lib::cpu::z80::parser::Z80_PARSER;
use emu_lib::cpu::z80::Z80;
use emu_lib::memory::MemoryDevice;
use indexmap::IndexMap;
use leptos::{island, view, IntoView};
use leptos::prelude::*;
use stylance::classes;

pub mod api;
pub mod login;
pub mod register;

stylance::import_style!(auth_style, "./auth.module.scss");
const bg_text: &'static str = include_str!("bg_text.txt");

fn background_lines() -> Vec<&'static str> {
    bg_text.lines().collect()
}

#[island]
pub fn AuthBackground() -> impl IntoView{
    let mut emulator = emu_lib::emulator::Emulator::<Z80>::default();
    let rom = include_bytes!("color.bin");
    emulator.memory.load(rom,true).expect("Failed to load ROM");
    let mut instructions = IndexMap::new();
    let mut current_address = 0;
    let mut current_instruction = 0;
    loop{
        if let Ok(0x00) = emulator.memory.read_8(current_address as u16) {
            break;
        }
        let instruction = Z80_PARSER.ins_from_machinecode(&emulator.memory, current_address as u16).expect("Failed to parse instruction");
        instructions.insert(current_address, (instruction.to_string(), current_instruction));
        current_address += instruction.common().length;
        current_instruction += 1;
    }
    let lines_len = instructions.len();
    let instructions = RwSignal::new(instructions);
    let emu_signal = RwSignal::new(emulator);
    let background_lines = move || instructions.with(|instructions| instructions.iter().map(|(_,(v,_))|v).enumerate().map(|(idx, val)| (idx, val.clone())).collect::<Vec<(usize, String)>>());
    let active = move || instructions.with(|instructions| instructions.get(&emu_signal.with(|emu| emu.cpu.registers.pc)).expect("Failed to get instruction").1);
    let next = move || {
        emu_signal.update(|emu| {emu.step();});
    };
    let instruction_class = move |idx:usize| {
        if idx == active() {
            classes! {
                auth_style::instruction,
                auth_style::active
            }.to_string()
        } else {
            classes! {
                auth_style::instruction
            }.to_string()
        }
    };
    Effect::new(move || {
        set_interval(next, Duration::from_millis(500));
    });
    view! {
        <div class=auth_style::authbg>
            <For each=background_lines key=|(id, val)| (*id,val.clone()) let((id,val))>
                <div class=move || instruction_class(id)>{val}</div>
            </For>
        </div>
    }
}