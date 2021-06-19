use crate::routes;
use warp::Filter;

pub fn register() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let health_endpoints = routes::health::register();
    let anime_routes = routes::crunchyroll::register();
    let showrss_info_routes = routes::showrss_info::register();

    health_endpoints.or(anime_routes).or(showrss_info_routes)
}
