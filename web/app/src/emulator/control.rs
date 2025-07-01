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
use js_sys::Date;

pub struct ControlContext {
    pub target_frequency: RwSignal<usize>,
    pub real_frequency: RwSignal<Option<usize>>
}

impl Default for ControlContext {
    fn default() -> Self {
        Self {
            target_frequency: RwSignal::new(3_579_545),
            real_frequency: RwSignal::new(None),
        }
    }
}

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

fn step_fn<FST, FSF>(
    mut step_count: usize,
    chunk_ticks: Memo<f64>,
    chunk_duration: Memo<f64>,
    step_ticks: FST,
    set_frequency: FSF,
    running: RwSignal<bool>,
    mut total_ticks: f64,
    start_time: f64,
    mut tick_accum: f64,
)
where
    FST: Fn(f64) + 'static,
    FSF: Fn(Option<usize>) + 'static,
{
    let now = Date::now();
    let elapsed = now - start_time;
    let chunk_dur = chunk_duration();

    // Calculate how many steps should have happened by now
    let expected_steps = (elapsed / chunk_dur).floor() as usize + 1;
    let missed_steps = expected_steps.saturating_sub(step_count);

    // Run enough ticks to catch up
    for _ in 0..=missed_steps {
        let ticks_per_step = chunk_ticks();
        let ticks_this_step = ticks_per_step.floor();
        tick_accum += ticks_per_step - ticks_this_step;
        let mut ticks = ticks_this_step;
        if tick_accum >= 1.0 {
            ticks += 1.0;
            tick_accum -= 1.0;
        }
        step_ticks(ticks);
        total_ticks += ticks;
        step_count += 1;
    }

    let real_frequency = if elapsed > 0.0 {
        total_ticks / elapsed * 1000.0
    } else {
        0.0
    };

    if running.get() {
        set_frequency(Some(real_frequency as usize));
        let next_target_time = start_time + (step_count as f64) * chunk_dur;
        let delay = (next_target_time - Date::now()).max(0.0);
        set_timeout(
            move || step_fn(
                step_count,
                chunk_ticks,
                chunk_duration,
                step_ticks,
                set_frequency,
                running,
                total_ticks,
                start_time,
                tick_accum,
            ),
            Duration::from_millis(delay as u64),
        );
    } else {
        set_frequency(None);
    }
}

