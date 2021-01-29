#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(arc_new_cyclic)]
// #![feature(associated_type_bounds)]
// #![feature(impl_trait_in_bindings)]
// #![feature(const_fn)]
// #![feature(new_uninit)]
// #![recursion_limit="512"]
#![allow(dead_code, unused_variables, unused_imports)]
use std::{cell::Cell, marker::{PhantomData, PhantomPinned}, mem::MaybeUninit, pin::Pin, rc::Rc, rc::Weak};

type Result<T> = std::result::Result<T, ()>;

// mod mess;
pub mod arch;
use arch::{Arch, A64Le, ArchNative};

// pub trait Deref<T> {
//     fn deref(&self) -> T;
// }

// pub trait GetAddress<A: Arch> {
//     fn get_address(&self) -> A::Pointer;
// }

// impl <'root, A: Arch, T: Sized> GetAddress<A> for LibraryBase<A, T> {
//     fn get_address(&self) -> A::Pointer {
//         todo!()
//     }
// }

pub trait Parent<A> {

}


#[derive(Copy, Clone)]
struct ProcessHandle<A: Arch> {
    _phantom: PhantomData<A>,
    // if you need a handle which can only exist once, use Rc here.
}

/////////////////////////
pub trait Root<A: Arch> {
    fn read_pointer(&self, ptr: A::Pointer) -> A::Pointer;
}
pub struct MyRoot<A: Arch, T: Sized> {
    process_handle: ProcessHandle<A>,
    child: Rc<T>,
    // _phantom: PhantomData<T>,
    // _pp: PhantomPinned,
}
impl <A: Arch, T> Root<A> for MyRoot<A, T> {
    fn read_pointer(&self, ptr: A::Pointer) -> A::Pointer {
        todo!()
    }
}

#[derive(Clone)]
struct LibraryBase<A: Arch, T: Sized> {
    root: Weak<dyn Root<A>>,
    name: &'static str,
    child: Rc<T>,
    // _phantom: PhantomData<T>,
}

pub struct Ptr<A: Arch, T: Sized + Copy> {
    // parent: &'a dyn Remote<A, &()>,
    address: A::Pointer,
    _phantom: PhantomData<T>,
}


impl <A: Arch, T: Sized> Parent<A> for LibraryBase<A, T> {

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

    pub fn via_lib<F, Inner>(&self, lib: &'static str, f: F) -> Rc<MyRoot<A, LibraryBase<A, Inner>>>
        where
            Inner: Sized +  'static,
            F: FnOnce(Weak<dyn Root<A>>, Weak<dyn Parent<A>>) -> Rc<Inner>,
    {
        // let mut root = Rc::new(MyRoot {
        //     process_handle: *self,
        //     child: None,
        // });
        let root : Rc<MyRoot<A, LibraryBase<A, Inner>>> = Rc::new_cyclic(|w_root: &Weak<MyRoot<A, _>>| {
            // let lib = Rc::new(LibraryBase::<A, Inner> {
            //     root: w.clone(),
            //     name: lib,
            //     child: None,
            // });
            // let lib = Rc::new_cyclic(|w_lib: &Weak<LibraryBase<A, _>>| {
            //     // let inner = f(w_root.clone(), w_lib.clone());

            //     LibraryBase {
            //         root: w_root.clone(),
            //         name: lib,
            //         child: f(w_root.clone(), w_lib.clone()),
            //     }
            // });

            MyRoot {
                process_handle: *self,
                child: Rc::new_cyclic(|w_lib: &Weak<LibraryBase<A, _>>| {
                    // let inner = f(w_root.clone(), w_lib.clone());
                    LibraryBase {
                        root: w_root.clone(),
                        name: lib,
                        child: f(w_root.clone(), w_lib.clone()),
                    }
                }),
            }
        });

        // let lib = Rc::new(LibraryBase {
        //     root: Rc::downgrade(&root) as Weak<dyn Root<A>>,
        //     name: lib,
        //     child: None,
        // });

        // insert lib into root.
        // Rc::get_mut(&mut root).unwrap().child = Some(lib); // <-- error here because two weaks.

        // let inner = f(
        //     Rc::downgrade(&root) as Weak<dyn Root<A>>,
        //     Rc::downgrade(root.child.as_ref().unwrap()) as Weak<dyn Parent<A>>,
        // );

        // // insert inner into lib.
        // Rc::get_mut
        // (
        //     Rc::get_mut(&mut root).unwrap()
        //         .child.as_mut().unwrap()
        // ).unwrap()
        //     .child = Some(inner);

        root
        // ()
    }
}


fn main() -> Result<()> {
    let ph : ProcessHandle<A64Le> = ProcessHandle { _phantom: PhantomData };

    let x = ph.via_lib("lib", |root, inner| Rc::new(0));

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
