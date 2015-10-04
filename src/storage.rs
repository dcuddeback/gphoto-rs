use std::borrow::Cow;
use std::ffi::CStr;

/// Structure containing information about a camera's storage.
///
/// ## Example
///
/// A `Storage` object can be used to retrieve information about a camera's storage:
///
/// ```no_run
/// let mut context = gphoto::Context::new().unwrap();
/// let mut camera = gphoto::Camera::autodetect(&mut context).unwrap();
///
/// for storage in camera.storage(&mut context).unwrap() {
///     println!("       base dir = {:?}", storage.base_dir());
///     println!("          label = {:?}", storage.label());
///     println!("    description = {:?}", storage.description());
///     println!("   storage type = {:?}", storage.storage_type());
///     println!("filesystem type = {:?}", storage.filesystem_type());
///     println!("    access type = {:?}", storage.access_type());
///     println!("    capacity kb = {:?}", storage.capacity_kbytes());
///     println!("        free kb = {:?}", storage.free_kbytes());
///     println!("    free images = {:?}", storage.free_images());
/// }
/// ```
///
/// The above example might print something like the following:
///
/// ```text
///        base dir = Some("/store_00010001")
///           label = Some("NIKON D750  [Slot 1]")
///     description = None
///    storage type = Some(RemoveableRam)
/// filesystem type = Some(DCF)
///     access type = Some(ReadDelete)
///     capacity kb = Some(31154688)
///         free kb = Some(30833088)
///     free images = Some(580)
/// ```
#[repr(C)]
pub struct Storage {
    inner: ::gphoto2::CameraStorageInformation,
}

impl Storage {
    /// Base directory of the storage.
    pub fn base_dir(&self) -> Option<Cow<str>> {
        if self.inner.fields & ::gphoto2::GP_STORAGEINFO_BASE != 0 {
            Some(unsafe {
                String::from_utf8_lossy(CStr::from_ptr(self.inner.basedir.as_ptr()).to_bytes())
            })
        }
        else {
            None
        }
    }

    /// The storage's label.
    pub fn label(&self) -> Option<Cow<str>> {
        if self.inner.fields & ::gphoto2::GP_STORAGEINFO_LABEL != 0 {
            Some(unsafe {
                String::from_utf8_lossy(CStr::from_ptr(self.inner.label.as_ptr()).to_bytes())
            })
        }
        else {
            None
        }
    }

    /// A description of the storage.
    pub fn description(&self) -> Option<Cow<str>> {
        if self.inner.fields & ::gphoto2::GP_STORAGEINFO_DESCRIPTION != 0 {
            Some(unsafe {
                String::from_utf8_lossy(CStr::from_ptr(self.inner.description.as_ptr()).to_bytes())
            })
        }
        else {
            None
        }
    }

    /// The storage's hardware type.
    pub fn storage_type(&self) -> Option<StorageType> {
        if self.inner.fields & ::gphoto2::GP_STORAGEINFO_STORAGETYPE != 0 {
            Some(match self.inner.storage_type {
                ::gphoto2::GP_STORAGEINFO_ST_FIXED_ROM     => StorageType::FixedRom,
                ::gphoto2::GP_STORAGEINFO_ST_REMOVABLE_ROM => StorageType::RemovableRom,
                ::gphoto2::GP_STORAGEINFO_ST_FIXED_RAM     => StorageType::FixedRam,
                ::gphoto2::GP_STORAGEINFO_ST_REMOVABLE_RAM => StorageType::RemoveableRam,
                ::gphoto2::GP_STORAGEINFO_ST_UNKNOWN       => StorageType::Unknown,
            })
        }
        else {
            None
        }
    }

    /// The hiearchy type of the storage's filesystem.
    pub fn filesystem_type(&self) -> Option<FilesystemType> {
        if self.inner.fields & ::gphoto2::GP_STORAGEINFO_FILESYSTEMTYPE != 0 {
            Some(match self.inner.fstype {
                ::gphoto2::GP_STORAGEINFO_FST_GENERICFLAT         => FilesystemType::Flat,
                ::gphoto2::GP_STORAGEINFO_FST_GENERICHIERARCHICAL => FilesystemType::Hierarchical,
                ::gphoto2::GP_STORAGEINFO_FST_DCF                 => FilesystemType::DCF,
                ::gphoto2::GP_STORAGEINFO_FST_UNDEFINED           => FilesystemType::Unknown,
            })
        }
        else {
            None
        }
    }

    /// The storage's access permissions.
    pub fn access_type(&self) -> Option<AccessType> {
        if self.inner.fields & ::gphoto2::GP_STORAGEINFO_ACCESS != 0 {
            Some(match self.inner.access {
                ::gphoto2::GP_STORAGEINFO_AC_READWRITE            => AccessType::ReadWrite,
                ::gphoto2::GP_STORAGEINFO_AC_READONLY             => AccessType::ReadOnly,
                ::gphoto2::GP_STORAGEINFO_AC_READONLY_WITH_DELETE => AccessType::ReadDelete,
            })
        }
        else {
            None
        }
    }

    /// The storage's total capacity in kilobytes.
    pub fn capacity_kbytes(&self) -> Option<u64> {
        if self.inner.fields & ::gphoto2::GP_STORAGEINFO_MAXCAPACITY != 0 {
            Some(self.inner.capacitykbytes)
        }
        else {
            None
        }
    }

    /// The storage's free space in kilobytes.
    pub fn free_kbytes(&self) -> Option<u64> {
        if self.inner.fields & ::gphoto2::GP_STORAGEINFO_FREESPACEKBYTES != 0 {
            Some(self.inner.freekbytes)
        }
        else {
            None
        }
    }

    /// An estimate of the number of images that could fit in the storage's remaining space.
    ///
    /// This value is estimated by the camera.
    pub fn free_images(&self) -> Option<u64> {
        if self.inner.fields & ::gphoto2::GP_STORAGEINFO_FREESPACEIMAGES != 0 {
            Some(self.inner.freeimages)
        }
        else {
            None
        }
    }
}

/// Types of storage hardware.
#[derive(Debug)]
pub enum StorageType {
    /// A fixed ROM storage.
    FixedRom,

    /// A removable ROM storage.
    RemovableRom,

    /// A fixed RAM storage.
    FixedRam,

    /// A removable RAM storage.
    ///
    /// This includes any kind of removable cards (SD card, CompactFlash, etc).
    RemoveableRam,

    /// Unknown storage type.
    Unknown,
}

/// Types of filesystem hierarchies.
#[derive(Debug)]
pub enum FilesystemType {
    /// All files stored in one directory.
    Flat,

    /// Files are stored in a generic tree-like hierarchy.
    Hierarchical,

    /// Files are stored in a DCF-compatible hierarchy.
    ///
    /// Design rule for Camera File system (DCF) is a standard that defines a directory structure
    /// (among other things) for use on digital cameras. A filesystem that follows the DCF standard
    /// will store its media in a `DCIM` directory.
    DCF,

    /// Filesystem hierarchy is unknown.
    Unknown,
}

/// Types of access permissions.
#[derive(Debug)]
pub enum AccessType {
    /// Read and write operations are allowed.
    ReadWrite,

    /// Read and delete operations are allowed.
    ReadDelete,

    /// Only read operations are allowed.
    ReadOnly,
}
