pub mod vector_math;
use vector_math::*;
pub mod transform;
use transform::Transform;
pub mod camera;
use camera::Camera;

/*use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};*/

fn main() {
    /*let mut t = Transform::new();
    t.set_position(Vector3::new(1.0, -5.0, 2.0));
    t.set_rotation(Vector3::new(35.0, -187.0, 12.0));
    t.set_scale(Vector3::new(2.0, 0.3, 2.5));

    println!("Local to World: {:?}", t.get_local_to_world_matrix());
    println!("World to Local: {:?}", t.get_world_to_local_matrix());
    println!("Identity: {:?}", t.get_local_to_world_matrix() * t.get_world_to_local_matrix());*/

    let mut c = Camera::new(30.0, 16.0 / 9.0, 0.01, 1000.0);

    println!("Local to World: {:?}", c.get_projection_matrix());
    println!("World to Local: {:?}", c.get_inv_projection_matrix());
    println!("Identity: {:?}", c.get_projection_matrix() * c.get_inv_projection_matrix());
}

/*fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();    

    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        *control_flow = ControlFlow::Poll;
    
        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        *control_flow = ControlFlow::Wait;
    
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            },
            Event::MainEventsCleared => {
                println!("This should loop");
                // Application update code.
    
                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
            },
            _ => ()
         }
     });
 }*/
