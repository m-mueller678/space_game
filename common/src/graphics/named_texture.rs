use graphics::{TextureHandle, RenderTarget};
use std::cell::Cell;

#[derive(Serialize, Deserialize, Debug)]
pub struct NamedTexture {
    name: String,
    #[serde(skip_serializing, skip_deserializing)]
    handle: Cell<Option<TextureHandle>>,
}

impl NamedTexture {
    pub fn get_texture<T: RenderTarget>(&self, rt: &mut T) -> TextureHandle {
        if let Some(h) = self.handle.get() {
            h
        } else {
            let h = rt.load_texture(&self.name);
            self.handle.set(Some(h));
            h
        }
    }
}