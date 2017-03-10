use graphics::{Sprite, RenderTarget};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompositeTexture {
    parts: Vec<Sprite>
}

impl CompositeTexture {
    pub fn draw<T: RenderTarget>(&self, rt: &mut T) {
        for s in self.parts.iter() {
            s.draw(rt);
        }
    }
}