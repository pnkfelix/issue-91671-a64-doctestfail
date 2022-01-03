#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![no_std]

extern crate alloc;
use alloc::boxed::Box;

mod std {
    use core::panic::PanicInfo;

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }

    #[lang = "eh_personality"]
    extern "C" fn eh_personality() {}

    struct FakeAllocator;

    #[global_allocator]
    static FAKE_ALLOC: FakeAllocator = FakeAllocator;

    use alloc::alloc::{GlobalAlloc, Layout};

    unsafe impl GlobalAlloc for FakeAllocator {
        unsafe fn alloc(&self, _: Layout) -> *mut u8 { loop { } }
        unsafe fn dealloc(&self, _: *mut u8, _: Layout) { loop { } }
    }

    #[alloc_error_handler]
    fn my_example_handler(layout: core::alloc::Layout) -> ! {
        loop { }
    }

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
