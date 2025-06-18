mod account;
mod control;
mod disassembler;
mod editor;
mod info;
mod memory;
mod registers;
mod display;


use crate::emulator::account::Account;
use crate::emulator::display::Display;
use crate::emulator::disassembler::DisassemblerContext;
use crate::emulator::editor::{Editor, EditorContext};
use crate::emulator::memory::MemoryContext;
use crate::emulator::registers::Registers;
use crate::utils::logger::LogStore;
use control::Control;
use disassembler::Disassembler;
use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use info::Info;
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use memory::Memory;
use crate::emulator::control::ControlContext;
use crate::emulator::display::DisplayMemoryDevice;

stylance::import_style!(emu_style, "./emulator.module.scss");

fn build_z80_emu(display: DisplayMemoryDevice) -> Emulator<Z80> {
    use emu_lib::memory;
    use emu_lib::memory::MemoryDevice;
    let mut memory = memory::Memory::new();
    let initial_ram_size = 0x4000;
    let initial_ram = memory::memdevices::RAM::new(initial_ram_size);
    let post_ram = memory::memdevices::RAM::new(0x10000 - initial_ram_size - display.size());
    memory.add_device(Box::new(initial_ram));
    memory.add_device(Box::new(display));
    memory.add_device(Box::new(post_ram));
    let mut emu = Emulator::<Z80>::new_w_mem(memory);
    emu.memory.record_changes(true);
    emu
}

pub struct EmulatorContext {
    pub emu: Emulator<Z80>,
}

impl EmulatorContext {
    fn new(display: DisplayMemoryDevice) -> Self {
        EmulatorContext { emu: build_z80_emu(display) }
    }
}

pub struct EmulatorCfgContext {
    pub mem_config: MemoryContext,
    pub disasm_config: DisassemblerContext,
    pub logstore: LogStore,
    pub editor: EditorContext,
    pub display: DisplayMemoryDevice,
    pub control: ControlContext,
}

impl EmulatorCfgContext {
    fn new(display: DisplayMemoryDevice) -> Self {
        EmulatorCfgContext {
            mem_config: MemoryContext::default(),
            disasm_config: DisassemblerContext::default(),
            logstore: LogStore::default(),
            editor: EditorContext::default(),
            display,
            control: ControlContext::default(),
        }
    }
}

#[island]
pub fn EmulatorNoTitle() -> impl IntoView {
    view! {
        <div>
            <Memory />
            <div class=emu_style::disasmregsinfoflex>
                <Disassembler />
                <div class=emu_style::regsinfo>
                    <Registers />
                    <Info />
                </div>
            </div>
        </div>
    }
}
#[island]
pub fn EmulatorInner() -> impl IntoView {
    if use_context::<RwSignal<EmulatorCfgContext>>().is_none() {
        let cfg = EmulatorCfgContext::new(DisplayMemoryDevice::new(0, 0));
        provide_context(RwSignal::new(cfg));
    }
    if use_context::<RwSignal<EmulatorContext>>().is_none() {
        let display = DisplayMemoryDevice::new(192, 128);
        provide_context(RwSignal::new(EmulatorContext::new(display)));
        let cfg = expect_context::<RwSignal<EmulatorCfgContext>>();
        cfg.update(|cfg| {
            cfg.logstore.log_info(
                "Emulator initialized",
                "Emulator initialized with default settings".to_string(),
            );
            cfg.display = display;
        })
    }
    view! {
        <div class=emu_style::emumain>
        <Control />
        <div class=emu_style::emulator>
            <EmulatorNoTitle />
            <div class=emu_style::midsection>
                <Display />
                <Editor />
            </div>
            <Account />
        </div>
        </div>
    }
}

#[component]
pub fn Emulator() -> impl IntoView {
    view! {
        <Meta name="og:title" content="Emulator" />
        <Title text="Emulator" />
        <EmulatorInner />
    }
}
