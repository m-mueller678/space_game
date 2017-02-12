#[cfg(feature = "graphics")]
mod inner {
    use sfml::graphics as sfml;

    pub trait RenderTarget: sfml::RenderTarget {}

    impl<T: sfml::RenderTarget> RenderTarget for T {}
}

#[cfg(not(feature = "graphics"))]
mod inner {
    pub trait RenderTarget {}

    impl RenderTarget for () {}
}

pub use self::inner::*;