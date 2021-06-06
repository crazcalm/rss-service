use crate::routes;
use warp::Filter;

pub fn register() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let health_endpoints = routes::health::register();
    let anime_routes = routes::crunchyroll::register();

    health_endpoints.or(anime_routes)
}
