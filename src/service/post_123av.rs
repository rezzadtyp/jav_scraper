use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::json;
use std::sync::Arc;

pub async fn fetch_movie_details(
    url: &str,
    client: &Arc<Client>,
) -> Result<serde_json::Value, String> {
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|_| "Failed to fetch the URL")?;
    let body = response
        .text()
        .await
        .map_err(|_| "Failed to read the response body")?;

    let document = Html::parse_document(&body);
    let selector =
        Selector::parse("div.container#page-video").map_err(|_| "Invalid CSS selector")?;

    if let Some(element) = document.select(&selector).next() {
        if let Some(attr) = element.value().attr("v-scope") {
            let id = attr
                .split("id: ")
                .nth(1)
                .and_then(|s| s.split(",").next())
                .map(|s| s.trim_matches('{').trim());
            if let Some(movie_id) = id {
                let plyr_url = format!("https://123av.com/en/ajax/v/{}/videos", movie_id);
                let fetch_plyr = client.get(&plyr_url).send().await.map_err(|_| format!("Failed to fetch {}", plyr_url))?;
                let api_response = fetch_plyr.json::<serde_json::Value>().await.map_err(|_| "Failed to get the response")?;
                if let Some(watch_urls) = api_response["data"]["watch"].as_array() {
                    let urls: Vec<String> = watch_urls.iter().filter_map(|item| item["url"].as_str().map(String::from)).collect();
                    return Ok(json!({"status": 200, "id": movie_id, "watch_urls": urls }));
                }
            }
        }
    }
    Err("Movie details not found".into())
}
