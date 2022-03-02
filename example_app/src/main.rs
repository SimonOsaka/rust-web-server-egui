#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use eframe::epi::IconData;
    use example_lib::app::App;

    let app = App::new();
    let icon = image::load_from_memory(include_bytes!("../../docs/icon-256.png"))
        .expect("icon not exist")
        .to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    let native_options = eframe::NativeOptions {
        maximized: true,
        icon_data: Some(IconData {
            rgba: icon.to_vec(),
            width: icon_width,
            height: icon_height,
        }),
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);
}
