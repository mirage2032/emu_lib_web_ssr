use std::collections::HashMap;
use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use leptos::ev::Event;
use leptos::prelude::*;
use leptos::web_sys::HtmlInputElement;
use super::emu_style;

#[derive(Clone,Debug)]
struct GPRegisterSignals16 {
    read:Signal<u16>,
    write:SignalSetter<u16>
}

impl GPRegisterSignals16 {
    fn new(sig:(Signal<u16>,SignalSetter<u16>)) -> Self{
        Self{
            read:sig.0,
            write:sig.1
        }
    }
}

#[derive(Clone,Debug)]
struct GPRegisterSignals8 {
    read:Signal<u8>,
    write:SignalSetter<u8>
}

impl GPRegisterSignals8 {
    fn new(sig:(Signal<u8>,SignalSetter<u8>)) -> Self{
        Self{
            read:sig.0,
            write:sig.1
        }
    }
}

#[derive(Default,Clone)]
struct GPRegistersAllSignals {
    signals16:HashMap<String, GPRegisterSignals16>,
    signals8:HashMap<String, GPRegisterSignals8>
}

#[island]
pub fn Register16Bit(name:String) -> impl IntoView{
    let signals = use_context::<RwSignal<GPRegistersAllSignals>>().expect("No GPRegistersSignals context found");
    let name_clone = name.clone();
    let full_val = move || {
        signals.get().signals16.get(&name_clone).expect("No signal found for this register").clone()
    };
    let full_val_clone = full_val.clone();
    let read_full = move || { format!("{:04X}", full_val().read.get()) };
    let write_full =  move |ev: Event| {
        let value = event_target_value(&ev);
        if let Ok(val) = u16::from_str_radix(&value, 16) {
            full_val_clone().write.set(val);
        }
    };
    view! {
        <table>
        <thead>
            <tr><th>{name}</th></tr>
        </thead>
        <tbody>
            <tr><td><input style:width="6ch" on:change=write_full prop:value=read_full maxlength=4/></td></tr>
        </tbody>
        </table>
    }
}

// #[island]
// pub fn Register8Bit(name:String) -> impl IntoView{
//     let signals = use_context::<RwSignal<GPRegistersAllSignals>>().expect("No GPRegistersSignals context found");
//     let sigclone = signals.clone();
//     let read_full = move || { format!("{:02X}", sigclone.get().signals8.get(&name).expect("No signal found for this register").read.get()) };
//     let write_full = |ev: Event| {
//         let value = event_target_value(&ev);
//         if let Ok(val) = u8::from_str_radix(&value, 16) {
//             signals.get().signals8.get(&name).expect("No signal found for this register").write.set(val);
//         }
//     };
//     view! {
//         <table>
//             <tr><th>{name.clone()}</th></tr>
//             <tr><td><input style:width="4ch" on:change=write_full prop:value=read_full maxlength=2/></td></tr>
//         </table>
//     }
// }

