//! mod mdl


use crate::error::Error;
use crate::memory::MemoryCaching;


/// AccessMode
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccessMode {
    KernelMode = win_kernel_sys::base::_MODE::KernelMode,
    UserMode = win_kernel_sys::base::_MODE::UserMode,
}


/// MemoryDescriptorList
pub struct MemoryDescriptorList {
    raw: *mut win_kernel_sys::base::MDL,
}

unsafe impl Send for MemoryDescriptorList {}

unsafe impl Sync for MemoryDescriptorList {}

impl MemoryDescriptorList {
    /// new MemoryDescriptorList
    pub fn new(addr: *mut core::ffi::c_void, size: usize) -> Result<Self, Error> {
        use win_kernel_sys::ntoskrnl::IoAllocateMdl;

        let raw = unsafe {
            IoAllocateMdl(
                addr,
                size as _,
                false as _,
                false as _,
                core::ptr::null_mut(),
            )
        };

        if raw.is_null() {
            return Err(Error::INSUFFICIENT_RESOURCES);
        }

        Ok(Self { raw })
    }
    
    /// build_for_non_paged_pool
    pub fn build_for_non_paged_pool(&mut self) {
        use win_kernel_sys::ntoskrnl::MmBuildMdlForNonPagedPool;

        unsafe {
            MmBuildMdlForNonPagedPool(self.raw);
        }
    }

    /// map_locked_pages
    pub fn map_locked_pages(
        self,
        access: AccessMode,
        caching: MemoryCaching,
        desired_addr: Option<*mut core::ffi::c_void>,
    ) -> Result<LockedMapping, Error> {
        use win_kernel_sys::ntoskrnl::MmMapLockedPagesSpecifyCache;

        let ptr = unsafe {
            MmMapLockedPagesSpecifyCache(
                self.raw,
                access as _,
                caching as _,
                desired_addr.unwrap_or(core::ptr::null_mut()),
                false as _,
                0,
            )
        };

        Ok(LockedMapping { raw: self.raw, ptr })
    }
}

impl Drop for MemoryDescriptorList {
    fn drop(&mut self) {
        use win_kernel_sys::ntoskrnl::IoFreeMdl;

        unsafe {
            IoFreeMdl(self.raw);
        }
    }
}

/// LockedMapping
pub struct LockedMapping {
    raw: *mut win_kernel_sys::base::MDL,
    ptr: *mut core::ffi::c_void,
}

unsafe impl Send for LockedMapping {}

unsafe impl Sync for LockedMapping {}

impl LockedMapping {
    /// out slef.ptr
    pub fn ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }

    /// unlock
    pub fn unlock(self) -> MemoryDescriptorList {
        use win_kernel_sys::ntoskrnl::MmUnmapLockedPages;

        unsafe {
            MmUnmapLockedPages(self.ptr, self.raw);
        }

        MemoryDescriptorList { raw: self.raw }
    }
}

impl Drop for LockedMapping {
    fn drop(&mut self) {
        use win_kernel_sys::ntoskrnl::{IoFreeMdl, MmUnmapLockedPages};

        unsafe {
            MmUnmapLockedPages(self.ptr, self.raw);
            IoFreeMdl(self.raw);
        }
    }
}
