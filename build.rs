use std::{collections::HashSet, env, fs, io::Write, path::Path};

use mixlib::MixFilename;

fn main() {
    println!("cargo::rerun-if-changed=td.csv");
    println!("cargo::rerun-if-changed=build.rs");

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .from_path("td.csv")
        .unwrap();
    let iter = reader.records();
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("crcs.rs");
    let mut w = fs::File::create(&dest_path).unwrap();
    writeln!(&mut w, "pub mod known_files {{").unwrap();
    writeln!(&mut w, "  use mixlib::MixFilename;").unwrap();

    let mut written: HashSet<String> = HashSet::new();

    for result in iter.map(|result| result.unwrap()) {
        let crc: u32 = result.get(0).unwrap().parse().unwrap();
        let filename = result.get(1).unwrap();
        let mf = MixFilename::new(filename);
        if mf.crc != crc {
            panic!(
                "CRC value for {filename} seems to diverge: {crc} <-> {}",
                mf.crc
            );
        }
        if written.insert(filename.to_string()) {
            let mut symbol = filename.replace(".", "_").replace("-", "_");
            if symbol.starts_with(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']) {
                symbol = symbol
                    .replacen("1", "ONE_", 1)
                    .replacen("2", "TWO_", 1)
                    .replacen("3", "THREE_", 1)
                    .replacen("4", "FOUR_", 1)
                    .replacen("5", "FIVE_", 1)
                    .replacen("6", "SIX_", 1)
                    .replacen("8", "EIGHT_", 1)
                    .replacen("12", "TWELVE_", 1)
                    .replacen("40", "FOURTY_", 1)
                    .replacen("50", "FIFTY_", 1);
            }
            writeln!(
                &mut w,
                "  pub const {symbol}: MixFilename = MixFilename::new(\"{filename}\");"
            )
            .unwrap();
        }
    }
    writeln!(&mut w, "}}").unwrap();
}
