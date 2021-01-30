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
pub mod wrappers;
pub mod process;
use error::{Error, Result};
use arch::{A64Le, Arch, ArchNative};
use wrappers::{LibraryBase, ViaLib, Via, Value, Ptr, At};
use process::{ProcessHandle, };

////////////////////////////////////////////////////////////////////
////////////// Tree stuff //////////////////////////////////////////
////////////////////////////////////////////////////////////////////

/// Roots handle platform-specific stuff; they usually hold the process handle.
pub trait Root<A: Arch>: Parent<A> {
    fn read_pointer(&self, addr: A::Pointer) -> Result<A::Pointer>;
    fn read(&self, addr: A::Pointer, into: &mut [u8]) -> Result<()>;
    fn write(&self, addr: A::Pointer, from: &[u8]) -> Result<()>;
}

/// Not pub because it's internal only really.
/// A convenience thing used by the Via/ViaLib/Value/etc impelmentations for Roots.
trait MkRoot<A: Arch> {
    type TRoot<T>;
    type TRootActual<T>;
    fn mk_root<F, Inner>(&self, f: F) -> Self::TRoot<Inner>
        where
        Inner: Sized + 'static,
        F: FnOnce(&Weak<Self::TRootActual<Inner>>) -> Rc<Inner>;
}

pub trait Parent<A: Arch> {
    fn root(&self) -> Result<Weak<dyn Root<A>>>;
    /// Get the base address of self.
    /// For a library, it'll be the base address.
    /// Think of it like shifting the whole address space.
    fn get_address(&self) -> Result<A::Pointer>;
}

////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

fn main() -> Result<()> {
    let ph: ProcessHandle<A64Le> = ProcessHandle {
        _phantom: PhantomData,
    };

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
