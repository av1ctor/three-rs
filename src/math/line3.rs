use serde::{Deserialize, Serialize};
use super::vector3::Vector3;

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
}