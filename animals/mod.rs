#[cfg(feature = "v1")]
mod v1;

#[cfg(feature = "v2")]
mod v2;

#[cfg(feature = "v3")]
mod v3;

#[cfg(feature = "v1")]
pub use v1::make_sound;

#[cfg(feature = "v2")]
pub use v2::make_sound;

#[cfg(feature = "v3")]
pub use v3::make_sound;
