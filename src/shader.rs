use anyhow::{anyhow, Result};

use vulkanalia::prelude::v1_0::*;
use vulkanalia::Device;

pub(crate) unsafe fn create_shader_module(
    device: &Device,
    name: String,
) -> Result<vk::ShaderModule> {
    let path = format!("lib/{}.spv", name);
    let file_content = std::fs::read(path)?;

    let bytecode = Vec::<u8>::from(file_content);
    let (prefix, code, suffix) = bytecode.align_to::<u32>();
    if !prefix.is_empty() || !suffix.is_empty() {
        return Err(anyhow!("Shader bytecode is not properly aligned."));
    }

    let info = vk::ShaderModuleCreateInfo::builder()
        .code_size(bytecode.len())
        .code(code);

    Ok(device.create_shader_module(&info, None)?)
}

pub(crate) fn delete_compiled_shaders() -> Result<()> {
    let files = std::fs::read_dir("lib")?;
    for file in files {
        let file = file?;
        let path = file.path();
        if let Some(ext) = path.extension() {
            if ext == "spv" {
                std::fs::remove_file(path)?;
            }
        }
    }
    Ok(())
}

pub(crate) fn compile_shaders() -> Result<()> {
    let output_vert = std::process::Command::new("glslc.exe")
        .arg("assets/shaders/shader.vert")
        .arg("-o")
        .arg("lib/vert.spv")
        .output()
        .expect("Failed to execute glslc.exe for vertex shader");

    let output_frag = std::process::Command::new("glslc.exe")
        .arg("assets/shaders/shader.frag")
        .arg("-o")
        .arg("lib/frag.spv")
        .output()
        .expect("Failed to execute glslc.exe for fragment shader");

    if !(output_vert.status.success() && output_frag.status.success()) {
        panic!(
            "Failed to compile shaders:\n{}\n{}",
            String::from_utf8_lossy(&output_vert.stderr),
            String::from_utf8_lossy(&output_frag.stderr)
        );
    }

    Ok(())
}
