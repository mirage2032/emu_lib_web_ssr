use super::{emu_style, EmulatorCfgContext, EmulatorContext};
use crate::utils::logger::LogLevel;
use emu_lib::cpu::instruction::ExecutableInstruction;
use emu_lib::cpu::z80::Z80;
use emu_lib::cpu::Cpu;
use emu_lib::emulator::{Emulator, StopReason};
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::{js_sys, HtmlInputElement};
use std::time::Duration;
use emu_lib::memory::MemoryDevice;
use leptos::wasm_bindgen;
use stylance::classes;
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

const BTN_CLASS: &str = "button";
#[island]
fn StepButton() -> impl IntoView {
    let emu_ctx = expect_context::<RwSignal<EmulatorContext>>();
    let emu_cfg_ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    view! {
        <input
            type="button"
            value="Step"
            on:click=move |_| {
                emu_ctx
                    .update(|emu| {
                        emu_cfg_ctx
                            .update(|emu_cfg| {
                                if let Err(err) = emu.emu.step() {
                                    emu_cfg
                                        .logstore
                                        .log_error(
                                            "Step error",
                                            format!(
                                                "Step error at {:#04X}: {}",
                                                emu.emu.cpu.registers.pc,
                                                err,
                                            ),
                                        );
                                } else {
                                    emu_cfg
                                        .logstore
                                        .log_info(
                                            "Step",
                                            format!("Step at {:#04X}", emu.emu.cpu.registers.pc),
                                        );
                                }
                            });
                    })
            }
        />
    }
}

