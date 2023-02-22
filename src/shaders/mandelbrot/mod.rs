use glium::uniform;
use glium::DrawParameters;
use glium::{backend::Facade, Program};
use nalgebra::Matrix4;
use opengl_renderer::insert_program;
use opengl_renderer::shader::Shader;
use std::any::Any;
use std::rc::Rc;

use crate::ScreenDim;

#[derive(Clone)]
pub struct MandelbrotShader {
    program: Rc<Program>,
    pub zoom: f32,
    pub pos: [f32; 2],
    pub model: Matrix4<f32>,
}

impl MandelbrotShader {
    pub fn load_from_fs(facade: &impl Facade) -> Self {
        let program = Rc::new(insert_program!("./vertex.glsl", "./fragment.glsl", facade));

        Self {
            program,
            zoom: 1.0,
            pos: [0.0; 2],
            model: Matrix4::new_translation(&[0.0; 3].into()),
        }
    }
}

impl Shader for MandelbrotShader {
    fn render<'a>(
        &self,
        vertex_buffer: glium::vertex::VerticesSource<'a>,
        index_buffer: glium::index::IndicesSource<'a>,
        surface: &mut opengl_renderer::renderer::Renderable,
        _camera: [[f32; 4]; 4],
        _position: [[f32; 4]; 4],
        scene_data: &opengl_renderer::renderer::SceneData,
    ) {
        let screen_dim: &ScreenDim = scene_data.get_scene_object().unwrap();

        let uniforms = uniform! {
            zoom: self.zoom,
            screen_pos: self.pos,
            screen_dim: [screen_dim.width as f32, screen_dim.height as f32],
        };

        surface
            .draw(
                vertex_buffer,
                index_buffer,
                &self.program,
                &uniforms,
                &DrawParameters {
                    ..Default::default()
                },
            )
            .unwrap();
    }

    fn get_model_mat(&self) -> Matrix4<f32> {
        self.model
    }

    fn set_model_mat(&mut self, model: Matrix4<f32>) {
        self.model = model;
    }

    fn equal_shader(&self, _shader: &dyn std::any::Any) -> bool {
        false
    }

    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn clone_shader(&self) -> Box<dyn Shader> {
        Box::new(self.clone())
    }
    fn clone_sized(&self) -> Self {
        self.clone()
    }
}
