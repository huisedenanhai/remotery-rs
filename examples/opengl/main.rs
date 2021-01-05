extern crate glium;

use std::mem::ManuallyDrop;

use glium::{glutin, uniform, Surface, VertexBuffer};
use glium::{implement_vertex, index::PrimitiveType, program};

use remotery_rs::{opengl::RemoteryOpenGL, opengl_sample, Remotery, Settings};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

struct Context {
    display: glium::Display,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
    program: glium::Program,
}

impl Context {
    fn new(event_loop: &glutin::event_loop::EventLoop<()>) -> Context {
        let wb = glium::glutin::window::WindowBuilder::new()
            .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
            .with_title("Remotery OpenGL Sample");
        let cb = glium::glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        // building the vertex buffer, which contains all the vertices that we will draw
        let vertex_buffer = {
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

        Context {
            display,
            vertex_buffer,
            index_buffer,
            program,
        }
    }

    fn draw(&self, remotery: &Remotery) {
        let Context {
            display,
            vertex_buffer,
            index_buffer,
            program,
        } = self;
        remotery.set_current_thread_name("Main");

        opengl_sample!(remotery, "Frame");
        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        {
            opengl_sample!(remotery, "Draw Triangle");
            target
                .draw(
                    vertex_buffer,
                    index_buffer,
                    program,
                    &uniform! {},
                    &Default::default(),
                )
                .unwrap();
        }
        target.finish().unwrap();
    }

    fn request_redraw(&self) {
        self.display.gl_window().window().request_redraw();
    }
}

fn main() {
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let context = Context::new(&event_loop);

    let mut remotery = ManuallyDrop::new(unsafe { Remotery::new(Settings::default()) }.unwrap());

    context.draw(&remotery);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    // remotery should be dropped before opengl context to avoid unexpected hanging
                    unsafe { ManuallyDrop::drop(&mut remotery) };
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }
                _ => {}
            },
            glutin::event::Event::MainEventsCleared => {
                context.request_redraw();
            }
            glutin::event::Event::RedrawRequested(_) => {
                context.draw(&remotery);
            }
            _ => {}
        }
    });
}
