use crate::math::{
    vector3::{Vector3, RIGHT, UP, FORWARD}, 
    euler::Euler, 
    quaternion::Quaternion, 
    matrix4::Matrix4, 
    matrix3::Matrix3
};

pub struct Object3d<'a> {
    pub id: usize,
    pub parent: Option<&'a Self>,
    pub children: Vec<&'a Self>,
    pub visible: bool,
    pub position: Vector3,
    pub rotation: Euler,
    pub quaternion: Quaternion,
    pub scale: Vector3,
    pub mv_matrix: Matrix4,
    pub matrix: Matrix4,
    pub matrix_world: Matrix4,
    pub cast_shadow: bool,
    pub receive_shadow: bool,
    pub frustum_culled: bool,
    pub render_order: usize,
}

impl<'a> Object3d<'_> {
    pub fn new(
        id: usize
    ) -> Self {
        Self { 
            id, 
            parent: None,
            children: vec![], 
            visible: true, 
            position: Vector3::zero(), 
            rotation: Euler::default(), 
            quaternion: Quaternion::default(), 
            scale: Vector3::one(), 
            mv_matrix: Matrix4::default(), 
            matrix: Matrix4::default(), 
            matrix_world: Matrix4::default(), 
            cast_shadow: true, 
            receive_shadow: true, 
            frustum_culled: true, 
            render_order: 0
        }
    }

    fn on_quaternion_updated(
        &mut self
    ) {
        self.rotation = Euler::from_quaternion(&self.quaternion, self.rotation.order);
    }

    fn update_matrix(
        &mut self
    ) {
        self.matrix = Matrix4::compose(&self.position, &self.quaternion, &self.scale);
    }

    pub fn apply_matrix(
        &mut self,
        m: &Matrix4
    ) {
        self.update_matrix();

        self.matrix = m.mul(&self.matrix);
        
        let (position, quaternion, scale) = self.matrix.decompose();
        self.position = position;
        self.quaternion = quaternion;
        self.on_quaternion_updated();
        self.scale = scale;
    }

    pub fn rotate_from_axis_angle(
        &mut self,
        axis: &Vector3,
        angle: f32
    ) {
        self.quaternion = Quaternion::from_axis_and_angle(axis, angle);
        self.on_quaternion_updated();
    }

    pub fn rotate_from_euler(
        &mut self,
        euler: &Euler
    ) {
        self.quaternion = Quaternion::from_vector(&euler.v, euler.order);
        self.on_quaternion_updated();
    }

    pub fn rotate_from_matrix(
        &mut self,
        m: &Matrix3
    ) {
        self.quaternion = Quaternion::from_matrix(m);
        self.on_quaternion_updated();
    }

    pub fn rotate_from_quaternion(
        &mut self,
        q: &Quaternion
    ) {
        self.quaternion = q.clone();
        self.on_quaternion_updated();
    }

    pub fn rotate_on_axis(
        &mut self,
        axis: &Vector3,
        angle: f32
    ) {
        self.quaternion = self.quaternion.rotate_on_axis(axis, angle);
        self.on_quaternion_updated();
    }

    pub fn rotate_x(
        &mut self,
        angle: f32
    ) {
        self.rotate_on_axis(&RIGHT, angle);
    }

    pub fn rotate_y(
        &mut self,
        angle: f32
    ) {
        self.rotate_on_axis(&UP, angle);
    }

    pub fn rotate_z(
        &mut self,
        angle: f32
    ) {
        self.rotate_on_axis(&FORWARD, angle);
    }

    pub fn rotate_on_world_axis(
        &mut self,
        axis: &Vector3,
        angle: f32
    ) {
        self.quaternion = Quaternion::from_axis_and_angle(axis, angle).mul(&self.quaternion);
        self.on_quaternion_updated();
    }

    pub fn translate_on_axis(
        &mut self,
        axis: &Vector3,
        distance: f32
    ) {
        self.position = self.position.add(&axis.apply_quaternion(&self.quaternion).mul_scalar(distance));
    }

    pub fn translate_x(
        &mut self,
        distance: f32
    ) {
        self.translate_on_axis(&RIGHT, distance);
    }

    pub fn translate_y(
        &mut self,
        distance: f32
    ) {
        self.translate_on_axis(&UP, distance);
    }

    pub fn translate_z(
        &mut self,
        distance: f32
    ) {
        self.translate_on_axis(&FORWARD, distance);
    }
}