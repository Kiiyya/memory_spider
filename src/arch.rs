#[cfg(target_arch = "x86_64")]
pub type ArchNative = u64;
#[cfg(target_arch = "x86")]
pub type ArchNative = u32;

enum Endian {
    Little,
    Big,
}

pub trait Arch : Clone + Copy {
    /// How many bytes one pointer is.
    const Width: usize;
    const Endian: Endian;
    /// usually u64 or u32 (not usize)
    type Pointer;
    /// usually i64 or i32
    type PointerRelative;

    fn ptr_null() -> Self::Pointer;
    /// Pointer addition, basically
    fn ptr_add(lhs: Self::Pointer, rhs: Self::Pointer) -> Self::Pointer;

    fn ptr_to_bytes(me: Self::Pointer, buf: &mut [u8; std::mem::size_of::<Self::Pointer>()]);
    fn ptr_from_bytes(buf: &[u8; std::mem::size_of::<Self::Pointer>()]) -> Self::Pointer;
}

#[derive(Debug, Clone, Copy)]
pub struct Arch64Little(u64);
#[derive(Debug, Clone, Copy)]
pub struct Arch32Little(u32);
// #[derive(Debug, Clone, Copy)]
// pub struct Arch64Big(u64);

impl Arch for Arch64Little {
    type Pointer = u64;
    type PointerRelative = i64;
    const Width: usize = 8;
    const Endian: Endian = Endian::Little;

    fn ptr_null() -> Self::Pointer {
        0
    }

}

// impl Arch for u32 {
//     type Relative = i32;
//     #[inline(always)]
//     fn null() -> Self { 0 }
//     fn add(&self, other: Self) -> Self {
//         Self::wrapping_add(*self, other)
//     }
// }

// impl Arch for Arch64Little {
//     type Relative = i64;
//     #[inline(always)]
//     fn null() -> Self { 0 }
//     fn add(&self, other: Self) -> Self {
//         Self::wrapping_add(*self, other)
//     }
// }

