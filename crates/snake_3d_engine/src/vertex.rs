#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
    }, // Top vertex
    Vertex {
        position: [-0.5, -0.5, 0.0],
    }, // Bottom left vertex
    Vertex {
        position: [0.5, -0.5, 0.0],
    }, // Bottom right vertex
];
