use std::{cell::RefCell, sync::Arc};

use pollster::block_on;
use zen_rs_poc::{
    graphics::{Geometry, Material, Primitive},
    math::Vector3,
    render::{RenderCollector, RenderTarget},
    scene::{Object3D, Scene},
    wgpu::Renderer,
};

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

struct App<'window> {
    // window: Arc<Window>,
    renderer: Renderer<'window>,
    screen_render_target: RefCell<RenderTarget>,
    render_collector: RenderCollector,
    scene: Scene,
}

impl<'window> App<'window> {
    async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let renderer = Renderer::new(window, size.width, size.height).await;

        let screen_render_target = RenderTarget::screen(300, 300);
        println!("Screen RenderTarget: {:?}", screen_render_target);

        let render_collector = RenderCollector {};

        let scene = Scene::new();

        for i in 0..10 {
            let geometry = Geometry::new();
            let material = Material::new();
            let primitive = Primitive::new(&geometry, &material);

            let obj = Object3D::new();
            obj.position
                .set(obj.position.get() + Vector3::new(i as f32, 2.0, 3.0));
            obj.primitives.borrow_mut().push(primitive);

            scene.add(&obj);
        }

        Self {
            // window,
            renderer,
            screen_render_target: RefCell::new(screen_render_target),
            render_collector,
            scene,
        }
    }

    pub fn set_window_resized(&self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.screen_render_target
            .borrow_mut()
            .set_size(new_size.width, new_size.height);

        println!(
            "Screen RenderTarget: {:?}",
            self.screen_render_target.borrow()
        );
    }

    pub fn render(&self) {
        self.scene.update_world_matrix();
        let render_list = self.render_collector.collect(&self.scene);
        self.renderer
            .render(&render_list, &self.screen_render_target.borrow());
    }
}

#[derive(Default)]
struct AppHandler {
    app: Option<App<'static>>,
}

impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("Basic Scene Example");
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let app = block_on(App::new(window));

        self.app = Some(app);

        println!("Resumed");
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if size.width == 0 || size.height == 0 {
                    return;
                }

                if let Some(app) = &self.app {
                    app.set_window_resized(size);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(app) = &self.app {
                    app.render();
                }
            }
            _ => (),
        }
    }
}

fn main() {
    let mut app_handler = AppHandler::default();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run_app(&mut app_handler)
        .expect("Failed to run event loop");
}
