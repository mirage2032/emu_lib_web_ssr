use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use emu_lib::memory::MemoryDevice;
use leptos::ev::Event;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::IntoView;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MemoryShape {
    pub width: u16,
    pub height: u16,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MemoryStart {
    pub start: u16,
}

#[island]
fn MemoryTHead() -> impl IntoView {
    let shape = expect_context::<RwSignal<MemoryShape>>();
    view! {
        <thead>
            <tr>
                <th></th>
                <For each=move || 0..shape.with(|shape| shape.width) key=|n| *n let:data>
                    <th>{data}</th>
                </For>
            </tr>
        </thead>
    }
}

fn get_mem_address(start: u16, width: u16, col: u16, row: u16) -> Option<u16> {
    row.checked_mul(width)?.checked_add(col)?.checked_add(start)
}

#[island]
fn MemoryMemCell(column: u16, row: u16) -> impl IntoView {
    let shape = expect_context::<RwSignal<MemoryShape>>();
    let start = expect_context::<RwSignal<MemoryStart>>();
    if let Some(address) = get_mem_address(
        start.read().start,
        shape.with(|shape| shape.width),
        column,
        row,
    ) {
        let emu_signal = expect_context::<RwSignal<Emulator<Z80>>>();
        let read_mem = move || {
            emu_signal.with(|emu| match emu.memory.read_8(address) {
                Ok(val) => format!("{:02X}", val),
                _ => "N/A".to_string(),
            })
        };
        let write_mem = move |ev: Event| {
            let value = event_target_value(&ev);
            match u8::from_str_radix(&value, 16) {
                Ok(val) => {
                    emu_signal.update(|emu| {
                        if let Err(err) = emu.memory.write_8(address, val) {
                            log!("{}", err);
                        } else {
                            log!("written: {} {}", address, val);
                        }
                    });
                }
                Err(_) => {
                    log!("{} is not a valid hex value", value);
                    emu_signal.notify();
                }
            }
        };
        view! { <input maxlength=2 style:width="3ch" on:change=write_mem prop:value=read_mem /> }
            .into_any()
    } else {
        view! { <input prop:value=move || "N/A" maxlength=2 disabled /> }.into_any()
    }
}

#[island]
fn MemoryTBody() -> impl IntoView {
    let shape = expect_context::<RwSignal<MemoryShape>>();
    let start = expect_context::<RwSignal<MemoryStart>>();
    view! {
        <tbody>
            <For each=move || 0..shape.with(|shape| shape.height) key=|n| *n let:row>
                <tr>
                    <th>
                        {get_mem_address(
                                start.read().start,
                                shape.with(|shape| shape.width),
                                0,
                                row,
                            )
                            .map(|val| format!("{:04X}", val))
                            .unwrap_or("N/A".to_string())}
                    </th>
                    <For each=move || 0..shape.with(|shape| shape.width) key=move |n| *n let:column>
                        <td>
                            <MemoryMemCell column row />
                        </td>
                    </For>
                </tr>
            </For>
        </tbody>
    }
}
#[island]
pub fn Memory() -> impl IntoView {
    if use_context::<RwSignal<MemoryShape>>().is_none() {
        let shape = RwSignal::new(MemoryShape {
            width: 0x10,
            height: 0x10,
        });
        provide_context(shape);
    }
    if use_context::<RwSignal<MemoryStart>>().is_none() {
        let start = MemoryStart { start: 0 };
        provide_context(RwSignal::new(start));
    }
    view! {
        <table>
            <MemoryTHead />
            <MemoryTBody />
        </table>
    }
}
