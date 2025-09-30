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
        obj.borrow_mut().position.x = 10.0;
        obj.borrow_mut().primitives.push(primitive);

        scene.add(&obj);
    }

    Object3D::traverse(&scene.root, &|o| {
        let mut o_ref = o.borrow_mut();

        println!(
            "Object3D {} has {} primitives",
            o_ref.name,
            o_ref.primitives.len()
        );

        if !o_ref.primitives.is_empty() {
            let primitive = &o_ref.primitives[0];
            let geometry = primitive.geometry();
            let material = primitive.material();

            println!("Geometry strong count: {}", Rc::strong_count(&geometry));
            println!("Material strong count: {}", Rc::strong_count(&material));

            // let mut o_ref = o.borrow_mut();
            o_ref.primitives.clear();

            println!("Geometry strong count: {}", Rc::strong_count(&geometry));
            println!("Material strong count: {}", Rc::strong_count(&material));
        }
    });

    scene.update_world_matrix();

    println!(
        "Scene root world matrix: {:?}",
        scene.root.borrow().children()[0]
            .borrow()
            .world_matrix
            .elements
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
