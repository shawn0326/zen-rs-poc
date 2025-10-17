use image::GenericImageView;
use pollster::block_on;
use std::{collections::VecDeque, sync::Arc, time::Instant};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};
use zen_rs_poc::scene::Camera;
use zen_rs_poc::{
    graphics::{Geometry, Material, Primitive, Texture},
    math::Vec3,
    render::{RenderCollector, RenderTarget},
    scene::{Object3D, Scene},
    wgpu::Renderer,
};

struct App<'window> {
    window: Arc<Window>,
    renderer: Renderer<'window>,
    screen_render_target: RenderTarget,
    render_collector: RenderCollector,
    scene: Scene,
    camera: Camera,
    frame_times: VecDeque<Instant>,
}

impl<'window> App<'window> {
    async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let size = window.inner_size();
        let surface = instance.create_surface(window.clone()).unwrap();

        let renderer = Renderer::new(&instance, surface, (size.width, size.height)).await;

        let screen_render_target = RenderTarget::screen(size.width, size.height);
        println!("Screen RenderTarget: {:?}", screen_render_target);

        let render_collector = RenderCollector {};

        let scene = Scene::new();

        let camera = Camera {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vec3::Y,
            aspect: size.width as f32 / size.height as f32,
            fovy: 45.0,
            near: 0.1,
            far: 100.0,
        };

        let diffuse_bytes = include_bytes!("../assets/textures/logo.jpg");
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();

        let texture = Texture::from_data(
            diffuse_image.to_rgba8().into_raw(),
            diffuse_image.dimensions(),
        );

        let geometry = Geometry::create_test_shape();
        let geometry2 = Geometry::create_unit_quad();
        let material = Material::new();
        let material2 = Material::new();
        material2.borrow_mut().set_texture(texture);

        for i in 0..10000 {
            let geom_ref = if i % 2 == 0 { &geometry } else { &geometry2 };
            let mat_ref = if i % 2 == 0 { &material } else { &material2 };
            let primitive = Primitive::new(geom_ref, mat_ref);

            let obj = Object3D::new();
            obj.position
                .set(obj.position.get() + Vec3::new(i as f32, 2.0, 3.0));
            obj.primitives.borrow_mut().push(primitive);

            scene.add(&obj);
        }

        Self {
            window,
            renderer,
            screen_render_target,
            render_collector,
            scene,
            camera,
            frame_times: VecDeque::new(),
        }
    }

    pub fn set_window_resized(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.screen_render_target
            .set_size(new_size.width, new_size.height);

        println!("Screen RenderTarget: {:?}", self.screen_render_target);
    }

    pub fn update_fps_stats(&mut self) {
        let now = Instant::now();

        self.frame_times.push_back(now);

        while self.frame_times.len() > 60 {
            self.frame_times.pop_front();
        }

        if self.frame_times.len() > 1 {
            let oldest = self.frame_times.front().unwrap();
            let elapsed = now.duration_since(*oldest).as_secs_f64();
            let fps = (self.frame_times.len() - 1) as f64 / elapsed;

            if self.frame_times.len() % 10 == 0 {
                self.window
                    .set_title(&format!("Basic Scene Example - FPS: {:.1}", fps));
            }
        }
    }

    pub fn render(&mut self) {
        self.scene.update_world_matrix();
        let render_list = self.render_collector.collect(&self.scene);
        self.renderer
            .render(&render_list, &self.camera, &self.screen_render_target);
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

        app.window.request_redraw();

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

                if let Some(app) = &mut self.app {
                    app.set_window_resized(size);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(app) = &mut self.app {
                    app.update_fps_stats();
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
    let mut app_handler = AppHandler::default();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run_app(&mut app_handler)
        .expect("Failed to run event loop");
}
