
use macroquad::prelude::*;
use path::DrawPath;
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
    // the matches are pretty wrong a most of the time
    // shapes like water and earth where earth just has one more line will
    // score the same when perfect water is drawn
    //
    // reverse score calculation of each shape point to the nearest draw path point?
    // is gonna have to be discretized and eh.. shouldn't be too hard tho hopefully.

    // todo: for something final as a result of this demo
    // it would be nice to check path standards and implement them
    // along with distance from path (for score)
    // and even step through path (for reverse score)
    //
    // the best fit would seemingly be Bezier curves, todo:
    // determine an even step-through a bezier curve - seemingly easy
    // determine distance from point to bezier curve - iterative approach
    // can't do circles tho
    // rewatch video on continuity of curves/splines by that one cool math girl on yt
    //
    // this should probably be on top of the line and circle segments as they
    // are simpler to operate and not as computationally intensive

    let shapes = vec![
        Shape::shape_water(),
        Shape::shape_fire(),
        Shape::shape_earth(),
        Shape::shape_air(),
        Shape::shape_lock(),
        Shape::shape_devil(),
        Shape::shape_circle(),
    ];
    let shape_names = vec![
        "water",
        "fire",
        "earth",
        "air",
        "lock",
        "devil",
        "circle",
    ];
    let mut min_index: usize = 0;
    let mut shape_scores = vec![0f64; shapes.len()];

    let mut render_path = true;
    let mut render_domain = true;
    let mut render_shape = true;
    let mut render_corners = true;
    let mut render_steps = true;
    let mut render_steps_amount: usize = 10;
    let mut shape_index: usize = 4;

    let mut path = DrawPath::new();

    loop {
        let mouse_pathpoint = PathPoint::from_mouse_pos();
        if is_mouse_button_down(MouseButton::Left) {
            path.push(mouse_pathpoint);
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            let mut min = f64::MAX;
            for i in 0..shapes.len() {
                shape_scores[i] = path.score(&shapes[i]);
                if shape_scores[i] < min {
                    min_index = i;
                    min = shape_scores[i]
                }
            }
            path = DrawPath::new();
        }

        // ui
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Info")
                .show(egui_ctx, |ui| {
                    ui.label("Left click to draw");
                    ui.label("Right click to score");
                    ui.label("The lower the score, the better");
                    ui.label(format!("FPS: {}", get_fps()));
                    ui.label(format!("MousePos: {}", mouse_pathpoint));
                    ui.label(format!("Score: {:.3}", shapes[shape_index].score(mouse_pathpoint)));
                }
            );
            egui::Window::new("Render")
                .show(egui_ctx, |ui| {
                    ui.checkbox(&mut render_path, "Render path");
                    ui.checkbox(&mut render_domain, "Render domain");
                    ui.checkbox(&mut render_shape, "Render shape");
                    ui.checkbox(&mut render_corners, "Render corners");
                    ui.checkbox(&mut render_steps, "Render steps");
                    ui.add(egui::Slider::new(& mut render_steps_amount, 2..=50));
                    ui.horizontal(|ui| {
                        ui.label("Current shape:");
                        ui.code(shape_names[shape_index]);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Prev").clicked() {
                            shape_index = (shape_index - 1 + shapes.len()).rem_euclid(shapes.len());
                        }
                        if ui.button("Next").clicked() {
                            shape_index = (shape_index + 1).rem_euclid(shapes.len());
                        }
                    });
                }
            );
            egui::Window::new("Score")
                .show(egui_ctx, |ui| {
                    ui.label(format!("Best match: {} ({:.3})", shape_names[min_index], shape_scores[min_index]));
                    for i in 0..shapes.len() {
                        ui.label(format!("{}: \t{:.5}", shape_names[i], shape_scores[i]));
                    }
                }
            );
        });

        // rendering
        clear_background(BLACK);

        if render_shape {
            for x in 0..(screen_width()/8.0) as u32 {
                for y in 0..(screen_height()/8.0) as u32 {
                    let (x, y) = (x as f32, y as f32);
                    let cell_point = PathPoint::from_screenspace(x*8.0+4.0, y*8.0+4.0);
                    let score = (shapes[shape_index].score(cell_point) * 255.0) as u8;
                    draw_rectangle(x*8.0, y*8.0, 8.0, 8.0, Color::from_rgba(score, score, score, 255));
                }
            }
        }

        if render_steps {
            for usdf in &shapes[shape_index].usdfs {
                for usdf_point in usdf.step_through(render_steps_amount) {
                    let mut point = usdf_point.clone();
                    if let Some(domain) = shapes[shape_index].domain {
                        point = point.lerp_to_normalized_domain(domain);
                    }
                    point = point.lerp_from_normalized_domain(PathPoint::screen_domain());
                    let PathPoint {x, y} = point;
                    draw_circle(x as f32, y as f32, 5.0, GREEN);
                }
            }
        }

        if render_corners {
            for usdf in &shapes[shape_index].usdfs {
                for usdf_point in usdf.get_corners() {
                    let mut point = usdf_point.clone();
                    if let Some(domain) = shapes[shape_index].domain {
                        point = point.lerp_to_normalized_domain(domain);
                    }
                    point = point.lerp_from_normalized_domain(PathPoint::screen_domain());
                    let PathPoint {x, y} = point;
                    draw_circle(x as f32, y as f32, 5.0, BLUE);
                }
            }
        }

        if let Some((min, max)) = path.domain {
            if render_domain {
                let PathPoint{x, y} = min.lerp_from_normalized_domain(PathPoint::screen_domain());
                let PathPoint{x: w, y: h} = (max-min).lerp_from_normalized_domain(PathPoint::screen_domain());
                draw_rectangle_lines(x as f32, y as f32, w as f32, h as f32, 1.0, BLUE);
            }

            if render_path {
                for i in 0..path.points.len()-1 {
                    let PathPoint{x: x1, y: y1} = path.points[i].lerp_from_normalized_domain(PathPoint::screen_domain());
                    let PathPoint{x: x2, y: y2} = path.points[i+1].lerp_from_normalized_domain(PathPoint::screen_domain());
                    draw_line(x1 as f32, y1 as f32, x2 as f32, y2 as f32, 1.0, RED);
                }
            }
        }

        egui_macroquad::draw();
        next_frame().await
    }
}
