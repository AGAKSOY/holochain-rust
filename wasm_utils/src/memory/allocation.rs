use holochain_core_types::{error::HolochainError, json::JsonString};
use memory::{MemoryBits, MemoryInt, MEMORY_INT_MAX};

#[derive(Copy, Clone, Debug)]
pub struct Offset(MemoryInt);
#[derive(Copy, Clone, Debug)]
pub struct Length(MemoryInt);

impl From<Offset> for MemoryInt {
    fn from(offset: Offset) -> Self {
        offset.0
    }
}

impl From<Offset> for MemoryBits {
    fn from(offset: Offset) -> Self {
        offset.0 as MemoryBits
    }
}

impl From<MemoryInt> for Offset {
    fn from(i: MemoryInt) -> Self {
        Offset(i)
    }
}

impl From<Length> for MemoryInt {
    fn from(length: Length) -> Self {
        length.0
    }
}

impl From<Length> for MemoryBits {
    fn from(length: Length) -> Self {
        length.0 as MemoryBits
    }
}

impl From<MemoryInt> for Length {
    fn from(i: MemoryInt) -> Self {
        Length(i)
    }
}

impl From<Length> for usize {
    fn from(length: Length) -> Self {
        length.0 as usize
    }
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum AllocationError {
    /// (de)allocation is either too large or implies negative values
    OutOfBounds,
    /// cannot allocate zero data
    ZeroLength,
    /// (de)allocation must occur at the top of the stack
    BadStackAlignment,
    /// writes can fail to serialize data before allocation occurs e.g. json
    Serialization,
}

impl From<AllocationError> for String {
    fn from(allocation_error: AllocationError) -> Self {
        match allocation_error {
            AllocationError::OutOfBounds => "Allocation out of bounds".into(),
            AllocationError::ZeroLength => "Allocation is zero length".into(),
            AllocationError::BadStackAlignment => "Allocation is not aligned with stack".into(),
            AllocationError::Serialization => "Allocation could not serialize data".into(),
        }
    }
}

impl From<AllocationError> for HolochainError {
    fn from(allocation_error: AllocationError) -> Self {
        HolochainError::ErrorGeneric(String::from(allocation_error))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct WasmAllocation {
    offset: Offset,
    length: Length,
}

impl WasmAllocation {
    // represent the max as MemoryBits type to allow gt comparisons
    pub fn max() -> MemoryBits {
        MEMORY_INT_MAX
    }

    pub fn new(offset: Offset, length: Length) -> Result<Self, AllocationError> {
        if (MemoryBits::from(offset) + MemoryBits::from(length)) > WasmAllocation::max() {
            Err(AllocationError::OutOfBounds)
        } else if MemoryInt::from(length) == 0 {
            Err(AllocationError::ZeroLength)
        } else {
            Ok(WasmAllocation { offset, length })
        }
    }

    pub fn offset(self) -> Offset {
        self.offset
    }

    pub fn length(self) -> Length {
        self.length
    }
}

pub type AllocationResult = Result<WasmAllocation, AllocationError>;
