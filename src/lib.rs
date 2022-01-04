#![feature(lang_items)]
#![feature(fundamental)]
#![feature(auto_traits)]
#![feature(no_core)]
#![no_core]

mod core {
    pub mod deref {
        use crate::Sized;

        #[lang = "receiver"]
        pub trait Receiver { }

        impl<T: ?Sized> Receiver for &T {}
        impl<T: ?Sized> Receiver for &mut T {}
    }

    pub mod marker {
        #[lang = "freeze"]
        pub(crate) unsafe auto trait Freeze {}

        #[lang = "copy"]
        pub trait Copy { }

        #[lang = "sized"]
        #[fundamental]
        pub trait Sized { }
    }

    pub mod result {
        pub enum Result<T, E> {
            #[lang = "Ok"]
            Ok(T),

            #[lang = "Err"]
            Err(E),
        }

        impl<T, E> Result<T, E> {
            pub fn map_err(self, op: fn(E) -> &'static E) -> Result<T, &'static E> {
                loop { }
            }
        }
    }
}

use core::marker::Sized;
use core::result::Result;

mod std {
    #[lang = "eh_personality"]
    extern "C" fn eh_personality() {}

    pub(crate) mod io {
        pub enum Error {
            Os(i32),
            SimpleMessage(usize),
        }
    }
}

fn registry_new(mut builder: impl Sized) -> Result<(), BuildError> { loop { } }

mod tp {
    use crate::{BuildError};
    use crate::core::result::Result;

    pub struct Pool { }

    pub(super) fn thread_pool_build() -> Result<(), BuildError>
    {
        let val = crate::registry_new(());
        let registry = match val {
            Result::Ok(x) => x,
            Result::Err(y) => return Result::Err(crate::id(y)),
        };
        loop { }
    }

    impl Pool {
        pub fn new() {
            let r: Result<_, &'static BuildError> = thread_pool_build().map_err(crate::alloc_ref);
            loop { }
        }
    }
}

fn alloc_ref<T>(x: T) -> &'static T { loop { } }

fn id<T>(x: T) -> T { x }

pub struct BuildError {
    kind: std::io::Error,
}

pub trait Trait { }

pub fn build<'a>(_: &'a ()) -> Result<tp::Pool, &'a dyn Trait> { loop { } }

pub struct X;
