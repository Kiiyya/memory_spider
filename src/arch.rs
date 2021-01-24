#[cfg(target_arch = "x86_64")]
pub type ArchNative = u64;
#[cfg(target_arch = "x86")]
pub type ArchNative = u32;

pub trait Arch : Clone + Copy {
    type Relative;

    fn null() -> Self;
    /// Pointer addition, basically
    fn add(&self, other: Self) -> Self;
}

impl Arch for u32 {
    type Relative = i32;
    #[inline(always)]
    fn null() -> Self { 0 }
    fn add(&self, other: Self) -> Self {
        Self::wrapping_add(*self, other)
    }
}

impl Arch for u64 {
    type Relative = i64;
    #[inline(always)]
    fn null() -> Self { 0 }
    fn add(&self, other: Self) -> Self {
        Self::wrapping_add(*self, other)
    }
}

#[repr(C)]
pub(crate) union Bytes<T>
    where
        T: Sized,
        [(); std::mem::size_of::<T>()]:
{
    pub(crate) t: std::mem::ManuallyDrop<T>,
    pub(crate) bytes: [u8; std::mem::size_of::<T>()]
}
