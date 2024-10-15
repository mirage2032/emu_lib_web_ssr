use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use leptos::ev::MouseEvent;
use leptos::prelude::*;

fn click_button<T:Fn(MouseEvent)->() + 'static>(cb:T) -> impl IntoView{
    view!{
        <input
        on:click=move |ev| cb(ev)
        type="button"/>
    }
}
#[island]
pub fn Control()-> impl IntoView {
    let emu_signal = expect_context::<RwSignal<Emulator<Z80>>>();
    let step_fn = move || {
        emu_signal.update(|emu| { emu.step().unwrap(); })
    };
    let step_fn = move |_| {
        emu_signal.update(|emu| { emu.step().unwrap(); })
    };
    view!{
        {click_button(step_fn)}
    }
}