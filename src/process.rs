use std::{fs::write, marker::PhantomData, rc::Rc, rc::Weak};
use crate::error::{Error, Result};
use crate::{arch::Arch, Root, MkRoot, Parent, };
use crate::wrappers::{Via, ViaLib, At, LibraryBase, Value, };

#[derive(Copy, Clone)]
pub struct ProcessHandle<A: Arch> {
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

impl <A: Arch> MkRoot<A> for ProcessHandle<A> {
    type TRoot<T> = RemoteRoot<A, T>;
    type TRootActual<T> = RemoteRootActual<A, T>;

    fn mk_root<F, Inner>(&self, f: F) -> Self::TRoot<Inner>
        where
        Inner: Sized + 'static,
        F: FnOnce(&Weak<Self::TRootActual<Inner>>) -> Rc<Inner>
    {
        RemoteRoot {
            actual: Rc::new_cyclic(|w_root: &Weak<RemoteRootActual<A, _>>| RemoteRootActual {
                myself: w_root.clone(),
                process_handle: *self,
                child: f(&w_root.clone()),
            }),
        }
    }
}

impl<A, R> ViaLib<A> for R //ProcessHandle<A>
    where
        A: Arch,
        R: MkRoot<A>,
{
    type Result<T> = R::TRootActual<LibraryBase<A, T>>;

    fn via_lib<F, Inner>(&self, name: &'static str, f: F) -> Self::Result<Inner>
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
