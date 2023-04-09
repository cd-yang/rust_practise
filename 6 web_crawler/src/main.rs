// use std::fs::{self, File};
// use std::io::prelude::*;
// use std::io::Write;
// use std::path::Path;

// #[tokio::main]
fn main() {
    let url = "http://1688.siyouyun.ren:6969/";
    download_folder(url, "./downloadFolder/")
}

fn download_folder(url: &str, download_path: &str) {
    ensure_folder_exists(download_path);

    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);
    let link_selector = scraper::Selector::parse("a").unwrap();
    let links = document.select(&link_selector);
    links.for_each(|item| {
        let href = item.value().attr("href").unwrap();
        if href != "/" {
            let href = item.value().attr("href").unwrap();
            let full_href = format!("{}{}", url, href);
            if href.ends_with("/") {
                download_folder(
                    full_href.as_str(),
                    format!("{}{}", download_path, href).as_str(),
                )
            } else {
                let file_name = item.inner_html();
                download_file(full_href.as_str(), file_name, download_path)
            }
        }
    });
}

fn ensure_folder_exists(download_path: &str) {
    println!("making folder:{}", download_path)
}

fn download_file(href: &str, file_name: String, download_path: &str) {
    println!(
        "downloading file:{} from:{} to folder:{}",
        file_name, href, download_path
    )
}
