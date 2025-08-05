use core::error;
use std::{fs, io::{Read, Write}, path::Path};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking;




pub fn download_file<T: AsRef<str>, P: AsRef<Path>>(url: T, path: P) -> Result<(), Box<dyn error::Error>>{
    // 実はこれ実装するの人生で3回目ぐらい

    let path = path.as_ref();
    let url = url.as_ref();
    let client = blocking::Client::new();

    let response = client.head(url).send()?;
    let total_size: u64 = response
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|len| len.to_str().ok())
        .and_then(|len| len.parse().ok())
        .unwrap_or_default();

    let progress_bar = ProgressBar::new(total_size);

    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:23}]\t{msg}\n{bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"),
    );

    progress_bar.set_position(0);
    let mut downloaded: u64 = 0;
    let mut buffer = vec![0; 8192];
    let mut response = client.get(url).send()?;
    let mut dest = fs::File::create(path)?;
    let file_name = get_filename(&response).unwrap_or(
        format!("{}", path.file_name().unwrap().to_str().unwrap().to_string())
    );
    progress_bar.set_message(file_name);

    while let Ok(n) = response.read(&mut buffer) {
        if n == 0 {
            break;
        }

        dest.write_all(&buffer[..n])?;
        downloaded += n as u64;

        progress_bar.set_position(downloaded);
    }

    
    dest.flush()?;
    progress_bar.finish();

    Ok(())
}



fn get_filename(response: &blocking::Response) -> Option<String> {

    // まずContent-Dispositionヘッダーを確認
    let filename = if let Some(content_disposition) = response.headers().get("content-disposition") {
        let disposition_str = content_disposition.to_str().unwrap();
        extract_filename_from_disposition(disposition_str)
    } else {
        None
    };


    return filename;
}


fn extract_filename_from_disposition(disposition: &str) -> Option<String> {
    // filename="example.pdf" のような形式を処理
    if let Some(start) = disposition.find("filename=") {
        let filename_part = &disposition[start + 9..];
        if let Some(end) = filename_part.find(';') {
            return Some(filename_part[..end].trim_matches('"').to_string());
        } else {
            return Some(filename_part.trim_matches('"').to_string());
        }
    }
    None
}
