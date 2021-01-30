
use std::{marker::PhantomData, mem::size_of, rc::Rc, rc::Weak};
use crate::{Error, Get, Result, arch::Arch};

////////////////////////////////////////////////////////////////////
////////////// Tree stuff //////////////////////////////////////////
////////////////////////////////////////////////////////////////////

pub trait GetAddress<A: Arch> {
    /// Get the base address of self.
    /// For a library, it'll be the base address.
    /// Think of it like shifting the whole address space.
    fn get_address(&self) -> Result<A::Pointer>;
}

pub trait Parent<A: Arch> : GetAddress<A> {
    fn root(&self) -> Result<Weak<dyn Root<A>>>;
}

/// Roots handle platform-specific stuff; they usually hold the process handle.
pub trait Root<A: Arch> : Parent<A> {
    fn read_pointer(&self, addr: A::Pointer) -> Result<A::Pointer>;

    fn read(&self, addr: A::Pointer, into: &mut [u8]) -> Result<()>;
    fn write(&self, addr: A::Pointer, from: &[u8]) -> Result<()>;
}

/// Since functions which have generics on them don't work because vtable derps,
/// we need this trick. It's really just a trick, because I couldn't put them
/// into Root directly...
trait TypedMemory<A: Arch> {
    fn read_t<T: Sized>(&self, addr: A::Pointer) -> Result<T>
        where [(); std::mem::size_of::<T>()]:;
    fn write_t<T: Sized>(&self, addr: A::Pointer, value: &T) -> Result<()>
        where [(); std::mem::size_of::<T>()]:;
}

impl <A: Arch> TypedMemory<A> for Rc<dyn Root<A>> {
    fn read_t<T: Sized>(&self, addr: A::Pointer) -> Result<T>
        where [(); std::mem::size_of::<T>()]:
    {
        let mut buf = [0u8; std::mem::size_of::<T>()];
        self.read(addr, &mut buf)?;
        Ok(unsafe { (buf.as_ptr() as *const T).read_unaligned() })
    }

    fn write_t<T: Sized>(&self, addr: A::Pointer, value: &T) -> Result<()>
        where [(); std::mem::size_of::<T>()]:
    {
        todo!()
    }
}

////////////////////////////////////////////////////////////////////
////////////// At //////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

/// The value we want, is at (parent address) + (offset).
pub trait At<A: Arch> {
    type Result<T>;
    fn at<T: Sized + 'static>(&self, offset: A::Pointer) -> Self::Result<T>;
}

/// A value located at the address (relative to parent).
#[derive(Clone)]
pub struct Value<A: Arch, T: Sized> {
    pub(crate) parent: Weak<dyn Parent<A>>,
    pub(crate) offset: A::Pointer,
    pub(crate) _phantom: PhantomData<T>,
}

impl <A: Arch, T> GetAddress<A> for Value<A, T> {
    fn get_address(&self) -> Result<A::Pointer> {
        if let Some(s) = self.parent.upgrade() {
            Ok(s.get_address()? + self.offset)
            // Ok(A::ptr_add(&s.get_address()?, &self.offset))
        } else {
            panic!() // TODO: fix upgrade panic!
        }
    }
}

impl<A: Arch, T: Sized> Parent<A> for Value<A, T> {
    fn root(&self) -> Result<Weak<dyn Root<A>>> {
        self.parent.upgrade().unwrap().root() // TODO: Fix upgrade.unwrap panic!
    }

    // fn get_address(&self) -> Result<A::Pointer> {
    //     if let Some(s) = self.parent.upgrade() {
    //         Ok(A::ptr_add(&s.get_address()?, &self.offset))
    //     } else {
    //         panic!() // TODO: fix upgrade panic!
    //     }
    // }
}

// maybe in the future we can have ValueNumeric specifically for numeric types,
// which then also takes into consideratio endianness.
impl <A, T> Get<A> for Value<A, T>
    where
        A: Arch,
        T: Sized + Copy,
        [(); std::mem::size_of::<T>()]:
{
    type T = T;

    fn get(&self) -> Result<T> {
        match self.root()?.upgrade() {
            Some(root) => Ok(root.read_t(self.get_address()?)?),
            None => Err(Error::RootDropped),
        }
    }
}

////////////////////////////////////////////////////////////////////
////////////// Via /////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

pub trait Via<A: Arch> {
    type Result<T>;
    fn via<F, Inner>(&self, offset: A::Pointer) -> Self::Result<Inner>
    where
        Inner: Sized + 'static,
        F: FnOnce(Weak<dyn Parent<A>>) -> Rc<Inner>;
}

#[derive(Clone)]
pub struct Ptr<A: Arch, T: Sized> {
    pub(crate) parent: Weak<dyn Parent<A>>,
    pub(crate) offset: A::Pointer,
    pub(crate) _phantom: PhantomData<T>,
}

impl <A: Arch, T> GetAddress<A> for Ptr<A, T> {
    fn get_address(&self) -> Result<A::Pointer> {
        let root;
        let addr;
        {
            let parent = self.parent.upgrade().unwrap();
            root = parent.root()?;
            addr = parent.get_address()?;
        } // parent goes out of scope here and is downgraded. We don't actually NEED to do this, but why not.

        // let ptr_at = A::ptr_add(&addr, &self.offset);
        let ptr_at = addr + self.offset;
        Ok(root.upgrade().unwrap().read_pointer(ptr_at)?)
    }
}

impl<A: Arch, T: Sized> Parent<A> for Ptr<A, T> {
    fn root(&self) -> Result<Weak<dyn Root<A>>> {
        self.parent.upgrade().unwrap().root() // TODO: Fix upgrade.unwrap panic!
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
    pub(crate) root: Weak<dyn Root<A>>,
    pub(crate) name: &'static str,
    pub(crate) child: Rc<T>,
}

impl <A: Arch, T> GetAddress<A> for LibraryBase<A, T> {
    fn get_address(&self) -> Result<A::Pointer> {
        todo!()
    }
}

impl<A: Arch, T: Sized> Parent<A> for LibraryBase<A, T> {
    fn root(&self) -> Result<Weak<dyn Root<A>>> {
        Ok(self.root.clone())
    }
}
