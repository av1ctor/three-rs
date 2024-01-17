use std::f32::consts::PI;

use super::Cylinder;


pub struct Cone {
}

impl Cone {
    pub fn new_ex(
        radius: f32, 
        height: f32,
        radial_segments: usize, 
        height_segments: usize, 
        open_ended: bool, 
        theta_start: f32,
        theta_length: f32
    ) -> Cylinder {
        Cylinder::new_ex(
            0.0, radius, 
            height, 
            radial_segments, height_segments, 
            open_ended, 
            theta_start, theta_length
        )
    }

    pub fn new(
        radius: f32,
        height: f32
    ) -> Cylinder {
        Self::new_ex(
            radius, 
            height, 
            32, 
            1, 
            false, 
            0.0, 
            PI * 2.0
        )
    }
}