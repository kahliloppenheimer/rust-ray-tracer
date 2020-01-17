use crate::vec3d::{Vec3D, VecMath};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Ray<'a> {
    start : &'a Vec3D,
    direction : &'a Vec3D,
}

impl Ray<'_> {
    // Returns the position of the ray at time `t`.
    fn at_time(&self, t : f64) -> Vec3D {
        self.start.add(&self.direction.scale(t))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic_at_time() {
        let r1 = Ray{
            start: &Vec3D::new(0.0, 0.0, 0.0),
            direction: &Vec3D::new(1.0, 1.0, 1.0)};
        assert_eq!(r1.at_time(1.0), Vec3D::new(1.0, 1.0, 1.0));
        assert_eq!(r1.at_time(0.5), Vec3D::new(0.5, 0.5, 0.5));
    }
}


