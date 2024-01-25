use std::{rc::Rc, cell::RefCell};
use crate::math::{
    Euler, Matrix3, Matrix4, Quaternion, {Vector3, RIGHT, UP, FORWARD}
};
use super::GeometricalRenderable;

pub struct ObjectData {
    pub visible: bool,
    
    pub(crate) children: Vec<Rc<RefCell<dyn GeometricalRenderable>>>,
    
    pub(crate) position: Vector3,
    
    pub(crate) rotation: Euler,
    pub(crate) quaternion: Quaternion,
    pub(crate) scale: Vector3,
    pub(crate) dirt: bool,
    
    pub(crate) matrix: Matrix4,
    pub(crate) world_matrix: Matrix4,
    
    pub cast_shadow: bool,
    pub receive_shadow: bool,
    pub frustum_culled: bool,
    pub render_order: usize,
}

impl Clone for ObjectData {
    fn clone(
        &self
    ) -> Self {
        Self { 
            visible: self.visible, 
            children: vec![], 
            position: self.position.clone(), 
            rotation: self.rotation.clone(), 
            quaternion: self.quaternion.clone(), 
            scale: self.scale.clone(), 
            dirt: self.dirt, 
            matrix: self.matrix.clone(), 
            world_matrix: self.world_matrix.clone(), 
            cast_shadow: self.cast_shadow,
            receive_shadow: self.receive_shadow, 
            frustum_culled: self.frustum_culled, 
            render_order: self.render_order
        }
    }
}

impl ObjectData {
    pub fn new(
) -> Self {
        Self { 
            children: vec![], 
            visible: true, 
            position: Vector3::zero(), 
            rotation: Euler::default(), 
            quaternion: Quaternion::default(), 
            scale: Vector3::one(), 
            dirt: false,
            matrix: Matrix4::identity(), 
            world_matrix: Matrix4::identity(), 
            cast_shadow: true, 
            receive_shadow: true, 
            frustum_culled: true, 
            render_order: 0
        }
    }

    pub fn add(
        &mut self,
        child: Rc<RefCell<dyn GeometricalRenderable>>
    ) -> &mut Self {
        self.children.push(child);
        self
    }

    fn on_quaternion_updated(
        &mut self
    ) {
        self.rotation = Euler::from_quaternion(
            &self.quaternion, self.rotation.order
        );
    }

    pub fn update_matrix(
        &mut self
    ) {
        if self.dirt {
            self.matrix = Matrix4::compose(
                &self.position, &self.quaternion, &self.scale
            );
            self.dirt = false;
        }
    }

    pub fn apply_matrix(
        &mut self,
        m: &Matrix4
    ) -> &mut Self {
        self.matrix = m.mul(&self.matrix);
        
        let (position, quaternion, scale) = self.matrix.decompose();
        self.position = position;
        self.quaternion = quaternion;
        self.on_quaternion_updated();
        self.scale = scale;
        self
    }

    pub fn rotate_from_axis_angle(
        &mut self,
        axis: &Vector3,
        angle: f32
    ) -> &mut Self {
        self.quaternion = Quaternion::from_axis_and_angle(axis, angle);
        self.on_quaternion_updated();
        self.dirt = true;
        self
    }

    pub fn rotate_from_euler(
        &mut self,
        euler: &Euler
    ) -> &mut Self {
        self.quaternion = Quaternion::from_euler(&euler);
        self.on_quaternion_updated();
        self.dirt = true;
        self
    }

    pub fn rotate_from_matrix(
        &mut self,
        m: &Matrix3
    ) -> &mut Self {
        self.quaternion = Quaternion::from_matrix(m);
        self.on_quaternion_updated();
        self.dirt = true;
        self
    }

    pub fn rotate_from_quaternion(
        &mut self,
        q: &Quaternion
    ) -> &mut Self {
        self.quaternion = q.clone();
        self.on_quaternion_updated();
        self.dirt = true;
        self
    }

    pub fn rotate_on_axis(
        &mut self,
        axis: &Vector3,
        angle: f32
    ) -> &mut Self {
        self.quaternion = self.quaternion.rotate_on_axis(axis, angle);
        self.on_quaternion_updated();
        self.dirt = true;
        self
    }

    pub fn rotate_x(
        &mut self,
        angle: f32
    ) -> &mut Self {
        self.rotate_on_axis(&RIGHT, angle);
        self
    }

    pub fn rotate_y(
        &mut self,
        angle: f32
    ) -> &mut Self {
        self.rotate_on_axis(&UP, angle);
        self
    }

    pub fn rotate_z(
        &mut self,
        angle: f32
    ) -> &mut Self {
        self.rotate_on_axis(&FORWARD, angle);
        self
    }

    pub fn rotate_on_world_axis(
        &mut self,
        axis: &Vector3,
        angle: f32
    ) -> &mut Self {
        self.quaternion = Quaternion::from_axis_and_angle(axis, angle)
            .mul(&self.quaternion);
        self.on_quaternion_updated();
        self.dirt = true;
        self
    }

    pub fn translate_on_axis(
        &mut self,
        axis: &Vector3,
        distance: f32
    ) -> &mut Self {
        self.position = self.position.add(
            &axis.apply_quaternion(&self.quaternion)
                .mul_scalar(distance)
        );
        self.dirt = true;
        self
    }

    pub fn translate_x(
        &mut self,
        distance: f32
    ) -> &mut Self {
        self.translate_on_axis(&RIGHT, distance);
        self
    }

    pub fn translate_y(
        &mut self,
        distance: f32
    ) -> &mut Self {
        self.translate_on_axis(&UP, distance);
        self
    }

    pub fn translate_z(
        &mut self,
        distance: f32
    ) -> &mut Self {
        self.translate_on_axis(&FORWARD, distance);
        self
    }

    pub fn set_position(
        &mut self,
        position: Vector3
    ) -> &mut Self {
        self.position = position;
        self.dirt = true;
        self
    }

    pub fn set_x(
        &mut self,
        x: f32
    ) -> &mut Self {
        self.position.x = x;
        self.dirt = true;
        self
    }

    pub fn set_y(
        &mut self,
        y: f32
    ) -> &mut Self {
        self.position.y = y;
        self.dirt = true;
        self
    }

    pub fn set_z(
        &mut self,
        z: f32
    ) -> &mut Self {
        self.position.z = z;
        self.dirt = true;
        self
    }

    pub fn set_scale(
        &mut self,
        s: Vector3
    ) -> &mut Self {
        self.scale = s;
        self.dirt = true;
        self
    }

    pub fn set_rotation(
        &mut self,
        q: Quaternion
    ) -> &mut Self {
        self.quaternion = q;
        self.on_quaternion_updated();
        self.dirt = true;
        self
    }

    pub fn set_rotation_from_euler(
        &mut self,
        euler: Euler
    ) -> &mut Self {
        self.rotation = euler;
        self.quaternion = Quaternion::from_euler(&euler);
        self.dirt = true;
        self
    }
}

pub trait Object3d {
    fn get_object(
        &self
    ) -> &ObjectData;

    fn get_object_mut(
        &mut self
    ) -> &mut ObjectData;

    fn add(
        &mut self,
        child: Rc<RefCell<dyn GeometricalRenderable>>
    ) {
        self.get_object_mut().add(child);
    }

    fn show(
        &mut self
    ) {
        self.get_object_mut().visible = true;
    }

    fn hide(
        &mut self
    ) {
        self.get_object_mut().visible = false;
    }
}