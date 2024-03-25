use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn run() {
    let event_loop = EventLoop::new().unwrap();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

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
