use glow::*;
use std::collections::HashMap;


pub struct ShaderManager {
    shader_up: Option<Shader>,
    example_shader: NativeProgram,
}

/// Enumerates all the shaders we have coded
pub enum Shader {
    Example(f32),
    Example2(f32, f32, f32),
}

impl Shader {
    fn get_source(self) -> Vec<(u32, &'static str)> {
        match self {
            Example => vec![
                (glow::VERTEX_SHADER, &include_str!("vertex.hlsl")), 
                (glow::FRAGMENT_SHADER, &include_str!("fragment.hlsl"))
            ]
        }
    }
}


impl ShaderManager {
    pub fn new(gl: &glow::Context) -> ShaderManager {
        let mut shader_programs = HashMap::new();

        // Initialize the example shader
        let example_shader = Self::init_shader(gl, vec![
            (glow::VERTEX_SHADER, &include_str!("vertex.hlsl")), 
            (glow::FRAGMENT_SHADER, &include_str!("fragment.hlsl"))
        ]);

        ShaderManager { shader_up: None, example_shader }
    }

    pub fn load(&mut self, gl: &glow::Context, shader: Shader) {
        
        self.shader_up = Some(shader);
    }

    pub update_uniforms(&mut self, shader: Shader) {

    }

    /// Create a program id for a shader
    fn init_shader(gl: &glow::Context, shader_src: Vec<(u32, &'static str)>) -> NativeProgram {
        // Make program
        unsafe {
            let program = gl.create_program().expect("Cannot create program");
        }
                
        // Make shader
        let mut shaders = Vec::with_capacity(shader_src.len());
        unsafe {
            for (shader_type, shader_source) in shader_sources.iter() {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");
                gl.shader_source(shader, shader_source);
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    panic!("{}", gl.get_shader_info_log(shader));
                }
                gl.attach_shader(program, shader);
                shaders.push(shader);
            }

            // Link shader to program
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            // Clean up
            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            program
        }
    }
}