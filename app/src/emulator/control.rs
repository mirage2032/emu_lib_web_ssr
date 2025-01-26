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

const BTN_CLASS: &str = "button";
#[island]
fn StepButton() -> impl IntoView {
    let emu_signal = expect_context::<RwSignal<Emulator<Z80>>>();
    view! {
        <input
            type="button"
            value="Step"
            class=BTN_CLASS
            on:click=move |_| {
                emu_signal
                    .update(|emu| {
                        if let Err(err) = emu.step() {
                            log!("{}",err);
                        }
                    })
            }
        />
    }
}

#[island]
fn RunButton() -> impl IntoView {
    let emu_signal = expect_context::<RwSignal<Emulator<Z80>>>();
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
            if let Err(err) = emu.step() {
                log!("{}", err);
                stop();
            }
        })
    };

    let start = move |duration| {
        let handle = set_interval_with_handle(step, duration);
        handle_sig.set(handle.ok());
    };

    let switch = move |duration| {
        handle_sig.update(|handle_opt| match handle_opt {
            Some(_) => stop(),
            None => start(duration),
        });
    };

    view! {
        <input
            type="button"
            value="Run"
            class=move || {
                classes!(BTN_CLASS,handle_sig.with(|&opt|if opt.is_some() {"active"} else {""}))
            }
            on:click=move |_| switch(Duration::from_millis(1))
        />
    }
}

#[island]
fn HaltButton() -> impl IntoView {
    let emu_signal = expect_context::<RwSignal<Emulator<Z80>>>();
    view! {
        <input
            type="button"
            value="Halt"
            class=move || {
                classes!(BTN_CLASS,if emu_signal.with(|emu|emu.cpu.halted()) {"active"} else {""})
            }
            on:click=move |_| {
                emu_signal
                    .update(|emu| {
                        emu.cpu.set_halted(!emu.cpu.halted());
                    })
            }
        />
    }
}

#[island]
fn ResetButton() -> impl IntoView {
    let emu_signal = expect_context::<RwSignal<Emulator<Z80>>>();
    view! {
        <input
            type="button"
            value="Reset"
            class=BTN_CLASS
            on:click=move |_| emu_signal.update(|emu| { emu.cpu = Z80::default() })
        />
    }
}

#[island]
fn LoadButton() -> impl IntoView {
    let emu_signal = expect_context::<RwSignal<Emulator<Z80>>>();
    view! {
        <input
            value="Load"
            class=BTN_CLASS
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
                                        if let Ok(_) = emu.memory.load(&data, true) {
                                            log!("Loaded file");
                                        }
                                    });
                            });
                        }
                    }
                }
            }
        />
    }
}

#[component]
pub fn Control() -> impl IntoView {
    view! {
        <div>
            <StepButton />
            <RunButton />
            <HaltButton />
            <ResetButton />
            <LoadButton />
        </div>
    }
}
