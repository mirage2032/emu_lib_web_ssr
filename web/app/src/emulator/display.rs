use std::ops::Deref;
use crate::emulator::EmulatorCfgContext;
use emu_lib::memory::errors::{MemoryRWCommonError, MemoryReadError, MemoryWriteError};
use emu_lib::memory::MemoryDevice;
use leptos::html::Canvas;
use leptos::logging::log;
use super::emu_style;
use leptos::prelude::*;
use leptos::wasm_bindgen::{Clamped, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

#[derive(Clone,Copy, Debug, PartialEq, Eq, Hash)]
pub struct U8Pixel(pub u8);

impl U8Pixel {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        // r 3 bits, g 3 bits, b 2 bits
        let pixel_value = (r & 0b11100000)
            | ((g & 0b11100000) >> 3)
            | (b & 0b11000000 >> 6);
        U8Pixel(pixel_value)
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        //topmost bits
        let r = self.0 & 0b11100000; // 3 bits for red
        let g = (self.0 & 0b00011100) << 3; // 3 bits for green
        let b = (self.0 & 0b00000011) << 5; // 2 bits for blue, shifted to fit in 8 bits
        (r, g, b)
    }
}
impl From<u8> for U8Pixel {
    fn from(value: u8) -> Self {
        U8Pixel(value)
    }
}

impl From<U8Pixel> for u8 {
    fn from(value: U8Pixel) -> Self {
        value.0
    }
}

#[derive(Clone, Debug)]
pub struct DisplayData {
    width: usize,
    height: usize,
    pixel_data: Vec<U8Pixel>,
}

impl DisplayData {
    pub fn new(width: usize, height: usize) -> Self {
        let pixel_data = vec![U8Pixel::from(0); width * height];
        DisplayData {
            width,
            height,
            pixel_data,
        }
    }
    pub fn get(&self, index:usize) -> Result<U8Pixel, &'static str> {
        if index < self.pixel_data.len() {
            Ok(self.pixel_data[index])
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn set(&mut self, index: usize, color: U8Pixel) -> Result<(), &'static str> {
        if index < self.pixel_data.len() {
            self.pixel_data[index] = color;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    fn get_index(&self, x: usize, y: usize) -> Result<usize, &'static str> {
        if x < self.width && y < self.height {
            Ok(y * self.width + x)
        } else {
            Err("Coordinates out of bounds")
        }
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, color: U8Pixel) -> Result<(), &'static str> {
        let index = self.get_index(x, y)?;
        self.set(index, color)
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Result<U8Pixel, &'static str> {
        let index = self.get_index(x, y)?;
        self.get(index)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn len(&self) -> usize {
        self.pixel_data.len()
    }
}

#[derive(Clone,Copy, Debug)]
pub struct DisplayMemoryDevice{
    pub display: RwSignal<DisplayData>,
}

impl DisplayMemoryDevice {
    pub fn new(width: usize, height: usize) -> Self {
        let display_data = DisplayData::new(width, height);
        DisplayMemoryDevice {
            display: RwSignal::new(display_data),
        }
    }

    pub fn new_with_display(display_data: DisplayData) -> Self {
        DisplayMemoryDevice {
            display: RwSignal::new(display_data),
        }
    }
}

impl MemoryDevice for DisplayMemoryDevice {
    fn size(&self) -> usize {
        self.display.with_untracked(|data| data.len()) //FIXME: not sure if it's ok to be untracked here
    }
    fn read_8(&self, addr: u16) -> Result<u8, MemoryReadError> {
        self.display.with(|data| {
            data.get(addr as usize)
                .map(|pixel| pixel.0)
                .map_err(|_| MemoryRWCommonError::OutOfBounds(addr).into())
        })
    }

    fn write_8(&mut self, addr: u16, value: u8) -> Result<(), MemoryWriteError> {
        let mut result: Result<(), MemoryWriteError> = Err(MemoryRWCommonError::OutOfBounds(addr).into());
        self.display.update(|data| {
            if let Ok(()) = data.set(addr as usize, U8Pixel::from(value)){
                result = Ok(());
            };
        });
        result
    }

    fn write_8_force(&mut self, addr: u16, data: u8) -> Result<(), MemoryWriteError> {
        self.write_8(addr, data)
    }
}

#[island]
fn DisplayData() -> impl IntoView {
    let emu_cfg_ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    let width = move || emu_cfg_ctx.with(|cfg|cfg.display.display.with(|dsp|dsp.width()));
    let height = move || emu_cfg_ctx.with(|cfg|cfg.display.display.with(|dsp|dsp.height()));
    view! {
        <div class=emu_style::displaydata>
            <div>Width: {width}</div>
            <div>Height: {height}</div>
        </div>
    }
}

#[island]
pub fn Display() -> impl IntoView {
    let canvas_ref:NodeRef<Canvas> = NodeRef::new();
    let emu_cfg_ctx = expect_context::<RwSignal<EmulatorCfgContext>>();
    let draw = move |dsp: &DisplayData| {
        if let Some(canvas) = canvas_ref.get_untracked() {
            let html_canvas = canvas
                .dyn_ref::<HtmlCanvasElement>()
                .expect("Canvas element not found")
                .clone();

            let buf_width = dsp.width() as u32;
            let buf_height = dsp.height() as u32;

            let ctx = canvas
                .get_context("2d")
                .expect("should not error")
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .expect("context should be 2d");

            // Create ImageData at original buffer size
            let pixel_bytes: Vec<u8> = dsp
                .pixel_data
                .iter()
                .map(|p| p.to_rgb())
                .flat_map(|(r, g, b)| vec![r, g, b, 255])
                .collect();

            let image_data = ImageData::new_with_u8_clamped_array_and_sh(
                Clamped(&pixel_bytes),
                buf_width,
                buf_height,
            )
            .expect("should create image data");

            // Create an offscreen canvas to hold the image data at original size
            let offscreen_canvas = document()
                .create_element("canvas")
                .unwrap()
                .dyn_into::<HtmlCanvasElement>()
                .unwrap();

            offscreen_canvas.set_width(buf_width);
            offscreen_canvas.set_height(buf_height);
            let offscreen_ctx = offscreen_canvas
                .get_context("2d")
                .expect("offscreen ctx")
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .expect("offscreen ctx2d");

            offscreen_ctx
                .put_image_data(&image_data, 0.0, 0.0)
                .expect("put image data on offscreen");

            // Now clear the main canvas and draw the offscreen canvas scaled to fit main canvas
            ctx.clear_rect(0.0, 0.0, html_canvas.width() as f64, html_canvas.height() as f64);

            ctx
                .draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &offscreen_canvas,
                    0.0,
                    0.0,
                    buf_width as f64,
                    buf_height as f64,
                    0.0,
                    0.0,
                    html_canvas.width() as f64,
                    html_canvas.height() as f64,
                )
                .expect("draw scaled image");
        }
    };
    Effect::watch(
        move || emu_cfg_ctx.with_untracked(|cfg| cfg.display.display.get()),
        move |dd,prev_dd,_| {
        draw(dd);
    },true
    );
    view! {
        <div class=emu_style::display>
            <div class=emu_style::sectop>
                <span>Display</span>
            </div>
            <div class=emu_style::secmid>
                // <DisplayData />
                <div class=emu_style::canvascontainer>
                    <canvas node_ref=canvas_ref></canvas>
                </div>
            </div>
        </div>
    }
}
