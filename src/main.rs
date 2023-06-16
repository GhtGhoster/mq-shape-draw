
use std::f64::consts::TAU;

use macroquad::prelude::*;
use point::PathPoint;
use shape::Shape;
use usdf::{CircleSegment, LineSegment};

mod path;
mod point;
mod usdf;
mod shape;

#[macroquad::main("mq-shape-draw")]
async fn main() {
    // debugging
    // std::env::set_var("RUST_BACKTRACE", "1");

    let mut shape: Shape = Shape::new();
    let circle_usdf: CircleSegment = CircleSegment {
        center: PathPoint {x: 0.5, y: 0.5},
        radius: 0.5,
        facing_angle: 0.0,
        angle_spread: TAU,
    };
    let line_usdf: LineSegment = LineSegment {
        start_point: PathPoint {x: 0.0, y: 0.0},
        end_point: PathPoint {x: 1.0, y: 1.0},
    };
    shape.usdfs.push(Box::new(circle_usdf));
    shape.usdfs.push(Box::new(line_usdf));

    loop {
        // ui
        // egui_macroquad::ui(|egui_ctx| {
        //     egui::Window::new("Hello")
        //         .show(egui_ctx, |ui| {
        //             ui.label("World!");
        //         }
        //     );
        // });

        // rendering
        clear_background(BLACK);
        let (x, y) = mouse_position();
        let col = shape.score(PathPoint::from_mouse_pos()) as u8;
        if col != 0 {
            println!("omg!");
        }
        draw_circle(x, y, 100.0, Color::from_rgba(col, col, col, 255));

        // egui_macroquad::draw();
        next_frame().await
    }
}