#[island]
fn RunButton() -> impl IntoView {
    let emu_ctx = expect_context::<RwSignal<EmulatorContext>>();
    let emu_cfg_ctx = expect_context::<RwSignal<EmulatorCfgContext>>();

    let cpu_frequency = Memo::new(move |_| {
        emu_cfg_ctx.with(|emu_cfg| emu_cfg.control.target_frequency.get())
    });
    let refresh_rate = Memo::new(move |_| {
        emu_cfg_ctx.with(|emu_cfg| emu_cfg.display.refresh_rate.get())
    });
    let chunk_ticks = Memo::new(move |_| {
        (cpu_frequency.get() / refresh_rate.get()) as f64
    });
    let chunk_duration = Memo::new(move |_| {
        Duration::from_millis((1000.0 / refresh_rate.get() as f64) as u64).as_millis_f64()
    });

    let running = RwSignal::new(false);
    let stop = move || {
        running.set(false);
        emu_cfg_ctx.update(|emu_cfg| {
            emu_cfg.logstore.log_info("Emulator stopped", "Emulator stopped".to_string());
        });
    };

    let step_ticks = move |ticks: f64| {
        emu_ctx.update(|emu| {
            if let Err(err) = emu.emu.run_ticks(
                ticks,
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
        })
    };

    let set_frequency = move |val: Option<usize>| {
        emu_cfg_ctx.update(|emu_cfg| {
            emu_cfg.control.real_frequency.set(val);
        });
    };

    let start = move || {
        if running.get() {
            return;
        }
        emu_cfg_ctx.update(|emu_cfg| {
            emu_cfg.logstore.log_info("Emulator started", "Emulator started".to_string());
        });
        running.set(true);
        let now = Date::now();
        step_fn(
            1, // step_count
            chunk_ticks.clone(),
            chunk_duration.clone(),
            step_ticks,
            set_frequency,
            running.clone(),
            0.0, // total_ticks
            now, // start_time
            0.0, // tick_accum
        );
    };

    let switch = move || {
        if running.get() {
            stop();
        } else {
            start();
        }
    };
    // Effect::watch(
    //     // Dependency getter: takes 0 arguments, returns tuple of dependencies
    //     move || (cpu_frequency.get(), refresh_rate.get(),running.get()),
    //     // Effect closure: takes (new_value, prev_value, context)
    //     move |(_cpu_freq, _refresh,is_running), _prev, _ctx| {
    //         if *is_running {
    //             running.set(false);
    //             set_timeout(
    //                 move || {
    //                     if !running.get() {
    //                         running.set(true);
    //                         let now = Date::now();
    //                         step_fn(
    //                             1,
    //                             chunk_ticks.clone(),
    //                             chunk_duration.clone(),
    //                             step_ticks.clone(),
    //                             set_frequency.clone(),
    //                             running.clone(),
    //                             0.0,
    //                             now,
    //                             0.0,
    //                         );
    //                     }
    //                 },
    //                 Duration::from_millis(0),
    //             );
    //         }
    //     },
    //     true,
    // );
    view! {
        <input
            type="button"
            value="Run"
            class=move || {
                classes!(
                    "button",running.with(|running|if *running==true {emu_style::activeinput} else {""})
                )
            }
            on:click=move |_| switch()
        />
    }
}

#[derive(Clone,Copy)]
enum FrequencyMultiplier {
    MHz,
    KHz,
    Hz,
}

impl FrequencyMultiplier {
    fn to_hz(&self, value: usize) -> usize {
        match self {
            FrequencyMultiplier::MHz => value * 1_000_000,
            FrequencyMultiplier::KHz => value * 1_000,
            FrequencyMultiplier::Hz => value,
        }
    }

    fn from_hz(&self, value: usize) -> usize {
        match self {
            FrequencyMultiplier::MHz => value / 1_000_000,
            FrequencyMultiplier::KHz => value / 1_000,
            FrequencyMultiplier::Hz => value,
        }
    }
}

#[island]
fn FrequencySelect() -> impl IntoView {
    let emu_cfg_ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    let multiplier = RwSignal::new(FrequencyMultiplier::MHz);
    let frequency = Memo::new(move |_| emu_cfg_ctx.with(|emu_cfg| multiplier.with(|mul|mul.from_hz(emu_cfg.control.target_frequency.get()))));
    let set_frequency = move |val| emu_cfg_ctx.update(|emu_cfg|emu_cfg.control.target_frequency.set(multiplier.with(|mul|mul.to_hz(val))));
    view! {
        <div class=emu_style::frequency>
            <input
            type="number"
            prop:value=frequency
            on:input=move |ev| {
                    let value = event_target_value(&ev);
                    if let Ok(val) = value.parse::<usize>() {
                        set_frequency(val);
                    } else {
                        emu_cfg_ctx.update(|emu_cfg| {
                            emu_cfg.logstore.log_error(
                                "Invalid frequency",
                                format!("Invalid frequency value: {}", value),
                            );
                        });
                    }
            }
            />
            <select
                on:change=move |ev| {
                        let value = event_target_value(&ev);
                        match value.as_str() {
                            "MHz" => multiplier.set(FrequencyMultiplier::MHz),
                            "KHz" => multiplier.set(FrequencyMultiplier::KHz),
                            "Hz" => multiplier.set(FrequencyMultiplier::Hz),
                            _ => log!("Unknown frequency multiplier: {}", value),
                        }
                }
            >
                <option value="MHz">MHz</option>
                <option value="KHz">KHz</option>
                <option value="Hz">Hz</option>
            </select>
        </div>
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
            <FrequencySelect />
            <HaltButton />
            <ResetButton />
            <ClearMemoryButton />
            <SaveButton />
            <LoadButton />
            <EmuLog />
        </div>
    }
}
