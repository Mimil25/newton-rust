use crate::sim_base::*;

pub struct EObject {
    o: Object,
    a: Vec2,
}

pub struct Euler {
    data: Vec<EObject>,
}

impl<I: Iterator<Item = Object>> From<I> for Euler {
    fn from(value: I) -> Self {
        Euler {
            data: Vec::from_iter(value.map(|o| {
                EObject {
                    o,
                    a: Vec2::zero(),
                }
            })),
        }
    }
}

impl Simulator for Euler {
    fn get_objects<'a>(&'a self) -> Box<(dyn Iterator<Item = &'a Object> + 'a)> {
        Box::new(self.data.iter().map(|o| {
            &o.o
        }))
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn step(&mut self, dt: f32) {
        for o in self.data.iter_mut(){
            o.a = Vec2::zero();
        };
        for i in 0..(self.data.len()-1) {
            for j in (i+1)..self.data.len() {
                let f = force_between(&self.data[i].o, &self.data[j].o);
                let inertia = 1. / self.data[i].o.m;
                self.data[i].a += f * inertia;
                let inertia = 1. / self.data[j].o.m;
                self.data[j].a -= f * inertia;
            }
        }

        for o in self.data.iter_mut() {
            o.o.p += o.o.v * dt;
        }
        for o in self.data.iter_mut() {
            o.o.v += o.a * dt;
        }
    }
}
