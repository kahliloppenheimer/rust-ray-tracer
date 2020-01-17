use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Vec3D(pub f64, pub f64, pub f64, pub f64);

impl Vec3D {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3D {
        Vec3D(x, y, z, 0.0)
    }
}

// Allows vectors to be formatted as Strings using the `{}` marker.
impl fmt::Display for Vec3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Vec3D {

    // Scales this vector by the given scalar `scale_factor`.
    pub fn scale(&self, scale_factor: f64) -> Vec3D {
        let Vec3D(x, y, z, w) = self;

        Vec3D(x * scale_factor, y * scale_factor, z * scale_factor, *w)
    }

    // Computes the sum of this vector and `other`.
    pub fn add(&self, other: &Vec3D) -> Vec3D {
        let Vec3D(x1, y1, z1, w1) = self;
        let Vec3D(x2, y2, z2, w2) = other;

        Vec3D(x1 + x2, y1 + y2, z1 + z2, (w1 + w2).min(1.0))
    }

    // Computes the difference of this vector and `other`.
    pub fn subtract(&self, other: &Vec3D) -> Vec3D {
        self.add(&other.scale(-1.0))
    }

    // Computes the dot product of this vector and `other`.
    pub fn dot(&self, other: &Vec3D) -> f64 {
        let Vec3D(x1, y1, z1, _) = self;
        let Vec3D(x2, y2, z2, _) = other;

        x1 * x2 + y1 * y2 + z1 * z2
    }

    // Computes the cross product of this vector and `other`.
    pub fn cross(&self, other: &Vec3D) -> Vec3D {
        let Vec3D(x1, y1, z1, _) = self;
        let Vec3D(x2, y2, z2, _) = other;

        Vec3D::new(
            y1 * z2 - y2 * z1,
            z1 * x2 - z2 * x1,
            x1 * y2 - x2 * y1
        )
    }

    // Normalizes this vector to have length 1.
    pub fn normalize(&self) -> Vec3D {
        let Vec3D(x, y, z, w) = self;
        let length = (x * x + y * y + z * z).sqrt();

        Vec3D(x / length, y / length, z / length, *w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    const ZERO_VEC : Vec3D = Vec3D(0.0, 0.0, 0.0, 0.0);

    #[test]
    fn add_basic_cases() {
        assert_eq!(ZERO_VEC.add(&ZERO_VEC), ZERO_VEC);

        let x = Vec3D(1.0, 2.0, 3.0, 4.0);
        let y = Vec3D(4.0, 3.0, 2.0, 1.0);
        let expected = Vec3D(5.0, 5.0, 5.0, 1.0);
        assert_eq!(x.add(&y), expected);
    }

    #[test]
    fn subtract_basic_cases() {
        assert_eq!(ZERO_VEC.subtract(&ZERO_VEC), ZERO_VEC);

        let x = Vec3D(1.0, 2.0, 3.0, 4.0);
        let y = Vec3D(4.0, 3.0, 2.0, 1.0);
        let expected = Vec3D(-3.0, -1.0, 1.0, 1.0);
        assert_eq!(x.subtract(&y), expected);
    }

    #[test]
    fn normalize_basic_cases() {
        let a = Vec3D::new(1.0, 1.0, 1.0);
        assert_eq!(a.normalize(), Vec3D::new(1.0 / 3.0_f64.sqrt(), 1.0 / 3.0_f64.sqrt(), 1.0 / 3.0_f64.sqrt()))
    }

    #[test]
    fn dot_basic_cases() {
        assert_eq!(ZERO_VEC.dot(&ZERO_VEC), 0.0);

        let p1 = Vec3D::new(1.0, -2.0, 3.0);
        assert_eq!(p1.dot(&ZERO_VEC), 0.0);

        let p2 = Vec3D::new(-4.0, 4.0, 4.0);
        assert_eq!(p1.dot(&p2), 0.0);

        let p3 = Vec3D::new(1.0, 2.0, 2.0);
        assert_eq!(p1.dot(&p3), 3.0);

        assert_eq!(p2.dot(&p3), 12.0);
    }

    #[test]
    fn cross_basic_cases() {
        let p1 = Vec3D::new(5.5, 6.8, -1.2);
        let p2 = Vec3D::new(-9.9, -10.1, -5.2);
        let crossed = p1.cross(&p2);
        // Check that cross sign is opposite depending on order.
        assert_eq!(crossed.scale(-1.0), p2.cross(&p1));
        // Check that cross is perpendicular to both input vectors.
        assert_approx_eq!(p1.dot(&crossed), 0.0);
        assert_approx_eq!(p2.dot(&crossed), 0.0);
    }
}
