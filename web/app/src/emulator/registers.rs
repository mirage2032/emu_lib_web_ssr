use std::collections::HashMap;
use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use leptos::prelude::*;
use super::emu_style;

#[derive(Clone, Debug)]
struct GPRegisterSignals{
    read:Signal<u16>,
    write:SignalSetter<u16>
}

impl GPRegisterSignals {
    fn new(sig:(Signal<u16>,SignalSetter<u16>)) -> Self{
        Self{
            read:sig.0,
            write:sig.1
        }
    }
}

#[derive(Default,Clone)]
struct GPRegistersAllSignals {
    signals:HashMap<String,GPRegisterSignals>
}

#[island]
pub fn GPRegister(name:String) -> impl IntoView{
    let signals = use_context::<RwSignal<GPRegistersAllSignals>>().expect("No GPRegistersSignals context found");
    let val = signals.get();
    let full_val = val.signals.get(&name).expect("No signal found for this register").clone();
    view! {
        <table>
            <thead>
                <tr>
                    <th style:rowspan="2">{name.clone()}</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td style:rowspan="2"><input prop:value=full_val.read maxlength=4/></td>
                </tr>
                <tr>
                    <th>{name.as_bytes()[0].clone()}</th>
                    <th>{name.as_bytes()[1].clone()}</th>
                </tr>
                <tr>
                    <td></td>
                    <td></td>
                </tr>
            </tbody>
        </table>
    }
}

#[island]
pub fn GPRegisters() -> impl IntoView{
    let emu = expect_context::<RwSignal<Emulator<Z80>>>();
    let af = create_slice(
        emu,
        |emu|emu.cpu.registers.gp.af,
        |emu,val|emu.cpu.registers.gp.af=val
    );
    let bc = create_slice(
        emu,
        |emu|emu.cpu.registers.gp.bc,
        |emu,val|emu.cpu.registers.gp.bc=val
    );
    let de = create_slice(
        emu,
        |emu|emu.cpu.registers.gp.de,
        |emu,val|emu.cpu.registers.gp.de=val
    );
    let hl = create_slice(
        emu,
        |emu|emu.cpu.registers.gp.hl,
        |emu,val|emu.cpu.registers.gp.hl=val
    );
    provide_context(RwSignal::new(GPRegistersAllSignals{
        signals:HashMap::from([
            ("AF".to_string(),GPRegisterSignals::new(af)),
            ("BC".to_string(),GPRegisterSignals::new(bc)),
            ("DE".to_string(),GPRegisterSignals::new(de)),
            ("HL".to_string(),GPRegisterSignals::new(hl)),
        ])
    }));
    view!{
        <div class=emu_style::registersgp>
            // <GPRegister name="AF".to_string()/>
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
