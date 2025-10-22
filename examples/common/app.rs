use image::GenericImageView;
use rand::Rng;
use std::sync::Arc;
use winit::window::Window;
use zen_rs_poc::scene::Camera;
use zen_rs_poc::{
    graphics::{Geometry, Material, Primitive, Texture},
    math::Vec3,
    render::{RenderCollector, RenderTarget},
    scene::{Object3D, Scene},
    wgpu::Renderer,
};

pub struct App<'window> {
    pub window: Arc<Window>,
    renderer: Renderer<'window>,
    screen_render_target: RenderTarget,
    render_collector: RenderCollector,
    pub scene: Scene,
    pub camera: Camera,
}

impl<'window> App<'window> {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: if cfg!(target_arch = "wasm32") {
                wgpu::Backends::BROWSER_WEBGPU
            } else {
                wgpu::Backends::all()
            },
            ..Default::default()
        });

        let mut size = window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);

        let surface = instance.create_surface(window.clone()).unwrap();

        let renderer = Renderer::new(&instance, surface, (size.width, size.height)).await;

        let screen_render_target = RenderTarget::screen(size.width, size.height);

        let render_collector = RenderCollector {};

        let scene = Scene::new();

        let camera = Camera {
            eye: (0.0, 0.0, 5.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vec3::Y,
            aspect: size.width as f32 / size.height as f32,
            fovy: 45.0,
            near: 0.1,
            far: 100.0,
        };

        Self {
            window,
            renderer,
            screen_render_target,
            render_collector,
            scene,
            camera,
        }
    }

    pub async fn new_benchmark(window: Arc<Window>, count: u32) -> Self {
        let app = Self::new(window).await;

        let diffuse_bytes = include_bytes!("../../assets/textures/logo.jpg");
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

        let mut rng = rand::thread_rng();

        for i in 0..count {
            let geom_ref = if i % 2 == 0 { &geometry } else { &geometry2 };
            let mat_ref = if i % 2 == 0 { &material } else { &material2 };
            let primitive = Primitive::new(geom_ref, mat_ref);

            let obj = Object3D::new();
            obj.position.set(Vec3::new(
                rng.gen_range(-2.0..2.0),
                rng.gen_range(-2.0..2.0),
                rng.gen_range(-2.0..2.0),
            ));
            obj.scale.set(Vec3::splat(rng.gen_range(0.02..0.08)));
            obj.primitives.borrow_mut().push(primitive);

            app.scene.add(&obj);
        }

        app
    }

    pub fn set_window_resized(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.screen_render_target
            .set_size(new_size.width, new_size.height);

        println!("Screen RenderTarget: {:?}", self.screen_render_target);
    }

    pub fn render(&mut self) {
        self.scene.update_world_matrix();
        let render_list = self.render_collector.collect(&self.scene);
        self.renderer
            .render(&render_list, &self.camera, &self.screen_render_target);
    }
}
