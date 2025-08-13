extern crate asn_winapi_traits;

use crate::render_manager::RenderManager;
use asn_winapi_traits::{TAsnRenderManager, TAsnWindowManager};

mod bind_groups;
mod render_manager;
mod wgpu_utils;

pub fn get_render_manager(
) -> impl TAsnRenderManager + TAsnWindowManager<Window = winit::window::Window> {
    RenderManager::new()
}
