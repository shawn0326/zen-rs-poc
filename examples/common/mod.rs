#[cfg(not(target_arch = "wasm32"))]
mod fps_counter;
#[cfg(not(target_arch = "wasm32"))]
pub use fps_counter::FpsCounter;

mod app;
pub use app::App;
