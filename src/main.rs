use rust_gba::cartridge::Cartridge;
use std::env;
fn main() {
    env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    let cartridge = Cartridge::new("RAREDKC2.GBA");
}
