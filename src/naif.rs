use crate::simulation::sim::*;

pub struct Naif {
    data: Vec<Object>,
}


impl Simulator for Naif {
    fn from_objects<I: Iterator<Item = Object>>(objects: I) -> Self {
        Naif { data: Vec::from_iter(objects) }
    }
    fn set_objects<I: Iterator<Item = Object>>(&mut self, objects: I) {
        self.data = Vec::from_iter(objects);
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
                self.data[i].v += f;
                self.data[j].v -= f;
            }
        }

        for o in self.data.iter_mut() {
            o.p += o.v * dt;
        }
    }
}
