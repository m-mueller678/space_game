#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TextureHandle(usize);

pub type Point = (f32, f32);

pub type Rect = (Point, Point);

pub type Color = [u8; 4];

pub trait RenderTarget {
    fn draw_texture(&mut self, rect: &Rect, &TextureHandle);
    fn draw_line(&mut self, Point, Point, Color);
}

pub struct TransformRender<'a, T: RenderTarget + 'a, F: Fn(Point) -> Point> {
    target: &'a mut T,
    transform: F,
}

impl<'a, T: RenderTarget, F: Fn(Point) -> Point> TransformRender<'a, T, F> {
    pub fn new(target: &'a mut T, transform: F) -> Self {
        TransformRender {
            target: target,
            transform: transform,
        }
    }
}

impl<'a, T: RenderTarget, F: Fn(Point) -> Point> RenderTarget for TransformRender<'a, T, F> {
    fn draw_texture(&mut self, &(p1, p2): &Rect, texture: &TextureHandle) {
        self.target.draw_texture(&((self.transform)(p1), (self.transform)(p2)), texture);
    }
    fn draw_line(&mut self, p1: Point, p2: Point, col: Color) {
        self.target.draw_line((self.transform)(p1), (self.transform)(p2), col);
    }
}

mod composite_texture;
mod sprite;

pub use self::sprite::Sprite;
pub use self::composite_texture::CompositeTexture;