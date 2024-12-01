//! is Device

use alloc::boxed::Box;

use bitflags::bitflags;

use win_kernel_sys::base::STATUS_SUCCESS;
use win_kernel_sys::base::{DEVICE_OBJECT, IRP, NTSTATUS};
use win_kernel_sys::base::{
    IRP_MJ_CLEANUP, IRP_MJ_CLOSE, IRP_MJ_CREATE, IRP_MJ_DEVICE_CONTROL, IRP_MJ_READ, IRP_MJ_WRITE,
};
use win_kernel_sys::ntoskrnl::{IoDeleteDevice, IoGetCurrentIrpStackLocation};

use crate::error::Error;
use crate::request::{IoControlRequest, IoRequest, ReadRequest, WriteRequest};

/// windows kernel access 
#[derive(Copy, Clone, Debug)]
pub enum Access {
    NonExclusive,
    Exclusive,
}

impl Access {
    /// is exclusive
    pub fn is_exclusive(&self) -> bool {
        matches!(*self, Access::Exclusive)
    }
}

bitflags! {
    /// DeviceFlags: u32
    pub struct DeviceFlags: u32 {
        /// FILE_DEVICE_SECURE_OPEN
        const SECURE_OPEN = win_kernel_sys::base::FILE_DEVICE_SECURE_OPEN;
    }
}

bitflags! {
    pub struct DeviceDoFlags: u32 {
        /// buffered io
        const DO_BUFFERED_IO = win_kernel_sys::base::DO_BUFFERED_IO;
        /// direct io
        const DO_DIRECT_IO   = win_kernel_sys::base::DO_DIRECT_IO;
    }
}

/// enmu DeviceType 
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeviceType {
    Port8042,
    Acpi,
    Battery,
    Beep,
    BusExtender,
    Cdrom,
    CdromFileSystem,
    Changer,
    Controller,
    DataLink,
    Dfs,
    DfsFileSystem,
    DfsVolume,
    Disk,
    DiskFileSystem,
    Dvd,
    FileSystem,
    Fips,
    FullscreenVideo,
    InportPort,
    Keyboard,
    Ks,
    Ksec,
    Mailslot,
    MassStorage,
    MidiIn,
    MidiOut,
    Modem,
    Mouse,
    MultiUncProvider,
    NamedPipe,
    Network,
    NetworkBrowser,
    NetworkFileSystem,
    NetworkRedirector,
    Null,
    ParallelPort,
    PhysicalNetcard,
    Printer,
    Scanner,
    Screen,
    Serenum,
    SerialPort,
    SerialMousePort,
    Smartcard,
    Smb,
    Sound,
    Streams,
    Tape,
    TapeFileSystem,
    Termsrv,
    Transport,
    Unknown,
    Vdm,
    Video,
    VirtualDisk,
    WaveIn,
    WaveOut,
}

