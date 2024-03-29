use crate::math::{Vector3, Matrix4, Euler, Matrix3, Quaternion};
use super::Object3d;

pub trait Transformable
    where Self: Object3d {
    fn apply_matrix(
        &mut self,
        m: &Matrix4
    ) -> &mut Self {
        self.get_object_mut().apply_matrix(m);
        self
    }

    fn rotate_from_axis_angle(
        &mut self,
        axis: &Vector3,
        angle: f32
    ) -> &mut Self {
        self.get_object_mut().rotate_from_axis_angle(axis, angle);
        self
    }

    fn rotate_from_euler(
        &mut self,
        euler: &Euler
    ) -> &mut Self {
        self.get_object_mut().rotate_from_euler(euler);
        self
    }

    fn rotate_from_matrix(
        &mut self,
        m: &Matrix3
    ) -> &mut Self {
        self.get_object_mut().rotate_from_matrix(m);
        self
    }

    fn rotate_from_quaternion(
        &mut self,
        q: &Quaternion
    ) -> &mut Self {
        self.get_object_mut().rotate_from_quaternion(q);
        self
    }

    fn rotate_on_axis(
        &mut self,
        axis: &Vector3,
        angle: f32
    ) -> &mut Self {
        self.get_object_mut().rotate_on_axis(axis, angle);
        self
    }

    fn rotate_x(
        &mut self,
        angle: f32
    ) -> &mut Self {
        self.get_object_mut().rotate_x(angle);
        self
    }

    fn rotate_y(
        &mut self,
        angle: f32
    ) -> &mut Self {
        self.get_object_mut().rotate_y(angle);
        self
    }

    fn rotate_z(
        &mut self,
        angle: f32
    ) -> &mut Self {
        self.get_object_mut().rotate_z(angle);
        self
    }

    fn translate_on_axis(
        &mut self,
        axis: &Vector3,
        distance: f32
    ) -> &mut Self {
        self.get_object_mut().translate_on_axis(axis, distance);
        self
    }

    fn translate_x(
        &mut self,
        distance: f32
    ) -> &mut Self {
        self.get_object_mut().translate_x(distance);
        self
    }

    fn translate_y(
        &mut self,
        distance: f32
    ) -> &mut Self {
        self.get_object_mut().translate_y(distance);
        self
    }

    fn translate_z(
        &mut self,
        distance: f32
    ) -> &mut Self {
        self.get_object_mut().translate_z(distance);
        self
    }

    fn set_position(
        &mut self,
        position: Vector3
    ) -> &mut Self {
        self.get_object_mut().set_position(position);
        self
    }
    
    fn set_scale(
        &mut self,
        s: Vector3
    ) -> &mut Self {
        self.get_object_mut().set_scale(s);
        self
    }

    fn set_rotation(
        &mut self,
        q: Quaternion
    ) -> &mut Self {
        self.get_object_mut().set_rotation(q);
        self
    }

    fn set_rotation_from_euler(
        &mut self,
        euler: Euler
    ) -> &mut Self {
        self.get_object_mut().set_rotation_from_euler(euler);
        self
    }

    fn on_matrix_updated(
        &mut self,
        _matrix: &Matrix4
    ) {
    }
}