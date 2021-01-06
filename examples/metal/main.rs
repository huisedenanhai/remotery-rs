extern crate metal_rs as metal;
extern crate objc;

use cocoa::{appkit::NSView, base::id as cocoa_id};

use mem::ManuallyDrop;
use metal::*;
use objc::runtime::YES;
use std::mem;
use winit::platform::macos::WindowExtMacOS;

use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

use remotery_rs::{
    cpu_sample, metal::RemoteryMetal, metal_binding, metal_sample, Remotery, Settings,
};

fn prepare_pipeline_state<'a>(device: &DeviceRef, library: &LibraryRef) -> RenderPipelineState {
    let vert = library.get_function("triangle_vertex", None).unwrap();
    let frag = library.get_function("triangle_fragment", None).unwrap();

    let pipeline_state_descriptor = RenderPipelineDescriptor::new();
    pipeline_state_descriptor.set_vertex_function(Some(&vert));
    pipeline_state_descriptor.set_fragment_function(Some(&frag));
    let attachment = pipeline_state_descriptor
        .color_attachments()
        .object_at(0)
        .unwrap();
    attachment.set_pixel_format(MTLPixelFormat::BGRA8Unorm);

    attachment.set_blending_enabled(true);
    attachment.set_rgb_blend_operation(metal::MTLBlendOperation::Add);
    attachment.set_alpha_blend_operation(metal::MTLBlendOperation::Add);
    attachment.set_source_rgb_blend_factor(metal::MTLBlendFactor::SourceAlpha);
    attachment.set_source_alpha_blend_factor(metal::MTLBlendFactor::SourceAlpha);
    attachment.set_destination_rgb_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);
    attachment.set_destination_alpha_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);

    device
        .new_render_pipeline_state(&pipeline_state_descriptor)
        .unwrap()
}

fn prepare_render_pass_descriptor(descriptor: &RenderPassDescriptorRef, texture: &TextureRef) {
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();

    color_attachment.set_texture(Some(texture));
    color_attachment.set_load_action(MTLLoadAction::Clear);
    color_attachment.set_clear_color(MTLClearColor::new(0.0, 0.0, 0.0, 1.0));
    color_attachment.set_store_action(MTLStoreAction::Store);
}

fn main() {
    let events_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .with_title("Metal Window Example".to_string())
        .build(&events_loop)
        .unwrap();

    let device = Device::system_default().expect("no device found");

    let layer = CoreAnimationLayer::new();
    layer.set_device(&device);
    layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
    layer.set_presents_with_transaction(false);

    let mut remotery = ManuallyDrop::new(unsafe { Remotery::new(Settings::default()) }.unwrap());
    remotery.set_current_thread_name("Main");

    unsafe {
        let view = window.ns_view() as cocoa_id;
        view.setWantsLayer(YES);
        view.setLayer(mem::transmute(layer.as_ref()));
    }

    let draw_size = window.inner_size();
    layer.set_drawable_size(CGSize::new(draw_size.width as f64, draw_size.height as f64));

    let library = device
        .new_library_with_source(include_str!("shaders.metal"), &CompileOptions::new())
        .unwrap();
    let pipeline_state = prepare_pipeline_state(&device, &library);
    let command_queue = device.new_command_queue();

    let vbuf = {
        let vertex_data = [0.0f32, 0.5, -0.5, -0.5, 0.5, -0.5];

        device.new_buffer_with_data(
            vertex_data.as_ptr() as *const _,
            (vertex_data.len() * mem::size_of::<f32>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
        )
    };

    events_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    unsafe {
                        ManuallyDrop::drop(&mut remotery);
                    }
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(size) => {
                    layer.set_drawable_size(CGSize::new(size.width as f64, size.height as f64));
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                cpu_sample!(remotery, "Frame");
                {
                    use foreign_types::ForeignTypeRef;

                    let drawable = match layer.next_drawable() {
                        Some(drawable) => drawable,
                        None => return,
                    };

                    let render_pass_descriptor = RenderPassDescriptor::new();

                    prepare_render_pass_descriptor(&render_pass_descriptor, drawable.texture());

                    let command_buffer = command_queue.new_command_buffer();
                    metal_binding!(remotery, command_buffer.as_ptr());
                    metal_sample!(remotery, "Frame");
                    {
                        metal_sample!(remotery, "Draw Triangle");
                        let encoder =
                            command_buffer.new_render_command_encoder(&render_pass_descriptor);
                        encoder.set_render_pipeline_state(&pipeline_state);
                        encoder.set_vertex_buffer(0, Some(&vbuf), 0);
                        encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 3);
                        encoder.end_encoding();
                    }
                    {
                        metal_sample!(remotery, "Present");
                        command_buffer.present_drawable(&drawable);
                    }
                    command_buffer.commit();
                }
            }
            _ => {}
        }
    });
}
