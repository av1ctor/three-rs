pub mod camera;
pub mod perspective;
pub mod orthographic;

pub use camera::*;
pub use perspective::*;
pub use orthographic::*;

use crate::core::{Object3d, Updatable};

pub trait ObjectCamera: Object3d + Updatable + Camera {}