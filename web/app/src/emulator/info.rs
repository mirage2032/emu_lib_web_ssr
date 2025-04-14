use super::{emu_style, EmulatorContext};
use leptos::prelude::*;

#[island]
pub fn InfoCounters() -> impl IntoView {
    let emu = expect_context::<RwSignal<EmulatorContext>>();
    let cycles = Memo::new(move |_| emu.with(|emu| emu.emu.cycles));
    let instructions = Memo::new(move |_| emu.with(|emu| emu.emu.instructions));
    view! {
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
