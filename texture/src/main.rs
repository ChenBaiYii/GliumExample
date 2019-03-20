#[macro_use]
extern crate glium;
extern crate image;

use glium::Surface;
use std::io::Cursor;

fn main() {
    use glium::{glutin, Surface};



    let mut event_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &event_loop).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("./a.PNG")[..]),
                            image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    // 将图片作为纹理上传
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, tex_coords);
    // 纹理坐标的范围在 0.0 到 1.0 之间. 坐标 (0.0, 0.0) 指的是纹理的左下角, 坐标 (1.0, 1.0) 指的是纹理的右上角.
    let vertex1 = Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] };
    let vertex2 = Vertex { position: [0.0, 0.5], tex_coords: [0.0, 0.1] };
    let vertex3 = Vertex { position: [0.5, -0.25], tex_coords: [1.0, 0.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    // 顶点着色器
    //  uniform变量是一个全局变量, 它的值是在绘制时, 由 draw 函数传递给GPU
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;
    // 片段着色器
    let fragment_shader_src = r#"
        #version 140
        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // 顶点缓冲
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let program = glium::Program::from_source(&display, vertex_shader_src,
                                              fragment_shader_src, None).unwrap();

    let mut t: f32 = -0.5;
    let mut closed = false;
    while !closed {
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        // 绘制
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0f32],
            ],
            tex: &texture,
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
        // 绘制结束

        event_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        })
    }
}