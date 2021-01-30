
use std::{fs::write, marker::PhantomData, rc::Rc, rc::Weak};
use crate::{Parent, arch::Arch, Root, Result, Error,};

////////////////////////////////////////////////////////////////////
////////////// At //////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

pub trait At<A: Arch> {
    type Result<T>;
    fn at<T: Sized + 'static>(&self, offset: A::Pointer) -> Self::Result<T>;
}

/// A value located at the address (relative to parent).
#[derive(Clone)]
pub struct Value<A: Arch, T: Sized> {
    parent: Weak<dyn Parent<A>>,
    offset: A::Pointer,
    _phantom: PhantomData<T>,
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

////////////////////////////////////////////////////////////////////
////////////// Via /////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

pub trait Via<A: Arch> {
    type Result;
    fn via<F, Inner>(&self, offset: A::Pointer) -> Self::Result
    where
        F: FnOnce(Weak<dyn Parent<A>>) -> Inner;
}

#[derive(Clone)]
pub struct Ptr<A: Arch, T: Sized> {
    parent: Weak<dyn Parent<A>>,
    offset: A::Pointer,
    _phantom: PhantomData<T>,
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

////////////////////////////////////////////////////////////////////
////////////// ViaLib //////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

pub trait ViaLib<A: Arch> {
    type Result<T>;
    fn via_lib<F, Inner>(&self, name: &'static str, f: F) -> Self::Result<Inner>
    where
        Inner: Sized + 'static,
        F: FnOnce(Weak<dyn Parent<A>>) -> Rc<Inner>;
}

#[derive(Clone)]
pub struct LibraryBase<A: Arch, T: Sized> {
    root: Weak<dyn Root<A>>,
    name: &'static str,
    child: Rc<T>,
}

impl<A: Arch, T: Sized> Parent<A> for LibraryBase<A, T> {
    fn root(&self) -> Result<Weak<dyn Root<A>>> {
        Ok(self.root.clone())
    }

    fn get_address(&self) -> Result<A::Pointer> {
        todo!()
    }
}
