use crate::bezier_curve::BezierCurve;
use raylib::math::Vector2;

pub fn create_curve_database() -> Vec<BezierCurve> {
    vec![
        // Slight curve
        BezierCurve::new(
            Vector2::new(0.0, 0.0),
            Vector2::new(33.0, -33.0),
            Vector2::new(66.0, -33.0),
            Vector2::new(100.0, 0.0),
        ),
        // S-curve
        BezierCurve::new(
            Vector2::new(0.0, 0.0),
            Vector2::new(33.0, -50.0),
            Vector2::new(66.0, 50.0),
            Vector2::new(100.0, 0.0),
        ),
        // Loop
        BezierCurve::new(
            Vector2::new(0.0, 0.0),
            Vector2::new(100.0, -66.0),
            Vector2::new(0.0, -66.0),
            Vector2::new(100.0, 0.0),
        ),
        // Sharp turn
        BezierCurve::new(
            Vector2::new(0.0, 0.0),
            Vector2::new(20.0, 0.0),
            Vector2::new(20.0, 80.0),
            Vector2::new(100.0, 80.0),
        ),
        // Zigzag
        BezierCurve::new(
            Vector2::new(0.0, 0.0),
            Vector2::new(33.0, -40.0),
            Vector2::new(66.0, 40.0),
            Vector2::new(100.0, 0.0),
        ),
    ]
}

pub fn create_input_curve() -> BezierCurve {
    BezierCurve::new(
        Vector2::new(0.0, 0.0),
        Vector2::new(40.0, -40.0),
        Vector2::new(60.0, -40.0),
        Vector2::new(100.0, 0.0),
    )
}
