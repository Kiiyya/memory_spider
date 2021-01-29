#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(associated_type_bounds)]
#![feature(impl_trait_in_bindings)]
#![allow(dead_code, unused_variables)]
use std::{marker::PhantomData};

type Result<T> = std::result::Result<T, ()>;

pub mod arch;
use arch::{Arch, ArchNative};

pub trait MemR<A: Arch> {
    fn read_pointer(&self, addr: A::Pointer) -> Result<A::Pointer> where Self: Sized;

    fn read_bytes(&self, addr: A::Pointer, buf: &mut [u8]) -> Result<()> where Self: Sized;

    fn read_t<T>(&self, addr: A::Pointer) -> Result<T>
        where Self: Sized,
        [(); std::mem::size_of::<T>()]:
    {
        let mut buf = [0u8; std::mem::size_of::<T>()];
        self.read_bytes(addr, &mut buf)?;
        Ok(unsafe { (buf.as_ptr() as *const T).read_unaligned() })
    }
}

pub trait MemRw<A: Arch> : MemR<A> {
    fn write_data(&self, addr: A::Pointer, buf: &[u8]) -> Result<()> where Self: Sized;
    fn write_t<T>(&self, val: &T) -> Result<()> where Self: Sized;
}

//////////////////////////////////////////////////////////////////////////////////////
// Things concerning more semantic vlaues.

pub trait GetAddress<A: Arch> {
    /// Where is our value?
    fn get_address(&self) -> Result<A::Pointer>;
}

pub trait Get<T: Sized + Copy> {
    fn get(&self) -> Result<T>;
}

// impl <T: Sized + Copy, A: Arch, X: GetAddress<T, A>> Get<T, A> for X {
//     fn get(&self) -> Result<T> {
        
//     }
// }

pub trait Set<T: Sized + Copy> {
    fn set(&self, value: &T) -> Result<()>;
}

////////////////////////////////////////////////////////////////////////////////////

pub trait Parent<A> : GetAddress<A>
    where
        A: Arch,
{
    // type Memory;
    fn memory_r(&self) -> &dyn MemR<A>;
    fn memory_rw(&self) -> &dyn MemRw<A>;
    // /// Might have to implement blocking or something.
    // fn memory_mut(&self) -> &mut M;
}

//////////// Pointer ///////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Ptr<'p, T, A>
    where
        T: Sized + Copy,
        A: Arch,
{
    parent: &'p dyn Parent<A>,
    offset: A::Pointer,
    _phantom: PhantomData<T>,
}

impl <'p, T, A> GetAddress<A> for Ptr<'p, T, A>
    where
        T: Sized + Copy,
        A: Arch,
{
    /// The pointer itself, not where it points to.
    fn get_address(&self) -> Result<A::Pointer>
        where
            T: Sized
    {
        Ok(A::ptr_add(
            self.parent.get_address()?,
            self.offset,
        ))
    }
}

impl <'p, T, A> Get<T> for Ptr<'p, T, A>
    where
        T: Sized + Copy,
        A: Arch,
        [(); std::mem::size_of::<T>()]:
{
    fn get(&self) -> Result<T>
    {
        let addr = self.get_address()?;
        let mut buf = [0u8; std::mem::size_of::<T>()];
        self.parent.memory_r().read_bytes(addr, &mut buf)?;
        Ok(unsafe { (buf.as_ptr() as *const T).read_unaligned() })
    }
}

impl <'p, T, A> Ptr<'p, T, A>
    where 
        T: Sized + Copy,
        A: Arch,
{
    // fn new(parent: &'p P, addr: A) -> Self {
    //     Self {
    //         parent,
    //         offset: addr,
    //         _phantom: PhantomData,
    //         _phantom2: PhantomData,
    //     }
    // }

    // fn next(&'p self, addr: A) -> Ptr<'p, Ptr<'p, T, A, M, impl Parent<A, M>>, A, M, impl Parent<A, M>> { // impossible. Infinite unification
    //     Ptr {
    //         parent: &self,
    //         offset: addr,
    //         _phantom: PhantomData,
    //         _phantom2: PhantomData,
    //     }
    // }
}

impl <'p, T, A> Parent<A> for Ptr<'p, T, A>
    where
        T: Sized + Copy,
        A: Arch,
{
}

///////////// ProcessHandle ////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
struct ProcessHandle<A> {
    _phantom: PhantomData<A>,
}

impl <A: Arch> GetAddress<A> for ProcessHandle<A> {
    fn get_address(&self) -> Result<A::Pointer> {
        Ok(A::ptr_null())
    }
}

impl <A> Parent<A> for ProcessHandle<A>
    where 
        A: Arch,
{

}

impl <A: Arch> MemR<A> for ProcessHandle<A> {
    fn read_pointer(&self, addr: A::Pointer) -> Result<A::Pointer> {
        todo!()
    }

    fn read_bytes(&self, addr: A::Pointer, buf: &mut [u8]) -> Result<()> {
        todo!()
    }
}

impl <A: Arch> MemRw<A> for ProcessHandle<A> {
    fn write_data(&self, addr: A::Pointer, buf: &[u8]) -> Result<()> {
        todo!()
    }

    fn write_t<T>(&self, val: &T) -> Result<()> {
        todo!()
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

fn user() -> Result<()> {
    let handle = ProcessHandle::<ArchNative> { _phantom: PhantomData, };

    let something : Ptr<u8, _, _, _> = Ptr::new(&handle, 0xffff); // whole type has to be known or inferred here already.
    // let gamestruct : Ptr<Game, _, Ptr<ProcessHandle<_>, _, _>>;



    todo!()
}
