use emu_lib::cpu::z80::Z80;
use emu_lib::cpu::Cpu;
use emu_lib::emulator::Emulator;
use gloo::file::callbacks::read_as_bytes;
use gloo::file::File;
use leptos::prelude::*;
use leptos::web_sys::HtmlInputElement;
use std::time::Duration;
use stylance::classes;

const BTN_CLASS: &str = "button";
#[island]
fn StepButton() -> impl IntoView {
    let emu_signal = expect_context::<RwSignal<Emulator<Z80>>>();
    view! {
        <input
            type="button"
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
            type="file"
            class=BTN_CLASS
            on:change=move |ev| {
                let input: HtmlInputElement = event_target(&ev);
                if let Some(files) = input.files() {
                    if let Some(file) = files.get(0) {
                        let file = File::from(file);
                        read_as_bytes(
                            &file,
                            move |res| {
                                match res {
                                    Ok(data) => {
                                        emu_signal
                                            .update(|emu| {
                                                if let Err(err) = emu.memory.load(&data) {
                                                    log!("Emulator load error: {:?}",err);
                                                }
                                            })
                                    }
                                    Err(err) => log!("File read error: {}",err),
                                }
                            },
                        );
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
