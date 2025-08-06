use std::process::exit;
use regex::{Regex, RegexBuilder};
use crate::print_error;
use crate::package_list::{package_xz, package_list::Package};


pub fn search_package<P: AsRef<str>>(regex: P) {
    let reg: Regex = RegexBuilder::new(regex.as_ref())
        .unicode(false)
        .case_insensitive(false)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .unwrap_or_else(|e| {
            print_error!("Failed to build Regex: {}", e.to_string());
            exit(1);
        });

    let package_list = package_xz::get_package_list();

    let mut results: Vec<&Package> = package_list.package.iter()
        .filter(|package| {
            reg.is_match(&package.package) || 
                reg.is_match(&package.description)
        }).collect();

    results.sort_by(|a, b| a.package.cmp(&b.package));


    for package in results {
        println!("{}-{} ({})\n  {}",
            package.package,
            package.version,
            package.arch,
            package.description.split("\n").collect::<Vec<&str>>().first().unwrap(),
        );
    }

}


