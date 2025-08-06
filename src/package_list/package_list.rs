use std::process::exit;

use serde::{Deserialize, Serialize};

use crate::print_error;

#[derive(Deserialize, Serialize)]
pub struct Packages{
    pub package: Vec<Package>
}

#[derive(Deserialize, Serialize)]
pub struct Package {
    pub package: String,
    pub version: String,
    pub arch: String,
    pub maintainer: String,
    pub filename: String,
    pub dependencies: Vec<String>,
    pub size: u64,
    pub sha256: String,
    pub sha1: String,
    pub md5sum: String,
    pub description: String,
}



pub fn parse_packages<T: AsRef<str>>(parse: T) -> Packages{
    toml::from_str(parse.as_ref()).unwrap_or_else(|e| {
        print_error!("Error while parsing Packages.xz:\n{e}");
        print_error!("Run `iris sync` to redownload Packages.xz");
        exit(1);
    })
}


