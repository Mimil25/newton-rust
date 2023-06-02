use crate::sim_base::*;

pub struct RungeKutta4 {
    data: Vec<Object>,
}

impl<I: Iterator<Item = Object>> From<I> for RungeKutta4 {
    fn from(value: I) -> Self {
        RungeKutta4 {
            data: Vec::from_iter(value),
        }
    }
}

impl RungeKutta4 {
    fn acceleration(&self, a: &Object, id: usize) -> Vec2 {
        self.data.iter()
            .enumerate()
            .filter(|b| {id != b.0})
            .map(|b| {force_between(&a, &b.1)})
            .sum()
    }
}

impl Simulator for RungeKutta4 {
    fn get_objects<'a>(&'a self) -> Box<(dyn Iterator<Item = &'a Object> + 'a)> {
        Box::new(self.data.iter())
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn step(&mut self, dt: f32) {
        // http://spiff.rit.edu/richmond/nbody/OrbitRungeKutta4.pdf

        let k1p: Vec<Vec2> = self.data.iter().map(|o| {o.v}).collect();
        let k1v: Vec<Vec2> = self.data.iter()
            .enumerate()
            .map(|(id, o)| {
                self.acceleration(o, id) * o.m.recip()
            })
            .collect();

        let ddt = dt / 2.;
        let k2p: Vec<Vec2> = k1p.iter()
            .zip(k1v.iter())
            .map(|(v, k1v)| {
                *v + *k1v * ddt
            })
            .collect();
        let k2v: Vec<Vec2> = self.data.iter()
            .zip(k1p.iter())
            .zip(k1v.iter())
            .map(|((o, k1p), k1v)| {
                Object {
                    p: o.p + *k1p * ddt,
                    v: o.v + *k1v * ddt,
                    m: o.m,
                    r: o.r,
                }
            })
            .enumerate()
            .map(|(id, o)| {
                self.data.iter()
                    .zip(k1p.iter())
                    .zip(k1v.iter())
                    .enumerate()
                    .filter(|(id2, _)| {id != *id2})
                    .map(|(_id, ((o, k1p), k1v))| {
                        Object {
                            p: o.p + *k1p * ddt,
                            v: o.v + *k1v * ddt,
                            m: o.m,
                            r: o.r,
                        }
                    })
                    .map(|b| {
                        force_between(&o, &b)
                    })
                    .sum::<Vec2>() * o.m.recip()
            })
            .collect();

        let k3p: Vec<Vec2> = k1p.iter()
            .zip(k2v.iter())
            .map(|(v, k2v)| {
                *v + *k2v * ddt
            })
            .collect();
        let k3v: Vec<Vec2> = self.data.iter()
            .zip(k2p.iter())
            .zip(k2v.iter())
            .map(|((o, k2p), k2v)| {
                Object {
                    p: o.p + *k2p * ddt,
                    v: o.v + *k2v * ddt,
                    m: o.m,
                    r: o.r,
                }
            })
            .enumerate()
            .map(|(id, o)| {
                self.data.iter()
                    .zip(k2p.iter())
                    .zip(k2v.iter())
                    .enumerate()
                    .filter(|(id2, _)| {id != *id2})
                    .map(|(_id, ((o, k2p), k2v))| {
                        Object {
                            p: o.p + *k2p * ddt,
                            v: o.v + *k2v * ddt,
                            m: o.m,
                            r: o.r,
                        }
                    })
                    .map(|b| {
                        force_between(&o, &b)
                    })
                    .sum::<Vec2>() * o.m.recip()
            })
            .collect();

        let k4p: Vec<Vec2> = k1p.iter()
            .zip(k3v.iter())
            .map(|(v, k3v)| {
                *v + *k3v * dt
            })
            .collect();
        let k4v: Vec<Vec2> = self.data.iter()
            .zip(k3p.iter())
            .zip(k3v.iter())
            .map(|((o, k3p), k3v)| {
                Object {
                    p: o.p + *k3p * dt,
                    v: o.v + *k3v * dt,
                    m: o.m,
                    r: o.r,
                }
            })
            .enumerate()
            .map(|(id, o)| {
                self.data.iter()
                    .zip(k3p.iter())
                    .zip(k3v.iter())
                    .enumerate()
                    .filter(|(id2, _)| {id != *id2})
                    .map(|(_id, ((o, k3p), k3v))| {
                        Object {
                            p: o.p + *k3p * dt,
                            v: o.v + *k3v * dt,
                            m: o.m,
                            r: o.r,
                        }
                    })
                    .map(|b| {
                        force_between(&o, &b)
                    })
                    .sum::<Vec2>() * o.m.recip()
            })
            .collect();

        let dp = k1p.into_iter()
            .zip(k2p)
            .zip(k3p)
            .zip(k4p)
            .map(|(((k1, k2), k3), k4)| {
                (k1 + k2+k2 + k3+k3 + k4) * (dt/6.)
            });
        let dv = k1v.into_iter()
            .zip(k2v)
            .zip(k3v)
            .zip(k4v)
            .map(|(((k1, k2), k3), k4)| {
                (k1 + k2+k2 + k3+k3 + k4) * (dt/6.)
            });
        for ((o, dp), dv) in self.data.iter_mut().zip(dp).zip(dv) {
            o.p += dp;
            o.v += dv;
        }
    }
}
