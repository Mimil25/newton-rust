use crate::simulation::sim::*;

pub struct Naif {
    data: Vec<Object>,
}


impl Simulator for Naif {
    fn from_objects<'a, I: Iterator<Item = &'a Object>>(objects: I) -> Self {
        Naif { data: Vec::from_iter(objects.map(|o| {
            (*o).to_owned()
        }))}
    }
    fn get_objects<'a>(&'a self) -> std::slice::Iter<'a, Object> {
        self.data.iter()
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
            o.p += o.v * dt
        }
    }
}
