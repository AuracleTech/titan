use cgmath::Point3;
use vulkanalia::prelude::v1_0::*;

use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Assets {
    pub(crate) meshes: HashMap<String, Mesh>,
    pub(crate) textures: HashMap<String, Texture>,
}

#[derive(Debug)]
pub(crate) struct Texture {
    pub image: vk::Image,
    pub image_view: vk::ImageView,
    pub image_memory: vk::DeviceMemory,
    pub width: u32,
    pub height: u32,
    pub mip_levels: u32,
    // OPTIMIZE use a reference to the image view to reuse the same image view for multiple textures
    pub format: vk::Format,
    // OPTIMIZE use a reference to the texture sampler to reuse the same sampler for multiple textures
    pub sampler: vk::Sampler,
}

#[derive(Debug)]
pub(crate) struct Mesh {
    pub(crate) vertex_buffer: vk::Buffer,
    pub(crate) vertex_buffer_memory: vk::DeviceMemory,
    pub(crate) index_buffer: vk::Buffer,
    pub(crate) index_buffer_memory: vk::DeviceMemory,
    pub(crate) instances_positions: Vec<Point3<f32>>,
    pub(crate) index_count: u32,
}
