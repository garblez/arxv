use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;
use tar::Archive;

use std::env;

fn main() -> Result<(), std::io::Error> {
    for (key, arg) in env::args().into_iter().enumerate() {
        print!("{}: {}, ", key, arg);
    }
    println!("Hello, world!");

    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(".", "./src")?;

    Ok(())
}
