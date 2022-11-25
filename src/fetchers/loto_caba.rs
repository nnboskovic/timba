use chromiumoxide::browser::{Browser, BrowserConfig};
use tokio_stream::StreamExt;
use crate::types::LotoResult;

/// Scrape the XML download link from the website
/// Sadly I need to use a headless chromium instance because lmao js
async fn get_loto_xml_download_link() -> Result<String, Box<dyn std::error::Error>> {
    let base_page = "https://loto.loteriadelaciudad.gob.ar";

    let (mut browser, mut handler) = Browser::launch(BrowserConfig::builder().build()?).await?;

    let handle = tokio::task::spawn(async move {
        loop {
            match handler.next().await {
                Some(h) => match h {
                    Ok(_) => continue,
                    Err(_) => break,
                },
                None => break,
            }
        }
    });

    let page = browser.new_page(base_page).await?;

    // doesn't properly load the page otherwise
    // TODO: less ugly option
    std::thread::sleep(std::time::Duration::from_secs(2));

    let link = page
        .find_element(r#"a[href$=".xml"]"#)
        .await?
        .attribute("href")
        .await?
        .unwrap();

    let dl_link = format!("{}/{}", base_page, link);

    browser.close().await?;
    handle.await.expect("Panic! at the Chromium");

    Ok(dl_link)
}

/// Fetch results from loteriadelaciudad.gob.ar's XML
pub async fn scrape_loto_results() -> Result<LotoResult, Box<dyn std::error::Error>> {
    let dl_link = get_loto_xml_download_link().await?;

    let xml_response = reqwest::get(dl_link).await?.text().await?;

    let doc = roxmltree::Document::parse(&xml_response).unwrap();

    for node in doc.descendants() {
        if node.is_element() {
            println!(
                "{:?} at {}",
                node.tag_name(),
                doc.text_pos_at(node.position())
            );
        }
    }

    // TODO: parse and return the actual value
    Ok(LotoResult::default())
}