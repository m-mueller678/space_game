use graphics::{RenderTarget, TextureHandle, Rect};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sprite {
    rect: Rect,
    texture: TextureHandle,
}

impl Sprite {
    pub fn draw<T: RenderTarget>(&self, rt: &mut T) {
        rt.draw_texture(&self.rect, &self.texture);
    }
}