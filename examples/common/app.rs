use image::GenericImageView;
use rand::Rng;
use std::sync::Arc;
use winit::window::Window;
use zen_rs_poc::{
    camera::{Camera, PerspectiveProjection},
    graphics::{Geometry, Primitive, Texture, TextureSource},
    math::{Color4, Mat4, Vec3},
    render::{LoadOp, RenderCollector, RenderTarget},
    scene::{Object3D, Scene},
    symbol,
    wgpu::Renderer,
};

pub struct MainCamera {
    eye: Vec3,
    target: Vec3,
    proj: PerspectiveProjection,
    inner: Camera,
}

impl MainCamera {
    pub fn new(eye: Vec3, target: Vec3, up: Vec3, proj: PerspectiveProjection) -> Self {
        let mut camera = Camera::default();
        camera.set_view(Mat4::look_at_rh(eye, target, up));
        camera.set_projection(proj.to_mat4());
        Self {
            eye,
            target,
            proj,
            inner: camera,
        }
    }

    pub fn update_view(&mut self, view: Mat4, eye: Vec3, target: Vec3) -> &mut Self {
        self.eye = eye;
        self.target = target;
        self.inner.set_view(view);
        self
    }

    pub fn update_aspect(&mut self, aspect: f32) -> &mut Self {
        self.proj.aspect = aspect;
        self.inner.set_projection(self.proj.to_mat4());
        self
    }

    pub fn fovy(&self) -> f32 {
        self.proj.fovy_deg.to_radians()
    }
}

pub struct App<'window> {
    pub window: Arc<Window>,
    renderer: Renderer<'window>,
    screen_render_target: RenderTarget,
    render_collector: RenderCollector,
    pub scene: Scene,
    pub camera: MainCamera,
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

        let renderer = Renderer::new(&instance, surface).await;

        let mut screen_render_target = RenderTarget::from_surface(0, size.width, size.height);
        let color_attachment_0 = screen_render_target.color_attachments.get_mut(0).unwrap();
        color_attachment_0.ops.load = LoadOp::Clear(Color4::new(0.1, 0.2, 0.3, 1.0));
        screen_render_target.with_depth24();

        let render_collector = RenderCollector {};

        let scene = Scene::new();

        let camera = MainCamera::new(
            (0.0, 0.0, 10.0).into(),
            (0.0, 0.0, 0.0).into(),
            Vec3::Y,
            PerspectiveProjection::new(45.0, size.width as f32 / size.height as f32, 0.1, 100.0),
        );

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
        let diffuse_dimensions = diffuse_image.dimensions();

        let texture = Texture::new()
            .with_source(TextureSource::D2 {
                data: diffuse_image.to_rgba8().into_raw(),
                width: diffuse_dimensions.0,
                height: diffuse_dimensions.1,
            })
            .into_ref();

        let geometry = Geometry::create_unit_cube();
        let geometry2 = Geometry::create_unit_quad();

        let unlit_shader = zen_rs_poc::shader::builtins::unlit_shader();
        let pbr_shader = zen_rs_poc::shader::builtins::pbr_shader();

        let material =
            zen_rs_poc::material::Material::from_shader(unlit_shader.clone()).into_rc_cell();
        material
            .borrow_mut()
            .set_param_vec4f(symbol!("albedo_factor"), [1.0, 1.0, 1.0, 1.0])
            .set_param_t(symbol!("albedo_texture"), texture.clone());

        let material2 =
            zen_rs_poc::material::Material::from_shader(pbr_shader.clone()).into_rc_cell();
        material2
            .borrow_mut()
            .set_param_col4(symbol!("albedo_factor"), Color4::new(0.4, 0.4, 1.0, 1.0))
            .set_param_f(symbol!("roughness"), 0.5)
            .set_param_f(symbol!("metallic"), 0.0);

        let mut rng = rand::thread_rng();

        for i in 0..count {
            let geom_ref = if i % 2 == 0 {
                geometry.clone()
            } else {
                geometry2.clone()
            };
            let mat_ref = if i % 3 == 0 {
                material.clone()
            } else {
                material2.clone()
            };
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
        self.camera
            .update_aspect(new_size.width as f32 / new_size.height as f32);
        self.screen_render_target
            .resize(new_size.width, new_size.height);
    }

    pub fn render(&mut self) {
        self.scene.update_world_matrix();
        let render_list = self.render_collector.collect(&self.scene);
        self.renderer
            .render(&render_list, &self.camera.inner, &self.screen_render_target);
    }
}