#[island]
fn RunButton() -> impl IntoView {
    let emu_ctx = expect_context::<RwSignal<EmulatorContext>>();
    let emu_cfg_ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
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
        emu_ctx.update(|emu| {
            if let Err(err) = emu.emu.run_ticks(
                100000.0,
                &Some(move |emu: &mut Emulator<_>, instruction: &dyn ExecutableInstruction<_>| {}),
            ) {
                emu_cfg_ctx.update(|emu_cfg| match err {
                    StopReason::Halt => {
                        emu_cfg.logstore.log_info(
                            "Emulator stopped: halt",
                            "Emulator stopped due to a halt".to_string(),
                        );
                    }
                    StopReason::Error(err) => {
                        emu_cfg.logstore.log_error("Error", err);
                    }
                    StopReason::Breakpoint => {
                        emu_cfg.logstore.log_info(
                            "Emulator stopped: breakpoint",
                            format!(
                                "Emulator stopped due to a breakpoint at {:#04X}",
                                emu.emu.cpu.registers.pc
                            ),
                        );
                    }
                });
                stop();
            }
            // if emu.emu.breakpoints.contains(&emu.emu.cpu.registers.pc) {
            //     log!("Breakpoint hit at {:#04X}", emu.emu.cpu.registers.pc);
            //     stop();
            // }
        })
    };

    let start = move |duration| {
        let handle = set_interval_with_handle(step, duration).expect("Could not set interval");
        handle_sig.set(Some(handle));
        emu_cfg_ctx.update(|emu_cfg| {
            emu_cfg
                .logstore
                .log_info("Emulator started", "Emulator started".to_string());
        });
    };

    let switch = move |duration| {
        let is_handle = handle_sig.with(|handle| handle.is_some());
        match is_handle {
            true => {
                stop();
                emu_cfg_ctx.update(|emu_cfg| {
                    emu_cfg
                        .logstore
                        .log_info("Emulator stopped", "Emulator stopped".to_string());
                });
            }
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
    let emu_ctx = expect_context::<RwSignal<EmulatorContext>>();
    let emu_cfg_ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    view! {
        <input
            type="button"
            value="Halt"
            class=move || {
                classes!(
                    if emu_ctx.with(|emu|emu.emu.cpu.halted()) {emu_style::activeinput} else {""}
                )
            }
            on:click=move |_| {
                emu_ctx
                    .update(|emu| {
                        emu_cfg_ctx
                            .update(|emu_cfg| {
                                if emu.emu.cpu.halted() {
                                    emu_cfg
                                        .logstore
                                        .log_info(
                                            "Emulater unhalted",
                                            "Emulator unhalted".to_string(),
                                        );
                                    emu.emu.cpu.set_halted(false);
                                } else {
                                    emu_cfg
                                        .logstore
                                        .log_info("Emulator halted", "Emulator halted".to_string());
                                    emu.emu.cpu.set_halted(true);
                                }
                            });
                    })
            }
        />
    }
}

#[island]
fn ResetButton() -> impl IntoView {
    let emu_ctx = expect_context::<RwSignal<EmulatorContext>>();
    let emu_cfg_ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    let on_reset = move |_| {
        emu_cfg_ctx.update(|emu| {
            emu.logstore
                .log_info("Emulator reset", "Emulator reset".to_string());
        });
        emu_ctx.update(|emu| {
            emu.emu.cpu = Z80::default();
            emu.emu.reset_counters();
        });
    };
    view! { <input type="button" value="Reset" on:click=on_reset /> }
}

#[island]
fn ClearMemoryButton() -> impl IntoView {
    let emu_ctx = expect_context::<RwSignal<EmulatorContext>>();
    let emu_cfg_ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    view! {
        <input
            type="button"
            value="Clear Memory"
            on:click=move |_| {
                emu_ctx.update(|emu| {
                    emu_cfg_ctx.update(|emu_cfg| {
                        for addr in 0..emu.emu.memory.size() {
                            match emu.emu.memory.write_8_force(addr as u16,0){
                            Ok(_) => {},
                            Err(err) => {
                                emu_cfg.logstore.log_error(
                                    "Memory clear error",
                                    format!("Error clearing memory at {:#04X}: {}", addr, err),
                                );
                            }
                        }
                    }
                    });
                });
            }
        />
    }
}

// TODO: Adapt implementetation below for the SaveButton
// #[component]
// pub fn App() -> impl IntoView {
//     let node = NodeRef::<leptos::html::A>::new();
//     let bytes = RwSignal::<Vec<u8>>::default();
//     let trigger_download = RwSignal::new(false);
//     let href = Memo::new(|_| {
//         // .. construct blob url;
//         String::from("blob:12345")
//     });
//
//     Effect::new(move |_| {
//         if trigger_download.get() && !bytes.with(Vec::is_empty) {
//             if let Some(node) = node.get() {
//                 // do other things
//                 node.click();
//             }
//         }
//     });
//
//     view! {
//         <a node_ref=node style="display: hidden" download="somefile.bin" href=href></a>
//     }
// }

#[island]
fn SaveButton() -> impl IntoView {
    let emu_signal = expect_context::<RwSignal<EmulatorContext>>();
    let emu_ctx_signal = expect_context::<RwSignal<EmulatorCfgContext>>();
    view! {
        <div class=emu_style::save>
            <input
                id="filedownload"
                value="Save"
                type="button"
                on:click=move |_| {
                    let emu_signal = emu_signal.clone();
                    spawn_local(async move {
                        let data = emu_signal.with_untracked(|emu| emu.emu.memory.save().expect("Error saving memory"));
                        let uint8_array = js_sys::Uint8Array::from(&*data);

                        let blob = Blob::new_with_u8_array_sequence_and_options(&js_sys::Array::of1(&uint8_array),
                                                                                        BlobPropertyBag::new().type_("application/octet-stream"))
                            .expect("Error creating Blob");
                        let url = Url::create_object_url_with_blob(&blob).expect("Error creating URL");

                            // Step 4: Create a temporary anchor element
                            let document = web_sys::window().unwrap().document().unwrap();
                            let a = document.create_element("a").unwrap()
                                .dyn_into::<HtmlAnchorElement>()
                                .unwrap();

                            // Step 5: Set the download attributes
                            a.set_href(&url);
                            a.set_download("emu_memory.bin");

                            // Step 6: Append the anchor to the DOM
                            document.body().unwrap().append_child(&a).unwrap();

                            // Step 7: Programmatically click the anchor
                            a.click();

                            // Step 8: Remove the anchor from the DOM
                            document.body().unwrap().remove_child(&a).unwrap();

                            // Step 9: Revoke the object URL
                            Url::revoke_object_url(&url).expect("Error revoking URL");
                })
                }
            />
        </div>
    }
}

#[island]
fn LoadButton() -> impl IntoView {
    let emu_signal = expect_context::<RwSignal<EmulatorContext>>();
    let emu_ctx_signal = expect_context::<RwSignal<EmulatorCfgContext>>();
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
                                            for addr in 0..emu.emu.memory.size(){
                                                emu.emu.memory.write_8_force(addr as u16, 0).expect("Error clearing memory");
                                            }
                                            emu.emu.memory.clear_changes();
                                            if let Ok(_) = emu.emu.memory.load(&data, true) {
                                                emu_ctx_signal
                                                    .update(|emu_ctx| {
                                                        emu_ctx
                                                            .logstore
                                                            .log_info(
                                                                "File loaded",
                                                                format!("File loaded: {}", file.name()),
                                                            );
                                                    });
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
    let last_log = Memo::new(move |_| cfg.with(|cfg| cfg.logstore.last_log().cloned()));
    let log_class = move || {
        last_log.with(|log| {
            if let Some(log) = log {
                match log.level {
                    LogLevel::Info => emu_style::info,
                    LogLevel::Warning => emu_style::warning,
                    LogLevel::Error => emu_style::error,
                }
            } else {
                ""
            }
        })
    };
    let log_message = move || {
        last_log.with(|log| {
            if let Some(log) = log {
                log.short_message.to_string()
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
            <ClearMemoryButton />
            <SaveButton />
            <LoadButton />
            <EmuLog />
        </div>
    }
}
