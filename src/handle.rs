pub mod prelude {
    pub use super::{Handle,HandleMut};
}

/// Types that manages access to a resource.
///
/// This trait is intended to be used by structs that manage the ownership of a resource from a C
/// library. Often, a pointer to the resource is needed to implement methods in other modules that
/// wrap the same C library, but the pointer should not be accessible from outside of the crate.
///
/// The `Handle` trait allows a struct to hand out access to an owned resource across module
/// boundaries. Because the `Handle` trait is not exported from the crate, the owned resources will
/// not be accessible from outside the crate.
pub trait Handle<T> {
    unsafe fn as_ptr(&self) -> *const T;
}

/// Types that manages access to a mutable resource.
///
/// Like the `Handle` trait, this trait is intended to be used by structs to hand out access to an
/// owned resource across module boundaries without exposing the resource outside of the crate.
pub trait HandleMut<T>: Handle<T> {
    unsafe fn as_mut_ptr(&mut self) -> *mut T;
}
