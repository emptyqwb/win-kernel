//! DRIVER_OBJECT  wrapper a driver object
use alloc::boxed::Box;

use widestring::U16CString;

use win_kernel_sys::base::DRIVER_OBJECT;
use win_kernel_sys::ntoskrnl::IoCreateDevice;

use crate::device::{
    Access, Device, DeviceDoFlags, DeviceExtension, DeviceFlags, DeviceOperations,
    DeviceOperationsVtable, DeviceType,
};
use crate::error::{Error, IntoResult};
use crate::string::create_unicode_string;


/// is DRIVER_OBJECT
pub struct Driver {
    pub(crate) raw: *mut DRIVER_OBJECT,
}

impl Driver {
    /// [Driver] from the [*mut DRIVER_OBJECT]
    pub unsafe fn from_raw(raw: *mut DRIVER_OBJECT) -> Self {
        Self { raw }
    }

    /// [Driver] as [*mut DRIVER_OBJECT]
    pub unsafe fn as_raw(&self) -> *const DRIVER_OBJECT {
        self.raw as _
    }

    /// [Driver] as mutable [*mut DRIVER_OBJECT]
    pub unsafe fn as_raw_mut(&mut self) -> *mut DRIVER_OBJECT {
        self.raw as _
    }

    /// Creates a new device with the given name, type, flags, do_flages access and data.
    pub fn create_device<T>(
        &mut self,
        name: &str,
        device_type: DeviceType,
        device_flags: DeviceFlags,
        device_do_flags: DeviceDoFlags,
        access: Access,
        data: T,
    ) -> Result<Device, Error>
    where
        T: DeviceOperations,
    {
        // Box the data.
        let data = Box::new(data);

        // Convert the name to UTF-16 and then create a UNICODE_STRING.
        let name = U16CString::from_str(name).unwrap();
        let mut name = create_unicode_string(name.as_slice());

        // Create the device.
        let mut device = core::ptr::null_mut();

        unsafe {
            IoCreateDevice(
                self.raw,
                core::mem::size_of::<DeviceExtension>() as u32,
                &mut name,
                device_type.into(),
                device_flags.bits(),
                access.is_exclusive() as _,
                &mut device,
            )
        }
        .into_result()?;

        unsafe {
            (*device).Flags |= device_do_flags.bits();
        }

        let device = unsafe { Device::from_raw(device) };

        // Store the boxed data and vtable.
        let extension = device.extension_mut();
        extension.device_type = device_type;
        extension.vtable = &DeviceOperationsVtable::<T>::VTABLE;
        extension.data = Box::into_raw(data) as *mut cty::c_void;

        Ok(device)
    }
}
