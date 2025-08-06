use std::fs;
use serde::{Deserialize, Serialize};
use crate::utils::path_list::*;
use crate::print_error;


#[derive(Deserialize, Serialize)]
pub struct MirrorList{
    pub mirrors: Vec<Mirror>,
}

#[derive(Deserialize, Serialize)]
pub struct Mirror {
    pub url: String,
}

fn read_mirror_list() -> String {
    fs::read_to_string(MIRROR_LIST).unwrap_or_else(|e| {
        print_error!("Failed to read mirror list({}). Please check {} is exists", e.kind(), MIRROR_LIST);
        std::process::exit(1);
    })
}

fn parse_mirrorlist<P: AsRef<str>>(mirror_list: P) -> MirrorList{
    let parsed: MirrorList = toml::from_str(mirror_list.as_ref()).unwrap_or_else(|e| {
        print_error!("Error while parsing mirror list: \n{}", e.message());
        eprintln!("Please fix mirror list and retry");
        eprintln!("Mirror list path: {}", MIRROR_LIST);
        std::process::exit(1);
    });

    parsed
}


pub fn get_mirrors() -> MirrorList{
    let mirror_list_str = read_mirror_list();
    parse_mirrorlist(mirror_list_str)
}


