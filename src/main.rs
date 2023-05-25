#![feature(return_position_impl_trait_in_trait)]

mod simulation;
use crate::simulation::sim;
mod naif;
use crate::naif::Naif;

use macroquad::prelude::*;
use macroquad::prelude::camera::mouse::Camera;

fn draw_simulation<S: sim::Simulator>(sim: S) {
}

fn handle_camera(cam: &mut Camera2D, old_mouse_pos: &mut Vec2, old_offset: &mut Vec2) {
    let dz = 0.3;
    let (unit, z) = if screen_width() > screen_height() {
        (vec2(screen_height()/screen_width(), 1.), cam.zoom.y)
    } else {
        (vec2(1., screen_width()/screen_height()), cam.zoom.x)
    };
    
    let m = vec2(mouse_position_local().x, - mouse_position_local().y);
    if is_mouse_button_down(MouseButton::Left) {
        cam.offset = *old_offset + m - *old_mouse_pos;
    } else {
        *old_mouse_pos = m;
        *old_offset = cam.offset;
    }

    cam.zoom = unit * (z*(dz*mouse_wheel().1 + 1.));
    set_camera(cam);
}

#[macroquad::main("Newton")]
async fn main() {


    let mut old_mouse_pos = vec2(0., 0.);
    let mut old_offset = vec2(0., 0.);
    let mut cam2d = Camera2D{
        ..Default::default()
    };

    loop {

        handle_camera(&mut cam2d, &mut old_mouse_pos, &mut old_offset);

        clear_background(BLACK);

        draw_circle(0., 0., 1., RED);
        draw_circle(1., 1., 0.5, BLUE);

        next_frame().await
    }
}
