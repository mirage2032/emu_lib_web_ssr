use super::{emu_style, EmulatorCfgContext, EmulatorContext};
use leptos::prelude::*;

#[island]
pub fn InfoCounters() -> impl IntoView {
    let emu = expect_context::<RwSignal<EmulatorContext>>();
    let cycles = Memo::new(move |_| emu.with(|emu| emu.emu.cycles));
    let instructions = Memo::new(move |_| emu.with(|emu| emu.emu.instructions));
    let emu_cfg = expect_context::<RwSignal<EmulatorCfgContext>>();
    let real_frequency = move || {
        emu_cfg.with(|emu_cfg| {
            if let Some(freq) = emu_cfg.control.real_frequency.get() {
                //format with commas
                let formatted_freq = freq.to_string();
                formatted_freq.chars().rev().enumerate().map(|(i, c)| {
                    if i > 0 && i % 3 == 0 {
                        format!(",{}", c)
                    } else {
                        c.to_string()
                    }
                }).collect::<String>().chars().rev().collect::<String>()
            } else {
                "N/A".to_string()
            }
        })
    };
    view! {
        <div class=emu_style::outerinfo>
        <Show when=move || emu_cfg.with(|emu_cfg| emu_cfg.control.real_frequency.get().is_some())>
            <div class=emu_style::frequencyinfo>Frequency: {move || real_frequency()} Hz</div>
        </Show>
        <div class=emu_style::infocounters>
            <div class=emu_style::counters>
                <div>
                    <div>
                        <span>Cycles</span>
                    </div>
                    <div>
                        <span>{cycles}</span>
                    </div>
                </div>
                <div>
                    <div>
                        <span>Instructions</span>
                    </div>
                    <div>
                        <span>{instructions}</span>
                    </div>
                </div>
            </div>
            <div class=emu_style::resetbutton>
                <input
                    type="button"
                    value="Reset"
                    on:click=move |_| emu.update(|emu| emu.emu.reset_counters())
                />
            </div>
        </div>
        </div>
    }
}
#[component]
pub fn Info() -> impl IntoView {
    view! {
        <div class=emu_style::emuinfo>
            <div class=emu_style::sectop>
                <span>Info</span>
            </div>
            <InfoCounters />
        </div>
    }
}
