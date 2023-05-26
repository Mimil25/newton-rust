#![feature(return_position_impl_trait_in_trait)]

mod simulation;
use crate::simulation::sim;
mod naif;
use crate::naif::Naif;

use macroquad::prelude::*;
use macroquad::rand::gen_range;
use simulation::sim::Simulator;

fn draw_simulation<S: sim::Simulator>(sim: &S, color: Color) {
    for o in sim.get_objects() {
        draw_circle(o.p.x, o.p.y, o.r, color);
    }
}

fn handle_camera(cam: &mut Camera2D, old_mouse_pos: &mut Vec2, old_offset: &mut Vec2) {
    let dz = 0.3;
    let (unit, z) = if screen_width() > screen_height() {
        (vec2(screen_height()/screen_width(), 1.), cam.zoom.y)
    } else {
        (vec2(1., screen_width()/screen_height()), cam.zoom.x)
    };
    
    let f = z*(dz*mouse_wheel().1 + 1.);
    cam.zoom = unit * f;
    let m = vec2(-mouse_position_local().x, mouse_position_local().y);
    if is_mouse_button_down(MouseButton::Left) {
        let mut mov = m - *old_mouse_pos;
        mov.x /= cam.zoom.x;
        mov.y /= cam.zoom.y;
        cam.target = *old_offset + mov;
    } else {
        *old_mouse_pos = m;
        *old_offset = cam.target;
    }
    set_camera(cam);
}

#[macroquad::main("Newton")]
async fn main() {


    let mut old_mouse_pos = vec2(0., 0.);
    let mut old_offset = vec2(0., 0.);
    let mut cam2d = Camera2D{
        ..Default::default()
    };
/*
    let mut objects = Vec::new();
    objects.resize_with(2, || {
        sim::Object {
            p: sim::Vec2 {
                x: gen_range(-1., 1.),
                y: gen_range(-1., 1.),
            },
            v: sim::Vec2 {
                x: gen_range(0., 0.4) - 0.2,
                y: gen_range(0., 0.4) - 0.2,
            },
            m: 0.5,
            r: 0.1,
        }
    });
*/
    let objects = vec![
        sim::Object {
            p: sim::Vec2 {
                x: 0.,
                y: 0.,
            },
            v: sim::Vec2 {
                x: 0.,
                y: 0.,
            },
            m: 1.,
            r: 0.2,
        },
        sim::Object {
            p: sim::Vec2 {
                x: 1.,
                y: 0.,
            },
            v: sim::Vec2 {
                x: 0.,
                y: 0.1,
            },
            m: 0.1,
            r: 0.05,
        }
    ];

    let mut sim1 = naif::Naif::from_objects(objects.iter());
    let mut sim2 = naif::Naif::from_objects(objects.iter());
    loop {
        sim1.step(0.01);
        for _ in 0..10 {
            sim2.step(0.001);
        }

        handle_camera(&mut cam2d, &mut old_mouse_pos, &mut old_offset);

        clear_background(BLACK);

        draw_simulation(&sim1, GREEN);
        draw_simulation(&sim2, RED);

        set_default_camera();

        let (ce, pe) = sim::total_energy(&sim1);
        draw_text(format!("Ec = {}",ce).as_str(), 10., 10., 20., GREEN);
        draw_text(format!("Ep = {}",pe).as_str(), 10., 30., 20., GREEN);
        draw_text(format!("E = {}",ce + pe).as_str(), 10., 50., 20., GREEN);

        let (ce, pe) = sim::total_energy(&sim2);
        draw_text(format!("Ec = {}",ce).as_str(), 10., 70., 20., RED);
        draw_text(format!("Ep = {}",pe).as_str(), 10., 90., 20., RED);
        draw_text(format!("E = {}",ce + pe).as_str(), 10., 110., 20., RED);

        next_frame().await
    }
}
