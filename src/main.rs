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
use tree::{LibraryBase, ViaLib, Via, Value, Ptr, At};
use process::{ProcessHandle, };


////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

fn main() -> Result<()> {
    let ph: ProcessHandle<A64Le> = ProcessHandle::new();

    let x = ph.via_lib("GameAssembly.dll", |inner| Rc::new(0));

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

    todo!()
}
