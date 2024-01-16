pub mod buffer_geometry;
pub mod object3d;

pub mod renderable;
pub mod geometrical;
pub mod transformable;
pub mod updatable;

pub use buffer_geometry::*;
pub use object3d::*;

pub use renderable::*;
pub use geometrical::*;
pub use transformable::*;
pub use updatable::*;

pub type RGB = [f32; 3];

pub trait GeometricalRenderable: Geometrical + Renderable {}
