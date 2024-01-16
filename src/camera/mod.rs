pub mod camera;
pub mod perspective;
pub mod orthographic;

pub use camera::*;
pub use perspective::*;
pub use orthographic::*;

use crate::core::{Objectifiable, Updatable};

pub trait ObjectifiableCamera: Objectifiable + Updatable + Camera {}