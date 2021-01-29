#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(associated_type_bounds)]
#![feature(impl_trait_in_bindings)]
#![feature(const_fn)]
#![feature(const_if_match)]
#![feature(new_uninit)]
#![recursion_limit="512"]
#![allow(dead_code, unused_variables, unused_imports)]
use std::{cell::Cell, marker::{PhantomData, PhantomPinned}, mem::MaybeUninit, pin::Pin, rc::Rc};

type Result<T> = std::result::Result<T, ()>;

// mod mess;
pub mod arch;
use arch::{Arch, A64Le, ArchNative};

pub trait Deref<T> {
    fn deref(&self) -> T;
}

pub trait GetAddress<A: Arch> {
    fn get_address(&self) -> A::Pointer;
}

impl <'root, A: Arch, T: Sized> GetAddress<A> for LibraryBase<'root, A, T> {
    fn get_address(&self) -> A::Pointer {
        todo!()
    }
}

pub trait Parent<A> {

}


#[derive(Copy, Clone)]
struct ProcessHandle<A: Arch> {
    _phantom: PhantomData<A>,
}

/////////////////////////
pub trait Root<A: Arch> {
    fn read_pointer(&self, ptr: A::Pointer) -> A::Pointer;
}
pub struct MyRoot<'ph, A: Arch, T: Sized> {
    process_handle: &'ph ProcessHandle<A>,
    child: Option<Pin<Box<T>>>,
    // _phantom: PhantomData<T>,
    // _pp: PhantomPinned,
}

#[derive(Clone)]
struct LibraryBase<'root, A: Arch, T: Sized> {
    root: Pin<&'root dyn Root<A>>,
    name: &'static str,
    child: Option<Pin<Box<T>>>,
    // _phantom: PhantomData<T>,
}

pub struct Ptr<A: Arch, T: Sized + Copy> {
    // parent: &'a dyn Remote<A, &()>,
    address: A::Pointer,
    _phantom: PhantomData<T>,
}

impl <A: Arch, T: Sized> Root<A> for MyRoot<'_, A, T> {
    fn read_pointer(&self, ptr: A::Pointer) -> A::Pointer {
        todo!()
    }
}
impl <A: Arch, T: Sized> Parent<A> for LibraryBase<'_, A, T> {

}
impl <A: Arch> ProcessHandle<A> {
    // pub fn point<T, const N: usize>(&self, offsets: [A::Pointer; N]) -> () {
    //     if N > 0 {
    //         self.point(offsets[1..].try_into().unwrap());
    //     }
    //     todo!();
    // }

    pub fn via(&self, offset: A::Pointer) -> Ptr<A, *const ()>
    {
        Ptr::<A, *const ()> {
            address: offset,
            _phantom: PhantomData,
        }
    }

    pub fn via_lib<'ph, F, Inner>(&'ph self, lib: &'static str, f: F) -> Pin<Box<MyRoot<'ph, A, LibraryBase<'ph, A, Inner>>>>
        where
            Inner: Sized + 'ph + Unpin, // TODO: REALLY unsure if this +Unpin is safe here!
            F: FnOnce(Pin<&'ph dyn Root<A>>, Pin<&'ph dyn Parent<A>>) -> Pin<Box<Inner>>
    {
        let mut root = Box::pin(MyRoot::<'ph, _, _> {
            process_handle: self,
            child: None,
            // _phantom: PhantomData,
            // _pp: PhantomPinned,
        });

        // let reff : &'ph dyn Root<A> = root.as_ref();
        root.child = Some(Box::pin(LibraryBase::<'ph, _, _> {
            root: root.as_ref(),
            name: lib,
            // _phantom: PhantomData,
            child: None,
        }));

        // TODO: change to unwrap_unchecked some day.
        root.child.unwrap().child = Some(f(root.as_ref(), root.child.unwrap().as_ref()));

        root
        // ()
    }
}


fn user() -> Result<()> {
    let ph : ProcessHandle<A64Le> = ProcessHandle { _phantom: PhantomData };

    // let game : impl Remote<A64Le, *const ()> = ph.point_somewhere(A64Le::ptr_null());

    // let game2 : impl Remote<A64Le, *const *const *const Game> = ph.via(0x123).via(0x01).via(0x02).to::<Game>();
    // let game3 = ph.via3([0x123, 0x01, 0x02]).to::<Game>();

    // let x : Box<dyn Remote<A64Le, LibraryBase<Ptr<u32>>>> = ph.via_lib("GameAssembly.dll",
    //     |root, parent1| inner.via(0x1734672,
    //         |parent2| inner2.to::<u32>()
    //     )
    // );

    // let y = x.

    todo!()
}
