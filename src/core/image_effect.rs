use crate::core::*;

///
/// A customizable 2D effect.
/// Can for example be used for adding an effect on top of a rendered image.
///
pub struct ImageEffect {
    program: Program,
    positions: VertexBuffer,
    uvs: VertexBuffer,
}

impl ImageEffect {
    ///
    /// Creates a new image effect which applies the calculations defined in the given fragment shader source when calling the [ImageEffect::apply] function.
    ///
    pub fn new(context: &Context, fragment_shader: &str) -> Result<Self> {
        let program = Program::from_source(
            &context,
            "in vec3 position;
                                                    in vec2 uv_coordinate;
                                                    out vec2 uv;
                                                    void main()
                                                    {
                                                        uv = uv_coordinate;
                                                        gl_Position = vec4(position, 1.0);
                                                    }",
            fragment_shader,
        )?;

        let positions = vec![-3.0, -1.0, 0.0, 3.0, -1.0, 0.0, 0.0, 2.0, 0.0];
        let uvs = vec![-1.0, 0.0, 2.0, 0.0, 0.5, 1.5];
        let positions = VertexBuffer::new_with_static(&context, &positions).unwrap();
        let uvs = VertexBuffer::new_with_static(&context, &uvs).unwrap();

        Ok(Self {
            program,
            positions,
            uvs,
        })
    }

    ///
    /// Applies the calculations defined in the fragment shader given at construction and output it to the current screen/render target.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write].
    ///
    pub fn apply(&self, render_states: RenderStates, viewport: Viewport) -> Result<()> {
        self.program
            .use_attribute_vec3("position", &self.positions)?;
        self.program
            .use_attribute_vec2("uv_coordinate", &self.uvs)?;
        self.program.draw_arrays(render_states, viewport, 3);
        Ok(())
    }
}

impl std::ops::Deref for ImageEffect {
    type Target = Program;

    fn deref(&self) -> &Self::Target {
        &self.program
    }
}