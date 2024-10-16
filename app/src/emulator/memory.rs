use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
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
fn TBody() -> impl IntoView {
    let shape = expect_context::<RwSignal<MemoryShape>>();
    let start = expect_context::<RwSignal<MemoryStart>>();
    view! {
        <tbody>
            <tr>
                <For each=move || 0..shape.with(|shape| shape.height) key=|n| *n let:height_idx>
                    <th>
                        {if let Some(val) = get_mem_address(
                            start.read().start,
                            shape.with(|shape| shape.width),
                            height_idx,
                            0,
                        ) {
                            format!("{:04X}", val)
                        } else {
                            "N/A".to_string()
                        }}
                    </th>
                    <For each=move || 0..shape.with(|shape| shape.width) key=|n| *n let:width_idx>
                        <td>
                            <input />
                            {height_idx}
                        </td>
                    </For>
                </For>
            </tr>
        </tbody>
    }
}
#[island]
pub fn Memory() -> impl IntoView {
    let emu_signal = expect_context::<RwSignal<Emulator<Z80>>>();
    provide_context(RwSignal::new(MemoryShape {
        width: 0x10,
        height: 0x10,
    }));
    view! {
        <table>
            <thead></thead>
            <tbody></tbody>
        </table>
    }
}
