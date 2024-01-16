use super::Object3d;

pub trait Object {
    fn get_object(
        &self
    ) -> &Object3d;

    fn get_object_mut(
        &mut self
    ) -> &mut Object3d;
}