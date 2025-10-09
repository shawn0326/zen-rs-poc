use std::rc::Rc;

use zen_rs_poc::{
    graphics::{Geometry, Material, Primitive},
    render::{RenderTarget, ScreenSurfaceLike},
    scene::{Object3D, Scene},
};

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
}
