use glow::*;

const VERTEX_SHADER_SOURCE: &str = r#"#version 330
  in vec2 in_position;
  out vec2 position;
  void main() {
    position = in_position;
    gl_Position = vec4(in_position - 0.5, 0.0, 1.0);
  }"#;
const FRAGMENT_SHADER_SOURCE: &str = r#"#version 330
  precision mediump float;
  in vec2 position;
  out vec4 color;
  void main() {
    color = vec4(position, 0.8, 1.0);
  }"#;

pub struct Renderer {
    pub program: glow::Program,
    pub vbo: glow::Buffer,
    pub vao: glow::VertexArray,
    pub gl: glow::Context
}

impl Renderer {
    pub unsafe fn new(gl: glow::Context) -> Self {

        let program = gl.create_program().expect("Cannot create program");

        let shader_sources = [
            (glow::VERTEX_SHADER, VERTEX_SHADER_SOURCE),
            (glow::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

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

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }
        gl.use_program(Some(program));

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

        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        Self{
            gl,
            program,
            vbo,
            vao
        }
    }

    pub unsafe fn render(&self){
        self.gl.clear(glow::COLOR_BUFFER_BIT);
        self.gl.draw_arrays(glow::TRIANGLES, 0, 3);
    }

    pub unsafe fn delete(&mut self){
        self.gl.delete_program(self.program);
        self.gl.delete_vertex_array(self.vao);
        self.gl.delete_buffer(self.vbo);    
    }
    
}
