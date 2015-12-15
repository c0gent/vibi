#![allow(dead_code, unused_variables)]
// use std::ops::{Deref};
use glium_text::{self, TextSystem, FontTexture, TextDisplay};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::{self, VertexBuffer, IndexBuffer, Program, DrawParameters, Surface,};
use glium::vertex::{EmptyInstanceAttributes as EIAttribs};
use super::{UiVertex, UiElement};

const TWOSR3: f32 = 1.15470053838;

pub struct Ui<'d> {
	vbo: Option<VertexBuffer<UiVertex>>,
	ibo: Option<IndexBuffer<u16>>,
	elements: Vec<UiElement>,
	program: Program,
	params: DrawParameters<'d>,
	display: &'d GlutinFacade,
	scale: f32,
	text_system: TextSystem,
	font_texture: FontTexture,
}

impl<'d> Ui<'d> {
	pub fn new(display: &'d GlutinFacade) -> Ui<'d> {
		let scale = 1.0;
		let vbo = None;
		let ibo = None;

		// Create program:
		let program = Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

		// Draw parameters:
		let params = DrawParameters {
			depth: glium::Depth {
				test: glium::DepthTest::IfLess,
				write: true,
				.. Default::default()
			},
			// backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
			.. Default::default()
		};

		// Glium text renderer:
		let text_system = TextSystem::new(display);

		// Text font:
		let font_size = 24;
		let font_texture = FontTexture::new(display, &include_bytes!(
				"/home/nick/projects/vibi/assets/fonts/NotoSans/NotoSans-Regular.ttf"
			)[..], font_size).unwrap();

		Ui { 
			vbo: vbo,
			ibo: ibo,
			elements: Vec::new(),
			program: program,
			params: params,
			display: display,
			scale: scale,
			text_system: text_system,
			font_texture: font_texture,
		}
	}

	pub fn element(mut self, element: UiElement) -> Ui<'d> {
		if self.vbo.is_some() || self.ibo.is_some() { 
			panic!("Ui::element(): [FIXME]: Cannot (yet) add element after initialization.") 
		}

		self.elements.push(element);
		self
	}

	pub fn init( mut self) -> Ui<'d> {
		let mut vertices: Vec<UiVertex> = Vec::new();
		let mut indices: Vec<u16> = Vec::new();

		for element in self.elements.iter_mut() {
			element.set_text_width(&self.text_system, &self.font_texture);

			indices.extend_from_slice(&element.indices(vertices.len() as u16));

			vertices.extend_from_slice(&element.vertices(
				self.display.get_framebuffer_dimensions(), self.scale
			));			
		}

		self.vbo = Some(VertexBuffer::dynamic(self.display, &vertices).unwrap());
		self.ibo = Some(IndexBuffer::new(self.display, glium::index::PrimitiveType::TrianglesList, 
			&indices).unwrap());

		self
	}

	pub fn resize(&mut self) {
		match self.vbo {
			Some(ref mut vbo) => {
				let mut vertices: Vec<UiVertex> = Vec::with_capacity(vbo.len());

				for element in self.elements.iter_mut() {
					vertices.extend_from_slice(&element.vertices(
						self.display.get_framebuffer_dimensions(), self.scale
					));
				}

				vbo.write(&vertices);
			},

			None => panic!("Ui::resize(): Cannot resize until Ui has been \
				initialized with .init()"),
		}
	}

	pub fn draw<S: Surface>(&self, target: &mut S) {
		if self.vbo.is_none() || self.ibo.is_none() { 
			panic!("Ui::draw(): Buffers not initialized.") 
		}

		let model_color = super::C_ORANGE;

		// Uniforms:
		let uniforms = uniform! {
			u_model_color: model_color,
		};

		// Draw elements:
		target.draw((self.vbo.as_ref().unwrap(), EIAttribs { len: 1 }), self.ibo.as_ref().unwrap(), 
			&self.program, &uniforms, &self.params).unwrap();

		// Draw element text:
		for element in self.elements.iter() {
			let text_display = TextDisplay::new(&self.text_system, &self.font_texture, 
				element.text());

			glium_text::draw(&text_display, &self.text_system, target, 
				element.text_matrix(), (0.99, 0.99, 0.99, 1.0));
		}
	}
}



// Vertex Shader:
#[allow(non_upper_case_globals)]
static vertex_shader_src: &'static str = r#"
	#version 330

	in vec3 position;
	in vec3 color;
	in vec3 normal;

	out vec3 v_position;
	// out vec3 v_color;
	// out vec3 v_normal;	

	// uniform uint grid_side;
	// uniform mat4 model;
	// uniform mat4 view;
	// uniform mat4 persp;

	void main() {
		gl_Position = vec4(position, 1.0);
		// gl_Position = persp * model * vec4(position, 1.0);

		// v_normal = transpose(inverse(mat3(model_view))) * normal;
		// v_color = color;
		// v_position = gl_Position.xyz / gl_Position.w;
	};
"#;
		

// Fragment Shader:
#[allow(non_upper_case_globals)]
static fragment_shader_src: &'static str = r#"
	#version 330

	// in vec3 v_color; // <-- currently unused (using uniform atm)
	// in vec3 v_normal;
	// in vec3 v_position;

	out vec4 color;

	// uniform vec3 u_light_pos;
	uniform vec3 u_model_color;

	// const float ambient_strength = 0.1;
	// const vec3 ambient_color = vec3(0.9, 0.9, 0.9);
	// const vec3 diffuse_color = vec3(0.2, 0.2, 0.2);
	// const vec3 specular_color = vec3(0.4, 0.4, 0.4);
	// const float specular_coeff = 16.0;

	// // Pastel orange:
	// const vec3 model_color = vec3(0.9607, 0.4745, 0.0);
	// // Pink model:
	// const vec3 model_color = vec3(0.9882, 0.4902, 0.7059);

	void main() {
		// float diffuse_ampl = max(dot(normalize(v_normal), normalize(u_light_pos)), 0.0);

		// vec3 camera_dir = normalize(-v_position);
		// vec3 half_direction = normalize(normalize(u_light_pos) + camera_dir);
		// float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 
		// 	specular_coeff);

		// color = vec4((ambient_color * u_model_color) + diffuse_ampl
		// 	* diffuse_color + specular * specular_color, 1.0);	

		color = vec4(u_model_color, 1.0);
	};
"#;
