use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use emu_lib::memory::MemoryDevice;
use leptos::prelude::*;
use leptos::IntoView;

struct MemoryShape {
    pub width: u16,
    pub height: u16,
}

struct MemoryStart {
    pub start: u16,
}

#[island]
fn THead() -> impl IntoView {
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
fn MemCell() -> impl IntoView {
    
}

#[island]
fn TBody() -> impl IntoView {
    let emu_signal = expect_context::<RwSignal<Emulator<Z80>>>();
    let shape = expect_context::<RwSignal<MemoryShape>>();
    let start = expect_context::<RwSignal<MemoryStart>>();
    view! {
        <tbody>
            <For each=move || 0..shape.with(|shape| shape.height) key=|n| *n let:row>
                <tr>
                    <th>
                        {if let Some(val) = get_mem_address(
                            start.read().start,
                            shape.with(|shape| shape.width),
                            0,
                            row,
                        ) {
                            format!("{:04X}", val)
                        } else {
                            "N/A".to_string()
                        }}
                    </th>
                    <For each=move || 0..shape.with(|shape| shape.width) key=move |n| *n let:column>
                        <td>
                            <input
                                maxlength=2
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    match u8::from_str_radix(&value, 16) {
                                        Ok(val) => {
                                            let mem_address_opt = get_mem_address(
                                                start.read().start,
                                                shape.with(|shape| shape.width),
                                                column,
                                                row,
                                            );
                                            if let Some(mem_address) = mem_address_opt {
                                                emu_signal
                                                    .update(|emu| {
                                                        if let Err(err) = emu.memory.write_8(mem_address, val) {
                                                            log!("{}",err);
                                                        } else {
                                                            log!("written");
                                                        }
                                                    });
                                            } else {
                                                log!("not in memory range");
                                            }
                                        }
                                        Err(_) => log!("{} is not a valid hex value",value),
                                    }
                                }
                                prop:value=move || {
                                    emu_signal
                                        .with(|emu| {
                                            match get_mem_address(
                                                start.read().start,
                                                shape.with(|shape| shape.width),
                                                column,
                                                row,
                                            ) {
                                                Some(addr) => {
                                                    match emu.memory.read_8(addr) {
                                                        Ok(val) => format!("{:02X}", val),
                                                        _ => format!("N/A"),
                                                    }
                                                }
                                                None => {
                                                    format!("N/A")
                                                }
                                            }
                                        })
                                }
                            />
                        </td>
                    </For>
                </tr>
            </For>
        </tbody>
    }
}
#[island]
pub fn Memory() -> impl IntoView {
    provide_context(RwSignal::new(MemoryShape {
        width: 0x10,
        height: 0x10,
    }));
    provide_context(RwSignal::new(MemoryStart { start: 0 }));
    view! {
        <table>
            <THead />
            <TBody />
        </table>
    }
}
