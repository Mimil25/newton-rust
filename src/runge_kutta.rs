use crate::sim_base::*;

#[derive(Debug)]
struct RK4Object {
    id: u32,
    o: Object,
    dp: Vec2,
}

pub struct RungeKutta4 {
    data: Vec<RK4Object>,
}

impl<I: Iterator<Item = Object>> From<I> for RungeKutta4 {
    fn from(value: I) -> Self {
        RungeKutta4 {
            data: Vec::from_iter(value.enumerate().map(|o| {
                RK4Object {
                    id: o.0 as u32,
                    o: o.1,
                    dp: Vec2::zero(),
                }
            })),
        }
    }
}

impl RungeKutta4 {
    fn acceleration(&self, a: &RK4Object) -> Vec2 {
        self.data.iter()
            .filter(|b| {a.id != b.id})
            .map(|b| {force_between(&a.o, &b.o)})
            .sum()
    }
}

impl Simulator for RungeKutta4 {
    fn get_objects<'a>(&'a self) -> Box<(dyn Iterator<Item = &'a Object> + 'a)> {
        Box::new(self.data.iter().map(|o| {
            &o.o
        }))
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn step(&mut self, dt: f32) {
        // http://spiff.rit.edu/richmond/nbody/OrbitRungeKutta4.pdf
        for i in 0..self.data.len() {
            let a = &self.data[i];
            let k1p = a.o.v;
            let k1v : Vec2 = self.acceleration(a);

            let k2p = a.o.v + k1v*(dt*0.5);
            let mut tmp = RK4Object {
                id: a.id,
                o:Object {
                    p: a.o.p + k1p * (dt*0.5),
                    v: Vec2::zero(), // not used
                    m: a.o.m,
                    r: a.o.r,
                },
                dp: Vec2::zero(), // not used
            };
            let k2v : Vec2 = self.acceleration(&tmp);
            let k3p = a.o.v + k2v*(dt*0.5);
            tmp.o.p = a.o.p + k2p * (dt*0.5);
            let k3v : Vec2 = self.acceleration(&tmp);

            let k4p = a.o.v + k3v * dt;
            tmp.o.p = a.o.p + k3p * dt;
            let k4v : Vec2 = self.acceleration(&tmp);

            let m = a.o.m;
            self.data[i].o.v += (k1v + k2v+k2v + k3v+k3v + k4v) * (dt / (6.*m));
            self.data[i].dp = (k1p + k2p+k2p + k3p+k3p + k4p) * (dt / 6.);

        }

        for o in self.data.iter_mut() {
            o.o.p += o.dp;
        }
    }
}
