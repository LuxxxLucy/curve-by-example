mod bezier_curve;
mod database;
mod interpolation;
mod similarity;

use bezier_curve::BezierCurve;
use raylib::prelude::*;
use std::f32::consts::PI;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const CURVE_SIZE: i32 = 50;
const CURVE_BACKGROUND: Color = Color::new(240, 240, 240, 255); // Slight grey
const CURRENT_CURVE_BACKGROUND: Color = Color::new(150, 150, 150, 255); // slightly darker grey

fn normalize_curve(curve: &BezierCurve) -> BezierCurve {
    let min_x = curve
        .start
        .x
        .min(curve.control1.x)
        .min(curve.control2.x)
        .min(curve.end.x);
    let max_x = curve
        .start
        .x
        .max(curve.control1.x)
        .max(curve.control2.x)
        .max(curve.end.x);
    let min_y = curve
        .start
        .y
        .min(curve.control1.y)
        .min(curve.control2.y)
        .min(curve.end.y);
    let max_y = curve
        .start
        .y
        .max(curve.control1.y)
        .max(curve.control2.y)
        .max(curve.end.y);

    let width = max_x - min_x;
    let height = max_y - min_y;
    let scale = (CURVE_SIZE as f32 - 4.0) / width.max(height);

    let normalize = |point: Vector2| -> Vector2 {
        Vector2::new(
            (point.x - min_x) * scale + 2.0,
            (point.y - min_y) * scale + 2.0,
        )
    };

    BezierCurve {
        start: normalize(curve.start),
        control1: normalize(curve.control1),
        control2: normalize(curve.control2),
        end: normalize(curve.end),
    }
}

fn draw_normalized_curve(
    d: &mut RaylibDrawHandle,
    curve: &BezierCurve,
    position: Vector2,
    size: i32,
    color: Color,
) {
    let normalized = normalize_curve(curve);
    let steps = 20;

    // Calculate the scale factor
    let scale_factor = size as f32 / CURVE_SIZE as f32;

    // Calculate the offset to center the curve within the rectangle
    let offset_x = (size as f32 - (CURVE_SIZE as f32 * scale_factor)) / 2.0;
    let offset_y = (size as f32 - (CURVE_SIZE as f32 * scale_factor)) / 2.0;

    for i in 0..steps {
        let t1 = i as f32 / steps as f32;
        let t2 = (i + 1) as f32 / steps as f32;
        let start = normalized.point_at(t1);
        let end = normalized.point_at(t2);
        d.draw_line(
            (start.x * scale_factor + position.x + offset_x) as i32,
            (start.y * scale_factor + position.y + offset_y) as i32,
            (end.x * scale_factor + position.x + offset_x) as i32,
            (end.y * scale_factor + position.y + offset_y) as i32,
            color,
        );
    }
}

fn calculate_weights(mouse_pos: Vector2, curve_positions: &[Vector2]) -> Vec<f32> {
    let distances: Vec<f32> = curve_positions
        .iter()
        .map(|&pos| mouse_pos.distance_to(pos).max(0.1))
        .collect();
    let max_distance = distances.iter().cloned().fold(0.0, f32::max);
    let total_inverse_distance: f32 = distances
        .iter()
        .map(|&d| (max_distance - d + 0.1).powi(2))
        .sum();

    distances
        .iter()
        .map(|&d| (max_distance - d + 0.1).powi(2) / total_inverse_distance)
        .collect()
}

