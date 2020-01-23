// 1. First approach with lazy_static fail with private type again.

// The type becomes very complicated.
// The author must have his own reason to make it(warp::filter) private.

// use warp::{
//     filters::BoxedFilter,
//     path,
//     Filter,
// };

// pub type JsonBody = warp::filter::and::And<impl warp::filter::Filter+std::marker::Copy, impl warp::filter::Filter+std::marker::Copy>;
// lazy_static! {
//     pub static ref JSONBODY: JsonBody = {
//         let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());
//         json_body
//     };
// }

// 2. Use macro then.

#[macro_export]
macro_rules! json_body {
    () => {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}