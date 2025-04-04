use emu_lib::cpu::z80::Z80;
use emu_lib::cpu::Cpu;
use emu_lib::emulator::Emulator;
use leptos::html::Input;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::closure::Closure;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;
use leptos::web_sys::{js_sys, HtmlInputElement};
use std::time::Duration;
use stylance::classes;
use crate::utils::logger::LogLevel;
use super::{emu_style, EmulatorCfgContext, EmulatorContext};
const BTN_CLASS: &str = "button";
#[island]
fn StepButton() -> impl IntoView {
    let emu_signal = expect_context::<RwSignal<EmulatorContext>>();
    view! {
        <input
            type="button"
            value="Step"
            on:click=move |_| {
                emu_signal
                    .update(|emu| {
                        if let Err(err) = emu.emu.step() {
                            log!("{}",err);
                        }
                    })
            }
        />
    }
}

#[island]
fn RunButton() -> impl IntoView {
    let emu_signal = expect_context::<RwSignal<EmulatorContext>>();
    let handle_sig: RwSignal<Option<IntervalHandle>> = RwSignal::new(None);

    let stop = move || {
        handle_sig.update(|handle_opt| {
            if let Some(handle) = handle_opt {
                handle.clear();
                *handle_opt = None;
            }
        })
    };

    let step = move || {
        emu_signal.update(|emu| {
            if let Err(err) = emu.emu.step() {
                log!("{}", err);
                stop();
            }
            if emu.emu.breakpoints.contains(&emu.emu.cpu.registers.pc) {
                log!("Breakpoint hit at {:#04X}", emu.emu.cpu.registers.pc);
                stop();
            }
        })
    };

    let start = move |duration| {
        let handle = set_interval_with_handle(step, duration).expect("Could not set interval");
        handle_sig.set(Some(handle));
        log!("Running");
    };

    let switch = move |duration| {
        let is_handle = handle_sig.with(|handle| handle.is_some());
        match is_handle {
            true => stop(),
            false => start(duration),
        };
    };

    view! {
        <input
            type="button"
            value="Run"
            class=move || {
                classes!(
                    BTN_CLASS,handle_sig.with(|&opt|if opt.is_some() {emu_style::activeinput} else {""})
                )
            }
            on:click=move |_| switch(Duration::from_millis(1))
        />
    }
}

#[island]
fn HaltButton() -> impl IntoView {
    let emu_signal = expect_context::<RwSignal<EmulatorContext>>();
    view! {
        <input
            type="button"
            value="Halt"
            class=move || {
                classes!(
                    if emu_signal.with(|emu|emu.emu.cpu.halted()) {emu_style::activeinput} else {""}
                )
            }
            on:click=move |_| {
                emu_signal
                    .update(|emu| {
                        emu.emu.cpu.set_halted(!emu.emu.cpu.halted());
                    })
            }
        />
    }
}

#[island]
fn ResetButton() -> impl IntoView {
    let emu_signal = expect_context::<RwSignal<EmulatorContext>>();
    view! {
        <input
            type="button"
            value="Reset"
            on:click=move |_| emu_signal.update(|emu| { emu.emu.cpu = Z80::default() })
        />
    }
}

#[island]
fn LoadButton() -> impl IntoView {
    let emu_signal = expect_context::<RwSignal<EmulatorContext>>();
    view! {
        <div class=emu_style::load>
            <label for="fileupload">
                <span>Load</span>
            </label>
            <input
                id="fileupload"
                value="Load"
                type="file"
                on:change=move |ev| {
                    if let Some(target) = ev.target() {
                        if let Some(files) = target.unchecked_ref::<HtmlInputElement>().files() {
                            if let Some(file) = files.get(0) {
                                let emu_signal = emu_signal.clone();
                                spawn_local(async move {
                                    let value = wasm_bindgen_futures::JsFuture::from(
                                            file.array_buffer(),
                                        )
                                        .await
                                        .expect("Error reading file");
                                    let array = js_sys::Uint8Array::new(&value);
                                    let data = array.to_vec();
                                    emu_signal
                                        .update(|emu| {
                                            if let Ok(_) = emu.emu.memory.load(&data, true) {
                                                log!("Loaded file");
                                            }
                                        });
                                });
                            }
                        }
                    }
                }
            />
        </div>
    }
}

pub fn fmt_timestamp(ts: &chrono::DateTime<chrono::Utc>) -> String {
    ts.format("%H:%M:%S").to_string()
}

#[island]
pub fn EmuLog() -> impl IntoView {
    let cfg = expect_context::<RwSignal<EmulatorCfgContext>>();
    let last_log = Memo::new(move |_| {
        cfg.with(|cfg| cfg.logstore.last_log().cloned())
    });
    let log_class =move || {
        last_log.with(|log| {
            if let Some(log) = log {
                match log.level {
                    LogLevel::Info => {
                        emu_style::info
                    },
                    LogLevel::Warning => {
                        emu_style::warning
                    },
                    LogLevel::Error => {
                        emu_style::error
                    }
                }
            } else {
                ""
            }
        })
    };
    let log_message = move || {
        last_log.with(|log| {
            if let Some(log) = log {
                log.message.clone()
            } else {
                String::new()
            }
        })
    };
    view! {
        <div class=emu_style::lastlog>
            <span class=log_class>{log_message}</span>
        </div>
    }
}

#[component]
pub fn Control() -> impl IntoView {
    view! {
        <div class=emu_style::emucontrol>
            <StepButton />
            <RunButton />
            <HaltButton />
            <ResetButton />
            <LoadButton />
            <EmuLog />
        </div>
    }
}
