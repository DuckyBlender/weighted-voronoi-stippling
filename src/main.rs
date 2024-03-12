use delaunator::{triangulate, Point as DPoint};
use macroquad::prelude::*;
use voronoi::{voronoi, make_polygons, Point as VPoint};

const POINT_SIZE: f32 = 1.0;
const POINT_COLOR: Color = BLUE;

const CENTROID_SIZE: f32 = 3.0;
const CENTROID_COLOR: Color = GREEN;

const LINE_THICKNESS: f32 = 1.0;
const LINE_COLOR: Color = WHITE;

const AMOUNT: i32 = 100;
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
    let mut dpoints = Vec::new();
    let mut vpoints = Vec::new();
    // TODO: Make this more efficient
    for _ in 0..AMOUNT {
        let x: f64 = rand::gen_range(0., screen_width() as f64);
        let y: f64 = rand::gen_range(0., screen_height() as f64);
        dpoints.push(DPoint {x, y} );
        vpoints.push(VPoint::new(x, y) );
    }

    // Calculate delaunay
    let delaunay = triangulate(dpoints.as_slice());
    // Calculate voronoi
    let vor_diagram = voronoi(vpoints, WINDOW_SIZE as f64);
    let vor_polys = make_polygons(&vor_diagram);

    loop {
        // Clear screen
        clear_background(BLACK);

        // Draw the points
        for i in 0..dpoints.len() {
            draw_circle(dpoints[i].x as f32, dpoints[i].y as f32, POINT_SIZE, POINT_COLOR);
        }

        // Draw the voronoi polygons
        for poly in vor_polys.iter() {
            // Begin creating a shape
            let mut shape = Vec::new();
            for i in 0..poly.len() {
                shape.push(vec2(poly[i].x.into_inner() as f32, poly[i].y.into_inner() as f32));
            }
            // Draw the shape
            // draw_poly_lines(x, y, sides, radius, rotation, thickness, color)
            draw_custom_shape(shape, LINE_COLOR, LINE_THICKNESS);
        }

        // Draw the delaunay triangles

        // Update screen
        next_frame().await
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