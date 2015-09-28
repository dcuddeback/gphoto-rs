use std::borrow::Cow;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem;

use ::libc::c_void;

/// Types of ports.
#[derive(Debug,PartialEq,Eq,Clone,Copy,Hash)]
pub enum PortType {
    /// Serial port.
    Serial,

    /// USB port.
    USB,

    /// Disk or local mountpoint.
    Disk,

    /// PTP or IP connection.
    PTPIP,

    /// Direct I/O on a USB mass storage device.
    Direct,

    /// USB mass storage raw SCSI port.
    SCSI,

    /// Unknown port type.
    Other,
}

/// A structure describing a port.
///
/// ## Example
///
/// A `Port` object can be used to report information about a camera's connection:
///
/// ```no_run
/// let mut context = gphoto::Context::new().unwrap();
/// let mut camera = gphoto::Camera::autodetect(&mut context).unwrap();
/// let port = camera.port();
///
/// println!("port type = {:?}", port.port_type());
/// println!("port name = {:?}", port.name());
/// println!("port path = {:?}", port.path());
/// ```
///
/// The above example may print something like the following:
///
/// ```text
/// port type = USB
/// port name = "Universal Serial Bus"
/// port path = "usb:020,007"
/// ```
pub struct Port<'a> {
    // GPPortInfo is a typedef for a pointer. Lifetime is needed because it borrows data owned by
    // the Camera struct.
    inner: ::gphoto2::GPPortInfo,
    __phantom: PhantomData<&'a c_void>,
}

impl<'a> Port<'a> {
    /// Returns the type of the port.
    pub fn port_type(&self) -> PortType {
        let mut port_type = unsafe { mem::uninitialized() };

        unsafe {
            assert_eq!(::gphoto2::GP_OK, ::gphoto2::gp_port_info_get_type(self.inner, &mut port_type));
        }

        match port_type {
            ::gphoto2::GP_PORT_SERIAL          => PortType::Serial,
            ::gphoto2::GP_PORT_USB             => PortType::USB,
            ::gphoto2::GP_PORT_DISK            => PortType::Disk,
            ::gphoto2::GP_PORT_PTPIP           => PortType::PTPIP,
            ::gphoto2::GP_PORT_USB_DISK_DIRECT => PortType::Direct,
            ::gphoto2::GP_PORT_USB_SCSI        => PortType::SCSI,
            ::gphoto2::GP_PORT_NONE | _        => PortType::Other,
        }
    }

    /// Returns the name of the port.
    pub fn name(&self) -> Cow<str> {
        let mut name = unsafe { mem::uninitialized() };

        unsafe {
            assert_eq!(::gphoto2::GP_OK, ::gphoto2::gp_port_info_get_name(self.inner, &mut name));
            String::from_utf8_lossy(CStr::from_ptr(name).to_bytes())
        }
    }

    /// Returns the path of the port.
    pub fn path(&self) -> Cow<str> {
        let mut path = unsafe { mem::uninitialized() };

        unsafe {
            assert_eq!(::gphoto2::GP_OK, ::gphoto2::gp_port_info_get_path(self.inner, &mut path));
            String::from_utf8_lossy(CStr::from_ptr(path).to_bytes())
        }
    }
}

#[doc(hidden)]
pub fn from_libgphoto2<'a>(_camera: &'a ::camera::Camera, ptr: ::gphoto2::GPPortInfo) -> Port<'a> {
    Port {
        inner: ptr,
        __phantom: PhantomData,
    }
}