impl Into<u32> for DeviceType {
    ///  the device type u32 into
    fn into(self) -> u32 {
        match self {
            DeviceType::Port8042 => win_kernel_sys::base::FILE_DEVICE_8042_PORT,
            DeviceType::Acpi => win_kernel_sys::base::FILE_DEVICE_ACPI,
            DeviceType::Battery => win_kernel_sys::base::FILE_DEVICE_BATTERY,
            DeviceType::Beep => win_kernel_sys::base::FILE_DEVICE_BEEP,
            DeviceType::BusExtender => win_kernel_sys::base::FILE_DEVICE_BUS_EXTENDER,
            DeviceType::Cdrom => win_kernel_sys::base::FILE_DEVICE_CD_ROM,
            DeviceType::CdromFileSystem => win_kernel_sys::base::FILE_DEVICE_CD_ROM_FILE_SYSTEM,
            DeviceType::Changer => win_kernel_sys::base::FILE_DEVICE_CHANGER,
            DeviceType::Controller => win_kernel_sys::base::FILE_DEVICE_CONTROLLER,
            DeviceType::DataLink => win_kernel_sys::base::FILE_DEVICE_DATALINK,
            DeviceType::Dfs => win_kernel_sys::base::FILE_DEVICE_DFS,
            DeviceType::DfsFileSystem => win_kernel_sys::base::FILE_DEVICE_DFS_FILE_SYSTEM,
            DeviceType::DfsVolume => win_kernel_sys::base::FILE_DEVICE_DFS_VOLUME,
            DeviceType::Disk => win_kernel_sys::base::FILE_DEVICE_DISK,
            DeviceType::DiskFileSystem => win_kernel_sys::base::FILE_DEVICE_DISK_FILE_SYSTEM,
            DeviceType::Dvd => win_kernel_sys::base::FILE_DEVICE_DVD,
            DeviceType::FileSystem => win_kernel_sys::base::FILE_DEVICE_FILE_SYSTEM,
            DeviceType::Fips => win_kernel_sys::base::FILE_DEVICE_FIPS,
            DeviceType::FullscreenVideo => win_kernel_sys::base::FILE_DEVICE_FULLSCREEN_VIDEO,
            DeviceType::InportPort => win_kernel_sys::base::FILE_DEVICE_INPORT_PORT,
            DeviceType::Keyboard => win_kernel_sys::base::FILE_DEVICE_KEYBOARD,
            DeviceType::Ks => win_kernel_sys::base::FILE_DEVICE_KS,
            DeviceType::Ksec => win_kernel_sys::base::FILE_DEVICE_KSEC,
            DeviceType::Mailslot => win_kernel_sys::base::FILE_DEVICE_MAILSLOT,
            DeviceType::MassStorage => win_kernel_sys::base::FILE_DEVICE_MASS_STORAGE,
            DeviceType::MidiIn => win_kernel_sys::base::FILE_DEVICE_MIDI_IN,
            DeviceType::MidiOut => win_kernel_sys::base::FILE_DEVICE_MIDI_OUT,
            DeviceType::Modem => win_kernel_sys::base::FILE_DEVICE_MODEM,
            DeviceType::Mouse => win_kernel_sys::base::FILE_DEVICE_MOUSE,
            DeviceType::MultiUncProvider => {
                win_kernel_sys::base::FILE_DEVICE_MULTI_UNC_PROVIDER
            }
            DeviceType::NamedPipe => win_kernel_sys::base::FILE_DEVICE_NAMED_PIPE,
            DeviceType::Network => win_kernel_sys::base::FILE_DEVICE_NETWORK,
            DeviceType::NetworkBrowser => win_kernel_sys::base::FILE_DEVICE_NETWORK_BROWSER,
            DeviceType::NetworkFileSystem => {
                win_kernel_sys::base::FILE_DEVICE_NETWORK_FILE_SYSTEM
            }
            DeviceType::NetworkRedirector => {
                win_kernel_sys::base::FILE_DEVICE_NETWORK_REDIRECTOR
            }
            DeviceType::Null => win_kernel_sys::base::FILE_DEVICE_NULL,
            DeviceType::ParallelPort => win_kernel_sys::base::FILE_DEVICE_PARALLEL_PORT,
            DeviceType::PhysicalNetcard => win_kernel_sys::base::FILE_DEVICE_PHYSICAL_NETCARD,
            DeviceType::Printer => win_kernel_sys::base::FILE_DEVICE_PRINTER,
            DeviceType::Scanner => win_kernel_sys::base::FILE_DEVICE_SCANNER,
            DeviceType::Screen => win_kernel_sys::base::FILE_DEVICE_SCREEN,
            DeviceType::Serenum => win_kernel_sys::base::FILE_DEVICE_SERENUM,
            DeviceType::SerialMousePort => win_kernel_sys::base::FILE_DEVICE_SERIAL_MOUSE_PORT,
            DeviceType::SerialPort => win_kernel_sys::base::FILE_DEVICE_SERIAL_PORT,
            DeviceType::Smartcard => win_kernel_sys::base::FILE_DEVICE_SMARTCARD,
            DeviceType::Smb => win_kernel_sys::base::FILE_DEVICE_SMB,
            DeviceType::Sound => win_kernel_sys::base::FILE_DEVICE_SOUND,
            DeviceType::Streams => win_kernel_sys::base::FILE_DEVICE_STREAMS,
            DeviceType::Tape => win_kernel_sys::base::FILE_DEVICE_TAPE,
            DeviceType::TapeFileSystem => win_kernel_sys::base::FILE_DEVICE_TAPE_FILE_SYSTEM,
            DeviceType::Termsrv => win_kernel_sys::base::FILE_DEVICE_TERMSRV,
            DeviceType::Transport => win_kernel_sys::base::FILE_DEVICE_TRANSPORT,
            DeviceType::Unknown => win_kernel_sys::base::FILE_DEVICE_UNKNOWN,
            DeviceType::Vdm => win_kernel_sys::base::FILE_DEVICE_VDM,
            DeviceType::Video => win_kernel_sys::base::FILE_DEVICE_VIDEO,
            DeviceType::VirtualDisk => win_kernel_sys::base::FILE_DEVICE_VIRTUAL_DISK,
            DeviceType::WaveIn => win_kernel_sys::base::FILE_DEVICE_WAVE_IN,
            DeviceType::WaveOut => win_kernel_sys::base::FILE_DEVICE_WAVE_OUT,
        }
    }
}


