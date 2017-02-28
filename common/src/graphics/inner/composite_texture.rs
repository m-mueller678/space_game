use super::Sprite;
use sfml::graphics::{RenderStates, RenderTarget};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CompositeTexture {
    parts: Vec<Sprite>,
}

impl CompositeTexture {
    pub fn draw<'a: 'b, 'b, RT: RenderTarget>(&'a self, target: &mut RT, rs: &mut RenderStates<'b>) {
        for p in self.parts.iter() {
            p.draw(target, rs);
        }
    }
}