use super::{emu_style, EmulatorCfgContext, EmulatorContext};
use crate::utils::ccompiler::{c_compile, c_format, c_syntax_check, CompilerError};
use leptos::ev::{Event, Targeted};
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys::{HtmlInputElement, HtmlTextAreaElement};
use serde::{Deserialize, Serialize};
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
            active_lang: CompileLanguage::C,
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
pub fn EditorText(lang: CompileLanguage) -> impl IntoView {
    let emu_ctx_signal = expect_context::<RwSignal<EmulatorCfgContext>>();
    let set_buffer = move |ev: Targeted<Event, HtmlTextAreaElement>| {
        emu_ctx_signal.update(|emu_ctx| {
            emu_ctx.editor.write_buffer(lang, ev.target().value());
        });
    };
    let get_buffer = move || {
        emu_ctx_signal.with(|emu_ctx| match lang {
            CompileLanguage::ASM => emu_ctx.editor.asm_buffer.clone(),
            CompileLanguage::C => emu_ctx.editor.c_buffer.clone(),
        })
    };
    view! { <textarea on:input:target=set_buffer prop:value=get_buffer></textarea> }
}

#[island]
pub fn EditorTextAreas() -> impl IntoView {
    let emu_ctx_signal = expect_context::<RwSignal<EmulatorCfgContext>>();

    let is_current_lang = move |lang: CompileLanguage| {
        emu_ctx_signal.with(|emu_ctx| emu_ctx.editor.active_lang == lang)
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
pub fn EditorTop() -> impl IntoView {
    let emu_ctx = expect_context::<RwSignal<EmulatorContext>>();
    let emu_cfg_ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    let on_compile = move |_| {
        let code = emu_cfg_ctx.with(|emu_ctx| emu_ctx.editor.c_buffer.clone());
        spawn_local(async move {
            let res = c_compile(code).await;
            match res {
                Ok(res) => {
                    if res.rc != 0 {
                        emu_cfg_ctx.update(|emu_cfg_ctx| {
                            emu_cfg_ctx.logstore.log_error(
                                "C Compilation error",
                                format!("C Compilation error: {}", res.stderr),
                            );
                        });
                        return;
                    }
                    emu_ctx.update(|emu_ctx| {
                        if let Err(err) = emu_ctx.emu.memory.load(&res.data, true) {
                            emu_cfg_ctx.update(|emu_cfg_ctx| {
                                emu_cfg_ctx.logstore.log_error(
                                    "C Compilation error",
                                    format!(
                                        "C Compilation error, writting into emulator memory: {:?}",
                                        err
                                    ),
                                );
                            });
                        } else {
                            emu_cfg_ctx.update(|emu_cfg_ctx| {
                                emu_cfg_ctx.logstore.log_info(
                                    "C Compilation success",
                                    "C Compilation success, program loaded into emulator memory"
                                        .to_string(),
                                );
                            });
                        }
                    });
                }
                Err(CompilerError::Unauthorized) => {
                    emu_cfg_ctx.update(|emu_cfg_ctx| {
                        emu_cfg_ctx.logstore.log_error(
                            "Unauthenticated",
                            "C Compilation error: Unauthorized".to_string(),
                        );
                    });
                }
                Err(err) => {
                    emu_cfg_ctx.update(|emu_cfg_ctx| {
                        emu_cfg_ctx.logstore.log_error(
                            "C Compilation error",
                            format!("C Compilation error: {:?}", err),
                        );
                    });
                }
            }
        });
    };
    let on_format = move |_| {
        let code = emu_cfg_ctx.with(|emu_ctx| emu_ctx.editor.c_buffer.clone());
        spawn_local(async move {
            let res = c_format(code).await;
            match res {
                Ok(res) => {
                    log!("Formatted C code: {:?}", res);
                    emu_cfg_ctx.update(|emu_ctx| {
                        emu_ctx
                            .editor
                            .write_buffer(CompileLanguage::C, res.data.clone());
                        emu_ctx.logstore.log_info(
                            "Formatted C code",
                            format!("Formatted C code: {}", res.data),
                        );
                    });
                }
                Err(CompilerError::Unauthorized) => {
                    emu_cfg_ctx.update(|emu_ctx| {
                        emu_ctx.logstore.log_error(
                            "Unauthenticated",
                            "Format C error: Unauthorized".to_string(),
                        );
                    });
                }
                Err(err) => {
                    emu_cfg_ctx.update(|emu_ctx| {
                        emu_ctx
                            .logstore
                            .log_error("Format C error", format!("Format C error: {:?}", err));
                    });
                }
            }
        });
    };
    let on_syntax_check = move |_| {
        let code = emu_cfg_ctx.with(|emu_ctx| emu_ctx.editor.c_buffer.clone());
        spawn_local(async move {
            let res = c_syntax_check(code).await;
            match res {
                Ok(res) => {
                    if res.stderr.is_empty() {
                        emu_cfg_ctx.update(|emu_ctx| {
                            emu_ctx.logstore.log_info(
                                "No C syntax error",
                                "C Syntax check: no errors".to_string(),
                            );
                        });
                    } else {
                        emu_cfg_ctx.update(|emu_ctx| {
                            emu_ctx.logstore.log_error(
                                "C Syntax errors",
                                format!("C Syntax errors: {}", res.stderr),
                            );
                        });
                    }
                }
                Err(CompilerError::Unauthorized) => {
                    emu_cfg_ctx.update(|emu_ctx| {
                        emu_ctx.logstore.log_error(
                            "Unauthenticated",
                            "C Syntax check error: Unauthorized".to_string(),
                        );
                    });
                }
                Err(err) => {
                    emu_cfg_ctx.update(|emu_ctx| {
                        emu_ctx.logstore.log_error(
                            "C Syntax errors",
                            format!("C Syntax check error: {:?}", err),
                        );
                    });
                }
            }
        });
    };
    view! {
        <div class=emu_style::editortop>
            <button on:click=on_compile>"Compile"</button>
            <button on:click=on_format>"Format"</button>
            <button on:click=on_syntax_check>"Syntax Check"</button>
        </div>
    }
}
#[island]
pub fn Editor() -> impl IntoView {
    view! {
        <div class=emu_style::editor>
            <div class=emu_style::sectop>
                <span>Assembler</span>
            </div>
            <EditorTop />
            <EditorTextAreas />
        </div>
    }
}
