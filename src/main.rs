#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(generic_associated_types)]
#![feature(arc_new_cyclic)]
// #![feature(associated_type_bounds)]
// #![feature(impl_trait_in_bindings)]
// #![feature(const_fn)]
// #![feature(new_uninit)]
// #![recursion_limit="512"]
#![allow(dead_code, unused_variables, unused_imports)]
use std::{marker::PhantomData, rc::Rc, rc::Weak};

#[derive(Debug)]
pub enum Error {
    StdIo(std::io::Error),
    RootOrParentInvalid,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::StdIo(e)
    }
}

type Result<T> = std::result::Result<T, Error>;

pub mod arch;
use arch::{A64Le, Arch, ArchNative};

////////////////////////////////////////////////////////////////////
////////////// Tree stuff //////////////////////////////////////////
////////////////////////////////////////////////////////////////////

pub trait MkRoot<A: Arch> {
    type Result<T>;
    type ResultActual<T>;
    fn mk_root<F, Inner>(&self, f: F) -> Self::Result<Inner>
        where
        Inner: Sized + 'static,
        F: FnOnce(Weak<Self::ResultActual<Inner>>) -> Rc<Inner>;
}

/// Roots handle platform-specific stuff; they usually hold the process handle.
pub trait Root<A: Arch>: Parent<A> {
    fn read_pointer(&self, addr: A::Pointer) -> Result<A::Pointer>;
    fn read(&self, addr: A::Pointer, into: &mut [u8]) -> Result<()>;
    fn write(&self, addr: A::Pointer, from: &[u8]) -> Result<()>;
}

pub trait Parent<A: Arch> {
    fn root(&self) -> Result<Weak<dyn Root<A>>>;
    /// Get the base address of self.
    /// For a library, it'll be the base address.
    /// Think of it like shifting the whole address space.
    fn get_address(&self) -> Result<A::Pointer>;
}

////////////////////////////////////////////////////////////////////
////////////// Wrappers ////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

/// A value located at the address (relative to parent).
#[derive(Clone)]
pub struct Value<A: Arch, T: Sized> {
    parent: Weak<dyn Parent<A>>,
    offset: A::Pointer,
    _phantom: PhantomData<T>,
}

#[derive(Clone)]
pub struct Ptr<A: Arch, T: Sized> {
    parent: Weak<dyn Parent<A>>,
    offset: A::Pointer,
    _phantom: PhantomData<T>,
}

#[derive(Clone)]
struct LibraryBase<A: Arch, T: Sized> {
    root: Weak<dyn Root<A>>,
    name: &'static str,
    child: Rc<T>,
}

impl<A: Arch, T: Sized> Parent<A> for Value<A, T> {
    fn root(&self) -> Result<Weak<dyn Root<A>>> {
        self.parent.upgrade().unwrap().root() // TODO: Fix upgrade.unwrap panic!
    }

    fn get_address(&self) -> Result<A::Pointer> {
        if let Some(s) = self.parent.upgrade() {
            Ok(A::ptr_add(&s.get_address()?, &self.offset))
        } else {
            panic!() // TODO: fix upgrade panic!
        }
    }
}

impl<A: Arch, T: Sized> Parent<A> for Ptr<A, T> {
    fn root(&self) -> Result<Weak<dyn Root<A>>> {
        self.parent.upgrade().unwrap().root() // TODO: Fix upgrade.unwrap panic!
    }

    fn get_address(&self) -> Result<A::Pointer> {
        let root;
        let addr;
        {
            let parent = self.parent.upgrade().unwrap();
            root = parent.root()?;
            addr = parent.get_address()?;
        } // parent goes out of scope here and is downgraded. We don't actually NEED to do this, but why not.

        let ptr_at = A::ptr_add(&addr, &self.offset);
        Ok(root.upgrade().unwrap().read_pointer(ptr_at)?)
    }
}

impl<A: Arch, T: Sized> Parent<A> for LibraryBase<A, T> {
    fn root(&self) -> Result<Weak<dyn Root<A>>> {
        Ok(self.root.clone())
    }

    fn get_address(&self) -> Result<A::Pointer> {
        todo!()
    }
}

pub trait Via<A: Arch> {
    type Result;
    fn via<F, Inner>(&self, offset: A::Pointer) -> Self::Result
    where
        F: FnOnce(Weak<dyn Parent<A>>) -> Inner;
}

