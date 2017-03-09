use super::{RenderTarget, Rect};
use super::named_texture::NamedTexture;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sprite {
    rect: Rect,
    texture: Rc<NamedTexture>,
}

impl Sprite {
    pub fn draw<T: RenderTarget>(&self, rt: &mut T) {
        let handle = self.texture.get_texture(rt);
        rt.draw_texture(&self.rect, &handle);
    }
}