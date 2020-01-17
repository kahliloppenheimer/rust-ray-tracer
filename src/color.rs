// RGB representation of a color.
#[derive(Debug, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {

    pub fn add(&self, other: &Color) -> Color {
        let Color(r1, g1, b1) = *self;
        let Color(r2, g2, b2) = *other;

        Color(
            r1.saturating_add(r2),
            g1.saturating_add(g2),
            b1.saturating_add(b2))
    }

    pub fn multiply(&self, other: &Color) -> Color {
        let Color(r1, g1, b1) = *self;
        let Color(r2, g2, b2) = *other;

        Color(
            r1.saturating_mul(r2),
            g1.saturating_mul(g2),
            b1.saturating_mul(b2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_basic_cases() {
        let c1 : Color = Color(1, 2, 3);
        let c2 : Color = Color(5, 6, 7);

        assert_eq!(c1.add(&c2), Color(6, 8, 10));
        assert_eq!(c1.add(&c2), c2.add(&c1));
    }

    #[test]
    fn add_bounds_overflow() {
        let c1 = Color(150, 150, 150);
        let c2 = Color(150, 150, 150);

        assert_eq!(c1.add(&c2), c2.add(&c1));
        assert_eq!(c1.add(&c2), Color(255, 255, 255));
    }
    
    #[test]
    fn mul_basic_cases() {
        let c1 = Color(1, 2, 3);
        let c2 = Color(5, 6, 7);
        
        assert_eq!(c1.multiply(&c2), c2.multiply(&c1));
        assert_eq!(c1.multiply(&c2), Color(5, 12, 21));
    }

    #[test]
    fn mul_bounds_overflow() {
        let c1 = Color(30, 30, 30);
        let c2 = Color(10, 10, 10);

        assert_eq!(c1.multiply(&c2), c2.multiply(&c1));
        assert_eq!(c1.multiply(&c2), Color(255, 255, 255));
    }
}
