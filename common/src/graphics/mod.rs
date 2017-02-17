#[cfg(feature = "graphics")]
mod inner {
    use sfml::graphics as sfml;

    mod composite_texture;

    pub use self::composite_texture::{CompositeTexture, init_thread_texture_path};
    pub type IfGraphics<T> = T;
    pub trait RenderTarget: sfml::RenderTarget {}
    impl<T: sfml::RenderTarget> RenderTarget for T {}
}

#[cfg(not(feature = "graphics"))]
mod inner {
    use std::marker::PhantomData;

    pub type CompositeTexture = ();
    pub type IfGraphics<T> = PhantomData<*const T>;
    pub trait RenderTarget {}
    impl RenderTarget for () {}
}

pub use self::inner::*;