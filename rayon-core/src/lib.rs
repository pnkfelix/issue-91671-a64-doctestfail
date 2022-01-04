#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(fundamental)]
#![feature(rustc_attrs)]
#![feature(negative_impls)]
#![feature(auto_traits)]
#![feature(extern_types)]
#![feature(const_trait_impl)]
#![feature(unboxed_closures)]
#![feature(no_core)]
#![no_core]

mod core {
    pub mod deref {
        use crate::Sized;

        #[lang = "deref"]
        pub trait Deref {
            #[lang = "deref_target"]
            type Target: ?Sized;

            fn deref(&self) -> &Self::Target;
        }

        impl<T: ?Sized> const Deref for &T {
            type Target = T;

            fn deref(&self) -> &T {
                loop { }
            }
        }

        impl<T: ?Sized> !DerefMut for &T {}

        impl<T: ?Sized> Deref for &mut T {
            type Target = T;

            fn deref(&self) -> &T {
                loop { }
            }
        }

        #[lang = "deref_mut"]
        pub trait DerefMut: Deref {
            fn deref_mut(&mut self) -> &mut Self::Target;
        }

        impl<T: ?Sized> DerefMut for &mut T {
            fn deref_mut(&mut self) -> &mut T {
                loop { }
            }
        }

        #[lang = "receiver"]
        pub trait Receiver { }

        impl<T: ?Sized> Receiver for &T {}

        impl<T: ?Sized> Receiver for &mut T {}
    }

    pub mod marker {
        #[lang = "freeze"]
        pub(crate) unsafe auto trait Freeze {}

        #[lang = "copy"]
        #[rustc_unsafe_specialization_marker]
        pub trait Copy { }

        #[lang = "sized"]
        #[fundamental]
        #[rustc_specialization_trait]
        pub trait Sized { }
    }
    pub mod ops {
        pub mod function {
            #[lang = "fn"]
            #[rustc_paren_sugar]
            #[fundamental]
            pub trait Fn<Args>: FnMut<Args> {
                extern "rust-call" fn call(&self, args: Args) -> Self::Output;
            }

            #[lang = "fn_mut"]
            #[rustc_paren_sugar]
            #[fundamental]
            pub trait FnMut<Args>: FnOnce<Args> {
                extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
            }

            #[lang = "fn_once"]
            #[rustc_paren_sugar]
            #[fundamental]
            pub trait FnOnce<Args> {
                #[lang = "fn_once_output"]
                type Output;

                extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
            }
        }
    }
    pub mod panic {
        #[lang = "panic_info"]
        pub struct PanicInfo;
    }
    pub mod ptr {
        use crate::core::marker::Sized;

        #[repr(transparent)]
        #[rustc_layout_scalar_valid_range_start(1)]
        pub struct Unique<T: ?Sized> {
            pointer: *const T,
        }

        #[lang = "drop_in_place"]
        #[allow(unconditional_recursion)]
        pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
            unsafe { drop_in_place(to_drop) }
        }
    }

    pub mod result {
        use crate::core::ops::function::FnOnce;

        pub enum Result<T, E> {
            #[lang = "Ok"]
            Ok(T),

            #[lang = "Err"]
            Err(E),
        }

        impl<T, E> Result<T, E> {
            pub fn map_err<F, O: FnOnce(E) -> F>(self, op: O) -> Result<T, F> {
                loop { }
            }
        }
    }
}

use core::marker::Sized;
use core::result::Result;

mod std {
    use crate::core::panic::PanicInfo;

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }

    #[lang = "eh_personality"]
    extern "C" fn eh_personality() {}

    pub(crate) mod io {
        pub enum Error {
            Os(i32),
            SimpleMessage(usize),
        }
    }
    pub(crate) mod error {
    }
}

pub trait Trait { }

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
            Result::Err(y) => return Result::Err(::id(y)),
        };
        loop { }
    }

    impl Pool {
        pub fn new() {
            let r: Result<_, &'static BuildError> = thread_pool_build().map_err(::alloc_ref);
            loop { }
        }
    }
}

fn alloc_ref<T>(x: T) -> &'static T { loop { } }

fn id<T>(x: T) -> T { x }

pub struct BuildError {
    kind: std::io::Error,
}

pub fn build<'a>(_: &'a ()) -> Result<tp::Pool, &'a dyn Trait> { loop { } }
