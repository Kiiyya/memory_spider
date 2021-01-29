#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(associated_type_bounds)]
#![feature(impl_trait_in_bindings)]
#![feature(const_fn)]
#![feature(const_if_match)]
#![feature(new_uninit)]
#![recursion_limit="512"]
#![allow(dead_code, unused_variables, unused_imports)]
use std::{cell::Cell, marker::{PhantomData, PhantomPinned}, mem::MaybeUninit, pin::Pin, rc::Rc, rc::Weak};

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

impl <'root, A: Arch, T: Sized> GetAddress<A> for LibraryBase<A, T> {
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
pub trait Root<'ph, A: Arch> {
    fn read_pointer(&self, ptr: A::Pointer) -> A::Pointer;
}
pub struct MyRoot<'ph, A: Arch, T: Sized> {
    process_handle: &'ph ProcessHandle<A>,
    child: Option<Rc<T>>,
    // _phantom: PhantomData<T>,
    // _pp: PhantomPinned,
}
impl <'ph, A: Arch, T> Root<'ph, A> for MyRoot<'_, A, T> {
    fn read_pointer(&self, ptr: A::Pointer) -> A::Pointer {
        todo!()
    }
}

#[derive(Clone)]
struct LibraryBase<'ph, A: Arch, T: Sized> {
    root: Weak<dyn Root<'ph, A>>,
    name: &'static str,
    child: Option<Rc<T>>,
    // _phantom: PhantomData<T>,
}

pub struct Ptr<A: Arch, T: Sized + Copy> {
    // parent: &'a dyn Remote<A, &()>,
    address: A::Pointer,
    _phantom: PhantomData<T>,
}


impl <'ph, A: Arch, T: Sized> Parent<A> for LibraryBase<'ph, A, T> {

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

    pub fn via_lib<'ph, F, Inner>(&'ph self, lib: &'static str, f: F) -> Rc<MyRoot<'ph, A, LibraryBase<A, Inner>>>
        where
            Inner: Sized + 'ph,
            F: FnOnce(Weak<dyn Root<A>>, Weak<dyn Parent<A>>) -> Rc<Inner>
    {
        let mut root = Rc::new(MyRoot::<'ph> {
            process_handle: self,
            child: None,
        });

        // let reff : &'ph dyn Root<A> = root.as_ref();
        root.child = Some(Rc::new(LibraryBase {
            root: Rc::downgrade(&root) as Weak<dyn Root<'ph, A>>,
            name: lib,
            child: None,
        }));

        // TODO: change to unwrap_unchecked some day.
        // root.child.unwrap().child = Some(f(root.as_ref(), root.child.unwrap().as_ref()));

        root
    }
}


fn user() -> Result<()> {
    let ph : ProcessHandle<A64Le> = ProcessHandle { _phantom: PhantomData };

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
