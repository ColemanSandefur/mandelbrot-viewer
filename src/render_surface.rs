use std::rc::Rc;

use glium::{
    backend::Facade,
    framebuffer::SimpleFrameBuffer,
    texture::{DepthTexture2d, SrgbTexture2d},
};

pub struct RenderSurface {
    pub texture: Rc<SrgbTexture2d>,
    pub depth: DepthTexture2d,
}

impl RenderSurface {
    pub fn new(
        facade: &impl Facade,
        width: u32,
        height: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let texture = SrgbTexture2d::empty(facade, width, height)?;
        let depth = DepthTexture2d::empty(facade, width, height)?;

        Ok(Self {
            texture: Rc::new(texture),
            depth,
        })
    }

    pub fn resize(
        &mut self,
        facade: &impl Facade,
        width: u32,
        height: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.texture = Rc::new(SrgbTexture2d::empty(facade, width, height)?);
        self.depth = DepthTexture2d::empty(facade, width, height)?;

        Ok(())
    }

    pub fn frame_buffer(
        &self,
        facade: &impl Facade,
    ) -> Result<SimpleFrameBuffer, Box<dyn std::error::Error>> {
        Ok(SimpleFrameBuffer::with_depth_buffer(
            facade,
            &*self.texture,
            &self.depth,
        )?)
    }

    pub fn size(&self) -> (u32, u32) {
        (self.texture.width(), self.texture.height())
    }

    pub fn width(&self) -> u32 {
        self.texture.width()
    }

    pub fn height(&self) -> u32 {
        self.texture.height()
    }
}
