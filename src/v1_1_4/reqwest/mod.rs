#[cfg(feature = "reqwest-async")]
mod asynch;

#[cfg(feature = "reqwest-async")]
pub use asynch::*;

#[cfg(feature = "reqwest-blocking")]
pub mod blocking;