pub trait ViaLib<A: Arch> {
    type Result<T>;
    fn via_lib<F, Inner>(&self, name: &'static str, f: F) -> Self::Result<Inner>
    where
        Inner: Sized + 'static,
        F: FnOnce(Weak<dyn Parent<A>>) -> Rc<Inner>;
}

pub trait At<A: Arch> {
    type Result<T>;
    fn at<T: Sized + 'static>(&self, offset: A::Pointer) -> Self::Result<T>;
}

// impl<A: Arch> ViaTrait<A> for Weak<dyn Parent<A>> {
//     type Result = ();

//     fn via(&self, offset: A::Pointer) -> Self::Result {
//         todo!()
//     }
// }

////////////////////////////////////////////////////////////////////
////////////// Process Handle //////////////////////////////////////
////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
struct ProcessHandle<A: Arch> {
    _phantom: PhantomData<A>,
    // if you need a handle which can only exist once, use Rc here.
}

/// We need this wrapper so that we don't expose `Rc<RemoteRoot<...>>` to the public.
#[derive(Clone)]
struct RemoteRootActual<A: Arch, T: Sized> {
    process_handle: ProcessHandle<A>,
    child: Rc<T>,
    myself: Weak<dyn Root<A>>,
}

/// Represents the memory address space of another process.
/// Eventually there might be a SelfRoot, or even NetworkRoot.
#[derive(Clone)]
pub struct RemoteRoot<A: Arch, T: Sized> {
    actual: Rc<RemoteRootActual<A, T>>,
}

impl <A: Arch> MkRoot<A> for ProcessHandle<A> {
    type Result<T> = RemoteRoot<A, T>;
    type ResultActual<T> = RemoteRootActual<A, T>;

    fn mk_root<F, Inner>(&self, f: F) -> Self::Result<Inner>
        where
        Inner: Sized + 'static,
        F: FnOnce(Weak<Self::ResultActual<Inner>>) -> Rc<Inner>
    {
        RemoteRoot {
            actual: Rc::new_cyclic(|w_root: &Weak<RemoteRootActual<A, _>>| RemoteRootActual {
                myself: w_root.clone(),
                process_handle: *self,
                child: f(w_root.clone()),
            }),
        }
    }
}

impl<A: Arch> ViaLib<A> for ProcessHandle<A> {
    type Result<T> = RemoteRoot<A, LibraryBase<A, T>>;

    fn via_lib<F, Inner>(&self, name: &'static str, f: F) -> Self::Result<Inner>
    //RemoteRoot<A, LibraryBase<A, Inner>>
    where
        Inner: Sized + 'static,
        F: FnOnce(Weak<dyn Parent<A>>) -> Rc<Inner>,
    {
        self.mk_root(|w_root| {
            Rc::new_cyclic(|w_lib: &Weak<LibraryBase<A, _>>| LibraryBase::<A, Inner> {
                root: w_root.clone(),
                name,
                child: f(w_lib.clone()),
            })
        })
    }
}

impl<A: Arch> At<A> for ProcessHandle<A> {
    type Result<T> = RemoteRoot<A, Value<A, T>>;

    fn at<T: Sized + 'static>(&self, offset: A::Pointer) -> RemoteRoot<A, Value<A, T>> {
        self.mk_root(|w_root| {
            Rc::new_cyclic(|w| Value {
                parent: w_root.clone(),
                offset,
                _phantom: PhantomData,
            })
        })
    }
}


impl<A: Arch, T> Parent<A> for RemoteRootActual<A, T> {
    fn root(&self) -> Result<Weak<dyn Root<A>>> {
        Ok(self.myself.clone())
    }

    fn get_address(&self) -> Result<A::Pointer> {
        Ok(A::ptr_null())
    }
}

impl<A: Arch, T> Root<A> for RemoteRootActual<A, T> {
    fn read_pointer(&self, ptr: A::Pointer) -> Result<A::Pointer> {
        todo!()
    }

    fn read(&self, addr: A::Pointer, into: &mut [u8]) -> Result<()> {
        todo!()
    }

    fn write(&self, addr: A::Pointer, from: &[u8]) -> Result<()> {
        todo!()
    }
}

////////////////////////////////////////////////////////////////////
//////////////// SelfRoot //////////////////////////////////////////
////////////////////////////////////////////////////////////////////

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
