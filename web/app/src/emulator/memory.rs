use super::{emu_style, EmulatorCfgContext, EmulatorContext};
use crate::utils::icons::Icon;
use emu_lib::memory::MemoryDevice;
use leptos::ev::Event;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::web_sys::HtmlInputElement;
use leptos::IntoView;
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[derive(PartialEq)]
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
            start: 0,
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

fn get_mem_address(start: u16, width: u16, col: u16, row: u16) -> Option<u16> {
    row.checked_mul(width)?.checked_add(col)?.checked_add(start)
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
    let ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    let display = Memo::new(move |_| ctx.with(|ctx| ctx.mem_config.display));
    let max_length = Memo::new(move |_| match display.get() {
        MemDisplay::Hex => 2,
        MemDisplay::Dec => 3,
        MemDisplay::Ascii => 1,
    });
    let shape = Memo::new(move |_| ctx.with(|ctx| ctx.mem_config));
    let vw = move || {
        if let Some(address) = get_mem_address(
            shape.with(|shape| shape.start),
            shape.with(|shape| shape.width),
            column,
            row,
        ) {
            let emu_signal = expect_context::<RwSignal<EmulatorContext>>();
            let changed = Memo::new(move |_| {
                emu_signal.with(|emu| { if let Some(addresses) = emu.emu.memory.get_changes() {
                    addresses.contains(&address)
                } else {
                    false
                } })});
            let changed_class = Memo::new(move |_| {
                if changed.get() {
                    emu_style::changed
                } else {
                    ""
                }
            });
            let read_mem = Memo::new(move |_| {
                emu_signal.with(|emu| match emu.emu.memory.read_8(address) {
                    Ok(val) => format_value(val, display()),
                    _ => "N/A".to_string(),
                })
            });
            let write_mem = move |ev: Event| {
                let value = event_target_value(&ev);
                match parse_value(&value, display()) {
                    Some(val) => {
                        emu_signal.update(|emu| {
                            if let Err(err) = emu.emu.memory.write_8(address, val) {
                                log!("{}", err);
                            } else {
                                log!("written: {} {}", address, val);
                            };
                            emu.emu.memory.clear_change(address); 
                        });
                    }
                    None => {
                        log!("{} is not a valid {} value", value, display().to_str());
                        let input: HtmlInputElement = event_target(&ev);
                        input.set_value(&format!("{}", read_mem()));
                    }
                }
            };
            view! { <input class=changed_class maxlength=max_length on:change=write_mem prop:value=read_mem /> }.into_any()
        } else {
            view! { <input prop:value=move || "N/A" maxlength=2 disabled /> }.into_any()
        }
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
                        {get_mem_address(
                                shape.with(|shape| shape.start),
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
pub fn SettingsInner() -> impl IntoView {
    let ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    let display = Memo::new(move |_| ctx.with(|ctx| ctx.mem_config.display));
    let change_display = move |dsp: MemDisplay| {
        ctx.update(|ctx| {
            ctx.mem_config.display = dsp;
        });
    };
    view! {
        <div class=emu_style::secsettingsinner>
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
                        ctx.with(|ctx| {
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
    let toggle_settings = move |_| {
        display_settings.update(|state| *state = !*state);
    };
    view! {
        <div class=emu_style::sectop>
            <span>Memory</span>
            <div class=emu_style::secsettings>
                <div on:click=toggle_settings>
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
    // if use_context::<RwSignal<MemoryCtx>>().is_none() {
    //     let shape = RwSignal::new(MemoryCtx {
    //         width: 0x10,
    //         height: 0x10,
    //         start: 0,
    //         display: MemDisplay::Hex,
    //     });
    //     provide_context(shape);
    // }
    view! {
        <div class=emu_style::memorymap>
            <Settings />
            <table class=emu_style::memorymaptable>
                <MemoryTHead />
                <MemoryTBody />
            </table>
        </div>
    }
}
