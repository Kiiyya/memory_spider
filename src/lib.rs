#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![allow(dead_code, unused_variables)]
use std::{marker::PhantomData, mem::ManuallyDrop, todo};

type Result<T> = std::result::Result<T, ()>;

#[cfg(target_arch = "x86_64")]
type ArchNative = u64;
#[cfg(target_arch = "x86")]
type ArchNative = u32;

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

pub trait Memory<A: Arch> {
    fn read_ptr(&self, addr: A) -> Result<A>;

    fn read_data(&self, addr: A, buf: &mut [u8]) -> Result<()>;
    fn write_data(&self, addr: A, buf: &[u8]) -> Result<()>;
}

pub trait MemAddressOf<A: Arch> {
    /// Where is our value?
    fn address_of(&self) -> Result<A>;
}

pub trait MemRead<T: Sized + Clone, A: Arch> {
    /// *(self.get_address())
    fn get(&self) -> Result<T>;

    // TODO: yeet_cache :D
}

pub trait MemWrite<T: Sized + Clone, A: Arch> {
    fn set(&self, val: &T) -> Result<()>;
}

struct ProcessHandle<A> {
    _phantom: PhantomData<A>,
}

pub trait Parent<A: Arch> : MemAddressOf<A> {
    fn memory(&self) -> Box<dyn Memory<A>>;
}

impl <A: Arch> Memory<A> for ProcessHandle<A> {
    fn read_ptr(&self, addr: A) -> Result<A> {
        unimplemented!()
    }

    fn read_data(&self, addr: A, buf: &mut [u8]) -> Result<()> {
        unimplemented!()
    }

    fn write_data(&self, addr: A, buf: &[u8]) -> Result<()> {
        unimplemented!()
    }
}

impl <T, A> MemAddressOf<A> for Ptr<T, A>
    where T: Sized + Clone, A: Arch,
{
    fn address_of(&self) -> Result<A> {
        Ok(self.parent.address_of()?.add(self.offset))
    }
}

#[repr(C)]
union Bytes<T>
    where
        T: Sized,
        [(); std::mem::size_of::<T>()]:
{
    t: std::mem::ManuallyDrop<T>,
    bytes: [u8; std::mem::size_of::<T>()]
}

impl <T, A> MemRead<T, A> for Ptr<T, A>
    where
        T: Sized + Clone,
        A: Arch,
        [(); std::mem::size_of::<T>()]:
{
    fn get(&self) -> Result<T>
    {
        let addr = self.address_of()?;
        let mut buf = Bytes::<T> { bytes: [0u8; std::mem::size_of::<T>()] };
        self.parent.memory().read_data(addr, unsafe { &mut buf.bytes } )?;
        Ok(ManuallyDrop::into_inner(unsafe { buf.t } ))
    }
}

pub struct Ptr<T, A> {
    // parent: Parent<A>,
    parent: Box<dyn Parent<A>>,
    offset: A,
    // cached: Option<T>,
    _phantom: PhantomData<T>,
}

impl <A: Arch> MemAddressOf<A> for ProcessHandle<A> {
    fn address_of(&self) -> Result<A> {
        Ok(A::null())
    }
}

impl <A: Arch> Parent<A> for ProcessHandle<A> {
    fn memory(&self) -> Box<dyn Memory<A>> {
        todo!()
        // Box::new(*self)
    }
}

// impl <T, A> Ptr<T, A> {
//     fn new_addr(handle: ProcessHandle<A>, addr: A) -> Ptr<T, A> {
//         Ptr {
//             parent: 
//             offset: addr,
//             _phantom: PhantomData,
//         }
//     }
// }

struct Array<T> {
    _phantom: PhantomData<T>,
}

struct Game {
    player_list: Array<u32>,
}

// impl <T> Ptr<Array<T>> {
//     pub fn index_array(&self, i: usize) -> Result<T> {

//     }
// }



// impl <A> ProcessHandle<A> {
//     pub fn offset<T>(&self, addr: A) -> Ptr<T, A> {
//         Ptr {

//         }
//     }
// }

fn user() {
    let handle = ProcessHandle::<ArchNative> { _phantom: PhantomData, };


}
