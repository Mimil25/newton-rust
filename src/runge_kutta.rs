use crate::sim_base::*;

pub struct RungeKutta {
    data: Vec<Object>,
}

impl<I: Iterator<Item = Object>> From<I> for RungeKutta {
    fn from(value: I) -> Self {
        RungeKutta {
            data: Vec::from_iter(value),
        }
    }
}

impl Simulator for RungeKutta {
    fn get_objects<'a>(&'a self) -> Box<(dyn Iterator<Item = &'a Object> + 'a)> {
        Box::new(self.data.iter())
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn step(&mut self, dt: f32) {
        for i in 0..(self.data.len()-1) {
            for j in (i+1)..self.data.len() {
                let f = force_between(&self.data[i], &self.data[j]) * dt;
                let inertia = 1. / self.data[i].m;
                self.data[i].v += f * inertia;
                let inertia = 1. / self.data[j].m;
                self.data[j].v -= f * inertia;
            }
        }

        for o in self.data.iter_mut() {
            o.p += o.v * dt;
        }
    }
}
