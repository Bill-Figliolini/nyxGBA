fn main() {
    #[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
    compile_error!("Only 32-bit and 64-bit desktop architectures supported");
    nyxgba::nyx_main();
}
