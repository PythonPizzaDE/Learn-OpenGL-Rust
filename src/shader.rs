extern crate gl;

use std::fs;
use std::collections::HashMap;
use std::{ffi::CString, ptr};

use cgmath::prelude::*;

#[derive(Debug)]
pub struct Shader {
    id: u32,
    uniforms: HashMap<String, i32>,
}

impl Shader {
    pub fn new(vertex_path: String, fragment_path: String) -> Self {

        let vertex_source = fs::read_to_string(vertex_path.clone()).expect("Couldn't read vertex shader from filesystem.");
        let fragment_source = fs::read_to_string(fragment_path.clone()).expect("Couldn't read fragment shader from filesystem.");

        let vertex_source_c = CString::new(vertex_source.as_bytes()).unwrap();
        let fragment_source_c = CString::new(fragment_source.as_bytes()).unwrap();


        let program: u32 = unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader, 1, &vertex_source_c.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_shader, 1, &fragment_source_c.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);

            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            program
        };


        Self {
            id: program,
            uniforms: HashMap::new(),
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn create_uniform(&mut self, uniform_name: &str) {
        let uniform_location = unsafe {
            let uniform_name_c = CString::new(uniform_name).unwrap();
            gl::GetUniformLocation(self.id, uniform_name_c.as_ptr())
        };
        self.uniforms.insert(uniform_name.to_string(), uniform_location);
    }
    
    pub fn set_matrix4_uniform(&self, uniform_name: &str, matrix: &cgmath::Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                self.uniforms.get(&uniform_name.to_string()).unwrap().clone(),
                1,
                gl::FALSE,
                matrix.as_ptr(),
            )
        }
    }
}
