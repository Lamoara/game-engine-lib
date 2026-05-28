use std::path::Path;

use crate::{error::Error, material::texture::Texture};

pub async fn load_binary(file_path: impl AsRef<Path>) -> anyhow::Result<Vec<u8>> {        
    let data = {
        let path = std::path::Path::new(env!("OUT_DIR"))
            .join("res")
            .join(file_path);
        std::fs::read(path).map_err(|e| Error::ErrorLoadingResource(e.into()))?
    };

    Ok(data)
}


async fn load_texture(
    path: impl AsRef<Path>,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> anyhow::Result<Texture> {
    let data = load_binary(path).await?;
    Texture::from_bytes(device, queue, &data, path)
}
