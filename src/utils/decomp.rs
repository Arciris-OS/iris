use std::{error::Error, fs::File, path::Path};
use xz::write::XzDecoder;
use xz::read;
use tar::Archive;


pub fn decomp_xz<P: AsRef<Path>>(path: P, output: P) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref();
    let output = output.as_ref();

    let file = File::open(path)?;
    let mut decoder = read::XzDecoder::new(file);
    let mut output = File::create(output)?;

    std::io::copy(&mut decoder, &mut output)?;
    Ok(())
}


pub fn decomp_tar_xz<P: AsRef<Path>>(path: P, out_dir: P) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;

    let decoder = XzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    archive.unpack(out_dir)?;

    Ok(())
}



