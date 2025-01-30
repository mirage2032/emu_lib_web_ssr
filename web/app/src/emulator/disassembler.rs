use emu_lib::cpu::instruction::InstructionParser;
use emu_lib::cpu::z80::parser::Z80_PARSER;
use emu_lib::cpu::z80::{parser::Z80Parser, Z80};
use emu_lib::emulator::Emulator;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum DisassemblerDisplayMode {
    String,
    Bytes,
}

#[derive(Clone, Copy, Debug)]
pub struct DisassemblerContext {
    pub start: Option<u16>,
    pub rows: u16,
    pub display_mode: DisassemblerDisplayMode,
}

impl Default for DisassemblerContext {
    fn default() -> Self {
        DisassemblerContext {
            start: None,
            rows: 66,
            display_mode: DisassemblerDisplayMode::String,
        }
    }
}

#[component]
pub fn DisassemblerTHead() -> impl IntoView {
    view! {
        <thead>
            <tr>
                <th>Address</th>
                <th></th>
                <th>Instruction</th>
            </tr>
        </thead>
    }
}

#[island]
pub fn DisassemblerTRow(address: usize) -> impl IntoView {
    let ctx = expect_context::<RwSignal<DisassemblerContext>>();
    let emu = expect_context::<RwSignal<Emulator<Z80>>>();
    let instruction = move || {
        // "N/A".to_string()
        if address > (u16::MAX as usize) {
            return "N/A".to_string();
        }
        emu.with(|emu| {
            let instruction_opt = Z80_PARSER.ins_from_machinecode(&emu.memory, address as u16);
            match instruction_opt {
                Ok(instruction) => match ctx.with(|ctx| ctx.display_mode) {
                    DisassemblerDisplayMode::String => instruction.to_string(),
                    DisassemblerDisplayMode::Bytes => instruction
                        .to_bytes()
                        .iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<Vec<_>>()
                        .join(" "),
                },
                Err(err) => " ".to_string(),
            }
        })
    };
    view! {
        <tr>
            <th>{move || format!("{:04X}", address)}</th>
            <td>X</td>
            <td>{instruction}</td>
        </tr>
    }
}

#[island]
pub fn DisassemblerTBody() -> impl IntoView {
    let ctx = expect_context::<RwSignal<DisassemblerContext>>();
    let emu = expect_context::<RwSignal<Emulator<Z80>>>();
    let table_rows = move || {
        let mut offset = 0;
        let mut rows = vec![];
        let start = match ctx.with(|ctx| ctx.start) {
            Some(start) => start,
            None => emu.with(|emu| emu.cpu.registers.pc),
        };
        for _ in 0..ctx.with(|ctx| ctx.rows) {
            let address: usize = start as usize + offset;
            if address > (u16::MAX as usize) {
                rows.push( view! { <DisassemblerTRow address /> });
            } else {
                rows.push( view! { <DisassemblerTRow address /> });
            }
            let ins_size = emu.with(|emu|{
                if let Ok(instruction) = Z80_PARSER.ins_from_machinecode(&emu.memory, address as u16){
                    instruction.common().length
                } else {
                    1
                }
            });
            offset += ins_size as usize;
        };
        rows
    };
    view! {
        <tbody>
            {table_rows}
        </tbody>
    }
}
#[island]
pub fn Disassembler() -> impl IntoView {
    if use_context::<RwSignal<DisassemblerContext>>().is_none() {
        let ctx = RwSignal::new(DisassemblerContext::default());
        provide_context(ctx);
    }

    view! {
        <table>
            <DisassemblerTHead />
            <DisassemblerTBody />
        </table>
    }
}
