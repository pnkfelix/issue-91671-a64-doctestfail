#![feature(start)]
#![no_std]

#[allow(unused_imports)] use a64_doctestfail;

// fn main() { }


#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    loop { }
}
