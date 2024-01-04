use super::{vector3::Vector3, line3::Line3};

#[derive(Clone, Default)]
pub struct Path3 {
    pub nodes: Vec<Vector3>,
}

impl Path3 {
    pub fn new(
        nodes: Vec<Vector3>
    ) -> Self {
        Self {
            nodes
        }
    }

    pub fn get_closest(
        &self,
        to: &Vector3
    ) -> Vector3 {
        let mut closest = 0;
        let mut min_distance = f32::MAX;

        if self.nodes.len() == 2 {
            return self.nodes[0].clone();
        }

        for i in 0..self.nodes.len()-1 {
            let seg = Line3::new(
                self.nodes[i].to_owned(), 
                self.nodes[i+1].to_owned()
            );

            let (_, distance, t) = seg.distance_to_point(to);
            if distance < min_distance {
                min_distance = distance;
                closest = if t <= 0.5 {i} else {i+1};
            }
        }

        self.nodes[closest].clone()
    }
}
