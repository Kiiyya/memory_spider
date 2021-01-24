#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![allow(dead_code, unused_variables)]
use std::{marker::PhantomData, mem::ManuallyDrop, todo};

type Result<T> = std::result::Result<T, ()>;

pub mod arch;
use arch::{Arch, ArchNative, Bytes};

pub trait Memory<A: Arch> {
    fn read_ptr(&self, addr: A) -> Result<A>;

    fn read_data(&self, addr: A, buf: &mut [u8]) -> Result<()>;
    fn write_data(&self, addr: A, buf: &[u8]) -> Result<()>;
}

pub trait MemGetAddress<A: Arch> {
    /// Where is our value?
    fn get_address(&self) -> Result<A>;
}

pub trait MemRead<T: Sized + Clone, A: Arch> : MemGetAddress<A> {
    /// *(self.get_address())
    fn get(&self) -> Result<T>;

    // TODO: yeet_cache :D
}

pub trait MemWrite<T: Sized + Clone, A: Arch> : MemGetAddress<A> {
    fn set(&self, val: &T) -> Result<()>;
}

pub trait Parent<A: Arch> : MemGetAddress<A> {
    fn memory(&self) -> Box<dyn Memory<A>>;
}

pub struct Ptr<T, A> {
    // parent: Parent<A>,
    parent: Box<dyn Parent<A>>,
    offset: A,
    // cached: Option<T>,
    _phantom: PhantomData<T>,
}

impl <T, A> MemGetAddress<A> for Ptr<T, A>
    where T: Sized + Clone, A: Arch,
{
    fn get_address(&self) -> Result<A> {
        Ok(self.parent.get_address()?.add(self.offset))
    }
}

impl <T, A> MemRead<T, A> for Ptr<T, A>
    where
        T: Sized + Clone,
        A: Arch,
        [(); std::mem::size_of::<T>()]:
{
    fn get(&self) -> Result<T>
    {
        let addr = self.get_address()?;
        let mut buf = Bytes::<T> { bytes: [0u8; std::mem::size_of::<T>()] };
        self.parent.memory().read_data(addr, unsafe { &mut buf.bytes } )?;
        Ok(ManuallyDrop::into_inner(unsafe { buf.t } ))
    }
}

impl <A: Arch> MemGetAddress<A> for ProcessHandle<A> {
    fn get_address(&self) -> Result<A> {
        Ok(A::null())
    }
}

impl <A: Arch> Parent<A> for ProcessHandle<A> {
    fn memory(&self) -> Box<dyn Memory<A>> {
        todo!()
        // Box::new(*self)
    }
}


struct ProcessHandle<A> {
    _phantom: PhantomData<A>,
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


struct Array<T> {
    _phantom: PhantomData<T>,
}

struct Game {
    player_list: Array<u32>,
}

fn user() {
    let handle = ProcessHandle::<ArchNative> { _phantom: PhantomData, };


}
