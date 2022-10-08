extern crate gl;

use std::{os::raw::*, mem};

pub struct VBO  {
    id: u32,
    vertices: Vec<f32>,
}

impl VBO {
    pub fn new(data: Vec<f32>) -> Self { 
        let vbo = unsafe {
            let mut vbo: u32 = 0;
            gl::CreateBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (data.len() * mem::size_of::<gl::types::GLfloat>()) as isize, &data[0] as *const f32 as *const c_void, gl::STATIC_DRAW);
            vbo
        };

        Self {
            id: vbo,
            vertices: data,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }
}
