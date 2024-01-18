use crate::core::BufferGeometry;
use super::Event;

pub struct GlRenderer {
}

impl GlRenderer {
    pub unsafe fn new(
        _title: &str,
        _w: u32,
        _h: u32
    ) -> Self {
        Self {}
    }

    pub fn poll_events(
        &mut self
    ) -> Vec<Event> {
        vec![]
    }

    pub fn swap_window(
        &self
    ) {
    }

    pub fn clear(
        &self
    ) {
    }

    pub fn delete_buffers(
        &self,
        _geo: &BufferGeometry
    ) {
    }
}

