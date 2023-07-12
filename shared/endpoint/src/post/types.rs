use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uchat_domain::{
    ids::{ImageId, PostId, UserId},
    post::{Caption, Headline, Message},
    Username,
};
use url::Url;

use crate::user::types::PublicUserProfile;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Chat {
    pub headline: Option<Headline>,
    pub message: Message,
}

impl From<Chat> for Content {
    fn from(value: Chat) -> Self {
        Content::Chat(value)
    }
}

// NOTE DataUrl is sent from the client, The server returns an Url to the client.
// ? so different types for the image make more sense than just `String`
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ImageKind {
    DataUrl(String),
    Id(ImageId),
    Url(Url),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Image {
    pub kind: ImageKind,
    pub caption: Option<Caption>,
}

impl From<Image> for Content {
    fn from(value: Image) -> Self {
        Content::Image(value)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Content {
    Chat(Chat),
    Image(Image),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct NewPostOptions {
    pub reply_to: Option<PostId>,
    pub direct_message_to: Option<UserId>,
    pub time_posted: DateTime<Utc>,
}

impl Default for NewPostOptions {
    fn default() -> Self {
        Self {
            reply_to: None,
            direct_message_to: None,
            time_posted: Utc::now(),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum LikeStatus {
    Dislike,
    Like,
    NoReaction,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PublicPost {
    pub id: PostId,
    pub by_user: PublicUserProfile,
    pub content: Content,
    pub time_posted: DateTime<Utc>,
    pub reply_to: Option<(Username, UserId, PostId)>,
    pub like_status: LikeStatus,
    pub bookmarked: bool,
    pub boosted: bool,
    pub likes: i64,
    pub dislikes: i64,
    pub boosts: i64,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum BookmarkAction {
    Add,
    Remove,
}

impl Into<bool> for BookmarkAction {
    fn into(self) -> bool {
        match self {
            BookmarkAction::Add => true,
            BookmarkAction::Remove => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum BoostAction {
    Add,
    Remove,
}

impl Into<bool> for BoostAction {
    fn into(self) -> bool {
        match self {
            BoostAction::Add => true,
            BoostAction::Remove => false,
        }
    }
}
