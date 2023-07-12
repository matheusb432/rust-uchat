use serde::{Deserialize, Serialize};

pub mod post;
pub mod user;

pub trait Endpoint {
    const URL: &'static str;
    fn url(&self) -> &'static str {
        Self::URL
    }
}

#[derive(thiserror::Error, Debug, Deserialize, Serialize)]
#[error("{msg}")]
pub struct RequestFailed {
    pub msg: String,
}

// ? Alternative route macro
// usage: route!(<NewPost>, "/post/new");
// macro_rules! route {
//     (<$for:ident>,$url:literal) => {
//         impl Endpoint for $for {
//             const URL: &'static str = $url;
//         }
//     };
// }

/// Macro for implementing Endpoint trait for a type
///
/// # Example
///
/// ```
/// route!("/post/new" => NewPost);
///
/// assert_eq!(NewPost::URL, "/post/new");
/// ```
macro_rules! route {
    ($url:literal => $request: ty) => {
        impl Endpoint for $request {
            const URL: &'static str = $url;
        }
    };
}

// public routes
route!("/account/create" => user::endpoint::CreateUser);
route!("/account/login" => user::endpoint::Login);

// authorized routes
route!("/post/new" => post::endpoint::NewPost);
route!("/post/bookmark" => post::endpoint::Bookmark);
route!("/post/boost" => post::endpoint::Boost);
route!("/posts/trending" => post::endpoint::TrendingPosts);
route!("/post/react" => post::endpoint::React);
