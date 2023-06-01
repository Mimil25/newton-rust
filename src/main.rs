#![feature(return_position_impl_trait_in_trait)]

mod sim_base;
mod generation;
mod sim;
mod naif;
mod euler;
mod runge_kutta;
use macroquad::prelude::*;
use sim_base::Simulator;

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

struct Config {
    sim: sim::AnySim,
    color: Color,
    base_energy: f32,
    cinetic_energy: f32,
    potential_energy: f32,
}

impl Config {
    fn draw_bodys(&self) {
        for o in self.sim.get_objects() {
            draw_circle(o.p.x, o.p.y, o.r, self.color);
        }
    }
    fn draw_energy(&self, x: f32, y: f32, font_size: f32) {
        draw_text(format!("Ec   = {:^+010.}", self.cinetic_energy).as_str(), x, y, font_size, self.color);
        draw_text(format!("Ep   = {:^+010.}", self.base_energy + self.potential_energy).as_str(), x, y+font_size, font_size, self.color);
        draw_text(format!("E    = {:>+010.}", self.cinetic_energy + self.potential_energy).as_str(), x, y+font_size*2., font_size, self.color);
        draw_text(format!("base E={:>+010.}", self.base_energy).as_str(), x, y+font_size*3., font_size, self.color);
    }
    fn calc_energy(&mut self) {
        (self.cinetic_energy, self.potential_energy) = sim_base::total_energy(&self.sim);
    }
    fn calc_base_energy(&mut self) {
        self.calc_energy();
        self.base_energy = self.cinetic_energy + self.potential_energy;
    }
}

#[macroquad::main("Newton")]
async fn main() {
    let mut old_mouse_pos = vec2(0., 0.);
    let mut old_offset = vec2(0., 0.);
    let mut cam2d = Camera2D{
        ..Default::default()
    };

    let mut conf1 = Config {
        sim: sim::AnySim::try_from(("naif", generation::circles(10))).unwrap(),
        color: GREEN,
        base_energy:0.,
        cinetic_energy:0.,
        potential_energy:0.,
    };

    let mut conf2 = Config {
        sim: sim::AnySim::try_from(("runge_kutta", generation::circles(10))).unwrap(),
        color: RED,
        base_energy:0.,
        cinetic_energy:0.,
        potential_energy:0.,
    };

    conf1.calc_base_energy();
    conf2.calc_base_energy();

    loop {
        let dt = get_frame_time() * 0.3;
        conf1.sim.step(dt);
        conf2.sim.step(dt);
        
        handle_camera(&mut cam2d, &mut old_mouse_pos, &mut old_offset);
        
        clear_background(BLACK);

        conf1.draw_bodys();
        conf2.draw_bodys();

        set_default_camera();

        conf1.calc_energy();
        conf1.draw_energy(10., 10., 20.);
        conf2.calc_energy();
        conf2.draw_energy(10., 100., 20.);

        next_frame().await
    }
}
