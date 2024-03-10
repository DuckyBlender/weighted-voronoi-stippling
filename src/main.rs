use delaunator::{triangulate, Point};
use macroquad::prelude::*;

const RADIUS: f32 = 2.0;
const AMOUNT: i32 = 1000;

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

    fn draw_points(&self) {
        for point in &self.points {
            draw_circle(point.x as f32, point.y as f32, RADIUS, WHITE);
        }
    }

    fn clear_points(&mut self) {
        self.points.clear();
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut canvas = Canvas::new();
    // Triangulate the points
    // let triangles = triangulate(&canvas.points); // This returns the indices of the points that make up the triangles

    loop {
        canvas.draw_points();

        next_frame().await
    }
}
