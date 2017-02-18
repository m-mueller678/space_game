use super::NamedTexture;
use sfml::graphics::{RenderTarget, RenderStates, Vertex, PrimitiveType};
use sfml::system::Vector2f;

#[derive(Serialize, Deserialize, Clone)]
pub struct Sprite {
    texture: NamedTexture,
    pos: [f32; 4]
}

impl Sprite {
    pub fn draw<'a: 'b, 'b, T: RenderTarget>(&'a self, rt: &mut T, rs: &mut RenderStates<'b>) {
        let ts = self.texture.texture().get_size();
        rs.texture = Some(self.texture.texture());
        let ver = [
            Vertex::new_with_pos_coords(&Vector2f::new(self.pos[0], self.pos[1]), &Vector2f::new(0., 0.)),
            Vertex::new_with_pos_coords(&Vector2f::new(self.pos[2], self.pos[1]), &Vector2f::new(ts.x as f32, 0.)),
            Vertex::new_with_pos_coords(&Vector2f::new(self.pos[2], self.pos[3]), &Vector2f::new(ts.x as f32, ts.y as f32)),
            Vertex::new_with_pos_coords(&Vector2f::new(self.pos[0], self.pos[3]), &Vector2f::new(0., ts.y as f32)),
        ];
        rt.draw_primitives(&ver, PrimitiveType::sfQuads, rs);
    }
}