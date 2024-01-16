use glow::TRIANGLES;
use crate::{core::{BufferGeometry, Geometrical}, math::Vector3};

#[derive(Clone)]
pub struct Box3 {
    pub geo: BufferGeometry,
}

enum Coords {
    ZYX,
    XZY,
    XYZ
}

impl Box3 {
    pub fn new(
        width: f32,
        height: f32,
        depth: f32,
        width_segs: usize,
        height_segs: usize,
        depth_segs: usize
    ) -> Self {
        let mut geo = BufferGeometry::new(
            TRIANGLES, 
            vec![], 
            vec![], 
            vec![]
        );

        let mut num_vertices = 0;

        num_vertices += Self::build_plane(
            &mut geo, 
            Coords::ZYX, 
            -1.0, -1.0, 
            depth, height, width, 
            depth_segs, height_segs,
            num_vertices
        );

        num_vertices += Self::build_plane(
            &mut geo, 
            Coords::ZYX, 
            1.0, -1.0, 
            depth, height, -width, 
            depth_segs, height_segs,
            num_vertices
        );

        num_vertices += Self::build_plane(
            &mut geo, 
            Coords::XZY, 
            1.0, 1.0, 
            width, depth, height, 
            width_segs, depth_segs,
            num_vertices
        );

        num_vertices += Self::build_plane(
            &mut geo, 
            Coords::XZY, 
            1.0, -1.0, 
            width, depth, -height, 
            width_segs, depth_segs,
            num_vertices
        );

        num_vertices += Self::build_plane(
            &mut geo, 
            Coords::XYZ, 
            1.0, -1.0, 
            width, height, depth, 
            width_segs, height_segs,
            num_vertices
        );

        num_vertices += Self::build_plane(
            &mut geo, 
            Coords::XYZ, 
            -1.0, -1.0, 
            width, height, -depth, 
            width_segs, height_segs,
            num_vertices
        );

        let mut color = 0.0;
        let inc = 1.0 / num_vertices as f32;
        for _ in 0..num_vertices {
            color += inc;
            geo.colors.push([0.0, 0.0, color]);
        }
        
        Self {
            geo
        }
    }

    fn build_plane(
        geo: &mut BufferGeometry, 
        coords: Coords, 
        udir: f32, 
        vdir: f32, 
        width: f32, 
        height: f32, 
        depth: f32, 
        grid_x: usize, 
        grid_y: usize,
        num_vertices: usize
    ) -> usize {
        let segment_width = width / grid_x as f32;
        let segment_height = height / grid_y as f32;

        let width_half = width / 2.0;
        let height_half = height / 2.0;
        let depth_half = depth / 2.0;

        let grid_x1 = grid_x + 1;
        let grid_y1 = grid_y + 1;

        let mut vertex_counter = 0;

        let mut vector = Vector3::default();

        for iy in 0..grid_y1 {
            let y = (iy as f32) * segment_height - height_half;
            for ix in 0..grid_x1 {
                let x = ix as f32 * segment_width - width_half;

                match coords {
                    Coords::ZYX => {
                        vector.z = x * udir;
                        vector.y = y * vdir;
                        vector.x = depth_half;
                    },
                    Coords::XZY => {
                        vector.x = x * udir;
                        vector.z = y * vdir;
                        vector.y = depth_half;
                    },
                    Coords::XYZ => {
                        vector.x = x * udir;
                        vector.y = y * vdir;
                        vector.z = depth_half;
                    },
                }

                geo.positions.push(vector);

                vertex_counter += 1;
            }
        }

        for iy in 0..grid_y {
            for ix in 0..grid_x {
                let a = (num_vertices + ix + grid_x1 * iy) as u32;
                let b = (num_vertices + ix + grid_x1 * (iy + 1)) as u32;
                let c = (num_vertices + (ix + 1) + grid_x1 * (iy + 1)) as u32;
                let d = (num_vertices + (ix + 1) + grid_x1 * iy) as u32;

                geo.indices.extend_from_slice(&[a, b, d]);
                geo.indices.extend_from_slice(&[b, c, d]);
            }
        }

        vertex_counter
    }
}

impl Geometrical for Box3 {
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