#[island]
pub fn GPRegister(name:String) -> impl IntoView{
    let signals = use_context::<RwSignal<GPRegistersAllSignals>>().expect("No GPRegistersSignals context found");
    let name_clone = name.clone();
    let full_val = move || signals.get().signals16.get(&name_clone).expect("No signal found for this register").clone();
    let full_val_clone = full_val.clone();
    let read_full = move || { format!("{:04X}", full_val_clone().read.get()) };
    let full_val_clone = full_val.clone();
    let write_full = move |ev: Event| {
        let value = event_target_value(&ev);
        if let Ok(val) = u16::from_str_radix(&value, 16) {
            full_val_clone().write.set(val);
        }
        else {
            let input:HtmlInputElement = event_target(&ev);
            input.set_value(&format!("{:04X}", full_val_clone().read.get()));
        }
    };
    let full_val_clone = full_val.clone();
    let read_higher = move || { let bytes = full_val_clone().read.get().to_be_bytes(); format!("{:02X}", bytes[0]) };
    let full_val_clone = full_val.clone();
    let write_higher = move |ev: Event| {
        let value = event_target_value(&ev);
        if let Ok(val) = u8::from_str_radix(&value, 16) {
            let mut bytes = full_val_clone().read.get().to_be_bytes();
            bytes[0] = val;
            full_val_clone().write.set(u16::from_be_bytes(bytes));
        }
        else {
            let input:HtmlInputElement = event_target(&ev);
            input.set_value(&format!("{:02X}", full_val_clone().read.get().to_be_bytes()[0]));
        }
    };
    let full_val_clone = full_val.clone();
    let read_lower = move || { let bytes = full_val_clone().read.get().to_be_bytes(); format!("{:02X}", bytes[1]) };
    let full_val_clone = full_val.clone();
    let write_lower = move |ev: Event| {
        let value = event_target_value(&ev);
        if let Ok(val) = u8::from_str_radix(&value, 16) {
            let mut bytes = full_val().read.get().to_be_bytes();
            bytes[1] = val;
            full_val_clone().write.set(u16::from_be_bytes(bytes));
        }
        else {
            let input:HtmlInputElement = event_target(&ev);
            input.set_value(&format!("{:02X}", full_val_clone().read.get().to_be_bytes()[1]));
        }
    };
    view! {
        <table>
            <thead>
                <tr>
                    <th colspan="2">{name.clone()}</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td colspan="2"><input maxlength=4 style:width="6ch" on:change=write_full prop:value=read_full/></td>
                </tr>
                <tr>
                    <th>{name.chars().next().unwrap().to_string()}</th>
                    <th>{name.chars().nth(1).unwrap().to_string()}</th>
                </tr>
                <tr>
                    <td><input maxlength=2 style:width="4ch" on:change=write_higher prop:value=read_higher/></td>
                    <td><input maxlength=2 style:width="4ch" on:change=write_lower prop:value=read_lower/></td>
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
    let af_alt = create_slice(
        emu,
        |emu|emu.cpu.registers.gp_alt.af,
        |emu,val|emu.cpu.registers.gp_alt.af=val
    );
    let bc_alt = create_slice(
        emu,
        |emu|emu.cpu.registers.gp_alt.bc,
        |emu,val|emu.cpu.registers.gp_alt.bc=val
    );
    let de_alt = create_slice(
        emu,
        |emu|emu.cpu.registers.gp_alt.de,
        |emu,val|emu.cpu.registers.gp_alt.de=val
    );
    let hl_alt = create_slice(
        emu,
        |emu|emu.cpu.registers.gp_alt.hl,
        |emu,val|emu.cpu.registers.gp_alt.hl=val
    );
    let pc = create_slice(
        emu,
        |emu|emu.cpu.registers.pc,
        |emu,val|emu.cpu.registers.pc=val
    );
    let sp = create_slice(
        emu,
        |emu|emu.cpu.registers.sp,
        |emu,val|emu.cpu.registers.sp=val
    );
    let ix = create_slice(
        emu,
        |emu|emu.cpu.registers.ix,
        |emu,val|emu.cpu.registers.ix=val
    );
    let iy = create_slice(
        emu,
        |emu|emu.cpu.registers.iy,
        |emu,val|emu.cpu.registers.iy=val
    );
    let i = create_slice(
        emu,
        |emu|emu.cpu.registers.i,
        |emu,val|emu.cpu.registers.i=val
    );
    let r = create_slice(
        emu,
        |emu|emu.cpu.registers.r,
        |emu,val|emu.cpu.registers.r=val
    );
    provide_context(RwSignal::new(GPRegistersAllSignals{
        signals16:HashMap::from([
            ("AF".to_string(), GPRegisterSignals16::new(af)),
            ("BC".to_string(), GPRegisterSignals16::new(bc)),
            ("DE".to_string(), GPRegisterSignals16::new(de)),
            ("HL".to_string(), GPRegisterSignals16::new(hl)),
            ("AF'".to_string(), GPRegisterSignals16::new(af_alt)),
            ("BC'".to_string(), GPRegisterSignals16::new(bc_alt)),
            ("DE'".to_string(), GPRegisterSignals16::new(de_alt)),
            ("HL'".to_string(), GPRegisterSignals16::new(hl_alt)),
            ("PC".to_string(), GPRegisterSignals16::new(pc)),
            ("SP".to_string(), GPRegisterSignals16::new(sp)),
            ("IX".to_string(), GPRegisterSignals16::new(ix)),
            ("IY".to_string(), GPRegisterSignals16::new(iy))
        ]),
        signals8:HashMap::from([
            ("I".to_string(), GPRegisterSignals8::new(i)),
            ("R".to_string(), GPRegisterSignals8::new(r))
        ])
    }));
    view!{
        <div class=emu_style::registersflex>
            <GPRegister name="AF".to_string()/>
            <GPRegister name="BC".to_string()/>
            <GPRegister name="DE".to_string()/>
            <GPRegister name="HL".to_string()/>
        </div>
        <div style:display="none" class=emu_style::registersflex>
            <GPRegister name="AF'".to_string()/>
            <GPRegister name="BC'".to_string()/>
            <GPRegister name="DE'".to_string()/>
            <GPRegister name="HL'".to_string()/>
        </div>
        <div class=emu_style::registersflex>
            <Register16Bit name="PC".to_string()/>
            <Register16Bit name="SP".to_string()/>
            <Register16Bit name="IX".to_string()/>
            <Register16Bit name="IY".to_string()/>
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
