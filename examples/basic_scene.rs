use zen_rs_poc::scene::{Object3D, Scene};

fn main() {
    // Create a new scene and add an Object3D to it
    let scene = Scene::new();
    let obj = Object3D::new();
    scene.add(&obj);
}