impl From<u32> for DeviceType {
    /// Get the device type from the u32
    fn from(value: u32) -> Self {
        match value {
            win_kernel_sys::base::FILE_DEVICE_8042_PORT => DeviceType::Port8042,
            win_kernel_sys::base::FILE_DEVICE_ACPI => DeviceType::Acpi,
            win_kernel_sys::base::FILE_DEVICE_BATTERY => DeviceType::Battery,
            win_kernel_sys::base::FILE_DEVICE_BEEP => DeviceType::Beep,
            win_kernel_sys::base::FILE_DEVICE_BUS_EXTENDER => DeviceType::BusExtender,
            win_kernel_sys::base::FILE_DEVICE_CD_ROM => DeviceType::Cdrom,
            win_kernel_sys::base::FILE_DEVICE_CD_ROM_FILE_SYSTEM => DeviceType::CdromFileSystem,
            win_kernel_sys::base::FILE_DEVICE_CHANGER => DeviceType::Changer,
            win_kernel_sys::base::FILE_DEVICE_CONTROLLER => DeviceType::Controller,
            win_kernel_sys::base::FILE_DEVICE_DATALINK => DeviceType::DataLink,
            win_kernel_sys::base::FILE_DEVICE_DFS => DeviceType::Dfs,
            win_kernel_sys::base::FILE_DEVICE_DFS_FILE_SYSTEM => DeviceType::DfsFileSystem,
            win_kernel_sys::base::FILE_DEVICE_DFS_VOLUME => DeviceType::DfsVolume,
            win_kernel_sys::base::FILE_DEVICE_DISK => DeviceType::Disk,
            win_kernel_sys::base::FILE_DEVICE_DISK_FILE_SYSTEM => DeviceType::DiskFileSystem,
            win_kernel_sys::base::FILE_DEVICE_DVD => DeviceType::Dvd,
            win_kernel_sys::base::FILE_DEVICE_FILE_SYSTEM => DeviceType::FileSystem,
            win_kernel_sys::base::FILE_DEVICE_FIPS => DeviceType::Fips,
            win_kernel_sys::base::FILE_DEVICE_FULLSCREEN_VIDEO => DeviceType::FullscreenVideo,
            win_kernel_sys::base::FILE_DEVICE_INPORT_PORT => DeviceType::InportPort,
            win_kernel_sys::base::FILE_DEVICE_KEYBOARD => DeviceType::Keyboard,
            win_kernel_sys::base::FILE_DEVICE_KS => DeviceType::Ks,
            win_kernel_sys::base::FILE_DEVICE_KSEC => DeviceType::Ksec,
            win_kernel_sys::base::FILE_DEVICE_MAILSLOT => DeviceType::Mailslot,
            win_kernel_sys::base::FILE_DEVICE_MASS_STORAGE => DeviceType::MassStorage,
            win_kernel_sys::base::FILE_DEVICE_MIDI_IN => DeviceType::MidiIn,
            win_kernel_sys::base::FILE_DEVICE_MIDI_OUT => DeviceType::MidiOut,
            win_kernel_sys::base::FILE_DEVICE_MODEM => DeviceType::Modem,
            win_kernel_sys::base::FILE_DEVICE_MOUSE => DeviceType::Mouse,
            win_kernel_sys::base::FILE_DEVICE_MULTI_UNC_PROVIDER => {
                DeviceType::MultiUncProvider
            }
            win_kernel_sys::base::FILE_DEVICE_NAMED_PIPE => DeviceType::NamedPipe,
            win_kernel_sys::base::FILE_DEVICE_NETWORK => DeviceType::Network,
            win_kernel_sys::base::FILE_DEVICE_NETWORK_BROWSER => DeviceType::NetworkBrowser,
            win_kernel_sys::base::FILE_DEVICE_NETWORK_FILE_SYSTEM => {
                DeviceType::NetworkFileSystem
            }
            win_kernel_sys::base::FILE_DEVICE_NETWORK_REDIRECTOR => {
                DeviceType::NetworkRedirector
            }
            win_kernel_sys::base::FILE_DEVICE_NULL => DeviceType::Null,
            win_kernel_sys::base::FILE_DEVICE_PARALLEL_PORT => DeviceType::ParallelPort,
            win_kernel_sys::base::FILE_DEVICE_PHYSICAL_NETCARD => DeviceType::PhysicalNetcard,
            win_kernel_sys::base::FILE_DEVICE_PRINTER => DeviceType::Printer,
            win_kernel_sys::base::FILE_DEVICE_SCANNER => DeviceType::Scanner,
            win_kernel_sys::base::FILE_DEVICE_SCREEN => DeviceType::Screen,
            win_kernel_sys::base::FILE_DEVICE_SERENUM => DeviceType::Serenum,
            win_kernel_sys::base::FILE_DEVICE_SERIAL_MOUSE_PORT => DeviceType::SerialMousePort,
            win_kernel_sys::base::FILE_DEVICE_SERIAL_PORT => DeviceType::SerialPort,
            win_kernel_sys::base::FILE_DEVICE_SMARTCARD => DeviceType::Smartcard,
            win_kernel_sys::base::FILE_DEVICE_SMB => DeviceType::Smb,
            win_kernel_sys::base::FILE_DEVICE_SOUND => DeviceType::Sound,
            win_kernel_sys::base::FILE_DEVICE_STREAMS => DeviceType::Streams,
            win_kernel_sys::base::FILE_DEVICE_TAPE => DeviceType::Tape,
            win_kernel_sys::base::FILE_DEVICE_TAPE_FILE_SYSTEM => DeviceType::TapeFileSystem,
            win_kernel_sys::base::FILE_DEVICE_TERMSRV => DeviceType::Termsrv,
            win_kernel_sys::base::FILE_DEVICE_TRANSPORT => DeviceType::Transport,
            win_kernel_sys::base::FILE_DEVICE_UNKNOWN => DeviceType::Unknown,
            win_kernel_sys::base::FILE_DEVICE_VDM => DeviceType::Vdm,
            win_kernel_sys::base::FILE_DEVICE_VIDEO => DeviceType::Video,
            win_kernel_sys::base::FILE_DEVICE_VIRTUAL_DISK => DeviceType::VirtualDisk,
            win_kernel_sys::base::FILE_DEVICE_WAVE_IN => DeviceType::WaveIn,
            win_kernel_sys::base::FILE_DEVICE_WAVE_OUT => DeviceType::WaveOut,
            _ => DeviceType::Unknown,
        }
    }
}


