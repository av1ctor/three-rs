use std::{cell::RefCell, rc::Rc};

use super::{Object3d, GeometricalRenderable};

pub trait Objectifiable {
    fn get_object(
        &self
    ) -> &Object3d;

    fn get_object_mut(
        &mut self
    ) -> &mut Object3d;

    fn add(
        &mut self,
        child: Rc<RefCell<dyn GeometricalRenderable>>
    ) {
        self.get_object_mut().add(child);
    }
}