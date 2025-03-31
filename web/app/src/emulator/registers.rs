use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use leptos::prelude::*;
use super::emu_style;

#[island]
pub fn AFRegister() -> impl IntoView{
    let emu = expect_context::<RwSignal<Emulator<Z80>>>();
    view! {
        <table>
            <thead>
                <tr>
                    <th>AF</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td><input maxlength=4/></td>
                </tr>
            </tbody>
        </table>
    }
}

#[island]
pub fn GPRegisters() -> impl IntoView{
    view!{
        <div class=emu_style::registersgp>
            <AFRegister/>
        </div>
    }
}

#[island]
pub fn Registers() -> impl IntoView {
    // if use_context::<RwSignal<DisassemblerContext>>().is_none() {
    //     let ctx = RwSignal::new(DisassemblerContext::default());
    //     provide_context(ctx);
    // }

    view! {
        <div class=emu_style::registers>
            <span>Registers</span>
            <GPRegisters/>
        </div>
    }
}
