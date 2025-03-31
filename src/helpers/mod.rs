/*
 * /src/helpers/mod.rs
*/

use warp::Filter;
use crate::models::Email;

/// Extract JSON Body 
///
/// Limits JSON body to 16KBs and returns valid JSON. 
pub fn json_body() -> 
impl Filter<Extract = (Email,), Error = warp::Rejection> + Clone {
    // Accept JSON body (reject huge payloads)
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

