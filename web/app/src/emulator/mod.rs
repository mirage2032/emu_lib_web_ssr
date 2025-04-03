mod control;
mod disassembler;
mod memory;
mod registers;
mod info;

use control::Control;
use info::Info;
use disassembler::Disassembler;
use memory::Memory;
use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use leptos::prelude::*;
use leptos_meta::Title;
use crate::emulator::disassembler::DisassemblerContext;
use crate::emulator::memory::MemoryContext;
use crate::emulator::registers::Registers;
use crate::utils::logger::LogStore;

stylance::import_style!(emu_style, "./emulator.module.scss");

fn default_emu() -> Emulator<Z80> {
    let mut emu = Emulator::<Z80>::default();
    emu.memory.record_changes(true);
    emu
}

pub struct EmulatorContext {
    pub emu: Emulator<Z80>,
}

impl Default for EmulatorContext {
    fn default() -> Self {
        EmulatorContext {
            emu: default_emu(),
        }
    }
}

pub struct EmulatorCfgContext {
    pub mem_config: MemoryContext,
    pub disasm_config: DisassemblerContext,
    pub logstore: LogStore,
}

impl Default for EmulatorCfgContext {
    fn default() -> Self {
        EmulatorCfgContext {
            mem_config: MemoryContext::default(),
            disasm_config: DisassemblerContext::default(),
            logstore: LogStore::default()
        }
    }
}

#[island]
pub fn EmulatorNoTitle() -> impl IntoView {
    if use_context::<RwSignal<EmulatorCfgContext>>().is_none() {
        let cfg = EmulatorCfgContext::default();
        provide_context(RwSignal::new(cfg));
    }
    if use_context::<RwSignal<EmulatorContext>>().is_none() {
        let emu = EmulatorContext::default();
        provide_context(RwSignal::new(emu));
        let cfg = expect_context::<RwSignal<EmulatorCfgContext>>();
        cfg.update(|cfg|{
            cfg.logstore.log_info("Emulator started".to_string());
        })
    }
    view! {
        <Control />
        <Memory />
        <div class=emu_style::disasmregsinfoflex>
            <Disassembler />
            <div class=emu_style::regsinfo>
                <Registers />
                <Info />
            </div>
        </div>
    }
}

#[component]
pub fn Emulator() -> impl IntoView {
    view! {
        <Title text="Emulator" />
        <div class=emu_style::emulator>
            <EmulatorNoTitle />
        // <div></div>
        </div>
    }
}
