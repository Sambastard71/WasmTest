mod utils;

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d};


#[derive(Copy, Clone)]
struct Data {
    x: f64,
    y: f64,
    z: f64,
}

static mut DATA_SET: [Data; 1000] = [Data {
    x: 0.,
    y: 0.,
    z: 0.,
}; 1000];

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();

    // create a new canvas element
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .map_err(|_| ())
    .unwrap();

    canvas.set_width(document.body().unwrap().client_width() as u32);
    canvas.set_height(document.body().unwrap().client_height() as u32);
    // get the 2D rendering context for painting on the canvas
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    ctx.set_image_smoothing_enabled(false);

    // paint the canvas black
    ctx.set_fill_style(&"#000".into());
    ctx.fill_rect(0., 0.,f64::from(canvas.width()), f64::from(canvas.height()));

    // paint a white rectangle at pos (100, 100)
    // that is 50 pixels wide and 25 pixels tall
    // ctx.set_global_alpha(0.5);
    // ctx.set_fill_style(&"#ff0".into());
    // ctx.fill_rect(100., 100., 50., 25.);

    unsafe{
        for i in &mut DATA_SET{
            i.x = js_sys::Math::random();
            i.y = js_sys::Math::random();
            i.z = js_sys::Math::random();
        }
    }

    Ok(())
}


#[wasm_bindgen]
pub fn clear_canvas() {
    let window = window().unwrap();
    let document = window.document().unwrap();

    // create a new canvas element
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .map_err(|_| ())
    .unwrap();

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    // paint the canvas black
    ctx.set_global_alpha(1.);
    ctx.set_fill_style(&"#000".into());
    ctx.fill_rect(0., 0., f64::from(canvas.width()), f64::from(canvas.height()));

}

pub fn draw_rect_canvas(x: f64, y: f64, z: f64, r: f64, g: f64, b: f64, a: f64) {
    let window = window().unwrap();
    let document = window.document().unwrap();

    // create a new canvas element
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .map_err(|_| ())
    .unwrap();

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    // paint a white rectangle at pos (100, 100)
    // that is 50 pixels wide and 25 pixels tall
    ctx.set_global_alpha(z + (a / 255.));

    let rgb =format!("rgba({}, {}, {}, {})", ((255.0 * x) + r).to_string(), ((255.0 * y) + g).to_string(), ((255.0 * z) + b).to_string(), ((255.0 * y)).to_string());
    ctx.set_fill_style(&rgb.into());
    
    console_log!("r {}", ((255.0 * x) + r).to_string());
    console_log!("g {}", ((255.0 * y) + g).to_string());
    console_log!("b {}", ((255.0 * z) + b).to_string());

    ctx.fill_rect(f64::from(canvas.width()- 50) * x, f64::from(canvas.height() - 50) * y, 50. * x, 50. * x);

}

#[wasm_bindgen]
pub fn draw_many_rect(many: u32, r: f64, g: f64, b: f64, a: f64,){
    let window = window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");

    let time_before = performance.now();
    console_log!("the current time before draw (in ms) is {}", time_before);

    unsafe{
        for i in 0..many {
            
            draw_rect_canvas(DATA_SET[i as usize].x, DATA_SET[i as usize].y, DATA_SET[i as usize].z, r, g, b, a);
        }
    }

    let time_after = performance.now();
    console_log!("the current time after draw (in ms) is {}", time_after);
    console_log!("the time spent drawing (in ms) is {}", (time_after - time_before));

    window.document().unwrap().get_element_by_id("time").unwrap().set_text_content(Some(&(time_after-time_before).to_string()));
}