// use scraper::{Html, Selector};
// use std::fs::{self, File};
// use std::io::prelude::*;
// use std::io::Write;
// use std::path::Path;

// #[tokio::main]
fn main() {
    let qunhui_tutorial_url = "http://1688.siyouyun.ren:6969/";
    let response = reqwest::blocking::get(qunhui_tutorial_url)
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&response);
    let link_selector = scraper::Selector::parse("a").unwrap();
    let links = document.select(&link_selector);
    links.for_each(|item| println!("{:#?}", item));

    // let resp = reqwest::get(qunhui_tutorial_url).await?;
    // let body = resp.text().await?;
    // let doc = Html::parse_fragment(&body);
    // let link_selector = Selector::parse("a").unwrap();

    // for node in doc.select(&link_selector) {
    //     println!("title:{}", node.inner_html());

    //     let node_value = node.value();
    //     let href = node_value.attr("href").unwrap_or("");
    //     // let url_wrapped = reqwest::Url::parse(href);
    //     // let url = url_wrapped.unwrap();
    //     // let filename = url.path_segments().unwrap().last().unwrap_or("");
    //     let filename = href;

    //     if !filename.is_empty() {
    //         let file_url = format!("{}{}", qunhui_tutorial_url, filename);
    //         let file_res = reqwest::get(file_url).await?;
    //         let mut file = std::fs::File::create(filename).unwrap();
    //         // file.w
    //         let file_bytes = file_res.bytes().await?;
    //         file.write_all(&file_bytes).unwrap();
    //     }
    //     // println!("title:{}", el.inner_html());
    // }

    // Ok(())
}

// async fn download_url(url: &str, base_path: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let response = reqwest::get(url).await?;
//     let body = response.text().await?;
//     let document = Html::parse_document(&body);

//     let link_selector = Selector::parse("a").unwrap();
//     let href_attr = "href";

//     for node in document.select(&link_selector) {
//         if let Some(href) = node.value().attr(href_attr) {
//             let url = Url::parse(&href)?;
//             let filename = url.path_segments().unwrap().last().unwrap_or("");

//             if !filename.is_empty() {
//                 let file_url = format!("{}{}", url, href);
//                 let file_path = Path::new(base_path).join(filename);
//                 if !file_path.exists() {
//                     let response = reqwest::get(&file_url).await?;
//                     let mut file = File::create(&file_path)?;
//                     let mut content = response.bytes().await?;
//                     file.write_all(&mut content)?;
//                 }
//             } else if url.path().ends_with('/') {
//                 let dir_url = format!("{}{}", url, href);
//                 let dir_path = Path::new(base_path).join(href);
//                 if !dir_path.exists() {
//                     fs::create_dir(&dir_path)?;
//                     download_url(&dir_url, &dir_path.into_os_string().to_str().unwrap()).await?;
//                 }
//             }
//         }
//     }

//     Ok(())
// }
