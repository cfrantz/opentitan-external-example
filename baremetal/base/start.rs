use crate::Console;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    extern "Rust" {
        fn not_main(console: &mut Console);
    }
    unsafe {
        let mut console = Console::default();
        not_main(&mut console);
    }
    loop {
        // The end.
    }
}
