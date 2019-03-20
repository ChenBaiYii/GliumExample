#[macro_use]
extern crate glium;


mod teapot;

fn main() {
    use glium::{glutin, Surface};



    let mut event_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &event_loop).unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    // IndexBuffer. 正如它的名字, 它是一种用于存储顶点索引的缓冲
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                          &teapot::INDICES).unwrap();

    // 顶点着色器
    //  uniform变量是一个全局变量, 它的值是在绘制时, 由 draw 函数传递给GPU
    let vertex_shader_src = r#"
        #version 140
        in vec3 position;
        in vec3 normal;
        uniform mat4 matrix;
        void main() {
            gl_Position = matrix * vec4(position, 1.0);
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

    let program = glium::Program::from_source(&display, vertex_shader_src,
                                              fragment_shader_src, None).unwrap();

    let mut closed = false;
    while !closed {
        // 绘制
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];

        target.draw((&positions, &normals), &indices, &program, &uniform! {matrix: matrix},
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


