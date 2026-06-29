#![no_std]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(c_variadic)]
#![feature(thread_local)]
#![allow(clippy::missing_safety_doc)]

pub mod errno;
pub mod mem;
pub mod stdio;

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    core::intrinsics::abort()
}
