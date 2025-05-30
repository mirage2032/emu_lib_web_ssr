use super::{emu_style, EmulatorCfgContext, EmulatorContext};
use crate::utils::ccompiler::{c_compile, c_format, c_syntax_check, CompilerError};
use emu_lib::cpu::instruction::InstructionParser;
use emu_lib::cpu::z80::parser::Z80_PARSER;
use leptos::ev::{Event, Targeted};
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys::HtmlTextAreaElement;
use serde::{Deserialize, Serialize};
use stylance::classes;

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
    view! { <textarea spellcheck=false on:input:target=set_buffer prop:value=get_buffer></textarea> }
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
    let on_compile_c = move || {
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
    let on_compile_asm = move || {
        emu_cfg_ctx.update(|emu_cfg_ctx| {
            let mut compiled: Vec<u8> = vec![];
            for line in emu_cfg_ctx.editor.asm_buffer.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
                if let Ok(instruction) = Z80_PARSER.ins_from_asm_string(line) {
                    compiled.extend(instruction.to_bytes());
                } else {
                    emu_cfg_ctx.logstore.log_error(
                        "ASM Compilation error",
                        format!("ASM Compilation error: Invalid instruction: \"{}\"", line),
                    );
                    return;
                }
            }
            emu_ctx.update(|emu_ctx| {
                if let Err(err) = emu_ctx.emu.memory.load(&compiled, true) {
                    emu_cfg_ctx.logstore.log_error(
                        "ASM Compilation error",
                        format!("ASM Compilation error, writting into emulator memory: {:?}", err),
                    );
                } else {
                    emu_cfg_ctx.logstore.log_info(
                        "ASM Compilation success",
                        "ASM Compilation success, program loaded into emulator memory"
                            .to_string(),
                    );
                }
            });
        })
    };
    let on_compile = move |_| {
        emu_ctx.update(|emu_ctx| {
            emu_ctx.emu.memory.clear_changes();
        });
        let lang = emu_cfg_ctx.with(|emu_cfg_ctx| {emu_cfg_ctx.editor.active_lang});
            match lang {
                CompileLanguage::ASM => on_compile_asm(),
                CompileLanguage::C => on_compile_c(),
            }
        };
    let on_format_c = move |_| {
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
    let on_syntax_check_c = move |_| {
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

    let lang_class = move |lang: CompileLanguage| {
        if emu_cfg_ctx.with(|emu_ctx| emu_ctx.editor.active_lang) == lang {
            classes!(emu_style::imgcontainer, emu_style::imgcontaineractive)
        } else {
            classes!(emu_style::imgcontainer)
        }
    };
    let set_active_lang = move |lang: CompileLanguage| {
        emu_cfg_ctx.update(|emu_ctx| {
            emu_ctx.editor.active_lang = lang;
        });
    };
    view! {
        <div class=emu_style::editortop>
            <div class=emu_style::editortopbtns>
                <button on:click=on_compile>"Compile"</button>
                <button on:click=on_format_c>"Format"</button>
                <button on:click=on_syntax_check_c>"Syntax Check"</button>
            </div>
            <div class=emu_style::editortoplang>
                <div
                    on:click=move |_| set_active_lang(CompileLanguage::ASM)
                    class=move || lang_class(CompileLanguage::ASM)
                >
                    <img
                        width="48"
                        height="48"
                        src="https://img.icons8.com/color/48/assembly.png"
                        alt="assembly"
                    />
                </div>
                <div
                    on:click=move |_| set_active_lang(CompileLanguage::C)
                    class=move || lang_class(CompileLanguage::C)
                >
                    <img
                        width="48"
                        height="48"
                        src="https://img.icons8.com/color/48/c-programming.png"
                        alt="c-programming"
                    />
                </div>
            </div>
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
