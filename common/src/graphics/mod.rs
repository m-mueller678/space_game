#[cfg(feature = "graphics")]
mod inner {
    mod composite_texture;
    mod static_texture;
    mod named_texture;

    pub use self::named_texture::NamedTexture;
    pub use self::composite_texture::CompositeTexture;
    pub use self::static_texture::init_thread_texture_path;

    use sfml::graphics as sfml;
    use self::static_texture::get as get_texture;

    pub type IfGraphics<T> = T;
    pub trait RenderTarget: sfml::RenderTarget {}
    impl<T: sfml::RenderTarget> RenderTarget for T {}
}

#[cfg(not(feature = "graphics"))]
mod inner {
    use std::marker::PhantomData;

    pub type NamedTexture = ();
    pub type CompositeTexture = ();
    pub type IfGraphics<T> = PhantomData<*const T>;
    pub trait RenderTarget {}
    impl RenderTarget for () {}
}

pub use self::inner::*;