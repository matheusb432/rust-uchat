use serde::{Deserialize, Serialize};

pub mod post;
pub mod user;

use load_dotenv::load_dotenv;

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

// NOTE load_dotenv will load the environment variables at compile time
load_dotenv!();

pub mod app_url {
    use std::str::FromStr;

    use url::Url;

    pub const API_URL: &str = std::env!("API_URL");

    /// Joins text to the API_URL environment variables
    ///
    /// # Example
    ///
    /// ```rust
    /// pub const API_URL: &str = "http://127.0.0.1:8070/";
    ///
    /// let url = domain_and("img/happy.png");
    ///
    /// assert_eq!(url.as_str(), "http://127.0.0.1:8070/img/happy.png");
    /// ```
    pub fn domain_and(fragment: &str) -> Url {
        Url::from_str(API_URL)
            .and_then(|url| url.join(fragment))
            .unwrap()
    }

    pub mod user_content {
        pub const ROOT: &str = "usercontent/";
        pub const IMAGES: &str = "img/";
    }
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