///  device operations  dispatch or release operations
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct device_operations {
    dispatch: Option<extern "C" fn(*mut DEVICE_OBJECT, *mut IRP, u8) -> NTSTATUS>,
    release: Option<extern "C" fn(*mut DEVICE_OBJECT)>,
}


///  device tpye
pub struct Device {
    raw: *mut DEVICE_OBJECT,
}

unsafe impl Send for Device {}

unsafe impl Sync for Device {}

impl Device {
    #[inline(always)]
    pub unsafe fn from_raw(raw: *mut DEVICE_OBJECT) -> Self {
        Self { raw }
    }

    #[inline(always)]
    pub unsafe fn as_raw(&self) -> *const DEVICE_OBJECT {
        self.raw as *const _
    }

    #[inline(always)]
    pub unsafe fn as_raw_mut(&self) -> *mut DEVICE_OBJECT {
        self.raw
    }

    #[inline(always)]
    pub fn into_raw(mut self) -> *mut DEVICE_OBJECT {
        core::mem::replace(&mut self.raw, core::ptr::null_mut())
    }

    #[inline(always)]
    pub(crate) fn extension(&self) -> &DeviceExtension {
        unsafe { &*((*self.raw).DeviceExtension as *const DeviceExtension) }
    }

    #[inline(always)]
    pub(crate) fn extension_mut(&self) -> &mut DeviceExtension {
        unsafe { &mut *((*self.raw).DeviceExtension as *mut DeviceExtension) }
    }

    #[inline(always)]
    pub(crate) fn device_type(&self) -> DeviceType {
        self.extension().device_type
    }

    #[inline(always)]
    pub(crate) fn vtable(&self) -> &device_operations {
        unsafe { &*(self.extension().vtable as *const _) }
    }

    #[inline(always)]
    pub fn data<T: DeviceOperations>(&self) -> &T {
        unsafe { &*(self.extension().data as *const T) }
    }

    #[inline(always)]
    pub fn data_mut<T: DeviceOperations>(&self) -> &mut T {
        unsafe { &mut *(self.extension().data as *mut T) }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        if self.raw.is_null() {
            return;
        }

        unsafe {
            if let Some(release) = self.vtable().release {
                release(self.raw);
            }

            IoDeleteDevice(self.raw);
        }
    }
}

