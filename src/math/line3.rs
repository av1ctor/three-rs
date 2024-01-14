use serde::{Deserialize, Serialize};
use super::Vector3;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Line3 {
    pub start: Vector3,
    pub end: Vector3,
}

impl Line3 {
    pub fn new(
        start: Vector3,
        end: Vector3
    ) -> Self {
        Self {
            start,
            end,
        }
    }

    pub fn distance_to_point(
        &self,
        p: &Vector3
    ) -> (
        Vector3, 
        f32, 
        f32
    ) {
        let a = p.sub(&self.start);
        let b = self.end.sub(&self.start);

        let dot = a.dot(&b);
        let len_sq = b.length_sq();
        let t = if len_sq != 0.0 {dot / len_sq} else {-1.0};

        let inter = if t < 0.0 {
            self.start.clone()
        }
        else if t > 1.0 {
            self.end.clone()
        }
        else {
            self.start.add(&b.mul_scalar(t))
        };

        let delta = p.sub(&inter);
        (
            delta, 
            delta.length(), 
            t
        )
    }
}