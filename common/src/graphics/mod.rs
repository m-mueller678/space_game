#[cfg(feature = "graphics")]
mod inner {
    mod composite_texture;
    mod static_texture;
    mod named_texture;
    mod sprite;

    pub use self::sprite::Sprite;
    pub use self::composite_texture::CompositeTexture;
    pub use self::static_texture::init_thread_texture_path;

    use self::named_texture::NamedTexture;
    use sfml::graphics as sfml;
    use self::static_texture::get as get_texture;

    pub trait RenderTarget: sfml::RenderTarget {}

    impl<T: sfml::RenderTarget> RenderTarget for T {}
}

#[cfg(feature = "graphics")]
pub use self::inner::*;

#[cfg(not(feature = "graphics"))]
pub type Sprite = ();
