
use gl;
use crate::*;
use crate::objects::*;

pub struct Wireframe {
    edges: ShadedEdges,
    vertices: ShadedVertices
}

impl Wireframe
{
    pub fn new(gl: &gl::Gl, indices: &[u32], positions: &[f32], tube_radius: f32) -> Wireframe
    {
        let edges = ShadedEdges::create(&gl, indices, positions, tube_radius);
        let mut vertices = ShadedVertices::new(&gl, positions);
        vertices.scale = 2.0 * tube_radius;

        Wireframe {edges, vertices}
    }

    pub fn render(&self, camera: &camera::Camera)
    {
        self.edges.render(camera);
        self.vertices.render(camera);
    }

    pub fn set_color(&mut self, color: &Vec3)
    {
        self.edges.color = *color;
        self.vertices.color = *color;
    }

    pub fn set_parameters(&mut self, diffuse_intensity: f32, specular_intensity: f32, specular_power: f32)
    {
        self.edges.diffuse_intensity = diffuse_intensity;
        self.edges.specular_intensity = specular_intensity;
        self.edges.specular_power = specular_power;
        self.vertices.diffuse_intensity = diffuse_intensity;
        self.vertices.specular_intensity = specular_intensity;
        self.vertices.specular_power = specular_power;
    }
}