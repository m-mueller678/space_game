use super::Sprite;
use super::get_texture;
use super::NamedTexture;
use sfml::graphics::*;

#[derive(Clone, Serialize, Deserialize)]
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