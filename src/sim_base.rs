use std::{ops::*, iter::Sum};

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

fn same_ref<T>(a: &T, b: &T) -> bool {
    a as *const T == b as *const T
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, o: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + o.x,
            y: self.y + o.y
        }
    }
}

impl Sum for Vec2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, e| {
            a + e
        }).unwrap_or(Vec2::zero())
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, o: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - o.x,
            y: self.y - o.y
        }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Mul<Self> for Vec2 {
    type Output = f32;
    fn mul(self, o: Self) -> f32 {
        self.x * o.x + self.y + o.y
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, k: f32) -> Vec2 {
        Vec2 {
            x: self.x *k,
            y: self.y *k
        }
    }
}

impl Vec2 {
    pub fn zero() -> Vec2 {
        Vec2 {
            x: 0.,
            y: 0.,
        }
    }

    pub fn norm2(self) -> f32 {
        self.x*self.x + self.y*self.y
    }

    pub fn norm(self) -> f32 {
        self.norm2().sqrt()
    }
}

#[derive(Clone, Debug)]
pub struct Object {
    pub p: Vec2, // position
    pub v: Vec2, // velocity
    pub m: f32, // masse
    pub r: f32, // radius
}

pub fn force_between(a: &Object, b: &Object) -> Vec2 {
    if same_ref(a, b) {
        Vec2::zero();
    }
    let v: Vec2 = b.p - a.p; // vec from i to j
    let d2 = v.norm2();
    if d2.is_normal() {
        let d = d2.sqrt();
        let u = v * (1f32/d);
        let f = u *( a.m * b.m / d2); // force of j on i
        if d > a.r + b.r {
            f
        } else { // repulsion
            f * -1.
        }
    } else {
        Vec2::zero()
    }
}

pub trait Simulator {
    fn get_objects<'a>(&'a self) -> Box<(dyn Iterator<Item = &'a Object> + 'a)>;
    fn len(&self) -> usize;
    fn step(&mut self, dt: f32);
}

pub fn total_energy<S: Simulator>(sim: &S) -> (f32, f32) {
    let mut cinetic_energy = 0.;
    for o in sim.get_objects() {
        cinetic_energy += o.v.norm2() * o.m;
    }
    let mut potential_energy = 0.;
    for a in sim.get_objects() {
        for b in sim.get_objects() {
            potential_energy -= force_between(a, b).norm() * (a.p - b.p).norm();
        }
    }
    potential_energy /= 2.;
    (cinetic_energy, potential_energy)
}
