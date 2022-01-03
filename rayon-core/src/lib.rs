#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(fundamental)]
#![feature(ptr_internals)]
#![feature(no_core)]
#![no_core]
extern crate core;

use core::marker::Sized;
use core::convert::From;
use core::result::Result;

use alloc::boxed::Box;

mod alloc {
    pub(crate) mod boxed {
        use core::convert::From;
        use core::marker::Sized;
        use core::ptr::Unique;

        pub struct Box<T: ?Sized>(Unique<T>);

        impl<T> From<T> for Box<T> { fn from(t: T) -> Self { loop { } } }
    }
}

mod std {
    use core::panic::PanicInfo;

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }

    #[lang = "eh_personality"]
    extern "C" fn eh_personality() {}

    pub(crate) mod io {
        pub struct Error {
            repr: Repr,
        }
        enum Repr {
            Os(i32),
            SimpleMessage(i32, &'static &'static ()),
        }
    }
    pub(crate) mod error {
        pub trait Error {
        }
    }
}

fn registry_new(mut builder: impl Sized) -> Result<(), BuildError> { loop { } }

mod tp {
    use super::Box;
    use crate::{BuildError};
    use core::convert::From;
    use core::result::Result;

    pub struct Pool { }

    pub(super) fn thread_pool_build() -> Result<(), BuildError>
    {
        let registry = match crate::registry_new(()) {
            Result::Ok(x) => x,
            Result::Err(y) => return Result::Err(From::from(y)),
        };
        loop { }
    }

    impl Pool {
        pub fn new() {
            let r: Result<_, Box<BuildError>> = thread_pool_build().map_err(Box::from);
            loop { }
        }
    }
}

pub struct BuildError {
    kind: std::io::Error,
}

pub fn build<'a>(_: &'a ()) -> Result<tp::Pool, &'a dyn std::error::Error> { loop { } }
