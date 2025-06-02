// unsafe extern "C" {
//     fn main(argc: isize, argv: *const *const u8) -> i32;
// }

#[unsafe(no_mangle)]
pub extern "C" fn _start(_argc: isize, _argv: *const *const u8) -> ! {
    // let result = unsafe { main(argc, argv) };

    // unsafe {
    //     crate::sys::thread_local::destructors::run();
    // }
    // crate::rt::thread_cleanup();

    crate::sys::os::exit(0);
}
