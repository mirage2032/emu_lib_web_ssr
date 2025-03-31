mod control;
mod disassembler;
mod memory;
mod registers;

use crate::utils::logger;

use control::Control;
use disassembler::Disassembler;
use memory::Memory;

use crate::utils::logger::{LoggerSignal, LoggerStoreSignal};
use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use leptos::prelude::*;
use leptos_meta::Title;
use crate::emulator::registers::Registers;

stylance::import_style!(emu_style, "./emulator.module.scss");

fn default_emu() -> Emulator<Z80> {
    let mut emu = Emulator::<Z80>::default();
    emu.memory.record_changes(true);
    emu
}

#[island]
pub fn EmulatorNoTitle() -> impl IntoView {
    if use_context::<RwSignal<Emulator<Z80>>>().is_none() {
        let emu = default_emu();
        provide_context(RwSignal::new(emu));
    }
    view! {
        <Control />
        <Memory />
        <div class=emu_style::disasmregsflex>
            <Disassembler />
            <Registers />
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
