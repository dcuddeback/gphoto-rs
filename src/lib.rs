extern crate gphoto2_sys as gphoto2;
extern crate libc;

pub use error::{Result,Error,ErrorKind};
pub use camera::{Camera};
pub use context::{Context};
pub use port::{PortType,Port};
pub use version::{LibraryVersion,libgphoto2_version};

#[macro_use]
mod error;
mod camera;
mod context;
mod port;
mod version;

// internal
mod handle;
