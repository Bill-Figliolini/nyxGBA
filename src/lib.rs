use crate::gba::Gba;

mod gba;

#[cfg(target_pointer_width = "16")]
compile_error!("Only 32-bit and 64-bit architectures supported");
pub fn nyx_main() {
    let mut gba = Gba::new();
    gba.run();
    gba.reset();
}

#[allow(clippy::print_stderr, reason = "temporary until better logging exists")]
pub(crate) fn log_error(string: &str) {
    eprintln!("{string}");
}
