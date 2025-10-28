#[path = "common/mod.rs"]
mod common;
use common::App;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowAttributesExtWebSys;

#[cfg(not(target_arch = "wasm32"))]
use pollster::block_on;

struct AppHandler {
    app: Option<App<'static>>,
    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<App<'static>>>,
    #[cfg(not(target_arch = "wasm32"))]
    fps_counter: common::FpsCounter,
}

impl ApplicationHandler<App<'static>> for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[cfg(target_arch = "wasm32")]
        {
            let window = wgpu::web_sys::window().unwrap_throw();
            let document = window.document().unwrap_throw();
            let canvas = document.create_element("canvas").unwrap_throw();
            canvas.set_attribute("style", "border: 0; margin: 0; padding: 0; display: block; width: 100vw; height: 100vh;").unwrap_throw();
            let html_canvas_element: wgpu::web_sys::HtmlCanvasElement = canvas.unchecked_into();
            html_canvas_element.set_id("wgpu-canvas");
            let body = document.body().unwrap_throw();
            body.append_child(&html_canvas_element).unwrap_throw();

            let window_attributes =
                Window::default_attributes().with_canvas(Some(html_canvas_element));

            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

            if let Some(proxy) = self.proxy.take() {
                wasm_bindgen_futures::spawn_local(async move {
                    log::debug!("Window size: {:?}", window.inner_size());
                    let _ = proxy.send_event(App::new_benchmark(window, 50000).await);
                });
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let window_attributes = Window::default_attributes().with_title("Basic Scene Example");
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

            let app = block_on(App::new_benchmark(window, 50000));
            app.window.request_redraw();
            self.app = Some(app);
        }
    }

    #[allow(unused_mut, unused_variables)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: App<'static>) {
        // This is where proxy.send_event() ends up
        #[cfg(target_arch = "wasm32")]
        {
            event.window.request_redraw();
            self.app = Some(event);
            log::debug!("App initialized in wasm32");
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if size.width == 0 || size.height == 0 {
                    return;
                }

                if let Some(app) = &mut self.app {
                    log::debug!("Size changed: {:?}", size);
                    app.set_window_resized(size);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(app) = &mut self.app {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let fps = self.fps_counter.tick();
                        app.window
                            .set_title(&format!("Basic Scene Example - FPS: {:.1}", fps));
                    }

                    app.render();
                    app.window.request_redraw();
                }
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if let Some(app) = &mut self.app {
                    let is_pressed = event.state == ElementState::Pressed;

                    if !is_pressed {
                        return;
                    }

                    match event.physical_key {
                        PhysicalKey::Code(KeyCode::KeyW) | PhysicalKey::Code(KeyCode::ArrowUp) => {
                            app.camera.eye.z -= 0.1;
                        }
                        PhysicalKey::Code(KeyCode::KeyA)
                        | PhysicalKey::Code(KeyCode::ArrowLeft) => {
                            app.camera.eye.x -= 0.1;
                        }
                        PhysicalKey::Code(KeyCode::KeyS)
                        | PhysicalKey::Code(KeyCode::ArrowDown) => {
                            app.camera.eye.z += 0.1;
                        }
                        PhysicalKey::Code(KeyCode::KeyD)
                        | PhysicalKey::Code(KeyCode::ArrowRight) => {
                            app.camera.eye.x += 0.1;
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut app_handler = AppHandler {
            app: None,
            fps_counter: common::FpsCounter::default(),
        };

        let event_loop = EventLoop::with_user_event().build().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop
            .run_app(&mut app_handler)
            .expect("Failed to run event loop");
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async move {
        console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize logger");

        let event_loop = EventLoop::with_user_event().build().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app_handler = AppHandler {
            app: None,
            proxy: Some(event_loop.create_proxy()),
        };

        event_loop.run_app(&mut app_handler).unwrap();
    });
}
