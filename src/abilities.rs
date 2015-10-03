use std::borrow::Cow;
use std::collections::HashSet;
use std::ffi::CStr;

use ::port::{PortType};

/// Describes the abilities of a device.
///
/// ## Example
///
/// An `Abilities` object can be used to retrieve information about a camera's driver:
///
/// ```no_run
/// let mut context = gphoto::Context::new().unwrap();
/// let mut camera = gphoto::Camera::autodetect(&mut context).unwrap();
/// let abilities = camera.abilities();
///
/// println!("      device type = {:?}", abilities.device_type());
/// println!("            model = {:?}", abilities.model());
/// println!("    driver status = {:?}", abilities.driver_status());
/// println!("       port types = {:?}", abilities.port_types());
/// println!("           speeds = {:?}", abilities.speeds());
/// println!("camera operations = {:?}", abilities.camera_operations());
/// println!("  file operations = {:?}", abilities.file_operations());
/// println!("folder operations = {:?}", abilities.folder_operations());
/// println!("       USB vendor = {:?}", abilities.usb_vendor());
/// println!("      USB product = {:?}", abilities.usb_product());
/// println!("        USB class = {:?}", abilities.usb_class());
/// println!("     USB subclass = {:?}", abilities.usb_subclass());
/// println!("     USB protocol = {:?}", abilities.usb_protocol());
/// ```
///
/// The above example may print something like the following:
///
/// ```text
///       device type = Camera
///             model = "Nikon DSC D750"
///     driver status = Production
///        port types = {USB}
///            speeds = []
/// camera operations = {CaptureImage, TriggerCapture, Config, CapturePreview}
///   file operations = {Delete, Preview}
/// folder operations = {MakeDirectory, RemoveDirectory, PutFile}
///        USB vendor = 1200
///       USB product = 1079
///         USB class = 0
///      USB subclass = 0
///      USB protocol = 0
/// ```
pub struct Abilities {
    inner: ::gphoto2::CameraAbilities
}

impl Abilities {
    /// Returns the type of the device.
    pub fn device_type(&self) -> DeviceType {
        match self.inner.device_type {
            ::gphoto2::GP_DEVICE_STILL_CAMERA => DeviceType::Camera,
            ::gphoto2::GP_DEVICE_AUDIO_PLAYER => DeviceType::Audio,
        }
    }

    /// Returns the name of the camera's model.
    pub fn model(&self) -> Cow<str> {
        unsafe {
            String::from_utf8_lossy(CStr::from_ptr(self.inner.model.as_ptr()).to_bytes())
        }
    }

    /// Returns the driver's stability status.
    pub fn driver_status(&self) -> DriverStatus {
        match self.inner.status {
            ::gphoto2::GP_DRIVER_STATUS_PRODUCTION   => DriverStatus::Production,
            ::gphoto2::GP_DRIVER_STATUS_TESTING      => DriverStatus::Testing,
            ::gphoto2::GP_DRIVER_STATUS_EXPERIMENTAL => DriverStatus::Experimental,
            ::gphoto2::GP_DRIVER_STATUS_DEPRECATED   => DriverStatus::Deprecated,
        }
    }

    /// Returns the supported port types.
    pub fn port_types(&self) -> HashSet<PortType> {
        let mut port_types = HashSet::<PortType>::new();

        if self.inner.port & ::gphoto2::GP_PORT_SERIAL != 0 {
            port_types.insert(PortType::Serial);
        }

        if self.inner.port & ::gphoto2::GP_PORT_USB != 0 {
            port_types.insert(PortType::USB);
        }

        if self.inner.port & ::gphoto2::GP_PORT_DISK != 0 {
            port_types.insert(PortType::Disk);
        }

        if self.inner.port & ::gphoto2::GP_PORT_PTPIP != 0 {
            port_types.insert(PortType::PTPIP);
        }

        if self.inner.port & ::gphoto2::GP_PORT_USB_DISK_DIRECT != 0 {
            port_types.insert(PortType::Direct);
        }

        if self.inner.port & ::gphoto2::GP_PORT_USB_SCSI != 0 {
            port_types.insert(PortType::SCSI);
        }

        port_types
    }

    /// Returns the supported serial port speeds.
    pub fn speeds(&self) -> Vec<usize> {
        self.inner.speed.iter().take_while(|&n| *n != 0).map(|&n| n as usize).collect()
    }

    /// Returns the camera operations supported by the device.
    pub fn camera_operations(&self) -> HashSet<CameraOperation> {
        let mut operations = HashSet::<CameraOperation>::new();

        if self.inner.operations & ::gphoto2::GP_OPERATION_CONFIG != 0 {
            operations.insert(CameraOperation::Config);
        }

        if self.inner.operations & ::gphoto2::GP_OPERATION_CAPTURE_IMAGE != 0 {
            operations.insert(CameraOperation::CaptureImage);
        }

        if self.inner.operations & ::gphoto2::GP_OPERATION_CAPTURE_VIDEO != 0 {
            operations.insert(CameraOperation::CaptureVideo);
        }

        if self.inner.operations & ::gphoto2::GP_OPERATION_CAPTURE_AUDIO != 0 {
            operations.insert(CameraOperation::CaptureAudio);
        }

        if self.inner.operations & ::gphoto2::GP_OPERATION_CAPTURE_PREVIEW != 0 {
            operations.insert(CameraOperation::CapturePreview);
        }

        if self.inner.operations & ::gphoto2::GP_OPERATION_TRIGGER_CAPTURE != 0 {
            operations.insert(CameraOperation::TriggerCapture);
        }

        operations
    }

