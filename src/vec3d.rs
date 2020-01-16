use std::fmt;

pub struct Vec3D(pub f64, pub f64, pub f64, pub f64);

impl Vec3D {
    fn new(x: f64, y: f64, z: f64) -> Vec3D {
        Vec3D(x, y, z, 0.0)
    }
}

// Allows vectors to be formatted as Strings using the `{}` marker.
impl fmt::Display for Vec3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

pub trait VecMath {
    // Scales this vector by the given scalar `scale_factor`.
    fn scale(&self, scale_factor: f64) -> Vec3D;

    // Computes the sum of this vector and `other`.
    fn add(&self, other: &Vec3D) -> Vec3D;

    // Computes the difference of this vector and `other`.
    fn subtract(&self, other: &Vec3D) -> Vec3D {
        self.add(&other.scale(-1.0))
    }

    // Normalizes this vector to have length 1.
    fn normalize(&self) -> Vec3D;

    // Computes the dot product of this vector and `other`.
    fn dot (&self, other : &Vec3D) -> f64;

    // Computes the cross product of this vector and `other`.
    fn cross(&self, other: &Vec3D) -> Vec3D;
}

impl VecMath for Vec3D {

    fn scale(&self, scale_factor: f64) -> Vec3D {
        let Vec3D(x, y, z, w) = self;

        Vec3D(x * scale_factor, y * scale_factor, z * scale_factor, *w)
    }

    fn add(&self, other: &Vec3D) -> Vec3D {
        let Vec3D(x1, y1, z1, w1) = self;
        let Vec3D(x2, y2, z2, w2) = other;

        Vec3D(x1 + x2, y1 + y2, z1 + z2, (w1 + w2).min(1.0))
    }

    fn dot(&self, other: &Vec3D) -> f64 {
        let Vec3D(x1, y1, z1, _) = self;
        let Vec3D(x2, y2, z2, _) = other;

        x1 * x2 + y1 * y2 + z1 * z2
    }

    fn cross(&self, other: &Vec3D) -> Vec3D {
        let Vec3D(x1, y1, z1, _) = self;
        let Vec3D(x2, y2, z2, _) = other;

        Vec3D::new(
            y1 * z2 - y2 * z1,
            z1 * x2 - z2 * x1,
            x1 * y2 - x2 * y1
        )
    }

    fn normalize(&self) -> Vec3D {
        let Vec3D(x, y, z, w) = self;
        let length = (x * x + y * y + z * z).sqrt();

        Vec3D(x / length, y / length, z / length, *w)
    }
}
