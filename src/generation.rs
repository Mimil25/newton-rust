use crate::sim_base;

fn circle_orbit(center: sim_base::Vec2, m: f32, b: &mut sim_base::Object, r: f32, alpha: f32) {
    let sin = alpha.sin();
    let cos = alpha.cos();
    b.p.x = center.x + r*cos;
    b.p.y = center.y + r*sin;
    let v = (m/r).sqrt();
    b.v.x = -v * sin;
    b.v.y = v * cos;
}

pub fn circles(n: u32) -> impl Iterator<Item = sim_base::Object> + 'static {
    let center_mass = n as f32;
    let center_radius = center_mass.sqrt() / 10.;
    (0..1).map(move |_| {
        sim_base::Object {
            p: sim_base::Vec2::zero(),
            v: sim_base::Vec2::zero(),
            m: center_mass,
            r: center_radius,
        }
    }).chain((1..(n+1)).map(move |i| {
        let mut o = sim_base::Object {
            p: sim_base::Vec2::zero(),
            v: sim_base::Vec2::zero(),
            m: 0.1,
            r: 0.1
        };
        circle_orbit(
            sim_base::Vec2::zero(),
            center_mass,
            &mut o,
            center_radius + i as f32,
            (i*53) as f32
        );
        o
    }))
}
