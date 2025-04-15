use super::{emu_style, EmulatorCfgContext, EmulatorContext};
use crate::utils::icons::Icon;
use emu_lib::memory::MemoryDevice;
use leptos::ev::Event;
use std::ops::Div;
use leptos::prelude::*;
use leptos::web_sys::HtmlInputElement;
use leptos::{html, IntoView};
use leptos_use::{on_click_outside_with_options, OnClickOutsideOptions};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum MemDisplay {
    Hex,
    Dec,
    Ascii,
}

impl MemDisplay {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "hex" => Some(MemDisplay::Hex),
            "dec" => Some(MemDisplay::Dec),
            "ascii" => Some(MemDisplay::Ascii),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            MemDisplay::Hex => "hex",
            MemDisplay::Dec => "dec",
            MemDisplay::Ascii => "ascii",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct MemoryContext {
    pub width: u16,
    pub height: u16,
    pub start: u16,
    pub display: MemDisplay,
}

impl Default for MemoryContext {
    fn default() -> Self {
        MemoryContext {
            width: 0x10,
            height: 0x10,
            start: 0x0,
            display: MemDisplay::Hex,
        }
    }
}

#[island]
fn MemoryTHead() -> impl IntoView {
    let emu_ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    let width = Memo::new(move |_| emu_ctx.with(|emu_ctx| emu_ctx.mem_config.width));
    view! {
        <thead>
            <tr>
                <th></th>
                <For each=move || 0..width() key=|n| *n let:data>
                    <th>{format!("{:X}", data)}</th>
                </For>
            </tr>
        </thead>
    }
}

fn get_mem_address(start: u16, width: u16, col: u16, row: u16) -> u16 {
    row.wrapping_mul(width)
        .wrapping_add(col)
        .wrapping_add(start)
}

fn parse_hex(value: &str) -> Option<u8> {
    u8::from_str_radix(value, 16).ok()
}

fn format_hex(value: u8) -> String {
    format!("{:02X}", value)
}

fn parse_dec(value: &str) -> Option<u8> {
    u8::from_str_radix(value, 10).ok()
}

fn format_dec(value: u8) -> String {
    format!("{}", value)
}

fn parse_ascii(value: &str) -> Option<u8> {
    value.chars().next().map(|c| c as u8)
}

fn format_ascii(value: u8) -> String {
    if value.is_ascii() {
        format!("{}", value as char)
    } else {
        //display some replacement character
        "?".to_string()
    }
}

fn parse_value(value: &str, display: MemDisplay) -> Option<u8> {
    match display {
        MemDisplay::Hex => parse_hex(value),
        MemDisplay::Dec => parse_dec(value),
        MemDisplay::Ascii => parse_ascii(value),
    }
}

fn format_value(value: u8, display: MemDisplay) -> String {
    match display {
        MemDisplay::Hex => format_hex(value),
        MemDisplay::Dec => format_dec(value),
        MemDisplay::Ascii => format_ascii(value),
    }
}

#[island]
fn MemoryMemCell(column: u16, row: u16) -> impl IntoView {
    let emu_ctx = expect_context::<RwSignal<EmulatorContext>>();
    let emu_cfg_ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    let display = Memo::new(move |_| emu_cfg_ctx.with(|ctx| ctx.mem_config.display));
    let max_length = Memo::new(move |_| match display.get() {
        MemDisplay::Hex => 2,
        MemDisplay::Dec => 3,
        MemDisplay::Ascii => 1,
    });
    let shape = Memo::new(move |_| emu_cfg_ctx.with(|ctx| ctx.mem_config));
    let vw = move || {
        let address = shape.with(|shape| get_mem_address(shape.start, shape.width, column, row));
        let changed = Memo::new(move |_| {
            emu_ctx.with(|emu| {
                if let Some(addresses) = emu.emu.memory.get_changes() {
                    addresses.contains(&address)
                } else {
                    false
                }
            })
        });
        let changed_class = Memo::new(move |_| {
            if changed.get() {
                emu_style::changed
            } else {
                ""
            }
        });
        let read_mem = Memo::new(move |_| {
            emu_ctx.with(|emu| match emu.emu.memory.read_8(address) {
                Ok(val) => format_value(val, display()),
                _ => "N/A".to_string(),
            })
        });
        let write_mem = move |ev: Event| {
            let value = event_target_value(&ev);
            match parse_value(&value, display()) {
                Some(val) => {
                    emu_ctx.update(|emu| {
                        emu_cfg_ctx.update(|cfg| {
                            if let Err(err) = emu.emu.memory.write_8(address, val) {
                                cfg.logstore.log_error(
                                    "Memory write error",
                                    format!("Memory write error: {}", err),
                                );
                            } else {
                                cfg.logstore.log_info(
                                    "Memory written",
                                    format!("Memory write: ({:#04X}) = {:#04X}", address, val),
                                );
                            }
                        });
                        emu.emu.memory.clear_change(address);
                    });
                }
                None => {
                    emu_cfg_ctx.update(|cfg| {
                        cfg.logstore.log_error(
                            "Memory write error",
                            format!(
                                "Memory write error: invalid value {}, not {}",
                                value,
                                cfg.mem_config.display.to_str()
                            ),
                        );
                    });
                    let input: HtmlInputElement = event_target(&ev);
                    input.set_value(&format!("{}", read_mem()));
                }
            }
        };
        view! { <input class=changed_class maxlength=max_length on:change=write_mem prop:value=read_mem /> }.into_any()
    };
    vw.into_view()
}

#[island]
fn MemoryTBody() -> impl IntoView {
    let ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    let shape = Memo::new(move |_| ctx.with(|ctx| ctx.mem_config));
    view! {
        <tbody>
            <For each=move || 0..shape.with(|shape| shape.height) key=|n| *n let:row>
                <tr>
                    <th>
                        {move || {
                            let val = get_mem_address(
                                shape.with(|shape| shape.start),
                                shape.with(|shape| shape.width),
                                0,
                                row,
                            );
                            format!("{:04X}", val)
                        }}
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

#[derive(Clone)]
struct DisplayMemorySettings{
    signal: RwSignal<bool>,
}

#[island]
pub fn SettingsInner() -> impl IntoView {
    let emu_cfg_ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    let display = Memo::new(move |_| emu_cfg_ctx.with(|ctx| ctx.mem_config.display));
    let change_display = move |dsp: MemDisplay| {
        emu_cfg_ctx.update(|ctx| {
            ctx.mem_config.display = dsp;
        });
    };
    let should_display = expect_context::<DisplayMemorySettings>();
    let noderef = NodeRef::new();
    let _ = on_click_outside_with_options(noderef, move |_| {should_display.signal.update(|state| *state = false)},
                                          OnClickOutsideOptions::default().ignore(["div", ".memsetbtn"]),
    );

    view! {
        <div node_ref=noderef class=emu_style::secsettingsinner>
            <div>
                <span>Display mode:</span>
            </div>
            <div>
                <input
                    type="radio"
                    id="hexdsp"
                    name="displaymode"
                    value="Hex"
                    on:click=move |_| change_display(MemDisplay::Hex)
                    prop:checked=move || {
                        if let MemDisplay::Hex = display.get() { true } else { false }
                    }
                />
                <label for="hexdsp">Hex</label>
            </div>
            <div>
                <input
                    type="radio"
                    id="decdsp"
                    name="displaymode"
                    value="Dec"
                    on:click=move |_| change_display(MemDisplay::Dec)
                    prop:checked=move || {
                        if let MemDisplay::Dec = display.get() { true } else { false }
                    }
                />
                <label for="decdsp">Dec</label>
            </div>
            <div>
                <input
                    type="radio"
                    id="asciidsp"
                    name="displaymode"
                    value="Ascii"
                    on:click=move |_| change_display(MemDisplay::Ascii)
                    prop:checked=move || {
                        emu_cfg_ctx
                            .with(|ctx| {
                                if let MemDisplay::Ascii = display.get() { true } else { false }
                            })
                    }
                />
                <label for="asciidsp">ASCII</label>
            </div>
        </div>
    }
}

#[island]
pub fn Settings() -> impl IntoView {
    let display_settings = RwSignal::new(false);
    provide_context::<DisplayMemorySettings>(DisplayMemorySettings {
        signal: display_settings.clone(),
    });
    let switch_settings = move |_| {
        display_settings.update(|state| *state = !*state);
    };
    view! {
        <div class=emu_style::sectop>
            <span>Memory</span>
            <div class=emu_style::secsettings>
                <div class="memsetbtn" on:click=switch_settings>
                    <Icon name="ri-settings-3-fill".to_string() />
                </div>
                <Show when=move || display_settings.get() fallback=move || { "".to_string() }>
                    <SettingsInner />
                </Show>
            </div>
        </div>
    }
}
#[island]
pub fn Memory() -> impl IntoView {
    let emu_cfg_ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    view! {
        <div class=emu_style::memorymap>
            <Settings />
            <table
                class=emu_style::memorymaptable
                on:wheel=move |ev| {
                    ev.prevent_default();
                    let delta_y = ev.delta_y();
                    let delta_direction = delta_y.signum() as i16;
                    let delta_abs = delta_y.abs();
                    let delta_magnitude = delta_abs.div(50.0).ceil() as i16;
                    emu_cfg_ctx
                        .update(|emu_cfg| {
                            let offset = delta_direction * delta_magnitude
                                * emu_cfg.mem_config.width as i16;
                            emu_cfg.mem_config.start = (emu_cfg.mem_config.start as i32)
                                .wrapping_add(offset as i32) as u16;
                        });
                }
            >
                <MemoryTHead />
                <MemoryTBody />
            </table>
        </div>
    }
}
