pub mod chat;
pub mod image;
pub mod new_post_app_bar;
pub mod poll;

pub use chat::NewChat;
pub use image::NewImage;
pub use poll::NewPoll;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NewPostPages {
    Chat,
    Image,
    Poll,
}
