use log::error;

use crate::error::Error::ErrorReadingTexture;

#[derive(Default)]
pub enum TextureFilter {
    #[default]
    Linear,
    Nearest,
}

impl TextureFilter {
    pub fn filter_mode(&self) -> wgpu::FilterMode {
        match self {
            TextureFilter::Linear => wgpu::FilterMode::Linear,
            TextureFilter::Nearest => wgpu::FilterMode::Nearest,
        }
    }
}

#[derive(Default)]
pub struct WrapMode {
    pub u: WrapAxis,
    pub v: WrapAxis,
    pub border_color: Option<wgpu::SamplerBorderColor>,
}

impl WrapMode {
    pub fn address_mode_u(&self) -> wgpu::AddressMode {
        let address_mode = self.u.address_mode();
        self.address_mode(address_mode)
    }

    pub fn address_mode_v(&self) -> wgpu::AddressMode {
        let address_mode = self.v.address_mode();
        self.address_mode(address_mode)
    }

    fn address_mode(&self, address_mode: wgpu::AddressMode) -> wgpu::AddressMode {
        if let (None, wgpu::AddressMode::ClampToBorder) = (self.border_color, address_mode) {
            error!(
                "Clamp to border in texture without border color set, defaulting to clamp to edge"
            ); //TODO! Make this a proper warp/error
            return wgpu::AddressMode::ClampToEdge;
        }
        address_mode
    }
}

#[derive(Default)]
pub enum WrapAxis {
    #[default]
    Clamp,
    Mirror,
    Repeat,
    Border,
}

impl WrapAxis {
    pub fn address_mode(&self) -> wgpu::AddressMode {
        match self {
            WrapAxis::Clamp => wgpu::AddressMode::ClampToEdge,
            WrapAxis::Mirror => wgpu::AddressMode::MirrorRepeat,
            WrapAxis::Repeat => wgpu::AddressMode::Repeat,
            WrapAxis::Border => wgpu::AddressMode::ClampToBorder,
        }
    }
}

#[derive(Default)]
pub enum AnisotropyClamp {
    #[default]
    X1,
    X2,
    X4,
    X8,
    X16,
}

impl AnisotropyClamp {
    pub fn value(&self) -> u16 {
        match self {
            AnisotropyClamp::X1 => 1,
            AnisotropyClamp::X2 => 2,
            AnisotropyClamp::X4 => 4,
            AnisotropyClamp::X8 => 8,
            AnisotropyClamp::X16 => 16,
        }
    }
}

#[derive(Default)]
pub struct TextureSamplerDescriptor {
    pub label: String,
    pub wrap_mode: WrapMode,
    pub texture_filter: TextureFilter,
    pub anisotropic_clamp: AnisotropyClamp,
}

#[derive(Default)]
pub enum TextureColorSpace {
    #[default]
    Srgb,
    Linear,
}

#[derive(Default)]
pub struct TextureDescriptor {
    color_space: TextureColorSpace,
    sampler_descriptor: TextureSamplerDescriptor,
}

impl Into<wgpu::SamplerDescriptor> for TextureSamplerDescriptor {
    fn into(self) -> wgpu::SamplerDescriptor {
        wgpu::SamplerDescriptor {
            label: self.label,
            address_mode_u: self.wrap_mode.address_mode_u(),
            address_mode_v: self.wrap_mode.address_mode_v(),
            mag_filter: self.texture_filter.filter_mode(),
            min_filter: self.texture_filter.filter_mode(),
            anisotropy_clamp: self.anisotropic_clamp.value(),
            border_color: self.wrap_mode.border_color,
            ..Default::default()
        }
    }
}

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::Texture,
    pub samplet: wgpu::Sampler,
}

impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> anyhow::Result<Texture> {
        let img = image::load_from_memory(bytes).map_err(|e| ErrorReadingTexture(e.into()))?;
        Self::from_image(device, queue, &img, Some(label))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> anyhow::Result<Self> {
    }
}
