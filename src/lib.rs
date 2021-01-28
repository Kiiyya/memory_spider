#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(associated_type_bounds)]
#![feature(impl_trait_in_bindings)]
#![feature(const_fn)]
#![feature(const_if_match)]
#![recursion_limit="256"]
#![allow(dead_code, unused_variables, unused_imports)]
use std::{marker::PhantomData};

type Result<T> = std::result::Result<T, ()>;

// mod mess;
pub mod arch;
use arch::{Arch, Arch64Little, ArchNative};

pub trait Deref<T> {
    fn deref(&self) -> T;
}

pub struct Remote<T> {
    _phantom: PhantomData<T>,
}

impl <T> Deref<Remote<T>> for Remote<&T>
{
    fn deref(&self) -> Remote<T> {
        todo!()
    }
}

#[derive(Copy, Clone)]
struct ProcessHandle<A: Arch> {
    _phantom: PhantomData<A>,
}

/////////////////////////

impl <A: Arch> ProcessHandle<A> {
    // pub fn point<T, const N: usize>(&self, offsets: [A::Pointer; N]) -> () {
    //     if N > 0 {
    //         self.point(offsets[1..].try_into().unwrap());
    //     }
    //     todo!();
    // }

    pub fn point_somewhere(&self, offset: A::Pointer) -> Remote<&()> {
        
    }
}

fn user() -> Result<()> {
    // let ph : ProcessHandle<Arch64Little> = ProcessHandle { _phantom: PhantomData };





    todo!()
}
