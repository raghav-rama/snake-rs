#![allow(dead_code)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]
mod vertex;

use std::path::PathBuf;

use wgpu::{
    util::DeviceExt, Backends, BufferUsages, CompositeAlphaMode, Device, DeviceDescriptor,
    Dx12Compiler, Features, Gles3MinorVersion, Instance, InstanceDescriptor, InstanceFlags, Limits,
    PowerPreference, PresentMode, Queue, RenderPipeline, RequestAdapterOptions, StoreOp, Surface,
    SurfaceConfiguration, SurfaceTarget, TextureUsages,
};

use vertex::Vertex;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
struct WgpuState<'a> {
    instance: Instance,
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    render_pipeline: RenderPipeline,
    vertex_buffer: wgpu::Buffer,
}

pub async fn run() -> Result<(), wgpu::SurfaceError> {
    let event_loop = EventLoop::new().unwrap();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::all(),
        flags: InstanceFlags::DEBUG,
        dx12_shader_compiler: Dx12Compiler::Dxc {
            dxil_path: Some(PathBuf::from(r"../../../dxc_libs/libdxil.so")),
            dxc_path: Some(PathBuf::from(r"../../../dxc_libs/libdxcompiler.so")),
        },
        gles_minor_version: Gles3MinorVersion::Automatic,
    });

    let surface_target = SurfaceTarget::Window(Box::new(_window));
    let surface = unsafe { instance.create_surface(surface_target).unwrap() };

    let request_adapter_options = RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    };

    let adapter = instance
        .request_adapter(&request_adapter_options)
        .await
        .unwrap();

    let device_descriptor = DeviceDescriptor {
        label: None,
        required_features: Features::default(),
        required_limits: Limits::default(),
    };

    let (device, queue) = adapter
        .request_device(&device_descriptor, None)
        .await
        .unwrap();

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .filter(|f| f.is_srgb())
        .next()
        .unwrap_or(surface_caps.formats[0]);

    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: 800,
        height: 600,
        present_mode: PresentMode::Fifo,
        desired_maximum_frame_latency: 3,
        alpha_mode: CompositeAlphaMode::Auto,
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    let frag_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.frag.wgsl").into()),
    });

    let vertex_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.vert.wgsl").into()),
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &vertex_shader,
            entry_point: "vs_main",
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &frag_shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertex::VERTICES),
        usage: BufferUsages::VERTEX,
    });

    let mut wgpu_state = WgpuState {
        instance,
        surface,
        device,
        queue,
        config,
        render_pipeline,
        vertex_buffer,
    };

    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            println!("The close button was pressed; stopping");
            elwt.exit();
        }
        Event::WindowEvent {
            event: WindowEvent::Resized(size),
            ..
        } => {
            // Reconfigure the surface with the new size
            wgpu_state.config.width = size.width;
            wgpu_state.config.height = size.height;
            wgpu_state
                .surface
                .configure(&wgpu_state.device, &wgpu_state.config);
        }
        Event::AboutToWait => {
            // Redraw the frame
            render(&mut wgpu_state).unwrap();
            elwt.set_control_flow(ControlFlow::Poll);
        }
        _ => (),
    });
    Ok(())
}

fn render(wgpu_state: &mut WgpuState) -> Result<(), wgpu::SurfaceError> {
    let output = wgpu_state.surface.get_current_texture().unwrap();
    let view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = wgpu_state
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&wgpu_state.render_pipeline);
        render_pass.set_vertex_buffer(0, wgpu_state.vertex_buffer.slice(..));
        render_pass.draw(0..3, 0..1);
    }

    wgpu_state.queue.submit(std::iter::once(encoder.finish()));
    output.present();
    Ok(())
}
