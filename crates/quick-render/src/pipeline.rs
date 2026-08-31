use crate::canvas::Canvas;

#[cfg(feature = "vello")]
use crate::vello_scene::VelloSceneBuilder;
#[cfg(feature = "vello")]
use vello::Scene;

pub struct RenderPipeline;

impl RenderPipeline {
    pub fn new() -> Self {
        Self
    }

    #[cfg(feature = "vello")]
    pub fn render_to_vello(&self, canvas: &Canvas, scene: &mut Scene) {
        VelloSceneBuilder::build(canvas, scene);
    }
}
