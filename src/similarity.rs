use crate::bezier_curve::BezierCurve;
use raylib::math::Vector2;

pub fn curve_similarity(a: &BezierCurve, b: &BezierCurve) -> f32 {
    let distance = |v1: Vector2, v2: Vector2| v1.distance_to(v2);

    distance(a.start, b.start)
        + distance(a.control1, b.control1)
        + distance(a.control2, b.control2)
        + distance(a.end, b.end)
}

pub fn find_similar_curves<'a>(
    input: &BezierCurve,
    database: &'a [BezierCurve],
    count: usize,
) -> Vec<&'a BezierCurve> {
    let mut similarities: Vec<(f32, &BezierCurve)> = database
        .iter()
        .map(|curve| (curve_similarity(input, curve), curve))
        .collect();

    similarities.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    similarities
        .into_iter()
        .take(count)
        .map(|(_, curve)| curve)
        .collect()
}
