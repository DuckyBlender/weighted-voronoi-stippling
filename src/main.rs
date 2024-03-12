use macroquad::prelude::*;
use voronoi::{make_polygons, voronoi, Point};

const POINT_SIZE: f32 = 2.0;
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
    let mut pause = false;
    let mut step = false;
    for _ in 0..POINT_AMOUNT {
        let x: f64 = rand::gen_range(0., screen_width() as f64);
        let y: f64 = rand::gen_range(0., screen_height() as f64);
        points.push(Point::new(x, y));
    }
    let mut vor_diagram = voronoi(points.clone(), WINDOW_SIZE as f64);
    let mut vor_polys = make_polygons(&vor_diagram);

    loop {
        // Clear screen
        clear_background(BLACK);

        if is_key_pressed(KeyCode::P) {
            pause = !pause;
        }

        if is_key_pressed(KeyCode::N) {
            if pause {
                step = true;
            }
        }

        if is_key_pressed(KeyCode::Space) {
            points.clear();
            for _ in 0..POINT_AMOUNT {
                let x: f64 = rand::gen_range(0., screen_width() as f64);
                let y: f64 = rand::gen_range(0., screen_height() as f64);
                points.push(Point::new(x, y));
            }
        }
        if is_key_down(KeyCode::Up) {
            lerp_amount = (lerp_amount + 0.01).min(1.0);
        }
        if is_key_down(KeyCode::Down) {
            lerp_amount = (lerp_amount - 0.01).max(0.0);
        }

        // Calculate delaunay
        // let delaunay = triangulate(dpoints.as_slice());
        // Calculate voronoi
        if !pause || step {
            vor_diagram = voronoi(points.clone(), WINDOW_SIZE as f64);
            vor_polys = make_polygons(&vor_diagram);
        }

        // Draw the voronoi polygons
        for poly in vor_polys.iter() {
            // Begin creating a shape
            let mut shape = Vec::new();
            for i in 0..poly.len() {
                shape.push(vec2(poly[i].x() as f32, poly[i].y() as f32));
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
                let cross = v0.x() * v1.y() - v1.x() * v0.y();
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
        if !pause || step {
            for i in 0..points.len() {
                // Check if i is out of bounds
                if i >= centroids.len() {
                    break;
                }

                let new = lerp(
                    Point::new(points[i].x(), points[i].y()),
                    Point::new(centroids[i].x as f64, centroids[i].y as f64),
                    lerp_amount,
                );

                let new_x = new.clone().x();
                let new_y = new.clone().y();

                points[i] = Point::new(new_x, new_y);
                // Check if points[i] is actually updated
                if points[i].x() != new_x || points[i].y() != new_y {
                    println!("Point {} not updated", i);
                }
            }
        }

        // Render centroids
        for centroid in &centroids {
            draw_circle(centroid.x, centroid.y, CENTROID_SIZE, CENTROID_COLOR);
        }

        // Render points
        for point in &points {
            draw_circle(point.x() as f32, point.y() as f32, POINT_SIZE, POINT_COLOR);
        }

        // Draw the lerp_amount value
        draw_text(
            &format!("LERP_AMOUNT: {:.2}", lerp_amount),
            10.0,
            20.0,
            20.0,
            WHITE,
        );

        // Draw instructions in the bottom-left
        draw_text(
            "P: Pause | N: Step | Up/Down: Lerp Amount | Space: Randomize",
            10.0,
            screen_height() - 10.0,
            20.0,
            WHITE,
        );

        if pause {
            draw_text(
                "PAUSED",
                // Top right of the screen
                screen_width() - 150.0,
                50.0,
                50.0,
                RED,
            );
        }

        // If step is true, we just finished a step so toggle it back off
        if step {
            step = false;
        }

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
