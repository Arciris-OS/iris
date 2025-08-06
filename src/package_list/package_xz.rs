use std::path::Path;
use std::process::exit;
use std::{fs, io};

use crate::print_error;
use crate::package_list::package_list::{self, Packages};
use crate::utils::path_list;


pub fn get_package_list() -> Packages{
    let path: &Path = path_list::PACKAGE_LIST.as_ref();
    let p2: &Path = path_list::DECOMPED_PACKAGE_LIST.as_ref();

    let readed = fs::read_to_string(path_list::DECOMPED_PACKAGE_LIST)
        .unwrap_or_else(|e| {
            print_error!("Error read {}: {}", path_list::DECOMPED_PACKAGE_LIST, e.kind());
            if e.kind() == io::ErrorKind::NotFound {
                eprintln!("Please run `iris sync` and retry");
            }
            exit(1);
        });

    let parsed = package_list::parse_packages(readed);
    
    parsed
}


