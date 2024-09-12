use crate::render_manager::RenderManager;
use asn_core_winapi::TAsnRenderManager;

mod bind_groups;
mod render_manager;
mod wgpu_utils;

pub fn get_render_manager() -> impl TAsnRenderManager {
    RenderManager::new()
}
