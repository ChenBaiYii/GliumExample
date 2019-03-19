#[macro_use]
extern crate glium;

use glium::Surface;


fn main() {
    use glium::{glutin, Surface};

    let mut event_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    // 顶点着色器
    //  uniform变量是一个全局变量, 它的值是在绘制时, 由 draw 函数传递给GPU
    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        uniform float t;
        void main() {
            vec2 pos = position;
            pos.x += t;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;
    // 片段着色器
    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [0.0, 0.5] };
    let vertex3 = Vertex { position: [0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];
    // 顶点缓冲
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let program = glium::Program::from_source(&display, vertex_shader_src,
                                              fragment_shader_src, None).unwrap();

    let mut t: f32 = -0.5;
    let mut close = false;
    while !close {
        t += 0.0002;
        if t > 0.5 {
            t -= 0.5;
        }

        // 绘制
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform!(t: t),
                    &Default::default()).unwrap();
        target.finish().unwrap();
        // 绘制结束

        event_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => close = true,
                    _ => (),
                },
                _ => (),
            }
        })
    }
}