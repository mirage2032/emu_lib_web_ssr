use emu_lib::cpu::instruction::{ExecutableInstruction, InstructionParser};
use emu_lib::cpu::z80::parser::Z80_PARSER;
use leptos::prelude::*;
use super::{emu_style, EmulatorCfgContext, EmulatorContext};

#[derive(Clone, Copy, Debug)]
pub enum DisassemblerDisplayMode {
    String,
    Bytes,
}

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq)]
pub struct DisassemblerContext {
    pub start: Option<u16>,
    pub rows: u16,
}

impl Default for DisassemblerContext {
    fn default() -> Self {
        DisassemblerContext {
            start: None,
            rows: 20,
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
                <th>HexCode</th>
            </tr>
        </thead>
    }
}

#[island]
pub fn DisassemblerTRow(address: usize) -> impl IntoView {
    // let ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    let emu = expect_context::<RwSignal<EmulatorContext>>();
    let instruction = move || {
        // "N/A".to_string()
        if address > (u16::MAX as usize) {
            return Err("Outside range".to_string());
        }
        emu.with(|emu| {
            Z80_PARSER.ins_from_machinecode(&emu.emu.memory, address as u16).map_err(|err|err.to_string())
        })
    };
    let ins_bytes = Memo::new(move |_| {
        if let Ok(instruction)=instruction() {
            instruction
                .to_bytes()
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        } else { "N/A".to_string() }
    });
    let ins_string =Memo::new(move |_| {
        if let Ok(instruction)=instruction() {
            instruction.to_string()
        } else { "N/A".to_string() }
    });
    let breakpoint = move || {
        if emu.with(|emu| emu.emu.breakpoints.iter().all(|&bp|bp as usize!=address) ) == true {
            ""
        }else{
            "X"
        }
    };
    view! {
        <tr>
            <th>{move || format!("{:04X}", address)}</th>
            <td>{breakpoint}</td>
            <td>{ins_string}</td>
            <td>{ins_bytes}</td>
        </tr>
    }
}

#[island]
pub fn DisassemblerTBody() -> impl IntoView {
    let ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    let disasm = Memo::new(move |_| {
        ctx.with(|ctx| ctx.disasm_config)
    });
    let emu = expect_context::<RwSignal<EmulatorContext>>();
    let table_rows = move || {
        let mut offset = 0;
        let mut rows = vec![];
        let start = match disasm.with(|disasm| disasm.start) {
            Some(start) => start,
            None => emu.with(|emu| emu.emu.cpu.registers.pc),
        };
        for _ in 0..disasm.with(|disasm| disasm.rows) {
            let address: usize = start as usize + offset;
            if address > (u16::MAX as usize) {
                rows.push(view! { <DisassemblerTRow address /> });
            } else {
                rows.push(view! { <DisassemblerTRow address /> });
            }
            let ins_size = emu.with(|emu| {
                if let Ok(instruction) =
                    Z80_PARSER.ins_from_machinecode(&emu.emu.memory, address as u16)
                {
                    instruction.common().length
                } else {
                    1
                }
            });
            offset += ins_size as usize;
        }
        rows
    };
    view! { <tbody>{table_rows}</tbody> }
}
#[island]
pub fn Disassembler() -> impl IntoView {
    // if use_context::<RwSignal<DisassemblerContext>>().is_none() {
    //     let ctx = RwSignal::new(DisassemblerContext::default());
    //     provide_context(ctx);
    // }

    view! {
        <div class=emu_style::disassembler>
            <div class=emu_style::sectop>
                <span>Disassembler</span>
            </div>
            <table class=emu_style::disassemblertable>
                <DisassemblerTHead />
                <DisassemblerTBody />
            </table>
        </div>
    }
}
