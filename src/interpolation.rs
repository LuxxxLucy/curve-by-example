use crate::bezier_curve::BezierCurve;
use raylib::math::Vector2;

pub fn interpolate_curves(curves: &[&BezierCurve], weights: &[f32]) -> BezierCurve {
    assert_eq!(
        curves.len(),
        weights.len(),
        "Number of curves and weights must match"
    );

    let total_weight: f32 = weights.iter().sum();

    let interpolate_point = |getter: fn(&BezierCurve) -> Vector2| -> Vector2 {
        let mut result = Vector2::new(0.0, 0.0);
        for (curve, &weight) in curves.iter().zip(weights.iter()) {
            let point = getter(curve);
            result.x += point.x * weight;
            result.y += point.y * weight;
        }
        result.x /= total_weight;
        result.y /= total_weight;
        result
    };

    BezierCurve {
        start: interpolate_point(|c| c.start),
        control1: interpolate_point(|c| c.control1),
        control2: interpolate_point(|c| c.control2),
        end: interpolate_point(|c| c.end),
    }
}
