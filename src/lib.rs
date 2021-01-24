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
pub trait Memory<A: Arch> : Copy {
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

pub trait Parent<A, M> : MemGetAddress<A> + Copy
    where
        A: Arch,
        M: Memory<A>,
{
    fn memory(&self) -> &M;
    // /// Might have to implement blocking or something.
    // fn memory_mut(&self) -> &mut M;
}

//////////// Pointer ///////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Ptr<'p, T, A, M, P>
    where
        T: Sized + Copy,
        A: Arch,
        M: Memory<A>,
        P: Parent<A, M>,
{
    parent: &'p P,
    offset: A,
    _phantom: PhantomData<T>,
    // for some reason, rustc complains that M is unused otherwise...
    _phantom2: PhantomData<M>,
}

impl <'p, T, A, M, P> MemGetAddress<A> for Ptr<'p, T, A, M, P>
    where
        T: Sized + Copy,
        A: Arch,
        M: Memory<A>,
        P: Parent<A, M>,
{
    fn get_address(&self) -> Result<A> {
        Ok(self.parent.get_address()?.add(self.offset))
    }
}

impl <'p, T, A, M, P> MemRead<T, A> for Ptr<'p, T, A, M, P>
    where
        T: Sized + Copy,
        A: Arch,
        M: Memory<A>,
        P: Parent<A, M>,
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

#[derive(Copy, Clone)]
struct FakeParent;

impl <A> MemGetAddress<A> for FakeParent
    where
        A: Arch,
{
    fn get_address(&self) -> Result<A> {
        panic!()
    }
}

impl <A, M> Parent<A, M> for FakeParent
    where
        A: Arch,
        M: Memory<A>,
{
    fn memory(&self) -> &M {
        panic!()
    }
}

impl <'p, T, A, M, P> Ptr<'p, T, A, M, P>
    where 
        T: Sized + Copy,
        A: Arch,
        M: Memory<A>,
        P: Parent<A, M>,
{
    fn new(parent: &'p P, addr: A) -> Self {
        Self {
            parent,
            offset: addr,
            _phantom: PhantomData,
            _phantom2: PhantomData,
        }
    }

    fn next(&self, addr: A) -> Ptr<'p, Ptr<'p, T, A, M, FakeParent>, A, M, P> { // impossible. Infinite unification

    }
}

impl <'p, T, A, M, P> Parent<A, M> for Ptr<'p, T, A, M, P>
    where
        T: Sized + Copy,
        A: Arch,
        M: Memory<A>,
        P: Parent<A, M>,
{
    fn memory(&self) -> &M {
        self.parent.memory()
    }

    // fn memory_mut(&mut self) -> &mut M {
    //     self.parent.memory_mut()
    // }
}

// trait PtrChain<'p, T, A, P>
//     where 
//         T: Sized + Copy,
//         A: Arch,
//         P: Parent<A, Memory: Memory<A>>,
// {

//     // when we have a 

//     // fn ptr(&self, addr: A) -> Ptr<>;
// }

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

impl <A> Parent<A, ProcessHandle<A>> for ProcessHandle<A>
    where 
        A: Arch,
{
    fn memory(&self) -> &Self {
        self
    }

    // fn memory_mut(&mut self) -> &mut Self {
    //     self
    // }
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

#[derive(Copy, Clone)]
struct NoParent;

impl <A> MemGetAddress<A> for NoParent
    where
        A: Arch,
{
    fn get_address(&self) -> Result<A> {
        panic!()
    }
}

impl <A, M> Parent<A, M> for NoParent
    where
        A: Arch,
        M: Memory<A>,
{
    fn memory(&self) -> &M {
        panic!()
    }
}

fn user() -> Result<()> {
    let handle = ProcessHandle::<ArchNative> { _phantom: PhantomData, };

    let something : Ptr<u8, _, _, _> = Ptr::new(&handle, 0xffff); // whole type has to be known or inferred here already.
    // let gamestruct : Ptr<Game, _, Ptr<ProcessHandle<_>, _, _>>;

    let target : Ptr<u8, u64, ProcessHandle<u64>, NoParent>;

    let x = something.get()?;


    todo!()
}
