use macroquad::prelude::*;
use voronoi::{make_polygons, voronoi, Point};

const POINT_SIZE: f32 = 1.0;
const POINT_COLOR: Color = BLUE;

const CENTROID_SIZE: f32 = 3.0;
const CENTROID_COLOR: Color = GREEN;

const LINE_THICKNESS: f32 = 1.0;
const LINE_COLOR: Color = WHITE;

const POINT_AMOUNT: i32 = 100;
const WINDOW_SIZE: i32 = 600;

fn window_conf() -> Conf {
    Conf {
        window_title: "Weighted Voronoi Stippling".to_owned(),
        fullscreen: false,
        sample_count: 4,
        window_width: WINDOW_SIZE,
        window_height: WINDOW_SIZE,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Setup random points
    let mut points = Vec::new();
    let mut lerp_amount: f32 = 1.0;
    for _ in 0..POINT_AMOUNT {
        let x: f64 = rand::gen_range(0., screen_width() as f64);
        let y: f64 = rand::gen_range(0., screen_height() as f64);
        points.push(Point::new(x, y));
    }

    loop {
        // Clear screen
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Space) {
            points.clear();
            for _ in 0..POINT_AMOUNT {
                let x: f64 = rand::gen_range(0., screen_width() as f64);
                let y: f64 = rand::gen_range(0., screen_height() as f64);
                points.push(Point::new(x, y));
            }
        }
        if is_key_down(KeyCode::Up) {
            lerp_amount += 0.01;
        }
        if is_key_down(KeyCode::Down) {
            lerp_amount -= 0.01;
        }

        // Calculate delaunay
        // let delaunay = triangulate(dpoints.as_slice());
        // Calculate voronoi
        let vor_diagram = voronoi(points.clone(), WINDOW_SIZE as f64);
        let vor_polys = make_polygons(&vor_diagram);

        // Draw the points
        for i in 0..points.len() {
            draw_circle(
                points[i].x() as f32,
                points[i].y() as f32,
                POINT_SIZE,
                POINT_COLOR,
            );
        }

        // Draw the voronoi polygons
        for poly in vor_polys.iter() {
            // Begin creating a shape
            let mut shape = Vec::new();
            for i in 0..poly.len() {
                shape.push(vec2(
                    poly[i].x() as f32,
                    poly[i].y() as f32,
                ));
            }
            // Draw the shape
            // draw_poly_lines(x, y, sides, radius, rotation, thickness, color)
            draw_custom_shape(shape, LINE_COLOR, LINE_THICKNESS);
        }

        // Draw the centroids
        let mut centroids = Vec::new();
        for poly in vor_polys.iter() {
            let mut area = 0.0;
            let mut centroid = Vec2::new(0.0, 0.0);
            for i in 0..poly.len() {
                let v0 = poly[i];
                let v1 = poly[(i + 1) % poly.len()];
                let cross =
                    v0.x() * v1.y() - v1.x() * v0.y();
                area += cross;
                centroid.x += ((v0.x() + v1.x()) * cross) as f32;
                centroid.y += ((v0.y() + v1.y()) * cross) as f32;
            }
            area /= 2.;
            centroid.x /= (6.0 * area) as f32;
            centroid.y /= (6.0 * area) as f32;
            centroids.push(centroid);
        }

        // Lerp the points to the centroids
        for i in 0..points.len() {
            // Check if i is out of bounds
            if i >= centroids.len() {
                break;
            }

            let new_x = lerp(
                Point::new(points[i].x(), points[i].y()),
                Point::new(centroids[i].x as f64, centroids[i].y as f64),
                lerp_amount,
            )
            .x();

            let new_y = lerp(
                Point::new(points[i].x(), points[i].y()),
                Point::new(centroids[i].x as f64, centroids[i].y as f64),
                lerp_amount,
            )
            .y();

            points[i] = Point::new(new_x, new_y);
        }

        // Draw centroids
        for i in 0..centroids.len() {
            draw_circle(
                centroids[i].x,
                centroids[i].y,
                CENTROID_SIZE,
                CENTROID_COLOR,
            );
        }

        // Draw the lerp_amount value
        draw_text(
            &format!("LERP_AMOUNT: {:.2}", lerp_amount),
            10.0,
            20.0,
            20.0,
            WHITE,
        );

        // Update screen
        next_frame().await
    }
}

fn lerp(a: Point, b: Point, t: f32) -> Point {
    Point::new(
        a.x() + (b.x() - a.x()) * t as f64,
        a.y() + (b.y() - a.y()) * t as f64,
    )
}

fn draw_custom_shape(points: Vec<Vec2>, color: Color, thickness: f32) {
    for i in 0..points.len() {
        draw_line(
            points[i].x,
            points[i].y,
            points[(i + 1) % points.len()].x, // modulo because we need to connect the last point to the first
            points[(i + 1) % points.len()].y,
            thickness,
            color,
        );
    }
}

#[cfg(test)]
mod tests;
