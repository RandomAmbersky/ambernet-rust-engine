extern crate core;

mod asn_scene;
mod asn_winapi;
mod runner;
mod winit_event_processor;

pub type WinApi = asn_winapi::AsnWgpuWinApi;
pub type AsnScene = asn_scene::AsnWgpuScene;

pub use runner::Runner;
