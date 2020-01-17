mod vec3d;
mod ray;
mod png_writer;

use crate::vec3d::Vec3D;
use crate::vec3d::VecMath;
use crate::png_writer::write_png;
use crate::png_writer::IMAGE_DATA_SIZE;

fn main() {
    let a: Vec3D = Vec3D(2.0, 2.0, 2.0, 2.0);
    let b: Vec3D = Vec3D(1.0, 1.0, 1.0, 1.0);

    println!("{} dot {} = {}", a, b, a.dot(&b));
    println!("{} + {} = {}", a, b, a.add(&b));
    println!("{} - {} = {}", a, b, a.subtract(&b));
    println!("{} cross {} = {}", a, b, a.cross(&b));

    let data : [u8; IMAGE_DATA_SIZE as usize] = [0; IMAGE_DATA_SIZE as usize];
    write_png("foo.png", &data[..]);
}
