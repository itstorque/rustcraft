mod graphics;
mod util;

use glow::*;
use std::str;
use graphics::shaders::{ShaderManager, Shader};

const VERTEX_SHADER_SOURCE: &str = &include_str!("graphics/shaders/vertex.hlsl");
const FRAGMENT_SHADER_SOURCE: &str = &include_str!("graphics/shaders/fragment.hlsl");

fn main() {
    unsafe {
        // Create a context from a sdl2 window
        let (gl, window, mut events_loop, _context) = create_sdl2_context();

        // Create a shader program from source
        let shader_manager = ShaderManager::new(&gl);
        shader_manager.load_example();

        // Upload uniforms
        shader_manager.set_uniforms(Shader::Example(0.8));

        // Create a vertex buffer and vertex array object
        let (vbo, vao) = create_vertex_buffer(&gl);

        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        'render: loop {
            {
                for event in events_loop.poll_iter() {
                    if let sdl2::event::Event::Quit { .. } = event {
                        break 'render;
                    }
                }
            }

            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
            window.gl_swap_window();
        }

        // Clean up
        gl.delete_vertex_array(vao);
        gl.delete_buffer(vbo)
    }
}

unsafe fn create_sdl2_context() -> (
    glow::Context,
    sdl2::video::Window,
    sdl2::EventPump,
    sdl2::video::GLContext,
) {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 0);
    let window = video
        .window("Hello triangle!", 1024, 769)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let gl = glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
    let event_loop = sdl.event_pump().unwrap();

    (gl, window, event_loop, gl_context)
}

unsafe fn create_vertex_buffer(gl: &glow::Context) -> (NativeBuffer, NativeVertexArray) {
    // This is a flat array of f32s that are to be interpreted as vec2s.
    let triangle_vertices = [0.5f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32];
    let triangle_vertices_u8: &[u8] = core::slice::from_raw_parts(
        triangle_vertices.as_ptr() as *const u8,
        triangle_vertices.len() * core::mem::size_of::<f32>(),
    );

    // We construct a buffer and upload the data
    let vbo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, triangle_vertices_u8, glow::STATIC_DRAW);

    // We now construct a vertex array to describe the format of the input buffer
    let vao = gl.create_vertex_array().unwrap();
    gl.bind_vertex_array(Some(vao));
    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);

    (vbo, vao)
}