use std::rc::Rc;

use pollster::block_on;
use zen_rs_poc::{
    graphics::{Geometry, Material, Primitive},
    math::Vector3,
    render::{RenderTarget, ScreenSurfaceLike},
    scene::{Object3D, Scene},
    wgpu::Renderer,
};

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

struct App {
    window: Window,
    renderer: Option<Renderer>,
}

impl App {
    fn new(window: Window) -> Self {
        println!("App created with window ID: {:?}", window.id());
        Self {
            window,
            renderer: None,
        }
    }

    async fn init(&mut self) {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(&self.window).unwrap();
        let size = self.window.inner_size();

        struct DummySurface;
        impl ScreenSurfaceLike for DummySurface {
            fn get_size(&self) -> (u32, u32) {
                (800, 600)
            }
        }
        let screen_render_target = RenderTarget::screen(Box::new(DummySurface), 300, 300);
        println!("Screen RenderTarget: {:?}", screen_render_target);

        let renderer = Renderer::new();
        renderer
            .init(&instance, &surface, size.width, size.height)
            .await;

        self.renderer = Some(renderer);
    }

    pub fn set_window_resized(&self, new_size: winit::dpi::PhysicalSize<u32>) {
        println!("Window resized to: {:?}", new_size);
    }
}

#[derive(Default)]
struct AppHandler {
    app: Option<App>,
}

impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("Basic Scene Example");
        let window = event_loop.create_window(window_attributes).unwrap();

        let mut app = App::new(window);

        let _ = block_on(app.init());

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
                if let Some(_app) = &self.app {
                    // Render here
                }
            }
            _ => (),
        }
    }
}

fn main() {
    let scene = Scene::new();

    {
        let geometry = Geometry::new();
        let material = Material::new();
        let primitive = Primitive::new(&geometry, &material);

        println!("Create primitive: {:?}", primitive);

        let obj = Object3D::new();
        obj.position
            .set(obj.position.get() + Vector3::new(1.0, 2.0, 3.0));
        obj.primitives.borrow_mut().push(primitive);

        scene.add(&obj);
    }

    Object3D::traverse(&scene.root, &|o| {
        let mut primitives = o.primitives.borrow_mut();
        println!("Object3D {} has {} primitives", o.name, primitives.len());

        if !primitives.is_empty() {
            let primitive = &primitives[0];
            let geometry = primitive.geometry();
            let material = primitive.material();

            println!("Geometry strong count: {}", Rc::strong_count(&geometry));
            println!("Material strong count: {}", Rc::strong_count(&material));

            primitives.clear();

            println!("Geometry strong count: {}", Rc::strong_count(&geometry));
            println!("Material strong count: {}", Rc::strong_count(&material));
        }

        println!("Object3D {} has {} primitives", o.name, primitives.len());
    });

    scene.update_world_matrix();

    println!(
        "Scene root world matrix: {:?}",
        scene.root.children()[0].world_matrix.get().elements
    );

    let mut app_handler = AppHandler::default();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run_app(&mut app_handler)
        .expect("Failed to run event loop");
}
