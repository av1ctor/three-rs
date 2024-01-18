pub enum Event {
    Quit,
    KeyDown(usize),
}

#[cfg(feature = "renderer")]
pub mod gl_renderer;
#[cfg(not(feature = "renderer"))]
pub mod null_renderer;

#[cfg(feature = "renderer")]
pub use gl_renderer::*;
#[cfg(not(feature = "renderer"))]
pub use null_renderer::*;
