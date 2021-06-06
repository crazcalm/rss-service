use crate::handlers::crunchyroll;
use warp::Filter;

pub fn register() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("rss-service" / "anime" / "crunchyroll" / "titles")
        .and(warp::get())
        .and_then(crunchyroll::feed_titles)
        .with(warp::log("crunchyroll endpoint"))
}
