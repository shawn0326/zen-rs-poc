use std::rc::Rc;

use zen_rs_poc::{
    graphics::{Geometry, Material, Primitive},
    render::{RenderTarget, ScreenSurfaceLike},
    scene::{Object3D, Scene},
    wgpu::Renderer,
};

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("Basic Scene Example");
        self.window = Some(event_loop.create_window(window_attributes).unwrap());

        println!("Resumed");
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
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
        let mut position = obj.position.get();
        position.x = 10.0;
        obj.position.set(position);
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

    struct DummySurface;
    impl ScreenSurfaceLike for DummySurface {
        fn get_size(&self) -> (u32, u32) {
            (800, 600)
        }
    }
    let screen_render_target = RenderTarget::screen(Box::new(DummySurface), 300, 300);
    println!("Screen RenderTarget: {:?}", screen_render_target);

    let renderer = Renderer::new();
    renderer.init();

    let mut app = App::default();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run_app(&mut app)
        .expect("Failed to run event loop");
}
