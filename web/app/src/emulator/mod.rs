mod account;
mod control;
mod disassembler;
mod editor;
mod info;
mod memory;
mod registers;

use crate::emulator::account::Account;
use crate::emulator::disassembler::DisassemblerContext;
use crate::emulator::editor::{Editor, EditorContext};
use crate::emulator::memory::MemoryContext;
use crate::emulator::registers::Registers;
use crate::utils::logger::LogStore;
use control::Control;
use disassembler::Disassembler;
use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use info::Info;
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use memory::Memory;

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
        EmulatorContext { emu: default_emu() }
    }
}

pub struct EmulatorCfgContext {
    pub mem_config: MemoryContext,
    pub disasm_config: DisassemblerContext,
    pub logstore: LogStore,
    pub editor: EditorContext,
}

impl Default for EmulatorCfgContext {
    fn default() -> Self {
        EmulatorCfgContext {
            mem_config: MemoryContext::default(),
            disasm_config: DisassemblerContext::default(),
            logstore: LogStore::default(),
            editor: EditorContext::default(),
        }
    }
}

#[island]
pub fn EmulatorNoTitle() -> impl IntoView {
    view! {
        <div>
            <Memory />
            <div class=emu_style::disasmregsinfoflex>
                <Disassembler />
                <div class=emu_style::regsinfo>
                    <Registers />
                    <Info />
                </div>
            </div>
        </div>
    }
}
#[island]
pub fn EmulatorInner() -> impl IntoView {
    if use_context::<RwSignal<EmulatorCfgContext>>().is_none() {
        let cfg = EmulatorCfgContext::default();
        provide_context(RwSignal::new(cfg));
    }
    if use_context::<RwSignal<EmulatorContext>>().is_none() {
        let emu = EmulatorContext::default();
        provide_context(RwSignal::new(emu));
        let cfg = expect_context::<RwSignal<EmulatorCfgContext>>();
        cfg.update(|cfg| {
            cfg.logstore.log_info(
                "Emulator initialized",
                "Emulator initialized with default settings".to_string(),
            );
        })
    }
    view! {
        <div class=emu_style::emumain>
        <Control />
        <div class=emu_style::emulator>
            <EmulatorNoTitle />
            <Editor />
            <Account />
        </div>
        </div>
    }
}

#[component]
pub fn Emulator() -> impl IntoView {
    view! {
        <Meta name="og:title" content="Emulator" />
        <Title text="Emulator" />
        <EmulatorInner />
    }
}
