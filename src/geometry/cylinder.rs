use std::f32::consts::PI;
use glow::TRIANGLES;
use crate::{
    core::{BufferGeometry, Geometrical}, 
    math::Vector3
};

#[derive(Clone)]
pub struct Cylinder {
    pub geo: BufferGeometry,
}

impl Cylinder {
    pub fn new_ex(
        radius_top: f32, 
        radius_bottom: f32, 
        height: f32,
        radial_segments: usize, 
        height_segments: usize, 
        open_ended: bool, 
        theta_start: f32,
        theta_length: f32
    ) -> Self {

        let mut indices = vec![];
        let mut positions = vec![];
        let mut index = 0;

        Self::generate_torso(
            radius_top, 
            radius_bottom, 
            height,
            &mut positions, 
            &mut indices,
            radial_segments, 
            height_segments, 
            theta_start,
            theta_length,
            &mut index
        );

        if !open_ended {
			if radius_top > 0.0 {
                Self::generate_cap( 
                    true,
                    radius_top, 
                    radius_bottom, 
                    height,
                    &mut positions, 
                    &mut indices,
                    radial_segments, 
                    theta_start,
                    theta_length,
                    &mut index 
                );
            }
        }
		
        if radius_bottom > 0.0 { 
            Self::generate_cap(
                false,
                radius_top, 
                radius_bottom, 
                height,
                &mut positions, 
                &mut indices,
                radial_segments, 
                theta_start,
                theta_length,
                &mut index 
            );
		}

        let mut colors = vec![];
        let mut color = 0.1;
        let inc = 0.9 / positions.len() as f32;
        for _ in 0..positions.len() {
            color += inc;
            colors.push([0.0, 0.0, color]);
        }
        
        Self {
            geo: BufferGeometry::new(
                TRIANGLES, 
                Some(indices), 
                Some(positions), 
                Some(colors),
            )
        }
    }

    pub fn new(
        radius_top: f32, 
        radius_bottom: f32, 
        height: f32
    ) -> Self {
        Self::new_ex(
            radius_top, 
            radius_bottom, 
            height, 
            32, 
            1, 
            false, 
            0.0, 
            PI * 2.0
        )
    }

    fn generate_torso(
        radius_top: f32, 
        radius_bottom: f32, 
        height: f32, 
        positions: &mut Vec<Vector3>, 
        indices: &mut Vec<u32>, 
        radial_segments: usize, 
        height_segments: usize, 
        theta_start: f32, 
        theta_length: f32,
        index: &mut usize
    ) {
        let mut index_array = vec![vec![]; height_segments+1];
        let half_height = height / 2.0;

        for y in 0..=height_segments {
            let mut index_row = vec![];
            let v = y as f32 / height_segments as f32;
            let radius = v * (radius_bottom - radius_top) + radius_top;

            for x in 0..=radial_segments {

                let u = x as f32 / radial_segments as f32;

                let theta = u * theta_length + theta_start;

                let sin_theta = theta.sin();
                let cos_theta = theta.cos();

                positions.push(Vector3::new(
                    radius * sin_theta,
                    -v * height + half_height,
                    radius * cos_theta
                ));

                index_row.push(*index);
                *index += 1;
            }

            index_array[y] = index_row;
        }

        for  x in 0..radial_segments {
            for y in 0..height_segments {
                let a = index_array[ y ][ x ];
                let b = index_array[ y + 1 ][ x ];
                let c = index_array[ y + 1 ][ x + 1 ];
                let d = index_array[ y ][ x + 1 ];

                indices.extend_from_slice(&[a as _, b as _, d as _]);
                indices.extend_from_slice(&[b as _, c as _, d as _]);
            }
        }
    }

    fn generate_cap(
        top: bool,
        radius_top: f32, 
        radius_bottom: f32, 
        height: f32, 
        positions: &mut Vec<Vector3>, 
        indices: &mut Vec<u32>, 
        radial_segments: usize, 
        theta_start: f32, 
        theta_length: f32,
        index: &mut usize
    ) {
        let center_index_start = *index;
        let radius = if top == true {radius_top} else {radius_bottom};
        let sign = if top == true {1.0} else {-1.0};
        let half_height = height / 2.0;

        for _ in 1..=radial_segments {
            positions.push(Vector3::new(0.0, half_height * sign, 0.0));
            *index += 1;
        }

        let center_index_end = *index;

        for x in 0..=radial_segments {
            let u = x as f32 / radial_segments as f32;
            let theta = u * theta_length + theta_start;

            let cos_theta = theta.cos();
            let sin_theta = theta.sin();
            
            positions.push(Vector3::new(
                radius * sin_theta,
                half_height * sign,
                radius * cos_theta
            ));

            *index += 1;
        }

        for x in 0..radial_segments {
            let c = center_index_start + x;
            let i = center_index_end + x;

            if top == true {
                indices.extend_from_slice(&[i as _, (i + 1) as _, c as _]);
            } 
            else {
                indices.extend_from_slice(&[(i + 1) as _, i as _, c as _]);
            }

        }
    }
}

impl Geometrical for Cylinder {
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
