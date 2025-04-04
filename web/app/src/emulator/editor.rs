use leptos::ev::{Event, Targeted};
use leptos::prelude::*;
use leptos::web_sys;
use leptos::web_sys::{HtmlInputElement, HtmlTextAreaElement};
use serde::{Deserialize, Serialize};
use super::{emu_style, EmulatorCfgContext, EmulatorContext};
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum CompileLanguage {
    ASM,
    C,
}
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct EditorContext {
    pub active_lang: CompileLanguage,
    pub c_buffer: String,
    pub asm_buffer: String,
}

impl Default for EditorContext {
    fn default() -> Self {
        EditorContext {
            active_lang: CompileLanguage::ASM,
            c_buffer: String::new(),
            asm_buffer: String::new(),
        }
    }
}

impl EditorContext {
    pub fn write_buffer(&mut self, lang: CompileLanguage, buffer: String) {
        match lang {
            CompileLanguage::ASM => self.asm_buffer = buffer,
            CompileLanguage::C => self.c_buffer = buffer,
        }
    }
}
#[island]
pub fn EditorText(lang:CompileLanguage) -> impl IntoView{
    let emu_ctx_signal = expect_context::<RwSignal<EmulatorCfgContext>>();
    let set_buffer =move |ev:Targeted<Event,HtmlTextAreaElement>| {
        emu_ctx_signal.update(|emu_ctx| {
            emu_ctx.editor.write_buffer(lang, ev.target().value());
        });
    };
    view! { <textarea on:input:target=set_buffer></textarea> }
}

#[island]
pub fn EditorTextAreas() -> impl IntoView {
    let emu_ctx_signal = expect_context::<RwSignal<EmulatorCfgContext>>();
    let is_current_lang =move |lang:CompileLanguage| {
        emu_ctx_signal.with(|emu_ctx| emu_ctx.editor.active_lang==lang)
    };
    view! {
        <div class=emu_style::editorta>
            <Show
                when=move || is_current_lang(CompileLanguage::ASM)
                fallback=move || { "".to_string() }
            >
                <EditorText lang=CompileLanguage::ASM />
            </Show>
            <Show
                when=move || is_current_lang(CompileLanguage::C)
                fallback=move || { "".to_string() }
            >
                <EditorText lang=CompileLanguage::C />
            </Show>
        </div>
    }
}

#[island]
pub fn EditorTop() -> impl IntoView{
    view!{
        <div class=emu_style::editortop>
        TOP
        </div>
    }
}
#[island]
pub fn Editor() -> impl IntoView {
    view! {
        <div class=emu_style::editor>
            <div class=emu_style::sectop>
                <span>Editor</span>
            </div>
            <EditorTop />
            <EditorTextAreas />
        </div>
    }
}