fn point_in_polygon(point: Vector2, vertices: &[Vector2]) -> bool {
    let mut inside = false;
    let mut j = vertices.len() - 1;
    for i in 0..vertices.len() {
        if ((vertices[i].y > point.y) != (vertices[j].y > point.y))
            && (point.x
                < (vertices[j].x - vertices[i].x) * (point.y - vertices[i].y)
                    / (vertices[j].y - vertices[i].y)
                    + vertices[i].x)
        {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn closest_point_on_edge(point: Vector2, v1: Vector2, v2: Vector2) -> Vector2 {
    let edge = v2 - v1;
    let t = ((point - v1).dot(edge) / edge.dot(edge)).clamp(0.0, 1.0);
    v1 + edge * t
}

fn clamp_to_polygon(point: Vector2, vertices: &[Vector2]) -> Vector2 {
    if point_in_polygon(point, vertices) {
        return point;
    }

    let mut closest_point = vertices[0];
    let mut min_distance = point.distance_to(vertices[0]);

    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        let edge_point = closest_point_on_edge(point, vertices[i], vertices[j]);
        let distance = point.distance_to(edge_point);
        if distance < min_distance {
            closest_point = edge_point;
            min_distance = distance;
        }
    }

    closest_point
}

fn draw_dotted_line(d: &mut RaylibDrawHandle, start: Vector2, end: Vector2, color: Color) {
    let distance = start.distance_to(end);
    let direction = Vector2::new((end.x - start.x) / distance, (end.y - start.y) / distance);
    let dot_spacing = 5.0;
    let dot_size = 1.0;

    for i in 0..((distance / dot_spacing) as i32) {
        let t = i as f32 * dot_spacing / distance;
        let pos = Vector2::new(
            start.x + direction.x * (i as f32 * dot_spacing),
            start.y + direction.y * (i as f32 * dot_spacing),
        );

        // Calculate dot opacity based on distance
        let opacity = ((1.0 - t) * 255.0) as u8;
        let dot_color = Color::new(color.r, color.g, color.b, opacity);

        d.draw_circle(pos.x as i32, pos.y as i32, dot_size, dot_color);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Curve by Example")
        .build();

    let database = database::create_curve_database();
    let input_curve = database::create_input_curve();

    let similar_curves = similarity::find_similar_curves(&input_curve, &database, 5);

    // Calculate polygon (simplex vertex) positions
    let center_x = SCREEN_WIDTH as f32 / 2.0;
    let center_y = SCREEN_HEIGHT as f32 / 2.0;
    let radius = 200.0;
    let curve_positions: Vec<Vector2> = (0..5)
        .map(|i| {
            let angle = (i as f32 * 2.0 * PI / 5.0) - PI / 2.0; // Start from top
            Vector2::new(
                center_x + radius * angle.cos(),
                center_y + radius * angle.sin(),
            )
        })
        .collect();

    while !rl.window_should_close() {
        // Update
        let raw_mouse_pos = rl.get_mouse_position();
        let mouse_pos = clamp_to_polygon(raw_mouse_pos, &curve_positions);
        let weights = calculate_weights(mouse_pos, &curve_positions);

        let interpolated_curve = interpolation::interpolate_curves(&similar_curves, &weights);

        // Draw
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        // Draw the polygon defined by curve positions
        for i in 0..curve_positions.len() {
            let j = (i + 1) % curve_positions.len();
            d.draw_line(
                curve_positions[i].x as i32,
                curve_positions[i].y as i32,
                curve_positions[j].x as i32,
                curve_positions[j].y as i32,
                Color::LIGHTGRAY,
            );
        }

        // Draw similar curves
        for (i, curve) in similar_curves.iter().enumerate() {
            let weight = weights[i];
            let size = (CURVE_SIZE as f32 * (0.5 + weight * 2.0)) as i32; // Vary size based on weight
            d.draw_rectangle(
                (curve_positions[i].x - size as f32 / 2.0) as i32,
                (curve_positions[i].y - size as f32 / 2.0) as i32,
                size,
                size,
                CURVE_BACKGROUND,
            );
            draw_normalized_curve(
                &mut d,
                curve,
                curve_positions[i] - Vector2::new(size as f32 / 2.0, size as f32 / 2.0),
                size,
                Color::BLACK,
            );

            // Draw dotted line from mouse position to curve position
            draw_dotted_line(
                &mut d,
                mouse_pos,
                curve_positions[i],
                Color::new(0, 0, 0, 128), // Fixed semi-transparent black
            );
        }

        // Draw interpolated curve at mouse position
        d.draw_rectangle(
            (mouse_pos.x - CURVE_SIZE as f32 / 2.0) as i32,
            (mouse_pos.y - CURVE_SIZE as f32 / 2.0) as i32,
            CURVE_SIZE,
            CURVE_SIZE,
            CURRENT_CURVE_BACKGROUND,
        );
        draw_normalized_curve(
            &mut d,
            &interpolated_curve,
            mouse_pos - Vector2::new(CURVE_SIZE as f32 / 2.0, CURVE_SIZE as f32 / 2.0),
            CURVE_SIZE,
            Color::RED,
        );

        d.draw_text(
            "Move the mouse to interpolate between curves",
            10,
            10,
            20,
            Color::DARKGRAY,
        );
    }
}
