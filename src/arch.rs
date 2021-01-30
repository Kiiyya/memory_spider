use std::ops::{Add, AddAssign};

#[cfg(target_arch = "x86_64")]
pub type ArchNative = A64Le;
// #[cfg(target_arch = "x86")]
// pub type ArchNative = u32;

pub enum Endian {
    Little,
    Big,
}

pub trait Arch: Clone + Copy + 'static {
    /// How many bytes one pointer is.
    const WIDTH: usize;
    const ENDIAN: Endian;
    /// usually u64 or u32 (not usize)
    type Pointer: Sized + Copy + Add<Output = Self::Pointer>;
    /// usually i64 or i32
    type PointerRelative: Sized + Copy + Add;

    fn ptr_null() -> Self::Pointer;
    /// Pointer addition, basically
    // fn ptr_add(lhs: &Self::Pointer, rhs: &Self::Pointer) -> Self::Pointer;

    fn ptr_to_bytes(me: Self::Pointer, buf: &mut [u8; std::mem::size_of::<Self::Pointer>()]);
    fn ptr_from_bytes(buf: &[u8; std::mem::size_of::<Self::Pointer>()]) -> Self::Pointer;

    /// Only really useful for addressing your own process. Otherwise may leave it as unimplemented or panic.
    fn addr_as_ptr<T: Sized + Copy>(addr: &Self::Pointer) -> *const T;
}

#[derive(Debug, Clone, Copy)]
pub struct A64Le;
#[derive(Debug, Clone, Copy)]
pub struct Arch32Little(u32);
// #[derive(Debug, Clone, Copy)]
// pub struct Arch64Big(u64);

impl Arch for A64Le {
    type Pointer = u64;
    type PointerRelative = i64;
    const WIDTH: usize = 8;
    const ENDIAN: Endian = Endian::Little;

    #[inline(always)]
    fn ptr_null() -> Self::Pointer {
        0
    }

    // #[inline(always)]
    // fn ptr_add(lhs: &Self::Pointer, rhs: &Self::Pointer) -> Self::Pointer {
    //     Self::Pointer::wrapping_add(*lhs, *rhs)
    // }

    #[inline(always)]
    fn ptr_to_bytes(me: Self::Pointer, buf: &mut [u8; std::mem::size_of::<Self::Pointer>()]) {
        todo!()
    }

    #[inline(always)]
    fn ptr_from_bytes(buf: &[u8; std::mem::size_of::<Self::Pointer>()]) -> Self::Pointer {
        todo!()
    }

    #[inline(always)]
    fn addr_as_ptr<T: Sized + Copy>(addr: &Self::Pointer) -> *const T {
        #[cfg(target_arch = "x86_64")]
        return (*addr) as *const T;

        #[cfg(not(target_arch = "x86_64"))]
        panic!()
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
