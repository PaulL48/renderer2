use wgpu::{
    AddressMode, CompareFunction, Device, Extent3d, FilterMode, Queue, Sampler, SamplerDescriptor,
    TextureDescriptor, TextureDimension, TextureFormat, TextureView, TextureViewDescriptor,
};

pub struct BinaryTexture<'a> {
    data: &'a [u8],
    format: TextureFormat,
    width: u32,
    height: u32,
}

pub struct Texture {
    _texture: wgpu::Texture,
    view: TextureView,
    sampler: Sampler,
}

impl Texture {
    pub fn view(&self) -> &TextureView {
        &self.view
    }

    pub fn sampler(&self) -> &Sampler {
        &self.sampler
    }

    pub fn new_depth_texture(
        device: &Device,
        width: u32,
        height: u32,
        label: &str,
        format: &TextureFormat,
    ) -> Self {
        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: *format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&TextureViewDescriptor::default());

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some(label),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            compare: Some(CompareFunction::LessEqual),
            ..Default::default()
        });

        Self {
            _texture: texture,
            view,
            sampler,
        }
    }

    pub fn from_binary(
        device: &Device,
        queue: &Queue,
        label: &str,
        binary_texture: &BinaryTexture,
    ) -> Self {
        let size = Extent3d {
            width: binary_texture.width,
            height: binary_texture.height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: binary_texture.format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            binary_texture.data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(Texture::bytes_per_row(
                    binary_texture.width,
                    &binary_texture.format,
                )),
                rows_per_image: Some(binary_texture.height),
            },
            size,
        );

        let view = texture.create_view(&TextureViewDescriptor::default());

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some(label),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            compare: Some(CompareFunction::LessEqual),
            ..Default::default()
        });

        Self {
            _texture: texture,
            view,
            sampler,
        }
    }

    fn bytes_per_row(width: u32, format: &TextureFormat) -> u32 {
        let bytes_per_pixel = match format {
            TextureFormat::R8Unorm => 1,
            TextureFormat::R8Snorm => 1,
            TextureFormat::R8Uint => 1,
            TextureFormat::R8Sint => 1,
            TextureFormat::R16Uint => 2,
            TextureFormat::R16Sint => 2,
            TextureFormat::R16Unorm => 2,
            TextureFormat::R16Snorm => 2,
            TextureFormat::R16Float => 2,
            TextureFormat::Rg8Unorm => 2,
            TextureFormat::Rg8Snorm => 2,
            TextureFormat::Rg8Uint => 2,
            TextureFormat::Rg8Sint => 2,
            TextureFormat::R32Uint => 4,
            TextureFormat::R32Sint => 4,
            TextureFormat::R32Float => 4,
            TextureFormat::Rg16Uint => 4,
            TextureFormat::Rg16Sint => 4,
            TextureFormat::Rg16Unorm => 4,
            TextureFormat::Rg16Snorm => 4,
            TextureFormat::Rg16Float => 4,
            TextureFormat::Rgba8Unorm => 4,
            TextureFormat::Rgba8UnormSrgb => 4,
            TextureFormat::Rgba8Snorm => 4,
            TextureFormat::Rgba8Uint => 4,
            TextureFormat::Rgba8Sint => 4,
            TextureFormat::Bgra8Unorm => 4,
            TextureFormat::Bgra8UnormSrgb => 4,
            TextureFormat::Rgb9e5Ufloat => 4,
            TextureFormat::Rgb10a2Uint => 4,
            TextureFormat::Rgb10a2Unorm => 4,
            TextureFormat::Rg11b10Float => 4,
            TextureFormat::Rg32Uint => 8,
            TextureFormat::Rg32Sint => 8,
            TextureFormat::Rg32Float => 8,
            TextureFormat::Rgba16Uint => 8,
            TextureFormat::Rgba16Sint => 8,
            TextureFormat::Rgba16Unorm => 8,
            TextureFormat::Rgba16Snorm => 8,
            TextureFormat::Rgba16Float => 8,
            TextureFormat::Rgba32Uint => 16,
            TextureFormat::Rgba32Sint => 16,
            TextureFormat::Rgba32Float => 16,
            TextureFormat::Stencil8 => 1,
            TextureFormat::Depth16Unorm => 2,
            TextureFormat::Depth24Plus => 3,
            TextureFormat::Depth24PlusStencil8 => 4,
            TextureFormat::Depth32Float => 4,
            TextureFormat::Depth32FloatStencil8 => 5,
            format => panic!(
                "Please fill this table with the bytes per pixel for format: {:?}",
                format
            ),
        };

        width * bytes_per_pixel
    }
}
