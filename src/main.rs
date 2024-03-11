use delaunator::{triangulate, Point};
use macroquad::prelude::*;

const RADIUS: f32 = 3.0;
const AMOUNT: i32 = 100;

fn window_conf() -> Conf {
    Conf {
        window_title: "Weighted Voronoi Stippling".to_owned(),
        fullscreen: false,
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

struct Canvas {
    points: Vec<Dot>,
}

struct Dot {
    pos: Vec2,
    velocity: Vec2,
}

impl Canvas {
    fn new() -> Self {
        // Generate random points
        let mut points = Vec::new();
        for _ in 0..AMOUNT {
            let x = rand::gen_range(0.0, screen_width());
            let y = rand::gen_range(0.0, screen_height());
            let velocity = vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0));

            points.push(Dot {
                pos: vec2(x, y),
                velocity,
            });
        }
        Canvas { points }
    }

    fn randomize_points(&mut self) {
        self.clear_points();
        for _ in 0..AMOUNT {
            let x = rand::gen_range(0.0, screen_width());
            let y = rand::gen_range(0.0, screen_height());
            self.points.push(Dot {
                pos: vec2(x, y),
                velocity: vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)),
            });
        }
    }

    fn draw_points(&self) {
        for point in &self.points {
            draw_circle(point.pos.x as f32, point.pos.y as f32, RADIUS, BLUE);
        }
    }

    fn clear_points(&mut self) {
        self.points.clear();
    }

    fn move_points(&mut self) {
        // Move the points looking at the velocity
        for point in &mut self.points {
            point.pos += point.velocity;
            if point.pos.x < 0.0 || point.pos.x > screen_width() {
                point.velocity.x *= -1.0;
            }
            if point.pos.y < 0.0 || point.pos.y > screen_height() {
                point.velocity.y *= -1.0;
            }
        }
    }

    fn as_points(&self) -> Vec<Point> {
        self.points
            .iter()
            .map(|point| Point {
                x: point.pos.x as f64,
                y: point.pos.y as f64,
            })
            .collect()
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

    loop {
        // If space is pressed, randomize the points
        if is_key_pressed(KeyCode::Space) {
            canvas.randomize_points();
            
        } else {
            canvas.move_points();

        }

        let triangles = triangulate(&canvas.as_points());
        let centroids = calculate_estimated_centroids(&triangles.triangles, &canvas.as_points());

        let points = &triangles.hull;
        let triangles = &triangles.triangles;



        // Draw the triangles
        for i in (0..triangles.len()).step_by(3) {
            let x = &canvas.points[triangles[i]];
            let y = &canvas.points[triangles[i + 1]];
            let z = &canvas.points[triangles[i + 2]];
            hollow_triangle(
                vec2(x.pos.x as f32, x.pos.y as f32),
                vec2(y.pos.x as f32, y.pos.y as f32),
                vec2(z.pos.x as f32, z.pos.y as f32),
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
