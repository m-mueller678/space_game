use sfml::graphics::*;
use super::get_texture;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::rc::Rc;
use std::fmt::{Debug, Formatter, Error};

#[derive(Clone)]
pub struct NamedTexture {
    name: String,
    texture: Rc<Texture>,
}

impl Debug for NamedTexture {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.name.fmt(f)
    }
}

impl Serialize for NamedTexture {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.name.serialize(ser)
    }
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
}

