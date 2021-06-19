use crate::handlers::showrss_info;
use warp::Filter;

pub fn register() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("rss-service" / "shows" / "showrss-info" / "titles")
        .and(warp::get())
        .and_then(showrss_info::feed_titles)
        .with(warp::log("showrss-info endpoint"))
}
