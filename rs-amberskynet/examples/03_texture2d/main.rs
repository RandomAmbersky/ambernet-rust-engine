extern crate core;

mod resource;
mod view_2d;

const GLOBAL_LOG_FILTER: log::LevelFilter = log::LevelFilter::Info;

use crate::resource::{SHADER_SOURCE, TEXTURE_SOURCE};
use crate::view_2d::View2D;
use rs_amberskynet::{AsnContext, ExtHandlerTrait};
use rs_gfx_core::AsnTextureTrait;
use rs_gfx_wgpu::AsnTexture;

fn init_log() {
    let mut builder = fern::Dispatch::new();
    let level_formatter;
    #[cfg(target_arch = "wasm32")]
    {
        level_formatter = |level| level;
        builder = builder.chain(fern::Output::call(console_log::log));
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use fern::colors::{Color, ColoredLevelConfig};
        let colors = ColoredLevelConfig::new()
            .info(Color::Blue)
            .debug(Color::Green);
        level_formatter = move |level| colors.color(level);
        builder = builder.chain(std::io::stdout());
    }
    builder
        .level(GLOBAL_LOG_FILTER)
        .level_for(module_path!(), log::LevelFilter::Debug)
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}:{}] {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                level_formatter(record.level()),
                record.target(),
                record.line().unwrap_or_default(),
                message
            ))
        })
        .apply()
        .unwrap();
}

struct Handler {
    view_2d: View2D,
}

impl Handler {
    pub fn new(ctx: &AsnContext) -> Self {
        let surface_caps = ctx
            .gfx
            .main_window
            .surface
            .get_capabilities(&ctx.gfx.adapter);

        // let surface_iter = surface_caps.formats.iter(); //.copied();
        // for form in surface_iter {
        //     println!("surface_format list: {:?}", form);
        // }

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            // .find(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        // println!("surface_format: {:?}", surface_format);

        let shader = ctx
            .gfx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
            });

        let texture = AsnTexture::from_raw_image(&ctx.gfx, TEXTURE_SOURCE).unwrap();

        // let texture_format = texture.texture.format();
        // println!("texture_format: {:?}", texture_format);
        // let update_format = texture_format.add_srgb_suffix();
        // println!("texture_format: {:?}", update_format);

        let view_2d = View2D::new(&ctx.gfx.device, &texture, surface_format, &shader);

        Self { view_2d }
    }
}

impl ExtHandlerTrait for Handler {
    fn draw(&mut self, ctx: &mut AsnContext) {
        let fcx = ctx.gfx.fcx.as_mut().unwrap();
        self.view_2d.draw(&mut fcx.encoder, &fcx.view);
    }
    fn update(&mut self, _e: &mut AsnContext) {}
}

pub fn main() {
    init_log();
    let (ctx, event_loop) = rs_amberskynet::init();
    let h = Handler::new(&ctx);
    rs_amberskynet::run(ctx, event_loop, h)
}
