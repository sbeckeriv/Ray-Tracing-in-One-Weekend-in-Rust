extern crate nalgebra;
extern crate nalgebra as na;
use nalgebra::Vec3;
use nalgebra::Absolute;
use objects::HitableDirection;
#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3<f32>,
    pub direction: Vec3<f32>,
    pub time: f32,
    pub debug: bool,
}

impl Ray {
    pub fn set_debug(&mut self) {
        self.debug = true;
    }
    pub fn new(origin: Vec3<f32>, direction: Vec3<f32>, time: f32) -> Self {
        Ray {
            origin: origin,
            direction: direction,
            time: time,
            debug: false,
        }
    }
    pub fn point_at_parameter(&self, t: f32) -> Vec3<f32> {
        // t should be whole numbers -1 0 1 2..
        self.origin + (self.direction * t)
    }

    pub fn closer(&self,
                  t: f32,
                  left: (Vec3<f32>, Vec3<f32>),
                  right: (Vec3<f32>, Vec3<f32>))
                  -> HitableDirection {
        let local = self.point_at_parameter(t);
        if Absolute::abs(&(local - left.0)).len() - Absolute::abs(&(right.1 - local)).len() <= 0 {
            HitableDirection::Left
        } else {
            HitableDirection::Right
        }
    }
}
