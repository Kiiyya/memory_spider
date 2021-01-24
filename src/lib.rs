#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(associated_type_bounds)]
#![feature(impl_trait_in_bindings)]
#![allow(dead_code, unused_variables)]
use std::{marker::PhantomData};

type Result<T> = std::result::Result<T, ()>;

pub mod arch;
use arch::{Arch, ArchNative};

/// More low-level thing.
pub trait Memory<A: Arch> {
    fn read_ptr(&self, addr: A) -> Result<A>;

    fn read_data(&self, addr: A, buf: &mut [u8]) -> Result<()>;
    fn write_data(&mut self, addr: A, buf: &[u8]) -> Result<()>;
}

pub trait MemGetAddress<A: Arch> {
    /// Where is our value?
    fn get_address(&self) -> Result<A>;
}

pub trait MemRead<T: Sized + Copy, A: Arch> : MemGetAddress<A> {
    /// *(self.get_address())
    fn get(&self) -> Result<T>;

    // TODO: yeet_cache :D
}

pub trait MemWrite<T: Sized + Copy, A: Arch> : MemGetAddress<A> {
    fn set(&self, val: &T) -> Result<()>;
}

pub trait Parent<A: Arch> : MemGetAddress<A> + Copy {
    type Memory;
    fn memory(&self) -> &Self::Memory;
    fn memory_mut(&mut self) -> &mut Self::Memory;
}

//////////// Pointer ///////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Ptr<'p, T, A, P>
    where
        T: Sized + Copy,
        A: Arch,
        P: Parent<A, Memory: Memory<A>>
{
    parent: &'p P,
    offset: A,
    _phantom: PhantomData<T>,
}

impl <'p, T, A, P> MemGetAddress<A> for Ptr<'p, T, A, P>
    where
        T: Sized + Copy,
        A: Arch,
        P: Parent<A, Memory: Memory<A>>
{
    fn get_address(&self) -> Result<A> {
        Ok(self.parent.get_address()?.add(self.offset))
    }
}

impl <'p, T, A, P> MemRead<T, A> for Ptr<'p, T, A, P>
    where
        T: Sized + Copy,
        A: Arch,
        P: Parent<A, Memory: Memory<A>>,
        [(); std::mem::size_of::<T>()]:
{
    fn get(&self) -> Result<T>
    {
        let addr = self.get_address()?;
        let mut buf = [0u8; std::mem::size_of::<T>()];
        self.parent.memory().read_data(addr, &mut buf)?;
        Ok(unsafe { (buf.as_ptr() as *const T).read_unaligned() })
    }
}

impl <'p, T, A, P> Ptr<'p, T, A, P>
    where 
        T: Sized + Copy,
        A: Arch,
        P: Parent<A, Memory: Memory<A>>,
{
    fn new(parent: &'p P, addr: A) -> Self {
        Self {
            parent,
            offset: addr,
            _phantom: PhantomData
        }
    }
}

impl <'p, T, A, P> Parent<A> for Ptr<'p, T, A, P>
    where
        T: Sized + Copy,
        A: Arch,
        P: Parent<A, Memory: Memory<A>>,
{
    type Memory = P::Memory;

    fn memory(&self) -> &Self::Memory {
        self.parent.memory()
    }

    fn memory_mut(&mut self) -> &mut Self::Memory {
        self.parent.memory_mut()
    }
}

trait PtrChain<'p, T, A, P>
    where 
        T: Sized + Copy,
        A: Arch,
        P: Parent<A, Memory: Memory<A>>,
{

    // when we have a 

    // fn ptr(&self, addr: A) -> Ptr<>;
}

///////////// ProcessHandle ////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
struct ProcessHandle<A> {
    _phantom: PhantomData<A>,
}

impl <A: Arch> MemGetAddress<A> for ProcessHandle<A> {
    fn get_address(&self) -> Result<A> {
        Ok(A::null())
    }
}

impl <A> Parent<A> for ProcessHandle<A>
    where 
        A: Arch,
{
    type Memory = ProcessHandle<A>;

    fn memory(&self) -> &Self::Memory {
        self
    }

    fn memory_mut(&mut self) -> &mut Self::Memory {
        self
    }
}

impl <A: Arch> Memory<A> for ProcessHandle<A> {
    fn read_ptr(&self, addr: A) -> Result<A> {
        unimplemented!()
    }

    fn read_data(&self, addr: A, buf: &mut [u8]) -> Result<()> {
        unimplemented!()
    }

    fn write_data(&mut self, addr: A, buf: &[u8]) -> Result<()> {
        unimplemented!()
    }
}

/////////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
struct Array<T> {
    _phantom: PhantomData<T>,
}

#[derive(Copy, Clone)]
struct Game {
    player_list: Array<u32>,
}

fn user() {
    let handle = ProcessHandle::<ArchNative> { _phantom: PhantomData, };

    let something : Ptr<Ptr<Game, u64, impl Parent<u64, Memory: Memory<u64>>>, u64, ProcessHandle<u64>> = Ptr::new(&handle, 0xffff); // whole type has to be known or inferred here already.
    // let gamestruct : Ptr<Game, _, Ptr<ProcessHandle<_>, _, _>>;



}
