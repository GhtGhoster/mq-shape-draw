
use std::f64::consts::PI;

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

    // todo: don't match shapes with a lot of black space left out somehow?
    // subtractive shapes that prevent activation and are subsets of existing shapes
    // automatically generate when creating a shape
    // math is gonna be tought cuz smaller shapes will have a better score

    let mut shape: Shape = Shape::new();
    let circle_usdf: CircleSegment = CircleSegment {
        center: PathPoint::new(0.5, 0.5),
        radius: 0.5,
        facing_angle: 0.1,
        angle_spread: PI/2.0,
    };
    let line_usdf: LineSegment = LineSegment {
        start_point: PathPoint::new(0.0, 0.0),
        end_point: PathPoint::new(1.0, 1.0),
    };

    // println!("{}", circle_usdf.distance(PathPoint::new(0.0, 0.0)));
    // panic!();

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
        let (mouse_x, mouse_y) = mouse_position();

        for x in 0..(screen_width()/8.0) as u32 {
            for y in 0..(screen_height()/8.0) as u32 {
                let (x, y): (f32, f32) = (x as f32, y as f32);
                let cell_point = PathPoint::from_screenspace(x*8.0+4.0, y*8.0+4.0);
                let score = (shape.score(cell_point) * 255.0) as u8;
                draw_rectangle(x*8.0, y*8.0, 8.0, 8.0, Color::from_rgba(score, score, score, 255));

                // if (x..x+8.0).contains(&mouse_x) && (y..y+8.0).contains(&mouse_y) {

                // }
            }
        }

        // draw_line(0.3*screen_width(), 0.3*screen_height(), 0.7*screen_width(), 0.7*screen_height(), 1.0, RED);
        let mouse_pathpoint = PathPoint::from_mouse_pos();
        draw_text(format!("FPS: {:?}", get_fps()).as_str(), 0.0, 20.0, 20.0, BLUE);
        draw_text(format!("{:?}", mouse_pathpoint).as_str(), 0.0, 40.0, 20.0, BLUE);
        draw_text(format!("Score: {:?}", shape.score(mouse_pathpoint)).as_str(), 0.0, 60.0, 20.0, BLUE);

        // egui_macroquad::draw();
        next_frame().await
    }
}
