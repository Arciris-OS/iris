use std::process::exit;
use crate::utils::{self, download, path_list, mirror};
use crate::{print_error, Verbose};

/// Get a sync list using mirror list
pub fn get_synclist() -> Vec<String> {
    let mirror_list = mirror::get_mirrors();

    let mut sync_list: Vec<String> = Vec::new();

    for mirror in mirror_list.mirrors {
        sync_list.push(format!("{}/Packages.xz", mirror.url));
    }

    sync_list
}

pub fn sync_package_list(output: Verbose){

    println!("Updating iris repositories...");

    let sync_list = get_synclist();
    let count = sync_list.iter().count();

    for (current, url) in sync_list.iter().enumerate() {
        println!("Downloading {}...", url);
        if let Err(e) = download::download_file_sync(url, path_list::PACKAGE_LIST){

            if (current + 1) == count {
                print_error!("Failed to download Package.xz from {url}: {}", e.to_string());
                print_error!("Are you connected to the Internet?");
                exit(1);
            }

            print_error!("Failed to download Package.xz from {url}: {}", e.to_string());
            print_error!("Trying an alternative mirror...");
            continue;
        };
    }
    let _ = utils::decomp::decomp_xz(path_list::PACKAGE_LIST, path_list::DECOMPED_PACKAGE_LIST);
}




