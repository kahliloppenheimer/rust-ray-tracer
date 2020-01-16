mod vec3d;

use crate::vec3d::Vec3D;
use crate::vec3d::VecMath;

fn main() {
    let a: Vec3D = Vec3D(2.0, 2.0, 2.0, 2.0);
    let b: Vec3D = Vec3D(1.0, 1.0, 1.0, 1.0);

    println!("{} dot {} = {}", a, b, a.dot(&b));
    println!("{} + {} = {}", a, b, a.add(&b));
    println!("{} - {} = {}", a, b, a.subtract(&b));
    println!("{} cross {} = {}", a, b, a.cross(&b));
}
