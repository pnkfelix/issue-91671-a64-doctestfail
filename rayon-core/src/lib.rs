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
    pub mod any {
        pub struct TypeId {
            t: u64,
        }

        pub trait Any: 'static {
            fn type_id(&self) -> TypeId;
        }
    }
    pub mod clone {
        use crate::Sized;

        #[lang = "clone"]
        #[rustc_diagnostic_item = "Clone"]
        pub trait Clone: Sized {
            #[must_use = "cloning is often expensive and is not expected to have side effects"]
            fn clone(&self) -> Self;

            #[inline]
            fn clone_from(&mut self, source: &Self) {
                *self = source.clone()
            }
        }
    }
    pub mod convert {
        use crate::core::marker::Sized;

        pub trait From<T>: Sized {
            #[lang = "from"]
            #[must_use]
            fn from(_: T) -> Self;
        }

        impl<T> const From<T> for T {
            fn from(t: T) -> T {
                t
            }
        }

        pub trait Into<T>: Sized {
            fn into(self) -> T;
        }
    }
    pub mod deref {
        use crate::Sized;

        #[lang = "deref"]
        #[rustc_diagnostic_item = "Deref"]
        pub trait Deref {
            #[rustc_diagnostic_item = "deref_target"]
            #[lang = "deref_target"]
            type Target: ?Sized;

            #[must_use]
            #[rustc_diagnostic_item = "deref_method"]
            fn deref(&self) -> &Self::Target;
        }

        impl<T: ?Sized> const Deref for &T {
            type Target = T;

            #[rustc_diagnostic_item = "noop_method_deref"]
            fn deref(&self) -> &T {
                *self
            }
        }

        impl<T: ?Sized> !DerefMut for &T {}

        impl<T: ?Sized> /*const*/ Deref for &mut T {
            type Target = T;

            fn deref(&self) -> &T {
                *self
            }
        }

        #[lang = "deref_mut"]
        pub trait DerefMut: Deref {
            fn deref_mut(&mut self) -> &mut Self::Target;
        }

        impl<T: ?Sized> DerefMut for &mut T {
            fn deref_mut(&mut self) -> &mut T {
                *self
            }
        }

        #[lang = "receiver"]
        pub trait Receiver {
            // Empty.
        }

        impl<T: ?Sized> Receiver for &T {}

        impl<T: ?Sized> Receiver for &mut T {}
    }
    pub mod fmt {
        use crate::core::option::Option;

        pub type Result = crate::core::result::Result<(), Error>;
        pub struct Error;

        pub struct Formatter<'a> {
            flags: u32,
            fill: char,
            align: rt::v1::Alignment,
            width: Option<usize>,
            precision: Option<usize>,

            buf: &'a mut (dyn Write + 'a),
        }

        pub trait Write {
            fn write_str(&mut self, s: &str) -> Result;
            fn write_char(&mut self, c: char) -> Result {
                loop { } // self.write_str(c.encode_utf8(&mut [0; 4]))
            }
            fn write_fmt(mut self: &mut Self, args: Arguments<'_>) -> Result {
                loop { } // write(&mut self, args)
            }
        }

        impl<W: Write + ?crate::Sized> Write for &mut W {
            fn write_str(&mut self, s: &str) -> Result {
                (**self).write_str(s)
            }

            fn write_char(&mut self, c: char) -> Result {
                (**self).write_char(c)
            }

            fn write_fmt(&mut self, args: Arguments<'_>) -> Result {
                (**self).write_fmt(args)
            }
        }

        extern "C" {
            type Opaque;
        }

        pub struct ArgumentV1<'a> {
            value: &'a Opaque,
            formatter: fn(&Opaque, &mut Formatter<'_>) -> Result,
        }

        pub struct Arguments<'a> {
            pieces: &'a [&'static str],
            fmt: Option<&'a [rt::v1::Argument]>,
            args: &'a [ArgumentV1<'a>],
        }

        pub mod rt {
            pub mod v1 {
                pub use self::Argument as ArgumentV1;

                pub struct Argument {
                    pub position: usize,
                    pub format: FormatSpec,
                }

                pub struct FormatSpec {
                    pub fill: char,
                    pub align: Alignment,
                    pub flags: u32,
                    pub precision: Count,
                    pub width: Count,
                }

                /// Possible alignments that can be requested as part of a formatting directive.
                pub enum Alignment {
                    /// Indication that contents should be left-aligned.
                    Left,
                    /// Indication that contents should be right-aligned.
                    Right,
                    /// Indication that contents should be center-aligned.
                    Center,
                    /// No alignment was requested.
                    Unknown,
                }

                /// Used by [width](https://doc.rust-lang.org/std/fmt/#width) and [precision](https://doc.rust-lang.org/std/fmt/#precision) specifiers.
                pub enum Count {
                    /// Specified with a literal number, stores the value
                    Is(usize),
                    /// Specified using `$` and `*` syntaxes, stores the index into `args`
                    Param(usize),
                    /// Not specified
                    Implied,
                }
            }
        }
    }

    pub mod marker {
        use crate::core::clone::Clone;

        #[lang = "freeze"]
        pub(crate) unsafe auto trait Freeze {}

        // impl<T: ?Sized> !Freeze for UnsafeCell<T> {}
        unsafe impl<T: ?Sized> Freeze for PhantomData<T> {}
        unsafe impl<T: ?Sized> Freeze for *const T {}
        unsafe impl<T: ?Sized> Freeze for *mut T {}
        unsafe impl<T: ?Sized> Freeze for &T {}
        unsafe impl<T: ?Sized> Freeze for &mut T {}

        #[lang = "copy"]
        // FIXME(matthewjasper) This allows copying a type that doesn't implement
        // `Copy` because of unsatisfied lifetime bounds (copying `A<'_>` when only
        // `A<'static>: Copy` and `A<'_>: Clone`).
        // We have this attribute here for now only because there are quite a few
        // existing specializations on `Copy` that already exist in the standard
        // library, and there's no way to safely have this behavior right now.
        #[rustc_unsafe_specialization_marker]
        #[rustc_diagnostic_item = "Copy"]
        pub trait Copy: Clone {
            // Empty.
        }

        #[lang = "unsize"]
        pub trait Unsize<T: ?Sized> { }

        pub unsafe auto trait Send { }

        impl<T: ?Sized> !Send for *const T {}
        impl<T: ?Sized> !Send for *mut T {}

        #[lang = "sync"]
        pub unsafe auto trait Sync { }

        impl<T: ?Sized> !Sync for *const T {}
        impl<T: ?Sized> !Sync for *mut T {}

        #[lang = "sized"]
        #[rustc_on_unimplemented(
            message = "the size for values of type `{Self}` cannot be known at compilation time",
            label = "doesn't have a size known at compile-time"
        )]
        #[fundamental] // for Default, for example, which requires that `[T]: !Default` be evaluatable
        #[rustc_specialization_trait]
        pub trait Sized {
            // Empty.
        }

        #[lang = "phantom_data"]
        pub struct PhantomData<T: ?Sized>;
    }
    pub mod ops {
        pub mod function {
            #[lang = "fn"]
            #[rustc_paren_sugar]
            #[fundamental] // so that regex can rely that `&str: !FnMut`
            #[must_use = "closures are lazy and do nothing unless called"]
            pub trait Fn<Args>: FnMut<Args> {
                /// Performs the call operation.
                extern "rust-call" fn call(&self, args: Args) -> Self::Output;
            }

            #[lang = "fn_mut"]
            #[rustc_paren_sugar]
            #[rustc_on_unimplemented(
                on(
                    Args = "()",
                    note = "wrap the `{Self}` in a closure with no arguments: `|| {{ /* code */ }}`"
                ),
                message = "expected a `{FnMut}<{Args}>` closure, found `{Self}`",
                label = "expected an `FnMut<{Args}>` closure, found `{Self}`"
            )]
            #[fundamental] // so that regex can rely that `&str: !FnMut`
            #[must_use = "closures are lazy and do nothing unless called"]
            pub trait FnMut<Args>: FnOnce<Args> {
                /// Performs the call operation.
                extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
            }

            #[lang = "fn_once"]
            #[rustc_paren_sugar]
            #[rustc_on_unimplemented(
                on(
                    Args = "()",
                    note = "wrap the `{Self}` in a closure with no arguments: `|| {{ /* code */ }}`"
                ),
                message = "expected a `{FnOnce}<{Args}>` closure, found `{Self}`",
                label = "expected an `FnOnce<{Args}>` closure, found `{Self}`"
            )]
            #[fundamental] // so that regex can rely that `&str: !FnMut`
            #[must_use = "closures are lazy and do nothing unless called"]
            pub trait FnOnce<Args> {
                /// The returned type after the call operator is used.
                #[lang = "fn_once_output"]
                type Output;

                /// Performs the call operation.
                extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
            }

            mod impls {
                use crate::core::ops::function::{Fn, FnMut, FnOnce};
                use crate::Sized;

                impl<A, F: ?Sized> Fn<A> for &F
                where
                    F: Fn<A>,
                {
                    extern "rust-call" fn call(&self, args: A) -> F::Output {
                        (**self).call(args)
                    }
                }

                impl<A, F: ?Sized> FnMut<A> for &F
                where
                    F: Fn<A>,
                {
                    extern "rust-call" fn call_mut(&mut self, args: A) -> F::Output {
                        (**self).call(args)
                    }
                }

                impl<A, F: ?Sized> FnOnce<A> for &F
                where
                    F: Fn<A>,
                {
                    type Output = F::Output;

                    extern "rust-call" fn call_once(self, args: A) -> F::Output {
                        (*self).call(args)
                    }
                }

                impl<A, F: ?Sized> FnMut<A> for &mut F
                where
                    F: FnMut<A>,
                {
                    extern "rust-call" fn call_mut(&mut self, args: A) -> F::Output {
                        (*self).call_mut(args)
                    }
                }

                impl<A, F: ?Sized> FnOnce<A> for &mut F
                where
                    F: FnMut<A>,
                {
                    type Output = F::Output;
                    extern "rust-call" fn call_once(self, args: A) -> F::Output {
                        (*self).call_mut(args)
                    }
                }
            }
        }
        pub mod unsize {
            use crate::Sized;
            use crate::core::marker::Unsize;

            #[lang = "coerce_unsized"]
            pub trait CoerceUnsized<T: ?crate::Sized> {
            }

            // &mut T -> &mut U
            impl<'a, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<&'a mut U> for &'a mut T {}
            // &mut T -> &U
            impl<'a, 'b: 'a, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<&'a U> for &'b mut T {}
            // &mut T -> *mut U
            impl<'a, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<*mut U> for &'a mut T {}
            // &mut T -> *const U
            impl<'a, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for &'a mut T {}
            // &T -> &U
            impl<'a, 'b: 'a, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<&'a U> for &'b T {}
            // &T -> *const U
            impl<'a, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for &'a T {}
            // *mut T -> *mut U
            impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<*mut U> for *mut T {}
            // *mut T -> *const U
            impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for *mut T {}
            // *const T -> *const U
            impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for *const T {}

            #[lang = "dispatch_from_dyn"]
            pub trait DispatchFromDyn<T> {
                // Empty.
            }

            // &T -> &U
            impl<'a, T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<&'a U> for &'a T {}
            // &mut T -> &mut U
            impl<'a, T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<&'a mut U> for &'a mut T {}
            // *const T -> *const U
            impl<T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<*const U> for *const T {}
            // *mut T -> *mut U
            impl<T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<*mut U> for *mut T {}
        }
    }
    pub mod option {
        pub enum Option<T> {
            #[lang = "None"]
            None,
            #[lang = "Some"]
            Some(T),
        }
    }
    pub mod panic {
        use crate::core::option::Option;

        #[lang = "panic_info"]
        pub struct PanicInfo<'a> {
            payload: &'a (dyn crate::core::any::Any + crate::core::marker::Send),
            message: Option<&'a crate::core::fmt::Arguments<'a>>,
            location: &'a Location<'a>,
        }

        #[lang = "panic_location"]
        pub struct Location<'a> {
            file: &'a str,
            line: u32,
            col: u32,
        }
    }
    pub mod ptr {
        use crate::core::marker::Sized;

        #[repr(transparent)]
        #[rustc_layout_scalar_valid_range_start(1)]
        pub struct Unique<T: ?Sized> {
            pointer: *const T,
            _marker: crate::core::marker::PhantomData<T>,
        }

        #[lang = "drop_in_place"]
        #[allow(unconditional_recursion)]
        pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
            unsafe { drop_in_place(to_drop) }
        }
    }

    pub mod result {
        use crate::core::ops::function::FnOnce;

        #[must_use = "this `Result` may be an `Err` variant, which should be handled"]
        #[rustc_diagnostic_item = "Result"]
        pub enum Result<T, E> {
            #[lang = "Ok"]
            Ok(T),

            #[lang = "Err"]
            Err(E),
        }

        impl<T, E> Result<T, E> {
            pub fn map_err<F, O: FnOnce(E) -> F>(self, op: O) -> Result<T, F> {
                match self {
                    Result::Ok(t) => Result::Ok(t),
                    Result::Err(e) => Result::Err(op(e)),
                }
            }

        }
    }
}

use core::marker::Sized;
use core::convert::From;
use core::result::Result;

use alloc::boxed::Box;

mod alloc {
    pub(crate) mod boxed {
        use crate::core::convert::From;
        use crate::core::marker::Sized;
        use crate::core::ptr::Unique;

        pub struct Box<T: ?Sized>(Unique<T>);

        impl<T> From<T> for Box<T> { fn from(t: T) -> Self { loop { } } }
    }
}

mod std {
    use crate::core::panic::PanicInfo;

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
    use crate::core::convert::From;
    use crate::core::result::Result;

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
