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

    fn draw_triangles(&self, triangles: &Vec<usize>) {
        for i in (0..triangles.len()).step_by(3) {
            let x = &self.points[triangles[i]];
            let y = &self.points[triangles[i + 1]];
            let z = &self.points[triangles[i + 2]];
            hollow_triangle(
                vec2(x.pos.x as f32, x.pos.y as f32),
                vec2(y.pos.x as f32, y.pos.y as f32),
                vec2(z.pos.x as f32, z.pos.y as f32),
                WHITE,
            );
        }
    }

    fn clear_points(&mut self) {
        self.points.clear();
    }



    fn move_points(&mut self) {
        // Move to the points to the centroid by 10%
        // TODO
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

fn area(vertices: &[Vec2]) -> f32 {
    let n = vertices.len();
    let mut area = 0.0;
    for i in 0..n {
        let j = (i + 1) % n;
        area += vertices[i].x * vertices[j].y;
        area -= vertices[i].y * vertices[j].x;
    }
    area.abs() / 2.0
}

fn centroid(vertices: &[Vec2]) -> Vec2 {
    let n = vertices.len();
    let area = 0.5 * vertices.iter().zip(vertices.iter().cycle().skip(1)).fold(0.0, |a, (&v0, &v1)| {
        a + (v0.x * v1.y - v1.x * v0.y)
    });
    let mut cx = 0.0;
    let mut cy = 0.0;
    for i in 0..n {
        let v0 = vertices[i];
        let v1 = vertices[(i + 1) % n];
        cx += (v0.x + v1.x) * (v0.x * v1.y - v1.x * v0.y);
        cy += (v0.y + v1.y) * (v0.x * v1.y - v1.x * v0.y);
    }
    Vec2::new(cx / (6.0 * area), cy / (6.0 * area))
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
        let estimated_centroid = calculate_estimated_centroids(&triangles.triangles, &canvas.as_points());
        let triangles = &triangles.triangles;

        // Calculate the area of each triangle
        let mut areas = Vec::new();
        for i in (0..triangles.len()).step_by(3) {
            let x = &canvas.points[triangles[i]];
            let y = &canvas.points[triangles[i + 1]];
            let z = &canvas.points[triangles[i + 2]];
            let vertices = vec![x.pos, y.pos, z.pos];
            areas.push(area(&vertices));
        }

        // We have the area, now we can calculate the centroid
        let mut centroids = Vec::new();
        for i in (0..triangles.len()).step_by(3) {
            let x = &canvas.points[triangles[i]];
            let y = &canvas.points[triangles[i + 1]];
            let z = &canvas.points[triangles[i + 2]];
            let vertices = vec![x.pos, y.pos, z.pos];
            centroids.push(centroid(&vertices));
        }

        // Draw the triangles
        canvas.draw_triangles(triangles);

        // Draw the centroids
        for centroid in &centroids {
            draw_circle(centroid.x as f32, centroid.y as f32, 2.0, GREEN);
        }

        // Draw the points
        canvas.draw_points();

        // Draw the text
        draw_text("GRAY - Estimated Centroid", 10.0, 10.0, 20.0, WHITE);
        draw_text("GREEN - Centroid", 10.0, 30.0, 20.0, WHITE);
        draw_text("BLUE - Random Points", 10.0, 50.0, 20.0, WHITE);

        next_frame().await
    }
}
