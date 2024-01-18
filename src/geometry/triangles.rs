use glow::TRIANGLES;
use crate::{
    core::{BufferGeometry, Geometrical, RGB}, 
    math::Triangle
};

#[derive(Clone)]
pub struct Triangles {
    pub geo: BufferGeometry,
}

impl Triangles {
    pub fn new(
        triangles: Vec<Triangle>,
        color: RGB,
    ) -> Self {
        let positions = triangles.iter()
            .map(|t| [t.a, t.b, t.c])
            .flatten()
            .collect::<Vec<_>>();
        
        let mut colors = vec![];
        for _ in 0..positions.len() {
            colors.push(color.clone());
        }
        
        Self {
            geo: BufferGeometry::new(
                TRIANGLES, 
                None, 
                Some(positions), 
                None,
                Some(colors),
            )
        }
    }
}

impl Geometrical for Triangles {
    fn get_geometry(
        &self
    ) -> &BufferGeometry {
        &self.geo
    }

    fn get_geometry_mut(
        &mut self
    ) -> &mut BufferGeometry {
        &mut self.geo
    }

    fn drop(
        &mut self, 
        renderer: &crate::renderer::GlRenderer
    ) {
        (self as &mut dyn Geometrical).destroy(renderer)
    }
}