pub struct RequestError(pub Error, pub IoRequest);

pub enum Completion {
    Complete(u32, IoRequest),
}

/// Trait definition with default implementations
/// To be implemented by Devices attached to Driver
pub trait DeviceOperations: Sync + Sized {
    fn create(&mut self, _device: &Device, request: IoRequest) -> Result<Completion, RequestError> {
        Ok(Completion::Complete(0, request))
    }

    fn close(&mut self, _device: &Device, request: IoRequest) -> Result<Completion, RequestError> {
        Ok(Completion::Complete(0, request))
    }

    fn cleanup(
        &mut self,
        _device: &Device,
        request: IoRequest,
    ) -> Result<Completion, RequestError> {
        Ok(Completion::Complete(0, request))
    }

    fn read(&mut self, _device: &Device, request: ReadRequest) -> Result<Completion, RequestError> {
        Ok(Completion::Complete(0, request.into()))
    }

    fn write(
        &mut self,
        _device: &Device,
        request: WriteRequest,
    ) -> Result<Completion, RequestError> {
        Ok(Completion::Complete(0, request.into()))
    }

    fn ioctl(
        &mut self,
        _device: &Device,
        request: IoControlRequest,
    ) -> Result<Completion, RequestError> {
        Ok(Completion::Complete(0, request.into()))
    }
}

/// Generic dispatch callback for all IRP codes
/// kernel_module! macro assigns dispatch_device callback for all of them
#[allow(non_snake_case)]
extern "C" fn dispatch_callback<T: DeviceOperations>(
    device: *mut DEVICE_OBJECT,
    irp: *mut IRP,
    major: u8,
) -> NTSTATUS {
    let device = unsafe { Device::from_raw(device) };
    let data: &mut T = device.data_mut();
    let request = unsafe { IoRequest::from_raw(irp) };
    let result = match major as _ {
        IRP_MJ_CREATE => data.create(&device, request),
        IRP_MJ_CLOSE => data.close(&device, request),
        IRP_MJ_CLEANUP => data.cleanup(&device, request),
        IRP_MJ_READ => {
            let read_request = ReadRequest { inner: request };

            data.read(&device, read_request)
        }
        IRP_MJ_WRITE => {
            let write_request = WriteRequest { inner: request };

            data.write(&device, write_request)
        }
        IRP_MJ_DEVICE_CONTROL => {
            let control_request = IoControlRequest { inner: request };

            if device.device_type() == control_request.control_code().device_type() {
                data.ioctl(&device, control_request)
            } else {
                Err(RequestError(
                    Error::INVALID_PARAMETER,
                    control_request.into(),
                ))
            }
        }
        _ => Err(RequestError(Error::INVALID_PARAMETER, request)),
    };

    device.into_raw();

    match result {
        Ok(Completion::Complete(size, request)) => {
            request.complete(Ok(size));
            STATUS_SUCCESS
        }
        Err(RequestError(e, request)) => {
            let status = e.to_ntstatus();
            request.complete(Err(e));
            status
        }
    }
}




extern "C" fn release_callback<T: DeviceOperations>(device: *mut DEVICE_OBJECT) {
    unsafe {
        let extension = (*device).DeviceExtension as *mut DeviceExtension;
        let ptr = core::mem::replace(&mut (*extension).data, core::ptr::null_mut());
        drop(Box::from_raw(ptr as *mut T));
    }
}

pub(crate) struct DeviceOperationsVtable<T>(core::marker::PhantomData<T>);

impl<T: DeviceOperations> DeviceOperationsVtable<T> { 
    pub(crate) const VTABLE: device_operations = device_operations {
        dispatch: Some(dispatch_callback::<T>),
        release: Some(release_callback::<T>),
    };
}

/// DeviceExtension
#[repr(C)]
pub struct DeviceExtension {
    pub(crate) vtable: *const device_operations,
    pub(crate) data: *mut cty::c_void,
    pub(crate) device_type: DeviceType,
}

/// dispatch device
pub extern "C" fn dispatch_device(device: *mut DEVICE_OBJECT, irp: *mut IRP) -> NTSTATUS {
    let stack_location = unsafe { &*IoGetCurrentIrpStackLocation(irp) };
    let device = unsafe { Device::from_raw(device) };
    let vtable = device.vtable();

    match vtable.dispatch {
        Some(dispatch) => dispatch(device.into_raw(), irp, stack_location.MajorFunction),
        _ => {
            device.into_raw();
            STATUS_SUCCESS
        }
    }
}
