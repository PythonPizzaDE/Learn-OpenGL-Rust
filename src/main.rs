extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key};
use std::{os::raw::*, mem};
// use cgmath::{Vector3, Matrix4};

mod shader;

// struct Camera {
//     pitch: f32,
//     yaw: f32,
//     speed: f32,
//     mouse_sensitivity: f32,
//     fov: f32,

//     last_mouse_x: f32,
//     last_mouse_y: f32,

//     position: Vector3<f32>,
//     front: Vector3<f32>,
//     up: Vector3<f32>,

//     view_matrix: Matrix4<f32>,
//     projection_matrix: Matrix4<f32>,

//     wireframe: bool,
// }

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Chello SIR, this is a WINDOW", glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| glfw.get_proc_address_raw(s));
    gl::Viewport::load_with(|s| glfw.get_proc_address_raw(s));

    let vertices = [
        -0.5f32, -0.5f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32,
         0.5f32, -0.5f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32,
         0.0f32,  0.5f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32,
    ];

    let mut vao: u32 = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

    let mut vbo: u32 = 0;
    unsafe {
        gl::CreateBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * mem::size_of::<gl::types::GLfloat>()) as isize, &vertices[0] as *const f32 as *const c_void, gl::STATIC_DRAW);
    }

    let mut sh = shader::Shader::new("shader/vertex.glsl".to_string(), "shader/fragment.glsl".to_string());
    sh.bind();
    sh.create_uniform("rotation");

    unsafe {
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (6 * mem::size_of::<gl::types::GLfloat>()) as i32, 0 as *const c_void);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, (6 * mem::size_of::<gl::types::GLfloat>()) as i32, (3 * mem::size_of::<gl::types::GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);
    }

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let mut t = 0;

    while !window.should_close() {
        t += 1;
        sh.set_matrix4_uniform("rotation", &cgmath::Matrix4::from_angle_x(cgmath::Rad::from(cgmath::Deg(t as f32))));

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.1f32, 0.1f32, 0.1f32, 1.0f32);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {},
            }
        }
    }
}
