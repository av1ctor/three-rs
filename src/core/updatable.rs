use super::Object3d;

pub trait Updatable 
    where Self: Object3d {
    fn update_matrix(
        &mut self,
    ) {
        self.get_object_mut().update_matrix();
    }
}