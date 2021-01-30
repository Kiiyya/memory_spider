#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(generic_associated_types)]
#![feature(arc_new_cyclic)]
#![feature(associated_type_bounds)]
// #![feature(impl_trait_in_bindings)]
// #![feature(const_fn)]
// #![feature(new_uninit)]
// #![recursion_limit="512"]
#![allow(dead_code, unused_variables, unused_imports)]
use std::{fs::write, marker::PhantomData, rc::Rc, rc::Weak};

pub mod arch;
pub mod error;
pub mod tree;
pub mod process;
mod mkroot;
use error::{Error, Result};
use arch::{A64Le, Arch, ArchNative};
use tree::{At, LibraryBase, Ptr, Root, Value, Via, ViaLib};
use process::{ProcessHandle, RemoteRoot};

pub trait Get<A: Arch> {
    type T;
    // type Res = Result<Rc<Self::T>>;
    fn get(&self) -> Result<Self::T>;
}


////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

fn main() -> Result<()> {
    let ph: ProcessHandle<A64Le> = ProcessHandle::new();

    let x = 123_i32;

    // let x = ph.via_lib("GameAssembly.dll", |inner| Rc::new(0));
    let test : RemoteRoot<A64Le, Value<A64Le, i32>> = ph.at::<i32>(&x as *const _ as u64);
    let x2 = test.get()?.get()?;

    let x_ref : Rc<Value<A64Le, i32>> = test.get()?;
    let x3 : i32 = x_ref.get()?;

    println!("x: {}, x2: {}, x3: {}", x, x2, x3);


    // let game : impl Remote<A64Le, *const ()> = ph.point_somewhere(A64Le::ptr_null());

    // let game2 : impl Remote<A64Le, *const *const *const Game> = ph.via(0x123).via(0x01).via(0x02).to::<Game>();
    // let game3 = ph.via3([0x123, 0x01, 0x02]).to::<Game>();

    // let somevalue = ph.at::<u32>(0xffff); // = .via(0xffff, |inner| inner.to::<Game>());

    // let x : Box<dyn Remote<A64Le, LibraryBase<Ptr<u32>>>> = ph.via_lib("GameAssembly.dll",
    //     |root, parent1| inner.via(0x1734672,
    //         |parent2| inner2.to::<u32>()
    //     )
    // );

    // let y = x.

    Ok(())
}
