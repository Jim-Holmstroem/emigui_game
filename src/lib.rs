extern crate serde_json;
extern crate wasm_bindgen;

extern crate emigui;
extern crate emigui_wasm;

use wasm_bindgen::prelude::*;

use {
    emigui::{
        Emigui,
        RawInput,
        label,
        color::{
            srgba,
        },
        widgets::{
            Label,
        },
        types::{
            PaintCmd,
            Outline,
        },
        math::{
            Vec2,
        },
    },
};

#[wasm_bindgen]
pub struct State {
    emigui: Emigui,
    webgl_painter: emigui_wasm::webgl::Painter,
}

impl State {
    fn new(canvas_id: &str, pixels_per_point: f32) -> Result<State, JsValue> {
        Ok(State {
            emigui: Emigui::new(pixels_per_point),
            webgl_painter: emigui_wasm::webgl::Painter::new(canvas_id)?,
        })
    }

    fn run(&mut self, raw_input: RawInput) -> Result<(), JsValue> {
        self.emigui.new_frame(raw_input);

        let mut region = self.emigui.whole_screen_region();
        let mut region = region.centered_column(800.0);

        region.add(label!("{:?}", region.input()));

        let white = srgba(255, 255, 255, 255);
        let width = 3.0;

        if let Some(position) = region.input().mouse_pos {
            region.add_paint_cmd(
                PaintCmd::Circle {
                    center: position,
                    fill_color: None,
                    outline: Some(Outline { color: white, width }),
                    radius: 32.0,
                }
            );
            region.add_paint_cmd(
                PaintCmd::Line {
                    points: vec![Vec2 { x: 0.0, y: 0.0 }, position],
                    color: white,
                    width,
                }
            );
        }

        let bg_color = srgba(0, 0, 0, 255);
        let mesh = self.emigui.paint();
        let result = self.webgl_painter.paint(
            bg_color,
            mesh,
            self.emigui.texture(),
            raw_input.pixels_per_point,
        );

        result
    }
}

#[wasm_bindgen]
pub fn new_webgl_gui(canvas_id: &str, pixels_per_point: f32) -> Result<State, JsValue> {
    State::new(canvas_id, pixels_per_point)
}

#[wasm_bindgen]
pub fn run_gui(state: &mut State, raw_input_json: &str) -> Result<(), JsValue> {
    state.run(
        serde_json::from_str(
            raw_input_json,
        ).unwrap()
    )
}
