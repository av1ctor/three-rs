use std::{rc::Rc, cell::RefCell};
use glow::TRIANGLES;
use crate::{
    math::{matrix4::Matrix4, vector3::Vector3},
    core::{BufferGeometry, Geometrical}, object::Mesh
};

pub struct Gltf {
    pub geo: BufferGeometry,
}

impl Gltf {
    pub fn load_from_bytes(
        bytes: &[u8]
    ) -> Result<Rc<RefCell<Mesh>>, String> {
        let gltf = gltf::Gltf::from_slice(&bytes).unwrap();
        let buffers = gltf::import_buffers(&gltf.document, None, gltf.blob).unwrap();
        Self::load(&gltf.document, &buffers)
    }

    pub fn load<'a>(
        doc: &gltf::Document,
        buffers: &Vec<gltf::buffer::Data>
    ) -> Result<Rc<RefCell<Mesh>>, String> {
        
        let scene = doc.default_scene().unwrap();
        let mut positions = vec![];
        let mut indices = vec![];
        let mut idx = 0;

        for node in scene.nodes() {
            traverse_meshes(
                &node,
                None, 
                &mut |mesh, world_matrix| {
                    for primitive in mesh.primitives() {
                        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                                
                        if let Some(iter) = reader.read_positions() {
                            let prim_pos = iter.collect::<Vec<_>>();
                            let prim_ind: Vec<u32> = reader.read_indices()
                                .map(|ind| ind.into_u32().collect())
                                .unwrap_or((0..prim_pos.len() as u32).collect());

                            match primitive.mode() {
                                gltf::mesh::Mode::Triangles => {
                                    for i in 0..prim_ind.len() {
                                        let v = Vector3::from_slice(&prim_pos[prim_ind[i] as usize])
                                            .apply_matrix4(&world_matrix);
                                        positions.push(v);
                                        indices.push(idx);
                                        idx += 1;
                                    }
                                }
                                _ => {
                                    return Err("Unsupported primitive".to_string());
                                },
                            }
                        }
                    }

                    Ok(())
                }
            )?;
        }

        let mut colors = vec![];
        let mut color = 0.0;
        let inc = 1.0 / positions.len() as f32;
        for _ in 0..positions.len() {
            color += inc;
            colors.push([0.0, color, 0.0]);
        }

        Ok(Mesh::new(
            &Self{
                geo: BufferGeometry::new(
                    TRIANGLES, 
                    Some(indices), 
                    Some(positions), 
                    Some(colors)
                )
            }
        ))
    }
}

fn traverse_meshes(
    node: &gltf::Node<'_>,
    world_matrix: Option<&Matrix4>,
    cb: &mut dyn FnMut (&gltf::Mesh<'_>, &Matrix4) -> Result<(), String>
) -> Result<(), String> {
    
    let matrix = Matrix4::from_slice2(&node.transform().matrix());
    let world_matrix = if let Some(m) = world_matrix {
        m.mul(&matrix)
    }
    else {
        matrix
    };
    
    if let Some(mesh) = node.mesh() {
        cb(&mesh, &world_matrix)?;
        for child in node.children() {
            traverse_meshes(&child, Some(&world_matrix), cb)?;
        }
    }
    else {
        for child in node.children() {
            traverse_meshes(&child, Some(&world_matrix), cb)?;
        }
    }

    Ok(())
}

impl Geometrical for Gltf {
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
