use std::collections::hash_map::{HashMap, Entry};
use std::path::PathBuf;
use common::graphics;
use sfml::graphics as sf;
use sfml::graphics::RenderTarget as SfTarget;
use sfml::system::Vector2f as V2;
use std::ops::{Deref, DerefMut};

pub struct SfRender {
    pub win: sf::RenderWindow,
    textures: Vec<sf::Texture>,
    handles: HashMap<String, graphics::TextureHandle>,
    path: PathBuf,
}

impl SfRender {
    pub fn new(win: sf::RenderWindow, path: PathBuf) -> Self {
        let mut render = SfRender {
            win: win,
            textures: Vec::new(),
            handles: HashMap::new(),
            path: path,
        };
        render.textures.push(sf::Texture::new_from_image(
            &sf::Image::new_from_color(1, 1, &sf::Color::magenta()).unwrap()).unwrap());
        render.handles.insert("null".into(), graphics::TextureHandle(0));
        render
    }
}

impl graphics::RenderTarget for SfRender {
    fn draw_texture(&mut self, rect: &graphics::Rect, texture: &graphics::TextureHandle) {
        if let Some(texture) = self.textures.get(texture.0) {
            let sx = texture.get_size().x as f32;
            let sy = texture.get_size().y as f32;
            let ver = [
                sf::Vertex::new_with_pos_coords(&V2::new((rect.0).0, (rect.0).1), &V2::new(0., 0.)),
                sf::Vertex::new_with_pos_coords(&V2::new((rect.1).0, (rect.0).1), &V2::new(sx, 0.)),
                sf::Vertex::new_with_pos_coords(&V2::new((rect.1).0, (rect.1).1), &V2::new(sx, sy)),
                sf::Vertex::new_with_pos_coords(&V2::new((rect.0).0, (rect.1).1), &V2::new(0., sy)),
            ];
            let mut rs = sf::RenderStates::default();
            rs.texture = Some(texture);
            self.win.draw_primitives(&ver, sf::PrimitiveType::sfQuads, &mut rs);
        }
    }
    fn draw_line(&mut self, p1: graphics::Point, p2: graphics::Point, col: graphics::Color) {
        let ver = [
            sf::Vertex::new_with_pos_color(&V2::new(p1.0, p1.1), &sf::Color::new_rgba(col[0], col[1], col[2], col[3])),
            sf::Vertex::new_with_pos_color(&V2::new(p2.0, p2.1), &sf::Color::new_rgba(col[0], col[1], col[2], col[3])),
        ];
        self.win.draw_primitives(&ver, sf::PrimitiveType::sfLines, &mut sf::RenderStates::default());
    }
    fn draw_triangle(&mut self, p: &[graphics::Point; 3], col: graphics::Color) {
        let col = sf::Color::new_rgba(col[0], col[1], col[2], col[3]);
        let ver = [
            sf::Vertex::new_with_pos_color(&V2::new(p[0].0, p[0].1), &col),
            sf::Vertex::new_with_pos_color(&V2::new(p[1].0, p[1].1), &col),
            sf::Vertex::new_with_pos_color(&V2::new(p[2].0, p[2].1), &col),
        ];
        self.win.draw_primitives(&ver, sf::PrimitiveType::sfTriangles, &mut sf::RenderStates::default());
    }

    fn load_texture(&mut self, name: &str) -> graphics::TextureHandle {
        match self.handles.entry(name.to_string()) {
            Entry::Occupied(e) => {
                *e.get()
            },
            Entry::Vacant(e) => {
                let mut path = self.path.clone();
                path.push(name);
                match sf::Texture::new_from_file(path.to_str().unwrap()) {
                    Some(texture) => {
                        let handle = graphics::TextureHandle(self.textures.len());
                        self.textures.push(texture);
                        e.insert(handle);
                        handle
                    },
                    None => {
                        error!("cannot load texture {:?}", name);
                        graphics::TextureHandle(0)
                    }
                }
            }
        }
    }
}

impl Deref for SfRender {
    type Target = sf::RenderWindow;
    fn deref(&self) -> &sf::RenderWindow {
        &self.win
    }
}

impl DerefMut for SfRender {
    fn deref_mut(&mut self) -> &mut sf::RenderWindow {
        &mut self.win
    }
}