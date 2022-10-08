extern crate gl;

pub struct VAO {
    id: u32,
}

impl VAO {
    pub fn new() -> Self {
        let vao_id = unsafe {
            let mut vao: u32 = 0;
            gl::GenVertexArrays(1, &mut vao);
            vao
        };

        Self {
            id: vao_id,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
}
