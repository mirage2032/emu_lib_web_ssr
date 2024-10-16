mod control;
mod memory;

use control::Control;
use memory::Memory;

use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use leptos::prelude::*;
use leptos_meta::Title;

fn default_emu() -> Emulator<Z80> {
    let mut emu = Emulator::<Z80>::default();
    emu.memory.record_changes(true);
    emu
}

#[island]
pub fn EmulatorNoTitle() -> impl IntoView {
    if let None = use_context::<RwSignal<Emulator<Z80>>>() {
        let emu = default_emu();
        provide_context(RwSignal::new(emu));
    }
    view! {
        <Control />
        <Memory />
    }
}
#[component]
pub fn Emulator() -> impl IntoView {
    view! {
        <Title text="Emulator" />
        <EmulatorNoTitle />
    }
}
