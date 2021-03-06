mod renderer;
mod rendererUV;
mod kimg;
mod kmath;
mod editor;
mod level;
mod application;
mod game;
mod manifest;
mod level_menu;
mod kgui;

use application::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use std::env;




fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let event_loop = glutin::event_loop::EventLoop::new();
    let mut application = Application::new(&event_loop);
    
    event_loop.run(move |event, _, control_flow| {
        application.handle_event(&event);
        match event {
            Event::LoopDestroyed |
            Event::WindowEvent {event: WindowEvent::CloseRequested, ..}
            => {
                *control_flow = ControlFlow::Exit;
            },
            _ => (),
        }
    });
}