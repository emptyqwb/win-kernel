//! windows -> self error
use win_kernel_sys::base::NTSTATUS;
use win_kernel_sys::base::{
    STATUS_ACCESS_VIOLATION, STATUS_ARRAY_BOUNDS_EXCEEDED, STATUS_BREAKPOINT,
    STATUS_DATATYPE_MISALIGNMENT, STATUS_END_OF_FILE, STATUS_FLOAT_DENORMAL_OPERAND,
    STATUS_FLOAT_DIVIDE_BY_ZERO, STATUS_FLOAT_INEXACT_RESULT, STATUS_FLOAT_INVALID_OPERATION,
    STATUS_FLOAT_OVERFLOW, STATUS_FLOAT_STACK_CHECK, STATUS_FLOAT_UNDERFLOW,
    STATUS_GUARD_PAGE_VIOLATION, STATUS_ILLEGAL_INSTRUCTION, STATUS_INSUFFICIENT_RESOURCES,
    STATUS_INTEGER_DIVIDE_BY_ZERO, STATUS_INTEGER_OVERFLOW, STATUS_INVALID_DISPOSITION,
    STATUS_INVALID_HANDLE, STATUS_INVALID_PARAMETER, STATUS_INVALID_USER_BUFFER,
    STATUS_IN_PAGE_ERROR, STATUS_NONCONTINUABLE_EXCEPTION, STATUS_NOT_IMPLEMENTED,
    STATUS_NO_MEMORY, STATUS_PRIVILEGED_INSTRUCTION, STATUS_SINGLE_STEP, STATUS_STACK_OVERFLOW,
    STATUS_SUCCESS, STATUS_UNSUCCESSFUL, STATUS_UNWIND_CONSOLIDATE,
};


/// Error codes from the Windows NTSTATUS system.
#[derive(Clone, Copy, Debug)]
pub struct Error(NTSTATUS);

impl Error {
    /// GUARD_PAGE_VIOLATION
    pub const GUARD_PAGE_VIOLATION: Error = Error(STATUS_GUARD_PAGE_VIOLATION);
    /// DATATYPE_MISALIGNMENT
    pub const DATATYPE_MISALIGNMENT: Error = Error(STATUS_DATATYPE_MISALIGNMENT);
    /// BREAKPOINT
    pub const BREAKPOINT: Error = Error(STATUS_BREAKPOINT);
    /// SINGLE_STEP
    pub const SINGLE_STEP: Error = Error(STATUS_SINGLE_STEP);
    /// UNWIND_CONSOLIDATE
    pub const UNWIND_CONSOLIDATE: Error = Error(STATUS_UNWIND_CONSOLIDATE);
    /// UNSUCCESSFUL
    pub const UNSUCCESSFUL: Error = Error(STATUS_UNSUCCESSFUL);
    /// NOT_IMPLEMENTED
    pub const NOT_IMPLEMENTED: Error = Error(STATUS_NOT_IMPLEMENTED);
    /// ACCESS_VIOLATION
    pub const ACCESS_VIOLATION: Error = Error(STATUS_ACCESS_VIOLATION);
    /// IN_PAGE_ERROR
    pub const IN_PAGE_ERROR: Error = Error(STATUS_IN_PAGE_ERROR);
    /// INVALID_HANDLE
    pub const INVALID_HANDLE: Error = Error(STATUS_INVALID_HANDLE);
    /// INVALID_PARAMETER
    pub const INVALID_PARAMETER: Error = Error(STATUS_INVALID_PARAMETER);
    /// END_OF_FILE
    pub const END_OF_FILE: Error = Error(STATUS_END_OF_FILE);
    /// NO_MEMORY
    pub const NO_MEMORY: Error = Error(STATUS_NO_MEMORY);
    /// ILLEGAL_INSTRUCTION
    pub const ILLEGAL_INSTRUCTION: Error = Error(STATUS_ILLEGAL_INSTRUCTION);
    /// NONCONTINUABLE_EXCEPTION
    pub const NONCONTINUABLE_EXCEPTION: Error = Error(STATUS_NONCONTINUABLE_EXCEPTION);
    /// INVALID_DISPOSITION
    pub const INVALID_DISPOSITION: Error = Error(STATUS_INVALID_DISPOSITION);
    /// ARRAY_BOUNDS_EXCEEDED
    pub const ARRAY_BOUNDS_EXCEEDED: Error = Error(STATUS_ARRAY_BOUNDS_EXCEEDED);
    /// FLOAT_DENORMAL_OPERAND
    pub const FLOAT_DENORMAL_OPERAND: Error = Error(STATUS_FLOAT_DENORMAL_OPERAND);
    /// FLOAT_DIVIDE_BY_ZERO
    pub const FLOAT_DIVIDE_BY_ZERO: Error = Error(STATUS_FLOAT_DIVIDE_BY_ZERO);
    /// FLOAT_INEXACT_RESULT
    pub const FLOAT_INEXACT_RESULT: Error = Error(STATUS_FLOAT_INEXACT_RESULT);
    /// FLOAT_INVALID_OPERATION
    pub const FLOAT_INVALID_OPERATION: Error = Error(STATUS_FLOAT_INVALID_OPERATION);
    /// FLOAT_OVERFLOW
    pub const FLOAT_OVERFLOW: Error = Error(STATUS_FLOAT_OVERFLOW);
    /// FLOAT_STACK_CHECK
    pub const FLOAT_STACK_CHECK: Error = Error(STATUS_FLOAT_STACK_CHECK);
    /// FLOAT_UNDERFLOW
    pub const FLOAT_UNDERFLOW: Error = Error(STATUS_FLOAT_UNDERFLOW);
    /// INTEGER_DIVIDE_BY_ZERO
    pub const INTEGER_DIVIDE_BY_ZERO: Error = Error(STATUS_INTEGER_DIVIDE_BY_ZERO);
    /// INTEGER_OVERFLOW
    pub const INTEGER_OVERFLOW: Error = Error(STATUS_INTEGER_OVERFLOW);
    /// PRIVILEGED_INSTRUCTION
    pub const PRIVILEGED_INSTRUCTION: Error = Error(STATUS_PRIVILEGED_INSTRUCTION);
    /// INSUFFICIENT_RESOURCES
    pub const INSUFFICIENT_RESOURCES: Error = Error(STATUS_INSUFFICIENT_RESOURCES);
    /// INVALID_USER_BUFFER
    pub const INVALID_USER_BUFFER: Error = Error(STATUS_INVALID_USER_BUFFER);
    /// STACK_OVERFLOW
    pub const STACK_OVERFLOW: Error = Error(STATUS_STACK_OVERFLOW);

    /// self [Error] from [NTSTATUS]
    pub fn from_ntstatus(status: NTSTATUS) -> Error {
        Error(status)
    }

    /// [NTSTATUS] to self [Error] 
    pub fn to_ntstatus(&self) -> NTSTATUS {
        self.0
    }
}


///# trait
/// self error into [Result<(), Error>] error is self err
pub trait IntoResult {
    /// [IntoResult] defualt
    fn into_result(self) -> Result<(), Error>;
}

impl IntoResult for NTSTATUS {
    /// impl [IntoResult] for [NTSTATUS]
    fn into_result(self) -> Result<(), Error> {
        match self {
            STATUS_SUCCESS => Ok(()),
            status => Err(Error::from_ntstatus(status)),
        }
    }
}
