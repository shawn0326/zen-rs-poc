use criterion::{Criterion, criterion_group, criterion_main};
use std::rc::Rc;
use std::{cell::RefCell, hint::black_box};
use zen_rs_poc::{
    graphics::{Geometry, Material, Primitive},
    scene::{Object3D, Scene},
};

const PYRAMID_LEVELS: usize = 8;

fn build_pyramid_scene() -> Scene {
    let scene = Scene::new();
    let geometry = Geometry::create_unit_cube();
    let material = Material::new();

    fn build_level(
        parent: &Rc<Object3D>,
        geometry: &Rc<RefCell<Geometry>>,
        material: &Rc<RefCell<Material>>,
        current_level: usize,
        max_levels: usize,
    ) {
        if current_level >= max_levels {
            return;
        }
        let children_count = max_levels - current_level;
        for _ in 0..children_count {
            let obj = Object3D::new();
            let primitive = Primitive::new(geometry, material);
            obj.primitives.borrow_mut().push(primitive);
            Object3D::add(parent, &obj);
            build_level(&obj, geometry, material, current_level + 1, max_levels);
        }
    }

    build_level(&scene.root, &geometry, &material, 0, PYRAMID_LEVELS);
    scene
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("scene_benchmarks");

    // group.bench_function("build_pyramid_scene", |b| {
    //     b.iter(|| {
    //         black_box(build_pyramid_scene());
    //     });
    // });

    let scene_for_update = build_pyramid_scene();
    group.bench_function("update_world_matrix", move |b| {
        b.iter(|| {
            scene_for_update.update_world_matrix();
        });
    });

    let scene_for_traverse = build_pyramid_scene();
    group.bench_function("traverse_scene", move |b| {
        b.iter(|| {
            for obj in Object3D::traverse(&scene_for_traverse.root) {
                black_box(obj.name.len());
            }
        });
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
