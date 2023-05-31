use crate::sim_base;
use crate::naif;
use crate::runge_kutta;

pub enum AnySim {
    Naif(naif::Naif),
    RungeKutta(runge_kutta::RungeKutta),
}

impl<'a, I: Iterator<Item = sim_base::Object>> TryFrom<(&str, I)> for AnySim {
    type Error = String;
    fn try_from(value: (&str, I)) -> Result<Self, Self::Error> {
        match value.0 {
            "naif" => Ok(Self::Naif(naif::Naif::from(value.1))),
            "runge_kutta" => Ok(Self::RungeKutta(runge_kutta::RungeKutta::from(value.1))),
            _ => Err("can't create simulator".to_string())
        }
    }
}

impl sim_base::Simulator for AnySim {
    fn len(&self) -> usize {
        match self {
            Self::Naif(sim) => sim.len(),
            Self::RungeKutta(sim) => sim.len(),
        }
    }
    fn step(&mut self, dt: f32) {
        match self {
            Self::Naif(sim) => sim.step(dt),
            Self::RungeKutta(sim) => sim.step(dt),
        }
    }
    fn get_objects<'a>(&'a self) -> Box<(dyn Iterator<Item = &'a sim_base::Object> + 'a)> {
        match self {
            Self::Naif(sim) => sim.get_objects(),
            Self::RungeKutta(sim) => sim.get_objects(),
        } 
    }
}
