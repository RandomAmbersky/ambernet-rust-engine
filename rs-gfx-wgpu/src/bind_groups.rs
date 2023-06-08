use core::num;

#[derive(Default, Debug)]
pub struct BindGroupLayoutBuilder {
    entries: Vec<wgpu::BindGroupLayoutEntry>,
}

#[derive(Default, Debug)]
pub struct BindGroupEntryBuilder<'a> {
    entries: Vec<wgpu::BindGroupEntry<'a>>,
}

impl<'a> BindGroupEntryBuilder<'a> {
    pub fn texture(mut self, texture_view: &'a wgpu::TextureView) -> Self {
        self.entries.push(wgpu::BindGroupEntry {
            binding: self.entries.len() as _,
            resource: wgpu::BindingResource::TextureView(texture_view),
        });
        self
    }
    pub fn sampler(mut self, sampler: &'a wgpu::Sampler) -> Self {
        self.entries.push(wgpu::BindGroupEntry {
            binding: self.entries.len() as _,
            resource: wgpu::BindingResource::Sampler(sampler),
        });
        self
    }

    #[inline]
    pub fn entries(&self) -> &[wgpu::BindGroupEntry] {
        &self.entries
    }
}

impl BindGroupLayoutBuilder {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn texture(mut self) -> Self {
        self.entries.push(wgpu::BindGroupLayoutEntry {
            binding: self.entries.len() as _,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                multisampled: false,
                view_dimension: wgpu::TextureViewDimension::D2,
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
            },
            count: None,
        });
        self
    }

    pub fn sampler(mut self) -> Self {
        self.entries.push(wgpu::BindGroupLayoutEntry {
            binding: self.entries.len() as _,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        });
        self
    }

    #[inline]
    pub fn entries(&self) -> &[wgpu::BindGroupLayoutEntry] {
        &self.entries
    }
}
