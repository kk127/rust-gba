use log::info;
use std::fs;
use std::path::Path;

pub struct Cartridge {
    rom: Vec<u8>,
}

impl Cartridge {
    pub fn new(cartridge_name: &str) -> Self {
        let path = Path::new("cartridges").join(cartridge_name);
        info!("Reading {:?} file...", path);
        let rom = fs::read(&path).expect("Error while reading ROM file");
        info!("Finish reading {:?} file", path);

        let game_title = rom[0xa0..=0xab]
            .iter()
            .filter(|&x| *x != 0)
            .map(|&x| x as char)
            .collect::<String>();

        info!("Game title: {}", game_title);

        let checksum = rom[0xa0..=0xbc]
            .iter()
            .cloned()
            .fold(0u8, u8::wrapping_sub)
            .wrapping_sub(0x19);

        if checksum != rom[0xbd] {
            panic!(
                "Incorrect checksum: rom[0xbd] = {}, checksum = {}",
                rom[0xbd], checksum
            );
        }

        Cartridge { rom }
    }
}
