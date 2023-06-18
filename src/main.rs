
use macroquad::prelude::*;
use point::PathPoint;
use shape::Shape;

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
    // PS - maybe it's not gonna be that problematic since the drawn path is going
    // to be scaled anyway, preventing smaller drawn shapes from matching very well
    // some verification should probably be in place tho later down the line

    // todo: keep in mind that these predefined spell shapes need to span the full
    // domain (<0-1> as of now, maybe <-1-1> in the future), not just a part of it
    // this is going to pose problems as there is no elipsoid USDF yet
    // (for later use in circular shapes spread to the whole domain)
    // maybe this can be fixed by adding a min/max x/y property to USDF?

    let shapes = vec![
        Shape::shape_water(),
        Shape::shape_fire(),
        Shape::shape_earth(),
        Shape::shape_air(),
        Shape::shape_lock(),
        Shape::shape_devil(),
    ];
    let shape_names = vec![
        "water",
        "fire",
        "earth",
        "air",
        "lock",
        "devil",
    ];
    let mut shape_index: usize = 0;

    loop {
        let mouse_pathpoint = PathPoint::from_mouse_pos();

        // ui
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Shape draw demo")
                .show(egui_ctx, |ui| {
                    ui.label(format!("FPS: {}", get_fps()));
                    ui.label(format!("MousePos: {}", mouse_pathpoint));
                    ui.label(format!("Score: {:.3}", shapes[shape_index].score(mouse_pathpoint)));
                    ui.horizontal(|ui| {
                        ui.label("Current shape:");
                        ui.code(shape_names[shape_index]);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Prev").clicked() {
                            shape_index = (shape_index - 1).rem_euclid(shapes.len());
                        }
                        if ui.button("Next").clicked() {
                            shape_index = (shape_index + 1).rem_euclid(shapes.len());
                        }
                    });
                }
            );
        });

        // rendering
        clear_background(BLACK);

        for x in 0..(screen_width()/8.0) as u32 {
            for y in 0..(screen_height()/8.0) as u32 {
                let (x, y): (f32, f32) = (x as f32, y as f32);
                let cell_point = PathPoint::from_screenspace(x*8.0+4.0, y*8.0+4.0);
                let score = (shapes[shape_index].score(cell_point) * 255.0) as u8;
                draw_rectangle(x*8.0, y*8.0, 8.0, 8.0, Color::from_rgba(score, score, score, 255));
            }
        }
        egui_macroquad::draw();
        next_frame().await
    }
}
