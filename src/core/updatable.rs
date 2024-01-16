use super::Objectifiable;

pub trait Updatable 
    where Self: Objectifiable {
    fn update_matrix(
        &mut self,
    ) {
        self.get_object_mut().update_matrix();
    }
}