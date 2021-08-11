use crate::renderer::shading::*;
use crate::renderer::*;

impl ShadedGeometry for InstancedMesh {
    fn geometry_pass(
        &self,
        camera: &Camera,
        viewport: Viewport,
        material: &Material,
    ) -> Result<(), Error> {
        let fragment_shader_source = geometry_fragment_shader(material);
        let program = self.get_or_insert_program(&fragment_shader_source)?;
        material.bind(program)?;
        self.render_internal(
            RenderStates {
                cull: self.cull,
                clip: None,
                write_mask: WriteMask::COLOR_AND_DEPTH,
                blend: None,
                depth_test: self.depth_test,
            },
            program,
            camera.uniform_buffer(),
            viewport,
        )
    }

    fn render_with_lighting(
        &self,
        camera: &Camera,
        material: &Material,
        lighting_model: LightingModel,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<(), Error> {
        let fragment_shader_source = shaded_fragment_shader(
            lighting_model,
            Some(material),
            directional_lights.len(),
            spot_lights.len(),
            point_lights.len(),
        );
        let program = self.get_or_insert_program(&fragment_shader_source)?;

        bind_lights(
            program,
            ambient_light,
            directional_lights,
            spot_lights,
            point_lights,
            camera.position(),
        )?;
        material.bind(program)?;
        self.render(program, WriteMask::default(), None, camera)?;
        Ok(())
    }
}
