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
            rows: 16,
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
            let pc = emu.cpu.registers.pc;
            let instruction_opt = Z80Parser::from_memdev(&emu.memory, pc);
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
                Err(err) => err,
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
    let start = move || ctx.with(|ctx| ctx.start.unwrap_or(emu.with(|emu| emu.cpu.registers.pc)));
    let rows = move || ctx.with(|ctx| ctx.rows);
    view! {
        <tbody>
            <For
                each=move || (start() as usize)..((start() as usize) + (rows() as usize))
                key=|n| *n
                let:address
            >
                <DisassemblerTRow address />
            </For>
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
        <table
        >
            <DisassemblerTHead />
            <DisassemblerTBody />
        </table>
    }
}
