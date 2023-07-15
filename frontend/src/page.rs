pub mod home;
pub mod login;
pub mod new_post;
pub mod register;
pub mod trending;

pub use home::Home;
pub use login::Login;
pub use new_post::*;
pub use register::Register;
pub use route::*;
pub use trending::Trending;

pub mod route {
    pub const ACCOUNT_REGISTER: &str = "/account/register";
    pub const ACCOUNT_LOGIN: &str = "/account/login";
    pub const HOME: &str = "/home";
    pub const POST_NEW_CHAT: &str = "/post/new_chat";
    pub const POSTS_TRENDING: &str = "/posts/trending";
    pub const POST_NEW_IMAGE: &str = "/post/new_image";
    pub const POST_NEW_POLL: &str = "/post/new_poll";
    pub const HOME_LIKED: &str = "/home/liked";
    pub const HOME_BOOKMARKED: &str = "/home/bookmarked";
}
