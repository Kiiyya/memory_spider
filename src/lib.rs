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
// enum Assert<const COND: bool> {}
// trait IsTrue {}
// impl IsTrue for Assert<true> {}

trait TypeGen<T, const N: usize> {
    type Result;
}

struct Wrap<T>(PhantomData<T>);
struct Helper;

impl <T> TypeGen<T, 0> for Helper { type Result = T; }
impl <T> TypeGen<T, 1> for Helper { type Result = Wrap<T>; }
impl <T> TypeGen<T, 2> for Helper { type Result = Wrap<Wrap<T>>; }
impl <T> TypeGen<T, 3> for Helper { type Result = Wrap<Wrap<Wrap<T>>>; }
impl <T> TypeGen<T, 4> for Helper { type Result = Wrap<Wrap<Wrap<Wrap<T>>>>; }
impl <T> TypeGen<T, 5> for Helper { type Result = Wrap<Wrap<Wrap<Wrap<Wrap<T>>>>>; }


// impl <T, const N: usize> TypeGen<T, N> for Helper
//     where
//         Assert::<{N > 0}>: IsTrue,
//         [(); N - 1]:,
// {
//     type Result = Wrap<<Helper as TypeGen<T, {N - 1}>>::Result>;
// }

// produce `Wrap<Wrap<Wrap<...>>>` with exactly N `Wrap`s.
pub fn wrap<T, const N: usize>(array: [T; N]) -> <Helper as TypeGen<T, N>>::Result {
    todo!()
}

// const fn matthh(n: usize) -> usize {
//     n - 1
// }

// impl <A: Arch> ProcessHandle<A> {
//     pub fn point<T, const N: usize>(&self, offsets: [A::Pointer; N]) -> <Helper as TypeGen<N, false>>::Result {
//         if N > 0 {
//             self.point(offsets[1..].try_into().unwrap());
//         }
//         todo!();
//     }
// }

fn user() -> Result<()> {
    // let ph : ProcessHandle<Arch64Little> = ProcessHandle { _phantom: PhantomData };





    todo!()
}
