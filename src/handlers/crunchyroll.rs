use rss::Channel;
use std::collections::HashSet;
use std::convert::Infallible;
use std::error::Error;

async fn anime_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://www.crunchyroll.com/rss/anime")
        .await?
        .bytes()
        .await?;

    let channel = Channel::read_from(&content[..]).unwrap();
    Ok(channel)
}

pub async fn feed_titles() -> Result<impl warp::Reply, Infallible> {
    //TODO: figure out how to return errors

    let channel = anime_feed().await.unwrap();
    let series_title_list: HashSet<String> = channel
        .items
        .iter()
        .map(|item| {
            let mut result = "No Title Found".to_string();

            let crunchyroll = item.extensions["crunchyroll"].clone();

            if let Some(title) = crunchyroll.get("seriesTitle") {
                result = title[0].value.clone().unwrap()
            }

            result
        })
        .collect();

    Ok(warp::reply::json(&series_title_list))
}