    /// Returns the file operations supported by the device.
    pub fn file_operations(&self) -> HashSet<FileOperation> {
        let mut operations = HashSet::<FileOperation>::new();

        if self.inner.file_operations & ::gphoto2::GP_FILE_OPERATION_DELETE != 0 {
            operations.insert(FileOperation::Delete);
        }

        if self.inner.file_operations & ::gphoto2::GP_FILE_OPERATION_PREVIEW != 0 {
            operations.insert(FileOperation::Preview);
        }

        if self.inner.file_operations & ::gphoto2::GP_FILE_OPERATION_RAW != 0 {
            operations.insert(FileOperation::Raw);
        }

        if self.inner.file_operations & ::gphoto2::GP_FILE_OPERATION_AUDIO != 0 {
            operations.insert(FileOperation::Audio);
        }

        if self.inner.file_operations & ::gphoto2::GP_FILE_OPERATION_EXIF != 0 {
            operations.insert(FileOperation::EXIF);
        }

        operations
    }

    /// Returns the folder operations supported by the device.
    pub fn folder_operations(&self) -> HashSet<FolderOperation> {
        let mut operations = HashSet::<FolderOperation>::new();

        if self.inner.folder_operations & ::gphoto2::GP_FOLDER_OPERATION_DELETE_ALL != 0 {
            operations.insert(FolderOperation::DeleteAll);
        }

        if self.inner.folder_operations & ::gphoto2::GP_FOLDER_OPERATION_PUT_FILE != 0 {
            operations.insert(FolderOperation::PutFile);
        }

        if self.inner.folder_operations & ::gphoto2::GP_FOLDER_OPERATION_MAKE_DIR != 0 {
            operations.insert(FolderOperation::MakeDirectory);
        }

        if self.inner.folder_operations & ::gphoto2::GP_FOLDER_OPERATION_REMOVE_DIR != 0 {
            operations.insert(FolderOperation::RemoveDirectory);
        }

        operations
    }

    /// USB vendor ID.
    pub fn usb_vendor(&self) -> u16 {
        self.inner.usb_vendor as u16
    }

    /// USB product ID.
    pub fn usb_product(&self) -> u16 {
        self.inner.usb_product as u16
    }

    /// USB device class.
    pub fn usb_class(&self) -> u8 {
        self.inner.usb_class as u8
    }

    /// USB device subclass.
    pub fn usb_subclass(&self) -> u8 {
        self.inner.usb_subclass as u8
    }

    /// USB device protocol.
    pub fn usb_protocol(&self) -> u8 {
        self.inner.usb_protocol as u8
    }
}

/// Types of devices.
#[derive(Debug,PartialEq,Eq,Clone,Copy,Hash)]
pub enum DeviceType {
    /// Still camera.
    Camera,

    /// Audio player.
    Audio,
}

/// Stability of camera driver.
#[derive(Debug,PartialEq,Eq,Clone,Copy,Hash)]
pub enum DriverStatus {
    /// Driver is production ready.
    Production,

    /// Driver is beta quality.
    Testing,

    /// Driver is alpha quality and might not even work.
    Experimental,

    /// Driver is no longer recommended and will be removed.
    Deprecated,
}

/// Operations that can be performed on a device.
#[derive(Debug,PartialEq,Eq,Clone,Copy,Hash)]
pub enum CameraOperation {
    /// Camera can be configured.
    Config,

    /// Camera can capture images.
    CaptureImage,

    /// Camera can capture video.
    CaptureVideo,

    /// Camera can capture audio.
    CaptureAudio,

    /// Camera can capture image previews.
    CapturePreview,

    /// Camera can trigger capture and wait for events.
    TriggerCapture,
}


/// Operations that can be performed on files on a device's storage.
#[derive(Debug,PartialEq,Eq,Clone,Copy,Hash)]
pub enum FileOperation {
    /// Files can be deleted.
    Delete,

    /// Viewfinder content can be previewed.
    Preview,

    /// Raw file data can be retrieved.
    Raw,

    /// Audio data can be retrieved.
    Audio,

    /// EXIF data can be retrieved.
    EXIF,
}

/// Operations that can be performed on folders on a device's storage.
#[derive(Debug,PartialEq,Eq,Clone,Copy,Hash)]
pub enum FolderOperation {
    /// Deleting all files on the device is supported.
    DeleteAll,

    /// Uploading files to the device is supported.
    PutFile,

    /// Making new directories on the device is supported.
    MakeDirectory,

    /// Removing directories from the device is supported.
    RemoveDirectory,
}


#[doc(hidden)]
pub fn from_libgphoto2(abilities: ::gphoto2::CameraAbilities) -> Abilities {
    Abilities { inner: abilities }
}
