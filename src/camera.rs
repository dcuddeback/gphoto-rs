use std::mem;

use ::context::Context;
use ::port::Port;

use ::handle::prelude::*;

/// A structure representing a camera connected to the system.
pub struct Camera {
    camera: *mut ::gphoto2::Camera,
}

impl Drop for Camera {
    fn drop(&mut self) {
        unsafe {
            ::gphoto2::gp_camera_unref(self.camera);
        }
    }
}

impl Camera {
    /// Opens the first detected camera.
    pub fn autodetect(context: &mut Context) -> ::Result<Self> {
        let mut ptr = unsafe { mem::uninitialized() };

        try_unsafe!(::gphoto2::gp_camera_new(&mut ptr));

        let camera = Camera { camera: ptr };

        try_unsafe!(::gphoto2::gp_camera_init(camera.camera, context.as_mut_ptr()));

        Ok(camera)
    }

    /// Returns information about the port the camera is connected to.
    pub fn port<'a>(&'a self) -> Port<'a> {
        let mut ptr = unsafe { mem::uninitialized() };

        unsafe {
            assert_eq!(::gphoto2::GP_OK, ::gphoto2::gp_camera_get_port_info(self.camera, &mut ptr));
        }

        ::port::from_libgphoto2(self, ptr)
    }
}
