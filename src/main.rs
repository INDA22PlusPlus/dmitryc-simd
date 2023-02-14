use::microbench::*;

struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

fn add_vec3_normal(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3{x:u.x + v.x, y:u.y + v.y, z:u.z + v.z}
}

fn main() {
    let u = Vec3{x:1.0, y:2.0, z:3.0};
    let v = Vec3{x:4.0, y:5.0, z:6.0};

    let options = Options::default();

    microbench::bench(&options, "add Vec3 normal", || add_vec3_normal(&u, &v));

    // println!("Hello, world!");
}
