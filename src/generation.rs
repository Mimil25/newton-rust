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
    (0..1).map(move |_| {
        sim_base::Object {
            p: sim_base::Vec2::zero(),
            v: sim_base::Vec2::zero(),
            m: (n as f32),
            r: (n as f32).sqrt()/10.,
        }
    }).chain((1..(n+1)).map(move |i| {
        let mut o = sim_base::Object {
            p: sim_base::Vec2::zero(),
            v: sim_base::Vec2::zero(),
            m: 0.00001,
            r: 0.1
        };
        circle_orbit(sim_base::Vec2::zero(), n as f32, &mut o, i as f32, 1.);
        o
    }))
}
