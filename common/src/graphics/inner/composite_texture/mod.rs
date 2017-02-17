mod static_texture;

pub use self::static_texture::init_thread_texture_path;
use self::static_texture::get as get_texture;
use sfml::graphics::*;
use sfml::system::Vector2f;

#[derive(Clone, Serialize, Deserialize)]
struct TexturePart {
    id: String,
    pos: [i32; 4],
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CompositeTexture {
    parts: Vec<TexturePart>,
}

fn draw_part<T: RenderTarget>(part: &TexturePart, rt: &mut T, rs: &mut RenderStates) {
    fn v2(x: i32, y: i32) -> Vector2f {
        Vector2f::new(x as f32, y as f32)
    }
    let texture = get_texture(&part.id);
    let t_s = texture.get_size();
    let ver = [
        Vertex::new_with_pos_coords(&v2(part.pos[0], part.pos[1]), &v2(0, 0)),
        Vertex::new_with_pos_coords(&v2(part.pos[2], part.pos[1]), &v2(t_s.x as i32, 0)),
        Vertex::new_with_pos_coords(&v2(part.pos[2], part.pos[3]), &v2(t_s.x as i32, t_s.y as i32)),
        Vertex::new_with_pos_coords(&v2(part.pos[0], part.pos[3]), &v2(0, t_s.y as i32)),
    ];
    let mut transform = Transform::new_identity();
    transform.combine(&mut rs.transform);
    let mut owned_rs = RenderStates::new(rs.blend_mode, transform, Some(&texture), rs.shader);
    rt.draw_primitives(&ver, PrimitiveType::sfQuads, &mut owned_rs);
}

impl Drawable for CompositeTexture {
    fn draw<RT: RenderTarget>(&self, target: &mut RT, rs: &mut RenderStates) {
        for p in self.parts.iter() {
            draw_part(p, target, rs);
        }
    }
}