mod control;

use emu_lib::cpu::z80::Z80;
use leptos::prelude::*;
use emu_lib::emulator::Emulator;
use leptos_meta::Title;

#[island]
pub fn EmulatorNoTitle()-> impl IntoView{
    let emu_signal = use_context::<RwSignal<Emulator<Z80>>>()
        .unwrap_or({
            let mut emu = Emulator::<Z80>::default();
            emu.memory.record_changes(true);
            RwSignal::new(emu)
        });
}
#[component]
pub fn Emulator() -> impl IntoView{
    view!{
        <Title text="emulator"/>
        <EmulatorNoTitle/>
    }
}
