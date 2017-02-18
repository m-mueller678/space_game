use sfml::graphics::*;
use super::get_texture;
use serde::{Deserialize, Deserializer};
use std::rc::Rc;

#[derive(Serialize, Clone)]
pub struct NamedTexture {
    name: String,
    #[serde(skip_serializing)]
    texture: Rc<Texture>,
}

impl Deserialize for NamedTexture {
    fn deserialize<D: Deserializer>(des: D) -> Result<Self, D::Error> {
        let name = String::deserialize(des)?;
        Ok(Self::new(name))
    }
}

impl NamedTexture {
    pub fn new(name: String) -> Self {
        let texture = get_texture(&name);
        NamedTexture {
            name: name,
            texture: texture,
        }
    }
    pub fn texture(&self) -> &Texture {
        self.texture.as_ref()
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

