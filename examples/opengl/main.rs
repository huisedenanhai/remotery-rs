extern crate glium;

use glium::{glutin, uniform, Surface};
use glium::{implement_vertex, index::PrimitiveType, program};

use remotery_rs::{opengl::RemoteryOpenGL, opengl_sample, Remotery, Settings};

fn main() {
    let mut event_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Remotery OpenGL Sample");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
        }

        implement_vertex!(Vertex, position);

        glium::VertexBuffer::new(
            &display,
            &[
                Vertex {
                    position: [-0.5, -0.5],
                },
                Vertex {
                    position: [0.0, 0.5],
                },
                Vertex {
                    position: [0.5, -0.5],
                },
            ],
        )
        .unwrap()
    };

    // building the index buffer
    let index_buffer =
        glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u16, 1, 2]).unwrap();

    // compiling shaders and linking them together
    let program = program!(&display,
         140 => {
             vertex: "
                #version 140
                in vec2 position;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            ",

             fragment: "
                #version 140
                out vec4 f_color;
                void main() {
                    f_color = vec4(0.0, 0.5, 0.5, 1.0);
                }
            "
         },
    )
    .unwrap();

    let remotery = unsafe { Remotery::new(Settings::default()) }.unwrap();
    let draw = move || {
        remotery.log_text("frame");

        opengl_sample!(remotery, "Frame");
        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        {
            opengl_sample!(remotery, "Draw Triangle");
            target
                .draw(
                    &vertex_buffer,
                    &index_buffer,
                    &program,
                    &uniform! {},
                    &Default::default(),
                )
                .unwrap();
        }
        target.finish().unwrap();
        display.gl_window().window().request_redraw();
    };

    // Draw the triangle to the screen.
    draw();

    // the main loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            glutin::event::Event::RedrawRequested(_) => {
                draw();
                glutin::event_loop::ControlFlow::Poll
            }
            _ => glutin::event_loop::ControlFlow::Poll,
        };
    });
}
