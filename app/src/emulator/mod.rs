mod control;
mod disassembler;
mod memory;
use crate::utils::logger;

use control::Control;
use disassembler::Disassembler;
use memory::Memory;

use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use leptos::prelude::*;
use leptos_meta::Title;
use crate::utils::logger::AnyLoggerSignal;

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
    if use_context::<logger::LoggerContext>().is_none() {
        let master_logger = logger::LoggerSignal::<logger::MasterLogger>::default();
        provide_context(master_logger.context())
    }
    view! {
        <Control />
        <Memory />
        <Disassembler />
    }
}
#[component]
pub fn Emulator() -> impl IntoView {
    view! {
        <Title text="Emulator" />
        <EmulatorNoTitle />
    }
}
