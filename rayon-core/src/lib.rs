#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(fundamental)]
#![feature(ptr_internals)]
#![no_std]

use alloc::boxed::Box;

mod alloc {
    pub(crate) mod boxed {
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

    pub struct Pool { }

    pub(super) fn thread_pool_build() -> Result<(), BuildError>
    {
        let registry = crate::registry_new(())?;
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
