use rss::Channel;
use std::collections::HashSet;
use std::convert::Infallible;
use std::error::Error;

async fn shows_rss_info_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://showrss.info/other/shows.rss")
        .await?
        .bytes()
        .await?;

    let channel = Channel::read_from(&content[..]).unwrap();
    Ok(channel)
}

pub async fn feed_titles() -> Result<impl warp::Reply, Infallible> {
    //TODO: figure out how to return errors

    let channel = shows_rss_info_feed().await.unwrap();

    let series_title_list: HashSet<String> = channel
        .items
        .iter()
        .map(|item| {
            let mut result = "No Title Found".to_string();

            println!("Item: {:?}", item);
            if let Some(title) = item.title.clone() {
                result = title;
            }

            result
        })
        .collect();

    println!("{:?}", channel);

    Ok(warp::reply::json(&series_title_list))
}
