use crate::error::Error::ErrorReadingTexture;

#[derive(Default)]
pub enum TextureFilter {
    #[default]
    Linear,
    Nearest,
}

#[derive(Default)]
pub struct WrapMode {
    pub u: WrapAxis,
    pub v: WrapAxis,
    pub border_color: Option<wgpu::SamplerBorderColor>,
}

#[derive(Default)]
pub enum WrapAxis {
    #[default]
    Clamp,
    Mirror,
    Repeat,
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

#[derive(Default)]
pub struct TextureSamplerDescriptor {
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

//TODO! Add mipmaps options

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
