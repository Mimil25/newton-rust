
pub mod sim {
    use std::ops::*;
    
    #[derive(Clone, Copy)]
    pub struct Vec2 {
        x: f32,
        y: f32,
    }

    impl AddAssign for Vec2 {
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
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
        pub fn norm2(self) -> f32 {
            self.x*self.x + self.y*self.y
        }

        pub fn norm(self) -> f32 {
            self.norm2().sqrt()
        }
    }

    pub struct Object {
        pub p: Vec2, // position
        pub v: Vec2, // velocity
        pub m: f32, // masse
    }

    pub trait Simulator {
        fn from_objects<I: Iterator<Item = Object>>(objects: I) -> Self;
        fn set_objects<I: Iterator<Item = Object>>(&mut self, objects: I);
        fn get_objects<'a>(&'a self) -> impl Iterator<Item = &'a Object>;
        fn step(&mut self, dt: f32);
    }
}
