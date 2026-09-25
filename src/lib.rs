use crate::gba::Gba;

mod gba;

#[cfg(target_pointer_width = "16")]
compile_error!("Only 32-bit and 64-bit architectures supported");
pub fn nyx_main() {
    let mut gba = Gba::startup();
    gba.run();
}
