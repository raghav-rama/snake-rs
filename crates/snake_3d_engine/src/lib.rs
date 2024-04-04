use std::path::PathBuf;

use wgpu::{
    Backends, CompositeAlphaMode, Device, DeviceDescriptor, Dx12Compiler, Features,
    Gles3MinorVersion, Instance, InstanceDescriptor, InstanceFlags, Limits, PowerPreference,
    PresentMode, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, SurfaceTarget,
    TextureFormat, TextureUsages,
};
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
}

pub async fn run() {
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
        required_features: Features::all(),
        required_limits: Limits::default(),
    };

    let (device, queue) = adapter
        .request_device(&device_descriptor, None)
        .await
        .unwrap();

    let mut view_formats = Vec::new();
    view_formats.push(TextureFormat::Rgba8Unorm);
    view_formats.push(TextureFormat::Bgra8Unorm);
    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: TextureFormat::R8Unorm,
        width: 800,
        height: 600,
        present_mode: PresentMode::AutoVsync,
        desired_maximum_frame_latency: 3,
        alpha_mode: CompositeAlphaMode::Auto,
        view_formats,
    };

    let wgpu_state = WgpuState {
        instance,
        surface,
        device,
        queue,
        config,
    };

    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            println!("The close button was pressed; stopping");
            elwt.exit();
        }
        _ => (),
    });
}
