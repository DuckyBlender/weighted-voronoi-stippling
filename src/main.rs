use delaunator::{triangulate, Point as DPoint};
use macroquad::prelude::*;
use voronoi::{make_polygons, voronoi, Point as VPoint};

const POINT_SIZE: f32 = 1.0;
const POINT_COLOR: Color = BLUE;

const CENTROID_SIZE: f32 = 3.0;
const CENTROID_COLOR: Color = GREEN;

const LINE_THICKNESS: f32 = 1.0;
const LINE_COLOR: Color = WHITE;

const POINT_AMOUNT: i32 = 100;
const WINDOW_SIZE: i32 = 600;
const LERP_AMOUNT: f32 = 0.01;

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
    let mut dpoints = Vec::new();
    let mut vpoints = Vec::new();
    // TODO: Make this more efficient
    for _ in 0..POINT_AMOUNT {
        let x: f64 = rand::gen_range(0., screen_width() as f64);
        let y: f64 = rand::gen_range(0., screen_height() as f64);
        dpoints.push(DPoint { x, y });
        vpoints.push(VPoint::new(x, y));
    }

    loop {
        // Clear screen
        clear_background(BLACK);

        // Calculate delaunay
        let delaunay = triangulate(dpoints.as_slice());
        // Calculate voronoi
        let vor_diagram = voronoi(vpoints.clone(), WINDOW_SIZE as f64);
        let vor_polys = make_polygons(&vor_diagram);

        // Draw the points
        for i in 0..dpoints.len() {
            draw_circle(
                dpoints[i].x as f32,
                dpoints[i].y as f32,
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
                    poly[i].x.into_inner() as f32,
                    poly[i].y.into_inner() as f32,
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
                    v0.x.into_inner() * v1.y.into_inner() - v1.x.into_inner() * v0.y.into_inner();
                area += cross;
                centroid.x += ((v0.x.into_inner() + v1.x.into_inner()) * cross) as f32;
                centroid.y += ((v0.y.into_inner() + v1.y.into_inner()) * cross) as f32;
            }
            area /= 2.;
            centroid.x /= (6.0 * area) as f32;
            centroid.y /= (6.0 * area) as f32;
            centroids.push(centroid);
        }

        // Lerp the points to the centroids
        for i in 0..dpoints.len() {
            dpoints[i].x = lerp(
                DPoint {
                    x: dpoints[i].x,
                    y: dpoints[i].y,
                },
                DPoint {
                    x: centroids[i].x as f64,
                    y: centroids[i].y as f64,
                },
                LERP_AMOUNT,
            )
            .x as f64;
            dpoints[i].y = lerp(
                DPoint {
                    x: dpoints[i].x,
                    y: dpoints[i].y,
                },
                DPoint {
                    x: centroids[i].x as f64,
                    y: centroids[i].y as f64,
                },
                LERP_AMOUNT,
            )
            .y as f64;

            // Update vpoints as well
            vpoints[i] = VPoint::new(dpoints[i].x, dpoints[i].y);
        }

        // Update screen
        next_frame().await
    }
}

fn lerp(a: DPoint, b: DPoint, t: f32) -> DPoint {
    DPoint {
        x: a.x + (b.x - a.x) * t as f64,
        y: a.y + (b.y - a.y) * t as f64,
    }
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
