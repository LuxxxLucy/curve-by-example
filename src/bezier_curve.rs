use raylib::math::Vector2;

#[derive(Clone, Debug)]
pub struct BezierCurve {
    pub start: Vector2,
    pub control1: Vector2,
    pub control2: Vector2,
    pub end: Vector2,
}

impl BezierCurve {
    pub fn new(start: Vector2, control1: Vector2, control2: Vector2, end: Vector2) -> Self {
        BezierCurve {
            start,
            control1,
            control2,
            end,
        }
    }

    pub fn point_at(&self, t: f32) -> Vector2 {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;

        Vector2::new(
            self.start.x * mt3
                + 3.0 * self.control1.x * mt2 * t
                + 3.0 * self.control2.x * mt * t2
                + self.end.x * t3,
            self.start.y * mt3
                + 3.0 * self.control1.y * mt2 * t
                + 3.0 * self.control2.y * mt * t2
                + self.end.y * t3,
        )
    }
}
