use ::handle::{Handle,HandleMut};

/// A `libgphoto2` library context.
pub struct Context {
    context: *mut ::gphoto2::GPContext,
}

impl Context {
    /// Creates a new context.
    pub fn new() -> ::Result<Context> {
        let ptr = unsafe { ::gphoto2::gp_context_new() };

        if !ptr.is_null() {
            Ok(Context { context: ptr })
        }
        else {
            Err(::error::from_libgphoto2(::gphoto2::GP_ERROR_NO_MEMORY))
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ::gphoto2::gp_context_unref(self.context);
        }
    }
}

#[doc(hidden)]
impl Handle<::gphoto2::GPContext> for Context {
    unsafe fn as_ptr(&self) -> *const ::gphoto2::GPContext {
        self.context
    }
}

#[doc(hidden)]
impl HandleMut<::gphoto2::GPContext> for Context {
    unsafe fn as_mut_ptr(&mut self) -> *mut ::gphoto2::GPContext {
        self.context
    }
}
