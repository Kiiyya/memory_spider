//! How to obtain a root. The entry point, so to speak.
//! Anything that supports MkRoot can be turned into a root.
//! As such, you will want your process handles to implement MkRoot, then everything
//! else will fall into place automatically.
//! This module contains blanket implementations of Via, etc, which do that.

use std::rc::{Rc, Weak};

use crate::{arch::Arch, tree::{LibraryBase, Parent, Root, ViaLib}};

/// Not pub because it's internal only really.
/// A convenience thing used by the Via/ViaLib/Value/etc impelmentations for Roots.
pub trait MkRoot<A: Arch> {
    type TRoot<T>;
    type TRootActual<T: 'static> : Root<A> + 'static;
    fn mk_root<F, Inner>(&self, f: F) -> Self::TRoot<Inner>
        where
        Inner: Sized + 'static,
        F: FnOnce(&Weak<Self::TRootActual<Inner>>) -> Rc<Inner>;
}

impl<A, R> ViaLib<A> for R //ProcessHandle<A>
    where
        A: Arch,
        R: MkRoot<A>,
        // <R as MkRoot<A>>::TRootActual<_>: 'static
{
    type Result<T> = R::TRoot<LibraryBase<A, T>>;

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
