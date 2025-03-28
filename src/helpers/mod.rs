use warp::Filter;
use crate::models::Email;

pub fn json_body() -> impl Filter<
Extract = (Email,), Error = warp::Rejection
> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

