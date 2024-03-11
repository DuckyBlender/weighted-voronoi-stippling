use delaunator::{triangulate, Point};
use macroquad::prelude::*;

const RADIUS: f32 = 3.0;
const AMOUNT: i32 = 100;

fn window_conf() -> Conf {
    Conf {
        window_title: "Weighted Voronoi Stippling".to_owned(),
        fullscreen: false,
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

struct Canvas {
    points: Vec<Point>,
}

impl Canvas {
    fn new() -> Self {
        // Generate random points
        let mut points = Vec::new();
        for _ in 0..AMOUNT {
            points.push(Point {
                x: rand::gen_range(0.0, screen_width().into()),
                y: rand::gen_range(0.0, screen_height().into()),
            });
        }
        Self { points }
    }

    fn randomize_points(&mut self) {
        self.clear_points();
        for _ in 0..AMOUNT {
            self.points.push(Point {
                x: rand::gen_range(0.0, screen_width().into()),
                y: rand::gen_range(0.0, screen_height().into()),
            });
        }
    }

    fn draw_points(&self) {
        for point in &self.points {
            draw_circle(point.x as f32, point.y as f32, RADIUS, BLUE);
        }
    }

    fn clear_points(&mut self) {
        self.points.clear();
    }

    fn get_points(&self) -> &Vec<Point> {
        &self.points
    }
}

fn hollow_triangle(x: Vec2, y: Vec2, z: Vec2, color: Color) {
    draw_line(x.x, x.y, y.x, y.y, 2.0, color);
    draw_line(y.x, y.y, z.x, z.y, 2.0, color);
    draw_line(z.x, z.y, x.x, x.y, 2.0, color);
}

fn calculate_estimated_centroids(triangles: &Vec<usize>, points: &Vec<Point>) -> Vec<Point> {
    let mut centroids = Vec::new();
    for i in (0..triangles.len()).step_by(3) {
        let x = &points[triangles[i]];
        let y = &points[triangles[i + 1]];
        let z = &points[triangles[i + 2]];
        let centroid = Point {
            x: (x.x + y.x + z.x) / 3.0,
            y: (x.y + y.y + z.y) / 3.0,
        };
        centroids.push(centroid);
    }
    centroids
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut canvas = Canvas::new();

    // Triangulate the points
    let mut triangles = triangulate(&canvas.points); // This returns the indices of the points that make up the triangles

    // Approximate the centroid by getting the average of the x and y coordinates
    let mut centroids = calculate_estimated_centroids(&triangles.triangles, &canvas.points);

    loop {
        // If space is pressed, randomize the points
        if is_key_pressed(KeyCode::Space) {
            canvas.randomize_points();
            triangles = triangulate(&canvas.points);
            centroids = calculate_estimated_centroids(&triangles.triangles, &canvas.points);
        }

        let points = &triangles.hull;
        let triangles = &triangles.triangles;



        // Draw the triangles
        for i in (0..triangles.len()).step_by(3) {
            let x = &canvas.points[triangles[i]];
            let y = &canvas.points[triangles[i + 1]];
            let z = &canvas.points[triangles[i + 2]];
            hollow_triangle(
                vec2(x.x as f32, x.y as f32),
                vec2(y.x as f32, y.y as f32),
                vec2(z.x as f32, z.y as f32),
                WHITE,
            );
        }

        // Draw the estimated centroids
        for centroid in &centroids {
            draw_circle(centroid.x as f32, centroid.y as f32, 2.0, RED);
        }

        // Draw the points
        canvas.draw_points();

        // Draw the text
        draw_text("RED - Estimated Centroid", 10.0, 10.0, 20.0, WHITE);
        draw_text("BLUE - Random Points", 10.0, 30.0, 20.0, WHITE);

        next_frame().await
    }
}
