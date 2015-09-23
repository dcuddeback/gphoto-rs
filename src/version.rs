use std::ffi::CStr;
use std::slice;
use std::str;

/// A structure that describes the version of the `libgphoto2` library.
///
/// The structure describes not only the version number, but also the state of several compile-time
/// configuration options of the `libgphoto2` library.
///
/// ## Example
///
/// ```no_run
/// let version = gphoto::libgphoto2_version();
/// println!("libgphoto2 {} {} {} {} {}",
///          version.version(),
///          version.camlibs(),
///          version.compiler(),
///          version.ltdl(),
///          version.exif());
/// ```
///
/// This may output something like the following:
///
/// ```text
/// libgphoto2 2.5.7 all camlibs clang ltdl no EXIF
/// ```
#[derive(Debug)]
pub struct LibraryVersion {
    version: &'static str,
    camlibs: &'static str,
    compiler: &'static str,
    ltdl: &'static str,
    exif: &'static str,
}

impl LibraryVersion {
    fn new() -> LibraryVersion {
        let ptr = unsafe {
            ::gphoto2::gp_library_version(::gphoto2::GPVersionVerbosity::GP_VERSION_SHORT)
        };

        let mut len: usize = 0;

        while !unsafe { *ptr.offset(len as isize) }.is_null() {
            len += 1;
        }

        assert!(len >= 5);

        let table = unsafe {
            slice::from_raw_parts(ptr, len)
        };

        LibraryVersion {
            version:  unsafe { str::from_utf8_unchecked(CStr::from_ptr(table[0]).to_bytes()) },
            camlibs:  unsafe { str::from_utf8_unchecked(CStr::from_ptr(table[1]).to_bytes()) },
            compiler: unsafe { str::from_utf8_unchecked(CStr::from_ptr(table[2]).to_bytes()) },
            ltdl:     unsafe { str::from_utf8_unchecked(CStr::from_ptr(table[3]).to_bytes()) },
            exif:     unsafe { str::from_utf8_unchecked(CStr::from_ptr(table[4]).to_bytes()) },
        }
    }

    /// Returns the `libgphoto2` version number.
    ///
    /// The version number is a string representing the version number, e.g., `"2.4.16"`.
    pub fn version(&self) -> &str {
        self.version
    }

    /// Describes which camlibs were compiled with `libgphoto2`.
    pub fn camlibs(&self) -> &str {
        self.camlibs
    }

    /// Returns the name of the C compiler that was used to compile `libgphoto2`.
    pub fn compiler(&self) -> &str {
        self.compiler
    }

    /// Returns a string describing whether `libgphoto2` was compiled for portable loading of
    /// camlibs.
    pub fn ltdl(&self) -> &str {
        self.ltdl
    }

    /// Returns a string describing whether `libgphoto2` was compiled with support for handling
    /// EXIF metadata.
    pub fn exif(&self) -> &str {
        self.exif
    }
}

/// Returns a structure with the version of the `libgphoto2` C library.
pub fn libgphoto2_version() -> LibraryVersion {
    LibraryVersion::new()
}
