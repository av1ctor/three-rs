pub mod buffer_geometry;
pub mod object3d;

pub mod objectifiable;
pub mod renderable;
pub mod geometrical;
pub mod transformable;

pub use buffer_geometry::*;
pub use object3d::*;
pub use objectifiable::*;
pub use renderable::*;
pub use geometrical::*;
pub use transformable::*;

pub type RGB = [f32; 3];

pub trait GeometricalRenderable: Geometrical + Renderable {}
