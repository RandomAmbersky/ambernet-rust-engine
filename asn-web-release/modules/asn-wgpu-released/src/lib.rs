use crate::render_manager::RenderManager;
use asn_core_winapi::TAsnRenderManager;
use std::sync::Arc;
use winit::window::Window;

mod bind_groups;
mod render_manager;
mod wgpu_utils;

pub trait WinitAdapter {
    fn init_window(&mut self, window: Arc<Window>);
}

pub fn get_render_manager() -> impl TAsnRenderManager + WinitAdapter {
    RenderManager::new()
}
