use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "d0a41df4-03f6-4af3-be33-d042ffc81a11"]
pub struct OutlineMaterial {
    #[texture(1)]
    #[sampler(2)]
    pub image: Handle<Image>,
}

impl Material2d for OutlineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/outline_material.wgsl".into()
    }
}
