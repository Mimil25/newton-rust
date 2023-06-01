use crate::sim_base::*;

struct RK4Object {
    o: Object,
    a: Vec2
}

pub struct RungeKutta4 {
    data: Vec<RK4Object>,
}

impl<I: Iterator<Item = Object>> From<I> for RungeKutta4 {
    fn from(value: I) -> Self {
        RungeKutta4 {
            data: Vec::from_iter(value.map(|o| {
                RK4Object {
                    o,
                    a: Vec2::zero(),
                }
            })),
        }
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
        for o in self.data.iter_mut(){
            o.a = Vec2::zero();
        };
        for i in 0..(self.data.len()-1) {
            for j in (i+1)..self.data.len() {
                let a = &self.data[i];
                let b = &self.data[j];
                
                // TODO

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
